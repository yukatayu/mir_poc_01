import MirroreaProofFirstReceivedResult
import ActualReference

namespace MirroreaProofFirst.ReceivedResultControls
open ReceivedResult

def main : IO Unit := do
  let waiting := ReferenceContinuation.drive 9 _root_.begun 0 7
  let some saved := waiting.state.waiting | throw (IO.userError "no actual-source pending request")
  let some value := InvocationBoundary.execute saved.entry.ticket |
    throw (IO.userError "reference owner calculation failed")
  unless (advanceReady waiting 0 7).isNone do
    throw (IO.userError "source scheduler fabricated a result without arrival")
  let foreign := {saved.entry with ticket := {saved.entry.ticket with id := saved.entry.ticket.id+1}}
  unless (arrive waiting foreign value).isNone && (arrive waiting saved.entry (value+1)).isNone do
    throw (IO.userError "foreign request or incorrect value was admitted")
  let some received := arrive waiting saved.entry value |
    throw (IO.userError "correct result rejected after independent malformed candidates")
  let reference := ReferenceContinuation.tick waiting 0 7
  unless received.state.values == reference.state.values && received.state.writes == reference.state.writes &&
      received.state.origins == reference.state.origins &&
      received.state.machine.store.events == reference.state.machine.store.events &&
      received.state.machine.pending == reference.state.machine.pending &&
      received.completed == reference.completed && received.remaining == reference.remaining &&
      received.stopped == reference.stopped && received.status == .ready do
    throw (IO.userError "delivered result changed source semantics or history")
  unless (arrive received saved.entry value).isNone do throw (IO.userError "same result consumed twice")
  let nextView := {ReferenceSourceControls.view with generation := 7}
  let some headed := ReferenceContinuation.authorityHead waiting nextView |
    throw (IO.userError "strict authority successor rejected")
  unless (arrive headed saved.entry value).isNone && headed.state.waiting == waiting.state.waiting do
    throw (IO.userError "old result revived after authority change")
  IO.println "RECEIVED_RESULT_MODEL_OK explicit result preserves source values/writes/origins/events/partition; wrong request/value rejected, valid arrival still accepted, duplicate/stale result rejected, waiting cannot compute its own result."
  IO.println "This is a local reference control, not authenticated delivery or a real owner process."

end MirroreaProofFirst.ReceivedResultControls
