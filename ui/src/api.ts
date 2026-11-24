import { invoke } from '@tauri-apps/api/core'
import type { Settings } from './generated/Settings'
import type { LibraryStats } from './generated/LibraryStats'
import type { IndexChanges } from './generated/IndexChanges'
import type { UiFilters } from './generated/UiFilters'
import type { SortSpec } from './generated/SortSpec'
import type { SearchPage } from './generated/SearchPage'
import type { AssetDetail } from './generated/AssetDetail'
import type { SongChildren } from './generated/SongChildren'
import type { FolderOpened } from './generated/FolderOpened'
import type { CheckReport } from './generated/CheckReport'
import type { ApiError } from './generated/ApiError'
import type { EditPatch } from './generated/EditPatch'
import type { EditResult } from './generated/EditResult'
import type { RecentEdits } from './generated/RecentEdits'
import type { TagSuggestions } from './generated/TagSuggestions'
import type { ParseStatus } from './generated/ParseStatus'
import type { HashStatus } from './generated/HashStatus'
import type { DedupOptions } from './generated/DedupOptions'
import type { DedupSection } from './generated/DedupSection'
import type { DedupPage } from './generated/DedupPage'
import type { ExportedReport } from './generated/ExportedReport'

export const pageSize = 200
export const api = {
  settings: () => invoke<Settings>('get_settings'),
  choose: () => invoke<Settings>('choose_library'),
  open: (root: string) => invoke<LibraryStats>('open_library', { root }),
  stats: () => invoke<LibraryStats>('stats'),
  rescan: () => invoke<IndexChanges>('rescan'),
  parseStatus: () => invoke<ParseStatus>('parse_status'),
  hashStatus: () => invoke<HashStatus>('hash_status'),
  startHash: (threads: number) => invoke<HashStatus>('start_hash', { threads }),
  pauseHash: () => invoke<HashStatus>('pause_hash'),
  cancelHash: () => invoke<HashStatus>('cancel_hash'),
  duplicates: (options: DedupOptions, section: DedupSection, limit: number, offset: number) =>
    invoke<DedupPage>('dedup_page', { options, section, limit, offset }),
  exportDuplicates: (options: DedupOptions) => invoke<ExportedReport>('export_duplicates', { options }),
  search: (query: string, filters: UiFilters, sort: SortSpec, offset: number) =>
    invoke<SearchPage>('search', { query, filters, sort, limit: pageSize, offset }),
  asset: (id: string) => invoke<AssetDetail>('get_asset', { id }),
  children: (songId: string) => invoke<SongChildren>('song_children', { songId }),
  folder: (id: string) => invoke<FolderOpened>('open_folder', { id }),
  copyFolder: (id: string, rel: string) => invoke<FolderOpened>('open_copy_folder', { id, rel }),
  check: () => invoke<CheckReport>('run_check'),
  edit: (id: string, patch: EditPatch, baseHash: string) => invoke<EditResult>('edit_asset', { id, patch, baseHash }),
  undo: (entryId: string) => invoke<EditResult>('undo_edit', { entryId }),
  edits: (limit: number) => invoke<RecentEdits>('list_edits', { limit }),
  suggestTags: (query: string, limit: number) => invoke<TagSuggestions>('suggest_tags', { query, limit }),
}

function isApiError(error: unknown): error is ApiError {
  return !!error && typeof error === 'object' && 'message' in error && typeof error.message === 'string'
    && 'code' in error && typeof error.code === 'string'
}

export function isConflict(error: unknown) {
  return isApiError(error) && error.code === 'conflict'
}

export function errorText(error: unknown) {
  if (isApiError(error)) return `${error.message}（${error.code}）`
  return error instanceof Error ? error.message : String(error)
}
