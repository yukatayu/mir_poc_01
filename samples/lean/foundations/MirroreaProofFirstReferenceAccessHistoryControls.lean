import MirroreaProofFirstReferenceAccessHistory
import MirroreaProofFirstReferenceAccessControls

namespace MirroreaProofFirst.ReferenceAccessHistoryControls
open ReferenceAccess ReferenceAccessHistory ReferenceAccessControls

def before : Frame 3 1 := ⟨system,accessPolicy⟩
def during : Frame 3 1 := ⟨revoked,accessPolicy⟩
def recovered : Frame 3 1 := ⟨{system with serial := 2},accessPolicy⟩
-- Concrete transient authorization loss in the SAME state/authority model.
-- A check of only the latest head forgets it; a complete recorded window does not.
#guard (saved.map (checks before request)) = some true
#guard (saved.map (checks during request)) = some false
#guard (saved.map (checks recovered request)) = some true
#guard (saved.map (usable recovered [before,during] request)) = some false
#guard (saved.map (usable recovered [before] request)) = some true
#guard (saved.map (usable during [] request)) = some false
-- A genuinely fresh explicit epoch can start its own new window; the future
-- owner-binding transition must authorize and record that reset, never hide it.
def newRequest := {request with epoch := 1,lineage := 1}
def fresh := prepare recovered.system recovered.policy newRequest
#guard fresh.isSome
#guard (fresh.map (usable recovered [] newRequest)) = some true

-- Transitive support can disappear without changing the selected target's
-- revision, owner membership, selected locus incarnation, or authority.
def dependency := InstanceState.instantiate InstanceState.Controls.empty 0 7 [1] none .top
def target := InstanceState.instantiate dependency 0 7 [0,2] none (.ref 0)
def actualReader := InstanceState.instantiate target 0 7 [0,2] none .top
def supportChain := {request.chain with reader := 2, options :=
  [FallbackStaticControls.primary,{FallbackStaticControls.terminal with target := 2}]}
def supportRequest := {request with chain := supportChain,place := 0}
def supportView : WorldProjection.AuthorityView 1 :=
  {system.view with authority := {system.view.authority with issued := [accessClaim,ManagementEntry.Controls.claim]}}
def supportMachine : CompositionMachine.Machine 3 1 :=
  ⟨{system with configuration := CompositionCore.config actualReader,view := supportView},[],[]⟩
def supportGuard := prepare supportMachine.system accessPolicy supportRequest
def supportLeft := CompositionMachine.manage supportMachine 0 0 7 10 (.leave 1)
def supportRejoined := supportLeft.bind fun (m,_) => CompositionMachine.manage m 0 0 7 11 (.join 1)
#guard supportGuard.isSome
#guard supportLeft.isSome
#guard supportRejoined.isSome
#guard (supportGuard.bind fun g => supportLeft.map fun (m,_) => check m.system accessPolicy supportRequest g) = some false
#guard (supportGuard.bind fun g => supportRejoined.map fun (m,_) => check m.system accessPolicy supportRequest g) = some true
def supportWindow := do
  let g ← supportGuard
  let (middle,_) ← supportLeft
  let (last,_) ← supportRejoined
  return usable ⟨last.system,accessPolicy⟩ [⟨supportMachine.system,accessPolicy⟩,⟨middle.system,accessPolicy⟩] supportRequest g
#guard supportWindow = some false
-- A later option that is expired when primary first fails remains expired in
-- this serial-clock model. Availability can still change by owner-held locus
-- participation; lazy/eager selection are not asserted trace equivalent.
end MirroreaProofFirst.ReferenceAccessHistoryControls
