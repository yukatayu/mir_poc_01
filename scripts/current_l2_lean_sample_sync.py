#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
SAMPLES_ROOT = REPO_ROOT / "samples" / "lean"
CURRENT_L2_ROOT = SAMPLES_ROOT / "current-l2"
FOUNDATIONS_ROOT = SAMPLES_ROOT / "foundations"


@dataclass(frozen=True)
class CurrentL2ExportSpec:
    sample_id: str
    sample_argument: str
    host_plan_path: Path | None
    summary: str
    rationale: str


@dataclass(frozen=True)
class FoundationSpec:
    filename: str
    summary: str
    explanation: str
    source_text: str


def current_l2_export_specs() -> list[CurrentL2ExportSpec]:
    return [
        CurrentL2ExportSpec(
            sample_id="e5-underdeclared-lineage",
            sample_argument="e5-underdeclared-lineage",
            host_plan_path=None,
            summary="Static underdeclared omission sample with two representative proof obligations.",
            rationale=(
                "This is the narrow static-side anchor for current theorem review-unit and "
                "Lean-stub generation."
            ),
        ),
        CurrentL2ExportSpec(
            sample_id="p06-typed-proof-owner-handoff",
            sample_argument=str(
                REPO_ROOT
                / "samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt"
            ),
            host_plan_path=REPO_ROOT
            / "samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.host-plan.json",
            summary="Typed/theorem bridge prototype for proof-owner handoff.",
            rationale=(
                "This is the current typed/theorem representative prototype. It stays bridge-floor "
                "evidence rather than the final strong typed calculus."
            ),
        ),
        CurrentL2ExportSpec(
            sample_id="p07-dice-late-join-visible-history",
            sample_argument=str(
                REPO_ROOT
                / "samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt"
            ),
            host_plan_path=REPO_ROOT
            / "samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.host-plan.json",
            summary="Authoritative-room prototype for late join with visible published history.",
            rationale=(
                "This is the order/handoff representative runtime prototype for late-join semantics."
            ),
        ),
        CurrentL2ExportSpec(
            sample_id="p08-dice-stale-reconnect-refresh",
            sample_argument=str(
                REPO_ROOT
                / "samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt"
            ),
            host_plan_path=REPO_ROOT
            / "samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.host-plan.json",
            summary="Authoritative-room prototype for stale reconnect fail-then-refresh.",
            rationale=(
                "This is the order/handoff representative runtime prototype for stale reconnect handling."
            ),
        ),
    ]


def foundation_specs() -> list[FoundationSpec]:
    return [
        FoundationSpec(
            filename="CurrentL2LabelModel.lean",
            summary="Two-point IFC label model with explicit authority-sensitive declassification lemmas.",
            explanation=(
                "This file is the first actual Lean fragment for Package 56. It does not expose final "
                "source syntax; it pins the minimal label semantics and authority-sensitive facts that the "
                "checker-adjacent IFC line relies on."
            ),
            source_text="""/-!
current-l2 first IFC / label-model fragment

This file is intentionally small and self-contained. It is not the final public type
system. It fixes only the minimal lattice and authority-sensitive lemmas needed for the
current checker-adjacent reading.
-/

namespace CurrentL2

inductive SecurityLabel where
  | low
  | high
deriving DecidableEq, Repr

open SecurityLabel

def flowsTo : SecurityLabel → SecurityLabel → Prop
  | low, _ => True
  | high, high => True
  | high, low => False

def join : SecurityLabel → SecurityLabel → SecurityLabel
  | high, _ => high
  | _, high => high
  | low, low => low

def CanDeclassify (hasAuthority : Bool) (fromLabel toLabel : SecurityLabel) : Prop :=
  hasAuthority = true ∨ flowsTo fromLabel toLabel

theorem flowsTo_refl (label : SecurityLabel) : flowsTo label label := by
  cases label <;> simp [flowsTo]

theorem flowsTo_trans
    {a b c : SecurityLabel}
    (hab : flowsTo a b)
    (hbc : flowsTo b c) :
    flowsTo a c := by
  cases a <;> cases b <;> cases c <;> simp [flowsTo] at hab hbc ⊢

theorem flowsTo_join_left (a b : SecurityLabel) : flowsTo a (join a b) := by
  cases a <;> cases b <;> simp [flowsTo, join]

theorem flowsTo_join_right (a b : SecurityLabel) : flowsTo b (join a b) := by
  cases a <;> cases b <;> simp [flowsTo, join]

theorem secret_to_public_requires_authority :
    ¬ flowsTo high low := by
  simp [flowsTo]

theorem no_declassify_without_authority :
    ¬ CanDeclassify false high low := by
  simp [CanDeclassify, flowsTo]

theorem authority_enables_secret_to_public :
    CanDeclassify true high low := by
  simp [CanDeclassify]

end CurrentL2
""",
        ),
        FoundationSpec(
            filename="CurrentL2ProofSkeleton.lean",
            summary="Mechanization-ready proof-obligation skeleton for review-unit to Lean-stub alignment.",
            explanation=(
                "This file is the first actual Lean fragment for Package 57. It proves structural facts "
                "about the repo-local review-unit to Lean-stub bridge. It does not claim the domain "
                "obligations are solved; it fixes the shape of the mechanization-ready carrier."
            ),
            source_text="""/-!
current-l2 first proof-skeleton fragment

This file captures the structural part of the theorem-side bridge: review units, emitted
Lean stubs, and the invariant that emission preserves subject and obligation identity.
-/

namespace CurrentL2

inductive ObligationKind where
  | rollbackCutNonInterference
  | noRePromotion
deriving DecidableEq, Repr

open ObligationKind

def obligationName : ObligationKind → String
  | rollbackCutNonInterference => "rollback_cut_non_interference"
  | noRePromotion => "no_re_promotion"

structure ReviewUnit where
  subjectRef : String
  obligationKind : ObligationKind
deriving Repr

structure LeanStub where
  subjectRef : String
  obligationKind : ObligationKind
  theoremName : String
deriving Repr

def mkLeanStub (unit : ReviewUnit) : LeanStub :=
  {
    subjectRef := unit.subjectRef
    obligationKind := unit.obligationKind
    theoremName := unit.subjectRef ++ "__" ++ obligationName unit.obligationKind
  }

def emitStubs (units : List ReviewUnit) : List LeanStub :=
  units.map mkLeanStub

theorem mkLeanStub_preserves_subject (unit : ReviewUnit) :
    (mkLeanStub unit).subjectRef = unit.subjectRef := by
  simp [mkLeanStub]

theorem mkLeanStub_preserves_obligation (unit : ReviewUnit) :
    (mkLeanStub unit).obligationKind = unit.obligationKind := by
  simp [mkLeanStub]

theorem mkLeanStub_names_theorem (unit : ReviewUnit) :
    (mkLeanStub unit).theoremName = unit.subjectRef ++ "__" ++ obligationName unit.obligationKind := by
  simp [mkLeanStub]

theorem emitStubs_length (units : List ReviewUnit) :
    (emitStubs units).length = units.length := by
  simp [emitStubs]

def e5ReviewUnits : List ReviewUnit :=
  [
    { subjectRef := "e5-underdeclared-lineage", obligationKind := rollbackCutNonInterference },
    { subjectRef := "e5-underdeclared-lineage", obligationKind := noRePromotion }
  ]

theorem e5_emission_preserves_count :
    (emitStubs e5ReviewUnits).length = 2 := by
  simp [emitStubs, e5ReviewUnits]

theorem e5_first_stub_subject :
    match emitStubs e5ReviewUnits with
    | stub :: _ => stub.subjectRef = "e5-underdeclared-lineage"
    | _ => False := by
  simp [emitStubs, e5ReviewUnits, mkLeanStub]

end CurrentL2
""",
        ),
    ]


def build_current_l2_explanation(spec: CurrentL2ExportSpec) -> str:
    return f"""# {spec.sample_id}

## Summary

- {spec.summary}
- {spec.rationale}

## What This Lean File Means

- This file is generated from the repo-local theorem bridge and was accepted by `lean`.
- The generated theorem bodies still contain `sorry`, so the current guarantee is **artifact well-formedness and bridge alignment**, not full mathematical discharge.
- In concrete terms, the repo has verified that the review-unit to Lean-stub route produces syntactically valid Lean text for this sample and that the sample stays on the current theorem-first bridge floor.
- This is **not the final public theorem contract** and not the final proof-object schema.

## Why It Is Still Useful

- It preserves an inspectable snapshot of the actual Lean text attached to the current sample.
- It makes the current proof obligations concrete enough to compare across `e5`, `p06`, `p07`, and `p08`.
- It keeps the distinction between "Lean accepted the generated file" and "the domain theorem is fully proved" explicit.
"""


def build_foundation_explanation(spec: FoundationSpec) -> str:
    return f"""# {spec.filename}

## Summary

- {spec.summary}

## Why This File Exists

- {spec.explanation}
- Unlike the generated current-L2 sample stubs, this file contains actual small proofs rather than `sorry`.
- It is still helper-local and non-production. The goal is to pin the first mechanization-ready core, not to freeze the final public type system or verifier contract.
"""


def build_top_level_readme() -> str:
    current_ids = ", ".join(spec.sample_id for spec in current_l2_export_specs())
    return f"""# samples/lean

This directory records what the repo currently validates with Lean in a repo-local, inspectable form.

## Layout

- `foundations/`
  - small self-contained Lean files with actual proofs
  - current focus: IFC / label-model first fragment and proof-skeleton / obligation-shape first fragment
- `current-l2/`
  - generated Lean theorem stubs for the representative theorem quartet: {current_ids}
  - these files are accepted by Lean but still contain `sorry`

## Reading Rule

- `foundations/` shows the **mechanization-ready core** that is already precise enough to prove small facts.
- `current-l2/` shows the **actual emitted theorem bridge surface** that the repo generates for representative samples.
- The generated current-L2 stubs demonstrate artifact alignment and Lean acceptance, not completed theorem discharge.

## Rebuild

Run:

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

This regenerates the committed Lean sample corpus and verifies it with `lean`.
"""


def bundle_example_command(spec: CurrentL2ExportSpec, output_path: Path) -> list[str]:
    command = [
        "cargo",
        "run",
        "-p",
        "mir-runtime",
        "--example",
        "current_l2_emit_theorem_lean_bundle",
        "--",
        spec.sample_argument,
    ]
    if spec.host_plan_path is not None:
        command.extend(["--host-plan", str(spec.host_plan_path)])
    command.extend(["--output", str(output_path)])
    return command


def ensure_lean_available() -> str:
    lean_path = shutil.which("lean")
    if lean_path is None:
        candidate = Path.home() / ".elan" / "bin" / "lean"
        if candidate.is_file():
            lean_path = str(candidate)
    if lean_path is None:
        raise RuntimeError("lean command not found in PATH")
    return lean_path


def lean_version_line(lean_path: str) -> str:
    completed = subprocess.run(
        [lean_path, "--version"],
        check=True,
        capture_output=True,
        text=True,
        cwd=REPO_ROOT,
    )
    return completed.stdout.strip().splitlines()[0]


def run_lean(lean_path: str, source_path: Path) -> dict[str, object]:
    completed = subprocess.run(
        [lean_path, str(source_path)],
        check=False,
        capture_output=True,
        text=True,
        cwd=REPO_ROOT,
    )
    return {
        "success": completed.returncode == 0,
        "stdout": completed.stdout,
        "stderr": completed.stderr,
        "returncode": completed.returncode,
    }


def write_text(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def sync_foundations(lean_path: str, version_line: str) -> list[dict[str, object]]:
    results: list[dict[str, object]] = []

    for spec in foundation_specs():
        lean_path_out = FOUNDATIONS_ROOT / spec.filename
        explanation_path = lean_path_out.with_suffix(".md")
        write_text(lean_path_out, spec.source_text)
        write_text(explanation_path, build_foundation_explanation(spec))

        verification = run_lean(lean_path, lean_path_out)
        if not verification["success"]:
            raise RuntimeError(
                f"Lean verification failed for foundation sample {lean_path_out}: "
                f"{verification['stderr']}"
            )
        results.append(
            {
                "kind": "foundation",
                "filename": spec.filename,
                "lean_path": str(lean_path_out.relative_to(REPO_ROOT)),
                "explanation_path": str(explanation_path.relative_to(REPO_ROOT)),
                "lean_version": version_line,
                "verification": verification,
            }
        )

    return results


def sync_current_l2(lean_path: str, version_line: str) -> list[dict[str, object]]:
    results: list[dict[str, object]] = []

    for spec in current_l2_export_specs():
        with tempfile.TemporaryDirectory() as temp_dir:
            bundle_path = Path(temp_dir) / f"{spec.sample_id}.bundle.json"
            subprocess.run(
                bundle_example_command(spec, bundle_path),
                check=True,
                cwd=REPO_ROOT,
            )
            bundle = json.loads(bundle_path.read_text(encoding="utf-8"))

        if bundle["pilot_status"] != "reached":
            raise RuntimeError(
                f"current-l2 Lean bundle did not reach theorem route for {spec.sample_id}: "
                f"{bundle['pilot_guard_reason']}"
            )
        if not bundle["lean_stub_artifacts"]:
            raise RuntimeError(f"no Lean stub artifacts emitted for {spec.sample_id}")

        sample_root = CURRENT_L2_ROOT / spec.sample_id
        sample_root.mkdir(parents=True, exist_ok=True)
        lean_output_path = sample_root / f"{spec.sample_id}.lean"
        explanation_path = sample_root / "README.md"
        bundle_output_path = sample_root / f"{spec.sample_id}.bundle.json"

        source_text = "\n".join(
            artifact["source_text"] for artifact in bundle["lean_stub_artifacts"]
        )
        write_text(lean_output_path, source_text)
        write_text(explanation_path, build_current_l2_explanation(spec))
        write_text(bundle_output_path, json.dumps(bundle, indent=2) + "\n")

        verification = run_lean(lean_path, lean_output_path)
        if not verification["success"]:
            raise RuntimeError(
                f"Lean verification failed for current-l2 sample {spec.sample_id}: "
                f"{verification['stderr']}"
            )

        results.append(
            {
                "kind": "current-l2",
                "sample_id": spec.sample_id,
                "lean_path": str(lean_output_path.relative_to(REPO_ROOT)),
                "explanation_path": str(explanation_path.relative_to(REPO_ROOT)),
                "bundle_path": str(bundle_output_path.relative_to(REPO_ROOT)),
                "lean_version": version_line,
                "theorem_names": [
                    artifact["theorem_name"] for artifact in bundle["lean_stub_artifacts"]
                ],
                "verification": verification,
            }
        )

    return results


def sync_samples() -> dict[str, object]:
    lean_path = ensure_lean_available()
    version_line = lean_version_line(lean_path)

    write_text(SAMPLES_ROOT / "README.md", build_top_level_readme())
    foundations = sync_foundations(lean_path, version_line)
    current_l2 = sync_current_l2(lean_path, version_line)

    manifest = {
        "lean_version": version_line,
        "foundations": foundations,
        "current_l2": current_l2,
    }
    write_text(SAMPLES_ROOT / "manifest.json", json.dumps(manifest, indent=2) + "\n")
    return manifest


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Regenerate the committed Lean sample corpus for the current L2 theorem bridge "
            "and first IFC/proof-skeleton fragments."
        )
    )
    return parser.parse_args()


def main() -> int:
    parse_args()
    manifest = sync_samples()
    print(json.dumps(manifest, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
