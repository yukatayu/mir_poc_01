#!/usr/bin/env python3

import argparse
import json
import sys
from pathlib import Path
from typing import Any


CLUSTER_NAME = "try_rollback_structural_floor"
TRY_FALLBACK_KIND = "TryFallback"
ATOMIC_CUT_KIND = "AtomicCut"
MISSING_FALLBACK_BODY = "missing_fallback_body"
DISALLOWED_FALLBACK_PLACEMENT = "disallowed_fallback_placement"
NO_FINDINGS = "no_findings"
FINDINGS_PRESENT = "findings_present"
MISSING_FALLBACK_BODY_REASON = "try fallback body must not be empty"
DISALLOWED_FALLBACK_PLACEMENT_REASON = "atomic cut may not appear inside fallback body"


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "current L2 の dedicated try/rollback structural helper。"
            " static gate artifact loop にだけ接続する non-production compare helper。"
        )
    )
    parser.add_argument("fixture_path")
    parser.add_argument("artifact_path")
    return parser


def snippet(value: Any) -> str:
    return json.dumps(value, ensure_ascii=False, indent=2)


def normalized_expected_findings(fixture: dict[str, Any]) -> list[dict[str, str]] | None:
    expected_static = fixture.get("expected_static")
    if not isinstance(expected_static, dict):
        return None
    findings = expected_static.get("checked_try_rollback_structural_findings")
    if findings is None:
        return None

    normalized: list[dict[str, str]] = []
    for row in findings:
        if not isinstance(row, dict):
            continue
        subject_kind = row.get("subject_kind")
        finding_kind = row.get("finding_kind")
        if isinstance(subject_kind, str) and isinstance(finding_kind, str):
            normalized.append(
                {
                    "subject_kind": subject_kind,
                    "finding_kind": finding_kind,
                }
            )
    return normalized


def expected_structural_verdict(fixture: dict[str, Any]) -> str | None:
    expected_static = fixture.get("expected_static")
    if not isinstance(expected_static, dict):
        return None
    verdict = expected_static.get("checked_try_rollback_structural_verdict")
    return verdict if isinstance(verdict, str) else None


def expected_static_verdict(fixture: dict[str, Any]) -> str | None:
    expected_static = fixture.get("expected_static")
    if not isinstance(expected_static, dict):
        return None
    verdict = expected_static.get("verdict")
    return verdict if isinstance(verdict, str) else None


def artifact_static_verdict(artifact: dict[str, Any]) -> str | None:
    checker_core = artifact.get("checker_core")
    if not isinstance(checker_core, dict):
        return None
    verdict = checker_core.get("static_verdict")
    return verdict if isinstance(verdict, str) else None


def artifact_reasons(artifact: dict[str, Any]) -> list[str] | None:
    checker_core = artifact.get("checker_core")
    if not isinstance(checker_core, dict):
        return None
    reasons = checker_core.get("reasons")
    if not isinstance(reasons, list):
        return None
    normalized: list[str] = []
    for reason in reasons:
        if isinstance(reason, str):
            normalized.append(reason)
    return normalized


def collect_findings_from_reasons(reasons: list[str]) -> list[dict[str, str]]:
    findings: list[dict[str, str]] = []
    for reason in reasons:
        if reason == MISSING_FALLBACK_BODY_REASON:
            findings.append(
                {
                    "subject_kind": TRY_FALLBACK_KIND,
                    "finding_kind": MISSING_FALLBACK_BODY,
                }
            )
        elif reason == DISALLOWED_FALLBACK_PLACEMENT_REASON:
            findings.append(
                {
                    "subject_kind": ATOMIC_CUT_KIND,
                    "finding_kind": DISALLOWED_FALLBACK_PLACEMENT,
                }
            )
    return findings


def actual_structural_verdict(findings: list[dict[str, str]]) -> str:
    return FINDINGS_PRESENT if findings else NO_FINDINGS


def status_and_exit_code(
    *,
    expected_verdict: str | None,
    expected_findings: list[dict[str, str]] | None,
    actual_verdict: str,
    actual_findings: list[dict[str, str]],
    expected_static_verdict_value: str | None,
    artifact_static_verdict_value: str | None,
) -> tuple[str, int]:
    if expected_verdict is None and expected_findings is None:
        if not actual_findings:
            return "out_of_scope", 0
        return "fixture_try_rollback_expectation_missing", 1

    if expected_verdict is None or expected_findings is None:
        return "fixture_try_rollback_contract_incomplete", 1

    if expected_static_verdict_value is not None and (
        artifact_static_verdict_value is None
        or expected_static_verdict_value != artifact_static_verdict_value
    ):
        return "static_gate_verdict_mismatch", 1

    if expected_verdict == actual_verdict and expected_findings == actual_findings:
        return "matched", 0
    return "mismatch", 1


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    fixture_path = Path(args.fixture_path)
    artifact_path = Path(args.artifact_path)

    fixture = read_json(fixture_path)
    artifact = read_json(artifact_path)

    reasons = artifact_reasons(artifact) or []
    actual_findings = collect_findings_from_reasons(reasons)
    actual_verdict = actual_structural_verdict(actual_findings)
    expected_verdict = expected_structural_verdict(fixture)
    expected_findings = normalized_expected_findings(fixture)
    expected_static_verdict_value = expected_static_verdict(fixture)
    artifact_static_verdict_value = artifact_static_verdict(artifact)

    status, exit_code = status_and_exit_code(
        expected_verdict=expected_verdict,
        expected_findings=expected_findings,
        actual_verdict=actual_verdict,
        actual_findings=actual_findings,
        expected_static_verdict_value=expected_static_verdict_value,
        artifact_static_verdict_value=artifact_static_verdict_value,
    )

    print(f"fixture: {fixture_path}")
    print(f"artifact: {artifact_path}")
    print(f"cluster: {CLUSTER_NAME}")
    print(f"status: {status}")
    print(f"fixture expected static verdict: {expected_static_verdict_value}")
    print(f"artifact static verdict: {artifact_static_verdict_value}")
    print(f"fixture structural verdict: {expected_verdict}")
    print(f"actual structural verdict: {actual_verdict}")
    print("fixture findings:")
    print(snippet(expected_findings or []))
    print("actual findings:")
    print(snippet(actual_findings))

    return exit_code


if __name__ == "__main__":
    sys.exit(main())
