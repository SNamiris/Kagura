import { useCallback, useEffect, useRef, useState } from 'react'
import { isTauri } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Activity, Disc3, FolderOpen, History, LibraryBig, RefreshCw, Search, ShieldCheck, SlidersHorizontal, X } from 'lucide-react'
import { api, errorText } from './api'
import { Catalog } from './Catalog'
import { CheckDialog } from './CheckDialog'
import { Details } from './Details'
import { DiscardDialog } from './DiscardDialog'
import { EditHistory } from './EditHistory'
import type { EditStatus } from './AssetEditor'
import { Sidebar } from './Sidebar'
import { defaultSort, emptyFilters, number } from './presentation'
import type { LibraryStats } from './generated/LibraryStats'
import './App.css'
import './editing.css'

export default function App() {
  const [library, setLibrary] = useState<LibraryStats>()
  const [busy, setBusy] = useState<'open' | 'rescan' | 'choose' | null>(isTauri() ? 'open' : null)
  const [message, setMessage] = useState('')
  const [error, setError] = useState('')
  const [input, setInput] = useState('')
  const [query, setQuery] = useState('')
  const [filters, setFilters] = useState(emptyFilters)
  const [sort, setSort] = useState(defaultSort)
  const [view, setView] = useState<'grid' | 'list'>('grid')
  const [revision, setRevision] = useState(0)
  const [history, setHistory] = useState<string[]>([])
  const [checking, setChecking] = useState(false)
  const [recent, setRecent] = useState(false)
  const [discardPrompt, setDiscardPrompt] = useState(false)
  const [detailRevision, setDetailRevision] = useState(0)
  const editStatus = useRef<EditStatus>('clean')
  const undoing = useRef(false)
  const pending = useRef<(() => void) | null>(null)
  const [filtersVisible, setFiltersVisible] = useState(false)
  const searchBox = useRef<HTMLInputElement>(null)
  const selected = history.at(-1) ?? null
  const clearDetails = useCallback(() => setHistory([]), [])
  const onStatus = useCallback((status: EditStatus) => { editStatus.current = status }, [])
  const guard = useCallback((action: () => void) => {
    if (editStatus.current === 'saving' || undoing.current) return
    if (editStatus.current === 'dirty') {
      if (pending.current) return
      pending.current = action
      setDiscardPrompt(true)
    } else {
      action()
    }
  }, [])

  function resolveDiscard(discard: boolean) {
    const action = pending.current
    pending.current = null
    setDiscardPrompt(false)
    if (discard && action) {
      onStatus('clean')
      setDetailRevision(n => n + 1)
      action()
    }
  }

  useEffect(() => {
    if (!isTauri()) return
    const desktop = getCurrentWindow()
    const listener = desktop.onCloseRequested(event => {
      if (editStatus.current === 'clean' && !undoing.current) return
      event.preventDefault()
      guard(() => { void desktop.destroy().catch((e: unknown) => setError(errorText(e))) })
    })
    return () => { void listener.then(unlisten => unlisten()) }
  }, [guard])
  useEffect(() => {
    const beforeUnload = (e: BeforeUnloadEvent) => {
      if (editStatus.current !== 'clean' || undoing.current) e.preventDefault()
    }
    window.addEventListener('beforeunload', beforeUnload)
    return () => window.removeEventListener('beforeunload', beforeUnload)
  }, [])

  useEffect(() => {
    if (!isTauri()) return
    let active = true
    api.settings().then(async settings => {
      if (!active || !settings.root) return
      const stats = await api.open(settings.root)
      if (active) setLibrary(stats)
    }).catch((e: unknown) => { if (active) setError(errorText(e)) })
      .finally(() => { if (active) setBusy(null) })
    return () => { active = false }
  }, [])
  useEffect(() => {
    const timer = setTimeout(() => setQuery(input.trim()), 200)
    return () => clearTimeout(timer)
  }, [input])
  useEffect(() => {
    const keys = (event: KeyboardEvent) => {
      if (document.querySelector('dialog[open]') || event.isComposing) return
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'k') {
        event.preventDefault()
        searchBox.current?.focus()
        searchBox.current?.select()
      } else if (event.key === 'Escape') {
        event.preventDefault()
        guard(() => { clearDetails(); setFiltersVisible(false) })
      }
    }
    window.addEventListener('keydown', keys)
    return () => window.removeEventListener('keydown', keys)
  }, [guard, clearDetails])

  async function choose() {
    setDetailRevision(n => n + 1)
    setError('')
    setMessage('')
    setBusy('choose')
    try {
      const selection = await api.choose()
      if (!selection.root) return
      setBusy('open')
      const stats = await api.open(selection.root)
      setLibrary(stats)
      setInput(''); setQuery(''); setFilters(emptyFilters); clearDetails(); setFiltersVisible(false)
      setRevision(n => n + 1)
    } catch (e: unknown) { setError(errorText(e)) } finally { setBusy(null) }
  }
  async function rescan() {
    setDetailRevision(n => n + 1)
    setError(''); setBusy('rescan'); setMessage('')
    try {
      const changes = await api.rescan()
      setLibrary(await api.stats())
      setRevision(n => n + 1)
      const changed = changes.inserted || changes.updated || changes.deleted || changes.files_changed || changes.files_deleted
      setMessage(changed ? `新增 ${changes.inserted}，更新 ${changes.updated}，移除 ${changes.deleted}，文件变更 ${changes.files_changed}，文件移除 ${changes.files_deleted}` : '重新扫描完成，资产库无变化')
    } catch (e: unknown) { setError(errorText(e)) } finally { setBusy(null) }
  }
  const follow = (id: string) => guard(() => setHistory(old => old.at(-1) === id ? old : [...old, id]))
  function refreshAfterEdit(notice: string) {
    onStatus('clean')
    setRevision(n => n + 1)
    setError('')
    setMessage(notice)
  }
  const loading = busy === 'open' ? '正在打开资产库并更新索引…' : busy === 'rescan' ? '正在扫描资产库…' : '正在选择库文件夹…'

  return <main className={'library-app' + (filtersVisible ? ' filters-open' : '') + (selected ? ' has-detail' : '')}>
    <header className="topbar">
      <div className="brand"><span className="brand-mark"><Disc3 size={24} strokeWidth={1.65}/></span><span lang="en">Kagura</span></div>
      {library && <><div className="search-box"><Search size={17}/><input aria-label="搜索资产" placeholder="搜索资产、作者、标签…" value={input} onChange={e => setInput(e.target.value)} ref={searchBox}/>
        {input ? <button className="icon-button" aria-label="清空搜索" onClick={() => { setInput(''); searchBox.current?.focus() }}><X size={14}/></button> : <kbd>Ctrl K</kbd>}</div>
        <div className="top-actions"><button aria-label="最近修改" title="最近修改" onClick={() => setRecent(true)} disabled={!!busy}><History size={16}/><span>最近修改</span></button>
          <button aria-label="库检查" title="库检查" onClick={() => setChecking(true)} disabled={!!busy}><ShieldCheck size={16}/><span>库检查</span></button>
          <button aria-label="重新扫描" title="重新扫描" onClick={() => guard(() => { void rescan() })} disabled={!!busy}><RefreshCw size={15}/><span>重新扫描</span></button>
          <button className="mobile-filter icon-button" aria-label="显示筛选" aria-pressed={filtersVisible} onClick={() => setFiltersVisible(v => !v)}><SlidersHorizontal size={16}/></button></div></>}
    </header>
    {(error || (library && (busy || message))) && <div className={error ? 'app-notice error-box' : 'app-notice'} role={error ? 'alert' : 'status'}>
      {busy && <span className="spinner"/>}<span data-testid={busy ? 'library-busy' : undefined}>{error || (busy ? loading : message)}</span>
      {!busy && <button className="icon-button" aria-label="关闭提示" onClick={() => { setError(''); setMessage('') }}><X size={14}/></button>}</div>}
    {library ? <div className="workspace">
      <Sidebar stats={library} filters={filters} onChange={setFilters} choose={() => guard(() => { void choose() })} busy={!!busy}/>
      <Catalog key={JSON.stringify([query, filters, sort, revision])} query={query} filters={filters} sort={sort} setSort={setSort}
        view={view} setView={setView} selected={selected} open={id => {
          if (id !== selected) guard(() => { setHistory([id]); setFiltersVisible(false) })
        }}/>
      {selected && <Details key={[selected, revision, detailRevision].join(':')} id={selected} close={() => guard(clearDetails)} follow={follow}
        canEdit={!busy} guard={guard} onStatus={onStatus} saved={() => refreshAfterEdit('已保存修改')}
        back={history.length > 1 ? () => guard(() => setHistory(old => old.slice(0, -1))) : undefined}/>}
    </div> : <section className="welcome"><div className="welcome-inner"><span className="welcome-icon"><LibraryBig size={36} strokeWidth={1.4}/></span>
      <h1>打开资产库</h1><p>{isTauri() ? '选择包含 Library 文件夹的库根目录。' : '请在 Kagura 桌面窗口中打开本地资产库。'}</p>
      {busy ? <p className="loading" role="status" data-testid="library-busy"><span className="spinner"/>{loading}</p>
        : <button className="primary" disabled={!isTauri()} onClick={() => void choose()}><FolderOpen size={17}/>选择库文件夹</button>}</div></section>}
    <footer className="statusbar"><span title={library?.root}><FolderOpen size={13}/>{library?.root ?? '尚未连接资产库'}</span>
      {library && <><span>{number(library.total)} 份资产</span><span><Activity size={13}/>上次扫描 {new Date(library.last_scan * 1000).toLocaleString('zh-CN',{hour12:false})}</span></>}</footer>
    {checking && <CheckDialog close={() => setChecking(false)}/>}
    {recent && <EditHistory close={() => setRecent(false)} guard={guard} setWorking={value => { undoing.current = value }}
      restored={() => refreshAfterEdit('已撤回修改')}/>}
    {discardPrompt && <DiscardDialog keep={() => resolveDiscard(false)} discard={() => resolveDiscard(true)}/>}
  </main>
}
