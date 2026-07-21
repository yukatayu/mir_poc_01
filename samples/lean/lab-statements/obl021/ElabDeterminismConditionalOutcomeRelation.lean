import samples.lean.«lab-statements».obl021.ElabDeterminismStatementDraft

/-!
LAB-only conditional outcome relation for the OBL-021 statement-shape draft.

The tagged Outcome carrier, OutcomeOf predicate, SameOutcome relation, and
OutcomeTotal premise are experiment-local.  The theorem does not select native
equality, relation laws, quotient semantics, or a Canon home for totality.
-/

namespace MirCore.Lab.OBL021.ConditionalOutcomeRelation

open MirCore.Lab.OBL021.StatementDraft

universe u

inductive Outcome (V : Vocab.{u}) where
  | success (result : V.Result)
  | reject (diagnostic : V.Diagnostic)

def OutcomeOf
    {V : Vocab.{u}}
    (P : Pred V)
    (env : V.Env)
    (ctx : V.Ctx)
    (locus : V.Locus)
    (item : V.SurfaceItem)
    (outcome : Outcome V) : Prop :=
  match outcome with
  | Outcome.success result => P.Elaborates env ctx locus item result
  | Outcome.reject diagnostic => P.Rejects env ctx locus item diagnostic

def SameOutcome
    {V : Vocab.{u}}
    (P : Pred V)
    (left right : Outcome V) : Prop :=
  match left, right with
  | Outcome.success leftResult, Outcome.success rightResult =>
      SameElabResult P leftResult rightResult
  | Outcome.reject leftDiagnostic, Outcome.reject rightDiagnostic =>
      SameDiagnostic P leftDiagnostic rightDiagnostic
  | _, _ => False

def OutcomeTotal
    {V : Vocab.{u}}
    (P : Pred V)
    (env : V.Env)
    (ctx : V.Ctx)
    (locus : V.Locus)
    (item : V.SurfaceItem) : Prop :=
  Exists fun outcome => OutcomeOf P env ctx locus item outcome

theorem statement_draft_implies_outcomes_related
    {V : Vocab.{u}}
    (P : Pred V)
    (draft : OBL021StatementDraft V P)
    (env : V.Env)
    (ctx : V.Ctx)
    (locus : V.Locus)
    (item : V.SurfaceItem)
    (wellScoped : P.WellScopedInput env ctx locus item) :
    forall left right,
      OutcomeOf P env ctx locus item left ->
      OutcomeOf P env ctx locus item right ->
      SameOutcome P left right := by
  intro left right leftOutcome rightOutcome
  cases left with
  | success leftResult =>
      cases right with
      | success rightResult =>
          exact (draft env ctx locus item wellScoped).1
            leftResult rightResult leftOutcome rightOutcome
      | reject rightDiagnostic =>
          exact (draft env ctx locus item wellScoped).2.2
            leftResult rightDiagnostic leftOutcome rightOutcome
  | reject leftDiagnostic =>
      cases right with
      | success rightResult =>
          exact False.elim ((draft env ctx locus item wellScoped).2.2
            rightResult leftDiagnostic rightOutcome leftOutcome)
      | reject rightDiagnostic =>
          exact (draft env ctx locus item wellScoped).2.1
            leftDiagnostic rightDiagnostic leftOutcome rightOutcome

theorem outcome_totality_and_draft_imply_unique_relation
    {V : Vocab.{u}}
    (P : Pred V)
    (draft : OBL021StatementDraft V P)
    (env : V.Env)
    (ctx : V.Ctx)
    (locus : V.Locus)
    (item : V.SurfaceItem)
    (wellScoped : P.WellScopedInput env ctx locus item)
    (total : OutcomeTotal P env ctx locus item) :
    Exists fun witness =>
      OutcomeOf P env ctx locus item witness /\
        forall left right,
          OutcomeOf P env ctx locus item left ->
          OutcomeOf P env ctx locus item right ->
          SameOutcome P left right := by
  rcases total with ⟨witness, witnessOutcome⟩
  exact ⟨witness, witnessOutcome,
    statement_draft_implies_outcomes_related P draft env ctx locus item wellScoped⟩

end MirCore.Lab.OBL021.ConditionalOutcomeRelation
