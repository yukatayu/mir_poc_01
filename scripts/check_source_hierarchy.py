#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent

REQUIRED_PATHS: dict[str, list[str]] = {
    "root_docs": [
        "AGENTS.md",
        "README.md",
        "Documentation.md",
        "progress.md",
        "tasks.md",
        "samples_progress.md",
    ],
    "specs": [
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
        "specs/12-decision-register.md",
    ],
    "policies": [
        ".docs/progress-task-axes.md",
        ".docs/continuous-task-policy.md",
        ".docs/current-l2-source-sample-authoring-policy.md",
    ],
    "plan": [
        "plan/00-index.md",
        "plan/01-status-at-a-glance.md",
        "plan/11-roadmap-near-term.md",
        "plan/17-research-phases-and-autonomy-gates.md",
    ],
    "scripts": [
        "scripts",
        "scripts/README.md",
    ],
    "samples": [
        "samples/README.md",
    ],
    "supporting_dirs": [
        "docs/reports",
        "docs/research_abstract",
        "samples/clean-near-end",
        "samples/clean-near-end/sugoroku-world",
        "crates/mir-runtime",
        "sub-agent-pro",
    ],
}


def build_status() -> dict[str, object]:
    present: list[str] = []
    missing: list[str] = []
    for paths in REQUIRED_PATHS.values():
        for rel_path in paths:
            if (REPO_ROOT / rel_path).exists():
                present.append(rel_path)
            else:
                missing.append(rel_path)
    return {
        "repo_root": str(REPO_ROOT),
        "required_count": len(present) + len(missing),
        "present_count": len(present),
        "missing_count": len(missing),
        "present": present,
        "missing": missing,
        "status": "ok" if not missing else "missing",
    }


def format_pretty(status: dict[str, object]) -> str:
    lines = [
        "source hierarchy check",
        f"  repo_root: {status['repo_root']}",
        f"  required: {status['required_count']}",
        f"  present: {status['present_count']}",
        f"  missing: {status['missing_count']}",
    ]
    missing = status["missing"]
    if missing:
        lines.append("  missing paths:")
        for rel_path in missing:
            lines.append(f"    - {rel_path}")
    else:
        lines.append("  all required paths present")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--format", choices=["pretty", "json"], default="pretty")
    args = parser.parse_args()

    status = build_status()
    if args.format == "json":
        print(json.dumps(status, indent=2, ensure_ascii=False))
    else:
        print(format_pretty(status))
    return 0 if status["missing_count"] == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
