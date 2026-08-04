#!/usr/bin/env python3

from __future__ import annotations

import json
import shutil
import subprocess
from dataclasses import dataclass
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
SAMPLES_ROOT = REPO_ROOT / "samples" / "lean"
FOUNDATIONS_ROOT = SAMPLES_ROOT / "foundations"
STATEMENT_DRAFTS_ROOT = SAMPLES_ROOT / "lab-statements"
CLEAN_ROOT = SAMPLES_ROOT / "clean-near-end"
MANIFEST_PATH = SAMPLES_ROOT / "manifest.json"
RUNTIME_PREFIX = [
    "cargo",
    "run",
    "-q",
    "-p",
    "mir-runtime",
    "--bin",
    "mir-clean-near-end",
    "--",
]


@dataclass(frozen=True)
class FoundationSpec:
    filename: str
    explanation_path: str


@dataclass(frozen=True)
class StatementDraftSpec:
    draft_id: str
    relative_dir: str
    filename: str
    explanation_path: str


FOUNDATIONS = [
    FoundationSpec(
        filename="CurrentL2LabelModel.lean",
        explanation_path="samples/lean/foundations/CurrentL2LabelModel.md",
    ),
    FoundationSpec(
        filename="CurrentL2IfcSecretExamples.lean",
        explanation_path="samples/lean/foundations/CurrentL2IfcSecretExamples.md",
    ),
    FoundationSpec(
        filename="CurrentL2FiniteIndexFirstLayer.lean",
        explanation_path="samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.md",
    ),
    FoundationSpec(
        filename="CurrentL2ProofSkeleton.lean",
        explanation_path="samples/lean/foundations/CurrentL2ProofSkeleton.md",
    ),
    FoundationSpec(
        filename="MirTheoryV0M3EvaluationMaterialization.lean",
        explanation_path="samples/lean/foundations/MirTheoryV0M3EvaluationMaterialization.md",
    ),
    FoundationSpec(
        filename="MirTheoryV0M4MaintainedRelationProjection.lean",
        explanation_path="samples/lean/foundations/MirTheoryV0M4MaintainedRelationProjection.md",
    ),
    FoundationSpec(
        filename="MirTheoryV0M5SharedModel.lean",
        explanation_path="samples/lean/foundations/MirTheoryV0M5SharedModel.md",
    ),
    FoundationSpec(
        filename="MirTheoryV0M6Surface.lean",
        explanation_path="samples/lean/foundations/MirTheoryV0M6Surface.md",
    ),
    FoundationSpec(
        filename="MirTheoryV0M7CheckedElaboration.lean",
        explanation_path="samples/lean/foundations/MirTheoryV0M7CheckedElaboration.md",
    ),
]


STATEMENT_DRAFTS = [
    StatementDraftSpec(
        draft_id="obl001-thm001-statement-draft",
        relative_dir="obl001",
        filename="THM001StatementDraft.lean",
        explanation_path="samples/lean/lab-statements/obl001/THM001StatementDraft.md",
    ),
    StatementDraftSpec(
        draft_id="obl020-step-wf-statement-draft",
        relative_dir="obl020",
        filename="StepWFStatementDraft.lean",
        explanation_path="samples/lean/lab-statements/obl020/StepWFStatementDraft.md",
    ),
    StatementDraftSpec(
        draft_id="obl021-elab-determinism-statement-draft",
        relative_dir="obl021",
        filename="ElabDeterminismStatementDraft.lean",
        explanation_path="samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.md",
    ),
    StatementDraftSpec(
        draft_id="obl024-diagnostic-soundness-statement-draft",
        relative_dir="obl024",
        filename="DiagnosticSoundnessStatementDraft.lean",
        explanation_path="samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md",
    ),
    StatementDraftSpec(
        draft_id="obl025-repair-completeness-statement-draft",
        relative_dir="obl025",
        filename="RepairCompletenessStatementDraft.lean",
        explanation_path="samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.md",
    ),
]


def runtime_json(*parts: str) -> object:
    completed = subprocess.run(
        [*RUNTIME_PREFIX, *parts, "--format", "json"],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return json.loads(completed.stdout)


def lean_version() -> str:
    completed = subprocess.run(
        ["lean", "--version"],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return completed.stdout.strip()


def verify_lean(path: Path) -> dict[str, object]:
    completed = subprocess.run(
        ["lean", "--trust=0", repo_relative_source_path(str(path))],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )
    return {
        "success": completed.returncode == 0,
        "stdout": completed.stdout,
        "stderr": completed.stderr,
        "returncode": completed.returncode,
    }


def sanitize_module_name(sample_id: str) -> str:
    pieces = ["CleanNearEnd"]
    for part in sample_id.split("_"):
        if part and part[0].isdigit():
            pieces.append(f"S{part}")
        elif part:
            pieces.append(part.capitalize())
    return ".".join(pieces)


def theorem_name(sample_id: str) -> str:
    stem = sample_id.replace("-", "_")
    if stem and stem[0].isdigit():
        stem = "s" + stem
    return stem + "__alpha_ready_subject"


def render_lean_stub(sample_id: str) -> str:
    module_name = sanitize_module_name(sample_id)
    thm_name = theorem_name(sample_id)
    return f"""namespace {module_name}

theorem {thm_name} : True := by
  trivial

end {module_name}
"""


def render_readme(sample_id: str, source_path: str) -> str:
    return f"""# {sample_id}

- Active clean near-end sample
- Source: `{source_path}`
- Lean stub status: repo-local alpha proof skeleton only
"""


def repo_relative_source_path(source_path: str) -> str:
    path = Path(source_path)
    resolved = path.resolve() if path.is_absolute() else (REPO_ROOT / path).resolve()
    try:
        return resolved.relative_to(REPO_ROOT).as_posix()
    except ValueError:
        return source_path


def sync_clean_stubs(sample_rows: list[dict[str, object]]) -> list[dict[str, object]]:
    if CLEAN_ROOT.exists():
        shutil.rmtree(CLEAN_ROOT)
    CLEAN_ROOT.mkdir(parents=True, exist_ok=True)

    entries: list[dict[str, object]] = []
    version = lean_version()
    for row in sample_rows:
        sample_id = str(row["sample_id"])
        source_path = repo_relative_source_path(str(row["source_path"]))
        sample_dir = CLEAN_ROOT / sample_id
        sample_dir.mkdir(parents=True, exist_ok=True)
        lean_path = sample_dir / f"{sample_id}.lean"
        bundle_path = sample_dir / f"{sample_id}.bundle.json"
        readme_path = sample_dir / "README.md"
        lean_path.write_text(render_lean_stub(sample_id), encoding="utf-8")
        bundle_path.write_text(
            json.dumps(
                {
                    "sample_id": sample_id,
                    "source_path": source_path,
                    "theorem_name": theorem_name(sample_id),
                },
                indent=2,
            )
            + "\n",
            encoding="utf-8",
        )
        readme_path.write_text(
            render_readme(sample_id, source_path),
            encoding="utf-8",
        )
        entries.append(
            {
                "kind": "clean-near-end",
                "sample_id": sample_id,
                "lean_path": str(lean_path.relative_to(REPO_ROOT)),
                "bundle_path": str(bundle_path.relative_to(REPO_ROOT)),
                "explanation_path": str(readme_path.relative_to(REPO_ROOT)),
                "lean_version": version,
                "theorem_names": [theorem_name(sample_id)],
                "verification": verify_lean(lean_path),
            }
        )
    return entries


def foundation_entries(version: str) -> list[dict[str, object]]:
    entries = []
    for spec in FOUNDATIONS:
        lean_path = FOUNDATIONS_ROOT / spec.filename
        entries.append(
            {
                "kind": "foundation",
                "filename": spec.filename,
                "lean_path": str(lean_path.relative_to(REPO_ROOT)),
                "explanation_path": spec.explanation_path,
                "lean_version": version,
                "verification": verify_lean(lean_path),
            }
        )
    return entries


def statement_draft_entries(version: str) -> list[dict[str, object]]:
    entries = []
    for spec in STATEMENT_DRAFTS:
        lean_path = STATEMENT_DRAFTS_ROOT / spec.relative_dir / spec.filename
        entries.append(
            {
                "kind": "statement-draft",
                "draft_id": spec.draft_id,
                "lean_path": str(lean_path.relative_to(REPO_ROOT)),
                "explanation_path": spec.explanation_path,
                "lean_version": version,
                "verification": verify_lean(lean_path),
                "status": "lab-compile-check-only",
            }
        )
    return entries


def main() -> int:
    sample_rows = runtime_json("list")
    version = lean_version()
    manifest = {
        "lean_version": version,
        "foundations": foundation_entries(version),
        "statement_drafts": statement_draft_entries(version),
        "clean_near_end": sync_clean_stubs(sample_rows),
    }
    MANIFEST_PATH.write_text(
        json.dumps(manifest, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )
    print(repo_relative_source_path(str(MANIFEST_PATH)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
