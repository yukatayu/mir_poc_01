/-!
LAB-only relation-polarity experiment.

Every carrier and relation below is a parameter. The file intentionally has no
imports or local data declarations, and no downstream source imports it.
-/

namespace MirCore.Lab.Theory.SameCarrierVarianceBoundary

universe u v w

theorem wf_preservation_transfers
    {State : Type u}
    {Action : Type v}
    (invariant : State -> Prop)
    (modelRelation intendedRelation : State -> Action -> State -> Prop)
    (modelPreserves :
      forall before action after,
        invariant before ->
        modelRelation before action after ->
        invariant after)
    (intendedIncluded :
      forall before action after,
        intendedRelation before action after ->
        modelRelation before action after) :
    forall before action after,
      invariant before ->
      intendedRelation before action after ->
      invariant after := by
  intro before action after beforeInvariant intendedTransition
  exact modelPreserves before action after beforeInvariant
    (intendedIncluded before action after intendedTransition)

theorem coherence_transfers
    {Input : Type u}
    {Success : Type v}
    {Failure : Type w}
    (intendedSuccess modelSuccess : Input -> Success -> Prop)
    (intendedFailure modelFailure : Input -> Failure -> Prop)
    (sameSuccess : Success -> Success -> Prop)
    (sameFailure : Failure -> Failure -> Prop)
    (modelSuccessCoherent :
      forall input left right,
        modelSuccess input left ->
        modelSuccess input right ->
        sameSuccess left right)
    (modelFailureCoherent :
      forall input left right,
        modelFailure input left ->
        modelFailure input right ->
        sameFailure left right)
    (modelExcludes :
      forall input success failure,
        modelSuccess input success ->
        modelFailure input failure ->
        False)
    (successIncluded :
      forall input success,
        intendedSuccess input success ->
        modelSuccess input success)
    (failureIncluded :
      forall input failure,
        intendedFailure input failure ->
        modelFailure input failure) :
    (forall input left right,
      intendedSuccess input left ->
      intendedSuccess input right ->
      sameSuccess left right) /\
    (forall input left right,
      intendedFailure input left ->
      intendedFailure input right ->
      sameFailure left right) /\
    (forall input success failure,
      intendedSuccess input success ->
      intendedFailure input failure ->
      False) := by
  constructor
  · intro input left right leftSuccess rightSuccess
    exact modelSuccessCoherent input left right
      (successIncluded input left leftSuccess)
      (successIncluded input right rightSuccess)
  constructor
  · intro input left right leftFailure rightFailure
    exact modelFailureCoherent input left right
      (failureIncluded input left leftFailure)
      (failureIncluded input right rightFailure)
  · intro input success failure successEvidence failureEvidence
    exact modelExcludes input success failure
      (successIncluded input success successEvidence)
      (failureIncluded input failure failureEvidence)

theorem outcome_existence_transfers
    {Input : Type u}
    {Success : Type v}
    {Failure : Type w}
    (inScope : Input -> Prop)
    (modelSuccess intendedSuccess : Input -> Success -> Prop)
    (modelFailure intendedFailure : Input -> Failure -> Prop)
    (modelOutcomes :
      forall input,
        inScope input ->
        (Exists fun success => modelSuccess input success) \/
        (Exists fun failure => modelFailure input failure))
    (successRealizable :
      forall input success,
        modelSuccess input success ->
        intendedSuccess input success)
    (failureRealizable :
      forall input failure,
        modelFailure input failure ->
        intendedFailure input failure) :
    forall input,
      inScope input ->
      (Exists fun success => intendedSuccess input success) \/
      (Exists fun failure => intendedFailure input failure) := by
  intro input scopeProof
  cases modelOutcomes input scopeProof with
  | inl successOutcome =>
      rcases successOutcome with ⟨success, modeled⟩
      exact Or.inl ⟨success, successRealizable input success modeled⟩
  | inr failureOutcome =>
      rcases failureOutcome with ⟨failure, modeled⟩
      exact Or.inr ⟨failure, failureRealizable input failure modeled⟩

end MirCore.Lab.Theory.SameCarrierVarianceBoundary
