import CohortStateCorrespondence
import OwnerLeasePhaseCorrespondence
open MirroreaProofFirst SharedHostCaptureReplay
namespace LeaseCohortCorrespondence
open OwnerLeasePhaseRelation
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- Fresh launch phase is retained from actual fromLaunch, not asserted about
-- arbitrary recovery. Existing active-entry and active-writer full journals
-- supply the focused memory; this additionally closes ALL other lease fields.
structure Correspondence (live : CohortHostExecution.Live base) : Prop where
  fields : CohortStateCorrespondence.Correspondence live
  leases : OwnerLeasePhaseRelation.Aligned live.current.state.inner

theorem live_correspondence (live : CohortHostExecution.Live base)
    (fresh : base.mode = .prelude owed) : Correspondence live :=
  ⟨CohortStateCorrespondence.live_correspondence live (by simp [fresh,CohortHostReceipt.paymentOf]),
    OwnerLeasePhaseCorrespondence.live_aligned live fresh⟩

theorem idle_current (live : CohortHostExecution.Live base) (fresh : base.mode = .prelude owed)
    (noWriter : live.current.state.inner.joined.active = none)
    (noEntry : live.current.state.inner.active = none) (owner : Fin p) :
    leasePair (live.current.state.inner.writers owner) =
      expected live.current.state.inner.joined.history.current.mode owner := by
  simpa [AtOwner,noWriter,noEntry] using (live_correspondence live fresh).leases owner

-- Parameter-general discriminators: Data equality alone cannot detect a
-- fabricated retained lease/entered flag. No mutation is added to Step.
theorem fabricated_idle_lease_rejected (state : SourceEntryPrefix.State base)
    (noWriter : state.joined.active = none) (noEntry : state.active = none)
    (ordinary : state.joined.history.current.mode = .ordinary)
    (owner : Fin p) (ticket : InvocationBoundary.Ticket) :
    ¬ Aligned {state with writers:=SourceEntryPrefix.put state.writers owner ({state.writers owner with lease:=some ticket})} := by
  intro invalid
  have selected := invalid owner
  simp [AtOwner,noWriter,noEntry,SourceEntryPrefix.put,leasePair,expected,ordinary,released] at selected

theorem fabricated_idle_entered_rejected (state : SourceEntryPrefix.State base)
    (noWriter : state.joined.active = none) (noEntry : state.active = none)
    (ordinary : state.joined.history.current.mode = .ordinary) (owner : Fin p) :
    ¬ Aligned {state with writers:=SourceEntryPrefix.put state.writers owner ({state.writers owner with entered:=true})} := by
  intro invalid
  have selected := invalid owner
  simp [AtOwner,noWriter,noEntry,SourceEntryPrefix.put,leasePair,expected,ordinary,released] at selected

theorem erased_entered_lease_rejected (state : SourceEntryPrefix.State base)
    (noWriter : state.joined.active = none) (noEntry : state.active = none)
    (entered : state.joined.history.current.mode = .entered dispatch) :
    ¬ Aligned {state with writers:=SourceEntryPrefix.put state.writers dispatch.endpoint ({state.writers dispatch.endpoint with lease:=none})} := by
  intro invalid
  have selected := invalid dispatch.endpoint
  simp [AtOwner,noWriter,noEntry,SourceEntryPrefix.put,leasePair,expected,entered,released] at selected

#print axioms live_correspondence
#print axioms idle_current
#print axioms fabricated_idle_lease_rejected
#print axioms fabricated_idle_entered_rejected
#print axioms erased_entered_lease_rejected
end LeaseCohortCorrespondence
