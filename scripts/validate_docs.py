#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
REQUIRED = [
    "README.md",
    "AGENTS.md",
    "Documentation.md",
    "specs/00-document-map.md",
    "specs/01-charter-and-decision-levels.md",
    "specs/02-system-overview.md",
    "specs/03-layer-model.md",
    "specs/04-mir-core.md",
    "specs/05-mirrorea-fabric.md",
    "specs/06-prismcascade-positioning.md",
    "specs/07-typed-effects-wiring-platform.md",
    "specs/08-cross-system-relations.md",
    "specs/09-invariants-and-constraints.md",
    "specs/10-open-questions.md",
    "specs/11-roadmap-and-workstreams.md",
    "docs/reports/TEMPLATE.md",
]


def main() -> int:
    missing = [p for p in REQUIRED if not (ROOT / p).exists()]
    if missing:
        print("Missing required files:")
        for p in missing:
            print(" -", p)
        return 1

    reports = sorted((ROOT / "docs" / "reports").glob("[0-9][0-9][0-9][0-9]-*.md"))
    if not reports:
        print("No numbered reports found in docs/reports")
        return 1

    print("Documentation scaffold looks complete.")
    print(f"Found {len(reports)} numbered report(s).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
