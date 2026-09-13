import MirroreaProofFirstContractExport

namespace MirroreaProofFirst.InstancePrograms
open LocalContract
namespace Machine
def lo : Int := -9223372036854775808
def hi : Int := 9223372036854775807
def run (code : Term) (input : Int) : Option Int :=
  if lo ≤ input ∧ input ≤ hi then ContractExport.CheckedArithmetic.evaluate lo hi [input] code else none
abbrev Executes (code : Term) (input result : Int) :=
  (lo ≤ input ∧ input ≤ hi) ∧ ContractExport.CheckedArithmetic.Denotes lo hi [input] code result
theorem run_exact (code : Term) (input result : Int) :
    run code input = some result ↔ Executes code input result := by
  by_cases bounds : lo ≤ input ∧ input ≤ hi
  · simp [run,Executes,bounds,ContractExport.CheckedArithmetic.exact]
  · simp [run,Executes,bounds]
theorem run_input_bounded (code : Term) (input result : Int) (ran : run code input = some result) :
    lo ≤ input ∧ input ≤ hi := ((run_exact code input result).mp ran).1
end Machine

-- Explicit finite input contract, not a silent restriction of Int64. A future
-- theory plugin may supply another independently sound inclusion checker.
structure Contract where
  inputs : List Int
  lower : Int
  upper : Int
  deriving DecidableEq, Repr

def Output (c : Contract) (value : Int) : Prop := c.lower ≤ value ∧ value ≤ c.upper
def outputCheck (c : Contract) (value : Int) : Bool :=
  decide (c.lower ≤ value ∧ value ≤ c.upper)

structure Definition where
  code : Term
  contract : Contract
  deriving DecidableEq, Repr

-- Declarative specification contains bounded arithmetic derivations, not the
-- evaluator or definition-checker's success as an assumed conclusion.
structure Satisfies (d : Definition) : Prop where
  scope : ContractExport.Scoped 1 d.code
  inhabited : d.contract.inputs ≠ []
  total : ∀ input ∈ d.contract.inputs, ∃ result,
    Machine.Executes d.code input result ∧ Output d.contract result

def check (d : Definition) : Bool :=
  ContractExport.scopeCheck 1 d.code && decide (d.contract.inputs ≠ []) &&
    d.contract.inputs.all fun input =>
      match Machine.run d.code input with
      | none => false
      | some result => outputCheck d.contract result

theorem check_exact (d : Definition) : check d = true ↔ Satisfies d := by
  simp only [check, Bool.and_eq_true, decide_eq_true_eq]
  constructor
  · rintro ⟨⟨scope, inhabited⟩, total⟩
    refine ⟨(ContractExport.scoped_exact _ _).mp scope, inhabited, ?_⟩
    intro input member
    have h := List.all_eq_true.mp total input member
    cases executed : Machine.run d.code input with
    | none => simp [executed] at h
    | some result =>
      exact ⟨result, (Machine.run_exact _ _ _).mp executed,
        by simpa [executed, outputCheck, Output] using h⟩
  · intro h
    refine ⟨⟨(ContractExport.scoped_exact _ _).mpr h.scope, h.inhabited⟩, ?_⟩
    apply List.all_eq_true.mpr
    intro input member
    obtain ⟨result, execution, output⟩ := h.total input member
    simp [ (Machine.run_exact _ _ _).mpr execution, outputCheck, Output] at output ⊢
    exact output

-- A replacement may extend accepted inputs and strengthen the result interval.
-- Its code is checked independently; identical arity/labels are insufficient.
structure Refines (old new : Contract) : Prop where
  inputs : ∀ input ∈ old.inputs, input ∈ new.inputs
  lower : old.lower ≤ new.lower
  upper : new.upper ≤ old.upper

def refinementCheck (old new : Contract) : Bool :=
  old.inputs.all (fun input => new.inputs.contains input) &&
    decide (old.lower ≤ new.lower ∧ new.upper ≤ old.upper)

theorem refinement_exact (old new : Contract) :
    refinementCheck old new = true ↔ Refines old new := by
  simp only [refinementCheck, Bool.and_eq_true, List.all_eq_true,
    List.contains_iff_mem, decide_eq_true_eq]
  exact ⟨fun ⟨a,b,c⟩ => ⟨a,b,c⟩, fun h => ⟨h.inputs,h.lower,h.upper⟩⟩

def exchangeCheck (old : Contract) (new : Definition) : Bool :=
  refinementCheck old new.contract && check new

theorem exchange_exact (old : Contract) (new : Definition) :
    exchangeCheck old new = true ↔ Refines old new.contract ∧ Satisfies new := by
  simp only [exchangeCheck, Bool.and_eq_true, refinement_exact, check_exact]

theorem replacement_preserves_success (old : Contract) (new : Definition)
    (accepted : exchangeCheck old new = true) (input : Int) (member : input ∈ old.inputs) :
    ∃ result, Machine.Executes new.code input result ∧ Output old result := by
  obtain ⟨refines, satisfies⟩ := (exchange_exact old new).mp accepted
  obtain ⟨result, execution, output⟩ := satisfies.total input (refines.inputs input member)
  exact ⟨result, execution, Int.le_trans refines.lower output.1,
    Int.le_trans output.2 refines.upper⟩

theorem permitted_call_completes (d : Definition) (checked : check d = true)
    (input : Int) (member : input ∈ d.contract.inputs) :
    ∃ result, Machine.run d.code input = some result ∧ Output d.contract result := by
  obtain ⟨result, executed, output⟩ := ((check_exact d).mp checked).total input member
  exact ⟨result,(Machine.run_exact _ _ _).mpr executed,output⟩

namespace Controls
def contract : Contract := ⟨[-2,-1,0,1,2],1,100⟩
def original : Definition := ⟨.add (.mul (.input 0) (.input 0)) (.integer 1),contract⟩
def replacement : Definition := ⟨.add (.mul (.input 0) (.input 0)) (.integer 2),contract⟩
#guard check original
#guard exchangeCheck original.contract replacement
#guard Machine.run original.code 2 = some 5
#guard Machine.run replacement.code 2 = some 6
#guard !check ⟨.input 0,contract⟩
#guard !check {original with contract := {contract with inputs := []}}
#guard !exchangeCheck contract {replacement with contract := {contract with inputs := [0]}}
#guard !exchangeCheck contract {replacement with contract := {contract with lower := 0}}
def overflow : Definition := ⟨.add (.mul (.input 0) (.integer 2))
  (.integer (-9223372036854775799)),⟨[4611686018427387904],1,100⟩⟩
#guard LocalContract.eval (fun _ => 4611686018427387904) overflow.code = 9
#guard !check overflow
-- The argument boundary is checked even when a constant body never reads it.
#guard Machine.run (.integer 1) (Machine.hi+1) = none
#guard Machine.run (.integer 1) (Machine.lo-1) = none
#guard Machine.run (.integer 1) Machine.hi = some 1
#guard Machine.run (.integer 1) Machine.lo = some 1
#guard !check ⟨.integer 1,⟨[Machine.hi+1],1,1⟩⟩
#guard !check ⟨.integer 1,⟨[Machine.lo-1],1,1⟩⟩
end Controls

#print axioms Machine.run_exact
#print axioms Machine.run_input_bounded
#print axioms check_exact
#print axioms refinement_exact
#print axioms exchange_exact
#print axioms replacement_preserves_success
#print axioms permitted_call_completes
end MirroreaProofFirst.InstancePrograms
