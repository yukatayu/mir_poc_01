import OwnerStatementResultCollection
import OwnerStatementSourceCustodianControls
namespace MirroreaProofFirst.OwnerStatementResultCollectionControls
open OwnerStatementLiveCustodian OwnerStatementSourceCustodian OwnerStatementSourceCustodianControls
open OwnerStatementResultCollection

def actual := executed.bind fun (s,ticket) => collect s ticket
#guard actual.isSome
#guard (actual.map fun result => match result with | .committed written => written.value | .refused _ => 0) = some 190
#guard (staged.bind fun (s,ticket) => collect s ticket).isNone
#guard (executed.bind fun (s,ticket) => collect s {ticket with designation := ticket.designation+1}).isNone
#guard (executed.bind fun (s,ticket) => collect s {ticket with saved := {ticket.saved with body := {ticket.saved.body with target := ticket.saved.body.target+1}}}).isNone

def ambiguated := executed.map fun (s,ticket) =>
 let owner := s.live.session.state.owner
 ({s with live := {s.live with session := {s.live.session with state := {s.live.session.state with owner := {owner with attempts := owner.attempts ++ owner.attempts}}}}},ticket)
#guard (ambiguated.bind fun (s,ticket) => collect s ticket).isNone

def exactReported := executed.bind fun (s,ticket) => (reportActual s ticket).map fun next => (next,ticket)
#guard exactReported.isSome
#guard (exactReported.map fun (s,_) => (s.live.session.state.owner.store 0,count s.live.session,s.dispatched.length)) = some (some 190,0,1)
#guard (exactReported.bind fun (s,ticket) => collect s ticket) = actual
#guard (exactReported.bind fun (s,ticket) => reportActual s ticket).isNone
#guard (exactReported.bind fun (s,ticket) => consume s ticket 0 7).isSome
#guard (exactReported.bind fun (s,ticket) => (consume s ticket 0 7).bind fun next => collect next ticket).isNone

def refused := staged.bind fun (s,ticket) => (execute s ticket (FallibleFlow.signed 2)).map fun next => (next,ticket)
#guard refused.isSome
#guard (refused.map fun (s,_) => s.live.session.state.owner.store 0) = some (some 200)
#guard (refused.bind fun (s,ticket) => collect s ticket |>.map fun result => match result with | .refused _ => true | .committed _ => false) = some true
#guard (refused.bind fun (s,ticket) => reportActual s ticket).isNone
#guard (refused.bind fun (s,ticket) => consume s ticket 0 7).isNone
end MirroreaProofFirst.OwnerStatementResultCollectionControls


namespace MirroreaProofFirst.OwnerStatementResultCollectionControls.Projection
open OwnerStatementResultCollection.Projection
-- Fixed controls only; general statements and original advance connection are
-- in the imported module. No finite decide result is counted as a theorem.
def refusedDelivery := refused.bind fun (s,ticket) =>
 (OwnerStatementResultCollection.collect s ticket).map fun answer => deliver answer (none : Option Nat)
def projectedDelivery := refused.bind fun (s,ticket) =>
 (OwnerStatementResultCollection.collect s ticket).map fun answer => deliver answer (some 7)
def successfulUnreported := actual.map fun answer => deliver answer (none : Option Nat)
def successfulReported := actual.map fun answer => deliver answer (some 7)
#guard (refusedDelivery.bind failure).isSome
#guard (projectedDelivery.bind failure) = (refusedDelivery.bind failure)
#guard successfulUnreported = some .unavailable
#guard (successfulReported.map fun r => match r with | .committed _ _ => true | _ => false) = some true
#guard (refusedDelivery.map fun r => match r with | .committed _ _ => true | _ => false) = some false
end MirroreaProofFirst.OwnerStatementResultCollectionControls.Projection
