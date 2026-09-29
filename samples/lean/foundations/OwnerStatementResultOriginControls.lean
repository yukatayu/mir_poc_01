import OwnerStatementResultOrigin
import OwnerStatementResultCollectionControls
namespace MirroreaProofFirst.OwnerStatementResultOriginControls
open OwnerStatementLiveCustodian OwnerStatementSourceCustodianControls OwnerStatementResultCollection

-- Deliberately invalid raw data, not an admitted import/transition. Queue
-- uniqueness alone does not authenticate the data stored in an attempts field.
def forged := executed.bind fun (s,ticket) => do
 let .committed original ← collect s ticket | none
 let written := {original with value := 17}
 let owner := {s.live.session.state.owner with store := (fun _ => some 200), history := [], queued := none, attempts := [(ticket.saved,.committed written)]}
 return ({s with live := {s.live with session := {s.live.session with state := {s.live.session.state with owner := owner}}}},ticket)
#guard forged.isSome
theorem raw_singleton_invariant : MixedOwnerAttemptQueue.Invariant
 (MixedOwnerAttemptQueue.State.mk store [] none [(saved,result)]) := by
 simp [MixedOwnerAttemptQueue.Invariant]
#guard (forged.map fun (s,_) => s.live.session.state.owner.queued.isNone) = some true
#guard (forged.map fun (s,_) => s.live.session.state.owner.store 0) = some (some 200)
#guard (forged.map fun (s,_) => s.live.session.state.owner.history.isEmpty) = some true
#guard (forged.bind fun (s,t) => collect s t |>.map fun r => match r with | .committed w => w.value | _ => 0) = some 17
#guard (forged.bind fun (s,t) => reportActual s t).isSome

-- The actual consumer requires the admission lineage separately. This general
-- rejection follows from that lineage, not from deciding one fixture.
theorem missing_history_not_admitted
 (collected : collect s ticket = some (.committed written))
 (missing : written ∉ s.live.session.state.owner.history) :
 ¬OwnerStatementRegistryInvariant.Admitted realm authority policy store s.live := by
 intro entered
 exact OwnerStatementResultOrigin.admitted_missing_history_denies entered missing collected
#print axioms missing_history_not_admitted
end MirroreaProofFirst.OwnerStatementResultOriginControls
