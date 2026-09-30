import MirroreaProofFirstCoordinatorUse
namespace MirroreaProofFirst.CoordinatorUseControls
open CoordinatorUse

def evaluate (view : Nat × Nat) (command : Nat) : Option (Nat × Nat) :=
 some (view.1 + command,view.2 + 1)
abbrev S := State 3 (Nat × Nat) Nat
abbrev A := Action 3 (Nat × Nat) Nat
def begin : S := initial 3 40 (100,7)
def run : S → List A → Option S
 | s,[] => some s
 | s,a::rest => (execute evaluate s a).bind fun next => run next rest

def g0 : Grant 3 := ⟨0,0,.issue⟩
def held := execute evaluate begin (.acquire 0 .issue)
def closing := held.bind fun s => execute evaluate s .close
#guard held.isSome && closing.isSome
#guard (closing.bind fun s => execute evaluate s (.stage 5)).isNone
#guard (closing.bind fun s => execute evaluate s .publish).isNone
#guard (closing.bind fun s => execute evaluate s (.acquire 1 .body)).isNone
#guard (closing.bind fun s => execute evaluate s (.finish {g0 with endpoint := 1})).isNone
#guard (closing.bind fun s => execute evaluate s (.finish {g0 with kind := .body})).isNone
#guard (closing.bind fun s => execute evaluate s (.finish {g0 with serial := 9})).isNone
def drained := closing.bind fun s => execute evaluate s (.finish g0)
def preparing := drained.bind fun s => run s [.stage 5,.freeze 0 41,.freeze 1 41,.freeze 2 41]
#guard drained.isSome && preparing.isSome
#guard (preparing.bind fun s => execute evaluate s (.preparedAck 0 41 (105,7))).isNone
#guard (preparing.bind fun s => execute evaluate s (.preparedAck 0 42 (105,8))).isNone
def twoPrepared := preparing.bind fun s => run s [.preparedAck 0 41 (105,8),.preparedAck 1 41 (105,8)]
#guard twoPrepared.isSome
#guard (twoPrepared.bind fun s => execute evaluate s .publish).isNone
def allPrepared := twoPrepared.bind fun s => execute evaluate s (.preparedAck 2 41 (105,8))
#guard allPrepared.isSome
#guard (allPrepared.map fun s => (s.useState.base.barrier.published,s.useState.base.barrier.installed 0,s.useState.base.current,s.useState.base.cached 0)) = some (40,40,(100,7),(100,7))
#guard (allPrepared.bind fun s => execute evaluate s (.activate 0 41)).isNone
#guard (allPrepared.bind fun s => execute evaluate s .reopen).isNone
#guard (allPrepared.bind fun s => execute evaluate s (.acquire 0 .issue)).isNone
def published := allPrepared.bind fun s => execute evaluate s .publish
#guard (published.map fun s => (s.useState.base.barrier.published,s.useState.base.current,s.useState.base.cached 0)) = some (41,(105,8),(100,7))
#guard (published.bind fun s => execute evaluate s .reopen).isNone
def twoActive := published.bind fun s => run s [.activate 0 41,.activate 1 41]
#guard (twoActive.bind fun s => execute evaluate s .reopen).isNone
def ready := twoActive.bind fun s => run s [.activate 2 41,.reopen]
#guard ready.isSome
def heldAgain := ready.bind fun s => execute evaluate s (.acquire 0 .issue)
#guard (heldAgain.map fun s => s.active) = some (some ⟨0,1,.issue⟩)
#guard (heldAgain.map fun s => s.useState.held 0) = some (some (41,(105,8)))
#guard (heldAgain.bind fun s => execute evaluate s (.finish g0)).isNone
#guard (heldAgain.bind fun s => execute evaluate s (.finish ⟨0,1,.issue⟩)).isSome
#guard (heldAgain.bind fun s => execute evaluate s (.finish ⟨0,1,.issue⟩) |>.bind fun s => execute evaluate s (.finish ⟨0,1,.issue⟩)).isNone

theorem run_reached (prior : Reached evaluate 3 revision value s)
 (ran : run s actions = some next) : Reached evaluate 3 revision value next := by
 induction actions generalizing s with
 | nil => cases ran; exact prior
 | cons action rest ih =>
   unfold run at ran
   cases performed : execute evaluate s action with
   | none => simp [performed] at ran
   | some middle =>
     simp only [performed,Option.bind_some] at ran
     have accepted := execute_exact.mp performed
     have advanced : Reached evaluate 3 revision value middle := by
       rw [accepted.2]
       exact .step prior (.action accepted.1)
     exact ih advanced ran

#print axioms run_reached
end MirroreaProofFirst.CoordinatorUseControls
