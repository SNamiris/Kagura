import { AudioLines, Box, Frame, Music2 } from 'lucide-react'
import type { AssetSummary } from './generated/AssetSummary'
import type { UiFilters } from './generated/UiFilters'
import type { SortSpec } from './generated/SortSpec'

export const types = [
  { value: 'model', label: '模型', icon: Box },
  { value: 'stage', label: '场景', icon: Frame },
  { value: 'song', label: '歌曲', icon: Music2 },
  { value: 'motion_part', label: '动作素材', icon: AudioLines },
] as const

export const kinds = [
  { parent: 'model', value: 'character', label: '角色' }, { parent: 'model', value: 'prop', label: '物品' },
  { parent: 'stage', value: 'stage', label: '一般场景' }, { parent: 'stage', value: 'skydome', label: '天空球' },
  { parent: 'stage', value: 'hdri', label: 'HDRI' },
]

export const typeOf = (value: string) => types.find(t => t.value === value) ?? types[0]
const parts: Record<string, string> = { motion: '动作', camera: '镜头', face: '表情', lip: '口型', pack: '组合包', pose: '姿势' }
export const partName = (part: string) => parts[part] ?? part
export const assetName = (asset: AssetSummary) =>
  asset.name || asset.title || (asset.part ? [partName(asset.part), asset.suffix].filter(Boolean).join(' · ') : '') || asset.rel_dir.split('/').at(-1) || asset.id
export const person = (name?: string | null) => !name || name === '_Unknown' ? '未知' : name
export const number = (value: number) => value.toLocaleString('zh-CN')
export function size(bytes: number) {
  if (!bytes) return '0 B'
  const unit = Math.min(4, Math.floor(Math.log(bytes) / Math.log(1024)))
  return `${(bytes / 1024 ** unit).toLocaleString('zh-CN', { maximumFractionDigits: 1 })} ${['B', 'KiB', 'MiB', 'GiB', 'TiB'][unit]}`
}

export const emptyFilters: UiFilters = { types: [], kinds: [], author: null, tag: null, missing: [] }
export const defaultSort: SortSpec = { field: 'name', descending: false }
export function toggle<T>(values: T[], value: T) {
  return values.includes(value) ? values.filter(v => v !== value) : [...values, value]
}
