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
            summary="代表的な 2 本の proof obligation を持つ static underdeclared omission sample。",
            rationale=(
                "current theorem review-unit と Lean stub 生成を結ぶ、"
                "最小の static-side anchor として読む。"
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
            summary="proof owner handoff を表す typed/theorem bridge prototype。",
            rationale=(
                "current typed/theorem representative prototype であり、"
                "final strong typed calculus ではなく bridge-floor evidence に留める。"
            ),
        ),
        CurrentL2ExportSpec(
            sample_id="p10-typed-authorized-fingerprint-declassification",
            sample_argument=str(
                REPO_ROOT
                / "samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt"
            ),
            host_plan_path=REPO_ROOT
            / "samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.host-plan.json",
            summary="authority あり declassification を表す typed/IFC prototype。",
            rationale=(
                "current checker-adjacent IFC line で、authority-sensitive release が "
                "success 側にどう映るかを見る representative prototype として読む。"
            ),
        ),
        CurrentL2ExportSpec(
            sample_id="p11-typed-unauthorized-fingerprint-release",
            sample_argument=str(
                REPO_ROOT
                / "samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt"
            ),
            host_plan_path=REPO_ROOT
            / "samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.host-plan.json",
            summary="authority なし release を explicit failure で止める typed/IFC prototype。",
            rationale=(
                "current checker-adjacent IFC line で、holder と release authority を "
                "分けた negative evidence を sample-visible に保つ representative prototype として読む。"
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
            summary="公開済み history を過去として見せる late join の authoritative-room prototype。",
            rationale=(
                "late-join semantics を current order/handoff line で見る representative runtime prototype として読む。"
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
            summary="stale reconnect を fail-then-refresh で扱う authoritative-room prototype。",
            rationale=(
                "stale reconnect handling を current order/handoff line で見る representative runtime prototype として読む。"
            ),
        ),
    ]


def foundation_specs() -> list[FoundationSpec]:
    return [
        FoundationSpec(
            filename="CurrentL2LabelModel.lean",
            summary="明示的 authority-sensitive declassification lemma を持つ two-point IFC label model。",
            explanation=(
                "Package 56 の最初の actual Lean fragment である。"
                "final source syntax は出さず、checker-adjacent IFC line が依拠する "
                "最小 label semantics と authority-sensitive fact を固定する。"
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
            filename="CurrentL2IfcSecretExamples.lean",
            summary="secret-key valid/invalid と explicit authority declassification を固定する IFC concrete example 集。",
            explanation=(
                "Package 56 の first-fragment を label model の定義だけで止めず、"
                "secret-key valid/invalid と explicit authority declassification を "
                "mechanization-ready な concrete example として置く。"
            ),
            source_text="""/-!
current-l2 IFC secret examples fragment

This file keeps the first concrete IFC examples for Package 56 in one self-contained
Lean artifact. The goal is not the final public typed surface. The goal is to make the
secret-key valid/invalid reading executable at the proof-fragment level.
-/

namespace CurrentL2IfcSecretExamples

inductive SecurityLabel where
  | low
  | high
deriving DecidableEq, Repr

open SecurityLabel

def flowsTo : SecurityLabel → SecurityLabel → Prop
  | low, _ => True
  | high, high => True
  | high, low => False

def CanDeclassify (hasAuthority : Bool) (fromLabel toLabel : SecurityLabel) : Prop :=
  hasAuthority = true ∨ flowsTo fromLabel toLabel

structure Labeled (label : SecurityLabel) (α : Type) where
  value : α

abbrev SecretKey := Labeled high String
abbrev SecretFingerprint := Labeled high String
abbrev PublicFingerprint := Labeled low String

def fingerprint (key : SecretKey) : SecretFingerprint :=
  { value := "fp:" ++ key.value }

def declassify
    (hasAuthority : Bool)
    {fromLabel toLabel : SecurityLabel}
    (_proof : CanDeclassify hasAuthority fromLabel toLabel)
    (value : Labeled fromLabel α) :
    Labeled toLabel α :=
  { value := value.value }

theorem no_secret_release_without_authority :
    ¬ CanDeclassify false high low := by
  simp [CanDeclassify, flowsTo]

theorem authorized_secret_release_is_available :
    CanDeclassify true high low := by
  simp [CanDeclassify]

def liveSecretKey : SecretKey :=
  { value := "sk_live" }

def authorizedPublicFingerprint : PublicFingerprint :=
  declassify true authorized_secret_release_is_available (fingerprint liveSecretKey)

theorem authorized_public_fingerprint_keeps_payload :
    authorizedPublicFingerprint.value = "fp:sk_live" := by
  rfl

theorem invalid_release_has_no_authority_proof :
    ¬ ∃ _proof : CanDeclassify false high low, True := by
  intro h
  rcases h with ⟨proof, _⟩
  exact no_secret_release_without_authority proof

theorem valid_release_has_authority_proof :
    ∃ _proof : CanDeclassify true high low, True := by
  exact ⟨authorized_secret_release_is_available, trivial⟩

end CurrentL2IfcSecretExamples
""",
        ),
        FoundationSpec(
            filename="CurrentL2ProofSkeleton.lean",
            summary="review-unit と Lean-stub の整合を固定する mechanization-ready proof-obligation skeleton。",
            explanation=(
                "Package 57 の最初の actual Lean fragment である。"
                "repo-local review-unit to Lean-stub bridge の構造的 fact を証明し、"
                "domain obligation が解けたとは主張せず mechanization-ready carrier の shape を固定する。"
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
    current_ids = " / ".join(export_spec.sample_id for export_spec in current_l2_export_specs())
    return f"""# {spec.sample_id}

## 要約

- {spec.summary}
- {spec.rationale}

## この Lean ファイルが意味すること

- この Lean ファイルは repo-local theorem bridge から生成されたもので、`lean` に受理される。
- 生成された theorem body にはまだ `sorry` が残るため、現時点の保証は **artifact well-formedness and bridge alignment** であり、完全な mathematical discharge ではない。
- 具体的には、review-unit から Lean stub への route がこの sample に対して構文的に正しい Lean text を出し、sample が current theorem-first bridge floor に留まっていることを repo が確認した。
- これは最終的な public theorem contract でも final proof-object schema でもない。

## それでも保持する理由

- current sample に結び付いた actual Lean text を inspectable snapshot として保持できる。
- `{current_ids}` のあいだで current proof obligation を具体物として比較できる。
- 「Lean が生成ファイルを受理した」ことと「domain theorem が fully proved である」ことを明示的に分けたままにできる。
"""


def build_foundation_explanation(spec: FoundationSpec) -> str:
    return f"""# {spec.filename}

## 要約

- {spec.summary}

## このファイルを置く理由

- {spec.explanation}
- 生成された current-L2 sample stub と違い、このファイルは `sorry` ではなく実際に小さな証明を含む。
- ただし依然として helper-local / non-production cut に留める。目的は first mechanization-ready core を固定することであり、final public type system や verifier contract を凍らせることではない。
"""


def build_top_level_readme() -> str:
    current_ids = ", ".join(spec.sample_id for spec in current_l2_export_specs())
    return f"""# samples/lean

このディレクトリは、repo が現在 Lean でどこまで検証しているかを、
repo-local かつ inspectable な形で保存する。

## 構成

- `foundations/`
  - 実際に小さな証明を含む self-contained Lean file を置く
  - 現在の主眼は IFC / label-model first fragment、secret valid/invalid concrete example、proof-skeleton / obligation-shape first fragment である
- `current-l2/`
  - 現在の current-L2 定理ブリッジから representative sample set `{current_ids}` 向けに生成された Lean theorem stub を置く
  - これらの file は Lean に受理されるが、まだ `sorry` を含む

## 読み方

- `foundations/` は、すでに小さな fact を証明できる **mechanization-ready core** を示す。
- `current-l2/` は、repo が representative sample から生成する **actual emitted theorem bridge surface** を示す。
- generated current-L2 stub は artifact alignment と Lean acceptance を示すのであって、completed theorem discharge を示すものではない。

## 再生成

次を実行する:

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

これにより committed Lean sample corpus を再生成し、`lean` で検証する。
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
            "current L2 theorem bridge と first IFC / proof-skeleton / secret-example fragments "
            "に対する committed Lean sample corpus を再生成する。"
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
