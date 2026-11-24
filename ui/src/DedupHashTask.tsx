import { useCallback, useEffect, useRef, useState } from 'react'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { CircleStop, Fingerprint, Pause, Play } from 'lucide-react'
import { api, errorText } from './api'
import type { DedupSummary } from './generated/DedupSummary'
import type { HashStatus } from './generated/HashStatus'
import { number, size } from './presentation'

const phaseLabels: Record<HashStatus['phase'], string> = {
  idle: '待开始', running: '正在计算', pausing: '正在暂停', paused: '已暂停',
  cancelling: '正在取消', cancelled: '已取消', complete: '已完成', failed: '已中断',
}
const active = (p: HashStatus['phase']) => p === 'running' || p === 'pausing' || p === 'cancelling'
const threadKey = 'kagura.hash.threads'

function savedThreadChoice() {
  try {
    const threads = Number(localStorage.getItem(threadKey))
    return Number.isInteger(threads) && threads >= 1 && threads <= 4 ? threads : undefined
  } catch { return undefined }
}

function remaining(seconds: number | null) {
  if (seconds === null || !Number.isFinite(seconds)) return '估算中'
  const n = Math.ceil(Math.max(0, seconds))
  if (n < 60) return n + ' 秒'
  const minutes = Math.ceil(n / 60)
  if (minutes < 60) return minutes + ' 分钟'
  return Math.floor(minutes / 60) + ' 小时 ' + minutes % 60 + ' 分钟'
}

export function DedupHashTask({ root, summary, refresh }: {
  root: string, summary?: DedupSummary, refresh: () => void,
}) {
  const [status, setStatus] = useState<HashStatus>()
  const [threadChoice, setThreadChoice] = useState(savedThreadChoice)
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState('')
  const latest = useRef<HashStatus | undefined>(undefined)

  const accept = useCallback((next: HashStatus) => {
    if (next.root !== root && !(next.phase === 'idle' && next.root === null)) return
    const prev = latest.current
    if (prev && prev.generation > next.generation) return
    if (prev && prev.generation === next.generation) {
      if ((prev.phase !== 'idle' && next.phase === 'idle') ||
          ((prev.phase === 'complete' || prev.phase === 'failed' || prev.phase === 'cancelled') && next.phase !== prev.phase) ||
          ((!active(prev.phase) && prev.phase !== 'idle') && active(next.phase)) ||
          ((prev.phase === 'pausing' || prev.phase === 'cancelling') && next.phase === 'running')) return
      if (prev.phase === next.phase && prev.completed_files > next.completed_files) return
    }
    latest.current = next
    setStatus(next)
    if (prev && (prev.generation !== next.generation || prev.phase !== next.phase) &&
        next.phase !== 'idle' && !active(next.phase)) refresh()
  }, [root, refresh])

  useEffect(() => {
    let mounted = true
    let stop: UnlistenFn | undefined
    async function connect() {
      const unlisten = await listen<HashStatus>('hash-progress', event => { if (mounted) accept(event.payload) })
      if (!mounted) { unlisten(); return }
      stop = unlisten
      const snapshot = await api.hashStatus()
      if (mounted) accept(snapshot)
    }
    void connect().catch((e: unknown) => { if (mounted) setError(errorText(e)) })
    return () => { mounted = false; stop?.() }
  }, [accept])

  const phase = status?.phase ?? 'idle'
  const running = active(phase)
  const settling = phase === 'pausing' || phase === 'cancelling'
  const threads = running && status ? status.threads : threadChoice ?? status?.threads ?? 1
  const useJob = !!status && phase !== 'idle'
  const done = useJob ? status.completed_files : summary?.hashed_files ?? 0
  const total = useJob ? status.total_files : summary?.total_files ?? 0
  const knownBytes = !!status && (useJob || status.total_files > 0)
  const bytes = knownBytes ? status.completed_bytes : 0
  const totalBytes = knownBytes ? status.total_bytes : 0
  const ratio = totalBytes ? bytes / totalBytes : total ? done / total : 0
  const percent = Math.min(100, Math.max(0, Math.floor(ratio * 100)))
  const complete = total > 0 && done === total

  async function run(action: 'start' | 'pause' | 'cancel') {
    setBusy(true); setError('')
    try {
      const next = action === 'pause' ? await api.pauseHash()
        : action === 'cancel' ? await api.cancelHash() : await api.startHash(threads)
      accept(next)
    } catch (e: unknown) { setError(errorText(e)) }
    finally { setBusy(false) }
  }

  return <section className="dedup-hash" aria-label="BLAKE3 哈希任务">
    <div className="dedup-section-heading"><h3><Fingerprint size={17}/>内容哈希</h3><span>BLAKE3</span></div>
    <div className="dedup-hash-heading"><strong>{phase === 'idle' && complete ? '缓存完整' : phaseLabels[phase]}</strong>
      {total > 0 && <span>{percent}%</span>}</div>
    <progress aria-label="哈希进度" value={running && !total ? undefined : percent} max={100}/>
    <div className="dedup-hash-count">{number(done)} / {number(total)} 个文件</div>
    {knownBytes && <div className="dedup-hash-count">{size(bytes)} / {size(totalBytes)}</div>}
    {running && <dl className="dedup-speed">
      <div><dt>读取速度</dt><dd>{status?.bytes_per_second ? size(status.bytes_per_second) + '/s' : '估算中'}</dd></div>
      <div><dt>预计剩余</dt><dd>{remaining(status?.eta_seconds ?? null)}</dd></div>
    </dl>}
    <label className="dedup-threads">读取线程<select aria-label="读取线程" value={threads}
      disabled={busy || running || !status} onChange={e => {
        setThreadChoice(Number(e.target.value))
        try { localStorage.setItem(threadKey, e.target.value) } catch { /* Keep the choice for this window. */ }
      }}>
      {[1, 2, 3, 4].map(n => <option key={n} value={n}>{n} 线程</option>)}</select></label>
    <div className="dedup-job-actions">
      {running ? <button disabled={busy || settling} onClick={() => void run('pause')}><Pause size={14}/>暂停</button>
        : <button className="primary" disabled={busy || !status} onClick={() => void run('start')}>
          <Play size={14}/>{complete ? '复核文件哈希' : done || phase === 'paused' ? '继续计算' : '开始计算'}</button>}
      {(running || phase === 'paused') && <button disabled={busy || settling}
        onClick={() => void run('cancel')}><CircleStop size={14}/>取消</button>}
    </div>
    {(error || status?.error) && <p role="alert" className="dedup-error">{error || status?.error}</p>}
  </section>
}
