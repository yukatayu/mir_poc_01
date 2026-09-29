import OwnerStatementRegistryControls
import OwnerStatementLiveCustodian
namespace MirroreaProofFirst.OwnerStatementRegistryReviewControls
open OwnerStatementRegistrySelection OwnerStatementRegistryControls

-- Full elimination of the ACTUAL initial-install guard, not a new issuer.
theorem install_exact : install s place registry = some next ↔
 s.bank place = none ∧ registry.slots = [] ∧ registry.used = [] ∧ registry.serial = 0 ∧
 OwnerMetadataManagement.Attached ⟨s.session.state.source.machine.store.core.system,registry⟩ place ∧
 next = {s with bank := set s.bank place registry} := by
 unfold install
 split
 · rename_i checked
   have parts : s.bank place = none ∧ registry.slots = [] ∧ registry.used = [] ∧ registry.serial = 0 ∧
     OwnerMetadataManagement.Attached ⟨s.session.state.source.machine.store.core.system,registry⟩ place := by
     simpa only [Bool.and_eq_true,Option.isNone_iff_eq_none,List.isEmpty_iff,beq_iff_eq,
       OwnerMetadataManagement.attachment_exact,and_assoc] using checked
   simp only [Option.some.injEq]
   constructor
   · intro equal; exact ⟨parts.1,parts.2.1,parts.2.2.1,parts.2.2.2.1,parts.2.2.2.2,equal.symm⟩
   · rintro ⟨_,_,_,_,_,rfl⟩; rfl
 · rename_i denied
   constructor
   · intro impossible; cases impossible
   · rintro ⟨absent,slots,used,serial,attached,_⟩
     apply False.elim; apply denied
     simp [absent,slots,used,serial,OwnerMetadataManagement.attachment_exact.mpr attached]

-- Same-label reactivation removes the old lower-label confound.
def sameLabel := retiredT.bind fun s => changeMetadata s 0 1 7 ⟨"sequence.mir",22⟩ (.activate keyT 0)
#guard sameLabel.isSome
#guard (sameLabel.map fun s => ((service s (FallibleFlow.signed 63)).session.state.owner.store 1,
 (service s (FallibleFlow.signed 63)).session.state.owner.history.length,
 (service s (FallibleFlow.signed 63)).session.state.owner.attempts.length,
 (service s (FallibleFlow.signed 63)).session.state.owner.queued.isSome)) = some (some 10,1,1,true)
#guard (sameLabel.map OwnerStatementLiveCustodian.currentCheck) = some false

-- This deliberately bypasses ONLY the outer metadata incarnation packet gate.
-- It is a discriminator, not an admitted entry or a recommended executor.
def bypassMetadata (s : State 3 1) : Option (MixedOwnerSourceTrace.State 3 1) := do
 let saved ← s.session.state.owner.queued
 let (place,registry) ← select s.session s.bank saved
 return MixedOwnerSourceTrace.service (FallibleFlow.signed 63)
   (OwnerMetadataPending.currentFields s.addresses registry place.val) s.session.state
#guard (sameLabel.bind bypassMetadata |>.map fun s => (s.owner.store 1,s.owner.history.length,s.owner.attempts.length)) = some (some 35,2,2)
#guard (sameLabel.map fun s => MixedOwnerSourceTrace.serviceReady s.session.state) = some true

-- Omit first T installation along the real source path; no raw bank deletion.
def missingBeforeIssue := preparedT.map fun s => tick s 0 7
#guard missingBeforeIssue.isSome
#guard (missingBeforeIssue.map fun s => (s.session.state.owner.history.length,
 s.session.state.owner.queued.isNone,s.session.state.source.waiting.isNone,
 (OwnerStatementPosition.writes s.session.cursor.completed).map Prod.fst)) = some (1,true,true,[0])

-- An unrelated metadata edit is routed through the real shared source
-- allocator, not assumed harmless merely from bank-slot equality.
def changedSWhileTQueued := queuedT.bind fun s => changeMetadata s 0 0 7 ⟨"sequence.mir",31⟩ (.retire keyS)
#guard changedSWhileTQueued.isSome
#guard (changedSWhileTQueued.map fun s => ((service s (FallibleFlow.signed 63)).session.state.owner.store 1,
 (service s (FallibleFlow.signed 63)).session.state.owner.history.length)) = some (some 35,2)

-- Raw field erasure can pass local current predicates. Closed admission is
-- stronger, and this actual malformed packet has no admitted root.
def erasedFields := queuedT.map fun s => {s with packet := s.packet.map (fun (packet : OwnerMetadataPending.Packet) => {packet with fields := []})}
#guard (erasedFields.map OwnerStatementLiveCustodian.currentCheck) = some true
#guard (erasedFields.map fun s => (service s (FallibleFlow.signed 63)).session.state.owner.history.length) = some 2

theorem erased_not_admitted
 (packet : s.packet = some erased) (empty : erased.fields = []) :
 ¬OwnerStatementRegistryInvariant.Admitted realm authority policy store s := by
 rintro ⟨initial,entered,path⟩
 exact (rooted_packet_nonempty entered.2.2 path packet) empty

-- Actual finite nonvacuity of the new custodian entry, kept separate from its
-- general preservation/completeness theorems and physical initial-designation TCB.
def staged := queuedT.bind fun s => OwnerStatementLiveCustodian.stage 2 (OwnerStatementLiveCustodian.start 8 s)
def dispatched := staged.bind fun (s,ticket) => OwnerStatementLiveCustodian.execute s ticket (FallibleFlow.signed 63)
#guard staged.isSome && dispatched.isSome
#guard (dispatched.map fun s => (s.live.session.state.owner.store 1,s.dispatched.length,s.nextSerial)) = some (some 35,1,1)
#guard (staged.bind fun (s,ticket) => OwnerStatementLiveCustodian.execute {s with designation := 9} ticket (FallibleFlow.signed 63)).isNone
#guard (dispatched.bind fun s => OwnerStatementLiveCustodian.stage 2 s).isNone
#guard (staged.bind fun (s,ticket) => do
 let next ← OwnerStatementLiveCustodian.execute s ticket (FallibleFlow.signed 63)
 OwnerStatementLiveCustodian.execute next ticket (FallibleFlow.signed 63)).isNone
#guard (staged.bind fun (s,ticket) => do
 let next ← OwnerStatementLiveCustodian.execute s ticket (FallibleFlow.signed 63)
 OwnerStatementLiveCustodian.accept next ticket).isNone
#guard (staged.bind fun (s,ticket) => do
 let next ← OwnerStatementLiveCustodian.execute s ticket (FallibleFlow.signed 63)
 let reported ← OwnerStatementLiveCustodian.report next ticket
 OwnerStatementLiveCustodian.accept reported ticket |>.map fun done =>
   (done.live.session.state.owner.history.length,done.dispatched.length,done.nextSerial,done.held.isNone)) = some (2,1,1,true)

#print axioms install_exact
#print axioms erased_not_admitted
end MirroreaProofFirst.OwnerStatementRegistryReviewControls
