import MirroreaProofFirstInstancePrograms

namespace MirroreaProofFirst.SourcePureLowering
open LocalContract InstancePrograms
open ContractExport.CheckedArithmetic

-- Reuse the closed arithmetic carrier without dropping strict evaluation of
-- source lets. This administrative lowering is valid only for the pure bounded
-- arithmetic/result-or-rejection profile, not external effects, error ordering
-- classes, resource/cost equality, or a final compiler optimization contract.
def sequence (first rest : Term) : Term := .add (.mul first (.integer 0)) rest

theorem sequence_run (first rest : Term) (input : Int) :
    Machine.run (sequence first rest) input =
      (do let _ ← Machine.run first input; Machine.run rest input) := by
  by_cases bounds : Machine.lo ≤ input ∧ input ≤ Machine.hi
  · simp only [Machine.run,if_pos bounds,sequence,evaluate]
    cases ha : evaluate Machine.lo Machine.hi [input] first with
    | none => simp
    | some x =>
        cases hb : evaluate Machine.lo Machine.hi [input] rest with
        | none => simp [checked,Machine.lo,Machine.hi]
        | some y =>
            have range := (denotes_math ((ContractExport.CheckedArithmetic.exact _ _ _ _ _).mp hb)).2.2
            have zero : Machine.lo ≤ (0 : Int) ∧ 0 ≤ Machine.hi := by decide
            simpa [ha,hb,checked,Int.mul_zero,Int.zero_add,zero,InRange] using
              (show (if Machine.lo ≤ y ∧ y ≤ Machine.hi then some y else none) = some y from if_pos range)
  · simp [Machine.run,bounds]

def prependChecks (checks : List Term) (result : Term) : Term := checks.foldr sequence result

def strictRun (checks : List Term) (result : Term) (input : Int) : Option Int :=
  match checks with
  | [] => Machine.run result input
  | first :: rest => do let _ ← Machine.run first input; strictRun rest result input

theorem prefix_run (checks : List Term) (result : Term) (input : Int) :
    Machine.run (prependChecks checks result) input = strictRun checks result input := by
  induction checks with
  | nil => rfl
  | cons first rest ih =>
      simp only [prependChecks,List.foldr_cons,sequence_run,strictRun]
      change (do let _ ← Machine.run first input; Machine.run (prependChecks rest result) input) = _
      rw [ih]

namespace Controls
def overflowing := Term.mul (.integer Machine.hi) (.integer 2)
#guard Machine.run (.integer 1) 0 = some 1
#guard Machine.run overflowing 0 = none
#guard Machine.run (prependChecks [overflowing] (.integer 1)) 0 = none
#guard Machine.run (prependChecks [.integer 2] (.integer 1)) 0 = some 1
end Controls
#print axioms sequence_run
#print axioms prefix_run
end MirroreaProofFirst.SourcePureLowering
