import MixedOwnerMaterialization
import MixedCatalogControls
namespace MirroreaProofFirst.MixedOwnerMaterializationControls
open MixedOwnerMaterialization MixedCatalogControls

def metadata (key : Nat) : Option (Nat × Nat) := if key=0 then some (0,0) else none
def issued := pending view 2 add.body
def guarded := issued.map fun e => serve (FallibleFlow.signed 63) installed view metadata initial e
#guard (guarded.map fun r => r.1.store 0) = some (some 15)
#guard (guarded.map fun r => r.1.history.length) = some 1
#guard (guarded.map fun r => r.2.isSome) = some true
#guard (issued.map fun e => (serve (FallibleFlow.signed 63) installed view (fun _ => none) initial e).2) = some none
#guard (issued.map fun e => (serve (FallibleFlow.signed 63) installed view (fun _ => some (0,1)) initial e).2) = some none
#guard (issued.map fun e => (serve (FallibleFlow.signed 63) installed view (fun _ => some (1,0)) initial e).2) = some none
#guard (issued.map fun e => (serve (FallibleFlow.signed 63) installed revoked metadata initial e).2) = some (some (.refused .authority))
#guard (issued.map fun e => (serve (FallibleFlow.signed 63) installed revoked metadata initial e).1.history.length) = some 0
-- The current field gate and the old CurrentUse gate are independently needed.
#guard (issued.map fun e => readyCheck add metadata initial e) = some true
#guard (issued.map fun e => (serve (FallibleFlow.signed 63) installed view metadata initial
 {e with request := {e.request with arguments := [.boolean true]}}).2) = some none
#guard (issued.map fun e => (serve (FallibleFlow.signed 63) installed view metadata initial
 {e with request := {e.request with arguments := [.integer 9223372036854775808]}}).2) = some none
#guard (issued.map fun e => (serve (FallibleFlow.signed 63) installed view metadata initial
 {e with request := {e.request with arguments := [.integer 5,.integer 1]}}).2) = some none

def constant := {add with body := {add.body with tree := .integer 5},contract := {add.contract with arguments := []}}
def constantCatalog := {installed with definitions := fun k => if k=2 then .owner constant else installed.definitions k}
def constantPending := issued.map fun e => {e with
 body := constant.body
 request := {e.request with arguments := []}
 original := {e.original with context := {e.original.context with arguments := []}}}
def absent : OwnerEffectService.Owner (WorldProjection.size 1 3 3) := ⟨fun _ => none,[]⟩
-- Raw helper inserts a missing target; guarded service refuses it unchanged.
-- This is an explicit mathematical comparison, not a forged production entry.
#guard (constantPending.map fun e => (MixedCatalogService.serve (FallibleFlow.signed 63)
 constantCatalog view absent e).1.store 0) = some (some 5)
#guard (constantPending.map fun e => (serve (FallibleFlow.signed 63)
 constantCatalog view metadata absent e).1.store 0) = some none
#guard (constantPending.map fun e => (serve (FallibleFlow.signed 63)
 constantCatalog view metadata absent e).2) = some none
#guard (constantPending.map fun e => (serve (FallibleFlow.signed 63)
 constantCatalog view metadata initial e).1.store 0) = some (some 5)

open MixedNamedOwnerSource in
#guard (MixedNamedOwnerSource.elaborate MixedNamedOwnerSource.Controls.ctx
 [("amount",.plain (.integer 9223372036854775808 false))] MixedNamedOwnerSource.Controls.labels 0
 MixedNamedOwnerSource.Controls.source).isSome
#guard ((MixedNamedOwnerSource.elaborate MixedNamedOwnerSource.Controls.ctx
 [("amount",.plain (.integer 9223372036854775808 false))] MixedNamedOwnerSource.Controls.labels 0
 MixedNamedOwnerSource.Controls.source).map sourcePayloadCheck) = some false
end MirroreaProofFirst.MixedOwnerMaterializationControls
