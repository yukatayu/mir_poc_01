import MixedReferenceOrigins
import MixedReferenceControls
import MixedRequestControls
namespace MirroreaProofFirst.MixedStoreControls
open MixedReferenceStore MixedReferenceMutation
-- Explicit issuer fixture. All catalog/row/request/history changes below use
-- their actual named Store entries, starting from an empty Store. This is a
-- finite local-model control, not parsed-source or physical-network evidence.
def bindingClaim : CurrentUse.Claim :=
 {ManagementEntry.Controls.claim with id := 87,actions := [9,10,11,12],targets := [0,1]}
def view : WorldProjection.AuthorityView 1 :=
 let v := MixedManagementControls.view
 {v with authority := {v.authority with issued :=
  MixedReferenceControls.accessClaim :: MixedReferenceControls.otherClaim ::
  {MixedReferenceControls.holdClaim with targets := [0,1]} :: bindingClaim :: v.authority.issued}}
def policy (action : Nat) : CurrentUse.Policy :=
 if action=13 then MixedReferenceControls.accessPolicy else
 if action=14 then MixedReferenceControls.holdPolicy else ManagementEntry.Controls.policy
def initial : Machine 3 1 := MixedReferenceStore.initial 91 view policy

def built := do
 let (m,_) ← manage initial 0 0 7 0 MixedManagementControls.pureCode
 let (m,_) ← manage m 0 0 7 1 (.instantiate 0 7 [0] none .top)
 let (m,_) ← manage m 0 0 7 2 MixedManagementControls.ownerCode
 let (m,_) ← manage m 0 0 7 3 (.instantiate 1 7 [0] none .top)
 let (m,_) ← manage m 0 0 7 4 (.instantiate 0 7 [0] none .top)
 acquire m 0 0 7 5 MixedReferenceControls.chain

def ownerWaiting := built.bind fun (m,_) => startOwner m 0 0 7 6 1 MixedRequestControls.origin [.integer 5]
def bothWaiting := ownerWaiting.bind fun (m,saved) => (start m 0 0 7 7 0 2).map fun (next,t) => (next,t,saved)
def degraded := bothWaiting.map fun (m,t,saved) =>
 let revoked := authorityHead m {m.core.system.view with authority := {m.core.system.view.authority with revoked := [84]}}
 (normalize revoked 0 0 7 8 0,t,saved)
#guard built.isSome && ownerWaiting.isSome && bothWaiting.isSome
#guard (ownerWaiting.bind fun (m,_) => reacquire m 0 0 7 6 0).isNone
#guard (bothWaiting.bind fun (m,_,_) => reacquire m 0 0 7 7 0).isNone
#guard (degraded.map fun (r,_,_) => r.consumed) = some true
#guard (degraded.map fun (r,_,_) => r.result.toOption.map ReferenceSelection.Choice.index) = some (some 1)
#guard (degraded.map fun (r,t,saved) => decide (r.state.core.pending = [.pure t,.owner saved])) = some true
#guard (degraded.map fun (r,_,_) => (lookup r.state 0).map fun b => b.holding.frontier) = some (some 5)
#guard (degraded.map fun (r,_,_) => decide (r.state.history.length = r.state.events.length+1)) = some true

def restored := degraded.map fun (r,_,_) =>
 let m := authorityHead r.state {r.state.core.system.view with authority := {r.state.core.system.view.authority with revoked := []}}
 normalize m 0 0 7 9 0
#guard (restored.map fun r => r.result.toOption.map ReferenceSelection.Choice.index) = some (some 1)
#guard (restored.map fun r => r.consumed) = some false

def released := restored.bind fun r => release r.state 0 0 7 9 0
def newBindingAtEnd := released.bind fun m => acquire m 0 0 7 10 MixedReferenceControls.chain
#guard (released.map fun m => m.bindings[0]?) = some (some none)
#guard (released.bind fun m => reacquire m 0 0 7 10 0).isNone
#guard (newBindingAtEnd.map fun (_,key) => key) = some 1
#guard (newBindingAtEnd.map fun (m,_) => m.bindings[0]?) = some (some none)
end MirroreaProofFirst.MixedStoreControls
