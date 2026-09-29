import OwnerStatementRetainedHistory
namespace MirroreaProofFirst.OwnerStatementEntryGate
open OwnerMetadataSession

-- Examine the actual queued request, not a caller-selected packet.saved.
-- This is the existing metadata guard exposed for discriminating controls;
-- it grants neither authority nor executable custody to serialized fields.
def serviceCheck (s : State p a) : Bool :=
 match s.packet,s.session.state.owner.queued with
 | some packet,some saved =>
   match CompositionCore.index p packet.owner with
   | none => false
   | some place => OwnerMetadataPending.check s.addresses (management s) place saved packet
 | _,_ => false

theorem refused_exact (stale : serviceCheck s = false) : service s ops = s := by
 unfold serviceCheck at stale
 unfold service
 split
 · split
   · rfl
   · split
     · simp_all
     · rfl
 · rfl

-- Incoming candidate data is compared with the one live queue. It does not
-- carry a clone/import of an executable backend. The physical exclusivity and
-- linear publication of this live state are still implementation obligations.
def execute (live : State p a) (candidate : OwnerSavedPending.Saved)
 (ops : FallibleFlow.Arithmetic) : State p a :=
 if live.session.state.owner.queued = some candidate then service live ops else live

theorem execute_route : execute live candidate ops = live ∨
 (live.session.state.owner.queued = some candidate ∧ execute live candidate ops = service live ops) := by
 unfold execute
 split
 · exact Or.inr ⟨‹_›,rfl⟩
 · exact Or.inl rfl

theorem execute_refines (path : Rooted initial live) : Rooted initial (execute live candidate ops) := by
 rcases execute_route (live:=live) (candidate:=candidate) (ops:=ops) with same | ⟨_,same⟩
 · rw [same]; exact path
 · rw [same]; exact .service path

theorem execute_mismatch (different : live.session.state.owner.queued ≠ some candidate) :
 execute live candidate ops = live := by simp [execute,different]

theorem execute_empty (empty : live.session.state.owner.queued = none) :
 execute live candidate ops = live := by simp [execute,empty]

theorem execute_stale (stale : serviceCheck live = false) : execute live candidate ops = live := by
 unfold execute
 split
 · exact refused_exact stale
 · rfl

-- A changed actual lower attempt drains its one queue even on a known lower
-- refusal. A metadata boundary refusal leaves the whole state unchanged.
theorem trace_changed_empty
 (changed : MixedOwnerSourceTrace.service ops current trace ≠ trace) :
 (MixedOwnerSourceTrace.service ops current trace).owner.queued = none := by
 unfold MixedOwnerSourceTrace.service at changed ⊢
 by_cases ready : MixedOwnerSourceTrace.serviceReady trace = true
 · simp only [if_pos ready] at changed ⊢
   unfold MixedOwnerSourceTrace.serviceAdmitted at changed ⊢
   dsimp only at changed ⊢
   cases run : (MixedOwnerAttemptQueue.advance ops
     trace.source.machine.store.core.system.configuration.state
     trace.source.machine.store.core.system.view current trace.owner).2 with
   | none =>
     have same := MixedOwnerAttemptQueue.not_attempted_retains run
     simp [run,same] at changed
   | some answer =>
     obtain ⟨_,_,_,_,_,_,_,empty,_⟩ := MixedOwnerAttemptQueue.advance_parts run
     exact empty
 · simp [ready] at changed

theorem service_changed_empty {p a : Nat} {live : State p a} (changed : service live ops ≠ live) :
 (service live ops).session.state.owner.queued = none := by
 cases packetFound : live.packet with
 | none => simp [service,packetFound] at changed
 | some packet =>
   cases queueFound : live.session.state.owner.queued with
   | none => simp [service,packetFound,queueFound] at changed
   | some saved =>
     cases placeFound : CompositionCore.index p packet.owner with
     | none => simp [service,packetFound,queueFound,placeFound] at changed
     | some place =>
       by_cases current : OwnerMetadataPending.check live.addresses (management live) place saved packet = true
       · simp only [service,packetFound,queueFound,placeFound,if_pos current] at changed ⊢
         apply trace_changed_empty
         intro unchanged
         apply changed
         simp only [MixedOwnerContinuation.service,unchanged]
         rw [←packetFound]
       · simp [service,packetFound,queueFound,placeFound,current] at changed

-- Retrying old candidate data against the UPDATED live state cannot perform
-- another attempt. Calling service on an independent copy of the OLD state is
-- deliberately not covered: that would violate the stated live-state boundary.
theorem no_repeated_execution (changed : execute live candidate ops ≠ live) :
 execute (execute live candidate ops) other nextOps = execute live candidate ops := by
 rcases execute_route (live:=live) (candidate:=candidate) (ops:=ops) with same | ⟨_,same⟩
 · exact False.elim (changed same)
 · apply execute_empty
   rw [same]
   exact service_changed_empty (by rwa [same] at changed)

-- Completeness is relative to independent declarative source custody,
-- metadata/current field validity and lower Commits, not to a successful new
-- gateway call. It excludes an all-refusing gateway on this admitted cut.
theorem execute_committed_complete {p a : Nat} {live : State p a}
 {place : Fin p} {candidate : OwnerSavedPending.Saved} {packet : OwnerMetadataPending.Packet}
 {pending : OwnerEffectService.Pending
   (WorldProjection.size a p live.session.state.source.machine.store.core.system.configuration.count)}
 {written : OwnerEffectService.Write
   (WorldProjection.size a p live.session.state.source.machine.store.core.system.configuration.count)}
 (queued : live.session.state.owner.queued = some candidate)
 (packetHeld : live.packet = some packet)
 (current : OwnerMetadataPending.Current live.addresses (management live) place candidate packet)
 (waiting : live.session.state.source.waiting = some (.inr held))
 (same : held.saved = candidate)
 (retained : .owner candidate ∈ live.session.state.source.machine.store.core.pending)
 (materialized : OwnerSavedPending.materialize
   (WorldProjection.size a p live.session.state.source.machine.store.core.system.configuration.count)
   candidate = some pending)
 (admitted : MixedOwnerMaterialization.Admitted
   live.session.state.source.machine.store.core.system.configuration.state
   (OwnerMetadataPending.currentFields live.addresses live.registry place.val)
   ⟨live.session.state.owner.store,[]⟩ pending)
 (meaning : OwnerEffectService.Commits ops
   (MixedCatalogService.world live.session.state.source.machine.store.core.system.configuration.state
     live.session.state.source.machine.store.core.system.view)
   (MixedCatalogService.registry live.session.state.source.machine.store.core.system.configuration.state)
   ⟨live.session.state.owner.store,[]⟩ pending written) :
 (execute live candidate ops).session.state.owner.history =
   live.session.state.owner.history ++ [MixedOwnerAttemptQueue.record written] ∧
 (execute live candidate ops).session.state.owner.queued = none := by
 have ready : MixedOwnerSourceTrace.serviceReady live.session.state = true :=
   MixedOwnerSourceTrace.service_ready_exact.mpr ⟨held,candidate,waiting,queued,same,retained⟩
 have lower := MixedOwnerAttemptQueue.committed_complete queued materialized admitted meaning
 have result := MixedOwnerSourceTrace.service_commit_retained ready lower
 have index : CompositionCore.index p packet.owner = some place := by
   rw [current.2.2.2.1]
   exact CompositionCore.index_roundtrip place
 have checked := OwnerMetadataPending.check_exact.mpr current
 simpa only [execute,queued,if_pos rfl,service,packetHeld,index,if_pos checked,
   MixedOwnerContinuation.service] using And.intro result.1 result.2.1

#print axioms execute_committed_complete

#print axioms refused_exact
#print axioms execute_refines
#print axioms execute_stale
#print axioms trace_changed_empty
#print axioms service_changed_empty
#print axioms no_repeated_execution
end MirroreaProofFirst.OwnerStatementEntryGate
