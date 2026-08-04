#!/usr/bin/env python3
"""Evaluate the bounded T0/G0 semantic-assertion profile from Git blobs."""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import subprocess
import sys
from collections.abc import Mapping
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent
PROFILE_PATH = "mirrorea_canon/plan/04-t0-g0-semantic-assertion-profile.md"
PROFILE_ID = "phase-governance/t0-g0"
PROFILE_VERSION = 3
KIND = "semantic-assertion-evaluation"
ASSERTION_IDS = (
    "SA-01-canon-exclusive-normativity",
    "SA-02-north-star-meaning-before-interface",
    "SA-03-domain-vocabulary-not-core",
    "SA-04-canon-lab-separation",
    "SA-05-clean-suite-is-lab-evidence",
    "SA-06-agents-follow-source-hierarchy",
)


def _contains_all(source: str, required: tuple[str, ...]) -> bool:
    lowered = source.casefold()
    return all(fragment.casefold() in lowered for fragment in required)


def evaluate_assertions(sources: Mapping[str, str]) -> list[dict[str, object]]:
    """Return profile rows from the selected semantic witnesses only.

    This function intentionally returns no whole-file hash: the Git revision is
    the evidence cut, while each selector decides a semantic predicate over the
    designated source witnesses.  Unselected reader-facing wording cannot
    change a row result.
    """

    checks = (
        (
            ASSERTION_IDS[0],
            (
                "CANON.md",
                "README.md",
                "mirrorea_canon/meta/source-hierarchy.md",
            ),
            "canonical-source-and-lab-evidence-notices",
            (
                ("CANON.md", ("mirrorea_canon/", "normative", "LAB")),
                ("README.md", ("mirrorea_canon/", "sole normative source")),
                (
                    "mirrorea_canon/meta/source-hierarchy.md",
                    ("canon(規範) > LAB(evidence)",),
                ),
            ),
        ),
        (
            ASSERTION_IDS[1],
            (
                "mirrorea_canon/NORTH-STAR.md",
                "mirrorea_canon/DESIGN-CONSTITUTION.md",
            ),
            "meaning-before-communication-interface",
            (
                (
                    "mirrorea_canon/NORTH-STAR.md",
                    ("通信境界・API 境界は設計の出発点ではなく", "射影の産物"),
                ),
                (
                    "mirrorea_canon/DESIGN-CONSTITUTION.md",
                    ("Communication interfaces", "projections of checked source meaning"),
                ),
            ),
        ),
        (
            ASSERTION_IDS[2],
            (
                "mirrorea_canon/adr/ADR-0001.md",
                "mirrorea_canon/DESIGN-CONSTITUTION.md",
            ),
            "world-room-avatar-game-are-not-core-primitives",
            (
                (
                    "mirrorea_canon/adr/ADR-0001.md",
                    ("World", "Room", "Avatar", "Game", "ドメイン"),
                ),
                (
                    "mirrorea_canon/DESIGN-CONSTITUTION.md",
                    ("`World`", "`Room`", "`Avatar`", "`Game`", "never", "Core primitives"),
                ),
            ),
        ),
        (
            ASSERTION_IDS[3],
            (
                "mirrorea_canon/adr/ADR-0012.md",
                "mirrorea_canon/meta/source-hierarchy.md",
            ),
            "canon-is-normative-lab-is-evidence",
            (
                ("mirrorea_canon/adr/ADR-0012.md", ("canon", "LAB")),
                (
                    "mirrorea_canon/meta/source-hierarchy.md",
                    ("canon(規範) > LAB(evidence)",),
                ),
            ),
        ),
        (
            ASSERTION_IDS[4],
            ("samples/clean-near-end/README.md",),
            "clean-suite-explicitly-lab-evidence",
            (("samples/clean-near-end/README.md", ("LAB evidence",)),),
        ),
        (
            ASSERTION_IDS[5],
            ("AGENTS.md",),
            "agents-canon-first-read-order",
            (
                (
                    "AGENTS.md",
                    ("Canon notice", "mirrorea_canon/README.md", "mirrorea_canon/MAP.md"),
                ),
            ),
        ),
    )

    rows: list[dict[str, object]] = []
    for identifier, subject_refs, selector, requirements in checks:
        passed = all(
            _contains_all(sources.get(path, ""), fragments)
            for path, fragments in requirements
        )
        rows.append(
            {
                "id": identifier,
                "subject_refs": list(subject_refs),
                "selector": selector,
                "result": "pass" if passed else "fail",
                "normalized_finding": (
                    "selected semantic witnesses satisfy the selector"
                    if passed
                    else "one or more selected semantic witnesses do not satisfy the selector"
                ),
            }
        )
    return rows


def canonical_artifact_bytes(artifact: Mapping[str, object]) -> bytes:
    value = copy.deepcopy(dict(artifact))
    value.pop("artifact_digest", None)
    return json.dumps(
        value, ensure_ascii=False, sort_keys=True, separators=(",", ":")
    ).encode("utf-8")


def make_artifact(
    *,
    source_revision: str,
    profile_source_sha256: str,
    producer_sha256: str,
    rows: list[dict[str, object]],
) -> dict[str, object]:
    result = "pass" if all(row["result"] == "pass" for row in rows) else "fail"
    artifact: dict[str, object] = {
        "kind": KIND,
        "profile": {
            "id": PROFILE_ID,
            "version": PROFILE_VERSION,
            "path": PROFILE_PATH,
            "source_revision": source_revision,
            "source_sha256": profile_source_sha256,
        },
        "producer": {
            "path": "scripts/evaluate_t0_semantic_assertions.py",
            "sha256": producer_sha256,
        },
        "assertions": rows,
        "result": result,
        "non_claims": [
            "SCN conformance",
            "C-static",
            "C-runtime",
            "C-distributed",
            "proof",
            "I1 authorization",
            "runtime implementation",
            "public API/ABI/wire",
        ],
    }
    artifact["artifact_digest"] = {
        "algorithm": "sha256",
        "canonicalization": "sorted-keys-utf8-compact-json-v1",
        "scope": "complete-artifact-with-artifact_digest-omitted",
        "value": hashlib.sha256(canonical_artifact_bytes(artifact)).hexdigest(),
    }
    return artifact


def validate_artifact(artifact: Mapping[str, object]) -> None:
    if artifact.get("kind") != KIND:
        raise ValueError("artifact kind")
    profile = artifact.get("profile")
    if not isinstance(profile, dict) or profile.get("id") != PROFILE_ID:
        raise ValueError("profile identity")
    if profile.get("version") != PROFILE_VERSION:
        raise ValueError("profile version")
    digest = artifact.get("artifact_digest")
    if not isinstance(digest, dict) or not isinstance(digest.get("value"), str):
        raise ValueError("artifact digest shape")
    actual = hashlib.sha256(canonical_artifact_bytes(artifact)).hexdigest()
    if digest["value"] != actual:
        raise ValueError("artifact digest mismatch")
    rows = artifact.get("assertions")
    if not isinstance(rows, list) or [row.get("id") for row in rows if isinstance(row, dict)] != list(ASSERTION_IDS):
        raise ValueError("assertion order")
    if any(not isinstance(row, dict) or row.get("result") not in {"pass", "fail"} for row in rows):
        raise ValueError("assertion result")
    expected_result = "pass" if all(row["result"] == "pass" for row in rows) else "fail"
    if artifact.get("result") != expected_result:
        raise ValueError("root result")


def git_blob(revision: str, path: str) -> bytes:
    completed = subprocess.run(
        ["git", "-C", str(REPO_ROOT), "show", f"{revision}:{path}"],
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if completed.returncode != 0:
        raise ValueError(f"missing Git blob: {revision}:{path}")
    return completed.stdout


def resolve_revision(revision: str) -> str:
    completed = subprocess.run(
        ["git", "-C", str(REPO_ROOT), "rev-parse", "--verify", f"{revision}^{{commit}}"],
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if completed.returncode != 0:
        raise ValueError(f"invalid source revision: {revision}")
    return completed.stdout.strip()


def evaluate_revision(revision: str) -> dict[str, object]:
    source_revision = resolve_revision(revision)
    paths = {
        "CANON.md",
        "README.md",
        "AGENTS.md",
        "samples/clean-near-end/README.md",
        "mirrorea_canon/NORTH-STAR.md",
        "mirrorea_canon/DESIGN-CONSTITUTION.md",
        "mirrorea_canon/adr/ADR-0001.md",
        "mirrorea_canon/adr/ADR-0012.md",
        "mirrorea_canon/meta/source-hierarchy.md",
    }
    sources = {
        path: git_blob(source_revision, path).decode("utf-8") for path in paths
    }
    profile_bytes = git_blob(source_revision, PROFILE_PATH)
    producer_bytes = git_blob(source_revision, "scripts/evaluate_t0_semantic_assertions.py")
    artifact = make_artifact(
        source_revision=source_revision,
        profile_source_sha256=hashlib.sha256(profile_bytes).hexdigest(),
        producer_sha256=hashlib.sha256(producer_bytes).hexdigest(),
        rows=evaluate_assertions(sources),
    )
    validate_artifact(artifact)
    return artifact


def rendered_artifact(artifact: Mapping[str, object]) -> bytes:
    """Return the sole checked-in presentation form for a profile artifact."""

    return (json.dumps(artifact, ensure_ascii=False, indent=2) + "\n").encode("utf-8")


def evaluate_with_declared_producer(revision: str) -> dict[str, object]:
    """Run the producer blob declared by a source cut, not this checkout's code.

    The in-memory launcher gives the historical blob the canonical script path
    for its ``__file__``-relative repository lookup.  No temporary source file
    is written into the repository.  This keeps a later validator change from
    silently reinterpreting an older producer SHA as current evaluator logic.
    """

    source_revision = resolve_revision(revision)
    producer_bytes = git_blob(
        source_revision, "scripts/evaluate_t0_semantic_assertions.py"
    )
    declared_path = REPO_ROOT / "scripts" / "evaluate_t0_semantic_assertions.py"
    launcher = (
        "import sys\n"
        f"__file__ = {str(declared_path)!r}\n"
        "exec(compile(sys.stdin.buffer.read(), __file__, 'exec'))\n"
    )
    completed = subprocess.run(
        [sys.executable, "-c", launcher, "--revision", source_revision],
        cwd=REPO_ROOT,
        input=producer_bytes,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if completed.returncode != 0:
        detail = completed.stderr.decode("utf-8", errors="replace").strip()
        raise ValueError(f"declared producer rejected source revision: {detail}")
    try:
        artifact = json.loads(completed.stdout.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ValueError(f"declared producer JSON: {error}") from error
    if not isinstance(artifact, dict):
        raise ValueError("declared producer root")
    validate_artifact(artifact)
    return artifact


def validate_stored_artifact(path: Path) -> None:
    """Validate integrity, source binding, and exact declared-producer output."""

    raw = path.read_bytes()
    try:
        artifact = json.loads(raw.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ValueError(f"artifact JSON: {error}") from error
    if not isinstance(artifact, dict):
        raise ValueError("artifact root")
    validate_artifact(artifact)
    profile = artifact.get("profile")
    if not isinstance(profile, dict) or not isinstance(
        profile.get("source_revision"), str
    ):
        raise ValueError("profile source revision")
    reproduced = evaluate_with_declared_producer(profile["source_revision"])
    if raw != rendered_artifact(reproduced):
        raise ValueError("artifact reproduction mismatch")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    action = parser.add_mutually_exclusive_group(required=True)
    action.add_argument("--revision", help="Git commit to evaluate")
    action.add_argument(
        "--validate-artifact",
        metavar="PATH",
        help="validate a checked-in artifact and reproduce its declared source cut",
    )
    arguments = parser.parse_args(argv)
    try:
        if arguments.validate_artifact:
            validate_stored_artifact(Path(arguments.validate_artifact))
            print("semantic assertion artifact validation passed")
            return 0
        assert arguments.revision is not None
        artifact = evaluate_revision(arguments.revision)
    except ValueError as error:
        print(f"semantic assertion evaluation rejected: {error}", file=sys.stderr)
        return 2
    print(rendered_artifact(artifact).decode("utf-8"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
