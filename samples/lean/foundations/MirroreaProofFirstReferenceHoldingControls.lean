import MirroreaProofFirstReferenceHolding
import MirroreaProofFirstReferenceOwnerControls

namespace MirroreaProofFirst.ReferenceHoldingControls
open ReferenceHolding

-- Independent finite test issuer inputs. No issuer/code branch depends on
-- these example names. Split leaves expose distinct revocation meanings.
def acquireClaim : CurrentUse.Claim := {ReferenceOwnerControls.ownerClaim with id := 61,actions := [9]}
def degradeClaim : CurrentUse.Claim := {ReferenceOwnerControls.ownerClaim with id := 62,actions := [10]}
def reacquireClaim : CurrentUse.Claim := {ReferenceOwnerControls.ownerClaim with id := 63,actions := [11]}
def releaseClaim : CurrentUse.Claim := {ReferenceOwnerControls.ownerClaim with id := 64,actions := [12]}
def holdClaim : CurrentUse.Claim := {ReferenceOwnerControls.ownerClaim with id := 70,predicate := 50,actions := [14]}
def holdPolicy : CurrentUse.Policy := ⟨80,1,0,.leaf ⟨4,50⟩⟩
def system : ManagementEntry.System 3 1 :=
  {ReferenceAccessControls.system with
    controlPolicy := fun action => if action = 14 then holdPolicy else ReferenceOwnerControls.ownerPolicy
    view := {ReferenceAccessControls.system.view with authority :=
      {ReferenceAccessControls.system.view.authority with issued := [acquireClaim,degradeClaim,reacquireClaim,releaseClaim,holdClaim]}}}
def request := ReferenceAccessControls.request
def savedH := prepare .separate system request
def savedH2 := prepare .acquisition system request
#guard savedH.isSome
#guard savedH2.isSome
#guard (savedH.map (check .separate system request)) = some true
#guard (savedH2.map (check .acquisition system request)) = some true
def revoke (ids : List Nat) : ManagementEntry.System 3 1 :=
  {system with view := {system.view with authority := {system.view.authority with revoked := ids}}}
#guard (savedH.map (check .separate (revoke [61]) request)) = some true
#guard (savedH2.map (check .acquisition (revoke [61]) request)) = some false
#guard (savedH.map (check .separate (revoke [70]) request)) = some false
#guard (savedH2.map (check .acquisition (revoke [70]) request)) = some true
#guard (savedH.map (check .separate (revoke [62,63,64]) request)) = some true
#guard (savedH2.map (check .acquisition (revoke [62,63,64]) request)) = some true
#guard (savedH.map (check .separate {system with serial := system.serial+1} request)) = some true
#guard (savedH2.map (check .acquisition {system with serial := system.serial+1} request)) = some true
#guard (savedH.map (check .acquisition system request)) = some false
#guard (savedH2.map (check .separate system request)) = some false
-- Holding cannot mint selected access, invocation, or owner mutation authority.
#guard (ReferenceAccess.prepare system ReferenceAccessControls.accessPolicy request).isNone
#guard (ReferenceAccessControls.invocation system).isNone
#guard (ReferenceOwner.authorize (revoke [64]) 0 (.release ReferenceOwnerControls.binding) 0 2 7 80).isNone
-- Holding describes owner permission, not target availability/code validity.
#guard (savedH.map (check .separate {system with configuration := ReferenceAccessControls.replaced.configuration} request)) = some true
#guard (savedH.map (check .separate {system with configuration := ReferenceAccessControls.rejoined.configuration} request)) = some false
#guard (savedH.map (check .separate {system with view := {system.view with generation := system.view.generation+1}} request)) = some false

def frame (s : ManagementEntry.System 3 1) : ReferenceAccessHistory.Frame 3 1 := ⟨s,s.controlPolicy 13⟩
def history := [frame system,frame (revoke [70]),frame system]
#guard (savedH.map (liveCheck .separate request · 0 history)) = some false
#guard (savedH.map (liveCheck .separate request · 0 (history ++ [frame system]))) = some false
#guard (savedH.map (liveCheck .separate request · 100 history)) = some false
#guard (savedH2.map (liveCheck .acquisition request · 0 history)) = some true
-- This deliberately demonstrates why an arbitrary replacement origin is not
-- admitted: resetting to the restoration frame would launder the lost right.
#guard (savedH.map (liveCheck .separate request · 2 history)) = some true

def nextRequest := {request with epoch := 1,lineage := 1}
#guard (savedH.map (check .separate system nextRequest)) = some false
#guard (prepare .separate system nextRequest).isSome
#guard (prepare .separate (revoke [70]) nextRequest).isNone
#guard (savedH.map (check .separate system {request with optionIndex := 1})) = some false
#guard (savedH.map (check .separate system {request with binding := 1})) = some false
#guard (savedH.map (check .separate system {request with chain := {request.chain with reader := 1}})) = some false
#guard (savedH.map (check .separate system {request with principal := 8})) = some false
#guard (savedH.map (check .separate system {request with place := 0})) = some false
#guard (savedH.map (check .separate {system with controlPolicy := fun _ => {holdPolicy with version := 2}} request)) = some false

-- Existential authorization after branch loss does not repair a saved witness.
def alternate : CurrentUse.Claim := {holdClaim with id := 71,predicate := 51}
def eitherPolicy : CurrentUse.Policy := {holdPolicy with expression := .either (.leaf ⟨4,50⟩) (.leaf ⟨4,51⟩)}
def eitherSystem : ManagementEntry.System 3 1 :=
  {system with controlPolicy := fun _ => eitherPolicy, view := {system.view with authority :=
    {system.view.authority with issued := [holdClaim,alternate]}}}
def eitherSaved := prepare .separate eitherSystem request
def eitherLost : ManagementEntry.System 3 1 :=
  {eitherSystem with view := {eitherSystem.view with authority := {eitherSystem.view.authority with revoked := [70]}}}
#guard (eitherSaved.map Guard.witness) = some (.left (.leaf holdClaim))
#guard (eitherSaved.map (check .separate eitherLost request)) = some false
#guard ((prepare .separate eitherLost request).map Guard.witness) = some (.right (.leaf alternate))
end MirroreaProofFirst.ReferenceHoldingControls
