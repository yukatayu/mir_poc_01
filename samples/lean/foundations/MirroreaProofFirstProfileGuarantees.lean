import MirroreaProofFirstContractExport
namespace MirroreaProofFirst.ProfileGuarantees
open LocalContract ContractExport

-- A reusable code contract, stronger than positivity at one actual invocation.
def UniformPositive (b : Binding) : Prop :=
 ∀ input : Nat → Int, AssumptionsHold input b.assumptions → 0 < eval input b.code

theorem accepted_symbolic_uniform {b : Binding} {e : Envelope} {n : Nat}
 (accepted : accept b e = some n) (profile : b.profile = .symbolicNonnegative) :
 UniformPositive b := by
 unfold accept at accepted; split at accepted
 · rename_i checked
   have both : commonCheck b e = true ∧ profileCheck b e = true := by
     simpa only [Bool.and_eq_true] using checked
   have symbolic := both.2
   simp only [profileCheck,profile,Bool.and_eq_true,decide_eq_true_eq] at symbolic
   intro input assumptions
   rw [symbolic.1]
   exact positive_result symbolic.2 ((assumptions_exact _ _).mpr assumptions)
 · contradiction

-- The conditional uniform property has an actual satisfying valuation: the
-- same accepted invocation. An inconsistent assumption set cannot pass export.
theorem accepted_symbolic_inhabited {b : Binding} {e : Envelope} {n : Nat}
 (accepted : accept b e = some n) (profile : b.profile = .symbolicNonnegative) :
 UniformPositive b ∧ ∃ input, AssumptionsHold input b.assumptions :=
 ⟨accepted_symbolic_uniform accepted profile,inputs b,(accept_sound accepted).common.assumptions⟩

-- Successful checked-value evidence cannot be strengthened to the uniform contract.
theorem checked_value_not_uniform : ¬ UniformPositive ContractExport.Controls.dynamic := by
 intro uniform
 have h := uniform (fun _ => -1) (by
   simp [ContractExport.Controls.dynamic,ContractExport.Controls.b,AssumptionsHold])
 simp [ContractExport.Controls.dynamic,eval] at h

example : accept ContractExport.Controls.dynamic
 {ContractExport.Controls.e with binding := ContractExport.Controls.dynamic,result := 4} = some 4 := by decide

#print axioms accepted_symbolic_uniform
#print axioms accepted_symbolic_inhabited
#print axioms checked_value_not_uniform
end MirroreaProofFirst.ProfileGuarantees
