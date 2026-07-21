import samples.lean.«lab-statements».obl021.ElabDeterminismStatementDraft

/-!
LAB-only countermodel for outcome existence in the OBL-021 statement-shape
draft. A well-scoped input has neither a successful Result nor a Diagnostic.
The file checks only that the draft's pairwise clauses do not require an
outcome; it does not assign any future totality law to Canon or an OBL.
-/

namespace MirCore.Lab.OBL021.OutcomeTotalityCountermodel

open MirCore.Lab.OBL021.StatementDraft

def V : Vocab where
  Env := Unit
  Ctx := Unit
  Locus := Unit
  SurfaceItem := Unit
  Result := Unit
  Diagnostic := Unit
  CoreTerm := Unit
  TypeOut := Unit
  ModeOut := Unit
  EffectRow := Unit
  FailureRow := Unit
  ConstraintSet := Unit
  ObligationSet := Unit
  GeneratedEdgeSet := Unit
  SourceSpanMap := Unit

def P : Pred V where
  WellScopedInput := fun _ _ _ _ => True
  Elaborates := fun _ _ _ _ _ => False
  Rejects := fun _ _ _ _ _ => False
  CoreTermOf := fun _ _ => False
  TypeOf := fun _ _ => False
  ModeOf := fun _ _ => False
  EffectRowOf := fun _ _ => False
  FailureRowOf := fun _ _ => False
  ConstraintsOf := fun _ _ => False
  ObligationsOf := fun _ _ => False
  GeneratedEdgesOf := fun _ _ => False
  SourceSpansOf := fun _ _ => False
  EquivalentCoreTerm := fun _ _ => True
  EquivalentTypeOut := fun _ _ => True
  EquivalentModeOut := fun _ _ => True
  EquivalentEffectRow := fun _ _ => True
  EquivalentFailureRow := fun _ _ => True
  EquivalentConstraintSet := fun _ _ => True
  EquivalentObligationSet := fun _ _ => True
  EquivalentGeneratedEdges := fun _ _ => True
  EquivalentSourceSpanMap := fun _ _ => True
  EquivalentDiagnostic := fun _ _ => True

theorem well_scoped_input_exists : P.WellScopedInput () () () () := by
  simp [P]

theorem no_successful_result_exists :
    ¬ Exists fun result : V.Result => P.Elaborates () () () () result := by
  simp [P]

theorem no_diagnostic_exists :
    ¬ Exists fun diagnostic : V.Diagnostic => P.Rejects () () () () diagnostic := by
  simp [P]

theorem statement_draft_holds : OBL021StatementDraft V P := by
  simp [OBL021StatementDraft, ElabDeterministicPost, SameElabResult,
    SameDiagnostic, V, P]

theorem well_scoped_input_has_no_outcome :
    P.WellScopedInput () () () () /\
      (¬ Exists fun result : V.Result => P.Elaborates () () () () result) /\
      (¬ Exists fun diagnostic : V.Diagnostic => P.Rejects () () () () diagnostic) /\
      OBL021StatementDraft V P :=
  ⟨well_scoped_input_exists, no_successful_result_exists, no_diagnostic_exists,
    statement_draft_holds⟩

end MirCore.Lab.OBL021.OutcomeTotalityCountermodel
