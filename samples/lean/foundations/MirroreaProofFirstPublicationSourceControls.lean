import MirroreaProofFirstPublicationSession
import MirroreaProofFirstPublicationUseControls
import ActualReference

namespace MirroreaProofFirst.PublicationSourceControl
open PublicationUseControls

-- Reuses this run's actual Rust-parser-generated W3 consumer, unchanged.
-- This is a local protocol/model test. It is NOT real-network W4 evidence.
abbrev PhysicalModel := PublicationUse.State 3 (ReferenceContinuation.Session 3 1) (PublicationSession.Command 3 1)

def start : PhysicalModel := PublicationUse.initial 3 0 _root_.begun

def drive : Nat → PhysicalModel → Option PhysicalModel
  | 0,s => some s
  | fuel+1,s => (run PublicationSession.evaluate s (round s (.tick 0 7))).bind (drive fuel)

theorem drive_reached (fuel : Nat) (s next : PhysicalModel)
    (path : PublicationUse.Reached PublicationSession.evaluate 3 0 _root_.begun s)
    (accepted : drive fuel s = some next) :
    PublicationUse.Reached PublicationSession.evaluate 3 0 _root_.begun next := by
  induction fuel generalizing s next with
  | zero => cases accepted; exact path
  | succ fuel ih =>
      cases ran : run PublicationSession.evaluate s (round s (.tick 0 7)) with
      | none => simp [drive,ran] at accepted
      | some intermediate =>
          exact ih _ _ (run_reached path ran) (by simpa [drive,ran] using accepted)

theorem admitted_output (fuel : Nat) (next : PhysicalModel) (accepted : drive fuel start = some next) :
    ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy next.base.current :=
  PublicationSession.reached_rooted _ _ _ _ _root_.admittedLaunch
    (PublicationUse.reached_payload (drive_reached fuel start next .initial accepted))

def main : IO Unit := do
  let fuel := 2*_root_.program.length+1
  let some result := drive fuel start | throw (IO.userError "publication/source model rejected normal source")
  let s := result.base.current
  let expected := _root_.finished
  unless s.status == expected.status && s.status == .ready &&
      s.completed == expected.completed && s.remaining == expected.remaining && s.stopped == expected.stopped &&
      s.superseded == expected.superseded && s.state.values == expected.state.values &&
      s.state.writes == expected.state.writes && s.state.origins == expected.state.origins &&
      s.state.machine.store.events == expected.state.machine.store.events &&
      s.state.machine.pending == expected.state.machine.pending do
    throw (IO.userError "publication/source model changed W3 source values, pending, origin or history")
  for i in List.finRange 3 do
    unless PublicationUse.check PublicationSession.evaluate result (.enter i) &&
        (result.base.cached i).state.values == s.state.values &&
        (result.base.cached i).state.writes == s.state.writes do
      throw (IO.userError "publication/source model left a participant closed or on stale source state")
  let nextView := {ReferenceSourceControls.view with generation := 7}
  let some headed := run PublicationSession.evaluate start (round start (.head nextView)) |
    throw (IO.userError "publication/source model rejected an actual strict authority successor")
  unless headed.base.barrier.published == 1 &&
      headed.base.current.state.machine.store.core.system.view.generation == 7 do
    throw (IO.userError "protocol revision was confused with authority generation")
  let some newHeadResult := drive fuel headed | throw (IO.userError "new-head normal source did not progress")
  unless newHeadResult.base.current.status == .ready &&
      newHeadResult.base.current.state.values == expected.state.values do
    throw (IO.userError "new-head source stopped producing its permitted results")
  let some waiting := drive 9 start | throw (IO.userError "source did not reach its first pending call")
  let some saved := waiting.base.current.state.waiting | throw (IO.userError "source has no saved pending call")
  unless (ReferenceExecution.finish waiting.base.current.state.machine saved.entry 10).isSome do
    throw (IO.userError "old-result positive control was not eligible before publication")
  let some afterHead := run PublicationSession.evaluate waiting (round waiting (.head nextView)) |
    throw (IO.userError "authority publication wrongly required draining source pending work")
  unless afterHead.base.current.state.waiting == some saved &&
      afterHead.base.current.state.machine.pending == waiting.base.current.state.machine.pending &&
      (ReferenceExecution.finish afterHead.base.current.state.machine saved.entry 10).isNone do
    throw (IO.userError "publication erased pending work or resurrected an old result")
  IO.println s!"SOURCE_PUBLICATION_MODEL_OK statements={_root_.program.length} rounds={fuel} participants=3 writes={s.state.writes.length}"
  IO.println "Source values, writes, origins, runtime-model events, pending and source partitions equal the actual generated W3 consumer. Local model only."
  IO.println "HEAD_MODEL_OK protocol revision 1 carries authority generation 7; new source succeeds; eligible old pending result is rejected after publication and pending is retained."

#print axioms drive_reached
#print axioms admitted_output
end MirroreaProofFirst.PublicationSourceControl
