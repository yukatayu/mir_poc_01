import MixedCancelEntry
namespace MirroreaProofFirst.MixedCancelEmbedding
open MixedRequestEmbedding MixedCancelEntry MixedRequestCore

theorem check_exact (m : CompositionMachine.Machine p a) (member : Fin a) (place : Fin p)
 (principal id : Nat) (t : InvocationBoundary.Ticket) (permit : MixedCancellation.Permit) :
 cancelCheck (machine m) member place principal id t permit =
 CompositionMachine.cancelCheck m member place principal id t permit := by
 simp only [cancelCheck,CompositionMachine.cancelCheck,MixedRequestEmbedding.fresh_exact]
 simp only [machine,MixedManagementEmbedding.system,List.length_map]
 have mem : (m.pending.map Pending.pure).contains (.pure t) = m.pending.contains t := by simp
 rw [mem]
 rfl

theorem abandon_exact (m : CompositionMachine.Machine p a) (t : InvocationBoundary.Ticket)
 (principal id : Nat) (permit : MixedCancellation.Permit) :
 abandon (machine m) t principal id permit =
 machine (CompositionMachine.abandon m t principal id permit) := by
 simp only [abandon,CompositionMachine.abandon,machine,MixedManagementEmbedding.system,
   List.filter_map,List.map_cons,event,pendingId,Function.comp_def]
 rfl

theorem cancel_exact (m : CompositionMachine.Machine p a) (member : Fin a) (place : Fin p)
 (principal id : Nat) (t : InvocationBoundary.Ticket) (permit : MixedCancellation.Permit) :
 cancel (machine m) member place principal id t permit =
 (CompositionMachine.cancel m member place principal id t permit).map machine := by
 simp only [cancel,CompositionMachine.cancel,check_exact,abandon_exact]
 split <;> rfl

#print axioms check_exact
#print axioms abandon_exact
#print axioms cancel_exact
end MirroreaProofFirst.MixedCancelEmbedding
