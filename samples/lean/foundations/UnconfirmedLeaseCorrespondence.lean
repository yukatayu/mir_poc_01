import OwnerLeaseCompleteCorrespondence
import RetiredLeaseCorrespondence
import CohortFaultProgress
open MirroreaProofFirst
namespace UnconfirmedLeaseCorrespondence
open CohortCommitJournal
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- Unknown IO preserves the last settled host/native cursor and separately owns
-- the real physical residual. The physical endpoint may already have advanced;
-- no equation identifies that endpoint with this last-known native state.
structure Correspondence (state : CohortFaultProgress.State base) : Prop where
  lastKnown : LeaseCohortCorrespondence.Correspondence state.current
  everyOwner : ∀ owner, OwnerLeaseCompleteCorrespondence.AtOwner state.current.current.state.inner owner
  memoryKept : state.current.current.state.cohort.memory = state.origin.anchor.current.state.cohort.memory
  pendingKept : state.current.current.state.cohort.pending = state.origin.anchor.current.state.cohort.pending
  historyKept : state.current.source.state.joined.history = state.origin.anchor.source.state.joined.history
  physical : SharedWireLifetime.Runs (SharedWireLifetime.initial base)
    (state.current.source.state.joined.history.actions++state.origin.stopped.pending.residual.actions)
    (SharedReplyRetention.wire state.origin.armed.wire.attempt state.slot)
  noPromotion : HostReplyPrefix.absorb state.origin.stopped.pending = none

theorem correspondence (state : CohortFaultProgress.State base) (fresh : base.mode = .prelude owed) :
    Correspondence state := by
  have fields := LeaseCohortCorrespondence.live_correspondence state.current fresh
  have kept := state.keeps
  exact ⟨fields,OwnerLeaseCompleteCorrespondence.complete fields,kept.1,kept.2.1,kept.2.2,
    state.rooted,state.no_absorb⟩

-- The old arm rule independently requires no unfinished cohort-store debt.
-- Cleanup retains that fact; it is not permission to absorb the unknown reply.
theorem pending_empty (state : CohortFaultProgress.State base) :
    state.current.current.state.cohort.pending = [] :=
  state.keeps.2.1.trans (CohortHostFailure.armed_gate state.origin.armed).2.2.2

theorem observed_last_known (state : CohortFaultProgress.State base)
    (fresh : base.mode = .prelude owed)
    (accepted : CohortHostObservation.memoryEq state.current.current.state.cohort.memory observed = true) :
    observed.bootstrapped = true ∧
    observed.snapshot = CohortSnapshotCorrespondence.nativeSnapshot state.current.current.state.inner ∧
    CohortAdministrativeCorrespondence.Corresponds observed state.current.current.state.inner.joined.closed ∧
    (∀ envelope ∈ observed.produced, CohortProductionCorrespondence.ProducedAt state.current.current.state.inner envelope) ∧
    observed.pendingPayment = CohortPaymentCorrespondence.expected state.current.current.state.inner :=
  CohortStateCorrespondence.observed_discharged state.current
    (by simp [fresh,CohortHostReceipt.paymentOf]) (pending_empty state) accepted

#print axioms correspondence
#print axioms pending_empty
#print axioms observed_last_known
end UnconfirmedLeaseCorrespondence
