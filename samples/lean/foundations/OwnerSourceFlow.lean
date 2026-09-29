import OwnerPartialAbort
import MirroreaProofFirstOwnerEffectService
namespace MirroreaProofFirst.OwnerSourceFlow
open ProducerFlow OwnerCheckedArithmetic OwnerEffectService

-- The existing checked tree and owner/capture distinction are retained. Labels
-- and ownership here are supplied metadata, NOT inferred or authenticated source
-- facts. Fixed metadata cannot justify a changed policy/head or authorize release.
def lower (bodies : List Body) : OwnerPartialAbort.Program (Sum Nat Nat) :=
 bodies.map fun b => (.inl b.target,translate b.tree)

def check (owner labels capturedLabels : Nat → Nat) (pc : Nat) (bodies : List Body) : Bool :=
 bodies.all (fun b => OwnerAssignment.localCheck owner (owner b.target) (translate b.tree)) &&
 AbortFlow.check (fun _ => .int) (OwnerAssignment.classes labels capturedLabels) pc (lower bodies)

def Admissible (owner labels capturedLabels : Nat → Nat) (pc : Nat) (bodies : List Body) : Prop :=
 (∀ b ∈ bodies, OwnerAssignment.Local owner (owner b.target) (translate b.tree)) ∧
 AbortFlow.Safe (fun _ => .int) (OwnerAssignment.classes labels capturedLabels) pc (lower bodies)

theorem check_exact : check owner labels capturedLabels pc bodies = true ↔
 Admissible owner labels capturedLabels pc bodies := by
 simp only [check,Bool.and_eq_true,List.all_eq_true,OwnerAssignment.local_exact,
   AbortFlow.check_exact,Admissible]

-- Actual finite arithmetic source semantics, stopping at the first failed lookup
-- or operator; earlier writes remain. This is a reference inner layer, not a
-- replacement for issue/service/ack, auth/resource admission or source lifecycle.
def execute (ops : FallibleFlow.Arithmetic) (store captured : Nat → Option Int) (bodies : List Body) :=
 OwnerPartialAbort.run ops (environment store captured) (lower bodies)

def LowInt (labels : Nat → Nat) (level : Nat) (s t : Nat → Option Int) : Prop :=
 ∀ k, labels k ≤ level → s k = t k

theorem environment_low (stateLow : LowInt labels level s t)
 (argsLow : LowInt capturedLabels level captured otherArgs) :
 OwnerPartial.LowEq (OwnerAssignment.classes labels capturedLabels) level
 (environment s captured) (environment t otherArgs) := by
 intro ref visible
 cases ref with
 | inl k => exact congrArg (Option.map Value.int) (stateLow k visible)
 | inr j => exact congrArg (Option.map Value.int) (argsLow j visible)

theorem two_run (admitted : Admissible owner labels capturedLabels pc bodies)
 (stateLow : LowInt labels level s t) (argsLow : LowInt capturedLabels level captured otherArgs)
 (ops : FallibleFlow.Arithmetic) :
 OwnerPartial.LowEq (OwnerAssignment.classes labels capturedLabels) level
 (execute ops s captured bodies).1 (execute ops t otherArgs bodies).1 ∧
 OwnerPartialAbort.visible (OwnerAssignment.classes labels capturedLabels) level
 (execute ops s captured bodies).2 =
 OwnerPartialAbort.visible (OwnerAssignment.classes labels capturedLabels) level
 (execute ops t otherArgs bodies).2 :=
 OwnerPartialAbort.noninterference admitted.2 ops _ _ (environment_low stateLow argsLow)

-- A source assignment never changes immutable arguments. In particular, two
-- arguments aliased to one owner cell still retain their captured values.
theorem captures_retained (ops : FallibleFlow.Arithmetic) (store captured : Nat → Option Int)
 (bodies : List Body) (j : Nat) :
 (execute ops store captured bodies).1 (.inr j) = (captured j).map Value.int := by
 have frame (program : OwnerPartialAbort.Program (Sum Nat Nat))
   (s : Sum Nat Nat → Option Value)
   (targets : ∀ row ∈ program, row.1 ≠ Sum.inr j) :
   (OwnerPartialAbort.run ops s program).1 (.inr j) = s (.inr j) := by
   induction program generalizing s with
   | nil => rfl
   | cons head rest ih =>
     obtain ⟨key,tree⟩ := head
     have ne : key ≠ Sum.inr j := targets (key,tree) (by simp)
     simp only [OwnerPartialAbort.run]
     split
     · rfl
     · rw [ih _ (fun row member => targets row (List.mem_cons_of_mem _ member))]
       simp [OwnerPartialAbort.put,OwnerPartial.put,Ne.symm ne]
 apply frame
 intro row member
 obtain ⟨body,_,same⟩ := List.mem_map.mp member
 subst row
 simp

-- Connect the lowered source instruction to the existing checked evaluator,
-- without replacing a missing lookup or a failed computation by a value.
theorem single_success (evaluated : evaluate ops store captured body.tree = some value) :
 execute ops store captured [body] =
 (OwnerPartial.put (environment store captured) (.inl body.target) (.int value),
  [.wrote (.inl body.target) (.int value)]) := by
 simp only [execute,lower,List.map_cons,List.map_nil,OwnerPartialAbort.run,extraction,evaluated,Option.map_some]

theorem single_failure (evaluated : evaluate ops store captured body.tree = none) :
 execute ops store captured [body] = (environment store captured,[.failed (.inl body.target)]) := by
 simp only [execute,lower,List.map_cons,List.map_nil,OwnerPartialAbort.run,extraction,evaluated,Option.map_none]

theorem preserves_integer_values (admitted : Admissible owner labels capturedLabels pc bodies)
 (ops : FallibleFlow.Arithmetic) (store captured : Nat → Option Int) :
 ∀ key value, (execute ops store captured bodies).1 key = some value → value.ty = .int := by
 apply OwnerPartialAbort.run_typed admitted.2 ops (environment store captured)
 intro key value present
 cases key with
 | inl k =>
   cases got : store k with
   | none => simp [environment,got] at present
   | some x => simp only [environment,Sum.elim_inl,got,Option.map_some,Option.some.injEq] at present; rw [←present]; rfl
 | inr j =>
   cases got : captured j with
   | none => simp [environment,got] at present
   | some x => simp only [environment,Sum.elim_inr,got,Option.map_some,Option.some.injEq] at present; rw [←present]; rfl

def completionObservation (labels capturedLabels : Nat → Nat) (level : Nat)
 (ops : FallibleFlow.Arithmetic) (store captured : Nat → Option Int) (bodies : List Body) : Option Bool :=
 if AbortFlow.completionLevel (OwnerAssignment.classes labels capturedLabels) (lower bodies) ≤ level then
  some (AbortFlow.completed (execute ops store captured bodies).2)
 else none

theorem completion_low (stateLow : LowInt labels level s t)
 (argsLow : LowInt capturedLabels level captured otherArgs) (ops : FallibleFlow.Arithmetic) :
 completionObservation labels capturedLabels level ops s captured bodies =
 completionObservation labels capturedLabels level ops t otherArgs bodies :=
 OwnerPartialAbort.completion_noninterference ops _ level (lower bodies) _ _ (environment_low stateLow argsLow)


namespace Controls
def labels (k : Nat) : Nat := if k = 0 then 0 else 1
def owners (_ : Nat) : Nat := 5
def capturedLabels (_ : Nat) : Nat := 1
def publicBody : Body := ⟨0,.add (.state 0) (.integer 1)⟩
def privateBody : Body := ⟨1,.add (.state 1) (.parameter 0)⟩
def constantPrivate : Body := ⟨1,.integer 7⟩
def safe := [publicBody,privateBody]
def leaky := [privateBody,publicBody]
def store (secret : Option Int) (k : Nat) : Option Int := if k = 0 then some 10 else secret
def captured (_ : Nat) : Option Int := some 1
def result (secret : Option Int) (bodies : List Body) := execute (FallibleFlow.signed 63) (store secret) captured bodies
def publicView (secret : Option Int) (bodies : List Body) :=
 OwnerPartialAbort.visible (OwnerAssignment.classes labels capturedLabels) 0 (result secret bodies).2
#guard check owners labels capturedLabels 0 safe = true
#guard check owners labels capturedLabels 0 leaky = false
#guard publicView (some 0) safe = [.wrote (.inl 0) (.int 11)]
#guard publicView (some (2^63-1)) safe = [.wrote (.inl 0) (.int 11)]
#guard publicView none safe = [.wrote (.inl 0) (.int 11)]
#guard publicView (some 0) leaky ≠ publicView (some (2^63-1)) leaky
#guard publicView (some 0) leaky ≠ publicView none leaky
-- The inherited profile does not arbitrarily taint a constant private write.
#guard check owners labels capturedLabels 0 [constantPrivate,publicBody] = true
-- Explicit captured labels forbid making a secret parameter a public write.
#guard check owners labels capturedLabels 0 [⟨0,.parameter 0⟩] = false
#guard check owners labels (fun _ => 0) 1 [publicBody] = false
#guard check (fun k => k) labels capturedLabels 0 [⟨1,.state 2⟩] = false
end Controls

#print axioms single_success
#print axioms single_failure
#print axioms preserves_integer_values
#print axioms completion_low
#print axioms check_exact
#print axioms environment_low
#print axioms two_run
#print axioms captures_retained
end MirroreaProofFirst.OwnerSourceFlow
