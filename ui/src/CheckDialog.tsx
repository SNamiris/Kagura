import { useEffect, useRef, useState } from 'react'
import { CircleCheck, RefreshCw, ShieldCheck, TriangleAlert, X } from 'lucide-react'
import { api, errorText } from './api'
import type { CheckReport } from './generated/CheckReport'
import { number } from './presentation'

const labels: Record<string, string> = {
  id_unique: 'ID 唯一性', path_matches_key: '目录与识别键', refs_resolve: '资产引用', files_exist: '文件存在性',
  no_orphan_dirs: '目录完整性', author_case: '作者大小写', path_length: '路径长度', newline_style: '换行风格', scan: '扫描错误',
}

export function CheckDialog({ close }: { close: () => void }) {
  const modal = useRef<HTMLDialogElement>(null)
  const [report, setReport] = useState<CheckReport>()
  const [error, setError] = useState('')
  const [attempt, setAttempt] = useState(0)
  const [shown, setShown] = useState(100)
  useEffect(() => {
    const node = modal.current
    node?.showModal()
    return () => node?.close()
  }, [])
  useEffect(() => {
    let active = true
    const timer = setTimeout(() => {
      api.check().then(data => { if (active) setReport(data) })
        .catch((reason: unknown) => { if (active) setError(errorText(reason)) })
    }, 0)
    return () => { active = false; clearTimeout(timer) }
  }, [attempt])
  const rerun = () => { setReport(undefined); setError(''); setShown(100); setAttempt(n => n + 1) }
  return <dialog ref={modal} onCancel={close} className="check-dialog" aria-labelledby="check-title">
    <div className="panel-heading"><h2 id="check-title"><ShieldCheck size={20}/>库检查</h2>
      <button className="icon-button" aria-label="关闭检查" onClick={close}><X size={18}/></button></div>
    <div className="check-content">
      {error && <p className="error-box" role="alert">{error}</p>}
      {!report && !error && <p className="loading" role="status"><span className="spinner"/>正在检查资产库…</p>}
      {report && <>
        <div className={'check-overview ' + (report.findings.length ? 'warning' : '')}>
          {report.findings.length ? <TriangleAlert size={29}/> : <CircleCheck size={29}/>}
          <div><h3>{report.findings.length} 个问题</h3><p>已检查 {number(Object.values(report.counts).reduce<number>((n, v) => n + (v ?? 0), 0))} 份资产清单</p></div></div>
        <dl className="checks">{Object.entries(report.check_counts).map(([key, value]) =>
          <div key={key}><dt>{labels[key] ?? key}</dt><dd>{value}</dd></div>)}</dl>
        {report.findings.length > 0 && <section className="findings"><h3>问题列表</h3>{report.findings.slice(0, shown).map((f, i) =>
          <article key={i}><strong>{labels[f.check] ?? f.check}</strong><p>{f.path}</p><p>{f.detail}</p></article>)}
          {shown < report.findings.length && <button onClick={() => setShown(n => n + 100)}>显示更多</button>}</section>}
      </>}
    </div>
    <div className="dialog-actions"><button onClick={rerun} disabled={!report && !error}><RefreshCw size={15}/>重新检查</button><button className="primary" onClick={close}>完成</button></div>
  </dialog>
}
