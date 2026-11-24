import { useEffect, useRef } from 'react'

export function DiscardDialog({ keep, discard }: { keep: () => void, discard: () => void }) {
  const modal = useRef<HTMLDialogElement>(null)
  useEffect(() => {
    const node = modal.current
    node?.showModal()
    return () => node?.close()
  }, [])
  return <dialog ref={modal} className="check-dialog discard-dialog" aria-labelledby="discard-title"
    aria-describedby="discard-help" onCancel={e => { e.preventDefault(); keep() }}>
    <div className="check-content"><h2 id="discard-title">有未保存的修改</h2><p id="discard-help">离开将丢弃当前资产的草稿。</p></div>
    <div className="dialog-actions"><button className="primary" autoFocus onClick={keep}>继续编辑</button>
      <button onClick={discard}>放弃修改</button></div>
  </dialog>
}
