import MirroreaProofFirstPublication

open MirroreaProofFirst.Publication

def run (s : State n) (actions : List (Action n)) : Option (State n) :=
  actions.foldl (fun state action => state.bind (execute · action)) (some s)

def round (g : Nat) : List (Action 2) :=
  [.begin,.freeze 1 g,.freeze 0 g,.acknowledge 0 g,.acknowledge 1 g,
   .publish,.install 1 g,.install 0 g,.use 0,.use 1]

-- Fixed controls are not the general proof in the imported module.
example : (run (initial 2 7) (round 8)).isSome = true := by decide
example : (run (initial 2 7) (round 8 ++ round 9)).isSome = true := by decide
example : run (initial 2 7) [.begin,.publish] = none := by decide
example : run (initial 2 7) [.begin,.freeze 0 8,.acknowledge 0 8,.publish] = none := by decide
example : run (initial 2 7) [.begin,.acknowledge 0 8] = none := by decide
example : run (initial 2 7) [.begin,.freeze 0 8,.use 0] = none := by decide
example : run (initial 2 7) [.begin,.freeze 0 8,.install 0 8] = none := by decide
example : run (initial 2 7) (round 8 ++ [.begin,.freeze 0 9,.install 0 8]) = none := by decide
example : run (initial 2 7) (round 8 ++ [.begin,.acknowledge 0 8,.acknowledge 1 8,.publish]) = none := by decide
example : (run (initial 2 7) ([.begin,.freeze 0 8,.freeze 0 8,.acknowledge 0 8,
    .acknowledge 0 8,.freeze 1 8,.acknowledge 1 8,.publish,.install 0 8,.use 0])).isSome = true := by decide
example : run (initial 2 7) (round 8 ++ [.install 1 7]) = none := by decide
example : (run (initial 2 7) (round 8 ++ [.freeze 0 8,.acknowledge 0 8,.use 0])).isSome = true := by decide
example : check (ownerOnlyPublish (initial 2 7) 8) (.use 0) = true ∧
    (ownerOnlyPublish (initial 2 7) 8).installed 0 ≠ (ownerOnlyPublish (initial 2 7) 8).published := by decide

#eval (run (initial 2 7) (round 8 ++ round 9)).map (fun s =>
  (s.published,s.installed 0,s.installed 1,check s (.use 0),check s (.use 1)))
