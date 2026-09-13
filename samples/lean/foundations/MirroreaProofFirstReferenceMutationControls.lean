import MirroreaProofFirstReferenceMutation
import MirroreaProofFirstReferenceOwnerControls

namespace MirroreaProofFirst.ReferenceMutationControls
open ReferenceOwnerControls ReferenceAccessControls ReferenceOwner ReferenceStore ReferenceMutation

def primaryClaim : CurrentUse.Claim := {accessClaim with targets := [1]}
def terminalClaim : CurrentUse.Claim := {accessClaim with id := 41,targets := [0]}
def initialView : WorldProjection.AuthorityView 1 :=
  {system.view with authority := {system.view.authority with issued :=
    [primaryClaim,terminalClaim,ownerClaim,holdClaim,ManagementEntry.Controls.claim,InvocationBoundary.Controls.invocationClaim]}}
def initialSystem : ManagementEntry.System 3 1 :=
  {system with
    view := initialView
    controlPolicy := fun action => if action = 14 then holdPolicy else if action = 13 then accessPolicy else
      if action ∈ [9,10,11,12] then ownerPolicy else ManagementEntry.Controls.policy}
def initialCore : CompositionMachine.Machine 3 1 := ⟨initialSystem,[],[]⟩
-- Component start cut. Normal-source binding construction is a later consumer;
-- these initial core records are not presented as new source E2E evidence.
def initial : ReferenceStore.Machine 3 1 := ⟨initialCore,[],[frame initialCore],[]⟩
def acquired := acquire initial 0 2 7 100 FallbackStaticControls.chain
#guard acquired.isSome
#guard (acquired.map Prod.snd) = some 0
def selectedIndex (m : ReferenceStore.Machine p a) := (lookup m 0).bind fun b => b.selected.map ReferenceSelection.Choice.index
#guard (acquired.bind fun (m,_) => selectedIndex m) = some 0
#guard (acquired.map fun (m,_) => m.core.system.serial) = some 1
#guard (acquired.map fun (m,_) => coreEvents m.events == m.core.events) = some true
#guard (acquired.map fun (m,_) => (normalize m 0 2 7 101 0).consumed) = some false
def head (m : ReferenceStore.Machine 3 1) (ids : List Nat) :=
  ReferenceStore.authorityHead m {m.core.system.view with authority := {m.core.system.view.authority with revoked := ids}}
def lost := acquired.map fun (m,_) => head m [40]
def degraded := lost.map fun m => normalize m 0 2 7 101 0
#guard (degraded.map Outcome.consumed) = some true
#guard (degraded.bind fun result => selectedIndex result.state) = some 1
#guard (degraded.map fun result => result.state.core.system.serial) = some 3
#guard (degraded.map fun result => result.result matches .ok _) = some true
#guard (degraded.map fun result => result.state.core.system.used.contains ⟨91,7,101⟩) = some true
#guard (degraded.map fun result => coreEvents result.state.events == result.state.core.events) = some true
def recovered := degraded.map fun result => head result.state []
#guard (recovered.map fun m => (normalize m 0 2 7 102 0).consumed) = some false
#guard (recovered.bind fun m => selectedIndex (normalize m 0 2 7 102 0).state) = some 1
def reacquired := recovered.bind fun m => reacquire m 0 2 7 102 0
#guard (reacquired.bind selectedIndex) = some 0
#guard (reacquired.bind fun m => (lookup m 0).map fun b => (b.request.epoch,b.request.lineage)) = some (1,1)
-- Earlier consumed ids cannot authorize even another binding action.
#guard (acquired.bind fun (m,_) => release m 0 2 7 100 0).isNone
def noOwner := acquired.map fun (m,_) => head m [40,60]
def denied := noOwner.map fun m => normalize m 0 2 7 101 0
#guard (denied.map Outcome.consumed) = some false
#guard (denied.map fun result => result.result matches .error .mutationRejected) = some true
#guard (noOwner.map fun m => (normalize m 0 2 7 101 0).state.bindings == m.bindings) = some true
#guard (noOwner.map fun m => (normalize m 0 2 7 101 0).state.events == m.events) = some true
def allLost := acquired.map fun (m,_) => head m [40,41]
def exhausted := allLost.map fun m => normalize m 0 2 7 101 0
#guard (exhausted.map Outcome.consumed) = some true
#guard (exhausted.map fun result => result.result matches .error .exhausted) = some true
#guard (exhausted.bind fun result => (lookup result.state 0).map fun b => b.selected.isNone) = some true
#guard (exhausted.map fun result => (normalize (head result.state []) 0 2 7 102 0).result matches .error .exhausted) = some true
#guard (exhausted.bind fun result => (reacquire (head result.state []) 0 2 7 102 0).bind selectedIndex) = some 0
def released := acquired.bind fun (m,_) => release m 0 2 7 101 0
#guard (released.map fun m => (lookup m 0).isNone) = some true
#guard (released.map fun m => m.events.length) = some 2
#guard (released.map fun m => m.history.length) = some 3

-- Current binding floors constrain tentative reparent; release removes the
-- CURRENT row while retaining the old binding in a real owner occurrence.
def linkedCore : CompositionMachine.Machine 3 1 :=
  {initialCore with system := {initialSystem with configuration := CompositionCore.config FallbackStaticControls.linked}}
def linked : ReferenceStore.Machine 3 1 := ⟨linkedCore,[],[frame linkedCore],[]⟩
def childBound := acquire linked 0 2 7 100 FallbackStaticControls.childReader
#guard childBound.isSome
#guard (childBound.bind fun (m,_) => CompositionMachine.manage m.core 0 2 7 101 (.reparent 1 none)).isSome
#guard (childBound.bind fun (m,_) => ReferenceStore.manage m 0 2 7 101 (.reparent 1 none)).isNone
def childReleased := childBound.bind fun (m,_) => release m 0 2 7 101 0
#guard (childReleased.bind fun m => ReferenceStore.manage m 0 2 7 102 (.reparent 1 none)).isSome
end MirroreaProofFirst.ReferenceMutationControls
