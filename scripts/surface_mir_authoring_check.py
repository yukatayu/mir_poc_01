#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]
SURFACE_ROOT = REPO_ROOT / "samples" / "full-system-v1-surface"


def _mir_sources() -> list[Path]:
    if not SURFACE_ROOT.exists():
        return []
    return sorted(SURFACE_ROOT.rglob("*.mir"))


def _is_negative_source(path: Path) -> bool:
    return any("negative" in part for part in path.parts)


def _parse_source(path: Path) -> dict[str, Any] | None:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-ast",
            "--example",
            "surface_mir_alpha_parse",
            "--",
            str(path),
            "--format",
            "json",
        ],
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError:
        return None


def check_all() -> dict[str, Any]:
    sources = _mir_sources()
    bracket_scope_hits = []
    positive_parse_failures = []
    parser_unavailable = []
    for source in sources:
        payload = _parse_source(source)
        relative_path = str(source.relative_to(REPO_ROOT))
        if payload is None:
            parser_unavailable.append(relative_path)
            continue
        codes = [row["code"] for row in payload.get("diagnostics") or []]
        if "bracket_place_scope_not_supported" in codes and not _is_negative_source(source):
            bracket_scope_hits.append(relative_path)
        if payload.get("accepted") is not True and not _is_negative_source(source):
            positive_parse_failures.append(
                {
                    "path": relative_path,
                    "diagnostic_codes": codes,
                }
            )
    package_artifacts = sorted(SURFACE_ROOT.rglob("package.mir.json")) if SURFACE_ROOT.exists() else []
    diagnostics = []
    if bracket_scope_hits:
        diagnostics.append(
            {
                "code": "bracket_place_scope_source_found",
                "paths": bracket_scope_hits,
            }
        )
    if positive_parse_failures:
        diagnostics.append(
            {
                "code": "surface_positive_source_parse_failed",
                "failures": positive_parse_failures,
            }
        )
    if parser_unavailable:
        diagnostics.append(
            {
                "code": "surface_parser_report_unavailable",
                "paths": parser_unavailable,
            }
        )
    if package_artifacts:
        diagnostics.append(
            {
                "code": "surface_source_root_contains_package_artifact",
                "paths": [str(path.relative_to(REPO_ROOT)) for path in package_artifacts],
            }
        )
    return {
        "command": "check-all",
        "family": "surface_mir_authoring_check",
        "sample_root": str(SURFACE_ROOT.relative_to(REPO_ROOT)),
        "source_count": len(sources),
        "accepted": not diagnostics,
        "diagnostics": diagnostics,
        "canonical_place_scope_syntax": "S { ... }",
        "source_authority": ".mir",
        "package_mir_json_role": "alpha_artifact_not_source_authority",
        "final_public_grammar_frozen": False,
    }


def format_pretty(payload: dict[str, Any]) -> str:
    return "\n".join(
        [
            "SURFACE MIR AUTHORING CHECK",
            f"sample root: {payload['sample_root']}",
            f"sources: {payload['source_count']}",
            f"accepted: {payload['accepted']}",
            f"diagnostics: {len(payload['diagnostics'])}",
        ]
    )


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=["check-all"])
    parser.add_argument("--format", choices=["json", "pretty"], default="pretty")
    args = parser.parse_args(argv)

    payload = check_all()
    if args.format == "json":
        print(json.dumps(payload, indent=2, ensure_ascii=False))
    else:
        print(format_pretty(payload))
    return 0 if payload["accepted"] else 2


if __name__ == "__main__":
    raise SystemExit(main())
