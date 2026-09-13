import MirroreaProofFirstReferenceAccess
import MirroreaProofFirstFallbackStaticControls

namespace MirroreaProofFirst.ReferenceAccessControls
open ReferenceAccess FallbackStaticControls

def accessClaim : CurrentUse.Claim := ⟨40,4,0,7,0,1,91,30,[13],[0,1],0⟩
def accessPolicy : CurrentUse.Policy := ⟨50,1,0,.leaf ⟨4,30⟩⟩
def system : ManagementEntry.System 3 1 :=
  {ManagementEntry.Controls.initial with
    configuration := CompositionCore.config state,
    view := {InvocationBoundary.Controls.view with
      authority := {InvocationBoundary.Controls.view.authority with issued := [accessClaim]}}}
def request : Request := ⟨0,0,0,chain,0,0,2,7⟩
def saved := prepare system accessPolicy request
#guard saved.isSome
#guard (saved.map (check system accessPolicy request)) = some true
-- Holding a reference is not an invocation permit, in either direction.
def invocation (s : ManagementEntry.System 3 1) := (CompositionCore.index s.configuration.count 1).bind fun key =>
  InvocationBoundary.prepare s.configuration.state s.view 0 key 2 7 80 2
#guard (invocation system).isNone
def invocationOnly : ManagementEntry.System 3 1 := {system with view := InvocationBoundary.Controls.view}
#guard (invocation invocationOnly).isSome
#guard (prepare invocationOnly accessPolicy request).isNone
#guard (saved.map (check system accessPolicy {request with epoch := 1})) = some false
#guard (saved.map (check system accessPolicy {request with lineage := 1})) = some false
#guard (saved.map (check system accessPolicy {request with binding := 1})) = some false
#guard (saved.map (check system accessPolicy {request with optionIndex := 1})) = some false
#guard (prepare {system with serial := 100} accessPolicy request).isNone
#guard (saved.map (check {system with serial := 100} accessPolicy request)) = some false
#guard (prepare system accessPolicy {request with optionIndex := 99}).isNone
#guard (prepare system accessPolicy {request with member := 1}).isNone
#guard (prepare system accessPolicy {request with principal := 8}).isNone
def revoked : ManagementEntry.System 3 1 := {system with view := {system.view with
  authority := {system.view.authority with revoked := [40]}}}
#guard (saved.map (check revoked accessPolicy request)) = some false
def rejoined : ManagementEntry.System 3 1 :=
  {system with configuration := CompositionCore.config (InstanceState.join (InstanceState.leave state 2) 2)}
#guard (saved.map (check rejoined accessPolicy request)) = some false
#guard (prepare rejoined accessPolicy request).isSome
def replaced : ManagementEntry.System 3 1 :=
  {system with configuration := CompositionCore.config InstanceState.Controls.replaced}
#guard (saved.map (check replaced accessPolicy request)) = some false
#guard (prepare replaced accessPolicy request).isSome
def alternative : CurrentUse.Claim := {accessClaim with id := 41,predicate := 31}
def eitherPolicy : CurrentUse.Policy := {accessPolicy with expression := .either (.leaf ⟨4,30⟩) (.leaf ⟨4,31⟩)}
def eitherSystem : ManagementEntry.System 3 1 := {system with view := {system.view with
  authority := {system.view.authority with issued := [accessClaim,alternative]}}}
def eitherSaved := prepare eitherSystem eitherPolicy request
#guard (eitherSaved.map Guard.witness) = some (.left (.leaf accessClaim))
def leftRevoked : ManagementEntry.System 3 1 := {eitherSystem with view := {eitherSystem.view with
  authority := {eitherSystem.view.authority with revoked := [40]}}}
#guard (eitherSaved.map (check leftRevoked eitherPolicy request)) = some false
#guard ((prepare leftRevoked eitherPolicy request).map Guard.witness) = some (.right (.leaf alternative))
-- Even an alternative issued claim for the SAME leaf cannot replace the saved
-- leaf identity. This is stronger than merely checking disjunction-branch tags.
def sameLeaf : CurrentUse.Claim := {accessClaim with id := 42}
def sameLeafSystem : ManagementEntry.System 3 1 := {system with view := {system.view with
  authority := {system.view.authority with issued := [accessClaim,sameLeaf]}}}
def sameLeafSaved := prepare sameLeafSystem accessPolicy request
def firstLeafRevoked : ManagementEntry.System 3 1 := {sameLeafSystem with view := {sameLeafSystem.view with
  authority := {sameLeafSystem.view.authority with revoked := [40]}}}
#guard (sameLeafSaved.map Guard.witness) = some (.leaf accessClaim)
#guard (sameLeafSaved.map (check firstLeafRevoked accessPolicy request)) = some false
#guard ((prepare firstLeafRevoked accessPolicy request).map Guard.witness) = some (.leaf sameLeaf)
-- This new guard is only suitable for fresh acquisition. Later binding code
-- must not use it to repair an invalid saved guard at the same option.
end MirroreaProofFirst.ReferenceAccessControls
