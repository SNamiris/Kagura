import { useState } from 'react'
import { ChevronDown, FolderOpen, FolderSymlink } from 'lucide-react'
import type { DedupPage } from './generated/DedupPage'
import type { DedupSection } from './generated/DedupSection'
import type { DedupSummary } from './generated/DedupSummary'
import type { DuplicateAsset } from './generated/DuplicateAsset'
import { number, size } from './presentation'

const perExpansion = 24
const scopeNames = { cross: '跨资产重复', within: '同一资产内重复' } as const
const exclusionNames: Record<string, string> = { small: '小文件', toon: '共用 toon', accessory: '场景配件' }

export function DedupOverview({ info, browse }: { info: DedupSummary, browse: (section: DedupSection) => void }) {
  const [showAll, setShowAll] = useState(false)
  return <div className="dedup-overview">
    <dl className="dedup-main-stats">
      <div><dt>可回收空间</dt><dd>{size(info.all.reclaim_bytes)}</dd></div>
      <div><dt>重复内容组</dt><dd>{number(info.all.groups)}</dd></div>
      <div><dt>组内文件</dt><dd>{number(info.all.files)}</dd></div>
    </dl>
    <section aria-label="重复范围" className="dedup-scopes">
      {(['cross','within'] as const).map(scope => <button key={scope} onClick={() => browse(scope)}>
        <span><strong>{scopeNames[scope]}</strong><small>{number(info[scope].groups)} 组 · {number(info[scope].files)} 个文件</small></span>
        <b>{size(info[scope].reclaim_bytes)}</b></button>)}
    </section>
    <section className="dedup-matches"><h3>资产关系</h3>
      <div className="dedup-asset-links">
        <button onClick={() => browse('identical')}>整份相同 <strong>{number(info.identical_groups)} 组</strong>
          <span>共 {number(info.identical_assets)} 份资产</span></button>
        <button onClick={() => browse('subsets')}>严格子集 <strong>{number(info.subset_pairs)} 对</strong></button>
      </div>
    </section>
    <section className="dedup-extensions"><h3>按扩展名分布</h3>
      {!info.extensions.length ? <p className="dedup-empty">当前范围内没有重复文件。</p> :
        <table><thead><tr><th>扩展名</th><th>涉及内容组</th><th>文件数</th><th>可回收空间</th></tr></thead>
          <tbody>{info.extensions.slice(0, showAll ? undefined : 10).map(ext =>
            <tr key={ext.extension}><th scope="row">{ext.extension ? '.' + ext.extension : '无扩展名'}</th>
              <td>{number(ext.groups)}</td><td>{number(ext.files)}</td><td>{size(ext.reclaim_bytes)}</td></tr>)}</tbody></table>}
      {info.extensions.length > 10 && <button className="dedup-link" onClick={() => setShowAll(v => !v)}>
        {showAll ? '收起扩展名' : '查看全部 ' + number(info.extensions.length) + ' 种扩展名'}</button>}
    </section>
  </div>
}

type OpenAsset = (id: string, file?: string) => void
function AssetPath({ asset, file, exclusions, open }: { asset: DuplicateAsset, file?: string, exclusions?: string[], open: OpenAsset }) {
  const label = asset.name || asset.rel_dir.split('/').at(-1) || asset.id
  const path = file ? asset.rel_dir + '/' + file : asset.rel_dir
  const actionLabel = file ? '打开所在文件夹' : '打开资产目录'
  return <div className="dedup-asset-path"><div><strong>{label}</strong>
    {!!exclusions?.length && <span className="dedup-reason">{exclusions.map(code => exclusionNames[code] || code).join('、')}</span>}
    <small>{path}</small></div>
    <button className="icon-button" aria-label={actionLabel + ' ' + (file || label)} title={actionLabel}
      onClick={() => open(asset.id, file)}><FolderOpen size={16}/></button></div>
}

function More({ total, shown, more }: { total: number, shown: number, more: () => void }) {
  return shown < total && <button className="dedup-link" onClick={more}>
    再显示 {number(Math.min(total - shown, perExpansion))} 项（剩余 {number(total - shown)} 项）</button>
}

function FileGroup({ group, open }: { group: DedupPage['file_groups'][number], open: OpenAsset }) {
  const [shown, setShown] = useState(perExpansion)
  const label = group.copies[0]?.rel.split('/').at(-1) || group.hash
  return <details className="dedup-row">
    <summary><ChevronDown size={15}/><span className="dedup-row-name"><b>{label}</b><small>单份 {size(group.size)} · {number(group.copies.length)} 个副本</small></span>
      <strong title="该组可回收空间">{size(group.reclaim_bytes)}</strong></summary>
    <div className="dedup-row-body"><p className="dedup-digest">BLAKE3 <code>{group.hash}</code></p>
      {group.copies.slice(0, shown).map(copy =>
        <AssetPath key={JSON.stringify([copy.asset.id, copy.rel])} asset={copy.asset} file={copy.rel} exclusions={copy.exclusions} open={open}/>)}
      <More total={group.copies.length} shown={shown} more={() => setShown(n => n + perExpansion)}/>
    </div>
  </details>
}

function EqualGroup({ group, open }: { group: DedupPage['asset_groups'][number], open: OpenAsset }) {
  const [shown, setShown] = useState(perExpansion)
  return <details className="dedup-row">
    <summary><ChevronDown size={15}/><span className="dedup-row-name"><b>{group.assets[0]?.name || group.assets[0]?.rel_dir}</b>
      <small>{number(group.unique_files)} 种文件内容 · 去重后 {size(group.unique_bytes)}</small></span>
      <strong>{number(group.assets.length)} 份资产</strong></summary>
    <div className="dedup-row-body">{group.assets.slice(0, shown).map(asset =>
      <AssetPath key={asset.id} asset={asset} open={open}/>)}
      <More total={group.assets.length} shown={shown} more={() => setShown(n => n + perExpansion)}/>
    </div>
  </details>
}

export function DedupResults({ section, data, open }: { section: DedupSection, data: DedupPage, open: OpenAsset }) {
  const relation = section === 'identical' || section === 'subsets'
  return <div className="dedup-results">
    <p className="dedup-results-note">{relation
      ? '仅比较已完整哈希的非空资产，按不同文件内容组成的集合匹配。'
      : '按可回收空间排序。展开内容组可查看所有副本与所属资产。'}</p>
    {!data.total && <p className="dedup-empty">当前范围内没有匹配的{relation ? '资产关系' : '重复内容组'}。</p>}
    {data.file_groups.map(group => <FileGroup key={group.hash} group={group} open={open}/>)}
    {data.asset_groups.map(group => <EqualGroup key={group.assets[0].id} group={group} open={open}/>)}
    {data.subsets.map(pair => <article key={pair.smaller.id + '/' + pair.larger.id} className="dedup-subset">
      <div><span>子集 · {number(pair.smaller_files)} 种内容</span><AssetPath asset={pair.smaller} open={open}/></div>
      <FolderSymlink size={18} aria-hidden="true"/>
      <div><span>包含它的资产 · {number(pair.larger_files)} 种内容</span><AssetPath asset={pair.larger} open={open}/></div>
    </article>)}
  </div>
}
