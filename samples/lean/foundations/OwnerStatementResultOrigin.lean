import OwnerStatementResultCollection
namespace MirroreaProofFirst.OwnerStatementResultOrigin
open OwnerStatementLiveCustodian OwnerStatementResultCollection

-- Raw list membership alone is not admission. These facts are derived from
-- a fresh source launch and the original complete registry lifecycle.
theorem admitted_collected_commit
 (entered : OwnerStatementRegistryInvariant.Admitted realm authority policy store s.live)
 (collected : collect s ticket = some (.committed written)) :
 written.pending = ticket.saved ∧ written ∈ s.live.session.state.owner.history := by
 have retained := (OwnerStatementRegistryInvariant.admitted_valid entered).1.retained
 have member := collected_actual collected
 refine ⟨retained.2 _ _ member,?_⟩
 rw [retained.1]
 exact List.mem_filterMap.mpr ⟨(ticket.saved,.committed written),member,rfl⟩

theorem admitted_missing_history_denies
 (entered : OwnerStatementRegistryInvariant.Admitted realm authority policy store s.live)
 (missing : written ∉ s.live.session.state.owner.history) :
 collect s ticket ≠ some (.committed written) := by
 intro collected
 exact missing (admitted_collected_commit entered collected).2

-- Expose the actual lower queue transition reached by the independent current
-- metadata/source gate. No successful execute/report result is a premise.
theorem checked_service_route {p a : Nat} {s : OwnerStatementRegistrySelection.State p a} (ready : currentCheck s = true) :
 ∃ place : Fin p, ∃ registry,
 (∃ saved, s.session.state.owner.queued = some saved ∧
 OwnerStatementRegistrySelection.select s.session s.bank saved = some (place,registry)) ∧
 (OwnerStatementRegistrySelection.service s ops).session.state.owner =
 (MixedOwnerAttemptQueue.advance ops
   s.session.state.source.machine.store.core.system.configuration.state
   s.session.state.source.machine.store.core.system.view
   (OwnerMetadataPending.currentFields s.addresses registry place.val)
   s.session.state.owner).1 := by
 unfold currentCheck at ready
 cases packet : s.packet with
 | none => simp [packet] at ready
 | some packetValue =>
   cases queued : s.session.state.owner.queued with
   | none => simp [packet,queued] at ready
   | some saved =>
     cases selected : OwnerStatementRegistrySelection.select s.session s.bank saved with
     | none => simp [packet,queued,selected] at ready
     | some pair =>
       obtain ⟨place,registry⟩ := pair
       simp only [packet,queued,selected,Bool.and_eq_true,decide_eq_true_eq] at ready
       have index : CompositionCore.index p packetValue.owner = some place := by
         rw [ready.1.1]; exact CompositionCore.index_roundtrip place
       have check : OwnerMetadataPending.check s.addresses
         (OwnerMetadataSession.management (OwnerStatementRegistrySelection.view s registry))
         place saved packetValue = true := by
         simpa [OwnerStatementEntryGate.serviceCheck,OwnerStatementRegistrySelection.view,packet,queued,index] using ready.1.2
       have gate : OwnerStatementEntryGate.execute (OwnerStatementRegistrySelection.view s registry) saved ops =
         OwnerMetadataSession.service (OwnerStatementRegistrySelection.view s registry) ops := by
         simp [OwnerStatementEntryGate.execute,OwnerStatementRegistrySelection.view,queued]
       have metadata : (OwnerMetadataSession.service (OwnerStatementRegistrySelection.view s registry) ops).session =
         MixedOwnerContinuation.service s.session ops (OwnerMetadataPending.currentFields s.addresses registry place.val) := by
         simp only [OwnerStatementRegistrySelection.view,packet] at check
         simp only [OwnerMetadataSession.service,OwnerStatementRegistrySelection.view,packet,queued,index,if_pos check]
       refine ⟨place,registry,⟨saved,rfl,selected⟩,?_⟩
       simp only [OwnerStatementRegistrySelection.service,packet,queued,selected,if_pos ready.1.1]
       change (OwnerStatementEntryGate.execute (OwnerStatementRegistrySelection.view s registry) saved ops).session.state.owner = _
       rw [gate,metadata]
       simp [MixedOwnerContinuation.service,MixedOwnerSourceTrace.service,ready.2,MixedOwnerSourceTrace.serviceAdmitted]

theorem checked_service_owner {p a : Nat} {s : OwnerStatementRegistrySelection.State p a}
 {place : Fin p} {registry : OwnerMetadataRegistry.Registry}
 (ready : currentCheck s = true) (queued : s.session.state.owner.queued = some saved)
 (selected : OwnerStatementRegistrySelection.select s.session s.bank saved = some (place,registry)) :
 (OwnerStatementRegistrySelection.service s ops).session.state.owner =
 (MixedOwnerAttemptQueue.advance ops
   s.session.state.source.machine.store.core.system.configuration.state
   s.session.state.source.machine.store.core.system.view
   (OwnerMetadataPending.currentFields s.addresses registry place.val)
   s.session.state.owner).1 := by
 obtain ⟨other,otherRegistry,⟨entry,atQueue,found⟩,same⟩ := checked_service_route (ops:=ops) ready
 have eqEntry : entry = saved := Option.some.inj (atQueue.symm.trans queued)
 subst entry
 have pair := Prod.mk.inj (Option.some.inj (found.symm.trans selected))
 obtain ⟨rfl,rfl⟩ := pair
 exact same

-- A live guarded execution, not an arbitrary unreported record, produces the
-- collectible result. Refusal and commitment use this same actual transition.
theorem execute_collect_lower {p a : Nat} {s : State p a}
 {place : Fin p} {registry : OwnerMetadataRegistry.Registry}
 (ready : Ready s ticket)
 (valid : MixedOwnerAttemptQueue.Invariant s.live.session.state.owner)
 (selected : OwnerStatementRegistrySelection.select s.live.session s.live.bank ticket.saved = some (place,registry))
 (completed : (MixedOwnerAttemptQueue.advance ops
   s.live.session.state.source.machine.store.core.system.configuration.state
   s.live.session.state.source.machine.store.core.system.view
   (OwnerMetadataPending.currentFields s.live.addresses registry place.val)
   s.live.session.state.owner).2 = some answer) :
 ∃ next, execute s ticket ops = some next ∧ collect next ticket = some answer := by
 refine ⟨_,execute_complete ready,?_⟩
 simp only [collect,ready.1,true_and]
 rw [checked_service_owner ready.2.2.2 ready.2.2.1 selected]
 exact lookup_actual_advance valid ready.2.2.1 completed

-- Success completeness uses independent source/current metadata readiness and
-- declarative lower Commits. No successful wrapper call is assumed.
theorem execute_collect_committed {p a : Nat} {s : State p a}
 {authority : WorldProjection.AuthorityView a}
 {place : Fin p} {registry : OwnerMetadataRegistry.Registry}
 {pending : OwnerEffectService.Pending
   (WorldProjection.size a p s.live.session.state.source.machine.store.core.system.configuration.count)}
 {written : OwnerEffectService.Write
   (WorldProjection.size a p s.live.session.state.source.machine.store.core.system.configuration.count)}
 (entered : OwnerStatementRegistryInvariant.Admitted realm authority policy store s.live)
 (ready : Ready s ticket)
 (selected : OwnerStatementRegistrySelection.select s.live.session s.live.bank ticket.saved = some (place,registry))
 (materialized : OwnerSavedPending.materialize
   (WorldProjection.size a p s.live.session.state.source.machine.store.core.system.configuration.count)
   ticket.saved = some pending)
 (admitted : MixedOwnerMaterialization.Admitted
   s.live.session.state.source.machine.store.core.system.configuration.state
   (OwnerMetadataPending.currentFields s.live.addresses registry place.val)
   ⟨s.live.session.state.owner.store,[]⟩ pending)
 (meaning : OwnerEffectService.Commits ops
   (MixedCatalogService.world s.live.session.state.source.machine.store.core.system.configuration.state
     s.live.session.state.source.machine.store.core.system.view)
   (MixedCatalogService.registry s.live.session.state.source.machine.store.core.system.configuration.state)
   ⟨s.live.session.state.owner.store,[]⟩ pending written) :
 ∃ next, execute s ticket ops = some next ∧
 collect next ticket = some (.committed (MixedOwnerAttemptQueue.record written)) := by
 have valid := (OwnerStatementRegistryInvariant.admitted_valid entered).1.joint.2.2.1
 exact execute_collect_lower ready valid selected
   (MixedOwnerAttemptQueue.committed_complete ready.2.2.1 materialized admitted meaning)

#print axioms admitted_collected_commit
#print axioms admitted_missing_history_denies
#print axioms checked_service_route
#print axioms checked_service_owner
#print axioms execute_collect_lower
#print axioms execute_collect_committed
end MirroreaProofFirst.OwnerStatementResultOrigin
