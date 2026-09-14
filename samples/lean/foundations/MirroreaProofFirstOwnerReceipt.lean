import MirroreaProofFirstOwnerOccurrence
import MirroreaProofFirstReceivedTicket

namespace MirroreaProofFirst.OwnerReceipt
open OwnerCodecTree

-- A private response carries the exact request tuple and namespace. This is
-- correlation data, NOT a self-authenticating receipt or disclosure permission.
-- The owner's retained image and caller-private continuation are not sent.
structure Envelope where
  scopeId : Nat
  ordinal : Nat
  revision : Nat
  ticket : InvocationBoundary.Ticket
  result : OwnerEvaluator.Result
  deriving DecidableEq, Repr

def project (record : OwnerOccurrence.Record p a) : Envelope :=
  ⟨record.scopeId,record.ordinal,record.revision,record.ticket,record.result⟩

def codec : Codec Envelope := iso
  (product natural (product natural (product natural (product OwnerFullCodec.ticket OwnerPacketCodec.result))))
  (fun r => (r.scopeId,r.ordinal,r.revision,r.ticket,r.result))
  (fun (scopeId,ordinal,revision,ticket,result) => ⟨scopeId,ordinal,revision,ticket,result⟩)
  (by intro r; cases r; rfl) (by intro r; rfl)

def accept (expectedScope : Nat) (state : ReferenceSource.State p a) (envelope : Envelope) :
    Option (ReferenceSource.State p a) :=
  if envelope.scopeId ≠ expectedScope then none else
    match envelope.result with
    | .rejected _ => none
    | .value value => ReceivedTicket.accept state envelope.ticket value

theorem accept_exact : accept scopeId state envelope = some next ↔
    envelope.scopeId = scopeId ∧ ∃ value, envelope.result = .value value ∧
      ReceivedTicket.accept state envelope.ticket value = some next := by
  by_cases scope : envelope.scopeId = scopeId
  · cases result : envelope.result <;> simp [accept,scope,result]
  · simp [accept,scope]

theorem original_pending (accepted : accept scopeId state envelope = some next) :
    envelope.scopeId = scopeId ∧ ∃ saved value,
      state.waiting = some saved ∧ saved.entry.ticket = envelope.ticket ∧
      envelope.result = .value value ∧
      saved.entry ∈ state.machine.pending ∧
      ReferenceExecution.Protected state.machine.store saved.entry := by
  obtain ⟨scope,value,result,received⟩ := accept_exact.mp accepted
  obtain ⟨saved,waiting,ticket,pending,guarded,_⟩ := ReceivedTicket.original_binding received
  exact ⟨scope,saved,value,waiting,ticket,result,pending,guarded⟩

theorem no_double_accept (accepted : accept scopeId state envelope = some next)
    (otherScope : Nat) (other : Envelope) : accept otherScope next other = none := by
  obtain ⟨_,value,_,received⟩ := accept_exact.mp accepted
  unfold accept
  split
  · rfl
  · cases other.result with
    | rejected _ => rfl
    | value result => exact ReceivedTicket.no_double_accept received other.ticket result

theorem distinct_ticket_refused (waiting : state.waiting = some saved)
    (different : saved.entry.ticket ≠ envelope.ticket) : accept scopeId state envelope = none := by
  unfold accept
  split
  · rfl
  · cases envelope.result with
    | rejected _ => rfl
    | value value => exact ReceivedTicket.wrong_ticket_refused waiting different

theorem issued_response_complete
    (produced : OwnerOccurrence.submit owner scopeId ticket = (nextOwner,.produced record))
    (ready : ReceivedTicket.accept state record.ticket value = some next)
    (result : record.result = .value value) : accept scopeId state (project record) = some next := by
  obtain ⟨scope,_,_,_,recordEq,_⟩ := OwnerOccurrence.produced_parts produced
  apply accept_exact.mpr
  refine ⟨?_,value,result,ready⟩
  simp only [project,recordEq,scope]

theorem bytes_preserve (envelope : Envelope) :
    OwnerPacketCodec.decode codec (OwnerPacketCodec.encode codec envelope) = some envelope :=
  OwnerPacketCodec.roundtrip codec envelope

-- Construct receipt consumption from independent current-state premises. The
-- earlier issued_response_complete merely transported an already accepted
-- receive. Here the owner computation plus the exact current image supplies
-- value checking; retained private pending/protection supplies consumption.
-- Physical origin of this production and image equality are still separate.
theorem current_production_completes
    (valid : ReferenceExecution.Invariant state.machine)
    (waiting : state.waiting = some saved)
    (pending : saved.entry ∈ state.machine.pending)
    (guarded : ReferenceExecution.Protected state.machine.store saved.entry)
    (current : owner.image = OwnerImage.capture (OwnerProjection.project state))
    (produced : OwnerOccurrence.submit owner scopeId saved.entry.ticket = (nextOwner,.produced record)) :
    ∃ next, accept scopeId state (project record) = some next := by
  obtain ⟨value,result,admitted,executed⟩ := OwnerOccurrence.produced_meaning produced
  have parts := OwnerOccurrence.produced_parts produced
  have ticketAt : record.ticket = saved.entry.ticket := by rw [parts.2.2.2.2.1]
  have imageAt : record.image = owner.image := by rw [parts.2.2.2.2.1]
  have checked := (OwnerProjection.check_exact _ _ _).mpr admitted.2.2.2
  rw [imageAt,current,OwnerImage.check_roundtrip,ticketAt] at checked
  have invocation : InvocationBoundary.check state.machine.store.core.system.configuration.state
      state.machine.store.core.system.view saved.entry.ticket = true := by
    simp only [OwnerProjection.check,Bool.and_eq_true] at checked
    exact checked.2
  have computation : InvocationBoundary.execute saved.entry.ticket = some value := by
    rw [ticketAt] at executed
    exact (InstancePrograms.Machine.run_exact _ _ _).mpr executed
  have corePending : saved.entry.ticket ∈ state.machine.store.core.pending := by
    rw [← valid.2]
    exact List.mem_map.mpr ⟨saved.entry,pending,rfl⟩
  have unused := valid.1.1.1.2.2 saved.entry.ticket corePending
  have finishChecked : CompositionMachine.finishCheck state.machine.store.core saved.entry.ticket value = true := by
    simp [CompositionMachine.finishCheck,corePending,unused,InvocationBoundary.resultCheck,invocation,computation]
  have finished : ∃ machine, ReferenceExecution.finish state.machine saved.entry value = some machine := by
    simp [ReferenceExecution.finish,pending,(ReferenceExecution.protection_exact _ _).mpr guarded,
      ReferenceStore.finishPlain,CompositionMachine.finish,finishChecked]
  obtain ⟨machine,finished⟩ := finished
  have received : ∃ next, ReceivedResult.accept state saved.entry value = some next := by
    simp [ReceivedResult.accept,waiting,finished]
  obtain ⟨next,received⟩ := received
  refine ⟨next,issued_response_complete produced ?_ result⟩
  rw [ticketAt]
  exact ReceivedTicket.received_complete received

#print axioms accept_exact
#print axioms original_pending
#print axioms no_double_accept
#print axioms distinct_ticket_refused
#print axioms issued_response_complete
#print axioms bytes_preserve
#print axioms current_production_completes
end MirroreaProofFirst.OwnerReceipt
