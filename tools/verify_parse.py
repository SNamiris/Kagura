"""Compare cached Rust results with independent, read-only Python section readers."""
import argparse
import hashlib
import json
from pathlib import Path
import sqlite3
import struct


def exact(stream, count):
    data = stream.read(count)
    if len(data) != count:
        raise EOFError(f"truncated at byte {stream.tell()}")
    return data


def i32(stream):
    n, = struct.unpack("<i", exact(stream, 4))
    if n < 0:
        raise ValueError(f"negative count at {stream.tell() - 4}")
    return n


def pmx(stream):
    if exact(stream, 4) != b"PMX ":
        raise ValueError("PMX signature")
    version, size = struct.unpack("<fB", exact(stream, 5))
    if round(version, 3) not in (2.0, 2.1):
        raise ValueError(f"PMX version {version}")
    encoding, uv, vi, ti, _mi, bi, _moi, _ri = exact(stream, size)[:8]
    codec = ("utf-16-le", "utf-8")[encoding]
    name_size = i32(stream)
    if name_size > 8 * 1024 * 1024:
        raise ValueError(f"model name exceeds 8 MiB at byte {stream.tell()}")
    name = exact(stream, name_size).decode(codec, errors="replace")

    def skip(n):
        stream.seek(n, 1)

    def skip_string():
        skip(i32(stream))

    for _ in range(3):
        skip_string()
    vertices = i32(stream)
    weights = {0: bi, 1: 2 * bi + 4, 2: 4 * bi + 16, 3: 2 * bi + 40, 4: 4 * bi + 16}
    for _ in range(vertices):
        skip(32 + 16 * uv)
        weight, = exact(stream, 1)
        skip(weights[weight] + 4)
    skip(i32(stream) * vi)
    for _ in range(i32(stream)):
        skip_string()
    for _ in range(i32(stream)):
        skip_string()
        skip_string()
        skip(65 + 2 * ti + 1)
        shared, = exact(stream, 1)
        skip(1 if shared else ti)
        skip_string()
        skip(4)
    bones = i32(stream)
    for _ in range(bones):
        skip_string()
        skip_string()
        skip(12 + bi + 4)
        flags, = struct.unpack("<H", exact(stream, 2))
        skip(bi if flags & 1 else 12)
        if flags & 0x0300:
            skip(bi + 4)
        if flags & 0x0400:
            skip(12)
        if flags & 0x0800:
            skip(24)
        if flags & 0x2000:
            skip(4)
        if flags & 0x0020:
            skip(bi + 8)
            for _ in range(i32(stream)):
                skip(bi)
                if exact(stream, 1) == b"\x01":
                    skip(24)
    return dict(name=name, vertices=vertices, bones=bones, morphs=i32(stream))


def vmd(stream):
    stamp = exact(stream, 30).partition(b"\0")[0]
    widths = {b"Vocaloid Motion Data 0002": 20, b"Vocaloid Motion Data file": 10}
    target = exact(stream, widths[stamp]).partition(b"\0")[0].decode("cp932", "replace")
    result = {"target_model": target}
    high = 0
    # vmdlib.py supplies the six section sizes/counts. Read frame indices as well.
    sections = (("bone", 111, 15), ("morph", 23, 15), ("camera", 61, 0),
                ("light", 28, 0), ("shadow", 9, 0), ("ik", 0, 0))
    for key, stride, frame_at in sections:
        raw = stream.read(4)
        if not raw and key != "bone":
            count = 0
        else:
            count, = struct.unpack("<I", raw)
        result[key + "_keys"] = count
        if key == "ik":
            for _ in range(count):
                frame, _, links = struct.unpack("<IBI", exact(stream, 9))
                high = max(frame, high)
                for _ in range(links):
                    exact(stream, 21)
        else:
            for block in range(0, count, 1024):
                data = exact(stream, min(count - block, 1024) * stride)
                high = max(high, max(struct.unpack_from("<I", data, i)[0]
                                     for i in range(frame_at, len(data), stride)))
    return dict(max_frame=high, **result)


def sample(db, ext, count):
    candidates = db.execute(
        "SELECT a.id, a.rel_dir, f.rel FROM files f JOIN assets a ON f.asset_id=a.id "
        "WHERE lower(substr(f.rel,-4))=?", ("." + ext,))
    def rank(row):
        return hashlib.sha256((row[1] + "/" + row[2]).encode()).digest()
    return sorted(candidates, key=rank)[:count]


def compare(db, root, ext, count):
    reader, table = (pmx, "model_info") if ext == "pmx" else (vmd, "motion_info")
    paths = sample(db, ext, count)
    agree = 0
    rejected = []
    errors = []
    for asset_id, directory, name in paths:
        path = Path(directory, name)
        row = db.execute(f"SELECT * FROM {table} WHERE asset_id=? AND file=?",
                         (asset_id, name)).fetchone()
        try:
            with (root / path).open("rb") as stream:
                expected = reader(stream)
        except (ValueError, KeyError, OSError, EOFError, struct.error) as err:
            result = {"file": path.as_posix(), "python": str(err),
                      "rust": row["parse_error"] if row is not None else "missing cache row"}
            (rejected if row is not None and row["parse_error"] is not None else errors).append(result)
            continue
        if row is None or row["parse_error"] is not None:
            errors.append({"file": path.as_posix(), "rust": "missing cache row" if row is None else row["parse_error"]})
            continue
        diff = {k: [v, row[k]] for k, v in expected.items() if v != row[k]}
        if diff:
            errors.append({"file": path.as_posix(), "python_rust": diff})
        else:
            agree += 1
    print(json.dumps(dict(format=ext, count=len(paths), equal_fields=agree, both_rejected=rejected, differences=errors),
                     ensure_ascii=False))
    return len(paths) == count and not errors


def main():
    args = argparse.ArgumentParser(description=__doc__)
    args.add_argument("root", type=Path)
    args.add_argument("index", type=Path)
    args.add_argument("--count", type=int, default=200)
    opt = args.parse_args()
    if opt.count < 1:
        args.error("--count must be positive")
    with sqlite3.connect(opt.index.resolve().as_uri() + "?mode=ro", uri=True) as db:
        db.row_factory = sqlite3.Row
        if not Path(db.execute("SELECT root FROM meta").fetchone()[0]).samefile(opt.root):
            args.error("database belongs to another library")
        checks = [compare(db, opt.root, ext, opt.count) for ext in ("pmx", "vmd")]
    return 0 if all(checks) else 1


if __name__ == "__main__":
    raise SystemExit(main())
