import MixedOwnerSourceIssue
import MixedExecutionControls
import MixedOwnerMaterializationControls
namespace MirroreaProofFirst.MixedOwnerSourceIssueControls
open MixedOwnerSourceIssue ReferenceSourceData

-- Exact admitted management/Execution fixture, with explicit fixture authority.
-- The generated operation binding is supplied here: this does not establish the
-- still-missing ordinary-source deployment/whole-Session bridge.
def before : Option (State 3 1) := MixedExecutionControls.built.map fun (machine,_) =>
 ⟨machine,[("pure",.plain (.callable 0)),("generated_assignment",.plain (.callable 1))],6,[],[],none⟩
def pureCall : Located := ⟨⟨"shared-source.mir",30⟩,.plain (.invoke "amount" "pure" (.integer 2))⟩
def requested := before.map fun s => advancePure s 0 0 7 pureCall
def calculated := requested.map fun r => completePure r.state
#guard (requested.map fun r => r.status) = some .waiting
#guard (calculated.map fun r => r.status) = some .ready
#guard (calculated.map fun r => lookup r.state.values "amount") = some (some (.plain (.integer 5 false)))

def fields : MixedNamedOwnerSource.Fields := ⟨[("health",0),("alias",0)],[(0,(0,0))]⟩
def labels := [("amount",0)]
def statement : MixedNamedOwnerSource.Assignment :=
 ⟨⟨"shared-source.mir",40⟩,"health",.add (.state "alias") (.parameter "amount")⟩
def issued := calculated.bind fun r => issue r.state fields labels 0 7 12 1 0 "generated_assignment" statement
#guard issued.isSome
#guard (issued.map fun (s,w) => decide (s.waiting = some (.inr w)) &&
 decide (s.machine.store.core.pending = [.owner w.saved]) && s.machine.pending.isEmpty) = some true
#guard (issued.map fun (s,_) => s.nextRequest) = some 8
#guard (issued.map fun (_,w) => w.saved.request.arguments) = some [.integer 5]
#guard (issued.map fun (_,w) => w.saved.origin) = some ⟨"shared-source.mir",40,12,1,0⟩
#guard (issued.map fun (_,w) => w.dependencies.map ReferenceSource.Read.name) = some ["amount"]
#guard (issued.map fun (_,w) => w.operationDependency.name) = some "generated_assignment"
#guard (issued.map fun (s,_) => (advancePure s 0 0 7 pureCall).status) = some (.failed .awaiting)
#guard (issued.map fun (s,_) => (completePure s).status) = some (.failed .awaiting)
#guard (issued.bind fun (s,_) => issue s fields labels 0 7 12 1 0 "generated_assignment" statement).isNone
-- Registration has add's complete contract, so a different body, same body with
-- a different flow contract, and a pure-kind binding cannot select it.
#guard (calculated.bind fun r => issue r.state fields labels 0 7 12 1 0 "generated_assignment"
 {statement with rhs := .integer 1}).isNone
#guard (calculated.bind fun r => issue r.state {fields with metadata := [(0,(0,1))]} [("amount",1)]
 0 7 12 1 1 "generated_assignment" statement).isNone
#guard (calculated.bind fun r => issue r.state fields labels 0 7 12 1 0 "pure" statement).isNone
#guard (calculated.bind fun r => issue r.state fields [("amount",1)] 0 7 12 1 0 "generated_assignment" statement).isNone
#guard (calculated.bind fun r => issue r.state fields [] 0 7 12 1 0 "generated_assignment" statement).isNone
#guard (calculated.bind fun r => issue r.state fields labels 0 7 12 1 1 "generated_assignment" statement).isNone
#guard (calculated.bind fun r => issue {r.state with values := [("amount",.plain (.integer 9223372036854775808 false)),
 ("generated_assignment",.plain (.callable 1))]} fields labels 0 7 12 1 0 "generated_assignment" statement).isNone
-- No caller-supplied captured payload parameter exists: the actual five is read
-- from the current source value, and the earlier source result/write is retained.
#guard (issued.map fun (s,_) => s.writes.length) = some 1
#guard (issued.map fun (_,w) => w.saved.body) = some MixedCatalogControls.add.body
-- Reproduce the review's counterexamples at the EXACT lower boundary. These
-- witnesses do not claim an admitted source-origin forgery or restore exploit.
def twice := MixedOwnerMaterializationControls.issued.map fun e =>
 let first := MixedOwnerMaterialization.serve (FallibleFlow.signed 63) MixedCatalogControls.installed
   MixedCatalogControls.view MixedOwnerMaterializationControls.metadata MixedCatalogControls.initial e
 MixedOwnerMaterialization.serve (FallibleFlow.signed 63) MixedCatalogControls.installed
   MixedCatalogControls.view MixedOwnerMaterializationControls.metadata first.1 e
#guard (twice.map fun r => r.1.store 0) = some (some 20)
#guard (twice.map fun r => r.1.history.length) = some 2
#guard (MixedOwnerMaterializationControls.issued.map fun e =>
 (MixedOwnerMaterialization.serve (FallibleFlow.signed 63) MixedCatalogControls.installed MixedCatalogControls.view
 MixedOwnerMaterializationControls.metadata MixedCatalogControls.initial
 {e with origin := {e.origin with controlLabel := 1}}).1.store 0) = some (some 15)

def ambiguous : MixedNamedOwnerSource.Fields := ⟨[("x",0),("x",1)],[(0,(0,0)),(1,(0,0))]⟩
def ambiguousAssignment : MixedNamedOwnerSource.Assignment := ⟨⟨"raw-context",0⟩,"x",.integer 0⟩
#guard (MixedOwnerSourceFootprint.elaborate ambiguous [] [] 0 ambiguousAssignment |>.map fun p => p.operation.body.target) = some 0
#guard (MixedOwnerSourceFootprint.elaborate {ambiguous with names := ambiguous.names.reverse} [] [] 0 ambiguousAssignment |>.map fun p => p.operation.body.target) = some 1
-- No claim that these supplied name/label tables are authenticated lexical bindings.
end MirroreaProofFirst.MixedOwnerSourceIssueControls
