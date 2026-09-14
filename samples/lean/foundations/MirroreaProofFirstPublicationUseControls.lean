import MirroreaProofFirstPublicationUse

namespace MirroreaProofFirst.PublicationUseControls
open PublicationUse

def run (evaluate : Value → Command → Option Value) : State n Value Command → List (Action n Command) → Option (State n Value Command)
  | s,[] => some s
  | s,action :: rest => (execute evaluate s action).bind (run evaluate · rest)

theorem run_reached (path : Reached evaluate n revision value s)
    (accepted : run evaluate s actions = some next) : Reached evaluate n revision value next := by
  induction actions generalizing s next with
  | nil => cases accepted; exact path
  | cons action rest ih =>
      cases step : execute evaluate s action with
      | none => simp [run,step] at accepted
      | some intermediate =>
          obtain ⟨allowed,equal⟩ := (execute_exact _ _ _ _).mp step
          have advance : Step evaluate s intermediate := equal ▸ .action allowed
          exact ih (.step path advance) (by simpa [run,step] using accepted)

-- Concrete normal schedule, generic in participant count and command. Its
-- executable success remains checked; no scheduling/fairness axiom is added.
def round (s : State n Value Command) (command : Command) : List (Action n Command) :=
  let revision := s.base.barrier.published + 1
  [.administrative (.stage command)] ++
    (List.finRange n).map (fun i => .administrative (.freeze i revision)) ++
    (List.finRange n).map (fun i => .administrative (.acknowledge i revision)) ++
    [.administrative .publish] ++
    (List.finRange n).map (fun i => .administrative (.install i revision)) ++
    (List.finRange n).flatMap (fun i => [.enter i,.finish i])

def advance (value : Nat) (_ : Unit) : Option Nat := some (value+10)
def start : State 2 Nat Unit := initial 2 7 100
def finished := run advance start (round start ())
example : finished.map (fun s => s.base.current) = some 110 := by decide
example : finished.map (fun s => s.base.cached 0) = some 110 := by decide
example : finished.map (fun s => s.base.cached 1) = some 110 := by decide

example : run advance start [.enter 0,.administrative (.stage ()),.administrative (.freeze 0 8)] = none := by decide
example : run advance start [.enter 0,.enter 0] = none := by decide
example : run advance start [.finish 0] = none := by decide
example : (run advance start [.enter 0,.administrative (.stage ()),.finish 0,
    .administrative (.freeze 0 8),.administrative (.freeze 1 8),
    .administrative (.acknowledge 0 8),.administrative (.acknowledge 1 8),
    .administrative .publish,.administrative (.install 0 8),.enter 0]).map
    (fun s => s.held 0) = some (some (8,110)) := by decide
example : run advance start [.administrative (.stage ()),.administrative (.freeze 0 8),.enter 0] = none := by decide
example : (finished.bind fun s => run advance s [.enter 0,.administrative (.install 0 8)]).isNone = true := by decide

#print axioms run_reached
end MirroreaProofFirst.PublicationUseControls
