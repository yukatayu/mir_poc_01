import MixedManagementControls
namespace MirroreaProofFirst.MixedCatalogSuccessorControls
open MixedManagementEntry MixedManagementControls MixedOperationDefinitions

-- Successor of retained predecessor countermodels. Legacy claims no longer
-- retire/reparent owner instances; explicit independent20/21 grants are needed.
def legacyRetires := bothBuilt.bind fun (s,_) => perform
 {s with view := {s.view with authority := {s.view.authority with revoked := [81,82]}}}
 0 0 7 4 (.retire 1)
def legacyReparents := bothBuilt.bind fun (s,_) => perform
 {s with view := {s.view with authority := {s.view.authority with revoked := [81,82]}}}
 0 0 7 4 (.reparent 1 (some 0))
#guard legacyRetires.isNone
#guard legacyReparents.isNone

#guard (bothBuilt.bind fun (s,_) => perform s 0 0 7 4 (.retire 1)).isSome
#guard (bothBuilt.bind fun (s,_) => perform s 0 0 7 4 (.reparent 1 (some 0))).isSome
#guard (bothBuilt.bind fun (s,_) => perform
 {s with view := {s.view with authority := {s.view.authority with revoked := [81,82]}}}
 0 0 7 4 (.retire 0)).isSome

def other : MixedCompositionCore.Raw := .register
 (.owner {MixedCatalogControls.add with body := ⟨0,.integer 99⟩}) none
-- Broad action17 already grants either body. Equality checks do NOT sign or
-- authenticate earlier issuance; a source carrier must bind that historical fact.
def rebound := (authorize initial 0 0 7 0 ownerCode).bind fun e =>
 commit initial 0 0 7 0 other {e with context := current initial 0 0 7 0 other}
#guard rebound.isSome

-- Different independently valid branches can have the SAME serial/coordinates.
def leftBranch := perform initial 0 0 7 0 ownerCode
def rightBranch := perform initial 0 0 7 0 other
def crossBranch := do
 let (left,_) ← leftBranch
 let (right,_) ← rightBranch
 let raw : MixedCompositionCore.Raw := .instantiate 0 7 [0] none .top
 let e ← authorize left 0 0 7 1 raw
 commit right 0 0 7 1 raw e
#guard crossBranch.isSome

def envelope : OwnerContract := ⟨[(0,(0,0)),(1,(1,0))],[],0⟩
def leftBody : OwnerDefinition := ⟨⟨0,.integer 0⟩,envelope⟩
def rightBody : OwnerDefinition := ⟨⟨1,.integer 0⟩,envelope⟩
#guard ownerCheck leftBody && ownerCheck rightBody
#guard exchangeCheck (.owner envelope) (.owner rightBody)
#guard MixedCatalogService.located (.owner leftBody) (0 : Fin 2)
#guard !MixedCatalogService.located (.owner rightBody) (0 : Fin 2)

def overflow : OwnerDefinition :=
 ⟨⟨0,.add (.integer (2^63-1)) (.integer 1)⟩,⟨[(0,(0,0))],[],0⟩⟩
#guard ownerCheck overflow
#guard OwnerCheckedArithmetic.evaluate (FallibleFlow.signed 63) (fun (_ : Nat) => none)
 (fun (_ : Nat) => none) overflow.body.tree = none

-- Raw malformed state can serve without Valid: literal/IFC/store claims require
-- admitted provenance and cannot be inferred from the body-tag lemma alone.
def malformedDefinition (k : Fin 3) : Definition :=
 if k.val = 2 then .owner {MixedCatalogControls.add with contract := ⟨[],[0],0⟩}
 else MixedCatalogControls.installed.definitions k
def malformed := {MixedCatalogControls.installed with definitions := malformedDefinition}

#guard !MixedInstanceState.registrationCheck MixedCatalogControls.old
 (.owner {MixedCatalogControls.add with contract := ⟨[],[0],0⟩}) none
#guard ((MixedCatalogControls.pending MixedCatalogControls.view 2 MixedCatalogControls.add.body).map fun request =>
 (MixedCatalogService.serve (FallibleFlow.signed 63) malformed MixedCatalogControls.view MixedCatalogControls.initial request).1.store 0) = some (some 15)

end MirroreaProofFirst.MixedCatalogSuccessorControls
