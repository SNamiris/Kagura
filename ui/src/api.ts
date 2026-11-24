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

export const pageSize = 200
export const api = {
  settings: () => invoke<Settings>('get_settings'),
  choose: () => invoke<Settings>('choose_library'),
  open: (root: string) => invoke<LibraryStats>('open_library', { root }),
  stats: () => invoke<LibraryStats>('stats'),
  rescan: () => invoke<IndexChanges>('rescan'),
  search: (query: string, filters: UiFilters, sort: SortSpec, offset: number) =>
    invoke<SearchPage>('search', { query, filters, sort, limit: pageSize, offset }),
  asset: (id: string) => invoke<AssetDetail>('get_asset', { id }),
  children: (songId: string) => invoke<SongChildren>('song_children', { songId }),
  folder: (id: string) => invoke<FolderOpened>('open_folder', { id }),
  check: () => invoke<CheckReport>('run_check'),
}

function isApiError(error: unknown): error is ApiError {
  return !!error && typeof error === 'object' && 'message' in error && typeof error.message === 'string'
    && 'code' in error && typeof error.code === 'string'
}

export function errorText(error: unknown) {
  if (isApiError(error)) return `${error.message}（${error.code}）`
  return error instanceof Error ? error.message : String(error)
}
