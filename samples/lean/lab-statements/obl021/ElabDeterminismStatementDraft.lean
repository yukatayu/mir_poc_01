/-!
LAB-only OBL-021 elaboration determinism statement-shape draft.

This file checks that the determinism postcondition can be expressed as Lean
propositions without importing final MirCore datatypes, final equality
relations, runtime scheduling semantics, conformance evidence, or proof
obligation status changes.
-/

namespace MirCore.Lab.OBL021.StatementDraft

universe u

structure Vocab where
  Env : Type u
  Ctx : Type u
  Locus : Type u
  SurfaceItem : Type u
  Result : Type u
  Diagnostic : Type u
  CoreTerm : Type u
  TypeOut : Type u
  ModeOut : Type u
  EffectRow : Type u
  FailureRow : Type u
  ConstraintSet : Type u
  ObligationSet : Type u
  GeneratedEdgeSet : Type u
  SourceSpanMap : Type u

structure Pred (V : Vocab.{u}) where
  WellScopedInput : V.Env -> V.Ctx -> V.Locus -> V.SurfaceItem -> Prop
  Elaborates :
    V.Env -> V.Ctx -> V.Locus -> V.SurfaceItem -> V.Result -> Prop
  Rejects :
    V.Env -> V.Ctx -> V.Locus -> V.SurfaceItem -> V.Diagnostic -> Prop
  CoreTermOf : V.Result -> V.CoreTerm -> Prop
  TypeOf : V.Result -> V.TypeOut -> Prop
  ModeOf : V.Result -> V.ModeOut -> Prop
  EffectRowOf : V.Result -> V.EffectRow -> Prop
  FailureRowOf : V.Result -> V.FailureRow -> Prop
  ConstraintsOf : V.Result -> V.ConstraintSet -> Prop
  ObligationsOf : V.Result -> V.ObligationSet -> Prop
  GeneratedEdgesOf : V.Result -> V.GeneratedEdgeSet -> Prop
  SourceSpansOf : V.Result -> V.SourceSpanMap -> Prop
  EquivalentCoreTerm : V.CoreTerm -> V.CoreTerm -> Prop
  EquivalentTypeOut : V.TypeOut -> V.TypeOut -> Prop
  EquivalentModeOut : V.ModeOut -> V.ModeOut -> Prop
  EquivalentEffectRow : V.EffectRow -> V.EffectRow -> Prop
  EquivalentFailureRow : V.FailureRow -> V.FailureRow -> Prop
  EquivalentConstraintSet : V.ConstraintSet -> V.ConstraintSet -> Prop
  EquivalentObligationSet : V.ObligationSet -> V.ObligationSet -> Prop
  EquivalentGeneratedEdges :
    V.GeneratedEdgeSet -> V.GeneratedEdgeSet -> Prop
  EquivalentSourceSpanMap : V.SourceSpanMap -> V.SourceSpanMap -> Prop
  EquivalentDiagnostic : V.Diagnostic -> V.Diagnostic -> Prop

def SameElabResult
    {V : Vocab.{u}}
    (P : Pred V)
    (left right : V.Result) : Prop :=
  (forall c₁ c₂,
    P.CoreTermOf left c₁ ->
    P.CoreTermOf right c₂ ->
      P.EquivalentCoreTerm c₁ c₂) /\
  (forall a₁ a₂,
    P.TypeOf left a₁ ->
    P.TypeOf right a₂ ->
      P.EquivalentTypeOut a₁ a₂) /\
  (forall m₁ m₂,
    P.ModeOf left m₁ ->
    P.ModeOf right m₂ ->
      P.EquivalentModeOut m₁ m₂) /\
  (forall e₁ e₂,
    P.EffectRowOf left e₁ ->
    P.EffectRowOf right e₂ ->
      P.EquivalentEffectRow e₁ e₂) /\
  (forall f₁ f₂,
    P.FailureRowOf left f₁ ->
    P.FailureRowOf right f₂ ->
      P.EquivalentFailureRow f₁ f₂) /\
  (forall c₁ c₂,
    P.ConstraintsOf left c₁ ->
    P.ConstraintsOf right c₂ ->
      P.EquivalentConstraintSet c₁ c₂) /\
  (forall o₁ o₂,
    P.ObligationsOf left o₁ ->
    P.ObligationsOf right o₂ ->
      P.EquivalentObligationSet o₁ o₂) /\
  (forall g₁ g₂,
    P.GeneratedEdgesOf left g₁ ->
    P.GeneratedEdgesOf right g₂ ->
      P.EquivalentGeneratedEdges g₁ g₂) /\
  (forall s₁ s₂,
    P.SourceSpansOf left s₁ ->
    P.SourceSpansOf right s₂ ->
      P.EquivalentSourceSpanMap s₁ s₂)

def SameDiagnostic
    {V : Vocab.{u}}
    (P : Pred V)
    (left right : V.Diagnostic) : Prop :=
  P.EquivalentDiagnostic left right

def ElabDeterministicPost
    {V : Vocab.{u}}
    (P : Pred V)
    (env : V.Env)
    (ctx : V.Ctx)
    (locus : V.Locus)
    (item : V.SurfaceItem) : Prop :=
  (forall left right,
    P.Elaborates env ctx locus item left ->
    P.Elaborates env ctx locus item right ->
      SameElabResult P left right) /\
  (forall left right,
    P.Rejects env ctx locus item left ->
    P.Rejects env ctx locus item right ->
      SameDiagnostic P left right) /\
  (forall result diagnostic,
    P.Elaborates env ctx locus item result ->
    P.Rejects env ctx locus item diagnostic ->
      False)

def OBL021StatementDraft
    (V : Vocab.{u})
    (P : Pred V) : Prop :=
  forall
    (env : V.Env)
    (ctx : V.Ctx)
    (locus : V.Locus)
    (item : V.SurfaceItem),
      P.WellScopedInput env ctx locus item ->
        ElabDeterministicPost P env ctx locus item

end MirCore.Lab.OBL021.StatementDraft
