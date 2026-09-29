import MixedReferenceSelection
import MixedReferenceHolding
import MixedCatalogTransitions
import MixedManagementControls
namespace MirroreaProofFirst.MixedReferenceControls
open MixedReferenceAccess
-- Explicit independently supplied fixture grants for access and holding.
-- Actual catalog construction uses the common authorized management entries.
def accessClaim : CurrentUse.Claim := ⟨84,4,0,7,0,1,91,30,[13],[0],0⟩
def otherClaim : CurrentUse.Claim := {accessClaim with id := 86,targets := [2]}
def holdClaim : CurrentUse.Claim := {accessClaim with id := 85,predicate := 50,actions := [14]}
def accessPolicy : CurrentUse.Policy := ⟨50,1,0,.leaf ⟨4,30⟩⟩
def holdPolicy : CurrentUse.Policy := ⟨80,1,0,.leaf ⟨4,50⟩⟩
def built := MixedManagementControls.bothBuilt.bind fun (s,_) =>
 (MixedManagementEntry.perform s 0 0 7 4 (.instantiate 0 7 [0] none .top)).map fun (next,_) =>
 {next with
  view := {next.view with authority := {next.view.authority with issued := accessClaim :: otherClaim :: holdClaim :: next.view.authority.issued}},
  controlPolicy := fun action => if action=13 then accessPolicy else if action=14 then holdPolicy else next.controlPolicy action}
def first : FallbackStatic.OptionDecl := ⟨"first",0,some "pure-read",.read,InstancePrograms.Controls.contract,100⟩
def second : FallbackStatic.OptionDecl := {first with name := "second",target := 2}
def chain : FallbackStatic.Chain := ⟨2,[first,second],[some ⟨"first","second",true⟩]⟩
def request : ReferenceAccess.Request := ⟨0,0,0,chain,0,0,0,7⟩
def saved := built.bind fun s => (prepare s accessPolicy request).map fun g => (s,g)
def held := built.bind fun s => (MixedReferenceHolding.prepare .separate s request).map fun g => (s,g)
def revoke (s : MixedManagementEntry.System 3 1) (ids : List Nat) :=
 {s with view := {s.view with authority := {s.view.authority with revoked := ids}}}
def frame (s : MixedManagementEntry.System 3 1) : MixedReferenceHistory.Frame 3 1 := ⟨s,accessPolicy⟩
#guard built.isSome
#guard saved.isSome && held.isSome
#guard (built.map fun s => match MixedFallbackStatic.check s.configuration.state chain with | .ok _ => true | .error _ => false) = some true
#guard (saved.map fun (s,g) => check s accessPolicy request g) = some true
#guard (saved.map fun (s,g) => check (revoke s [84]) accessPolicy request g) = some false
#guard (held.map fun (s,g) => MixedReferenceHolding.check .separate (revoke s [84]) request g) = some true
#guard (saved.map fun (s,g) => check (revoke s [85]) accessPolicy request g) = some true
#guard (held.map fun (s,g) => MixedReferenceHolding.check .separate (revoke s [85]) request g) = some false
#guard (saved.map fun (s,g) => MixedReferenceHistory.usable (frame s) [frame s,frame (revoke s [84])] request g) = some false
#guard (held.map fun (s,g) => MixedReferenceHolding.liveCheck .separate request g 0
 [frame s,frame (revoke s [85]),frame s]) = some false
#guard (built.bind fun s => MixedReferenceSelection.search s accessPolicy request 0).map ReferenceSelection.Choice.index = some 0
#guard (built.bind fun s => MixedReferenceSelection.search (revoke s [84]) accessPolicy request 0).map ReferenceSelection.Choice.index = some 1
#guard (built.bind fun s => MixedReferenceSelection.search (revoke s [84,86]) accessPolicy request 0).isNone
#guard (saved.map fun (s,g) => check {s with serial := 100} accessPolicy request g) = some false
#guard (held.map fun (s,g) => MixedReferenceHolding.check .separate {s with serial := 100} request g) = some true
-- Owner-tagged catalog entries are not callable pure fallback options.
def ownerChain : FallbackStatic.Chain := ⟨1,[{first with target := 1}],[]⟩
#guard (built.map fun s => match MixedFallbackStatic.check s.configuration.state ownerChain with | .error .unresolved => true | _ => false) = some true
#guard (built.bind fun s => prepare s accessPolicy {request with chain := ownerChain}).isNone

-- Actual leave/join through another participating locus, not a fabricated image.
def rejoined := saved.bind fun (s,g) => do
 let (left,_) ← MixedManagementEntry.perform s 0 1 7 5 (.leave 0)
 let (back,_) ← MixedManagementEntry.perform left 0 1 7 6 (.join 0)
 return (back,g)
#guard rejoined.isSome
#guard (rejoined.map fun (s,g) => check s accessPolicy request g) = some false
#guard (rejoined.bind fun (s,_) => prepare s accessPolicy request).isSome
-- Fresh acquisition is possible; existing guard remains old. Store mutation
-- closure, full source, image import and physical head provenance are not claimed.
end MirroreaProofFirst.MixedReferenceControls
