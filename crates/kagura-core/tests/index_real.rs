use std::{collections::BTreeMap, env, error::Error, fs, path::PathBuf, time::Instant};

use kagura_core::{
    index::{Index, SearchFilters},
    scan::scan_library_for_check,
};
use rusqlite::{Connection, OpenFlags};

fn counts(pairs: &[(&str, usize)]) -> BTreeMap<String, usize> {
    pairs.iter().map(|&(name, n)| (name.to_owned(), n)).collect()
}

#[test]
#[ignore = "set KAGURA_LIBRARY to the MMD root; the index is created in the system temp directory"]
fn real_index() -> Result<(), Box<dyn Error>> {
    let Some(root) = env::var_os("KAGURA_LIBRARY").map(PathBuf::from) else {
        return Ok(());
    };
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("index.sqlite");

    let start = Instant::now();
    let mut index = Index::open_for_library(&path, &root)?;
    let scan = scan_library_for_check(&root);
    let scanned = start.elapsed();
    index.rebuild(&scan)?;
    let rebuilt = start.elapsed();
    drop(scan);
    drop(index);
    println!(
        "SQLite {} | rebuild {:.3}s (including scan {:.3}s) | database {} bytes",
        rusqlite::version(),
        rebuilt.as_secs_f64(),
        scanned.as_secs_f64(),
        fs::metadata(&path)?.len()
    );

    let mut index = Index::open_for_library(&path, &root)?;
    let start = Instant::now();
    let delta = index.update(&root)?;
    println!(
        "unchanged update {:.3}s | {delta:?}",
        start.elapsed().as_secs_f64()
    );
    assert_eq!(
        (
            delta.inserted,
            delta.updated,
            delta.deleted,
            delta.files_changed,
            delta.files_deleted
        ),
        (0, 0, 0, 0, 0)
    );

    let stats = index.stats()?;
    println!("stats: {stats:?}");
    assert_eq!(
        stats.counts,
        counts(&[
            ("model", 2036),
            ("stage", 1358),
            ("song", 2220),
            ("motion_part", 3210)
        ])
    );
    for (kind, expected) in [("model", 1289), ("stage", 725), ("motion_part", 2898)] {
        assert_eq!(stats.unknown_author.get(kind), Some(&expected), "{kind}");
    }
    assert_eq!(stats.unknown_composer, 2206);
    for (kind, expected) in [("model", 433), ("stage", 65)] {
        assert_eq!(stats.explicit_license.get(kind), Some(&expected), "{kind}");
    }

    for query in ["ミク", "初音", "小苹果", "Tda", "ECHO"] {
        let page = index.search(query, &SearchFilters::default(), 3, 0)?;
        assert_eq!(page.used_like, query.chars().count() < 3);
        assert_eq!(page.items.len(), page.total.min(3));
        let first: Vec<_> = page.items.iter().map(|a| &a.rel_dir).collect();
        println!(
            "search {query} | {} | {} | {first:?}",
            if page.used_like { "LIKE" } else { "FTS5" },
            page.total
        );
    }

    let audit = Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let enabled: bool = audit.query_row("SELECT sqlite_compileoption_used('ENABLE_FTS5')", [], |row| {
        row.get(0)
    })?;
    let journal: String = audit.query_row("PRAGMA journal_mode", [], |row| row.get(0))?;
    assert!(rusqlite::version_number() >= 3_034_000 && enabled);
    assert_eq!(journal, "wal");
    let files: i64 = audit.query_row("SELECT COUNT(*) FROM files", [], |row| row.get(0))?;
    let prehashed: i64 = audit.query_row("SELECT COUNT(blake3) FROM files", [], |row| row.get(0))?;
    println!("inventory {files} payload files | hashes after rebuild/update {prehashed} | {journal}");
    assert_eq!(prehashed, 0);

    // Opt in separately to reading payload bytes on a real library.
    if env::var_os("KAGURA_INDEX_HASH_SAMPLE").as_deref() == Some(std::ffi::OsStr::new("1")) {
        let mut select = audit.prepare(
            "SELECT a.id FROM assets a JOIN files f ON f.asset_id = a.id WHERE a.type = ?1
             GROUP BY a.id HAVING SUM(f.size) BETWEEN 1 AND 67108864 ORDER BY a.id LIMIT 5",
        )?;
        let mut ids = Vec::new();
        for kind in ["model", "stage", "song", "motion_part"] {
            let group = select
                .query_map([kind], |row| row.get::<_, String>(0))?
                .collect::<Result<Vec<_>, _>>()?;
            assert_eq!(group.len(), 5, "{kind} hash sample");
            ids.extend(group);
        }
        let start = Instant::now();
        let mut files = 0;
        let mut bytes = 0;
        for id in &ids {
            let result = index.hash_asset(id)?;
            files += result.hashed_files;
            bytes += result.bytes_hashed;
            assert_eq!(result.cached_files, 0);
        }
        println!(
            "BLAKE3 {} assets | {files} files | {bytes} bytes | {:.3}s",
            ids.len(),
            start.elapsed().as_secs_f64()
        );
        let start = Instant::now();
        let mut cached = 0;
        for id in ids {
            let result = index.hash_asset(&id)?;
            assert_eq!((result.hashed_files, result.bytes_hashed), (0, 0));
            cached += result.cached_files;
        }
        assert_eq!(cached, files);
        println!(
            "BLAKE3 cache hit {cached} files | {:.3}s",
            start.elapsed().as_secs_f64()
        );
    }
    Ok(())
}
