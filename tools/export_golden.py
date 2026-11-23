"""Export Kagura naming and layout fixtures from the read-only Python reference."""

import json
import platform
import sys
from collections import Counter
from importlib.metadata import version
from pathlib import Path

ROOT = Path(r"G:\MMD")
OUTPUT = Path(__file__).resolve().parents[1] / "crates/kagura-core/tests/golden"
KEY_LIMITS = {
    "model": {"author": 40, "name": 40},
    "stage": {"author": 40, "name": 40},
    "song": {"title": 40, "composer": 40, "edition": 24},
    "motion_part": {"author": 40, "suffix": 24},
}


def write_jsonl(path, rows):
    with path.open("w", encoding="utf-8", newline="\n") as stream:
        for row in rows:
            stream.write(json.dumps(row, ensure_ascii=False) + "\n")


def main():
    # Importing plan also imports scan. Keep __pycache__ off the reference drive.
    sys.dont_write_bytecode = True
    sys.path.insert(0, str(ROOT / "_design/migration"))
    import plan

    if not (ROOT / "Library").is_dir():
        raise FileNotFoundError(ROOT / "Library")

    songs = plan.known_songs(ROOT)
    inputs = set()
    destinations = []
    counts = Counter()
    skipped = []
    for path, record in plan.iter_library_metadata(ROOT, skipped=skipped):
        kind = record["type"]
        relative = path.relative_to(ROOT)
        directory = relative.parent.as_posix()
        actual = relative.as_posix() if kind == "song" else directory
        expected = plan.destination(record, songs)
        if expected != actual:
            raise ValueError(f"{record['id']}: destination {expected!r} != actual {actual!r}")

        counts[kind] += 1
        destinations.append({"id": record["id"], "type": kind, "dir": directory})
        key = record["key"]
        for field, limit in KEY_LIMITS[kind].items():
            if field in key:
                inputs.add((key[field], limit))

    names = []
    for name, limit in sorted(inputs):
        row = {"input": name, "limit": limit}
        try:
            row["output"] = plan.safe(name, limit)
        except ValueError as error:
            row["error"] = str(error)
        names.append(row)

    destinations.sort(key=lambda row: (row["id"], row["dir"]))
    summary = {
        "python": platform.python_version(),
        "jsonschema": version("jsonschema"),
        "counts": dict(sorted(counts.items())),
        "total": sum(counts.values()),
        "skipped": sorted(skipped, key=lambda row: (row["path"], row["reason"])),
    }
    OUTPUT.mkdir(parents=True, exist_ok=True)
    write_jsonl(OUTPUT / "safe.jsonl", names)
    write_jsonl(OUTPUT / "destination.jsonl", destinations)
    with (OUTPUT / "scan_summary.json").open("w", encoding="utf-8", newline="\n") as stream:
        json.dump(summary, stream, ensure_ascii=False, indent=2)
        stream.write("\n")


if __name__ == "__main__":
    main()
