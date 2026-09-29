import MixedCatalogService
namespace MirroreaProofFirst.MixedCatalogControls
open CurrentUse WorldProjection MixedInstanceState MixedCatalogService
open MixedOperationDefinitions (OwnerDefinition)

-- Explicit LAB grants. No source/definition checker constructs them. Structural
-- registration here is a probe; authorized management/source construction follows.
def add : OwnerDefinition :=
 {MixedOperationDefinitions.Controls.add with contract := ⟨[(0,(0,0))],[0],0⟩}
def old := MixedCatalogEmbedding.state InstanceState.Controls.two
def registered := MixedInstanceState.register old (.owner add) none
def installed := MixedInstanceState.instantiate registered 2 7 [0] none .top
#guard MixedInstanceState.registrationCheck old (.owner add) none
#guard !MixedInstanceState.registrationCheck old (.owner add) (some 0)
#guard !MixedInstanceState.replacementCheck installed 0 2
#guard !MixedInstanceState.replacementCheck installed 2 0

def claim : Claim := {InvocationBoundary.Controls.invocationClaim with id := 81,actions := [16],targets := List.range 32}
def view : AuthorityView 1 :=
 {InvocationBoundary.Controls.view with authority :=
   {InvocationBoundary.Controls.view.authority with issued :=
      [{InvocationBoundary.Controls.invocationClaim with targets := List.range 32},claim]}}
def withoutOwner : AuthorityView 1 :=
 {view with authority := {view.authority with issued :=
   [{InvocationBoundary.Controls.invocationClaim with targets := List.range 32}]}}
def current := MixedCatalogService.world installed view

def handle (slot : Slot 1 3 3) : Handle (size 1 3 3) :=
 ⟨91,encode slot,(MixedCatalogService.slotRecord installed view slot).identity⟩
def pureValue : Option Int :=
 match operationDefinition installed 0 with
 | .pure d => InstancePrograms.Machine.run d.code 2
 | .owner _ => none
#guard pureValue = some 5

def pending (v : AuthorityView 1) (key : Fin 3) (body : OwnerEffectService.Body) :
 Option (OwnerEffectService.Pending (size 1 3 3)) := do
 let value ← pureValue
 let request : UseRequest (size 1 3 3) :=
  ⟨7,handle (.member 0),handle (.locus 0),handle (.moduleSlot key),handle (.operation key 0),91,[.integer value]⟩
 let w := MixedCatalogService.world installed v
 let e ← authorize w.authority (w.policies request.operation.key) (currentContext w request)
 if checkUse w request e then some ⟨⟨"mixed-catalog-core-control",40,12,1,0⟩,request,e,body⟩ else none

def initial : OwnerEffectService.Owner (size 1 3 3) := ⟨fun k => if k=0 then some 10 else none,[]⟩
def effectResult := (pending view 2 add.body).map fun p =>
 MixedCatalogService.serve (FallibleFlow.signed 63) installed view initial p
#guard (effectResult.map fun r => r.1.store 0) = some (some 15)
#guard (effectResult.map fun r => r.1.history.length) = some 1
#guard (effectResult.map fun r => r.1.history.map (fun row => row.reads)) = some [[(0,10)]]
#guard (pending withoutOwner 2 add.body).isNone

-- Same realm and CURRENT pure permission. Raw external registry substitution
-- can cause a write under the old helper; the single tagged catalog rejects it.
def wrongKind := pending view 0 add.body
#guard wrongKind.isSome
#guard (wrongKind.map fun p => CurrentUse.checkUse current p.request p.original) = some true
#guard (wrongKind.map fun p => (OwnerEffectService.serve (FallibleFlow.signed 63) current
  (fun _ => some add.body) initial p).1.store 0) = some (some 15)
#guard (wrongKind.map fun p => (MixedCatalogService.serve (FallibleFlow.signed 63) installed view initial p).2) =
 some (.refused .code)
#guard (wrongKind.map fun p => (MixedCatalogService.serve (FallibleFlow.signed 63) installed view initial p).1.history) = some []
#guard (wrongKind.map fun p => (MixedCatalogService.serve (FallibleFlow.signed 63) installed view initial p).1.store 0) = some (some 10)

-- Code/body substitution remains rejected even with actual owner-use authority.
#guard ((pending view 2 {add.body with tree := .integer 99}).map fun p =>
 (MixedCatalogService.serve (FallibleFlow.signed 63) installed view initial p).2) = some (.refused .code)
def revoked : AuthorityView 1 := {view with generation := 1,authority := {view.authority with revoked := [81]}}
#guard ((pending view 2 add.body).map fun p =>
 (MixedCatalogService.serve (FallibleFlow.signed 63) installed revoked initial p).2) = some (.refused .authority)

-- Declaration owner means locus, distinct from operation's principal/instance owner.
#guard located (.owner add) (0 : Fin 3)
#guard !located (.owner add) (1 : Fin 3)
end MirroreaProofFirst.MixedCatalogControls
