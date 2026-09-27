import CohortProductionCorrespondence
open MirroreaProofFirst SharedHostCaptureReplay
open CohortCommitJournal CohortHostReceipt
namespace ProducedRootedness
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- The witness type already carries its path from the SAME base. No added
-- premise, operational guard, backward uniqueness, or stronger predicate.
theorem origin_rooted {state : SourceEntryPrefix.State base}
    (origin : CohortProductionCorrespondence.ProducedAt state envelope) :
    ∃ history : SharedWireLifetime.History base,
      SharedWireLifetime.Runs (SharedWireLifetime.initial base) history.actions
        (SharedWireLifetime.initial history.current) ∧
      Extends history state.joined.history ∧ FinishedProduction history.current.joint envelope := by
  obtain ⟨history,extension,production⟩ := origin
  exact ⟨history,history.path,extension,production⟩

theorem live_rooted (live : CohortHostExecution.Live base)
    (present : envelope ∈ (target live.current.state.cohort).produced) :
    ∃ history : SharedWireLifetime.History base,
      SharedWireLifetime.Runs (SharedWireLifetime.initial base) history.actions
        (SharedWireLifetime.initial history.current) ∧
      Extends history live.current.state.inner.joined.history ∧
      FinishedProduction history.current.joint envelope :=
  origin_rooted (CohortProductionCorrespondence.live_origin live envelope present)

#print axioms origin_rooted
#print axioms live_rooted
end ProducedRootedness
