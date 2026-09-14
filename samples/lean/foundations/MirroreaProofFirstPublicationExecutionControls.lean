import MirroreaProofFirstPublicationExecution
import MirroreaProofFirstPublicationUseControls
import ActualReference

namespace MirroreaProofFirst.PublicationExecutionControls
open PublicationExecution PublicationUseControls

abbrev Model := PublicationUse.State 3 (Snapshot 3 1) (Command 3 1)

def ordinary : Nat → Model → Option Model
  | 0,s => some s
  | fuel+1,s => (run snapshotEvaluate s (round s (.source (.tick 0 7)))).bind (ordinary fuel)

def sameSource (s t : ReferenceContinuation.Session 3 1) : Bool :=
  s.status == t.status && s.completed == t.completed && s.remaining == t.remaining &&
  s.stopped == t.stopped && s.superseded == t.superseded &&
  s.state.values == t.state.values && s.state.writes == t.state.writes &&
  s.state.origins == t.state.origins && s.state.nextRequest == t.state.nextRequest &&
  s.state.waiting == t.state.waiting && s.state.machine.pending == t.state.machine.pending &&
  s.state.machine.store.events == t.state.machine.store.events

def main : IO Unit := do
  let start : Model := PublicationUse.initial 3 0 ⟨_root_.begun,[]⟩
  let some waiting := ordinary 9 start | throw (IO.userError "ordinary source failed to reach pending call")
  let s := waiting.base.current.session
  let some saved := s.state.waiting | throw (IO.userError "no actual source pending invocation")
  let some value := InvocationBoundary.execute saved.entry.ticket |
    throw (IO.userError "local reference calculation failed")
  -- Oracle falsifier: the old model's publication evaluator alone produces a
  -- pending result. This is valid reference execution, invalid physical evidence.
  let old := PublicationUse.initial 3 0 s
  let some bypass := run PublicationSession.evaluate old (round old (.tick 0 7)) |
    throw (IO.userError "old unrestricted publication counterexample did not run")
  unless bypass.base.current.state.writes.length == s.state.writes.length+1 do
    throw (IO.userError "old waiting tick did not synthesize result write")
  unless (ordinary 1 waiting).isNone do
    throw (IO.userError "closed publication still completed waiting source without result input")
  let foreign := {saved.entry with ticket := {saved.entry.ticket with id := saved.entry.ticket.id+1}}
  for command in [Command.received foreign value,Command.received saved.entry (value+1)] do
    let (after,observation) := attempt s command
    unless observation == .refused && sameSource after s do
      throw (IO.userError "immutable rejection frame changed source state")
    unless (run snapshotEvaluate waiting (round waiting command)).isNone do
      throw (IO.userError "malformed result was published")
  let some received := run snapshotEvaluate waiting (round waiting (.received saved.entry value)) |
    throw (IO.userError "explicit valid input was rejected after malformed candidates")
  unless sameSource received.base.current.session (ReferenceContinuation.tick s 0 7) do
    throw (IO.userError "published explicit arrival changed source result/continuation/history")
  let some reply := received.base.current.replies.head? | throw (IO.userError "result reply missing")
  unless reply.result == .received value && reply.ordinal == 9 do
    throw (IO.userError "published result interface was erased or misnumbered")
  unless (run snapshotEvaluate received (round received (.received saved.entry value))).isNone do
    throw (IO.userError "duplicate result consumed")
  for i in List.finRange 3 do
    unless sameSource (received.base.cached i).session received.base.current.session &&
        PublicationUse.check snapshotEvaluate received (.enter i) do
      throw (IO.userError "received-result publication left stale or unusable participant")
  let nextView := {ReferenceSourceControls.view with generation := 7}
  let some headed := run snapshotEvaluate waiting (round waiting (.source (.head nextView))) |
    throw (IO.userError "head update while pending refused")
  unless headed.base.current.session.state.waiting == s.state.waiting &&
      (run snapshotEvaluate headed (round headed (.received saved.entry value))).isNone do
    throw (IO.userError "head update erased pending or revived old result")
  -- Every later source instruction and model result uses the same closed
  -- publication evaluator. Values here remain supplied by LOCAL computation;
  -- this is a semantics control and explicitly not an actual-owner test.
  let mut all := start
  for _ in [:2*_root_.program.length+1] do
    let current := all.base.current.session
    if current.status == .ready && !current.remaining.isEmpty then
      let some next := ordinary 1 all | throw (IO.userError "useful ordinary statement refused")
      all := next
    else if current.status == .waiting then
      let some pending := current.state.waiting | throw (IO.userError "waiting continuation lost")
      let some output := InvocationBoundary.execute pending.entry.ticket |
        throw (IO.userError "local reference result unavailable")
      let some next := run snapshotEvaluate all (round all (.received pending.entry output)) |
        throw (IO.userError "explicit result refused")
      all := next
  unless sameSource all.base.current.session _root_.finished && all.base.current.session.status == .ready do
    throw (IO.userError "closed source/explicit-result run lost useful source behavior")
  let ordinals := all.base.current.replies.map Reply.ordinal
  unless ordinals == (List.range ordinals.length).reverse do
    throw (IO.userError "empty-root generated reply ordinals reused")
  IO.println s!"EXECUTION_MODEL_OK source={_root_.program.length} writes={all.base.current.session.state.writes.length} replies={ordinals.length}; old publication bypass exhibited, closed publication rejects waiting tick, explicit results and full ordinary source succeed."
  IO.println "Wrong id/value rejected with immutable state frame; valid arrival, pending head invalidation, duplicate rejection and participant installation checked. Model inputs are locally computed: no physical owner, transport or mutable-driver claim."

end MirroreaProofFirst.PublicationExecutionControls
