import { useEffect, useRef, useState } from 'react'
import { History, RefreshCw, Undo2, X } from 'lucide-react'
import { api, errorText, isConflict } from './api'
import type { RecentEdits } from './generated/RecentEdits'

const fieldNames: Record<string, string> = { tags: '标签', rating: '评分', note: '简评', 'source.url': '来源链接', updated: '更新日期' }

export function EditHistory({ close, guard, restored, setWorking }: {
  close: () => void, guard: (action: () => void) => void, restored: () => void, setWorking: (value: boolean) => void,
}) {
  const modal = useRef<HTMLDialogElement>(null)
  const [entries, setEntries] = useState<RecentEdits>()
  const [limit, setLimit] = useState(50)
  const [revision, setRevision] = useState(0)
  const [error, setError] = useState('')
  const [confirmId, setConfirmId] = useState<string>()
  const [workingId, setWorkingId] = useState<string>()
  const inFlight = useRef(false)
  useEffect(() => {
    const node = modal.current
    node?.showModal()
    return () => node?.close()
  }, [])
  useEffect(() => {
    let active = true
    api.edits(limit).then(data => { if (active) setEntries(data) })
      .catch((e: unknown) => { if (active) setError(errorText(e)) })
    return () => { active = false }
  }, [revision, limit])

  function refresh() {
    setError('')
    setConfirmId(undefined)
    setRevision(n => n + 1)
  }

  async function undo(entryId: string) {
    if (inFlight.current) return
    inFlight.current = true
    setWorking(true)
    setWorkingId(entryId)
    setError('')
    try {
      await api.undo(entryId)
      setEntries(current => current?.map(entry => entry.entry_id === entryId ? { ...entry, undone: true } : entry))
      restored()
      refresh()
    } catch (e: unknown) {
      setError(isConflict(e) ? '这份记录在此后有过修改，无法撤回。请重新加载并核对当前内容。' : errorText(e))
      setConfirmId(undefined)
    } finally {
      inFlight.current = false
      setWorking(false)
      setWorkingId(undefined)
    }
  }

  return <dialog ref={modal} className="check-dialog history-dialog" aria-labelledby="history-title"
    onCancel={e => { e.preventDefault(); if (!inFlight.current) close() }}>
    <div className="panel-heading"><h2 id="history-title"><History size={18}/>最近修改</h2>
      <button className="icon-button" aria-label="关闭最近修改" disabled={!!workingId} onClick={close}><X size={17}/></button></div>
    <div className="check-content edit-history">
      {error && <p className="edit-error" role="alert">{error}</p>}
      {!entries && !error && <p className="loading" role="status"><span className="spinner"/>正在读取修改记录…</p>}
      {entries && !entries.length && <p className="history-empty">当前资产库还没有编辑记录。</p>}
      {entries?.map(entry => <article key={entry.entry_id} data-entry-id={entry.entry_id}>
        <div className="history-row"><time dateTime={entry.timestamp}>{new Date(entry.timestamp).toLocaleString('zh-CN', { hour12: false })}</time>
          {entry.undone ? <span>已撤回</span> : entry.entry_id !== confirmId &&
            <button disabled={!!workingId} onClick={() => { setError(''); setConfirmId(entry.entry_id) }}><Undo2 size={13}/>撤回</button>}</div>
        <strong>{entry.relative_path.split('/').at(-2) ?? entry.id}</strong>
        <p className="history-fields">{entry.changed_fields.map(f => fieldNames[f] ?? f).join('、')}</p>
        <p className="history-path">{entry.relative_path}</p>
        {confirmId === entry.entry_id && <div className="undo-confirm"><p>将还原这次编辑前的文件内容。</p><div>
          <button disabled={!!workingId} onClick={() => setConfirmId(undefined)}>取消</button>
          <button className="primary" disabled={!!workingId} onClick={() => guard(() => { void undo(entry.entry_id) })}>
            {workingId ? '正在撤回…' : '确认撤回'}</button></div></div>}
      </article>)}
      {entries && entries.length === limit && <button disabled={!!workingId} onClick={() => setLimit(n => n + 50)}>查看更早的修改</button>}
    </div>
    <div className="dialog-actions"><button disabled={!!workingId} onClick={refresh}><RefreshCw size={14}/>重新加载</button>
      <button disabled={!!workingId} onClick={close}>关闭</button></div>
  </dialog>
}
