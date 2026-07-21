#!/usr/bin/env python3
"""Build INDEX.json from front matter; validate ids and depends_on.

Usage: python3 meta/build-index.py [--check]
Run from the canon root or any subdirectory of it.
"""
import json, os, re, sys

def find_root(p):
    p = os.path.abspath(p)
    while p != "/":
        if os.path.exists(os.path.join(p, "NORTH-STAR.md")):
            return p
        p = os.path.dirname(p)
    sys.exit("canon root not found")

def parse_front_matter(text, path):
    m = re.match(r"\A---\n(.*?)\n---\n", text, re.S)
    if not m:
        return None
    fm, out, errors = m.group(1), {}, []
    for line in fm.splitlines():
        mm = re.match(r"(\w+):\s*(.*)$", line)
        if not mm:
            if line.strip():
                errors.append(f"malformed front matter line '{line}': {path}")
            continue
        k, v = mm.group(1), mm.group(2).strip()
        if k in out:
            errors.append(f"duplicate front matter field '{k}': {path}")
        if v.startswith("["):
            items = [x.strip() for x in v.strip("[]").split(",") if x.strip()]
            out[k] = items
        else:
            out[k] = v
    return out, errors

def main():
    root = find_root(os.getcwd())
    entries, errors = [], []
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if not d.startswith(".")]
        for fn in sorted(filenames):
            if not fn.endswith(".md"):
                continue
            path = os.path.join(dirpath, fn)
            rel = os.path.relpath(path, root)
            with open(path, encoding="utf-8") as f:
                text = f.read()
            parsed = parse_front_matter(text, rel)
            if parsed is None:
                errors.append(f"missing front matter: {rel}")
                continue
            fm, parse_errors = parsed
            errors.extend(parse_errors)
            for req in ("id", "status", "maturity", "summary"):
                if req not in fm:
                    errors.append(f"missing '{req}': {rel}")
            fm.setdefault("depends_on", [])
            fm.setdefault("open_items", [])
            fm["path"] = rel
            fm["bytes"] = len(text.encode("utf-8"))
            entries.append(fm)
    ids = {}
    for e in entries:
        i = e.get("id")
        if i in ids:
            errors.append(f"duplicate id '{i}': {e['path']} vs {ids[i]}")
        ids[i] = e["path"]
    for e in entries:
        for d in e.get("depends_on", []):
            if d not in ids:
                errors.append(f"dangling depends_on '{d}' in {e['path']}")
    index = {"canon_version": "0.1.0",
             "files": len(entries),
             "entries": sorted(entries, key=lambda e: e["path"])}
    index_path = os.path.join(root, "INDEX.json")
    if errors:
        print("VALIDATION ERRORS:")
        for e in errors:
            print(" -", e)
        sys.exit(1)
    if "--check" in sys.argv:
        try:
            with open(index_path, encoding="utf-8") as f:
                existing_index = json.load(f)
        except (OSError, json.JSONDecodeError) as exc:
            errors.append(f"INDEX.json cannot be read: {exc}")
        else:
            if existing_index != index:
                errors.append("INDEX.json is stale; run python3 meta/build-index.py")
    else:
        with open(index_path, "w", encoding="utf-8") as f:
            json.dump(index, f, ensure_ascii=False, indent=1)
    if errors:
        print("VALIDATION ERRORS:")
        for e in errors:
            print(" -", e)
        sys.exit(1)
    print(f"ok: {len(entries)} files indexed")

if __name__ == "__main__":
    main()
