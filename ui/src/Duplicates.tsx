import { useCallback, useEffect, useRef, useState } from 'react'
import { ArrowLeft, ChevronLeft, ChevronRight, Copy, Download, RefreshCw } from 'lucide-react'
import { api, errorText } from './api'
import { DedupHashTask } from './DedupHashTask'
import { DedupOverview, DedupResults } from './DedupResults'
import type { DedupOptions } from './generated/DedupOptions'
import type { DedupPage } from './generated/DedupPage'
import type { DedupSection } from './generated/DedupSection'
import { number } from './presentation'
import './duplicates.css'

type Tab = 'overview' | DedupSection
const tabs: { value: Tab, name: string }[] = [
  { value: 'overview', name: '总览' }, { value: 'cross', name: '跨资产文件' },
  { value: 'within', name: '资产内文件' }, { value: 'identical', name: '整份相同' }, { value: 'subsets', name: '子集关系' },
]
const switches: { key: keyof DedupOptions, name: string, hint: string }[] = [
  { key: 'include_small', name: '小文件', hint: '小于 4 KiB' },
  { key: 'include_toon', name: '共用 toon 贴图', hint: 'toon01.bmp 至 toon10.bmp' },
  { key: 'include_accessories', name: '场景控制器', hint: '≤ 64 KiB 的配件模型，按文件名识别' },
]
const pageSize = 12
const defaults: DedupOptions = { include_small: false, include_toon: false, include_accessories: false }

export function Duplicates({ root, close }: { root: string, close: () => void }) {
  const modal = useRef<HTMLDialogElement>(null)
  const scroll = useRef<HTMLDivElement>(null)
  const [view, setView] = useState<Tab>('overview')
  const [page, setPage] = useState(0)
  const [options, setOptions] = useState(defaults)
  const [revision, setRevision] = useState(0)
  const [loaded, setLoaded] = useState<{ key: string, data: DedupPage }>()
  const [loadError, setLoadError] = useState<{ key: string, text: string }>()
  const [feedback, setFeedback] = useState<{ text: string, error: boolean }>()
  const [exporting, setExporting] = useState(false)
  const refresh = useCallback(() => setRevision(value => value + 1), [])
  const key = JSON.stringify([root, options, view, page, revision])
  const data = loaded?.key === key ? loaded.data : undefined
  const problem = !data && loadError?.key === key ? loadError.text : undefined
  const section = view === 'overview' ? 'cross' : view
  const pages = data ? Math.ceil(data.total / pageSize) : 0

  useEffect(() => {
    const el = modal.current
    el?.showModal()
    return () => el?.close()
  }, [])
  useEffect(() => {
    let active = true
    const timer = setTimeout(() => {
      api.duplicates(options, section, view === 'overview' ? 0 : pageSize, page * pageSize).then(result => {
        if (!active) return
        if (view !== 'overview' && page > 0 && page * pageSize >= result.total) {
          setPage(Math.max(0, Math.ceil(result.total / pageSize) - 1))
          return
        }
        setLoaded({ key, data: result })
        scroll.current?.scrollTo({ top: 0 })
      }).catch((e: unknown) => { if (active) setLoadError({ key, text: errorText(e) }) })
    }, 0)
    return () => { active = false; clearTimeout(timer) }
  }, [key, options, section, view, page])

  function browse(next: Tab) {
    setView(next); setPage(0); setFeedback(undefined)
  }
  function toggle(field: keyof DedupOptions) {
    setOptions(old => ({ ...old, [field]: !old[field] }))
    setPage(0); setFeedback(undefined)
  }
  function open(id: string, rel?: string) {
    setFeedback(undefined)
    const action = rel === undefined ? api.folder(id) : api.copyFolder(id, rel)
    action.catch((e: unknown) => setFeedback({ text: errorText(e), error: true }))
  }
  async function exportReport() {
    setExporting(true); setFeedback(undefined)
    try {
      const result = await api.exportDuplicates(options)
      if (result.path) setFeedback({ text: '已导出至 ' + result.path, error: false })
    } catch (e: unknown) { setFeedback({ text: errorText(e), error: true }) }
    finally { setExporting(false) }
  }

  return <dialog ref={modal} className="duplicates-page" aria-labelledby="duplicates-title" onCancel={close}>
    <header className="dedup-header"><div className="dedup-header-start">
      <button className="dedup-back" onClick={close}><ArrowLeft size={16}/>返回资产库</button>
      <h1 id="duplicates-title"><Copy size={21}/>重复报告</h1>
    </div><div className="dedup-actions">
      <button title="刷新报告" aria-label="刷新报告" onClick={refresh}><RefreshCw size={15}/></button>
      <button disabled={exporting || !data} onClick={() => void exportReport()}>
        <Download size={15}/>{exporting ? '正在导出…' : '导出 JSON'}</button>
    </div></header>
    {feedback && <div className={'dedup-feedback' + (feedback.error ? ' dedup-error' : '')} role={feedback.error ? 'alert' : 'status'}>{feedback.text}</div>}
    <div className="dedup-workspace"><aside className="dedup-sidebar">
      <DedupHashTask root={root} summary={loaded?.data.summary} refresh={refresh}/>
      <fieldset className="dedup-options"><legend>显示排除项</legend>
        {switches.map(s => <label key={s.key}><input type="checkbox" checked={options[s.key]} onChange={() => toggle(s.key)}/>
          <span>{s.name}<small>{s.hint}</small></span></label>)}
        <p>只影响文件组与空间统计。整份相同和子集始终比较全部负载。</p>
      </fieldset>
    </aside>
    <section className="dedup-panel"><nav className="dedup-tabs" aria-label="重复报告视图">
      {tabs.map(t => <button key={t.value} aria-pressed={view === t.value} onClick={() => browse(t.value)}>{t.name}</button>)}
    </nav>
    <div ref={scroll} className="dedup-scroll" aria-busy={!data && !problem}>
      {problem ? <div className="dedup-error" role="alert">{problem} <button onClick={refresh}>重新加载</button></div>
        : !data ? <p className="loading"><span className="spinner"/>正在读取重复报告…</p>
          : <>{data.summary.hashed_files < data.summary.total_files &&
            <p className="dedup-partial">当前统计覆盖 {number(data.summary.hashed_files)} / {number(data.summary.total_files)} 个文件，
              已完成 {number(data.summary.complete_assets)} 份资产的哈希。</p>}
          {view === 'overview' ? <DedupOverview info={data.summary} browse={browse}/> :
            <DedupResults section={view} data={data} open={open}/>}</>}
    </div>
    {view !== 'overview' && data && <footer className="dedup-pagination"><span>共 {number(data.total)} {view === 'subsets' ? '对' : '组'}</span>
      {pages > 1 && <><button aria-label="上一页" disabled={page === 0} onClick={() => setPage(n => n - 1)}><ChevronLeft size={15}/></button>
        <span>{number(page + 1)} / {number(pages)}</span>
        <button aria-label="下一页" disabled={page + 1 >= pages} onClick={() => setPage(n => n + 1)}><ChevronRight size={15}/></button></>}
    </footer>}
    </section></div>
    <footer className="dedup-footer"><span title={root}>{root}</span>
      {data && <span>{number(data.summary.excluded_files)} 个文件因排除规则隐藏</span>}</footer>
  </dialog>
}
