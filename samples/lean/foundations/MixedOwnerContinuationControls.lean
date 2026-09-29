import MixedOwnerContinuation
import MixedOwnerSourceDeclarationControls
namespace MirroreaProofFirst.MixedOwnerContinuationControls
open ReferenceSourceData MixedOwnerProgram MixedOwnerContinuation

def program : Program 3 :=
 ⟨0,MixedOwnerSourceIssueControls.fields,MixedOwnerSourceIssueControls.labels,0,
   MixedOwnerSourceDeclarationControls.pureProgram.map Item.ordinary ++
    [.ordinary MixedOwnerSourceIssueControls.pureCall,
     .assignment MixedOwnerSourceIssueControls.statement,
     .ordinary ⟨⟨"shared-source.mir",50⟩,.plain (.invoke "later" "pure" (.integer 1))⟩]⟩
def begun := launch 91 MixedManagementControls.view (fun _ => ManagementEntry.Controls.policy)
 MixedOwnerSourceTraceControls.initialStore program
#guard (MixedOwnerProgram.compile [] program).isSome
#guard (begun.map fun s => (s.program.items.length,s.entries.length,s.activation)) = some (5,7,0)
def waiting := begun.map fun s => drive 7 s 0 7
#guard (waiting.map fun s => s.status) = some .waiting
#guard (waiting.map fun s => (s.cursor.completed.length,s.cursor.remaining.length,s.cursor.stopped.isSome)) = some (5,1,true)
#guard (waiting.map fun s => lookup s.state.source.values "amount") = some (some (.plain (.integer 5 false)))
#guard (waiting.map fun s => s.state.source.nextRequest) = some 6
#guard (waiting.bind fun s => MixedOwnerSourceIssue.ownerWaiting s.state.source.waiting |>.map fun w =>
 (w.saved.origin.activation,w.saved.origin.ordinal,w.operationDependency.producer,w.dependencies.map ReferenceSource.Read.producer)) = some (0,3,some 4,[some 2])
-- A tick cannot invent the missing service or acknowledgment.
#guard (waiting.map fun s => (tick s 0 7).status) = some .waiting
#guard (waiting.map fun s => (tick s 0 7).cursor) = waiting.map Session.cursor
#guard (waiting.map fun s => (tick s 0 7).state.owner.history.length) = some 0

def transferred := waiting.bind transfer
def served := transferred.map fun s => service s (FallibleFlow.signed 63) MixedOwnerMaterializationControls.metadata
def acknowledged := served.map fun s => tick s 0 7
def finished := acknowledged.map fun s => drive 2 s 0 7
#guard served.isSome && acknowledged.isSome && finished.isSome
#guard (served.map fun s => (s.state.owner.store 0,s.state.owner.history.length,s.state.inbox.length)) = some (some 15,1,1)
#guard (acknowledged.map fun s => s.status) = some .ready
#guard (finished.map fun s => (s.status,s.cursor.completed.length,s.cursor.remaining.length,s.cursor.stopped.isNone)) = some (.ready,7,0,true)
#guard (finished.map fun s => lookup s.state.source.values "later") = some (some (.plain (.integer 2 false)))
#guard (finished.map fun s => (s.state.source.nextRequest,s.state.source.writes.length,s.state.owner.history.length)) = some (7,6,1)

def nextProgram : Program 3 := {program with items := [.assignment MixedOwnerSourceIssueControls.statement]}
def continued := finished.bind fun s => continueWith s nextProgram
def waitingAgain := continued.map fun s => drive 3 s 0 7
def servedAgain := (waitingAgain.bind transfer).map fun s => service s (FallibleFlow.signed 63) MixedOwnerMaterializationControls.metadata
def finishedAgain := servedAgain.map fun s => tick s 0 7
#guard (continued.map fun s => (s.activation,s.superseded.length,s.superseded.head?.map fun h => h.program.items.length)) = some (1,1,some 5)
#guard (finishedAgain.map fun s => (s.status,s.state.owner.store 0,s.state.owner.history.length,s.state.owner.attempts.length)) = some (.ready,some 20,2,2)
#guard (waitingAgain.bind fun s => MixedOwnerSourceIssue.ownerWaiting s.state.source.waiting |>.map fun w =>
 (w.saved.origin.activation,w.saved.origin.ordinal)) = some (1,0)
#guard (waiting.bind fun s => continueWith s nextProgram).isNone

-- Commit precedes head change and denied acknowledgment. Neither the actual
-- write nor the stopped source program disappears; adoption cannot erase wait.
def changed := served.bind fun s => authorityHead s
 {s.state.source.machine.store.core.system.view with generation := s.state.source.machine.store.core.system.view.generation+1}
def denied := changed.map fun s => tick s 0 7
#guard (denied.map fun s => s.status) = some (.failed .rejected)
#guard (denied.map fun s => (s.state.owner.store 0,s.state.owner.history.length,s.cursor.stopped.isSome,s.cursor.remaining.length,s.state.source.waiting.isSome)) = some (some 15,1,true,1,true)
#guard (denied.bind fun s => replaceResidual s nextProgram).isNone
#guard (denied.map fun s => (service s (FallibleFlow.signed 63) MixedOwnerMaterializationControls.metadata).state.owner.history.length) = some 1
-- Capture type/name, placement and information-flow errors are before launch.
#guard (MixedOwnerProgram.compile [] {program with items := [.assignment MixedOwnerSourceIssueControls.statement]}).isNone
#guard (MixedOwnerProgram.compile [] {program with labels := [("amount",1)]}).isNone
#guard (MixedOwnerProgram.compile [] {program with fields := {program.fields with metadata := [(0,(8,0))]}}).isNone
end MirroreaProofFirst.MixedOwnerContinuationControls
