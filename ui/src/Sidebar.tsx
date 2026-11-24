import { FolderOpen, ListFilter, RotateCcw } from 'lucide-react'
import type { UiFilters } from './generated/UiFilters'
import type { LibraryStats } from './generated/LibraryStats'
import { emptyFilters, kinds, number, toggle, types } from './presentation'

export function Sidebar({ filters, stats, onChange, choose, busy }: {
  filters: UiFilters, stats: LibraryStats, onChange: (next: UiFilters) => void, choose: () => void, busy: boolean
}) {
  const changed = JSON.stringify(filters) !== JSON.stringify(emptyFilters)
  function selectType(type: string) {
    const selected = toggle(filters.types, type)
    const available = kinds.filter(k => !selected.length || selected.includes(k.parent))
    onChange({ ...filters, types: selected, kinds: filters.kinds.filter(k => available.some(item => item.value === k)) })
  }
  return <aside className="filters" aria-label="资产筛选">
    <div className="section-title"><h2><ListFilter size={16} />筛选</h2>
      {changed && <button className="icon-button" title="清除筛选" aria-label="清除筛选" onClick={() => onChange(emptyFilters)}><RotateCcw size={15} /></button>}
    </div>
    <div className="filter-scroll">
      <fieldset><legend>资产类型</legend>
        {types.map(({ value, label, icon: Icon }) => <label key={value} className="type-filter" data-type={value}>
          <input type="checkbox" aria-label={label} checked={filters.types.includes(value)} onChange={() => selectType(value)} />
          <Icon size={15} /><span>{label}</span><small>{number(stats.counts[value] ?? 0)}</small>
        </label>)}
      </fieldset>
      {(!filters.types.length || filters.types.includes('model') || filters.types.includes('stage')) &&
        <fieldset><legend>细分类别</legend><div className="kind-group">
          {kinds.filter(k => !filters.types.length || filters.types.includes(k.parent)).map(k =>
            <button key={k.value} aria-pressed={filters.kinds.includes(k.value)} onClick={() => onChange({ ...filters, kinds: toggle(filters.kinds, k.value) })}>{k.label}</button>)}
        </div></fieldset>}
      <fieldset className="field-pair"><legend>作者与标签</legend>
        <label><span>作者</span><input value={filters.author ?? ''} placeholder="输入作者名" onChange={e => onChange({ ...filters, author: e.target.value || null })} /></label>
        <label><span>标签</span><input value={filters.tag ?? ''} placeholder="输入完整标签" onChange={e => onChange({ ...filters, tag: e.target.value || null })} /></label>
      </fieldset>
      <fieldset><legend>缺失项</legend>
        {([['author','作者不明'],['composer','作曲者不明'],['license','授权不明']] as const).map(([value,label]) =>
          <label key={value} className="check-label"><input type="checkbox" checked={filters.missing.includes(value)}
            onChange={() => onChange({ ...filters, missing: toggle(filters.missing, value) })} />{label}</label>)}
      </fieldset>
      <fieldset><legend>模型解析</legend>
        <label className="check-label"><input type="checkbox" checked={filters.missing_textures}
          onChange={e => onChange({ ...filters, missing_textures: e.target.checked })}/>缺失贴图</label>
        <label className="check-label"><input type="checkbox" checked={filters.nonstandard_bones}
          onChange={e => onChange({ ...filters, nonstandard_bones: e.target.checked })}/>非标准骨骼（角色）</label>
      </fieldset>
    </div>
    <button className="switch-root" onClick={choose} disabled={busy}><FolderOpen size={16} />切换库文件夹</button>
  </aside>
}
