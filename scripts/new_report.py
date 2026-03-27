#!/usr/bin/env python3
from __future__ import annotations

import argparse
from datetime import datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REPORT_DIR = ROOT / "docs" / "reports"
TEMPLATE = REPORT_DIR / "TEMPLATE.md"


def next_index() -> int:
    nums = []
    for p in REPORT_DIR.glob("[0-9][0-9][0-9][0-9]-*.md"):
        try:
            nums.append(int(p.name.split("-", 1)[0]))
        except ValueError:
            pass
    return (max(nums) + 1) if nums else 1


def slugify(s: str) -> str:
    out = []
    for ch in s.strip().lower():
        if ch.isalnum():
            out.append(ch)
        elif ch in " -_":
            out.append("-")
    slug = "".join(out)
    while "--" in slug:
        slug = slug.replace("--", "-")
    return slug.strip("-") or "report"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--slug", required=True)
    args = ap.parse_args()

    idx = next_index()
    slug = slugify(args.slug)
    path = REPORT_DIR / f"{idx:04d}-{slug}.md"

    text = TEMPLATE.read_text(encoding="utf-8")
    title = f"Report {idx:04d} — {slug.replace('-', ' ')}"
    text = text.replace("# Report XXXX — <title>", f"# {title}")
    text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")

    path.write_text(text, encoding="utf-8")
    print(path)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
