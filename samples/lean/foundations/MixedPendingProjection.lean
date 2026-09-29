import MixedRequestCore
namespace MirroreaProofFirst.MixedPendingProjection
open MixedRequestCore
-- Pure protection classifications are not a second owner queue. Their ticket
-- projection must match ALL actual pure entries in the single common core.
-- An empty pure projection alone never means that the common core is drained.
def pureTicket : Pending → Option InvocationBoundary.Ticket
 | .pure t => some t
 | .owner _ => none
def tickets (pending : List Pending) : List InvocationBoundary.Ticket := pending.filterMap pureTicket

@[simp] theorem pure_cons (t : InvocationBoundary.Ticket) (rest : List Pending) :
 tickets (.pure t :: rest) = t :: tickets rest := rfl
@[simp] theorem owner_cons (s : OwnerSavedPending.Saved) (rest : List Pending) :
 tickets (.owner s :: rest) = tickets rest := rfl

 theorem ids_sublist (pending : List Pending) :
 List.Sublist ((tickets pending).map CompositionMachine.ticketId) (pending.map pendingId) := by
 induction pending with
 | nil => exact .slnil
 | cons head rest ih =>
   cases head with
   | pure t => exact .cons₂ _ ih
   | owner s => exact .cons _ ih

theorem ids_nodup (pending : List Pending) (unique : (pending.map pendingId).Nodup) :
 ((tickets pending).map CompositionMachine.ticketId).Nodup := (ids_sublist pending).nodup unique

theorem filter_ids (pending : List Pending) (id : CurrentUse.UseId) :
 tickets (pending.filter (fun other => decide (pendingId other ≠ id))) =
 (tickets pending).filter (fun t => decide (CompositionMachine.ticketId t ≠ id)) := by
 induction pending with
 | nil => rfl
 | cons head rest ih =>
   cases head with
   | pure t =>
     by_cases same : CompositionMachine.ticketId t = id <;>
       simp [tickets,pureTicket,pendingId,List.filter_cons,same] at ih ⊢ <;> exact ih
   | owner s =>
     by_cases same : pendingId (.owner s) = id <;>
       simp [tickets,pureTicket,List.filter_cons,same] at ih ⊢ <;> exact ih

theorem pure_image (ts : List InvocationBoundary.Ticket) : tickets (ts.map Pending.pure) = ts := by
 induction ts with
 | nil => rfl
 | cons t rest ih => simpa using congrArg (List.cons t) ih

theorem owner_wait_not_drained (s : OwnerSavedPending.Saved) :
 tickets [.owner s] = [] ∧ ([Pending.owner s] : List Pending) ≠ [] := ⟨rfl,by simp⟩
theorem member_core (member : ticket ∈ tickets pending) : Pending.pure ticket ∈ pending := by
 obtain ⟨entry,present,same⟩ := List.mem_filterMap.mp member
 cases entry with
 | owner saved => cases same
 | pure other =>
   have equal : other = ticket := Option.some.inj same
   simpa [equal] using present

theorem owner_consumption_keeps_tickets (valid : MixedRequestCore.Invariant m)
 (present : Pending.owner saved ∈ m.pending) :
 tickets (consumeOwner m saved evidence).pending = tickets m.pending := by
 rw [consumeOwner,filter_ids]
 apply List.filter_eq_self.mpr
 intro ticket member
 have distinct : CompositionMachine.ticketId ticket ≠ pendingId (.owner saved) := by
   intro same
   have equal := same_id_same_pending m.pending valid.2.1 (member_core member) present same
   cases equal
 simpa using distinct
#print axioms member_core
#print axioms owner_consumption_keeps_tickets
#print axioms ids_nodup
#print axioms filter_ids
#print axioms pure_image
#print axioms owner_wait_not_drained
end MirroreaProofFirst.MixedPendingProjection
