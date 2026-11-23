use std::collections::BTreeMap;

use rusqlite::{params_from_iter, OptionalExtension, Row};
use serde::Serialize;
use serde_json::Value;

use super::{Index, IndexError};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AssetSummary {
    pub id: String,
    pub asset_type: String,
    pub kind: Option<String>,
    pub rel_dir: String,
    pub author: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub composer: Option<String>,
    pub edition: Option<String>,
    pub part: Option<String>,
    pub suffix: Option<String>,
    pub rating: Option<i64>,
    pub added: Option<String>,
    pub updated: Option<String>,
}

const SUMMARY_COLUMNS: &str = "a.id, a.type, a.kind, a.rel_dir, a.author, a.name, a.title, a.composer,
                               a.edition, a.part, a.suffix, a.rating, a.added, a.updated";

impl AssetSummary {
    fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            asset_type: row.get("type")?,
            kind: row.get("kind")?,
            rel_dir: row.get("rel_dir")?,
            author: row.get("author")?,
            name: row.get("name")?,
            title: row.get("title")?,
            composer: row.get("composer")?,
            edition: row.get("edition")?,
            part: row.get("part")?,
            suffix: row.get("suffix")?,
            rating: row.get("rating")?,
            added: row.get("added")?,
            updated: row.get("updated")?,
        })
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum Sort {
    #[default]
    Name,
    Added,
    Updated,
    Rating,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Missing {
    Any,
    Author,
    Composer,
    License,
}

#[derive(Debug, Clone, Default)]
pub struct SearchFilters {
    pub asset_type: Option<String>,
    pub kind: Option<String>,
    pub author: Option<String>,
    pub tag: Option<String>,
    pub missing: Option<Missing>,
    pub sort: Sort,
    /// None sorts names ascending, dates and ratings descending.
    pub descending: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct SearchPage {
    pub total: usize,
    pub items: Vec<AssetSummary>,
    /// True for nonempty searches shorter than three Unicode scalar values.
    pub used_like: bool,
}

#[derive(Debug, Default, Serialize, PartialEq, Eq)]
pub struct IndexStats {
    pub counts: BTreeMap<String, usize>,
    pub unknown_author: BTreeMap<String, usize>,
    pub unknown_composer: usize,
    pub explicit_license: BTreeMap<String, usize>,
}

fn count(row: &Row<'_>, column: usize) -> rusqlite::Result<usize> {
    let n: i64 = row.get(column)?;
    usize::try_from(n).map_err(|_| rusqlite::Error::IntegralValueOutOfRange(column, n))
}

const SEARCH_FIELDS: [&str; 9] = [
    "name",
    "name_original",
    "author",
    "character",
    "series",
    "title",
    "title_alias",
    "tags",
    "note",
];

impl Index {
    /// Search for a literal substring. An empty query browses all matching assets.
    pub fn search(
        &self,
        query: &str,
        filters: &SearchFilters,
        limit: usize,
        offset: usize,
    ) -> Result<SearchPage, IndexError> {
        if query.contains('\0') {
            return Err(IndexError::Invalid("搜索词不能含有 NUL".into()));
        }
        let mut from = String::from("FROM assets a");
        let mut predicates: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();
        let used_like = !query.is_empty() && query.chars().take(3).count() < 3;
        if !query.is_empty() {
            from.push_str(" JOIN assets_fts ON assets_fts.rowid = a.rowid");
            if used_like {
                let escaped = query
                    .replace('\\', "\\\\")
                    .replace('%', "\\%")
                    .replace('_', "\\_");
                values.push(format!("%{escaped}%"));
                let matches = SEARCH_FIELDS.map(|field| format!("assets_fts.{field} LIKE ?1 ESCAPE '\\'"));
                predicates.push(format!("({})", matches.join(" OR ")));
            } else {
                values.push(format!("\"{}\"", query.replace('"', "\"\"")));
                predicates.push("assets_fts MATCH ?".into());
            }
        }
        for (predicate, value) in [
            ("a.type = ?", &filters.asset_type),
            ("a.kind = ?", &filters.kind),
            ("a.author = ?", &filters.author),
            (
                "EXISTS (SELECT 1 FROM tags t WHERE t.id = a.id AND t.tag = ?)",
                &filters.tag,
            ),
        ] {
            if let Some(value) = value {
                predicates.push(predicate.into());
                values.push(value.clone());
            }
        }
        if let Some(missing) = filters.missing {
            let predicate = match missing {
                Missing::Any => {
                    "(a.author = '_Unknown' OR a.composer = '_Unknown' OR a.license_explicit = 0)"
                }
                Missing::Author => "a.author = '_Unknown'",
                Missing::Composer => "a.composer = '_Unknown'",
                Missing::License => "a.license_explicit = 0",
            };
            predicates.push(predicate.into());
        }
        if !predicates.is_empty() {
            from.push_str(" WHERE ");
            from.push_str(&predicates.join(" AND "));
        }
        let order = match filters.sort {
            Sort::Name => "COALESCE(a.name, a.title, a.part, a.rel_dir) COLLATE NOCASE",
            Sort::Added => "a.added",
            Sort::Updated => "a.updated",
            Sort::Rating => "a.rating",
        };
        let descending = filters.descending.unwrap_or(filters.sort != Sort::Name);
        let direction = if descending { "DESC" } else { "ASC" };
        // Keep both reads on the same WAL snapshot when another connection updates the index.
        let tx = self.conn.unchecked_transaction()?;
        let total = tx.query_row(
            &format!("SELECT count(*) {from}"),
            params_from_iter(&values),
            |row| count(row, 0),
        )?;
        let limit = i64::try_from(limit).unwrap_or(i64::MAX);
        let offset = i64::try_from(offset).unwrap_or(i64::MAX);
        let items = tx
            .prepare(&format!(
                "SELECT {SUMMARY_COLUMNS} {from} ORDER BY {order} {direction} NULLS LAST, a.id
             LIMIT {limit} OFFSET {offset}"
            ))?
            .query_map(params_from_iter(&values), AssetSummary::from_row)?
            .collect::<rusqlite::Result<_>>()?;
        tx.commit()?;
        Ok(SearchPage {
            total,
            items,
            used_like,
        })
    }

    /// The complete cached manifest, in its original key order.
    pub fn get(&self, id: &str) -> Result<Option<Value>, IndexError> {
        let text: Option<String> = self
            .conn
            .query_row("SELECT manifest_json FROM assets WHERE id = ?", [id], |row| {
                row.get(0)
            })
            .optional()?;
        Ok(text.map(|json| serde_json::from_str(&json)).transpose()?)
    }

    /// All motion_part records whose key references this song.
    pub fn children(&self, song_id: &str) -> Result<Vec<AssetSummary>, IndexError> {
        let mut statement = self.conn.prepare(&format!(
            "SELECT {SUMMARY_COLUMNS} FROM assets a WHERE a.type = 'motion_part' AND a.song_id = ?
             ORDER BY a.part, a.rel_dir, a.id"
        ))?;
        let rows = statement.query_map([song_id], AssetSummary::from_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// License figures count explicit grant/deny flags (commercial, modify or redistribute).
    pub fn stats(&self) -> Result<IndexStats, IndexError> {
        let mut statement = self.conn.prepare(
            "SELECT type, count(*), count(*) FILTER (WHERE author = '_Unknown'),
             count(*) FILTER (WHERE composer = '_Unknown'), sum(license_explicit)
             FROM assets GROUP BY type",
        )?;
        let mut stats = IndexStats::default();
        let mut rows = statement.query([])?;
        while let Some(row) = rows.next()? {
            let asset_type: String = row.get(0)?;
            stats.counts.insert(asset_type.clone(), count(row, 1)?);
            stats.unknown_author.insert(asset_type.clone(), count(row, 2)?);
            stats.unknown_composer += count(row, 3)?;
            stats.explicit_license.insert(asset_type, count(row, 4)?);
        }
        Ok(stats)
    }
}
