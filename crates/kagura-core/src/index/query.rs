use std::collections::BTreeMap;

use rusqlite::{params_from_iter, OptionalExtension, Row};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Index, IndexError};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
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
    #[cfg_attr(feature = "bindings", ts(type = "number | null"))]
    pub rating: Option<i64>,
    pub added: Option<String>,
    pub updated: Option<String>,
    /// A verified image inside the active library. Filled by load_previews for desktop views.
    pub preview_path: Option<String>,
}

pub(super) const SUMMARY_COLUMNS: &str =
    "a.id, a.type, a.kind, a.rel_dir, a.author, a.name, a.title, a.composer,
                               a.edition, a.part, a.suffix, a.rating, a.added, a.updated";

impl AssetSummary {
    pub(super) fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
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
            preview_path: None,
        })
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[serde(rename_all = "snake_case")]
pub enum Sort {
    #[default]
    Name,
    Added,
    Updated,
    Rating,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[serde(rename_all = "snake_case")]
pub enum Missing {
    Any,
    Author,
    Composer,
    License,
}

impl Missing {
    fn predicate(self) -> &'static str {
        match self {
            Self::Any => "(a.author = '_Unknown' OR a.composer = '_Unknown' OR a.license_explicit = 0)",
            Self::Author => "a.author = '_Unknown'",
            Self::Composer => "a.composer = '_Unknown'",
            Self::License => "a.license_explicit = 0",
        }
    }
}

fn literal_contains(value: &str) -> String {
    let escaped = value
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_");
    format!("%{escaped}%")
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[serde(default)]
pub struct SearchFilters {
    pub asset_type: Option<String>,
    pub kind: Option<String>,
    pub author: Option<String>,
    pub tag: Option<String>,
    pub missing: Option<Missing>,
    pub types: Vec<String>,
    pub kinds: Vec<String>,
    pub author_contains: Option<String>,
    pub missing_any: Vec<Missing>,
    pub missing_textures: bool,
    pub nonstandard_bones: bool,
    pub sort: Sort,
    /// None sorts names ascending, dates and ratings descending.
    pub descending: Option<bool>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct SearchPage {
    pub total: usize,
    pub items: Vec<AssetSummary>,
    /// True for nonempty searches shorter than three Unicode scalar values.
    pub used_like: bool,
}

#[derive(Debug, Default, Serialize, PartialEq, Eq)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
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
    /// Existing labels most often used in this library. '%' and '_' are literal input.
    pub fn suggest_tags(&self, query: &str, limit: usize) -> Result<Vec<String>, IndexError> {
        let mut stmt = self.conn.prepare_cached(
            "SELECT tag FROM tags WHERE tag LIKE ?1 ESCAPE '\\' GROUP BY tag
             ORDER BY count(*) DESC, tag COLLATE NOCASE, tag LIMIT ?2",
        )?;
        let rows = stmt.query_map(
            rusqlite::params![literal_contains(query), i64::try_from(limit).unwrap_or(i64::MAX)],
            |row| row.get(0),
        )?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

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
                values.push(literal_contains(query));
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
        for (column, selected) in [("a.type", &filters.types), ("a.kind", &filters.kinds)] {
            if !selected.is_empty() {
                predicates.push(format!("{column} IN (SELECT value FROM json_each(?))"));
                values.push(serde_json::to_string(selected)?);
            }
        }
        if let Some(author) = filters.author_contains.as_deref().filter(|s| !s.is_empty()) {
            predicates.push("a.author LIKE ? ESCAPE '\\'".into());
            values.push(literal_contains(author));
        }
        if let Some(missing) = filters.missing {
            predicates.push(missing.predicate().into());
        }
        if !filters.missing_any.is_empty() {
            let terms: Vec<_> = filters
                .missing_any
                .iter()
                .map(|missing| missing.predicate())
                .collect();
            predicates.push(format!("({})", terms.join(" OR ")));
        }
        if filters.missing_textures {
            predicates.push("EXISTS(SELECT 1 FROM model_info m JOIN model_variants v ON v.asset_id=m.asset_id AND v.file_key=m.file_key WHERE m.asset_id=a.id AND EXISTS(SELECT 1 FROM model_textures t WHERE t.info_id=m.info_id AND t.present=0))".into());
        }
        if filters.nonstandard_bones {
            predicates.push("EXISTS(SELECT 1 FROM model_info m JOIN model_variants v ON v.asset_id=m.asset_id AND v.file_key=m.file_key WHERE m.asset_id=a.id AND m.standard_bones=0)".into());
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
