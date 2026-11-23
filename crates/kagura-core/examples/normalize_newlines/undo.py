"""Restore manifest bytes from the accompanying migration log and backups."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import stat
import tempfile


def digest(data):
    return hashlib.sha256(data).hexdigest()


def checked_path(base, parts):
    """Only walk regular directories/files in the selected library or backup."""
    path = base
    for part in ("", *parts):
        path = path / part
        meta = path.lstat()
        if stat.S_ISLNK(meta.st_mode) or getattr(meta, "st_file_attributes", 0) & stat.FILE_ATTRIBUTE_REPARSE_POINT:
            raise ValueError(f"reparse point: {path}")
    if not stat.S_ISREG(meta.st_mode):
        raise ValueError(f"not a regular file: {path}")
    return path


def plan_restore(library, run):
    entries = []
    seen = set()
    with checked_path(run, ["log.jsonl"]).open(encoding="utf-8") as log:
        for line in log:
            row = json.loads(line)
            name = row["path"]
            parts = name.split("/")
            if (parts[-1] != "_asset.manifest.json" or parts[0] not in {"Models", "Stages", "Motions"}
                    or any(p in {"", ".", ".."} or "\\" in p or ":" in p for p in parts)):
                raise ValueError(f"invalid manifest path: {name!r}")
            key = os.path.normcase(name)
            if key in seen:
                raise ValueError(f"duplicate log path: {name!r}")
            seen.add(key)
            original = checked_path(run, ["backup", *parts]).read_bytes()
            before, after = row["old_sha256"], row["new_sha256"]
            if digest(original) != before or digest(original.replace(b"\r\n", b"\n")) != after:
                raise ValueError(f"backup/log mismatch: {name}")
            target = checked_path(library, parts)
            current = digest(target.read_bytes())
            if current == before:
                continue
            if current != after:
                raise ValueError(f"modified since migration: {target}")
            entries.append((parts, before, after))
    if not seen:
        raise ValueError("empty migration log")
    return entries, len(seen)


def restore_one(library, run, parts, before, after):
    target = checked_path(library, parts)
    original = checked_path(run, ["backup", *parts]).read_bytes()
    if digest(original) != before:
        raise ValueError(f"backup changed: {target}")
    temporary = None
    try:
        with tempfile.NamedTemporaryFile(dir=target.parent, prefix=".kagura-undo-", delete=False) as stream:
            temporary = Path(stream.name)
            stream.write(original)
            stream.flush()
            os.fsync(stream.fileno())
        current = digest(checked_path(library, parts).read_bytes())
        if current == before:
            return False
        if current != after:
            raise ValueError(f"source changed: {target}")
        os.replace(temporary, target)
        temporary = None
        if digest(target.read_bytes()) != before:
            raise ValueError(f"restore verification failed: {target}")
        return True
    finally:
        if temporary is not None:
            temporary.unlink(missing_ok=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", required=True, type=Path, help="Parent of Library, e.g. G:\\MMD")
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()
    run = Path(__file__).resolve().parent
    library = args.root.absolute() / "Library"
    entries, total = plan_restore(library, run)
    if args.dry_run:
        print(f"verified={total}, would_restore={len(entries)}, already_original={total - len(entries)}")
        return
    restored = sum(restore_one(library, run, *entry) for entry in entries)
    print(f"verified={total}, restored={restored}, already_original={total - restored}")


if __name__ == "__main__":
    main()
