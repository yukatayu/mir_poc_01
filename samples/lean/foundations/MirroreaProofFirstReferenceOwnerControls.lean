import MirroreaProofFirstReferenceOwner
import MirroreaProofFirstReferenceAccessControls

namespace MirroreaProofFirst.ReferenceOwnerControls
open ReferenceOwner ReferenceAccessControls
def ownerClaim : CurrentUse.Claim := ⟨60,4,0,7,0,1,91,40,[9,10,11,12],[0],0⟩
def ownerPolicy : CurrentUse.Policy := ⟨70,1,0,.leaf ⟨4,40⟩⟩
def holdClaim : CurrentUse.Claim := {ownerClaim with id := 70,predicate := 50,actions := [14]}
def holdPolicy : CurrentUse.Policy := ⟨80,1,0,.leaf ⟨4,50⟩⟩
def ownerSystem : ManagementEntry.System 3 1 :=
  {system with
    controlPolicy := fun _ => ownerPolicy
    view := {system.view with authority := {system.view.authority with issued := [ownerClaim]}}}
-- Full proposed payload for owner authorization tests, not a rooted stored
-- binding assertion. Release intentionally requires no live holding right.
def binding : Binding := ⟨request,ReferenceSelection.search system accessPolicy request 0,0,0,
  ⟨⟨ReferenceHolding.current .separate ownerSystem request 0 2,holdPolicy.id,holdPolicy.version,.leaf holdClaim⟩,0,0⟩⟩
def release := Change.release binding
def releasePermit := authorize ownerSystem 0 release 0 2 7 80
#guard releasePermit.isSome
#guard (releasePermit.map (check ownerSystem 0 release 0 2 7 80)) = some true
#guard (authorize ownerSystem 0 release 0 2 8 80).isNone
#guard (authorize ownerSystem 0 release 0 0 7 80).isNone
#guard (releasePermit.map (check ownerSystem 1 release 0 2 7 80)) = some false
#guard (releasePermit.map (check ownerSystem 0 release 0 2 7 81)) = some false
-- Read-reference authorization and owner mutation authorization are distinct.
#guard (authorize system 0 release 0 2 7 80).isNone
#guard (ReferenceAccess.prepare ownerSystem accessPolicy request).isNone
def changedEpoch := Change.release {binding with request := {binding.request with epoch := 1}}
def changedBinding := Change.release {binding with request := {binding.request with binding := 1}}
#guard (releasePermit.map (check ownerSystem 0 changedEpoch 0 2 7 80)) = some false
#guard (releasePermit.map (check ownerSystem 0 changedBinding 0 2 7 80)) = some false
#guard (authorize ownerSystem 0 changedBinding 0 2 7 80).isNone
def ownerRejoined : ManagementEntry.System 3 1 :=
  {ownerSystem with configuration := rejoined.configuration,serial := 2}
#guard (releasePermit.map (check ownerRejoined 0 release 0 2 7 80)) = some false
#guard (authorize ownerRejoined 0 release 0 2 7 81).isSome
def memberRejoined : ManagementEntry.System 3 1 :=
  {ownerSystem with view := {ownerSystem.view with members := fun k => {ownerSystem.view.members k with incarnation := 2}}}
#guard (authorize memberRejoined 0 release 0 2 7 81).isNone
-- A new issued claim is explicit environment input in this control. The proof
-- checker does not manufacture it from the binding or old claim.
def newlyIssued : ManagementEntry.System 3 1 :=
  {memberRejoined with view := {memberRejoined.view with authority := {memberRejoined.view.authority with
    issued := [{ownerClaim with memberIncarnation := 2}]}}}
#guard (authorize newlyIssued 0 release 0 2 7 81).isSome
#guard (releasePermit.map (check newlyIssued 0 release 0 2 7 80)) = some false
end MirroreaProofFirst.ReferenceOwnerControls
