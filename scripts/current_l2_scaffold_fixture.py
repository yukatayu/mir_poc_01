#!/usr/bin/env python3

import argparse
import json
import sys
from pathlib import Path
from typing import Any


DEFAULT_OUTPUT_DIR = Path("target/current-l2-fixture-scaffolds")


def ensure_fixture_stem(stem: str) -> str:
    if not stem:
        raise ValueError("fixture stem must not be empty")
    if stem in {".", ".."} or "/" in stem:
        raise ValueError("fixture stem must be a single path segment")
    return stem


def default_fixture_id(stem: str) -> str:
    return stem.replace("-", "_")


def fixture_paths(output_dir: Path, stem: str) -> tuple[Path, Path]:
    return (
        output_dir / f"{stem}.json",
        output_dir / f"{stem}.host-plan.json",
    )


def build_fixture_document(
    stem: str,
    kind: str,
    fixture_id: str,
    source_example_id: str,
) -> dict[str, Any]:
    enters_evaluation = kind == "runtime"
    final_outcome = "OPEN_RUNTIME_OUTCOME" if enters_evaluation else "not_evaluated"
    runtime_notes = (
        ["OPEN QUESTION: fill runtime outcome notes"]
        if enters_evaluation
        else ["static-only scaffold: runtime evaluation does not start"]
    )
    must_explain = (
        ["OPEN QUESTION: fill explanation obligations"]
        if enters_evaluation
        else ["OPEN QUESTION: explain why static gate stops evaluation"]
    )

    return {
        "schema_version": "current-l2-ast-fixture-v0",
        "fixture_id": fixture_id,
        "source_example_id": source_example_id,
        "program": {
            "kind": "Program",
            "body": [],
        },
        "expected_static": {
            "verdict": "OPEN_STATIC_VERDICT",
            "reasons": ["OPEN QUESTION: fill static reasons"],
        },
        "expected_runtime": {
            "enters_evaluation": enters_evaluation,
            "final_outcome": final_outcome,
            "notes": runtime_notes,
        },
        "expected_trace_audit": {
            "event_kinds": [],
            "non_admissible_metadata": [],
            "narrative_explanations": [],
            "must_explain": must_explain,
        },
    }


def build_host_plan_document() -> dict[str, Any]:
    return {
        "schema_version": "current-l2-host-plan-v0",
        "predicate_rules": [],
        "effect_rules": [],
        "trace_expectation_override": {},
    }


def write_json(path: Path, document: dict[str, Any]) -> None:
    path.write_text(
        json.dumps(document, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "current L2 fixture authoring 用の non-production scaffold helper。"
            " semantics を推論せず、required carrier の骨組みだけを出す。"
        )
    )
    parser.add_argument("fixture_stem", help="new fixture stem without extension")
    parser.add_argument(
        "--kind",
        choices=["runtime", "static-only"],
        required=True,
        help="whether the scaffold is runtime or static-only",
    )
    parser.add_argument(
        "--fixture-id",
        help="explicit fixture_id; default derives from stem by replacing '-' with '_'",
    )
    parser.add_argument(
        "--source-example-id",
        default="OPEN-SOURCE-EXAMPLE-ID",
        help="source example identifier placeholder",
    )
    parser.add_argument(
        "--output-dir",
        default=str(DEFAULT_OUTPUT_DIR),
        help="output directory for scaffold files",
    )
    parser.add_argument(
        "--overwrite",
        action="store_true",
        help="allow replacing existing scaffold files",
    )
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    stem = ensure_fixture_stem(args.fixture_stem)
    output_dir = Path(args.output_dir)
    fixture_path, host_plan_path = fixture_paths(output_dir, stem)

    targets = [fixture_path]
    if args.kind == "runtime":
        targets.append(host_plan_path)

    if not args.overwrite:
        for path in targets:
            if path.exists():
                print(
                    f"scaffold target already exists: {path} (use --overwrite to replace)",
                    file=sys.stderr,
                )
                return 2

    output_dir.mkdir(parents=True, exist_ok=True)
    fixture_document = build_fixture_document(
        stem=stem,
        kind=args.kind,
        fixture_id=args.fixture_id or default_fixture_id(stem),
        source_example_id=args.source_example_id,
    )
    write_json(fixture_path, fixture_document)

    if args.kind == "runtime":
        write_json(host_plan_path, build_host_plan_document())

    print(fixture_path)
    if args.kind == "runtime":
        print(host_plan_path)
    return 0


if __name__ == "__main__":
    sys.exit(main())
