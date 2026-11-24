import { useEffect, useRef, useState } from 'react'
import { Check, Plus, RotateCcw, Star, X } from 'lucide-react'
import { api, errorText, isConflict } from './api'
import type { AssetDetail } from './generated/AssetDetail'
import type { EditPatch } from './generated/EditPatch'
import { assetName, typeOf } from './presentation'

export type EditStatus = 'clean' | 'dirty' | 'saving'

function originalFields(detail: AssetDetail) {
  const m = detail.manifest
  if (!m || typeof m !== 'object' || Array.isArray(m)) throw Error('资产记录格式有误')
  const source = m.source
  const url = source && typeof source === 'object' && !Array.isArray(source) ? source.url : null
  return {
    tags: Array.isArray(m.tags) ? m.tags.filter((v): v is string => typeof v === 'string') : [],
    rating: typeof m.rating === 'number' ? m.rating : 0,
    note: typeof m.note === 'string' ? m.note : '',
    url: typeof url === 'string' ? url : '',
  }
}

function addTag(tags: string[], input: string) {
  const tag = input.trim()
  return tag && !tags.includes(tag) ? [...tags, tag] : tags
}

export function AssetEditor({ detail, onStatus, cancel, saved, reload }: {
  detail: AssetDetail
  onStatus: (status: EditStatus) => void
  cancel: () => void
  saved: () => void
  reload: () => void
}) {
  const [base] = useState(() => originalFields(detail))
  const [draft, setDraft] = useState(base)
  const [input, setInput] = useState('')
  const [focused, setFocused] = useState(false)
  const [suggestions, setSuggestions] = useState<string[]>([])
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState('')
  const [conflict, setConflict] = useState(false)
  const tagInput = useRef<HTMLInputElement>(null)
  const saving = useRef(false)
  const category = typeOf(detail.summary.asset_type)
  const Icon = category.icon

  const tags = addTag(draft.tags, input)
  const patch: EditPatch = {}
  if (tags.length !== base.tags.length || tags.some((tag, i) => base.tags[i] !== tag)) patch.tags = tags
  if (draft.rating !== base.rating) patch.rating = draft.rating
  if (draft.note !== base.note) patch.note = draft.note
  if (draft.url !== base.url && draft.url.trim() !== base.url) patch.source = { url: draft.url.trim() || null }
  const changed = Object.keys(patch).length > 0

  useEffect(() => { onStatus(busy ? 'saving' : changed ? 'dirty' : 'clean') }, [busy, changed, onStatus])
  useEffect(() => () => onStatus('clean'), [onStatus])
  useEffect(() => {
    if (!focused) return
    let active = true
    const timer = setTimeout(() => {
      api.suggestTags(input.trim(), 8).then(items => {
        if (active) setSuggestions(items)
      }).catch(() => { if (active) setSuggestions([]) })
    }, 120)
    return () => { active = false; clearTimeout(timer) }
  }, [input, focused])

  function append(value: string) {
    setDraft(old => ({ ...old, tags: addTag(old.tags, value) }))
    setInput('')
    tagInput.current?.focus()
  }

  async function save() {
    if (!changed || conflict || saving.current) return
    saving.current = true
    onStatus('saving')
    setBusy(true)
    setError('')
    try {
      await api.edit(detail.summary.id, patch, detail.base_hash)
      saved()
    } catch (e: unknown) {
      setConflict(isConflict(e))
      setError(isConflict(e) ? '这份记录已在其他位置修改。重新加载最新版本后可继续编辑。' : errorText(e))
    } finally {
      saving.current = false
      setBusy(false)
    }
  }

  return <form className="asset-editor" aria-label="编辑资产" onSubmit={e => { e.preventDefault(); void save() }}>
    <div className="editor-scroll">
      <div className="detail-title" data-type={detail.summary.asset_type}>
        <span className="asset-medallion"><Icon size={23}/></span>
        <div><span className="type-label">{category.label}</span><h2>{assetName(detail.summary)}</h2></div>
      </div>
      <fieldset disabled={busy || conflict} className="edit-fields">
        <div className="edit-field">
          <div className="edit-label"><span>评分</span><span className="field-value">{draft.rating ? `${draft.rating} / 5` : '未评分'}</span></div>
          <div className="star-picker" role="group" aria-label="评分">
            {[1,2,3,4,5].map(n => <button key={n} type="button" className={n <= draft.rating ? 'filled' : ''}
              aria-label={`${n} 星`} aria-pressed={n === draft.rating} onClick={() => setDraft(old => ({ ...old, rating: n }))}>
              <Star size={22} strokeWidth={1.6}/></button>)}
            <button type="button" className="clear-rating" disabled={!draft.rating} onClick={() => setDraft(old => ({ ...old, rating: 0 }))}>清除评分</button>
          </div>
        </div>
        <div className="edit-field tag-field" onBlur={e => { if (!e.currentTarget.contains(e.relatedTarget)) setFocused(false) }}>
          <label htmlFor="asset-tags">标签</label>
          {draft.tags.length > 0 && <div className="edit-tags">{draft.tags.map(tag => <span key={tag}>{tag}
            <button type="button" aria-label={`移除标签 ${tag}`} onClick={() => setDraft(old => ({ ...old, tags: old.tags.filter(t => t !== tag) }))}><X size={12}/></button></span>)}</div>}
          <div className="tag-input"><input id="asset-tags" aria-label="添加标签" ref={tagInput} value={input}
            placeholder="输入标签，按回车添加" autoComplete="off" onFocus={() => setFocused(true)} onChange={e => setInput(e.target.value)}
            onKeyDown={e => { if (e.key === 'Enter' && !e.nativeEvent.isComposing && e.nativeEvent.keyCode !== 229) { e.preventDefault(); append(input) } }}/>
            <button type="button" aria-label="添加当前标签" disabled={!input.trim()} onClick={() => append(input)}><Plus size={16}/></button></div>
          {focused && suggestions.some(s => !draft.tags.includes(s)) && <div className="suggestions" aria-label="已有标签">
            {suggestions.filter(s => !draft.tags.includes(s)).map(tag => <button type="button" key={tag} onClick={() => append(tag)}>{tag}</button>)}</div>}
        </div>
        <label className="edit-field"><span>简评</span><textarea aria-label="简评" value={draft.note}
          rows={4} onChange={e => { const note = e.currentTarget.value; setDraft(old => ({ ...old, note })) }}/></label>
        <label className="edit-field"><span>来源链接</span><input aria-label="来源链接" inputMode="url" value={draft.url}
          placeholder="https://" onChange={e => { const url = e.currentTarget.value; setDraft(old => ({ ...old, url })) }}/></label>
      </fieldset>
      {error && <div className="edit-error" role="alert"><p>{error}</p>
        {conflict && <button type="button" onClick={reload}><RotateCcw size={14}/>重新加载</button>}</div>}
    </div>
    <div className="editor-actions"><span role="status">{busy ? '正在保存…' : changed ? '有未保存的修改' : ''}</span>
      <button type="button" disabled={busy} onClick={cancel}>取消</button>
      <button className="primary" type="submit" disabled={!changed || busy || conflict}><Check size={15}/>保存</button></div>
  </form>
}
