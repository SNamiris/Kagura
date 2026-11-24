import { memo, useCallback, useEffect, useRef, useState } from 'react'
import { useVirtualizer } from '@tanstack/react-virtual'
import { convertFileSrc } from '@tauri-apps/api/core'
import { ArrowDownWideNarrow, ArrowUpNarrowWide, Grid2X2, List, SearchX, Star } from 'lucide-react'
import { api, errorText, pageSize } from './api'
import type { SearchPage } from './generated/SearchPage'
import type { AssetSummary } from './generated/AssetSummary'
import type { UiFilters } from './generated/UiFilters'
import type { SortSpec } from './generated/SortSpec'
import { assetName, number, person, typeOf } from './presentation'

function Thumbnail({ asset }: { asset: AssetSummary }) {
  const [failed, setFailed] = useState(false)
  const Icon = typeOf(asset.asset_type).icon
  return <span className="thumbnail" aria-hidden="true">
    <Icon className="thumb-symbol" strokeWidth={1.3} />
    {asset.preview_path && !failed && <img src={convertFileSrc(asset.preview_path)} alt="" loading="lazy" draggable="false" onError={() => setFailed(true)} />}
  </span>
}

const AssetCard = memo(function AssetCard({ asset, selected, open }: { asset: AssetSummary, selected: boolean, open: (id: string) => void }) {
  const category = typeOf(asset.asset_type)
  const title = assetName(asset)
  return <button type="button" className="asset-card" data-type={asset.asset_type} data-asset-id={asset.id}
    aria-label={`${category.label}：${title}`} aria-pressed={selected} onClick={() => open(asset.id)} title={title}>
    <Thumbnail asset={asset} />
    <span className="card-copy">
      <span className="card-meta"><span className="type-label">{category.label}</span>
        <span className="rating" title={asset.rating ? `${asset.rating} / 5` : '未评分'}><Star size={12}/>{asset.rating || '未评'}</span></span>
      <strong>{title}</strong><span className="card-author">{person(asset.author || asset.composer)}</span>
    </span>
  </button>
})

export function Catalog({ query, filters, sort, setSort, selected, open, view, setView, parsingMatches }: {
  query: string, filters: UiFilters, sort: SortSpec, setSort: (s: SortSpec) => void,
  selected: string | null, open: (id: string) => void, view: 'grid' | 'list', setView: (v: 'grid' | 'list') => void, parsingMatches: boolean,
}) {
  const viewport = useRef<HTMLDivElement>(null)
  const [width, setWidth] = useState(640)
  const [pages, setPages] = useState<Record<number, SearchPage>>({})
  const [error, setError] = useState('')
  const fetched = useRef(new Set<number>())
  const active = useRef(false)
  const pending = useRef(0)
  const [loading, setLoading] = useState(false)
  const [attempt, setAttempt] = useState(0)
  const total = pages[0]?.total ?? 0
  const columns = view === 'list' ? 1 : Math.max(1, Math.floor((width + 12) / 169))
  const rowHeight = view === 'list' ? 64 : 172
  // TanStack Virtual exposes a mutable scroll instance. Read the range at render time.
  // eslint-disable-next-line react-hooks/incompatible-library
  const rows = useVirtualizer({ count: Math.ceil(total / columns), getScrollElement: () => viewport.current,
    estimateSize: () => rowHeight, overscan: 3 })
  const visible = rows.getVirtualItems()
  const firstRow = visible[0]?.index ?? 0
  const lastRow = visible.at(-1)?.index ?? 0
  const bufferRows = 2 * Math.ceil((rows.scrollRect?.height ?? rowHeight) / rowHeight)
  const finalPage = Math.max(0, Math.ceil(total / pageSize) - 1)
  const pageForRow = (row: number) => Math.min(finalPage, Math.floor(Math.max(0, row) * columns / pageSize))
  const firstPage = pageForRow(firstRow)
  const lastPage = Math.min(finalPage, Math.floor(((lastRow + 1) * columns - 1) / pageSize))
  const bufferStart = pageForRow(firstRow - bufferRows)
  const bufferEnd = Math.min(finalPage, Math.floor(((lastRow + bufferRows + 1) * columns - 1) / pageSize))

  const load = useCallback((page: number) => {
    if (fetched.current.has(page)) return
    fetched.current.add(page)
    pending.current++
    setLoading(true)
    api.search(query, filters, sort, page * pageSize).then(result => {
      if (active.current) setPages(old => ({ ...old, [page]: result }))
    }).catch((reason: unknown) => {
      fetched.current.delete(page)
      if (active.current) setError(errorText(reason))
    }).finally(() => {
      pending.current--
      if (active.current && !pending.current) setLoading(false)
    })
  }, [query, filters, sort])

  useEffect(() => {
    active.current = true
    const element = viewport.current
    if (!element) return
    const observer = new ResizeObserver(([entry]) => setWidth(entry.contentRect.width))
    observer.observe(element)
    return () => { active.current = false; observer.disconnect() }
  }, [])
  useEffect(() => {
    // On a fast jump, prioritize visible pages before prefetching the nearby range.
    const frame = requestAnimationFrame(() => {
      for (let page = firstPage; page <= lastPage; page++) load(page)
      for (let page = bufferStart; page < firstPage; page++) load(page)
      for (let page = lastPage + 1; page <= bufferEnd; page++) load(page)
    })
    return () => cancelAnimationFrame(frame)
  }, [firstPage, lastPage, bufferStart, bufferEnd, load, attempt])
  useEffect(() => { rows.measure() }, [rows, rowHeight, columns])

  return <section className="catalog" aria-label="资产浏览">
    <div className="catalog-heading"><div className="catalog-title"><h1>资产库</h1>
      <span className="result-count" data-testid="result-count">{pages[0] ? `${number(total)} 条结果` : '正在查询…'}</span></div>
      <div className="view-tools"><label className="sort-field"><span className="sr-only">排序字段</span>
        <select value={sort.field} onChange={e => { const field = e.target.value as SortSpec['field']; setSort({ field, descending: field !== 'name' }) }}>
          <option value="name">名称</option><option value="added">添加日期</option><option value="updated">更新日期</option><option value="rating">评分</option>
        </select></label>
        <button className="icon-button" title={sort.descending ? '降序，点击改为升序' : '升序，点击改为降序'} aria-label="切换排序方向" onClick={() => setSort({ ...sort, descending: !sort.descending })}>
          {sort.descending ? <ArrowDownWideNarrow size={16}/> : <ArrowUpNarrowWide size={16}/>}</button>
        <div className="view-toggle"><button title="网格视图" aria-label="网格视图" aria-pressed={view === 'grid'} onClick={() => setView('grid')}><Grid2X2 size={15}/></button>
          <button title="列表视图" aria-label="列表视图" aria-pressed={view === 'list'} onClick={() => setView('list')}><List size={17}/></button></div>
      </div></div>
    {error && <div className="error-box" role="alert">{error}<button onClick={() => { setError(''); setAttempt(a => a + 1) }}>重试</button></div>}
    {pages[0]?.used_like && <p className="search-note">短词匹配 · LIKE</p>}
    {parsingMatches && <p className="search-note">后台正在解析文件，筛选结果将在完成后刷新。</p>}
    <div className={'viewport ' + view} ref={viewport} aria-label="资产结果视口" data-testid="asset-viewport" role="region" tabIndex={0}
      aria-busy={loading} data-loaded-pages={Object.keys(pages).length}>
      {pages[0] && !total && <div className="empty-results"><SearchX size={28}/><h3>{parsingMatches ? '已解析的文件中暂无匹配项' : '没有找到资产'}</h3>
        {!parsingMatches && <p>试试其他关键词，或清除筛选条件。</p>}</div>}
      <div className="virtual-list" role="list" style={{ height: rows.getTotalSize() }}>
        {visible.map(row => <div className="virtual-row" key={row.key} role="presentation"
          style={{ height: rowHeight, transform: `translateY(${row.start}px)`, gridTemplateColumns: `repeat(${columns}, minmax(0, 1fr))` }}>
          {Array.from({ length: Math.min(columns, total - row.index * columns) }, (_, col) => {
            const index = row.index * columns + col
            const asset = pages[Math.floor(index / pageSize)]?.items[index % pageSize]
            return <div key={asset?.id ?? index} role="listitem" aria-posinset={index + 1} aria-setsize={total}>
              {asset ? <AssetCard asset={asset} selected={asset.id === selected} open={open} /> : <div className="card-placeholder" data-placeholder aria-label="正在载入资产" />}
            </div>
          })}
        </div>)}
      </div>
    </div>
  </section>
}
