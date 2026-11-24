import { useEffect, useState } from 'react'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { api, errorText } from './api'
import type { ParseStatus } from './generated/ParseStatus'

const terminal = (s: ParseStatus) => s.phase === 'complete' || s.phase === 'failed'

export function useParseStatus(root: string | undefined, scanCycle: number) {
  const [view, setView] = useState<{
    root: string, status: ParseStatus | null, revision: number, error: string | null,
  }>()
  useEffect(() => {
    if (!root) return
    let active = true
    let stop: UnlistenFn | undefined
    function accept(status: ParseStatus) {
      if (!active || !root || status.root !== root) return
      setView(prev => {
        const old = prev?.root === root ? prev.status : null
        if (old && old.generation > status.generation) return prev
        if (old && old.generation === status.generation) {
          if (terminal(old) || (old.phase === 'running' && status.phase === 'idle')) return prev
          if (!terminal(status) && status.cached + status.processed < old.cached + old.processed) return prev
        }
        return { root, status, revision: (prev?.root === root ? prev.revision : 0) + Number(terminal(status)), error: null }
      })
    }
    async function connect() {
      const unlisten = await listen<ParseStatus>('parse-progress', event => accept(event.payload))
      if (!active) { unlisten(); return }
      stop = unlisten
      // A short task may finish before the listener was installed.
      accept(await api.parseStatus())
    }
    void connect().catch((e: unknown) => {
      if (active) setView(prev => ({ root, status: null, revision: prev?.root === root ? prev.revision : 0, error: errorText(e) }))
    })
    return () => { active = false; stop?.() }
  }, [root, scanCycle])
  return view?.root === root ? view : undefined
}
