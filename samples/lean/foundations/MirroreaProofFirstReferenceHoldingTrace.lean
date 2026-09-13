import MirroreaProofFirstReferenceChronology

namespace MirroreaProofFirst.ReferenceHoldingTrace
open ReferenceOwner

theorem restore_request (request : ReferenceAccess.Request) :
    {ReferenceCoherence.stableRequest request with epoch := request.epoch,lineage := request.lineage} = request := by
  cases request
  rfl

theorem same_epoch_request (before after : Binding) (progress : ReferenceCoherence.Progressed before after)
    (epoch : before.request.epoch = after.request.epoch) : before.request = after.request := by
  have lineage := progress.same_epoch_lineage epoch
  calc
    before.request = {ReferenceCoherence.stableRequest before.request with
      epoch := before.request.epoch,lineage := before.request.lineage} := (restore_request _).symm
    _ = {ReferenceCoherence.stableRequest after.request with
      epoch := after.request.epoch,lineage := after.request.lineage} := by rw [progress.stable,epoch,lineage]
    _ = after.request := restore_request _

-- Every admitted engine mutator preserves the complete original hold stamp at
-- the same epoch, including its history origin. No fresh-origin side entrance
-- exists through degradation, normalization, aliasing, or authority input.
theorem reached_same_hold (first next : ReferenceExecution.Machine p a) (key : Nat)
    (before after : Binding) (valid : ReferenceExecution.Invariant first)
    (path : ReferenceExecution.Reached first next)
    (old : ReferenceMutation.lookup first.store key = some before)
    (new : ReferenceMutation.lookup next.store key = some after)
    (epoch : before.request.epoch = after.request.epoch) :
    before.request = after.request ∧ before.holding = after.holding := by
  have progress := ReferenceTrace.reached_progress first.store next.store valid.1
    (ReferenceExecution.reached_projects _ _ path) key before after old new
  exact ⟨same_epoch_request _ _ progress epoch,progress.same_hold epoch⟩

-- This is a theorem about actual executions and stored history, not an
-- arbitrary caller-supplied suffix. Lost owner evidence in this epoch remains
-- lost after every further admitted transition, including restoration inputs.
theorem no_hold_resurrection (first next : ReferenceExecution.Machine p a) (key : Nat)
    (before after : Binding) (valid : ReferenceExecution.Invariant first)
    (path : ReferenceExecution.Reached first next)
    (old : ReferenceMutation.lookup first.store key = some before)
    (new : ReferenceMutation.lookup next.store key = some after)
    (epoch : before.request.epoch = after.request.epoch)
    (lost : ReferenceHolding.continuousCheck .separate before.request before.holding.guard
      (first.store.history.drop before.holding.activation) = false) :
    ReferenceMutation.holdingLive next.store after = false := by
  obtain ⟨requestAt,holdingAt⟩ := reached_same_hold _ _ _ _ _ valid path old new epoch
  obtain ⟨frames,_,historyAt,_⟩ := ReferenceChronology.reached_history _ _ (ReferenceExecution.reached_projects _ _ path)
  simp only [ReferenceMutation.holdingLive,historyAt,← requestAt,← holdingAt,
    ReferenceHolding.loss_persists _ _ _ _ _ frames lost,Bool.false_and]

theorem lost_epoch_result_rejected (first next : ReferenceExecution.Machine p a) (key : Nat)
    (before after : Binding) (valid : ReferenceExecution.Invariant first)
    (path : ReferenceExecution.Reached first next)
    (old : ReferenceMutation.lookup first.store key = some before)
    (new : ReferenceMutation.lookup next.store key = some after)
    (epoch : before.request.epoch = after.request.epoch)
    (lost : ReferenceHolding.continuousCheck .separate before.request before.holding.guard
      (first.store.history.drop before.holding.activation) = false)
    (ticket : InvocationBoundary.Ticket) : ReferenceResult.check next.store after ticket = false :=
  ReferenceResult.lost_holding_rejected _ _ _ (no_hold_resurrection _ _ _ _ _ valid path old new epoch lost)

theorem dead_hold_normalization (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) (binding : Binding)
    (found : ReferenceMutation.lookup m key = some binding)
    (actor : ReferenceOwner.actorCheck m.core.system (.release binding) member place principal = true)
    (lost : ReferenceMutation.holdingLive m binding = false) :
    ReferenceMutation.normalize m member place principal id key = ⟨m,.error .ownerNotLive,false⟩ := by
  simp [ReferenceMutation.normalize,found,actor,lost]

#print axioms reached_same_hold
#print axioms no_hold_resurrection
#print axioms lost_epoch_result_rejected
#print axioms dead_hold_normalization
end MirroreaProofFirst.ReferenceHoldingTrace
