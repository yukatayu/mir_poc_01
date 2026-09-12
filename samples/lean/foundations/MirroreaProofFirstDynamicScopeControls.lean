import MirroreaProofFirstDynamicIdentity
import MirroreaProofFirstSupportImpact
import MirroreaProofFirstCurrentChoice

namespace MirroreaProofFirst.EntryReviewControls
open Support Growth CurrentUse GraphValidation LocalImpact

-- Oracle entry findings reproduced as finite counterexamples. These controls
-- are not general theorems and do not close source/catalog realization.
def dead : Snapshot 1 := ⟨fun _ => .bottom,fun _ => true⟩
def moreDead := append dead (m:=1) (fun _ => .bottom) (fun _ => true)
#guard snapshotRank dead 0 = 0
#guard snapshotRank moreDead 0 = 1
#guard !snapshotLive dead 0 && !snapshotLive moreDead 0
#guard checkSnapshot moreDead (fun _ => false) (fun _ => 0)

def ident (inc : Nat) : RecordIdentity := ⟨.operation,inc,0⟩
def old : World 1 :=
  { instanceId := 9, generation := 0,
    support := ⟨fun _ => .top,fun _ => true⟩,
    records := fun _ => ⟨ident 7,0,0,0,0,10,20⟩,
    authority := ⟨[],[],fun _ => none⟩,
    policies := fun _ => ⟨0,0,0,.leaf ⟨0,0⟩⟩ }
def permuted : World 2 :=
  { instanceId := 9, generation := 0,
    support := ⟨fun _ => .top,fun _ => true⟩,
    records := fun k => ⟨ident (if k=0 then 8 else 7),0,1,1,0,10,20⟩,
    authority := old.authority, policies := fun _ => old.policies 0 }
#guard checkHandle old .operation ⟨9,0,ident 7⟩
#guard checkHandle permuted .operation ⟨9,1,ident 7⟩
#guard !checkHandle permuted .operation ⟨9,0,ident 7⟩
#guard snapshotLive old.support 0 && snapshotLive permuted.support 0 && snapshotLive permuted.support 1

-- A complete scan of the OLD catalog says nothing about a newly introduced row.
def oldGraph : Fin 3 → Fin 3 → Bool := fun _ _ => false
def enlargedGraph (x y : Fin 4) : Bool := decide (x=3 ∧ y=3)
#guard checkAcyclic oldGraph
#guard !checkAcyclic enlargedGraph

-- Inflationary iteration must restart from bottom after eligibility withdrawal.
def staleStep (s : Snapshot n) (prior : Fin n → Bool) (k : Fin n) : Bool :=
  prior k || (s.eligible k && eval prior (s.forms k))
def root : Snapshot 1 := ⟨fun _ => .top,fun _ => true⟩
def retired : Snapshot 1 := {root with eligible := fun _ => false}
#guard !snapshotLive retired 0
#guard staleStep retired (snapshotLive root) 0

def alternatives (k : Fin 3) : Formula (Fin 3) :=
  if k=0 then .either (.ref 1) (.ref 2) else .top
def missing : Snapshot 3 := ⟨alternatives,fun k => k==0⟩
def enabled : Snapshot 3 := ⟨alternatives,fun k => k != 1⟩
#guard !snapshotLive missing 0
#guard snapshotLive enabled 0
#guard affected alternatives (fun k => k==2) 0

namespace KernelReview
open CurrentUse.Controls
-- Already issued grants may mention not-yet-allocated numeric target slots.
def preGranted : World 4 := {world with authority :=
  {auth with issued := [{claimA with targets := [3,4]},{claimB with targets := [3,4]}]}}
def withTarget := IdentityGrowth.grow preGranted IdentityGrowth.Controls.more
#guard (CurrentChoice.offer withTarget IdentityGrowth.Controls.freshRequest).isSome

-- Fresh search can replace an invalidated branch; it is not saved-witness validation.
def eitherPolicy : Policy := {policy with expression := .either (.leaf ⟨10,11⟩) (.leaf ⟨20,22⟩)}
def eitherWorld : World 4 := {world with policies := fun _ => eitherPolicy}
def leftEvidence : Evidence := {evidence with witness := .left (.leaf claimA)}
def revokedLeft : World 4 := {eitherWorld with authority := {auth with revoked := [1]}}
def one : CurrentChoice.Cursor 4 := ⟨[request],0,12⟩
#guard checkUse eitherWorld request leftEvidence
#guard !checkUse revokedLeft request leftEvidence
#guard (CurrentChoice.offer revokedLeft request).isSome
#guard (CurrentChoice.resolve revokedLeft (CurrentChoice.resolve eitherWorld one).1).1.position = 0
#guard (CurrentChoice.resolve revokedLeft (CurrentChoice.resolve eitherWorld one).1).1.lineage = 12

-- Context omits module revision. Retaining exact original handles remains required.
def revisedRecord (k : Fin 4) : Record 4 :=
  if k = 2 then {world.records k with identity := ⟨.module,1,1⟩} else world.records k
def revisedModule : World 4 := {world with records := revisedRecord}
def refreshedRequest : UseRequest 4 := {request with moduleHandle := ⟨7,2,(revisedModule.records 2).identity⟩}
#guard !checkUse revisedModule request evidence
#guard currentContext revisedModule refreshedRequest = currentContext world request
#guard checkUse revisedModule refreshedRequest evidence
end KernelReview

end MirroreaProofFirst.EntryReviewControls
