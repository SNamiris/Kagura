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

-- A row records one attempt at the current files.size/mtime, including decode failures.
CREATE TABLE model_info (
    info_id INTEGER PRIMARY KEY, asset_id TEXT NOT NULL, file TEXT NOT NULL, file_key TEXT NOT NULL, format TEXT NOT NULL,
    version REAL, encoding TEXT, name TEXT, name_en TEXT, comment TEXT, comment_en TEXT,
    vertices INTEGER, faces INTEGER, materials INTEGER, bones INTEGER, morphs INTEGER,
    rigid_bodies INTEGER, joints INTEGER, standard_bones INTEGER, has_physics INTEGER,
    parse_error TEXT,
    UNIQUE(asset_id,file), FOREIGN KEY(asset_id,file) REFERENCES files(asset_id,rel) ON DELETE CASCADE
);
CREATE TABLE model_bones (
    info_id INTEGER REFERENCES model_info ON DELETE CASCADE, ordinal INTEGER NOT NULL, name TEXT NOT NULL,
    PRIMARY KEY(info_id,ordinal)
) WITHOUT ROWID;
CREATE TABLE model_morphs (
    info_id INTEGER REFERENCES model_info ON DELETE CASCADE, ordinal INTEGER NOT NULL, name TEXT NOT NULL,
    PRIMARY KEY(info_id,ordinal)
) WITHOUT ROWID;
CREATE TABLE model_textures (
    info_id INTEGER REFERENCES model_info ON DELETE CASCADE, ordinal INTEGER NOT NULL, name TEXT NOT NULL,
    resolved_path TEXT, present INTEGER NOT NULL,
    PRIMARY KEY(info_id,ordinal)
) WITHOUT ROWID;
CREATE INDEX texture_targets ON model_textures(resolved_path);
CREATE INDEX models_with_missing_textures ON model_textures(info_id) WHERE present=0;
CREATE TABLE model_variants (
    asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE, file_key TEXT NOT NULL,
    PRIMARY KEY(asset_id,file_key)
) WITHOUT ROWID;
CREATE TABLE motion_info (
    info_id INTEGER PRIMARY KEY, asset_id TEXT NOT NULL, file TEXT NOT NULL, target_model TEXT, is_camera INTEGER,
    max_frame INTEGER, seconds REAL, bone_keys INTEGER, morph_keys INTEGER, camera_keys INTEGER,
    light_keys INTEGER, shadow_keys INTEGER, ik_keys INTEGER, parse_error TEXT,
    UNIQUE(asset_id,file), FOREIGN KEY(asset_id,file) REFERENCES files(asset_id,rel) ON DELETE CASCADE
);
CREATE TABLE motion_bones (
    info_id INTEGER REFERENCES motion_info ON DELETE CASCADE, ordinal INTEGER NOT NULL, name TEXT NOT NULL,
    PRIMARY KEY(info_id,ordinal)
) WITHOUT ROWID;
CREATE TABLE motion_morphs (
    info_id INTEGER REFERENCES motion_info ON DELETE CASCADE, ordinal INTEGER NOT NULL, name TEXT NOT NULL,
    PRIMARY KEY(info_id,ordinal)
) WITHOUT ROWID;
CREATE TRIGGER invalidate_parse AFTER UPDATE OF size,mtime ON files
WHEN NEW.size != OLD.size OR NEW.mtime != OLD.mtime
BEGIN
    DELETE FROM model_info WHERE asset_id=OLD.asset_id AND file=OLD.rel;
    DELETE FROM motion_info WHERE asset_id=OLD.asset_id AND file=OLD.rel;
END;
CREATE TABLE parse_state (
    id INTEGER PRIMARY KEY CHECK(id=1), inventory_revision INTEGER NOT NULL, texture_revision INTEGER NOT NULL,
    path_digest TEXT NOT NULL
);
INSERT INTO parse_state VALUES(1,0,0,'');
