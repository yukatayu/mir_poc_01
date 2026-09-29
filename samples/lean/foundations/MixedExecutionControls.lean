import MixedReferenceExecution
import MixedStoreControls
import MixedCancelControls
namespace MirroreaProofFirst.MixedExecutionControls
open MixedReferenceExecution
-- All mutations use actual protected Execution entries from an empty root.
-- Grants are explicit independent fixture input; no source/physical E2E claim.
def view : WorldProjection.AuthorityView 1 :=
 {MixedStoreControls.view with authority := {MixedStoreControls.view.authority with issued :=
  {MixedCancelControls.claim with targets := [6]} :: MixedStoreControls.view.authority.issued}}
def built := do
 let initial : Machine 3 1 := MixedReferenceExecution.initial 91 view MixedStoreControls.policy
 let (m,_) ← manage initial 0 0 7 0 MixedManagementControls.pureCode
 let (m,_) ← manage m 0 0 7 1 (.instantiate 0 7 [0] none .top)
 let (m,_) ← manage m 0 0 7 2 MixedManagementControls.ownerCode
 let (m,_) ← manage m 0 0 7 3 (.instantiate 1 7 [0] none .top)
 let (m,_) ← manage m 0 0 7 4 (.instantiate 0 7 [0] none .top)
 acquire m 0 0 7 5 MixedReferenceControls.chain

def bothPending := built.bind fun (m,_) => do
 let (m,e) ← startReference m 0 0 7 6 0 2
 let (m,owner) ← startOwner m 0 0 7 7 1 MixedRequestControls.origin [.integer 5]
 return (m,e,owner)
#guard bothPending.isSome
#guard (bothPending.map fun (m,e,owner) => decide (m.pending = [e]) &&
 decide (m.store.core.pending = [.owner owner,.pure e.ticket]) && e.binding.isSome) = some true
#guard (bothPending.bind fun (m,e,_) => finishPlain m e.ticket 5).isNone
#guard (bothPending.bind fun (m,e,_) => finish m {e with binding := none} 5).isNone
#guard (bothPending.bind fun (m,_,_) => reacquire m 0 0 7 7 0).isNone
#guard (bothPending.bind fun (m,_,_) => reacquire m 0 0 7 6 0).isNone

def completed := bothPending.bind fun (m,e,owner) => (resume m e).map fun (next,value) => (next,e,owner,value)
#guard (completed.map fun (_,_,_,value) => value) = some 5
#guard (completed.map fun (m,_,owner,_) => m.pending.isEmpty && decide (m.store.core.pending = [.owner owner])) = some true
#guard (completed.bind fun (m,e,_,_) => resume m e).isNone
-- The pure classification is empty, but the shared owner wait remains real.
#guard (completed.map fun (m,_,_,_) => m.store.core.pending.isEmpty) = some false

def cancelled := bothPending.bind fun (m,e,owner) => (cancel m 0 0 7 8 e).map fun next => (next,e,owner)
#guard (cancelled.map fun (m,_,owner) => m.pending.isEmpty && decide (m.store.core.pending = [.owner owner])) = some true
#guard (cancelled.bind fun (m,e,_) => finish m e 5).isNone

def changedBinding := bothPending.bind fun (m,e,owner) => (reacquire m 0 0 7 8 0).map fun next => (next,e,owner)
#guard changedBinding.isSome
#guard (changedBinding.bind fun (m,e,_) => resume m e).isNone
#guard (changedBinding.bind fun (m,e,_) => cancel m 0 0 7 9 e).isSome

def lostHold := bothPending.map fun (m,e,owner) =>
 let next := authorityHead m {m.store.core.system.view with authority := {m.store.core.system.view.authority with revoked := [85]}}
 (next,e,owner)
#guard (lostHold.bind fun (m,e,_) => resume m e).isNone
#guard (lostHold.bind fun (m,e,_) => cancel m 0 0 7 8 e).isSome
#guard (lostHold.bind fun (m,e,_) =>
 let next := authorityHead m {m.store.core.system.view with authority := {m.store.core.system.view.authority with revoked := []}}
 resume next e).isNone
end MirroreaProofFirst.MixedExecutionControls
