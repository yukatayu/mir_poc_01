import OwnerMetadataManagement
import MixedManagementControls
namespace MirroreaProofFirst.OwnerMetadataManagementControls
open OwnerStructuredKeys OwnerSourceContext OwnerMetadataRegistry OwnerMetadataManagement

-- Mathematical issued-claim fixtures, not keys or an issuer implementation.
-- The configuration comes from actual checked register/instantiate operations.
def claimA : CurrentUse.Claim :=
 {ManagementEntry.Controls.claim with id := 92,actions := [22,23],targets := List.range 32}
def claimB : CurrentUse.Claim := {claimA with id := 93,issuer := 6,predicate := 44}
def policy : CurrentUse.Policy := ⟨24,1,0,.both (.leaf ⟨4,22⟩) (.leaf ⟨6,44⟩)⟩
def withMetadata (s : MixedManagementEntry.System 3 1) : MixedManagementEntry.System 3 1 :=
 let authority : CurrentUse.Authority :=
   { s.view.authority with
     issued := s.view.authority.issued ++ [claimA,claimB]
     epochs := fun issuer => if issuer = 6 then some 0 else s.view.authority.epochs issuer }
 { s with
   view := {s.view with authority := authority}
   controlPolicy := fun action => if action = 22 ∨ action = 23 then policy else s.controlPolicy action }

def key : Key := ⟨"player","self","hp"⟩
def other : Key := {key with entity := "other"}
def declaration : Declaration := ⟨"player","hp","S","Int",some "observer_safe"⟩
def attach (management : MixedManagementEntry.System 3 1) : Option (State 3 1) := do
 let moduleKey ← MixedCompositionCore.index management.configuration.count 1
 let ownerKey := WorldProjection.encode (a:=1) (n:=management.configuration.count) (.locus (0 : Fin 3))
 let encoded := WorldProjection.encode (a:=1) (p:=3) (.moduleSlot moduleKey)
 let entry := management.configuration.state.instances moduleKey
 return ⟨management,
  {instanceId := management.configuration.state.realm,ownerKey := ownerKey.val,
   ownerIdentity := ⟨.locus,management.configuration.state.placeIncarnation 0,0⟩,
   moduleKey := encoded.val,moduleIdentity := ⟨.module,1,entry.revision⟩,
   code := entry.definition.val,contract := entry.definition.val,
   ownerName := "S",schema := [declaration],serial := 0,slots := [],used := []}⟩
def initial := MixedManagementControls.bothBuilt.bind fun (s,_) => attach (withMetadata s)
def live := initial.bind fun s => perform s 0 0 7 4 (.activate key 3)
def retired := live.bind fun s => perform s 0 0 7 5 (.retire key)
def returned := retired.bind fun s => perform s 0 0 7 6 (.activate key 3)
#guard initial.isSome && live.isSome && retired.isSome && returned.isSome
#guard (live.map fun s => ((slot s.metadata key).generation,(slot s.metadata key).labelFloor)) = some (1,3)
#guard (retired.map fun s => current s.metadata key) = some none
#guard (returned.map fun s => ((slot s.metadata key).generation,(slot s.metadata key).labelFloor)) = some (3,3)
#guard (live.map fun s => (s.management.serial,s.metadata.serial,s.management.used.map CurrentUse.UseId.request)) = some (5,1,[4,3,2,1,0])
#guard (retired.bind fun s => perform s 0 0 7 6 (.activate key 0)).isNone
#guard (live.bind fun s => perform s 0 0 7 4 (.activate other 3)).isNone
#guard (live.bind fun s => perform s 0 0 7 5 (.activate other 3) |>.map fun s => current s.metadata key) = some (some ⟨declaration,1,3⟩)

def control (s : State 3 1) (requestId : Nat) (raw : MixedCompositionCore.Raw) := do
 let evidence ← MixedManagementEntry.authorize s.management 0 0 7 requestId raw
 manage s 0 0 7 requestId raw evidence
#guard (live.bind fun s => control s 4 (.retire 0)).isNone
#guard (live.bind fun s => control s 5 (.retire 0)).isSome
#guard (live.bind fun s => control s 5 (.retire 0) |>.bind fun (s,_) => perform s 0 0 7 5 (.activate other 3)).isNone
#guard (live.bind fun s => control s 5 (.retire 0) |>.bind fun (s,_) => perform s 0 0 7 6 (.activate other 3)).isSome

-- Module retirement leaves a historical record but removes current usability.
def bound : Binding := ⟨key,⟨declaration,1,3⟩⟩
def moduleRetired := live.bind fun s => control s 5 (.retire 1)
#guard (live.map fun s => OwnerMetadataManagement.usable s 0 bound) = some true
#guard (moduleRetired.map fun (s,_) => current s.metadata key) = some (some bound.metadata)
#guard (moduleRetired.map fun (s,_) => OwnerMetadataManagement.usable s 0 bound) = some false
#guard (moduleRetired.bind fun (s,_) => perform s 0 0 7 6 (.activate other 3)).isNone
#guard (returned.map fun s => OwnerMetadataManagement.usable s 0 bound) = some false

-- Dynamic unrelated addition must not invalidate existing module coordinates.
def added := live.bind fun s => control s 5 (.instantiate 0 7 [0] none .top)
#guard (added.map fun (s,_) => s.management.configuration.count) = some 3
#guard (added.map fun (s,_) => OwnerMetadataManagement.usable s 0 bound) = some true
#guard (added.bind fun (s,_) => perform s 0 0 7 6 (.activate other 3)).isSome

-- Evidence does not substitute for current two-layer authority or payload.
#guard (MixedManagementControls.bothBuilt.bind fun (s,_) => attach s |>.bind fun s => perform s 0 0 7 4 (.activate key 3)).isNone
#guard (initial.bind fun s => perform s 0 1 7 4 (.activate key 3)).isNone
#guard (initial.bind fun s => perform s 0 0 8 4 (.activate key 3)).isNone
#guard (initial.bind fun s => perform {s with metadata := {s.metadata with ownerName := "T"}} 0 0 7 4 (.activate key 3)).isNone
#guard (initial.bind fun s => perform {s with metadata := {s.metadata with schema := [declaration,declaration]}} 0 0 7 4 (.activate key 3)).isNone
#guard (initial.bind fun s => perform {s with metadata := {s.metadata with code := 999}} 0 0 7 4 (.activate key 3)).isNone
#guard (initial.bind fun s => perform s 0 0 7 4 (.activate {key with field := "missing"} 3)).isNone

def revoked (s : State 3 1) : State 3 1 := installAuthorityHead s
 { s.management.view with
   generation := s.management.view.generation + 1
   authority := {s.management.view.authority with revoked := [93]} }
#guard (initial.bind fun s => perform (revoked s) 0 0 7 4 (.activate key 3)).isNone
#guard (initial.bind fun s => perform (installAuthorityHead s {s.management.view with
 authority := {s.management.view.authority with issued := s.management.view.authority.issued.filter fun c => c.id != 93}})
 0 0 7 4 (.activate key 3)).isNone
#guard (initial.bind fun s => do
 let e ← authorize s 0 0 7 4 (.activate key 3)
 return check (revoked s) 0 0 7 4 (.activate key 3) e) = some false
#guard (initial.bind fun s => do
 let e ← authorize s 0 0 7 4 (.activate key 3)
 return check s 0 0 7 4 (.activate other 3) e) = some false
#guard (initial.bind fun s => do
 let e ← authorize s 0 0 7 4 (.activate key 3)
 return check {s with metadata := {s.metadata with ownerName := "T"}} 0 0 7 4 (.activate key 3) e) = some false
#guard (initial.bind fun s => do
 let e ← authorize s 0 0 7 4 (.activate key 3)
 let (next,_) ← control s 5 (.retire 0)
 return check next 0 0 7 4 (.activate key 3) e) = some false

-- Ordinary leave/join increments identity: restoring participation alone does
-- not revive metadata attached to an earlier locus incarnation.
def leftOwner := live.bind fun s => control s 5 (.leave 0)
#guard (leftOwner.map fun (s,_) => OwnerMetadataManagement.usable s 0 bound) = some false
-- Rejoin uses another participating actor locus and its explicit current policy.
def rejoined := leftOwner.bind fun (s,_) => do
 let e ← MixedManagementEntry.authorize s.management 0 1 7 6 (.join 0)
 manage s 0 1 7 6 (.join 0) e
#guard rejoined.isSome
#guard (rejoined.map fun (s,_) => s.management.configuration.state.participating 0) = some true
#guard (rejoined.map fun (s,_) => OwnerMetadataManagement.usable s 0 bound) = some false

-- Regression of the preserved pre-repair multiple-placement counterexample.
def multiBuilt := MixedManagementControls.ownerRegistered.bind fun (s,_) =>
 MixedManagementEntry.perform s 0 0 7 3 (.instantiate 1 7 [0,1] none .top)
def multiLive := multiBuilt.bind fun (s,_) => attach (withMetadata s) |>.bind fun s =>
 perform s 0 0 7 4 (.activate key 3)
def multiLeft := multiLive.bind fun s => control s 5 (.leave 0)
#guard multiLive.isSome && multiLeft.isSome
#guard (multiLive.map fun s => OwnerMetadataManagement.usable s 0 bound) = some true
#guard (multiLeft.map fun (s,_) => s.management.configuration.state.participating 0) = some false
#guard (multiLeft.map fun (s,_) => OwnerMetadataManagement.usable s 0 bound) = some false

-- Same admitted step relation, not a claimed E2E or schema authentication proof.
theorem performed_rooted (path : OwnerMetadataManagement.Rooted root s)
 (done : perform s member place principal requestId change = some result) : OwnerMetadataManagement.Rooted root result := by
 unfold perform at done
 cases issued : authorize s member place principal requestId change with
 | none => simp [issued] at done
 | some evidence => exact .metadata path (by simpa [issued] using done)

#print axioms performed_rooted
end MirroreaProofFirst.OwnerMetadataManagementControls
