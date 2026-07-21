import samples.lean.«lab-statements».obl021.ElabDeterminismStatementDraft

/-!
LAB-only countermodel for the OBL-021 statement-shape draft.

Every listed result projection is total and unique, and every component
equivalence is native equality.  The two `DistinctResult` constructors remain
different because the draft has no joint extensionality law or direct Result
relation connecting all projections to Result identity.
-/

namespace MirCore.Lab.OBL021.ProjectionExtensionalityCountermodel

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
  CoreTermOf := fun _ _ => True
  TypeOf := fun _ _ => True
  ModeOf := fun _ _ => True
  EffectRowOf := fun _ _ => True
  FailureRowOf := fun _ _ => True
  ConstraintsOf := fun _ _ => True
  ObligationsOf := fun _ _ => True
  GeneratedEdgesOf := fun _ _ => True
  SourceSpansOf := fun _ _ => True
  EquivalentCoreTerm := fun left right => left = right
  EquivalentTypeOut := fun left right => left = right
  EquivalentModeOut := fun left right => left = right
  EquivalentEffectRow := fun left right => left = right
  EquivalentFailureRow := fun left right => left = right
  EquivalentConstraintSet := fun left right => left = right
  EquivalentObligationSet := fun left right => left = right
  EquivalentGeneratedEdges := fun left right => left = right
  EquivalentSourceSpanMap := fun left right => left = right
  EquivalentDiagnostic := fun left right => left = right

theorem projection_predicates_are_total_and_unique :
    (forall result : V.Result, Exists fun core =>
      P.CoreTermOf result core /\
        forall other, P.CoreTermOf result other -> other = core) /\
      (forall result : V.Result, Exists fun output =>
        P.TypeOf result output /\
          forall other, P.TypeOf result other -> other = output) /\
      (forall result : V.Result, Exists fun output =>
        P.ModeOf result output /\
          forall other, P.ModeOf result other -> other = output) /\
      (forall result : V.Result, Exists fun output =>
        P.EffectRowOf result output /\
          forall other, P.EffectRowOf result other -> other = output) /\
      (forall result : V.Result, Exists fun output =>
        P.FailureRowOf result output /\
          forall other, P.FailureRowOf result other -> other = output) /\
      (forall result : V.Result, Exists fun constraints =>
        P.ConstraintsOf result constraints /\
          forall other, P.ConstraintsOf result other -> other = constraints) /\
      (forall result : V.Result, Exists fun obligations =>
        P.ObligationsOf result obligations /\
          forall other, P.ObligationsOf result other -> other = obligations) /\
      (forall result : V.Result, Exists fun edges =>
        P.GeneratedEdgesOf result edges /\
          forall other, P.GeneratedEdgesOf result other -> other = edges) /\
      (forall result : V.Result, Exists fun spans =>
        P.SourceSpansOf result spans /\
          forall other, P.SourceSpansOf result other -> other = spans) := by
  simp [V, P]

theorem component_equivalences_are_equality :
    (forall left right : V.CoreTerm, P.EquivalentCoreTerm left right ↔ left = right) /\
      (forall left right : V.TypeOut, P.EquivalentTypeOut left right ↔ left = right) /\
      (forall left right : V.ModeOut, P.EquivalentModeOut left right ↔ left = right) /\
      (forall left right : V.EffectRow, P.EquivalentEffectRow left right ↔ left = right) /\
      (forall left right : V.FailureRow, P.EquivalentFailureRow left right ↔ left = right) /\
      (forall left right : V.ConstraintSet, P.EquivalentConstraintSet left right ↔ left = right) /\
      (forall left right : V.ObligationSet, P.EquivalentObligationSet left right ↔ left = right) /\
      (forall left right : V.GeneratedEdgeSet, P.EquivalentGeneratedEdges left right ↔ left = right) /\
      (forall left right : V.SourceSpanMap, P.EquivalentSourceSpanMap left right ↔ left = right) := by
  simp [V, P]

theorem statement_draft_holds : OBL021StatementDraft V P := by
  simp [OBL021StatementDraft, ElabDeterministicPost, SameElabResult,
    SameDiagnostic, V, P]

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

theorem total_unique_projections_still_allow_distinct_results :
    (forall result : V.Result, Exists fun core =>
      P.CoreTermOf result core /\
        forall other, P.CoreTermOf result other -> other = core) /\
      OBL021StatementDraft V P /\
      Exists fun first => Exists fun second =>
        first ≠ second /\
          P.Elaborates () () () () first /\
          P.Elaborates () () () () second := by
  exact ⟨projection_predicates_are_total_and_unique.1,
    statement_draft_holds, distinct_results_can_elaborate⟩

end MirCore.Lab.OBL021.ProjectionExtensionalityCountermodel
