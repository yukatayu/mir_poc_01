import MirroreaProofFirstPublicationOutcome
import MirroreaProofFirstPublicationProgress
import ActualReference

namespace MirroreaProofFirst.PublicationOutcomeControls
open PublicationOutcome PublicationProgress

abbrev Model := PublicationUse.State 3 (Snapshot 3 1) (PublicationSession.Command 3 1)
def start : Model := PublicationUse.initial 3 0 ⟨_root_.begun,[]⟩
def round (s : Model) (command : PublicationSession.Command 3 1) : Option Model :=
  useRun evaluate s ((normal 3 (s.base.barrier.published+1)).map (lift command))

def main : IO Unit := do
  let definition := InstancePrograms.Controls.original
  let some registered := round start (.control 0 0 7 (.register definition none)) |
    throw (IO.userError "typed registration was rejected")
  unless (registered.base.current.replies.head?).map Reply.result == some (.control (some 0)) &&
      (registered.base.current.replies.head?).map Reply.ordinal == some 0 &&
      registered.base.current.session.state.machine.store.core.system.configuration.definitions == 1 do
    throw (IO.userError "registration return/id was erased or misbound")
  let some first := round registered (.control 0 0 7 (.instantiate 0 7 [0] none .top)) |
    throw (IO.userError "first creation was rejected")
  let some second := round first (.control 0 0 7 (.instantiate 0 7 [0] none .top)) |
    throw (IO.userError "second creation was rejected")
  unless (first.base.current.replies.head?).map Reply.result == some (.control (some 0)) &&
      (second.base.current.replies.head?).map Reply.result == some (.control (some 1)) &&
      second.base.current.replies.map Reply.ordinal == [2,1,0] do
    throw (IO.userError "consecutive control results/ordinals were confused")
  let some denied := round second (.cancellation 0 7) |
    throw (IO.userError "denied cancellation has no observable outcome")
  unless (denied.base.current.replies.head?).map Reply.result == some (.cancellation (.failed .awaiting)) &&
      denied.base.current.session.state.writes == second.base.current.session.state.writes &&
      denied.base.current.session.state.machine.pending == second.base.current.session.state.machine.pending do
    throw (IO.userError "a publication was mistaken for successful cancellation")
  let waiting := ReferenceContinuation.drive 9 _root_.begun 0 7
  unless waiting.state.waiting.isSome do throw (IO.userError "actual source did not reach pending")
  let held : Model := PublicationUse.initial 3 40 ⟨waiting,[]⟩
  let some rejected := round held (.cancellation 0 99) |
    throw (IO.userError "unauthorized cancellation lost its typed result")
  unless (rejected.base.current.replies.head?).map Reply.result == some (.cancellation (.failed .rejected)) &&
      rejected.base.current.session.state.waiting == waiting.state.waiting &&
      rejected.base.current.session.state.machine.pending == waiting.state.machine.pending do
    throw (IO.userError "unauthorized cancellation cleared pending work")
  let some cancelled := round rejected (.cancellation 0 7) |
    throw (IO.userError "authorized cancellation failed after an observed rejection")
  unless (cancelled.base.current.replies.head?).map Reply.result == some (.cancellation .ready) &&
      cancelled.base.current.session.status == .failed .cancelled &&
      cancelled.base.current.session.state.waiting.isNone do
    throw (IO.userError "authorized cancellation result did not match actual transition")
  for i in List.finRange 3 do
    unless PublicationUse.check evaluate cancelled (.enter i) &&
        (cancelled.base.cached i).replies.map Reply.result == cancelled.base.current.replies.map Reply.result do
      throw (IO.userError "an open participant did not receive the exact outcomes")
  IO.println "OUTCOME_MODEL_OK created ids 0 then 1 retained, generated ordinals distinct; absent/denied/authorized cancellation outcomes distinguished; pending survives denial then resolves by authorized cancellation."
  IO.println "General normal-round theorem covers arbitrary evaluable values; these are finite integration controls only, no network/authority issuance claim."

end MirroreaProofFirst.PublicationOutcomeControls
