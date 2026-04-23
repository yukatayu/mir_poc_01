# clean near-end Lean 01 detail

## この文書の役割

この文書は、current Lean line の **actual foundation code**、**generated stub の実体**、**verification result** を確認するための detail です。
sample 本体の valid / malformed / counterexample case は別 detail に掲載しているので、ここでは Lean 側の proof spine に集中します。

## 実行した command

2026-04-23 に次を実行しました。

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

### actual output

```text
/home/yukatayu/dev/mir_poc_01/samples/lean/manifest.json
```

## built-in と user-defined の境界

### Lean built-in

- `namespace`
- `inductive`
- `structure`
- `def`
- `theorem`
- `Bool`
- `Nat`
- `Prop`
- `String`
- `List`

### foundation file が定義している user-defined concept

- `SecurityLabel`
- `flowsTo`
- `join`
- `CanDeclassify`
- `Lifetime`
- `Capability`
- `CaptureSet`
- `ReviewUnit`
- `LeanStub`
- `mkLeanStub`

重要なのは、Mir 側の domain predicate も Lean built-in ではないことです。
foundation file の中で最小モデルとして定義しています。

## foundation file 全文

## `CurrentL2LabelModel.lean`

```lean
/-!
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

theorem low_flows_to_any (label : SecurityLabel) : flowsTo low label := by
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
```

## `CurrentL2IfcSecretExamples.lean`

```lean
/-!
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

theorem declassify_preserves_value
    (hasAuthority : Bool)
    {fromLabel toLabel : SecurityLabel}
    (proof : CanDeclassify hasAuthority fromLabel toLabel)
    (value : Labeled fromLabel α) :
    (declassify hasAuthority proof value).value = value.value := by
  rfl

theorem no_secret_release_without_authority :
    ¬ CanDeclassify false high low := by
  simp [CanDeclassify, flowsTo]

theorem authorized_secret_release_is_available :
    CanDeclassify true high low := by
  simp [CanDeclassify]

theorem low_to_low_release_without_authority_is_available :
    CanDeclassify false low low := by
  simp [CanDeclassify, flowsTo]

def publicAuditNote : Labeled low String :=
  { value := "audit-ok" }

def unchangedPublicAuditNote : Labeled low String :=
  declassify false low_to_low_release_without_authority_is_available publicAuditNote

theorem unchanged_public_audit_note_keeps_payload :
    unchangedPublicAuditNote.value = "audit-ok" := by
  simpa [unchangedPublicAuditNote, publicAuditNote] using
    declassify_preserves_value
      false
      low_to_low_release_without_authority_is_available
      publicAuditNote

def liveSecretKey : SecretKey :=
  { value := "sk_live" }

theorem fingerprint_keeps_secret_payload :
    (fingerprint liveSecretKey).value = "fp:sk_live" := by
  rfl

def authorizedPublicFingerprint : PublicFingerprint :=
  declassify true authorized_secret_release_is_available (fingerprint liveSecretKey)

theorem authorized_public_fingerprint_keeps_payload :
    authorizedPublicFingerprint.value = "fp:sk_live" := by
  simpa [authorizedPublicFingerprint, fingerprint, liveSecretKey] using
    declassify_preserves_value
      true
      authorized_secret_release_is_available
      (fingerprint liveSecretKey)

theorem invalid_release_has_no_authority_proof :
    ¬ ∃ _proof : CanDeclassify false high low, True := by
  intro h
  rcases h with ⟨proof, _⟩
  exact no_secret_release_without_authority proof

theorem valid_release_has_authority_proof :
    ∃ _proof : CanDeclassify true high low, True := by
  exact ⟨authorized_secret_release_is_available, trivial⟩

theorem authorized_live_fingerprint_release_has_witness :
    ∃ proof : CanDeclassify true high low,
      (declassify true proof (fingerprint liveSecretKey)).value = "fp:sk_live" := by
  refine ⟨authorized_secret_release_is_available, ?_⟩
  simpa [fingerprint, liveSecretKey] using
    declassify_preserves_value
      true
      authorized_secret_release_is_available
      (fingerprint liveSecretKey)

theorem unauthorized_live_fingerprint_release_is_impossible :
    ¬ ∃ proof : CanDeclassify false high low,
      (declassify false proof (fingerprint liveSecretKey)).value = "fp:sk_live" := by
  intro h
  rcases h with ⟨proof, _⟩
  exact no_secret_release_without_authority proof

end CurrentL2IfcSecretExamples
```

## `CurrentL2FiniteIndexFirstLayer.lean`

```lean
/-!
current-l2 finite-index first-layer fragment

This file keeps the smallest self-contained Lean facts for the finite-index first layer:
capture-set inclusion, lifetime preorder, and simple remote-call budget.
It is not the final public typed calculus.
-/

namespace CurrentL2FiniteIndexFirstLayer

inductive Lifetime where
  | step
  | session
deriving DecidableEq, Repr

open Lifetime

def outlives : Lifetime → Lifetime → Prop
  | session, _ => True
  | step, step => True
  | step, session => False

theorem outlives_refl (lifetime : Lifetime) : outlives lifetime lifetime := by
  cases lifetime <;> simp [outlives]

theorem session_outlives_step : outlives session step := by
  simp [outlives]

theorem step_does_not_outlive_session : ¬ outlives step session := by
  simp [outlives]

theorem outlives_trans
    {a b c : Lifetime}
    (hab : outlives a b)
    (hbc : outlives b c) :
    outlives a c := by
  cases a <;> cases b <;> cases c <;> simp [outlives] at hab hbc ⊢

inductive Capability where
  | roomHistory
  | ephemeralToken
deriving DecidableEq, Repr

open Capability

abbrev CaptureSet := Capability → Bool

def captureSubset (lhs rhs : CaptureSet) : Prop :=
  ∀ capability, lhs capability = true → rhs capability = true

def emptyCapture : CaptureSet := fun _ => false

def fullCapture : CaptureSet := fun _ => true

def ephemeralOnly : CaptureSet
  | ephemeralToken => true
  | roomHistory => false

def roomHistoryOnly : CaptureSet
  | roomHistory => true
  | ephemeralToken => false

theorem capture_subset_refl (captures : CaptureSet) : captureSubset captures captures := by
  intro capability h
  exact h

theorem empty_capture_subset (captures : CaptureSet) :
    captureSubset emptyCapture captures := by
  intro capability h
  simp [emptyCapture] at h

theorem capture_subset_trans
    {capturesA capturesB capturesC : CaptureSet}
    (hab : captureSubset capturesA capturesB)
    (hbc : captureSubset capturesB capturesC) :
    captureSubset capturesA capturesC := by
  intro capability h
  exact hbc capability (hab capability h)

theorem ephemeral_only_subset_of_full_capture :
    captureSubset ephemeralOnly fullCapture := by
  intro capability h
  simp [fullCapture]

theorem ephemeral_only_not_subset_of_empty :
    ¬ captureSubset ephemeralOnly emptyCapture := by
  intro h
  have hToken := h ephemeralToken rfl
  simp [emptyCapture] at hToken

theorem room_history_only_not_subset_of_ephemeral_only :
    ¬ captureSubset roomHistoryOnly ephemeralOnly := by
  intro h
  have hHistory := h roomHistory rfl
  simp [ephemeralOnly] at hHistory

def remoteCallAllowed (remainingCalls : Nat) : Prop :=
  0 < remainingCalls

def spendRemoteCall : Nat → Nat
  | 0 => 0
  | remainingCalls + 1 => remainingCalls

theorem zero_budget_rejects_remote_call :
    ¬ remoteCallAllowed 0 := by
  simp [remoteCallAllowed]

theorem positive_budget_allows_remote_call :
    remoteCallAllowed 1 := by
  simp [remoteCallAllowed]

theorem succ_budget_allows_remote_call (remainingCalls : Nat) :
    remoteCallAllowed (Nat.succ remainingCalls) := by
  simp [remoteCallAllowed]

theorem single_budget_is_exhausted_after_one_call :
    ¬ remoteCallAllowed (spendRemoteCall 1) := by
  simp [spendRemoteCall, remoteCallAllowed]

theorem two_budget_still_allows_after_one_call :
    remoteCallAllowed (spendRemoteCall 2) := by
  simp [spendRemoteCall, remoteCallAllowed]

end CurrentL2FiniteIndexFirstLayer
```

## `CurrentL2ProofSkeleton.lean`

```lean
/-!
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

theorem e5_second_stub_obligation :
    match emitStubs e5ReviewUnits with
    | _ :: stub :: _ => stub.obligationKind = noRePromotion
    | _ => False := by
  simp [emitStubs, e5ReviewUnits, mkLeanStub]

end CurrentL2
```

## representative generated stub

generated stub は full domain proof ではなく、sample ごとの theorem bridge の足場です。
current active suite 全 sample に対応する stub が `samples/lean/clean-near-end/` にあります。ここでは family をまたぐ代表例を載せます。

## typing valid

```lean
namespace CleanNearEnd.S01.Authorized.Declassification

theorem s01_authorized_declassification__alpha_ready_subject : True := by
  trivial

end CleanNearEnd.S01.Authorized.Declassification
```

## typing rejected

```lean
namespace CleanNearEnd.S04.Capture.Escape.Rejected

theorem s04_capture_escape_rejected__alpha_ready_subject : True := by
  trivial

end CleanNearEnd.S04.Capture.Escape.Rejected
```

## model-check counterexample

```lean
namespace CleanNearEnd.S02.Peterson.Relaxed.Counterexample

theorem s02_peterson_relaxed_counterexample__alpha_ready_subject : True := by
  trivial

end CleanNearEnd.S02.Peterson.Relaxed.Counterexample
```

## modal bridge

```lean
namespace CleanNearEnd.S02.Published.Witnessed.Mode.Bridge

theorem s02_published_witnessed_mode_bridge__alpha_ready_subject : True := by
  trivial

end CleanNearEnd.S02.Published.Witnessed.Mode.Bridge
```

## actual manifest

`python3 scripts/current_l2_lean_sample_sync.py` が更新・検証した `samples/lean/manifest.json` の内容です。

```json
{
  "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
  "foundations": [
    {
      "kind": "foundation",
      "filename": "CurrentL2LabelModel.lean",
      "lean_path": "samples/lean/foundations/CurrentL2LabelModel.lean",
      "explanation_path": "samples/lean/foundations/CurrentL2LabelModel.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "foundation",
      "filename": "CurrentL2IfcSecretExamples.lean",
      "lean_path": "samples/lean/foundations/CurrentL2IfcSecretExamples.lean",
      "explanation_path": "samples/lean/foundations/CurrentL2IfcSecretExamples.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "foundation",
      "filename": "CurrentL2FiniteIndexFirstLayer.lean",
      "lean_path": "samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean",
      "explanation_path": "samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "foundation",
      "filename": "CurrentL2ProofSkeleton.lean",
      "lean_path": "samples/lean/foundations/CurrentL2ProofSkeleton.lean",
      "explanation_path": "samples/lean/foundations/CurrentL2ProofSkeleton.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    }
  ],
  "clean_near_end": [
    {
      "kind": "clean-near-end",
      "sample_id": "01_authorized_declassification",
      "lean_path": "samples/lean/clean-near-end/01_authorized_declassification/01_authorized_declassification.lean",
      "bundle_path": "samples/lean/clean-near-end/01_authorized_declassification/01_authorized_declassification.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/01_authorized_declassification/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s01_authorized_declassification__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "clean-near-end",
      "sample_id": "02_unauthorized_declassification_rejected",
      "lean_path": "samples/lean/clean-near-end/02_unauthorized_declassification_rejected/02_unauthorized_declassification_rejected.lean",
      "bundle_path": "samples/lean/clean-near-end/02_unauthorized_declassification_rejected/02_unauthorized_declassification_rejected.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/02_unauthorized_declassification_rejected/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s02_unauthorized_declassification_rejected__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "clean-near-end",
      "sample_id": "03_label_flow_rejected",
      "lean_path": "samples/lean/clean-near-end/03_label_flow_rejected/03_label_flow_rejected.lean",
      "bundle_path": "samples/lean/clean-near-end/03_label_flow_rejected/03_label_flow_rejected.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/03_label_flow_rejected/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s03_label_flow_rejected__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "clean-near-end",
      "sample_id": "04_capture_escape_rejected",
      "lean_path": "samples/lean/clean-near-end/04_capture_escape_rejected/04_capture_escape_rejected.lean",
      "bundle_path": "samples/lean/clean-near-end/04_capture_escape_rejected/04_capture_escape_rejected.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/04_capture_escape_rejected/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s04_capture_escape_rejected__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "clean-near-end",
      "sample_id": "05_cost_bound_rejected",
      "lean_path": "samples/lean/clean-near-end/05_cost_bound_rejected/05_cost_bound_rejected.lean",
      "bundle_path": "samples/lean/clean-near-end/05_cost_bound_rejected/05_cost_bound_rejected.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/05_cost_bound_rejected/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s05_cost_bound_rejected__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "clean-near-end",
      "sample_id": "01_authorized_roll_publish_handoff",
      "lean_path": "samples/lean/clean-near-end/01_authorized_roll_publish_handoff/01_authorized_roll_publish_handoff.lean",
      "bundle_path": "samples/lean/clean-near-end/01_authorized_roll_publish_handoff/01_authorized_roll_publish_handoff.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/01_authorized_roll_publish_handoff/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s01_authorized_roll_publish_handoff__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "clean-near-end",
      "sample_id": "02_missing_witness_rejected",
      "lean_path": "samples/lean/clean-near-end/02_missing_witness_rejected/02_missing_witness_rejected.lean",
      "bundle_path": "samples/lean/clean-near-end/02_missing_witness_rejected/02_missing_witness_rejected.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/02_missing_witness_rejected/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s02_missing_witness_rejected__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "clean-near-end",
      "sample_id": "03_handoff_before_publication_rejected",
      "lean_path": "samples/lean/clean-near-end/03_handoff_before_publication_rejected/03_handoff_before_publication_rejected.lean",
      "bundle_path": "samples/lean/clean-near-end/03_handoff_before_publication_rejected/03_handoff_before_publication_rejected.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/03_handoff_before_publication_rejected/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s03_handoff_before_publication_rejected__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "clean-near-end",
      "sample_id": "04_stage_block_authorized_handoff",
      "lean_path": "samples/lean/clean-near-end/04_stage_block_authorized_handoff/04_stage_block_authorized_handoff.lean",
      "bundle_path": "samples/lean/clean-near-end/04_stage_block_authorized_handoff/04_stage_block_authorized_handoff.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/04_stage_block_authorized_handoff/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s04_stage_block_authorized_handoff__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "clean-near-end",
      "sample_id": "05_delegated_rng_service",
      "lean_path": "samples/lean/clean-near-end/05_delegated_rng_service/05_delegated_rng_service.lean",
      "bundle_path": "samples/lean/clean-near-end/05_delegated_rng_service/05_delegated_rng_service.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/05_delegated_rng_service/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s05_delegated_rng_service__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "clean-near-end",
      "sample_id": "06_auditable_authority_witness",
      "lean_path": "samples/lean/clean-near-end/06_auditable_authority_witness/06_auditable_authority_witness.lean",
      "bundle_path": "samples/lean/clean-near-end/06_auditable_authority_witness/06_auditable_authority_witness.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/06_auditable_authority_witness/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s06_auditable_authority_witness__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "clean-near-end",
      "sample_id": "01_peterson_sc_pass",
      "lean_path": "samples/lean/clean-near-end/01_peterson_sc_pass/01_peterson_sc_pass.lean",
      "bundle_path": "samples/lean/clean-near-end/01_peterson_sc_pass/01_peterson_sc_pass.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/01_peterson_sc_pass/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s01_peterson_sc_pass__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "clean-near-end",
      "sample_id": "02_peterson_relaxed_counterexample",
      "lean_path": "samples/lean/clean-near-end/02_peterson_relaxed_counterexample/02_peterson_relaxed_counterexample.lean",
      "bundle_path": "samples/lean/clean-near-end/02_peterson_relaxed_counterexample/02_peterson_relaxed_counterexample.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/02_peterson_relaxed_counterexample/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s02_peterson_relaxed_counterexample__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "clean-near-end",
      "sample_id": "03_broken_mutex_counterexample",
      "lean_path": "samples/lean/clean-near-end/03_broken_mutex_counterexample/03_broken_mutex_counterexample.lean",
      "bundle_path": "samples/lean/clean-near-end/03_broken_mutex_counterexample/03_broken_mutex_counterexample.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/03_broken_mutex_counterexample/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s03_broken_mutex_counterexample__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "clean-near-end",
      "sample_id": "01_stage_stable_later_minimal",
      "lean_path": "samples/lean/clean-near-end/01_stage_stable_later_minimal/01_stage_stable_later_minimal.lean",
      "bundle_path": "samples/lean/clean-near-end/01_stage_stable_later_minimal/01_stage_stable_later_minimal.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/01_stage_stable_later_minimal/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s01_stage_stable_later_minimal__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    },
    {
      "kind": "clean-near-end",
      "sample_id": "02_published_witnessed_mode_bridge",
      "lean_path": "samples/lean/clean-near-end/02_published_witnessed_mode_bridge/02_published_witnessed_mode_bridge.lean",
      "bundle_path": "samples/lean/clean-near-end/02_published_witnessed_mode_bridge/02_published_witnessed_mode_bridge.bundle.json",
      "explanation_path": "samples/lean/clean-near-end/02_published_witnessed_mode_bridge/README.md",
      "lean_version": "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)",
      "theorem_names": [
        "s02_published_witnessed_mode_bridge__alpha_ready_subject"
      ],
      "verification": {
        "success": true,
        "stdout": "",
        "stderr": "",
        "returncode": 0
      }
    }
  ]
}
```
