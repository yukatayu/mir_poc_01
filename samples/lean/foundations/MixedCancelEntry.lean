import MixedCancellation
namespace MirroreaProofFirst.MixedCancelEntry
open CurrentUse WorldProjection MixedRequestCore
variable {id : Nat}
abbrev Ticket := InvocationBoundary.Ticket
abbrev ticketId := CompositionMachine.ticketId
-- Existing pure cancellation only. Owner pending is distinct and survives.
-- No owner rollback, cancellation-on-wire or release of owner resources.
def cancelCheck (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (t : Ticket) (permit : MixedCancellation.Permit) : Bool :=
  m.pending.contains (.pure t) && !m.system.used.contains (ticketId t) &&
    fresh m (MixedManagementEntry.useId m.system principal id) &&
    MixedCancellation.check m.system m.events.length t member place principal id permit

def abandon (m : Machine p a) (t : Ticket) (principal id : Nat)
    (permit : MixedCancellation.Permit) : Machine p a :=
  {system := {m.system with
     serial := m.system.serial+1,
     used := ticketId t :: MixedManagementEntry.useId m.system principal id :: m.system.used},
   pending := m.pending.filter (fun other => decide (pendingId other ≠ ticketId t)),
   events := .cancelled permit :: m.events}

def cancel (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (t : Ticket) (permit : MixedCancellation.Permit) : Option (Machine p a) :=
  if cancelCheck m member place principal id t permit then some (abandon m t principal id permit) else none

theorem cancel_parts (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (t : Ticket) (permit : MixedCancellation.Permit) (next : Machine p a)
    (accepted : cancel m member place principal id t permit = some next) :
    Pending.pure t ∈ m.pending ∧ ticketId t ∉ m.system.used ∧
      fresh m (MixedManagementEntry.useId m.system principal id) = true ∧
      MixedCancellation.Submitted m.system m.events.length t member place principal id permit ∧
      next = abandon m t principal id permit := by
  unfold cancel at accepted
  split at accepted
  · rename_i checked
    have parts : Pending.pure t ∈ m.pending ∧ ticketId t ∉ m.system.used ∧
        fresh m (MixedManagementEntry.useId m.system principal id) = true ∧
        MixedCancellation.Submitted m.system m.events.length t member place principal id permit := by
      simpa [cancelCheck,MixedCancellation.check_exact,and_assoc] using checked
    exact ⟨parts.1,parts.2.1,parts.2.2.1,parts.2.2.2,(Option.some.inj accepted).symm⟩
  · cases accepted

theorem cancel_preserves (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (t : Ticket) (permit : MixedCancellation.Permit) (next : Machine p a)
    (valid : Invariant m) (accepted : cancel m member place principal id t permit = some next) : Invariant next := by
  obtain ⟨pending,unused,fresh,_,rfl⟩ := cancel_parts _ _ _ _ _ _ _ _ accepted
  have free := fresh_exact.mp fresh
  have distinct : ticketId t ≠ MixedManagementEntry.useId m.system principal id := by
    intro same
    exact free.2 (same ▸ List.mem_map.mpr ⟨Pending.pure t,pending,rfl⟩)
  refine ⟨⟨valid.1.1,?_⟩,?_,?_⟩
  · exact List.nodup_cons.mpr ⟨by simpa using And.intro distinct unused,List.nodup_cons.mpr ⟨free.1,valid.1.2⟩⟩
  · exact List.Nodup.sublist ((List.filter_sublist).map pendingId) valid.2.1
  · intro other member
    have mem : other ∈ m.pending ∧ pendingId other ≠ ticketId t := by simpa [abandon] using member
    have notCancel : pendingId other ≠ MixedManagementEntry.useId m.system principal id := by
      intro same
      exact free.2 (same ▸ List.mem_map.mpr ⟨other,mem.1,rfl⟩)
    simp only [abandon,List.mem_cons,not_or]
    exact ⟨mem.2,notCancel,valid.2.2 other mem.1⟩

theorem cancel_complete (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (t : Ticket) (permit : MixedCancellation.Permit)
    (pending : Pending.pure t ∈ m.pending) (unused : ticketId t ∉ m.system.used)
    (free : fresh m (MixedManagementEntry.useId m.system principal id) = true)
    (allowed : MixedCancellation.Submitted m.system m.events.length t member place principal id permit) :
    cancel m member place principal id t permit = some (abandon m t principal id permit) := by
  simp [cancel,cancelCheck,pending,unused,free,(MixedCancellation.check_exact _ _ _ _ _ _ _ _).mpr allowed]

theorem cancelled_cannot_finish (m : Machine p a) (t other : Ticket) (principal id : Nat)
    (permit : MixedCancellation.Permit) (value : Int) (same : ticketId other = ticketId t) :
    finishPure (abandon m t principal id permit) other value = none := by
  simp [finishPure,finishCheck,abandon,same]

theorem finished_cannot_cancel (m : Machine p a) (t other : Ticket) (value : Int)
    (member : Fin a) (place : Fin p) (principal id : Nat) (permit : MixedCancellation.Permit)
    (same : ticketId other = ticketId t) :
    cancel (consumePure m t value) member place principal id other permit = none := by
  simp [cancel,cancelCheck,consumePure,same]


theorem cancel_keeps_owner (valid : Invariant m)
 (accepted : cancel m member place principal id t permit = some next)
 (waiting : Pending.owner saved ∈ m.pending) : Pending.owner saved ∈ next.pending := by
 obtain ⟨pureMem,_,_,_,rfl⟩ := cancel_parts _ _ _ _ _ _ _ _ accepted
 have distinct : pendingId (.owner saved) ≠ ticketId t := by
   intro same
   have eq := same_id_same_pending m.pending valid.2.1 waiting pureMem same
   cases eq
 simpa [abandon] using And.intro waiting distinct

#print axioms cancel_preserves
#print axioms cancel_complete
#print axioms cancelled_cannot_finish
#print axioms finished_cannot_cancel
#print axioms cancel_keeps_owner
end MirroreaProofFirst.MixedCancelEntry
