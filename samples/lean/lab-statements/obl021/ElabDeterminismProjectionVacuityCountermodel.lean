import samples.lean.«lab-statements».obl021.ElabDeterminismStatementDraft

/-!
LAB-only countermodel for the OBL-021 statement-shape draft.

The model deliberately gives one well-scoped input two distinct successful
results while every result-projection predicate is empty.  It checks only that
the existing projection-based postcondition can hold vacuously; it does not
choose the final result equality, diagnostic equivalence, or OBL status.
-/

namespace MirCore.Lab.OBL021.ProjectionVacuityCountermodel

open MirCore.Lab.OBL021.StatementDraft

inductive DistinctResult where
  | left
  | right
deriving DecidableEq

def V : Vocab where
  Env := Unit
  Ctx := Unit
  Locus := Unit
  SurfaceItem := Unit
  Result := DistinctResult
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
  Elaborates := fun _ _ _ _ _ => True
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

theorem projection_predicates_are_empty (result : V.Result) :
    (¬ Exists fun core => P.CoreTermOf result core) /\
      (¬ Exists fun output => P.TypeOf result output) /\
      (¬ Exists fun output => P.ModeOf result output) /\
      (¬ Exists fun output => P.EffectRowOf result output) /\
      (¬ Exists fun output => P.FailureRowOf result output) /\
      (¬ Exists fun constraints => P.ConstraintsOf result constraints) /\
      (¬ Exists fun obligations => P.ObligationsOf result obligations) /\
      (¬ Exists fun edges => P.GeneratedEdgesOf result edges) /\
      (¬ Exists fun spans => P.SourceSpansOf result spans) := by
  simp [P]

theorem statement_draft_holds : OBL021StatementDraft V P := by
  simp [OBL021StatementDraft, ElabDeterministicPost, SameElabResult,
    SameDiagnostic, P]

theorem distinct_results_can_elaborate :
    Exists fun first => Exists fun second =>
      first ≠ second /\
        P.Elaborates () () () () first /\
        P.Elaborates () () () () second := by
  refine ⟨DistinctResult.left, DistinctResult.right, ?_, ?_, ?_⟩
  intro equality
  cases equality
  simp [P]
  simp [P]

theorem statement_draft_allows_distinct_successes :
    OBL021StatementDraft V P /\
      Exists fun first => Exists fun second =>
        first ≠ second /\
          P.Elaborates () () () () first /\
          P.Elaborates () () () () second :=
  ⟨statement_draft_holds, distinct_results_can_elaborate⟩

end MirCore.Lab.OBL021.ProjectionVacuityCountermodel
