import { useEffect, useRef, useState } from 'react'
import { invoke, isTauri } from '@tauri-apps/api/core'
import { useVirtualizer } from '@tanstack/react-virtual'
import type { CoreStatus as CoreStatusResult } from './generated/CoreStatus'
import './App.css'

const itemCount = 10_000
const cardWidth = 220
const gap = 12
const rowHeight = 118

const samples = [
  { type: '模型', format: 'PMX', name: '纸灯笼', color: 'model' },
  { type: '场景', format: 'PMX', name: '雨后站台', color: 'stage' },
  { type: '动作', format: 'VMD', name: '回身步', color: 'motion' },
  { type: '歌曲', format: 'WAV', name: '月下练习曲', color: 'song' },
  { type: '场景', format: 'PMX', name: '午后庭院', color: 'stage' },
  { type: '动作', format: 'VMD', name: '挥袖', color: 'motion' },
  { type: '歌曲', format: 'WAV', name: '山间小调', color: 'song' },
  { type: '模型', format: 'PMX', name: '油纸伞', color: 'model' },
] as const

function CoreStatus() {
  const desktop = isTauri()
  const [status, setStatus] = useState({ state: 'pending', text: '正在连接核心库…' })

  useEffect(() => {
    if (!desktop) return
    let active = true
    invoke<CoreStatusResult>('core_status').then(
      (text) => { if (active) setStatus({ state: 'ready', text }) },
      (error: unknown) => {
        if (active) setStatus({ state: 'error', text: `核心连接失败：${String(error)}` })
      },
    )
    return () => { active = false }
  }, [desktop])

  return (
    <p className="core-status" data-state={desktop ? status.state : 'browser'} role="status">
      <span className="status-dot" aria-hidden="true" />
      {desktop ? status.text : '浏览器预览 · 请在桌面窗口查看核心状态'}
    </p>
  )
}

function App() {
  const viewport = useRef<HTMLDivElement>(null)
  const [columns, setColumns] = useState(1)

  useEffect(() => {
    const element = viewport.current
    if (!element) return
    const resize = (width: number) => setColumns(Math.max(1, Math.floor((width + gap) / (cardWidth + gap))))
    const observer = new ResizeObserver(([entry]) => resize(entry.contentRect.width))
    observer.observe(element)
    return () => observer.disconnect()
  }, [])

  const rowCount = Math.ceil(itemCount / columns)
  // Keep the mutable virtualizer in this component so scrolling always reads the current rows.
  // eslint-disable-next-line react-hooks/incompatible-library
  const rows = useVirtualizer({
    count: rowCount,
    getScrollElement: () => viewport.current,
    estimateSize: () => rowHeight,
    overscan: 3,
  })

  return (
    <main className="library">
      <header className="masthead">
        <span className="brand" lang="en">Kagura</span>
        <CoreStatus />
      </header>
      <div className="catalog">
        <div className="catalog-head">
          <div>
            <h1 id="catalog-title">模拟资产库</h1>
            <p className="item-count">10,000 个样本</p>
          </div>
          <div className="scroll-controls" role="group" aria-label="网格定位">
            <button type="button" onClick={() => rows.scrollToIndex(0, { align: 'start' })}>跳至开头</button>
            <button type="button" onClick={() => rows.scrollToIndex(rowCount - 1, { align: 'end' })}>跳至末尾</button>
          </div>
        </div>
        <div className="viewport" ref={viewport} role="region" aria-labelledby="catalog-title" tabIndex={0}>
          <div className="virtual-list" role="list" aria-label="模拟资产条目" style={{ height: rows.getTotalSize() }}>
            {rows.getVirtualItems().map((row) => {
              const first = row.index * columns
              const count = Math.min(columns, itemCount - first)
              return (
                <div
                  className="virtual-row"
                  key={row.key}
                  role="presentation"
                  style={{
                    gridTemplateColumns: `repeat(${columns}, minmax(0, 1fr))`,
                    transform: `translateY(${row.start}px)`,
                    height: row.size,
                    gap,
                    paddingBottom: gap,
                  }}
                >
                  {Array.from({ length: count }, (_, offset) => {
                    const index = first + offset
                    const sample = samples[index % samples.length]
                    const number = String(index + 1).padStart(5, '0')
                    return (
                      <div className="asset-card" data-category={sample.color} key={index} role="listitem"
                        aria-posinset={index + 1} aria-setsize={itemCount}>
                        <span className="format-tag">{sample.format}</span>
                        <div className="asset-copy">
                          <span className="asset-type">{sample.type}</span>
                          <strong>{sample.name}</strong>
                          <span className="asset-id">#{number}</span>
                        </div>
                      </div>
                    )
                  })}
                </div>
              )
            })}
          </div>
        </div>
      </div>
    </main>
  )
}

export default App
