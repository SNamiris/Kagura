CREATE TABLE assets (
    id TEXT PRIMARY KEY NOT NULL,
    type TEXT NOT NULL,
    kind TEXT,
    rel_dir TEXT NOT NULL UNIQUE,
    author TEXT, name TEXT, name_original TEXT, title TEXT, composer TEXT, edition TEXT,
    part TEXT, suffix TEXT, song_id TEXT, character TEXT, series TEXT,
    rating INTEGER, added TEXT, updated TEXT, license_explicit INTEGER NOT NULL,
    manifest_mtime INTEGER NOT NULL, manifest_size INTEGER NOT NULL, manifest_json TEXT NOT NULL
);
CREATE INDEX assets_by_song ON assets(song_id);
CREATE INDEX assets_by_type ON assets(type);

CREATE TABLE refs (
    from_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    to_id TEXT NOT NULL,
    field TEXT NOT NULL,
    PRIMARY KEY(from_id,field)
) WITHOUT ROWID;
CREATE INDEX refs_by_target ON refs(to_id);
CREATE TABLE tags (
    id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    tag TEXT NOT NULL,
    PRIMARY KEY(id,tag)
) WITHOUT ROWID;
CREATE INDEX tags_by_tag ON tags(tag,id);
CREATE TABLE files (
    asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    rel TEXT NOT NULL, size INTEGER NOT NULL, mtime INTEGER NOT NULL, blake3 TEXT,
    PRIMARY KEY(asset_id,rel)
) WITHOUT ROWID;

CREATE VIRTUAL TABLE assets_fts USING fts5(
    name, name_original, author, character, series, title, title_alias, tags, note, tokenize='trigram'
);
CREATE TABLE meta (
    id INTEGER PRIMARY KEY CHECK(id=1),
    root TEXT NOT NULL, last_scan INTEGER NOT NULL, asset_count INTEGER NOT NULL, file_count INTEGER NOT NULL
);
CREATE TABLE skipped_manifests (
    path TEXT PRIMARY KEY, mtime INTEGER NOT NULL, size INTEGER NOT NULL
) WITHOUT ROWID;
