import OwnerStatementResultLifecycle
namespace MirroreaProofFirst.OwnerStatementAcknowledgment
open OwnerStatementLiveCustodian

-- Independent receipt rule on the actual retained pending inventory, current
-- world and original evidence. It does not assume a successful source tick.
def LowerReady (m : MixedReferenceExecution.Machine p a)
 (saved received : OwnerSavedPending.Saved) (evidence : CurrentUse.Evidence) : Prop :=
 .owner saved ∈ m.store.core.pending ∧
 MixedRequestCore.pendingId (.owner saved) ∉ m.store.core.system.used ∧
 ∃ (pending delivered : OwnerEffectService.Pending (WorldProjection.size a p m.store.core.system.configuration.count)),
 OwnerSavedPending.save pending = saved ∧ OwnerSavedPending.save delivered = received ∧
 OwnerEffectReceipt.Accepted
   (MixedCatalogService.world m.store.core.system.configuration.state m.store.core.system.view)
   pending ⟨delivered,evidence⟩

def lowerCheck (m : MixedReferenceExecution.Machine p a)
 (saved received : OwnerSavedPending.Saved) (evidence : CurrentUse.Evidence) : Bool :=
 m.store.core.pending.contains (.owner saved) &&
 !m.store.core.system.used.contains (MixedRequestCore.pendingId (.owner saved)) &&
 match OwnerSavedPending.materialize (WorldProjection.size a p m.store.core.system.configuration.count) saved,
       OwnerSavedPending.materialize (WorldProjection.size a p m.store.core.system.configuration.count) received with
 | some pending,some delivered => OwnerEffectReceipt.ackCheck
    (MixedCatalogService.world m.store.core.system.configuration.state m.store.core.system.view)
    pending ⟨delivered,evidence⟩
 | _,_ => false

theorem lowerCheck_exact : lowerCheck m saved received evidence = true ↔ LowerReady m saved received evidence := by
 unfold lowerCheck LowerReady
 cases hp : OwnerSavedPending.materialize (WorldProjection.size _ _ _) saved with
 | none => simp [←OwnerSavedPending.materialize_exact,hp]
 | some pending =>
   cases hd : OwnerSavedPending.materialize (WorldProjection.size _ _ _) received with
   | none => simp [←OwnerSavedPending.materialize_exact,hp,hd]
   | some delivered => simp [←OwnerSavedPending.materialize_exact,hp,hd,OwnerEffectReceipt.ack_exact,and_assoc]

theorem finished_ready (run : MixedReferenceExecution.finishOwner m saved received evidence = some next) :
 LowerReady m saved received evidence := by
 obtain ⟨core,run,_⟩ := MixedReferenceStore.finishOwner_parts _ (MixedReferenceExecution.finishOwner_parts _ run).1
 obtain ⟨present,fresh,pending,delivered,saved,received,accepted,_⟩ := MixedRequestCore.finishOwner_parts run
 exact ⟨present,fresh,pending,delivered,saved,received,accepted⟩

theorem ready_finish (ready : LowerReady m saved received evidence) :
 ∃ next, MixedReferenceExecution.finishOwner m saved received evidence = some next := by
 obtain ⟨present,fresh,pending,delivered,hs,hr,accepted⟩ := ready
 simp [MixedReferenceExecution.finishOwner,MixedReferenceStore.finishOwner,MixedRequestCore.finishOwner,
   present,fresh,OwnerSavedPending.materialize_exact.mpr hs,OwnerSavedPending.materialize_exact.mpr hr,
   OwnerEffectReceipt.ack_exact.mpr accepted]

def Ready (s : State p a) (ticket : Ticket) : Prop :=
 ticket.designation = s.designation ∧ s.held = some ⟨ticket,.reported⟩ ∧
 s.live.session.status = .waiting ∧
 (∃ name ordinal control source, s.live.session.cursor.stopped = some (.write name ordinal control source)) ∧
 ∃ waiting received rest,
 MixedOwnerSourceIssue.ownerWaiting s.live.session.state.source.waiting = some waiting ∧ waiting.saved = ticket.saved ∧
 s.live.session.state.inbox = received::rest ∧
 LowerReady s.live.session.state.source.machine waiting.saved received.pending received.evidence

def check (s : State p a) (ticket : Ticket) : Bool :=
 decide (ticket.designation = s.designation ∧ s.held = some ⟨ticket,.reported⟩ ∧ s.live.session.status = .waiting) &&
 (match s.live.session.cursor.stopped with | some (.write _ _ _ _) => true | _ => false) &&
 match MixedOwnerSourceIssue.ownerWaiting s.live.session.state.source.waiting,s.live.session.state.inbox with
 | some waiting,received::_ => decide (waiting.saved = ticket.saved) &&
     lowerCheck s.live.session.state.source.machine waiting.saved received.pending received.evidence
 | _,_ => false

theorem check_exact : check s ticket = true ↔ Ready s ticket := by
 unfold check Ready
 cases stopped : s.live.session.cursor.stopped with
 | none => simp
 | some entry =>
   cases entry with
   | ordinary item => simp
   | install name control source => simp
   | write name ordinal control source =>
     cases waiting : MixedOwnerSourceIssue.ownerWaiting s.live.session.state.source.waiting with
     | none => simp
     | some value =>
       cases box : s.live.session.state.inbox with
       | nil => simp
       | cons received rest => simp [lowerCheck_exact,and_assoc]

theorem ready_receive (ready : Ready s ticket) :
 ∃ next, MixedOwnerSourceTrace.receive s.live.session.state = some next ∧
 MixedOwnerSourceIssue.ownerWaiting next.source.waiting = none := by
 obtain ⟨_,_,_,_,waiting,received,rest,held,_,box,lower⟩ := ready
 obtain ⟨machine,finished⟩ := ready_finish lower
 unfold MixedOwnerSourceTrace.receive
 simp only [box,Option.bind_eq_bind]
 simp only [MixedOwnerSourceIssue.acknowledge,held,Option.bind_eq_bind,Option.bind_some,finished,Option.pure_def]
 exact ⟨_,rfl,rfl⟩

theorem ready_consume (ready : Ready s ticket) :
 ∃ next, OwnerStatementSourceCustodian.consume s ticket member principal = some next := by
 obtain ⟨received,run,cleared⟩ := ready_receive ready
 obtain ⟨designation,held,phase,⟨name,ordinal,control,source,stopped⟩,waiting,reply,rest,waitingHeld,same,box,_⟩ := ready
 have complete : MixedOwnerContinuation.complete s.live.session = ⟨received,.ready⟩ := by
   simp [MixedOwnerContinuation.complete,waitingHeld,box,run]
 have noWaiting : MixedOwnerSourceIssue.ownerWaiting
   (MixedOwnerContinuation.tick s.live.session member principal).state.source.waiting = none := by
   simpa [MixedOwnerContinuation.tick,MixedSourceCursor.tick,phase,complete] using cleared
 have bank : (OwnerStatementRegistrySelection.tick s.live member principal).session =
   MixedOwnerContinuation.tick s.live.session member principal := by
   simp [OwnerStatementRegistrySelection.tick,noWaiting]
 refine ⟨{s with live := OwnerStatementRegistrySelection.tick s.live member principal,held := none},OwnerStatementSourceCustodian.consume_exact.mpr ?_⟩
 refine ⟨designation,held,waiting,waitingHeld,same,?_,rfl⟩
 rw [bank]
 simp [OwnerStatementSourceCustodian.count,MixedOwnerContinuation.tick,MixedSourceCursor.tick,
   phase,complete,stopped,OwnerStatementPosition.writes,OwnerStatementPosition.entryOccurrence,List.filterMap_append]

theorem consumed_ready (facts : OwnerStatementRegistryInvariant.Facts s.live.session)
 (run : OwnerStatementSourceCustodian.consume s ticket member principal = some next) : Ready s ticket := by
 have consumed := OwnerStatementSourceCustodian.consume_exact.mp run
 have accepted := OwnerStatementSourceCustodian.consume_sound facts run
 obtain ⟨name,ordinal,control,source,held,received,rest,after,written,phase,stopped,waiting,_,_,box,_,ack,_⟩ := accepted
 obtain ⟨other,stored,finished,_⟩ := MixedOwnerSourceIssue.acknowledge_parts ack
 have equal : other = held := by simpa [MixedOwnerSourceIssue.ownerWaiting,stored] using waiting
 subst other
 obtain ⟨waiting2,held2,same,_⟩ := consumed.2.2
 have sameWaiting : waiting2 = held := Option.some.inj (held2.symm.trans waiting)
 subst waiting2
 exact ⟨consumed.1,consumed.2.1,phase,⟨name,ordinal,control,source,stopped⟩,
   held,received,rest,waiting,same,box,finished_ready finished⟩

theorem check_consume_exact (facts : OwnerStatementRegistryInvariant.Facts s.live.session) :
 check s ticket = true ↔ ∃ next, OwnerStatementSourceCustodian.consume s ticket member principal = some next := by
 rw [check_exact]
 exact ⟨ready_consume,fun ⟨_,run⟩ => consumed_ready facts run⟩


-- Historical successful service does not imply that the original evidence is
-- current at source acknowledgment. No successor evidence is minted here.
theorem current_generation_rejected
 (changed : pending.original.context.generation ≠ world.generation) :
 OwnerEffectReceipt.ackCheck world pending received = false := by
 have different : pending.original.context ≠ CurrentUse.currentContext world pending.request := by
  intro same
  exact changed (congrArg CurrentUse.Context.generation same)
 have rejected := CurrentUse.context_change_rejected world.authority
  (world.policies pending.request.operation.key) (CurrentUse.currentContext world pending.request) pending.original different
 simp [OwnerEffectReceipt.ackCheck,CurrentUse.checkUse,rejected]

#print axioms current_generation_rejected

#print axioms lowerCheck_exact
#print axioms finished_ready
#print axioms ready_finish
#print axioms check_exact
#print axioms ready_receive
#print axioms ready_consume
#print axioms consumed_ready
#print axioms check_consume_exact
end MirroreaProofFirst.OwnerStatementAcknowledgment
