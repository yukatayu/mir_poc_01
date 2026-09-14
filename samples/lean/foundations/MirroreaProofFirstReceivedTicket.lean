import MirroreaProofFirstReceivedResult

namespace MirroreaProofFirst.ReceivedTicket

-- Receiver-facing mathematical payload omits the caller's private reference
-- binding and source continuation. Transport authentication/owner ancestry is
-- still a separate gate. A matching scalar/ticket alone does not prove delivery.
def accept (state : ReferenceSource.State p a) (ticket : InvocationBoundary.Ticket) (value : Int) :
    Option (ReferenceSource.State p a) := do
  let saved ← state.waiting
  if saved.entry.ticket != ticket then none else
    ReceivedResult.accept state saved.entry value

theorem accept_exact (state next : ReferenceSource.State p a) (ticket : InvocationBoundary.Ticket) (value : Int) :
    accept state ticket value = some next ↔
      ∃ saved, state.waiting = some saved ∧ saved.entry.ticket = ticket ∧
        ReceivedResult.accept state saved.entry value = some next := by
  cases waiting : state.waiting with
  | none => simp [accept,waiting]
  | some saved =>
      by_cases same : saved.entry.ticket = ticket
      · simp [accept,waiting,same]
      · simp [accept,waiting,same]

theorem original_binding (accepted : accept state ticket value = some next) :
    ∃ saved, state.waiting = some saved ∧ saved.entry.ticket = ticket ∧
      saved.entry ∈ state.machine.pending ∧ ReferenceExecution.Protected state.machine.store saved.entry ∧
      InvocationBoundary.ResultMeaning state.machine.store.core.system.configuration.state
        state.machine.store.core.system.view ticket value := by
  obtain ⟨saved,waiting,same,received⟩ := (accept_exact _ _ _ _).mp accepted
  have guarded :=  ReceivedResult.accepted_value received
  exact ⟨saved,waiting,same,guarded.1,guarded.2.1,same ▸ guarded.2.2⟩

theorem no_double_accept (accepted : accept state ticket value = some next)
    (other : InvocationBoundary.Ticket) (result : Int) : accept next other result = none := by
  obtain ⟨saved,_,_,received⟩ := (accept_exact _ _ _ _).mp accepted
  obtain ⟨_,_,_,_,_,rfl⟩ := ReceivedResult.accept_parts received
  rfl

-- Same validity domain as the richer mathematical input when the caller's
-- exact saved entry is supplied; erasing it from the message is not erasing its
-- protected use. This excludes safety obtained by refusing every reference.
theorem received_complete (received : ReceivedResult.accept state entry value = some next) :
    accept state entry.ticket value = some next := by
  obtain ⟨saved,_,waiting,equal,_,_⟩ := ReceivedResult.accept_parts received
  exact (accept_exact _ _ _ _).mpr ⟨saved,waiting,congrArg ReferenceExecution.Pending.ticket equal,equal ▸ received⟩

theorem wrong_ticket_refused (waiting : state.waiting = some saved)
    (different : saved.entry.ticket ≠ ticket) : accept state ticket value = none := by
  simp [accept,waiting,different]

theorem invalid_private_binding_refused (waiting : state.waiting = some saved)
    (binding : ReferenceOwner.Binding) (bound : saved.entry.binding = some binding)
    (invalid : ReferenceResult.check state.machine.store binding saved.entry.ticket = false)
    (ticket : InvocationBoundary.Ticket) (value : Int) : accept state ticket value = none := by
  have refused := ReferenceExecution.finish_reference_invalid state.machine saved.entry binding value bound invalid
  by_cases same : saved.entry.ticket = ticket
  · simp [accept,waiting,same,ReceivedResult.accept,refused]
  · simp [accept,waiting,same]

#print axioms accept_exact
#print axioms original_binding
#print axioms no_double_accept
#print axioms received_complete
#print axioms invalid_private_binding_refused
end MirroreaProofFirst.ReceivedTicket
