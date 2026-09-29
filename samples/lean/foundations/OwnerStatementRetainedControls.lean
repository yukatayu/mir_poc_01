import OwnerStatementRetainedHistory
import OwnerStatementBoundaryControls
namespace MirroreaProofFirst.OwnerStatementRetainedControls
open OwnerMetadataSession

-- The SAME retained packet passes before retirement and fails afterwards.
-- These finite observations do not decide acknowledgment/disclosure policy.
def packetCurrent (s : State 3 1) : Option Bool := do
 let packet ← s.packet
 let place ← MixedCompositionCore.index 3 packet.owner
 return OwnerMetadataPending.check s.addresses (management s) place packet.saved packet
#guard (OwnerMetadataSessionControls.transferred.bind packetCurrent) = some true
#guard (OwnerStatementBoundaryControls.changedAfterService.bind packetCurrent) = some false
#guard (OwnerStatementBoundaryControls.changedAfterService.map fun s => s.packet) =
 OwnerMetadataSessionControls.served.map fun s => s.packet
#guard (OwnerStatementBoundaryControls.changedAfterService.map fun s => s.session.state.inbox) =
 OwnerMetadataSessionControls.served.map fun s => s.session.state.inbox
#guard (OwnerStatementBoundaryControls.changedAfterService.map fun s => s.session.state.owner.history) =
 OwnerMetadataSessionControls.served.map fun s => s.session.state.owner.history

theorem attached_consistent
 (run : OwnerMetadataManagementControls.attach system = some result) :
 OwnerMetadataRegistry.Consistent result.metadata := by
 unfold OwnerMetadataManagementControls.attach at run
 cases found : MixedCompositionCore.index system.configuration.count 1 with
 | none => simp [found] at run
 | some key =>
   simp only [found,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at run
   subst result
   exact OwnerMetadataRegistry.empty_consistent rfl

theorem prepared_rooted (run : OwnerMetadataSessionControls.prepared = some s) :
 MixedOwnerContinuation.Rooted 91 OwnerMetadataStoreControls.settings.view
 OwnerMetadataStoreControls.settings.controlPolicy MixedOwnerSourceTraceControls.initialStore s := by
 unfold OwnerMetadataSessionControls.prepared at run
 cases begun : OwnerMetadataSourceControls.begun with
 | none => simp [begun] at run
 | some initial =>
   simp only [begun,Option.map_some,Option.some.injEq] at run
   subst s
   have path : MixedOwnerContinuation.Rooted 91 OwnerMetadataStoreControls.settings.view
       OwnerMetadataStoreControls.settings.controlPolicy MixedOwnerSourceTraceControls.initialStore initial := .launch begun
   clear begun
   generalize fuelEq : 6 = fuel
   clear fuelEq
   induction fuel generalizing initial with
   | zero => exact path
   | succ fuel ih => exact ih _ (.tick path)

theorem attached_initial (run : OwnerMetadataSessionControls.attached = some s) :
 OwnerStatementAdmission.Initial 91 OwnerMetadataStoreControls.settings.view
 OwnerMetadataStoreControls.settings.controlPolicy MixedOwnerSourceTraceControls.initialStore s := by
 unfold OwnerMetadataSessionControls.attached at run
 cases prepared : OwnerMetadataSessionControls.prepared with
 | none => simp [prepared] at run
 | some session =>
   simp only [prepared,Option.bind_some] at run
   cases registry : OwnerMetadataManagementControls.attach session.state.source.machine.store.core.system with
   | none => simp [registry] at run
   | some metadata =>
     simp only [registry,Option.bind_eq_bind,Option.bind_some] at run
     exact ⟨session,metadata.metadata,[OwnerMetadataManagementControls.key],
       prepared_rooted prepared,attached_consistent registry,run⟩

theorem changed_rooted (start : OwnerMetadataSessionControls.attached = some initial)
 (run : OwnerStatementBoundaryControls.changedAfterService = some next) : Rooted initial next := by
 unfold OwnerStatementBoundaryControls.changedAfterService at run
 cases served : OwnerMetadataSessionControls.served with
 | none => simp [served] at run
 | some state =>
   simp only [served,Option.bind_some] at run
   exact .metadata (OwnerMetadataSessionControls.served_rooted start served) run

theorem changed_admitted (run : OwnerStatementBoundaryControls.changedAfterService = some next) :
 OwnerStatementAdmission.Admitted 91 OwnerMetadataStoreControls.settings.view
 OwnerMetadataStoreControls.settings.controlPolicy MixedOwnerSourceTraceControls.initialStore next := by
 cases start : OwnerMetadataSessionControls.attached with
 | none =>
   simp [OwnerStatementBoundaryControls.changedAfterService,OwnerMetadataSessionControls.served,
     OwnerMetadataSessionControls.transferred,OwnerMetadataSessionControls.requested,
     OwnerMetadataSessionControls.activated,start] at run
 | some initial => exact ⟨initial,attached_initial start,changed_rooted start run⟩

-- Do not kernel-normalize the full fixture into a giant decide proof.
-- Structural reachability and the generic theorem consume independently
-- evaluated small premises below.
def changed := OwnerStatementBoundaryControls.changedAfterService.getD OwnerStatementBoundaryControls.attached
theorem getD_run {α : Type} (value : Option α) (fallback : α) (inhabited : value.isSome = true) :
 value = some (value.getD fallback) := by cases value <;> simp_all
theorem changed_admitted_actual (inhabited : OwnerStatementBoundaryControls.changedAfterService.isSome = true) :
 OwnerStatementAdmission.Admitted 91 OwnerMetadataStoreControls.settings.view
 OwnerMetadataStoreControls.settings.controlPolicy MixedOwnerSourceTraceControls.initialStore changed :=
 changed_admitted (getD_run _ _ inhabited)
theorem actual_accepted
 (inhabited : OwnerStatementBoundaryControls.changedAfterService.isSome = true)
 (grows : OwnerStatementPosition.writes (tick changed 0 7).session.cursor.completed ≠
          OwnerStatementPosition.writes changed.session.cursor.completed) :
 OwnerStatementCompletion.Accepted changed.session (tick changed 0 7).session :=
 (OwnerStatementRetainedHistory.completion_changed_iff (changed_admitted_actual inhabited)).mp grows
#guard OwnerStatementBoundaryControls.changedAfterService.isSome = true
#guard OwnerStatementPosition.writes (tick changed 0 7).session.cursor.completed ≠
 OwnerStatementPosition.writes changed.session.cursor.completed
#guard ((tick changed 0 7).session.status,changed.session.state.owner.history.length,
 (OwnerStatementPosition.writes (tick changed 0 7).session.cursor.completed).length) = (.ready,1,1)

-- Counterfeit request/result pairing preserves the old Facts AND Exact
-- projection while violating the actual admission relation.
open OwnerStatementBoundaryControls (attached actual)
def counterfeit := do
 let written ← attached.session.state.owner.history.head?
 let substituted := {written.pending with origin := {written.pending.origin with ordinal := 999}}
 return (written,substituted,OwnerStatementRetainedHistory.replaceOne attached substituted written)
#guard attached.session.state.owner.queued = none
#guard (counterfeit.map fun (written,_,_) =>
 decide (attached.session.state.owner.attempts = [(written.pending,MixedOwnerAttemptQueue.Result.committed written)])) = some true
#guard (counterfeit.map fun (written,substituted,_) => decide (written.pending ≠ substituted)) = some true
#guard (counterfeit.map fun (_,_,state) => state.session.state.owner.history) =
 some attached.session.state.owner.history

#print axioms attached_consistent
#print axioms prepared_rooted
#print axioms attached_initial
#print axioms changed_rooted
#print axioms changed_admitted
#print axioms changed_admitted_actual
#print axioms actual_accepted
end MirroreaProofFirst.OwnerStatementRetainedControls
