import { useEffect, useState } from 'react'
import { ArrowLeft, ArrowUpRight, ChevronRight, FileAudio, FolderOpen, Pencil, X } from 'lucide-react'
import { api, errorText } from './api'
import { assetName, kinds, number, partName, person, size, typeOf } from './presentation'
import type { AssetDetail } from './generated/AssetDetail'
import type { AssetRef } from './generated/AssetRef'
import type { AssetSummary } from './generated/AssetSummary'
import { AssetEditor, type EditStatus } from './AssetEditor'

const labels: Record<string, string> = {
  author: '作者', name: '名称', name_original: '原名', title: '曲名', composer: '作曲者', edition: '版本', suffix: '后缀', part: '类别',
  commercial: '商业使用', modify: '修改', redistribute: '再分发', credit_required: '需要署名', r18: '成人内容', readme: '说明文件', summary: '授权摘要',
  obtained: '获取日期', url: '来源网址', platform: '平台', archive: '原始压缩包', file: '文件', label: '名称', main: '主要', note: '备注',
  role: '用途', performer: '表演者', singer: '歌手', offset_frames: '偏移帧数', kind: '类别', optional: '可选', format: '格式', made_for: '适配', variant: '变体',
  hash: '哈希', character: '角色', series: '系列', added: '添加日期', updated: '更新日期', name_ja: '日文名',
}

function object(value: unknown): Record<string, unknown> {
  return value && typeof value === 'object' && !Array.isArray(value) ? Object.fromEntries(Object.entries(value)) : {}
}
function list(value: unknown): unknown[] { return Array.isArray(value) ? value : [] }
function valueText(value: unknown): string {
  if (value == null || value === '_Unknown') return '未知'
  if (typeof value === 'boolean') return value ? '是' : '否'
  if (Array.isArray(value)) return value.map(valueText).join('、')
  return typeof value === 'object' ? JSON.stringify(value) : String(value)
}
function Fields({ fields }: { fields: Record<string, unknown> }) {
  return <dl className="fields">{Object.entries(fields).map(([key, value]) =>
    <div key={key}><dt>{labels[key] ?? key}</dt><dd>{key === 'part' && typeof value === 'string' ? partName(value) : valueText(value)}</dd></div>)}</dl>
}
function ManifestFiles({ title, value }: { title: string, value: unknown }) {
  const entries = list(value)
  return entries.length ? <section className="detail-section"><h3>{title} <small>{entries.length}</small></h3>
    {entries.map((entry, i) => <Fields key={i} fields={object(entry)} />)}</section> : null
}
function ReferenceLinks({ title, refs, follow }: { title: string, refs: AssetRef[], follow: (id: string) => void }) {
  return refs.length > 0 && <section className="detail-section"><h3>{title} <small>{refs.length}</small></h3>
    {refs.map(ref => <button key={ref.field + ref.id} className="reference" title={ref.id} onClick={() => follow(ref.id)}>
      <span>{ref.name || ref.id}<small>{ref.field}</small></span><ArrowUpRight size={14}/></button>)}</section>
}

export function Details({ id, close, follow, back, guard, onStatus, saved, canEdit }: {
  id: string, close: () => void, follow: (id: string) => void, back?: () => void,
  guard: (action: () => void) => void, onStatus: (status: EditStatus) => void, saved: () => void, canEdit: boolean,
}) {
  const [detail, setDetail] = useState<AssetDetail>()
  const [children, setChildren] = useState<AssetSummary[]>()
  const [rawOpen, setRawOpen] = useState(false)
  const [error, setError] = useState('')
  const [attempt, setAttempt] = useState(0)
  const [editing, setEditing] = useState(false)
  useEffect(() => {
    let active = true
    api.asset(id).then(async data => {
      if (!active) return
      setDetail(data)
      if (data.summary.asset_type === 'song') {
        const items = await api.children(id)
        if (active) setChildren(items)
      }
    }).catch((e: unknown) => { if (active) setError(errorText(e)) })
    return () => { active = false }
  }, [id, attempt])
  const asset = detail?.summary
  const manifest = object(detail?.manifest)
  const category = typeOf(asset?.asset_type ?? '')
  const Icon = category.icon
  const isSong = asset?.asset_type === 'song'
  const license = object(manifest.license)
  const source = object(manifest.source)
  const tags = list(manifest.tags)
  const groupedChildren = new Map<string, AssetSummary[]>()
  for (const item of children ?? []) {
    const part = item.part ?? '其他'
    const group = groupedChildren.get(part)
    if (group) group.push(item)
    else groupedChildren.set(part, [item])
  }
  const related = detail?.outgoing.filter(r => r.field.startsWith('related[')) ?? []
  const parent = detail?.outgoing.filter(r => r.field === 'song') ?? []
  const others = detail?.outgoing.filter(r => r.field !== 'song' && !r.field.startsWith('related[')) ?? []
  const extra = Object.fromEntries(['name_original', 'character', 'series','added','updated'].filter(k => manifest[k] != null).map(k => [k, manifest[k]]))

  return <aside className="details" aria-label="资产详情">
    <div className="panel-heading"><h2>{editing ? '编辑资产' : '资产详情'}</h2><div>
      {detail && !editing && <button className="edit-trigger" disabled={!canEdit} onClick={() => setEditing(true)}><Pencil size={14}/>编辑</button>}
      {back && <button className="icon-button" aria-label="返回上一资产" title="返回上一资产" onClick={back}><ArrowLeft size={16}/></button>}
      <button className="icon-button" aria-label="关闭详情" title="关闭详情" onClick={close}><X size={17}/></button></div></div>
    {editing && detail ? <AssetEditor detail={detail} onStatus={onStatus} saved={saved}
      cancel={() => guard(() => setEditing(false))}
      reload={() => { setDetail(undefined); setError(''); setChildren(undefined); setAttempt(n => n + 1) }}/>
      : <div className="detail-scroll">
      {error && <div className="error-box" role="alert">{error}<button onClick={() => { setError(''); setAttempt(n => n + 1) }}>重新载入详情</button></div>}
      {!detail && !error && <p role="status">正在读取资产…</p>}
      {asset && detail && <>
        <div className="detail-title" data-type={asset.asset_type}><span className="asset-medallion"><Icon size={23}/></span><div>
          <span className="type-label">{category.label}</span><h2>{assetName(asset)}</h2></div></div>
        <p className="detail-byline">{isSong ? '作曲者' : '作者'} · {person(asset.author || asset.composer)}</p>
        <div className="asset-location"><p title={detail.absolute_path}>{detail.absolute_path}</p>
          <button onClick={() => { api.folder(id).catch((e: unknown) => setError(errorText(e))) }}><FolderOpen size={14}/>打开所在文件夹</button></div>
        <section className="detail-section"><h3>基本信息</h3><dl className="fields">
          <div><dt>ID</dt><dd className="asset-id">{asset.id}</dd></div>
          {asset.kind && <div><dt>类别</dt><dd>{kinds.find(k => k.value === asset.kind)?.label ?? asset.kind}</dd></div>}
          <div><dt>评分</dt><dd>{asset.rating ? `${asset.rating} / 5` : '未评分'}</dd></div>
          <div><dt>文件</dt><dd>{number(detail.file_count)} 个 · {size(detail.total_bytes)}</dd></div></dl>
          <Fields fields={extra}/></section>
        {parent.length > 0 && <ReferenceLinks title="所属歌曲" refs={parent} follow={follow}/>}
        {isSong && <section className="detail-section"><h3>歌曲素材 {children && <small>{children.length}</small>}</h3>
          {!children ? <p>正在读取素材…</p> : !children.length ? <p>暂无素材</p> : [...groupedChildren].map(([part, entries]) =>
            <div className="child-group" key={part}><h4>{partName(part)}</h4>{entries.map(child => <button key={child.id} className="reference" onClick={() => follow(child.id)}>
              <span>{assetName(child)}<small>{person(child.author)}</small></span><ChevronRight size={14}/></button>)}</div>)}</section>}
        <ReferenceLinks title="相关版本" refs={related} follow={follow}/>
        {isSong && <section className="detail-section"><h3>歌曲音频 <small>{detail.audio_files.length}</small></h3>
          {detail.audio_files.length ? detail.audio_files.map(path => <p className="audio-file" key={path}><FileAudio size={14}/>{path}</p>) : <p>没有 _audio 文件</p>}</section>}
        <section className="detail-section"><h3>识别键 <small>key</small></h3><Fields fields={object(manifest.key)}/></section>
        <ManifestFiles title="变体" value={manifest.variants}/><ManifestFiles title="部件" value={manifest.parts}/>
        <ManifestFiles title="文件清单" value={manifest.files}/><ManifestFiles title="音频记录" value={manifest.audio}/>
        <section className="detail-section"><h3>授权</h3><Fields fields={{ commercial: null, modify: null, redistribute: null, ...license }}/></section>
        <section className="detail-section"><h3>来源</h3>{Object.keys(source).length ? <Fields fields={source}/> : <p>未知</p>}</section>
        <section className="detail-section"><h3>标签</h3>{tags.length ? <div className="tags">{tags.map(t => <span key={String(t)}>{valueText(t)}</span>)}</div> : <p>暂无标签</p>}</section>
        <section className="detail-section"><h3>备注</h3><p className="note">{manifest.note ? valueText(manifest.note) : '暂无备注'}</p></section>
        <ReferenceLinks title="引用的资产" refs={others} follow={follow}/><ReferenceLinks title="被这些资产引用" refs={detail.incoming} follow={follow}/>
        <details className="raw-manifest" onToggle={e => setRawOpen(e.currentTarget.open)}><summary>原始 manifest JSON</summary>
          {rawOpen && <pre>{JSON.stringify(detail.manifest, null, 2)}</pre>}</details>
      </>}
    </div>}
  </aside>
}
