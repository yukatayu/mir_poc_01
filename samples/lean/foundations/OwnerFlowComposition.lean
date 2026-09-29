import OwnerSourceFlow
namespace MirroreaProofFirst.OwnerFlowComposition
open ProducerFlow (Value Ty rank)
open OwnerPartialAbort
open FallibleFlow (Arithmetic Outcome)
variable {K : Type} [DecidableEq K]

-- Same existing checker/judgment; no target-label inflation or pc reset at an
-- arithmetic fragment boundary. Service/auth/resource completion is additional.
omit [DecidableEq K] in
theorem safe_append (xs ys : Program K) :
 Safe G L pc (xs ++ ys) ↔ Safe G L pc xs ∧
 Safe G L (max pc (AbortFlow.completionLevel L xs)) ys := by
 induction xs generalizing pc with
 | nil =>
   simp only [List.nil_append,AbortFlow.completionLevel,Nat.max_zero]
   exact ⟨fun h => ⟨.nil,h⟩,fun h => h.2⟩
 | cons head tail ih =>
   obtain ⟨key,e⟩ := head
   constructor
   · intro h
     cases h with
     | cons typed flow control rest =>
       obtain ⟨before,after⟩ := ih.mp rest
       exact ⟨.cons typed flow control before,by simpa [AbortFlow.completionLevel,Nat.max_assoc] using after⟩
   · rintro ⟨before,after⟩
     cases before with
     | cons typed flow control rest =>
       exact .cons typed flow control (ih.mpr ⟨rest,by simpa [AbortFlow.completionLevel,Nat.max_assoc] using after⟩)

omit [DecidableEq K] in
theorem safe_take (h : Safe G L pc program) (count : Nat) : Safe G L pc (program.take count) := by
 induction h generalizing count with
 | nil => simp only [List.take_nil]; exact .nil
 | cons typed flow control rest ih =>
   cases count with
   | zero => exact .nil
   | succ n => exact .cons typed flow control (ih n)

theorem run_append (ops : Arithmetic) (s : K → Option Value) (xs ys : Program K) :
 run ops s (xs ++ ys) =
 let before := run ops s xs
 if AbortFlow.completed before.2 then
  let after := run ops before.1 ys
  (after.1,before.2 ++ after.2)
 else before := by
 induction xs generalizing s with
 | nil => simp [run,AbortFlow.completed]
 | cons head tail ih =>
   obtain ⟨key,e⟩ := head
   simp only [List.cons_append,run]
   cases evaluated : OwnerPartial.eval ops s e with
   | none => simp [AbortFlow.completed]
   | some v =>
     dsimp only
     rw [ih]
     simp only [AbortFlow.completed,List.all_cons,Bool.true_and]
     split <;> simp_all

theorem trace_take (ops : Arithmetic) (s : K → Option Value) (program : Program K) (count : Nat) :
 (run ops s program).2.take count = (run ops s (program.take count)).2 := by
 induction program generalizing s count with
 | nil => simp [run]
 | cons head tail ih =>
   obtain ⟨key,e⟩ := head
   cases count with
   | zero => rfl
   | succ n =>
     simp only [List.take_succ_cons,run]
     cases evaluated : OwnerPartial.eval ops s e with
     | none => simp
     | some value => simp [ih]

-- Instruction-prefix comparison, NOT independently timed network/polling cuts.
theorem prefixes_low (safe : Safe G L pc program) (ops : Arithmetic)
 (s t : K → Option Value) (low : LowEq L level s t) (count : Nat) :
 LowEq L level (run ops s (program.take count)).1 (run ops t (program.take count)).1 ∧
 visible L level ((run ops s program).2.take count) =
 visible L level ((run ops t program).2.take count) := by
 have h := noninterference (safe_take safe count) ops s t low
 rw [trace_take,trace_take]
 exact h

-- Reuse the existing checked tree, preserving live/capture namespaces and
-- Option presence. This is the actual environment relation at each list step.
theorem environment_put (store captured : Nat → Option Int) (key : Nat) (v : Int) :
 OwnerCheckedArithmetic.environment (OwnerEffectService.put store key v) captured =
 OwnerPartial.put (OwnerCheckedArithmetic.environment store captured) (.inl key) (.int v) := by
 funext ref
 cases ref with
 | inl k => by_cases same : k=key <;>
     simp [OwnerCheckedArithmetic.environment,OwnerEffectService.put,OwnerPartial.put,same]
 | inr j => simp [OwnerCheckedArithmetic.environment,OwnerPartial.put]

theorem execute_cons_success (evaluated : OwnerCheckedArithmetic.evaluate ops store captured body.tree = some v) :
 OwnerSourceFlow.execute ops store captured (body::rest) =
 let after := OwnerSourceFlow.execute ops (OwnerEffectService.put store body.target v) captured rest
 (after.1,.wrote (.inl body.target) (.int v) :: after.2) := by
 simp only [OwnerSourceFlow.execute,OwnerSourceFlow.lower,List.map_cons,run,
   OwnerCheckedArithmetic.extraction,evaluated,Option.map_some,environment_put]
theorem execute_cons_failure (evaluated : OwnerCheckedArithmetic.evaluate ops store captured body.tree = none) :
 OwnerSourceFlow.execute ops store captured (body::rest) =
 (OwnerCheckedArithmetic.environment store captured,[.failed (.inl body.target)]) := by
 simp only [OwnerSourceFlow.execute,OwnerSourceFlow.lower,List.map_cons,run,
   OwnerCheckedArithmetic.extraction,evaluated,Option.map_none]

namespace Controls
open OwnerSourceFlow.Controls
open OwnerSourceFlow OwnerEffectService
def commonStore (k : Nat) : Option Int := if k=0 then some 10 else some 1
def args (a : Option Int) (_ : Nat) := a
def runWith (a : Option Int) (bodies : List Body) := execute (FallibleFlow.signed 63) commonStore (args a) bodies
def viewWith (a : Option Int) (bodies : List Body) :=
 visible (OwnerAssignment.classes labels capturedLabels) 0 (runWith a bodies).2
#guard viewWith (some 0) safe = [.wrote (.inl 0) (.int 11)]
#guard viewWith (some (2^63-1)) safe = [.wrote (.inl 0) (.int 11)]
#guard viewWith none safe = [.wrote (.inl 0) (.int 11)]
#guard viewWith (some 0) leaky ≠ viewWith (some (2^63-1)) leaky
#guard viewWith (some 0) leaky ≠ viewWith none leaky
def highCapture : Body := ⟨1,.parameter 0⟩
def publicConstant : Body := ⟨0,.integer 7⟩
#guard check owners labels capturedLabels 0 [highCapture]
#guard check owners labels capturedLabels 0 [publicConstant]
#guard !check owners labels capturedLabels 0 [highCapture,publicConstant]
#guard viewWith (some 0) [highCapture,publicConstant] ≠ viewWith none [highCapture,publicConstant]
-- In this interpreter a constant write does not read the old target's presence.
#guard publicView none [constantPrivate,publicBody] = publicView (some 0) [constantPrivate,publicBody]
def aliasProgram : List Body := [⟨0,.integer 7⟩,⟨0,.parameter 0⟩]
#guard (execute (FallibleFlow.signed 63) (fun _ => some 1) (fun _ => some 5) aliasProgram).1 (.inl 0) = some (.int 5)
#guard (execute (FallibleFlow.signed 63) (fun _ => some 1) (fun _ => some 5) aliasProgram).1 (.inr 0) = some (.int 5)
def retentionProgram := [publicConstant,highCapture,constantPrivate]
#guard check owners labels capturedLabels 0 retentionProgram
def wrongRetain (a : Option Int) :=
 visible (OwnerAssignment.classes labels capturedLabels) 0 ((runWith a retentionProgram).2.drop ((runWith a retentionProgram).2.length-2))
#guard wrongRetain none ≠ wrongRetain (some 0)
def rightRetain (a : Option Int) :=
 Passive.feed (FallibleFlow.project (OwnerAssignment.classes labels capturedLabels) 0) 2 [] (runWith a retentionProgram).2
#guard rightRetain none = rightRetain (some 0)
#guard AbortFlow.completed (runWith none [highCapture]).2 = false
#guard AbortFlow.completed (viewWith none [highCapture]) = true
#guard completionObservation labels capturedLabels 0 (FallibleFlow.signed 63) commonStore (args none) [highCapture] = none
end Controls

#print axioms safe_append
#print axioms safe_take
#print axioms run_append
#print axioms trace_take
#print axioms prefixes_low
#print axioms environment_put
#print axioms execute_cons_success
#print axioms execute_cons_failure
end MirroreaProofFirst.OwnerFlowComposition
