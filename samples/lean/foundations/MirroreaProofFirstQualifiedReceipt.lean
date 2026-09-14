import MirroreaProofFirstQualifiedCustody
import MirroreaProofFirstOwnerReceipt

namespace MirroreaProofFirst.QualifiedReceipt

-- Retained dispatch correlation, supplied by the admitted endpoint boundary.
-- Neither these naturals nor an Envelope authenticate a producer. Revision is
-- the dispatch's publication revision, not the latest receipt-time head.
structure Context where
  scopeId : Nat
  revision : Nat
  deriving DecidableEq, Repr

def accept (context : Context) (state : QualifiedCustody.State p a)
    (envelope : OwnerReceipt.Envelope) : Option (QualifiedCustody.State p a) :=
  if envelope.scopeId ≠ context.scopeId ∨ envelope.revision ≠ context.revision then none else
    match envelope.result with
    | .rejected _ => none
    | .value value => QualifiedCustody.evaluate state (.receive envelope.ticket value)

-- Declarative entry is the full private activation entry. In particular this
-- does not install the lower receiver's bare ReferenceSource.State.
def Entry (context : Context) (state : QualifiedCustody.State p a)
    (envelope : OwnerReceipt.Envelope) (next : QualifiedCustody.State p a) : Prop :=
  envelope.scopeId = context.scopeId ∧ envelope.revision = context.revision ∧
    ∃ value, envelope.result = .value value ∧
      QualifiedCustody.Entry state (.receive envelope.ticket value) next

theorem accept_exact : accept context state envelope = some next ↔ Entry context state envelope next := by
  by_cases correlated : envelope.scopeId = context.scopeId ∧ envelope.revision = context.revision
  · obtain ⟨scope,revision⟩ := correlated
    cases result : envelope.result <;> simp [accept,Entry,scope,revision,result,QualifiedCustody.evaluate_exact]
  · have different : envelope.scopeId ≠ context.scopeId ∨ envelope.revision ≠ context.revision := by
      by_cases scope : envelope.scopeId = context.scopeId
      · exact Or.inr (fun revision => correlated ⟨scope,revision⟩)
      · exact Or.inl scope
    have refused : ¬ Entry context state envelope next := fun admitted => correlated ⟨admitted.1,admitted.2.1⟩
    simp [accept,different,refused]

theorem accepted_evaluation (accepted : accept context state envelope = some next) :
    ∃ value, envelope.result = .value value ∧
      QualifiedCustody.evaluate state (.receive envelope.ticket value) = some next := by
  obtain ⟨_,_,value,result,entry⟩ := accept_exact.mp accepted
  exact ⟨value,result,QualifiedCustody.evaluate_exact.mpr entry⟩

theorem rooted_preserved (root : QualifiedCustody.Rooted realm view policy identity state)
    (accepted : accept context state envelope = some next) :
    QualifiedCustody.Rooted realm view policy identity next := by
  obtain ⟨_,_,computed⟩ := accepted_evaluation accepted
  exact .step root computed

theorem metadata_preserved (accepted : accept context state envelope = some next) :
    next.stamp = state.stamp ∧
    ∃ value, envelope.result = .value value ∧
      QualifiedSession.receive state.privateState.session envelope.ticket value = some next.privateState.session ∧
      next.privateState.replies =
        ⟨state.privateState.replies.length,.receive envelope.ticket value,.received value⟩ :: state.privateState.replies := by
  obtain ⟨_,_,value,result,entry⟩ := accept_exact.mp accepted
  obtain ⟨outcome,received,history⟩ := entry.2.2
  cases received with
  | receive ran => exact ⟨entry.2.1,value,result,ran,history⟩

theorem no_second_accept (accepted : accept context state envelope = some next)
    (otherContext : Context) (other : OwnerReceipt.Envelope) : accept otherContext next other = none := by
  obtain ⟨_,value,_,received,_⟩ := metadata_preserved accepted
  obtain ⟨_,source,_,nextAt⟩ := QualifiedSession.receive_parts received
  unfold accept
  split
  · rfl
  · cases other.result with
    | rejected _ => rfl
    | value value =>
      simp [QualifiedCustody.evaluate,QualifiedCustody.initiates,QualifiedPublication.evaluate,
        QualifiedPublication.evaluateResult,QualifiedSession.receive,nextAt]

-- This positive theorem is not stated with successful receipt as a premise.
-- Existing source invariants/private protection and actual owner production
-- supply the checked value; the full session receive then preserves metadata.
-- Physical origin/current image equality are still explicit obligations.
theorem current_production_completes {p a : Nat} {state : QualifiedCustody.State p a}
    {owner nextOwner : OwnerOccurrence.State p a} {record : OwnerOccurrence.Record p a}
    (waitingStatus : state.privateState.session.status = .waiting)
    (valid : ReferenceExecution.Invariant state.privateState.session.state.source.machine)
    (waiting : state.privateState.session.state.source.waiting = some saved)
    (pending : saved.entry ∈ state.privateState.session.state.source.machine.pending)
    (guarded : ReferenceExecution.Protected state.privateState.session.state.source.machine.store saved.entry)
    (current : owner.image = OwnerImage.capture (OwnerProjection.project state.privateState.session.state.source))
    (produced : OwnerOccurrence.submit owner context.scopeId saved.entry.ticket = (nextOwner,.produced record))
    (revision : record.revision = context.revision) :
    ∃ next, accept context state (OwnerReceipt.project record) = some next := by
  obtain ⟨source,received⟩ := OwnerReceipt.current_production_completes valid waiting pending guarded current produced
  obtain ⟨scope,value,result,lower⟩ := OwnerReceipt.accept_exact.mp received
  let session := state.privateState.session
  let qualified : QualifiedSource.State p a := ⟨source,session.state.tags⟩
  let nextSession : QualifiedSession.Session p a :=
    {session with state := qualified,status := .ready,completed := session.completed ++ session.stopped.toList,stopped := none}
  have upper : QualifiedSession.receive session record.ticket value = some nextSession := by
    dsimp only [OwnerReceipt.project] at lower
    simp [QualifiedSession.receive,waitingStatus,QualifiedSource.received,session,qualified,nextSession,lower]
  let snapshot : QualifiedPublication.Snapshot p a :=
    ⟨nextSession,⟨state.privateState.replies.length,.receive record.ticket value,.received value⟩ :: state.privateState.replies⟩
  refine ⟨⟨state.stamp,snapshot⟩,accept_exact.mpr ⟨scope,revision,value,result,?_,rfl,.received value,?_,rfl⟩⟩
  · simp [QualifiedCustody.initiates]
  · exact QualifiedPublication.Entry.receive upper

theorem encoded_accept (envelope : OwnerReceipt.Envelope) :
    (OwnerPacketCodec.decode OwnerReceipt.codec (OwnerPacketCodec.encode OwnerReceipt.codec envelope) >>=
      accept context state) = accept context state envelope := by
  rw [OwnerPacketCodec.roundtrip]; rfl

#print axioms accept_exact
#print axioms accepted_evaluation
#print axioms rooted_preserved
#print axioms metadata_preserved
#print axioms no_second_accept
#print axioms current_production_completes
#print axioms encoded_accept
end MirroreaProofFirst.QualifiedReceipt
