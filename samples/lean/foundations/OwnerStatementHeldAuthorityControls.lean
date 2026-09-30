import OwnerStatementHeldAuthority
import OwnerStatementAcknowledgmentControls
namespace MirroreaProofFirst.OwnerStatementHeldAuthorityControls
open OwnerStatementLiveCustodian OwnerStatementSourceCustodian
open OwnerStatementSourceCustodianControls OwnerStatementResultCollectionControls
open OwnerStatementHeldAuthority

def changed (input : Option (State 3 1 × Ticket)) := input.bind fun (s,t) =>
 (refresh s (successor s)).map fun next => (next,t)
def queued := changed staged
def done := changed executed
def delivered := changed exactReported
#guard queued.isSome && done.isSome && delivered.isSome
#guard (issued.bind fun s => refresh s (successor s)).isSome
#guard (staged.bind fun (s,_) => refresh s s.live.session.state.source.machine.store.core.system.view).isNone
#guard (done.map fun (s,_) => (s.live.session.state.owner.store 0,s.live.session.state.owner.history.length,count s.live.session)) = some (some 190,1,0)
#guard (done.bind fun (s,t) => OwnerStatementResultCollection.collect s t |>.map fun r => match r with | .committed w => w.value | _ => 0) = some 190
#guard (done.bind fun (s,t) => OwnerStatementResultCollection.reportActual s t).isSome
#guard (delivered.map fun (s,t) => OwnerStatementAcknowledgment.check s t) = some false
#guard (delivered.bind fun (s,t) => consume s t 0 7).isNone
#guard (exactReported.bind fun (s,t) => consume s t 0 7).isSome
#guard (delivered.bind fun (s,_) => invokeAgain s).isNone
def revokedDone := executed.bind fun (s,t) => (refresh s (revokeAll s)).map fun next => (next,t)
#guard revokedDone.isSome
#guard (revokedDone.bind fun (s,t) => OwnerStatementResultCollection.collect s t |>.map fun r => match r with | .committed w => w.value | _ => 0) = some 190
#guard (revokedDone.bind fun (s,t) => (OwnerStatementResultCollection.reportActual s t).bind fun next => consume next t 0 7).isNone
end MirroreaProofFirst.OwnerStatementHeldAuthorityControls
