use kagura_core::index::{Index, SearchFilters};
use rusqlite::{Connection, OpenFlags};
use std::{env, error::Error, path::PathBuf, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let root = PathBuf::from(
        args.next()
            .ok_or("usage: parse_library <library root> [index.sqlite]")?,
    );
    let database = match args.next() {
        Some(p) => PathBuf::from(p),
        None => Index::default_path()?,
    };
    if args.next().is_some() {
        return Err("unexpected argument".into());
    }
    let mut index = Index::open_for_library(&database, &root)?;
    let start = Instant::now();
    index.update(&root)?;
    println!("inventory_seconds={:.3}", start.elapsed().as_secs_f64());
    let start = Instant::now();
    let mut next = 0;
    let result = index.parse_files(&root, |p| {
        if p.processed >= next {
            eprintln!(
                "parsed={} cached={} total={} failures={}",
                p.processed, p.cached, p.total, p.failed
            );
            next = p.processed + 1024;
        }
        true
    })?;
    println!(
        "parse_seconds={:.3} processed={} cached={} total={} failed={}",
        start.elapsed().as_secs_f64(),
        result.processed,
        result.cached,
        result.total,
        result.failed
    );
    let db = Connection::open_with_flags(&database, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let mut counts = db.prepare(
        "WITH results AS (SELECT format,parse_error FROM model_info UNION ALL SELECT 'VMD',parse_error FROM motion_info)
         SELECT format,count(*),sum(parse_error IS NULL),sum(parse_error IS NOT NULL) FROM results GROUP BY format ORDER BY format",
    )?;
    let mut rows = counts.query([])?;
    while let Some(row) = rows.next()? {
        println!(
            "{} total={} ok={} failed={}",
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, i64>(3)?
        );
    }
    let mut groups = db.prepare(
        "WITH results AS (
         SELECT format,asset_id,file,parse_error FROM model_info WHERE parse_error IS NOT NULL UNION ALL
         SELECT 'VMD',asset_id,file,parse_error FROM motion_info WHERE parse_error IS NOT NULL
         ), classified AS (
         SELECT format,json_extract(parse_error,'$.section') AS section,parse_error,rel_dir || '/' || file AS path,
         row_number() OVER(PARTITION BY format,json_extract(parse_error,'$.section') ORDER BY rel_dir,file) AS n,
         count(*) OVER(PARTITION BY format,json_extract(parse_error,'$.section')) AS failures FROM results r JOIN assets a ON a.id=r.asset_id)
         SELECT format,section,failures,path,parse_error FROM classified WHERE n=1 ORDER BY format,failures DESC,section",
    )?;
    let mut rows = groups.query([])?;
    while let Some(row) = rows.next()? {
        println!(
            "error format={} section={} count={} example={} {}",
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?
        );
    }
    for missing in [true, false] {
        let filters = SearchFilters {
            missing_textures: missing,
            nonstandard_bones: !missing,
            ..Default::default()
        };
        let label = if missing {
            "missing_textures"
        } else {
            "nonstandard_bones"
        };
        let condition = if missing {
            "EXISTS(SELECT 1 FROM model_textures t WHERE t.info_id=m.info_id AND t.present=0)"
        } else {
            "standard_bones=0"
        };
        let sql = format!(
            "SELECT count(DISTINCT m.asset_id) FROM model_info m JOIN assets a ON a.id=m.asset_id
             WHERE a.type='model' AND {condition}"
        );
        let all_files: i64 = db.query_row(&sql, [], |r| r.get(0))?;
        println!(
            "{} variants={} all_model_payloads={}",
            label,
            index.search("", &filters, 0, 0)?.total,
            all_files
        );
    }
    Ok(())
}
