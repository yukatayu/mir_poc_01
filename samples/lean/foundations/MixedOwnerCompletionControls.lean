import MixedOwnerSourceIssueControls
namespace MirroreaProofFirst.MixedOwnerCompletionControls
open MixedOwnerSourceIssue MixedOwnerSourceIssueControls

-- The incoming acknowledgment here is made from the actual guarded service
-- result. No expected JSON, supplied integer result, or fabricated write.
-- This finite sequence has no replay queue/full-Session/physical transport claim.
def committed := issued.bind fun (s,w) => do
 let pending ← OwnerSavedPending.materialize (WorldProjection.size 1 3 s.machine.store.core.system.configuration.count) w.saved
 let owner : OwnerEffectService.Owner (WorldProjection.size 1 3 s.machine.store.core.system.configuration.count) :=
   ⟨fun k => if k=0 then some 10 else none,[]⟩
 let result := MixedOwnerMaterialization.serve (FallibleFlow.signed 63) s.machine.store.core.system.configuration.state
   s.machine.store.core.system.view MixedOwnerMaterializationControls.metadata owner pending
 let some (.committed write) := result.2 | none
 let ack := acknowledge s (OwnerSavedPending.save write.pending) write.serviceEvidence
 return (ack,result.1.store 0,result.1.history.length,OwnerSavedPending.save write.pending,write.serviceEvidence)
#guard (committed.map fun (ack,_,_,_,_) => ack.isSome) = some true
#guard (committed.map fun (_,value,_,_,_) => value) = some (some 15)
#guard (committed.map fun (_,_,history,_,_) => history) = some 1
#guard (committed.bind fun (ack,_,_,_,_) => ack.map fun s => s.waiting.isNone && s.machine.store.core.pending.isEmpty) = some true
#guard (committed.bind fun (ack,_,_,_,_) => ack.map fun s => s.writes.length) = some 1
#guard (committed.bind fun (ack,_,_,_,_) => ack.map fun s => s.nextRequest) = some 8
#guard (committed.bind fun (ack,_,_,pending,evidence) => ack.bind fun s => acknowledge s pending evidence).isNone
#guard (committed.bind fun (ack,_,_,_,_) => ack.map fun s =>
 (s.machine.store.core.system.used.map CurrentUse.UseId.request).contains 7) = some true

def nextPure := committed.bind fun (ack,_,_,_,_) => ack.map fun s =>
 completePure (advancePure s 0 0 7 ⟨⟨"shared-source.mir",50⟩,.plain (.invoke "after" "pure" (.integer 1))⟩).state
#guard (nextPure.map fun r => r.status) = some .ready
#guard (nextPure.map fun r => ReferenceSourceData.lookup r.state.values "after") = some (some (.plain (.integer 2 false)))
#guard (nextPure.map fun r => r.state.nextRequest) = some 9

-- The original pure contract permits only [-2,-1,0,1,2]. The first positive
-- mistakenly supplied3; retain its ACTUAL rejection as a negative, not a relaxed
-- contract. Completing a failed/no-wait state produces .awaiting and hid that
-- initial cause in the developmental control; inspect the entry directly.
#guard (committed.bind fun (ack,_,_,_,_) => ack.map fun s =>
 (advancePure s 0 0 7 ⟨⟨"shared-source.mir",50⟩,.plain (.invoke "after" "pure" (.integer 3))⟩).status) = some (.failed .rejected)
#guard (committed.bind fun (ack,_,_,_,_) => ack.map fun s =>
 (advancePure s 0 0 7 ⟨⟨"shared-source.mir",50⟩,.plain (.invoke "after" "pure" (.integer 3))⟩).state.nextRequest) = some 8

-- A real checked head successor preserves the exact old wait. Service can use
-- its generation-only revalidation path; old-context ack refuses. Keep the
-- ACTUAL resulting owner outside Option acknowledgement so commit is not erased.
def changedHead := issued.bind fun (s,w) => (authorityHead s
 {s.machine.store.core.system.view with generation := s.machine.store.core.system.view.generation+1}).map fun next => (next,w)
def stranded := changedHead.bind fun (s,w) => do
 let pending ← OwnerSavedPending.materialize (WorldProjection.size 1 3 s.machine.store.core.system.configuration.count) w.saved
 let owner : OwnerEffectService.Owner (WorldProjection.size 1 3 s.machine.store.core.system.configuration.count) :=
   ⟨fun k => if k=0 then some 10 else none,[]⟩
 let result := MixedOwnerMaterialization.serve (FallibleFlow.signed 63) s.machine.store.core.system.configuration.state
   s.machine.store.core.system.view MixedOwnerMaterializationControls.metadata owner pending
 let some (.committed write) := result.2 | none
 return (acknowledge s (OwnerSavedPending.save write.pending) write.serviceEvidence,
   result.1.store 0,result.1.history.length,s.waiting,s.machine.store.core.pending.length)
#guard changedHead.isSome
#guard (stranded.map fun (ack,_,_,_,_) => ack.isNone) = some true
#guard (stranded.map fun (_,value,_,_,_) => value) = some (some 15)
#guard (stranded.map fun (_,_,history,_,_) => history) = some 1
#guard (stranded.map fun (_,_,_,waiting,_) => waiting.isSome) = some true
#guard (stranded.map fun (_,_,_,_,pending) => pending) = some 1
end MirroreaProofFirst.MixedOwnerCompletionControls
