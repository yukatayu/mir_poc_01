import MixedOwnerSourceDeclaration
import MixedOwnerSourceTraceControls
namespace MirroreaProofFirst.MixedOwnerSourceDeclarationControls
open MixedOwnerSourceIssue ReferenceSourceData

-- Explicit authority/field/label fixtures remain. Definition code and callable
-- bindings are now produced by actual source entries from an empty catalog.
-- No independent generated body, fake value, or post-hoc expected history.
def sourceInitial : State 3 1 := embed (MixedReferenceSource.initial 91
 MixedManagementControls.view (fun _ => ManagementEntry.Controls.policy))
def pureProgram : List Located :=
 [⟨⟨"source-construction.mir",10⟩,.plain (.register "pureDef" InstancePrograms.Controls.original none)⟩,
  ⟨⟨"source-construction.mir",20⟩,.plain (.instantiate "pure" "pureDef" [0] none .top)⟩]
def pureBuilt := MixedReferenceSource.run (base sourceInitial) 0 0 7 pureProgram
def declared := MixedOwnerSourceDeclaration.declareOwner (embed pureBuilt.state)
 MixedOwnerSourceIssueControls.fields MixedOwnerSourceIssueControls.labels 0 0 0 7 "ownerDef"
 MixedOwnerSourceIssueControls.statement
#guard pureBuilt.status = .ready
#guard lookup pureBuilt.state.values "amount" = none
#guard declared.status = .ready
#guard lookup declared.state.values "ownerDef" = some (.plain (.definition 1))
#guard declared.state.nextRequest = 3
#guard declared.state.writes.length = 3
#guard declared.state.machine.store.core.system.configuration.definitions = 2

def built := advancePure declared.state 0 0 7
 ⟨⟨"source-construction.mir",35⟩,.plain (.instantiate "generated_assignment" "ownerDef" [0] none .top)⟩
#guard built.status = .ready
#guard lookup built.state.values "generated_assignment" = some (.plain (.callable 1))
#guard built.state.machine.store.core.system.configuration.count = 2

def pureRequested := advancePure built.state 0 0 7 MixedOwnerSourceIssueControls.pureCall
def calculated := completePure pureRequested.state
#guard pureRequested.status = .waiting
#guard calculated.status = .ready
#guard lookup calculated.state.values "amount" = some (.plain (.integer 5 false))
def sourceReady := MixedOwnerSourceTrace.initial calculated.state MixedOwnerSourceTraceControls.initialStore
def issued := MixedOwnerSourceTrace.issue sourceReady MixedOwnerSourceIssueControls.fields
 MixedOwnerSourceIssueControls.labels 0 7 12 1 0 "generated_assignment" MixedOwnerSourceIssueControls.statement
def transferred := issued.bind MixedOwnerSourceTrace.transfer
def served := transferred.map (MixedOwnerSourceTrace.service (FallibleFlow.signed 63) MixedOwnerMaterializationControls.metadata)
def delivered := served.bind MixedOwnerSourceTrace.receive
#guard issued.isSome && delivered.isSome
#guard (delivered.map fun s => (s.owner.store 0,s.owner.history.length,s.source.nextRequest,s.source.writes.length)) = some (some 15,1,6,5)
#guard (issued.bind fun s => ownerWaiting s.source.waiting |>.map fun w =>
 (w.dependencies.map ReferenceSource.Read.producer,w.operationDependency.producer)) = some ([some 4],some 3)

-- Declaration is a real permission-bearing management operation. Static code
-- correctness cannot manufacture its required owner-management authority.
def revoked := {pureBuilt.state with machine := (MixedReferenceExecution.authorityHead pureBuilt.state.machine
  {MixedManagementControls.view with authority := {MixedManagementControls.view.authority with revoked := [82]}})}
def refused := MixedOwnerSourceDeclaration.declareOwner (embed revoked)
 MixedOwnerSourceIssueControls.fields MixedOwnerSourceIssueControls.labels 0 0 0 7 "ownerDef" MixedOwnerSourceIssueControls.statement
#guard refused.status = .failed .rejected
#guard lookup refused.state.values "ownerDef" = none
#guard refused.state.nextRequest = revoked.nextRequest
#guard refused.state.writes = revoked.writes
#guard refused.state.machine.store.core.system.configuration.definitions = 1
-- No silent redeclaration, and no registration through an outstanding wait.
#guard (MixedOwnerSourceDeclaration.declareOwner declared.state MixedOwnerSourceIssueControls.fields
 MixedOwnerSourceIssueControls.labels 0 0 0 7 "ownerDef" MixedOwnerSourceIssueControls.statement).status = .failed .rejected
#guard (MixedOwnerSourceDeclaration.declareOwner pureRequested.state MixedOwnerSourceIssueControls.fields
 MixedOwnerSourceIssueControls.labels 0 0 0 7 "otherDef" MixedOwnerSourceIssueControls.statement).status = .failed .rejected
end MirroreaProofFirst.MixedOwnerSourceDeclarationControls
