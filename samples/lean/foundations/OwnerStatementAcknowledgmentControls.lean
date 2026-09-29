import OwnerStatementAcknowledgment
import OwnerStatementResultLifecycleControls
namespace MirroreaProofFirst.OwnerStatementAcknowledgmentControls
open OwnerStatementAcknowledgment OwnerStatementLiveCustodian OwnerStatementSourceCustodian
open OwnerStatementSourceCustodianControls OwnerStatementResultCollectionControls

def good := exactReported
#guard good.isSome
#guard (good.map fun (s,t) => check s t) = some true
#guard (good.bind fun (s,t) => consume s t 0 7).isSome
#guard (executed.map fun (s,t) => check s t) = some false
#guard (OwnerStatementResultLifecycleControls.oldReported.map fun (s,t) => check s t) = some false
#guard (OwnerStatementResultLifecycleControls.phaseOnlyFailure.map fun (s,t) => check s t) = some false
#guard (good.bind fun (s,t) => (consume s t 0 7).map fun next => check next t) = some false

-- Use the actual broader authority installer, not a forged receipt. The
-- restricted source-custodian Path excludes this held refresh; that scheduler
-- guard stays necessary. Raw lower history remains genuine across the change.
def changed := good.bind fun (s,t) =>
 (OwnerStatementRegistrySelection.authorityHead s.live (successor s)).map fun live => ({s with live := live},t)
#guard changed.isSome
#guard (changed.bind fun (s,t) => OwnerStatementResultCollection.collect s t |>.map fun r =>
 match r with | .committed w => w.value | _ => 0) = some 190
#guard (changed.map fun (s,t) => check s t) = some false
#guard (changed.bind fun (s,t) => consume s t 0 7).isNone
#guard (changed.map fun (s,_) => (s.live.session.state.owner.store 0,s.live.session.state.owner.history.length)) = some (some 190,1)
#guard (good.bind fun (s,_) => refreshAuthority s (successor s)).isNone

-- A checker that only authenticates historical outcome accepts the wrong cut.
def historyOnly := changed.bind fun (s,t) => OwnerStatementResultCollection.collect s t |>.map fun result =>
 match result with | .committed _ => true | .refused _ => false
#guard historyOnly = some true

-- Private constructor scope discriminator: the exact pending has already been
-- marked used while the historical result and wait are still retained.
def consumedId := good.map fun (s,t) =>
 let source := s.live.session.state.source
 let machine := source.machine
 let core := machine.store.core
 let core := {core with system := {core.system with used := MixedRequestCore.pendingId (.owner t.saved) :: core.system.used}}
 ({s with live := {s.live with session := {s.live.session with state := {s.live.session.state with
    source := {source with machine := {machine with store := {machine.store with core := core}}}}}}},t)
#guard (consumedId.bind fun (s,t) => OwnerStatementResultCollection.collect s t).isSome
#guard (consumedId.map fun (s,t) => check s t) = some false
#guard (consumedId.bind fun (s,t) => consume s t 0 7).isNone

-- Private constructor controls delimit Facts. check is a receipt-readiness
-- checker, not an independent admission/import or historical-origin checker.
def stopBinding (s : OwnerStatementLiveCustodian.State p a) : Bool :=
 match s.live.session.cursor.stopped,
   MixedOwnerSourceIssue.ownerWaiting s.live.session.state.source.waiting with
 | some (.write _ ordinal control statement),some waiting =>
   decide (waiting.statement = statement ∧ waiting.saved.origin =
     ⟨statement.site.document,statement.site.byteOffset,s.live.session.activation,ordinal,control⟩)
 | _,_ => false

def wrongStopped := good.map fun (s,t) =>
 let cursor := s.live.session.cursor
 let stopped : Option MixedOwnerProgram.Entry := cursor.stopped.map fun (entry : MixedOwnerProgram.Entry) => match entry with
   | .write name ordinal control statement => MixedOwnerProgram.Entry.write name (ordinal+1) control statement
   | other => other
 ({s with live := {s.live with session := {s.live.session with cursor := {cursor with stopped := stopped}}}},t)
#guard (good.map fun (s,_) => stopBinding s) = some true
#guard (wrongStopped.map fun (s,_) => stopBinding s) = some false
#guard (wrongStopped.map fun (s,t) => check s t) = some true
#guard (wrongStopped.bind fun (s,t) => consume s t 0 7).isSome

-- Erasing actual attempts/history cannot be disguised as an admitted path.
-- Readiness alone does not inspect those fields; original collection does.
def missingHistory := good.map fun (s,t) =>
 let owner := s.live.session.state.owner
 ({s with live := {s.live with session := {s.live.session with state := {s.live.session.state with
   owner := {owner with history := [],attempts := []}}}}},t)
#guard (missingHistory.map fun (s,t) => check s t) = some true
#guard (missingHistory.bind fun (s,t) => OwnerStatementResultCollection.collect s t).isNone
#guard (missingHistory.bind fun (s,t) => consume s t 0 7).isSome

-- A matching later reply never permits skipping the actual first inbox head.
def wrongHead := good.map fun (s,t) =>
 let state := s.live.session.state
 let inbox := match state.inbox with
   | [] => []
   | reply::rest => {reply with pending := {reply.pending with origin :=
       {reply.pending.origin with ordinal := reply.pending.origin.ordinal+1}}} :: reply :: rest
 ({s with live := {s.live with session := {s.live.session with state := {state with inbox := inbox}}}},t)
#guard (wrongHead.map fun (s,t) => check s t) = some false
#guard (wrongHead.bind fun (s,t) => consume s t 0 7).isNone

end MirroreaProofFirst.OwnerStatementAcknowledgmentControls
