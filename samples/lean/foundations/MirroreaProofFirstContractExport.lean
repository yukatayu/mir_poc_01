import MirroreaProofFirstLocalContract
namespace MirroreaProofFirst.ContractExport
open ResourceBoundary LocalContract
inductive Profile where
 | checkedValue
 | symbolicNonnegative
 deriving DecidableEq, Repr
structure Binding where
 profile : Profile
 theoryVersion : Nat
 contractVersion : Nat
 instanceId : Nat
 generation : Nat
 principal : Nat
 request : Nat
 code : Term
 arguments : List Int
 assumptions : List Term
 deriving DecidableEq, Repr
structure Envelope where
 binding : Binding
 result : Int
 base : Term
 certificate : Certificate
 deriving DecidableEq, Repr

def scopeCheck (n : Nat) : Term → Bool
 | .input i => decide (i < n)
 | .integer _ => true
 | .add a b | .mul a b => scopeCheck n a && scopeCheck n b
 | .square a => scopeCheck n a
inductive Scoped (n : Nat) : Term → Prop where
 | input {i} : i < n → Scoped n (.input i)
 | integer (z) : Scoped n (.integer z)
 | add {a b} : Scoped n a → Scoped n b → Scoped n (.add a b)
 | mul {a b} : Scoped n a → Scoped n b → Scoped n (.mul a b)
 | square {a} : Scoped n a → Scoped n (.square a)
theorem scoped_exact (n : Nat) (e : Term) : scopeCheck n e = true ↔ Scoped n e := by
 induction e with
 | input i =>
   simp only [scopeCheck,decide_eq_true_eq]; constructor
   · exact Scoped.input
   · intro h; cases h; assumption
 | integer z => simp only [scopeCheck]; exact ⟨fun _ => .integer z,fun _ => True.intro⟩
 | add a b ha hb =>
   simp only [scopeCheck,Bool.and_eq_true,ha,hb]; constructor
   · rintro ⟨a,b⟩; exact .add a b
   · intro h; cases h; constructor <;> assumption
 | mul a b ha hb =>
   simp only [scopeCheck,Bool.and_eq_true,ha,hb]; constructor
   · rintro ⟨a,b⟩; exact .mul a b
   · intro h; cases h; constructor <;> assumption
 | square a ha =>
   simp only [scopeCheck,ha]; constructor
   · exact Scoped.square
   · intro h; cases h; assumption

def inputs (b : Binding) : Nat → Int := fun i => b.arguments[i]?.getD 0
-- Missing argument is rejected by scope checking. The total evaluator's default
-- is not an author-facing absent-value semantics.
def commonCheck (expected : Binding) (e : Envelope) : Bool :=
 decide (e.binding = expected ∧ expected.theoryVersion = 1 ∧ expected.contractVersion = 1 ∧
         e.result = eval (inputs expected) expected.code) &&
 scopeCheck expected.arguments.length expected.code &&
 expected.assumptions.all (scopeCheck expected.arguments.length) &&
 assumptionsCheck (inputs expected) expected.assumptions

def profileCheck (expected : Binding) (e : Envelope) : Bool :=
 match expected.profile with
 | .checkedValue => decide (0 < e.result)
 | .symbolicNonnegative =>
   decide (expected.code = positiveTerm e.base) &&
   LocalContract.check expected.assumptions e.base e.certificate

def accept (expected : Binding) (e : Envelope) : Option Nat :=
 if commonCheck expected e && profileCheck expected e then some e.result.toNat else none

structure Common (b : Binding) (e : Envelope) : Prop where
 binding : e.binding = b
 theory : b.theoryVersion = 1
 contract : b.contractVersion = 1
 result : Denotes (inputs b) b.code e.result
 scope : Scoped b.arguments.length b.code
 assumptionScope : ∀ t, t ∈ b.assumptions → Scoped b.arguments.length t
 assumptions : AssumptionsHold (inputs b) b.assumptions

theorem common_exact (b : Binding) (e : Envelope) : commonCheck b e = true ↔ Common b e := by
 simp only [commonCheck,Bool.and_eq_true,decide_eq_true_eq,scoped_exact,List.all_eq_true,assumptions_exact]
 constructor
 · rintro ⟨⟨⟨⟨he,hv,hc,hr⟩,hs⟩,has⟩,ha⟩
   exact ⟨he,hv,hc,(eval_exact _ _ _).mp hr.symm,hs,has,ha⟩
 · intro h
   exact ⟨⟨⟨⟨h.binding,h.theory,h.contract,(denotes_eval h.result).symm⟩,h.scope⟩,
     h.assumptionScope⟩,h.assumptions⟩

theorem profile_positive {b : Binding} {e : Envelope} (common : Common b e)
 (checked : profileCheck b e = true) : 0 < e.result := by
 cases hp : b.profile with
 | checkedValue => simpa [profileCheck,hp] using checked
 | symbolicNonnegative =>
   simp only [profileCheck,hp,Bool.and_eq_true,decide_eq_true_eq] at checked
   have h := positive_result checked.2 ((assumptions_exact _ _).mpr common.assumptions)
   have value := denotes_eval common.result
   rw [checked.1] at value
   omega

structure PositiveExport (b : Binding) (e : Envelope) (n : Nat) : Prop where
 common : Common b e
 positive : 0 < e.result
 length : n = e.result.toNat

theorem accept_sound {b : Binding} {e : Envelope} {n : Nat} (h : accept b e = some n) : PositiveExport b e n := by
 unfold accept at h; split at h
 · rename_i checks
   have both : commonCheck b e = true ∧ profileCheck b e = true := by
     simpa only [Bool.and_eq_true] using checks
   have hc := both.1
   have hp := both.2
   have common := (common_exact b e).mp hc
   cases h
   exact ⟨common,profile_positive common hp,rfl⟩
 · contradiction

theorem accept_bound {b : Binding} {e : Envelope} {n : Nat} (h : accept b e = some n) :
 0 < n ∧ n = (eval (inputs b) b.code).toNat := by
 have sound := accept_sound h
 have positive := sound.positive
 rw [sound.length]
 constructor
 · omega
 · rw [denotes_eval sound.common.result]

theorem accepted_integer_length {b : Binding} {e : Envelope} {n : Nat}
 (h : accept b e = some n) : (Int.ofNat n) = eval (inputs b) b.code := by
 have hs := accept_sound h
 have positive := hs.positive
 have hn := hs.length
 have value := denotes_eval hs.common.result
 rw [hn,value]
 exact Int.toNat_of_nonneg (by omega)

theorem checked_profile_complete {b : Binding} {e : Envelope}
 (common : Common b e) (profile : b.profile = .checkedValue) (positive : 0 < e.result) :
 accept b e = some e.result.toNat := by
 have hc := (common_exact b e).mpr common
 simp [accept,hc,profileCheck,profile,positive]

theorem symbolic_profile_complete {b : Binding} {e : Envelope}
 (common : Common b e) (profile : b.profile = .symbolicNonnegative)
 (code : b.code = positiveTerm e.base) (derivation : Nonnegative b.assumptions e.base) :
 ∃ c, accept b {e with certificate := c} = some e.result.toNat := by
 obtain ⟨c,hc⟩ := check_complete derivation
 have cc : Common b {e with certificate := c} :=
   ⟨common.binding,common.theory,common.contract,common.result,common.scope,common.assumptionScope,common.assumptions⟩
 have ce := (common_exact b {e with certificate := c}).mpr cc
 exact ⟨c,by simp [accept,ce,profileCheck,profile,code,hc]⟩

-- Auth remains a separate current input and cannot be minted by export evidence.
def allocateExport (policy : Nat → Bool) (s : State) (b : Binding) (e : Envelope) : Option (State × List Handle) :=
 match accept b e with
 | none => none
 | some n => execute policy s (.allocate b.principal n)

theorem allocate_export_wf {policy : Nat → Bool} {s : State} (w : WF s)
 {b : Binding} {e : Envelope} {out : State × List Handle}
 (h : allocateExport policy s b e = some out) : WF out.1 := by
 unfold allocateExport at h; split at h
 · contradiction
 · exact execute_wf w h

theorem lawful_export_allocates {policy : Nat → Bool} {s : State} {b : Binding} {e : Envelope} {n : Nat}
 (accepted : accept b e = some n) (auth : policy b.principal = true) :
 allocateExport policy s b e = some (raw s (.allocate b.principal n)) := by
 have pos := (accept_bound accepted).1
 simp [allocateExport,accepted,execute,ResourceBoundary.check,auth,pos]

theorem export_cannot_grant {policy : Nat → Bool} {s : State} {b : Binding} {e : Envelope}
 (denied : policy b.principal = false) : allocateExport policy s b e = none := by
 unfold allocateExport; split
 · rfl
 · simp [execute,ResourceBoundary.check,denied]

namespace Controls
def b : Binding := ⟨.symbolicNonnegative,1,1,7,3,1,42,
 positiveTerm (.square (.input 0)),[-3],[]⟩
def e : Envelope := ⟨b,10,.square (.input 0),.square (.input 0)⟩
example : accept b e = some 10 := by decide
example : accept b {e with result := 20} = none := by decide
example : accept {b with generation := 4} e = none := by decide
example : accept {b with request := 43} e = none := by decide
example : accept b {e with binding := {b with theoryVersion := 2}} = none := by decide
example : accept {b with arguments := []} {e with binding := {b with arguments := []},result := 1} = none := by decide
-- The checked-value profile handles this positive instance without a symbolic derivation.
def dynamic : Binding := {b with profile := .checkedValue,code := .input 0,arguments := [4]}
example : accept dynamic {e with binding := dynamic,result := 4} = some 4 := by decide
-- The same syntax at a negative argument is rejected, not certified as a positive function.
example : accept {dynamic with arguments := [-4]}
 {e with binding := {dynamic with arguments := [-4]},result := -4} = none := by decide
example : allocateExport (fun _ => false) empty b e = none := by decide
example : (allocateExport (fun _ => true) empty b e).map
 (fun out => out.2.map (fun h => (h.id,h.region.lo,h.region.hi,h.region.holder))) =
 some [(0,0,10,1)] := by decide
example : (allocateExport (fun _ => true) empty dynamic {e with binding := dynamic,result := 4}).map
 (fun out => out.2.map (fun h => (h.id,h.region.lo,h.region.hi,h.region.holder))) =
 some [(0,0,4,1)] := by decide

end Controls
#print axioms scoped_exact
#print axioms common_exact
#print axioms accept_sound
#print axioms accept_bound
#print axioms accepted_integer_length
#print axioms checked_profile_complete
#print axioms symbolic_profile_complete
#print axioms allocate_export_wf
#print axioms lawful_export_allocates
#print axioms export_cannot_grant
end MirroreaProofFirst.ContractExport

-- Later unreviewed reference candidate; excluded from frozen Oracle packets.
namespace MirroreaProofFirst.ContractExport.RequestAllocation
open ResourceBoundary
-- Separately supplied current authorization. The callback must not be derived
-- from arithmetic evidence. Binding/authenticity and physical atomicity remain TCB.
def run (authorize : State → Binding → Action → Bool)
 (s : State) (b : Binding) (e : Envelope) : Option (State × List Handle) :=
 match accept b e with
 | none => none
 | some n => execute (fun _ => authorize s b (.allocate b.principal n))
     s (.allocate b.principal n)

theorem run_exact (authorize : State → Binding → Action → Bool)
 (s : State) (b : Binding) (e : Envelope) (out : State × List Handle) :
 run authorize s b e = some out ↔ ∃ n,
 accept b e = some n ∧ authorize s b (.allocate b.principal n) = true ∧
 out = raw s (.allocate b.principal n) := by
 unfold run
 cases ha : accept b e with
 | none => simp
 | some n =>
   have pos := (accept_bound ha).1
   simp [execute,ResourceBoundary.check,pos]
   intro _
   exact eq_comm

theorem sound {authorize : State → Binding → Action → Bool}
 {s : State} {b : Binding} {e : Envelope} {out : State × List Handle}
 (ok : run authorize s b e = some out) : ∃ n,
 PositiveExport b e n ∧ Int.ofNat n = LocalContract.eval (inputs b) b.code ∧
 authorize s b (.allocate b.principal n) = true ∧
 out = raw s (.allocate b.principal n) := by
 obtain ⟨n,accepted,auth,ho⟩ := (run_exact _ _ _ _ _).mp ok
 exact ⟨n,accept_sound accepted,accepted_integer_length accepted,auth,ho⟩

theorem wf {authorize : State → Binding → Action → Bool}
 {s : State} (w : WF s) {b : Binding} {e : Envelope} {out : State × List Handle}
 (ok : run authorize s b e = some out) : WF out.1 := by
 obtain ⟨n,accepted,auth,rfl⟩ := (run_exact _ _ _ _ _).mp ok
 exact allocate_wf w _ _ (accept_bound accepted).1

theorem denied {authorize : State → Binding → Action → Bool}
 {s : State} {b : Binding} {e : Envelope}
 (no : ∀ n, authorize s b (.allocate b.principal n) = false) :
 run authorize s b e = none := by
 unfold run
 split
 · rfl
 · simp [execute,ResourceBoundary.check,no]

theorem same_principal_policy (p : Nat → Bool) (b : Binding) (n m : Nat)
 (hn : 0 < n) (hm : 0 < m) :
 ResourceBoundary.check p empty (.allocate b.principal n) =
 ResourceBoundary.check p empty (.allocate b.principal m) := by
 simp [ResourceBoundary.check,hn,hm]

namespace Controls
open ContractExport.Controls
def exactGrant (_ : State) (binding : Binding) (a : Action) : Bool :=
 decide (binding = b ∧ a = .allocate b.principal 10)
example : (run exactGrant empty b e).map (fun o => o.2.map (fun h => h.region.hi)) = some [10] := by decide
example : run exactGrant empty {b with request := 43}
 {e with binding := {b with request := 43}} = none := by decide
-- A different valid result is not authorized by the 10-unit request grant.
def larger : Binding := {b with arguments := [4]}
def largerEnvelope : Envelope := {e with binding := larger,result := 17}
example : accept larger largerEnvelope = some 17 := by decide
example : run exactGrant empty larger largerEnvelope = none := by decide
-- Independently isolate length binding from equality of the Binding record.
def lengthGrant (_ : State) (_ : Binding) (a : Action) : Bool :=
 decide (a = .allocate b.principal 10)
example : run lengthGrant empty larger largerEnvelope = none := by decide
example : run (fun _ _ _ => false) empty b e = none := by decide
-- Original unary policy accepts both positive sizes. No deployed exploit claim.
example : (allocateExport (fun _ => true) empty larger largerEnvelope).map
 (fun o => o.2.map (fun h => h.region.hi)) = some [17] := by decide
end Controls
#print axioms run_exact
#print axioms sound
#print axioms wf
#print axioms denied
#print axioms same_principal_policy
end MirroreaProofFirst.ContractExport.RequestAllocation

-- Later unreviewed reference candidate; excluded from frozen Oracle packets.
namespace MirroreaProofFirst.ContractExport.CheckedArithmetic
open LocalContract

def InRange (lo hi z : Int) : Prop := lo ≤ z ∧ z ≤ hi
def checked (lo hi z : Int) : Option Int :=
 if lo ≤ z ∧ z ≤ hi then some z else none

def evaluate (lo hi : Int) (args : List Int) : Term → Option Int
 | .input i => do let z ← args[i]?; checked lo hi z
 | .integer z => checked lo hi z
 | .add a b => do let x ← evaluate lo hi args a; let y ← evaluate lo hi args b; checked lo hi (x+y)
 | .mul a b => do let x ← evaluate lo hi args a; let y ← evaluate lo hi args b; checked lo hi (x*y)
 | .square a => do let x ← evaluate lo hi args a; checked lo hi (x*x)

inductive Denotes (lo hi : Int) (args : List Int) : Term → Int → Prop where
 | input {i z} : args[i]? = some z → InRange lo hi z → Denotes lo hi args (.input i) z
 | integer {z} : InRange lo hi z → Denotes lo hi args (.integer z) z
 | add {a b x y} : Denotes lo hi args a x → Denotes lo hi args b y →
     InRange lo hi (x+y) → Denotes lo hi args (.add a b) (x+y)
 | mul {a b x y} : Denotes lo hi args a x → Denotes lo hi args b y →
     InRange lo hi (x*y) → Denotes lo hi args (.mul a b) (x*y)
 | square {a x} : Denotes lo hi args a x → InRange lo hi (x*x) →
     Denotes lo hi args (.square a) (x*x)

theorem checked_exact (lo hi x z : Int) :
 checked lo hi x = some z ↔ x = z ∧ InRange lo hi z := by
 constructor
 · intro hz
   unfold checked at hz
   split at hz
   · rename_i h
     cases hz
     exact ⟨rfl,h⟩
   · contradiction
 · rintro ⟨rfl,h⟩
   simp [checked,InRange] at h ⊢
   exact h

theorem bind_some {α β : Type} (a : Option α) (f : α → Option β) (y : β) :
 (do let x ← a; f x) = some y ↔ ∃ x, a = some x ∧ f x = some y := by
 cases a <;> simp

theorem exact (lo hi : Int) (args : List Int) (e : Term) (z : Int) :
 evaluate lo hi args e = some z ↔ Denotes lo hi args e z := by
 induction e generalizing z with
 | input i =>
   simp only [evaluate,bind_some,checked_exact]
   constructor
   · rintro ⟨x,h,rfl,range⟩; exact .input h range
   · intro h; cases h with | input h range => exact ⟨_,h,rfl,range⟩
 | integer x =>
   simp only [evaluate,checked_exact]
   constructor
   · rintro ⟨rfl,range⟩; exact .integer range
   · intro h; cases h with | integer range => exact ⟨rfl,range⟩
 | add a b ia ib =>
   simp only [evaluate,bind_some,checked_exact,ia,ib]
   constructor
   · rintro ⟨x,ha,y,hb,rfl,range⟩; exact .add ha hb range
   · intro h; cases h with | add ha hb range => exact ⟨_,ha,_,hb,rfl,range⟩
 | mul a b ia ib =>
   simp only [evaluate,bind_some,checked_exact,ia,ib]
   constructor
   · rintro ⟨x,ha,y,hb,rfl,range⟩; exact .mul ha hb range
   · intro h; cases h with | mul ha hb range => exact ⟨_,ha,_,hb,rfl,range⟩
 | square a ia =>
   simp only [evaluate,bind_some,checked_exact,ia]
   constructor
   · rintro ⟨x,ha,rfl,range⟩; exact .square ha range
   · intro h; cases h with | square ha range => exact ⟨_,ha,rfl,range⟩

theorem denotes_math {lo hi : Int} {args : List Int} {e : Term} {z : Int}
 (h : Denotes lo hi args e z) :
 LocalContract.eval (fun i => args[i]?.getD 0) e = z ∧
 Scoped args.length e ∧ InRange lo hi z := by
 induction h with
 | input lookup range =>
   refine ⟨by simp [LocalContract.eval,lookup],.input ?_,range⟩
   exact List.getElem?_eq_some_iff.mp lookup |>.1
 | integer range => exact ⟨rfl,.integer _,range⟩
 | add a b range ia ib => exact ⟨by simp [LocalContract.eval,ia.1,ib.1],.add ia.2.1 ib.2.1,range⟩
 | mul a b range ia ib => exact ⟨by simp [LocalContract.eval,ia.1,ib.1],.mul ia.2.1 ib.2.1,range⟩
 | square a range ia => exact ⟨by simp [LocalContract.eval,ia.1],.square ia.2.1,range⟩

theorem sound {lo hi : Int} {args : List Int} {e : Term} {z : Int}
 (h : evaluate lo hi args e = some z) :
 LocalContract.eval (fun i => args[i]?.getD 0) e = z ∧
 Scoped args.length e ∧ InRange lo hi z := denotes_math ((exact _ _ _ _ _).mp h)

-- Mathematical bounds at every syntax node, separate from machine execution.
def Bounds (lo hi : Int) (args : List Int) : Term → Prop
 | .input i => InRange lo hi (args[i]?.getD 0)
 | .integer z => InRange lo hi z
 | .add a b => Bounds lo hi args a ∧ Bounds lo hi args b ∧
     InRange lo hi (LocalContract.eval (fun i => args[i]?.getD 0) (.add a b))
 | .mul a b => Bounds lo hi args a ∧ Bounds lo hi args b ∧
     InRange lo hi (LocalContract.eval (fun i => args[i]?.getD 0) (.mul a b))
 | .square a => Bounds lo hi args a ∧
     InRange lo hi (LocalContract.eval (fun i => args[i]?.getD 0) (.square a))

theorem denotes_bounds {lo hi : Int} {args : List Int} {e : Term} {z : Int}
 (h : Denotes lo hi args e z) : Bounds lo hi args e := by
 induction h with
 | input lookup range => simpa [Bounds,lookup] using range
 | integer range => exact range
 | add ha hb range ia ib =>
   exact ⟨ia,ib,by simpa [LocalContract.eval,(denotes_math ha).1,(denotes_math hb).1] using range⟩
 | mul ha hb range ia ib =>
   exact ⟨ia,ib,by simpa [LocalContract.eval,(denotes_math ha).1,(denotes_math hb).1] using range⟩
 | square ha range ia =>
   exact ⟨ia,by simpa [LocalContract.eval,(denotes_math ha).1] using range⟩

theorem bounded_denotes {lo hi : Int} {args : List Int} {e : Term}
 (scope : Scoped args.length e) (bounds : Bounds lo hi args e) :
 Denotes lo hi args e (LocalContract.eval (fun i => args[i]?.getD 0) e) := by
 induction scope with
 | @input i hi =>
   apply Denotes.input
   · simp [LocalContract.eval,List.getElem?_eq_getElem hi]
   · exact bounds
 | integer z => exact .integer bounds
 | add ha hb ia ib => exact .add (ia bounds.1) (ib bounds.2.1) bounds.2.2
 | mul ha hb ia ib => exact .mul (ia bounds.1) (ib bounds.2.1) bounds.2.2
 | square ha ia => exact .square (ia bounds.1) bounds.2

theorem success_iff_scoped_bounds (lo hi : Int) (args : List Int) (e : Term) :
 evaluate lo hi args e = some (LocalContract.eval (fun i => args[i]?.getD 0) e) ↔
 Scoped args.length e ∧ Bounds lo hi args e := by
 constructor
 · intro h
   have d := (exact _ _ _ _ _).mp h
   exact ⟨(denotes_math d).2.1,denotes_bounds d⟩
 · rintro ⟨sc,bs⟩
   exact (exact _ _ _ _ _).mpr (bounded_denotes sc bs)

theorem accepted_machine_value {lo hi : Int} {b : Binding} {e : Envelope} {n : Nat}
 (accepted : accept b e = some n) (bounds : Bounds lo hi b.arguments b.code) :
 evaluate lo hi b.arguments b.code = some (Int.ofNat n) := by
 have scope := (accept_sound accepted).common.scope
 have h := (success_iff_scoped_bounds lo hi b.arguments b.code).mpr ⟨scope,bounds⟩
 rw [accepted_integer_length accepted]
 exact h

namespace Controls
def cancellation : Term := .add (.mul (.integer 2) (.integer 6)) (.integer (-3))
example : LocalContract.eval (fun _ => 0) cancellation = 9 := by rfl
example : evaluate (-10) 10 [] cancellation = none := by decide
example : evaluate (-20) 20 [] cancellation = some 9 := by decide
example : evaluate (-10) 10 [] (.input 0) = none := by rfl
example : LocalContract.eval (fun i => ([] : List Int)[i]?.getD 0) (.input 0) = 0 := by rfl
example : evaluate (-9223372036854775808) 9223372036854775807 [-3]
 (positiveTerm (.square (.input 0))) = some 10 := by decide
example : evaluate (-9223372036854775808) 9223372036854775807 [9223372036854775807]
 (positiveTerm (.square (.input 0))) = none := by decide
def wideCancellation : Term := .add (.mul (.input 0) (.integer 2)) (.integer (-9223372036854775799))
example : LocalContract.eval (fun _ => 4611686018427387904) wideCancellation = 9 := by decide
example : evaluate (-9223372036854775808) 9223372036854775807 [4611686018427387904]
 wideCancellation = none := by decide
end Controls
#print axioms checked_exact
#print axioms exact
#print axioms denotes_math
#print axioms sound
#print axioms denotes_bounds
#print axioms bounded_denotes
#print axioms success_iff_scoped_bounds
#print axioms accepted_machine_value
end MirroreaProofFirst.ContractExport.CheckedArithmetic

-- Later unreviewed failure-inclusive local flow candidate.
namespace MirroreaProofFirst.ContractExport.CheckedArithmetic
open LocalContract
-- Conditional local data-dependence boundary, not an adopted release policy.
def flowCheck (label : Nat → Nat) (observer : Nat) : Term → Bool
 | .input i => decide (label i ≤ observer)
 | .integer _ => true
 | .add a b | .mul a b => flowCheck label observer a && flowCheck label observer b
 | .square a => flowCheck label observer a
inductive Flows (label : Nat → Nat) (observer : Nat) : Term → Prop where
 | input {i} : label i ≤ observer → Flows label observer (.input i)
 | integer (z) : Flows label observer (.integer z)
 | add {a b} : Flows label observer a → Flows label observer b → Flows label observer (.add a b)
 | mul {a b} : Flows label observer a → Flows label observer b → Flows label observer (.mul a b)
 | square {a} : Flows label observer a → Flows label observer (.square a)
theorem flow_exact (label : Nat → Nat) (observer : Nat) (e : Term) :
 flowCheck label observer e = true ↔ Flows label observer e := by
 induction e with
 | input i =>
   simp only [flowCheck,decide_eq_true_eq]
   exact ⟨Flows.input,fun h => by cases h; assumption⟩
 | integer z =>
   simp only [flowCheck]
   exact ⟨fun _ => .integer z,fun _ => True.intro⟩
 | add a b ia ib =>
   simp only [flowCheck,Bool.and_eq_true,ia,ib]
   exact ⟨fun h => .add h.1 h.2,fun h => by cases h; constructor <;> assumption⟩
 | mul a b ia ib =>
   simp only [flowCheck,Bool.and_eq_true,ia,ib]
   exact ⟨fun h => .mul h.1 h.2,fun h => by cases h; constructor <;> assumption⟩
 | square a ia =>
   simp only [flowCheck,ia]
   exact ⟨Flows.square,fun h => by cases h; assumption⟩

theorem evaluate_low {label : Nat → Nat} {observer : Nat} {e : Term}
 (flow : Flows label observer e) (lo hi : Int) (a b : List Int)
 (same : ∀ i, label i ≤ observer → a[i]? = b[i]?) :
 evaluate lo hi a e = evaluate lo hi b e := by
 induction flow with
 | @input i low => simp [evaluate,same i low]
 | integer z => rfl
 | add ha hb ia ib => simp only [evaluate,ia,ib]
 | mul ha hb ia ib => simp only [evaluate,ia,ib]
 | square ha ia => simp only [evaluate,ia]

theorem checked_evaluate_low {label : Nat → Nat} {observer : Nat} {e : Term}
 (flow : flowCheck label observer e = true) (lo hi : Int) (a b : List Int)
 (same : ∀ i, label i ≤ observer → a[i]? = b[i]?) :
 evaluate lo hi a e = evaluate lo hi b e :=
 evaluate_low ((flow_exact _ _ _).mp flow) lo hi a b same

namespace FlowControls
def secretSquare : Term := positiveTerm (.square (.input 0))
example : flowCheck (fun _ => 1) 0 secretSquare = false := by decide
-- Even erasing successful payloads retains input-dependent completion.
example : (evaluate (-10) 10 [0] secretSquare).map (fun _ => ()) = some () ∧
 (evaluate (-10) 10 [4] secretSquare).map (fun _ => ()) = none := by decide
example : flowCheck (fun _ => 0) 0 secretSquare = true := by decide
end FlowControls
#print axioms flow_exact
#print axioms evaluate_low
#print axioms checked_evaluate_low
end MirroreaProofFirst.ContractExport.CheckedArithmetic

-- Later unreviewed finite-capacity reference extension; no production acceptance.
namespace MirroreaProofFirst.ContractExport.CheckedAllocation
open ResourceBoundary

def run (limits : BoundedIdentifiers.Limits) (lo hi : Int)
 (authorize : State → Binding → Action → Bool)
 (s : State) (b : Binding) (e : Envelope) : Option (State × List Handle) :=
 (CheckedArithmetic.evaluate lo hi b.arguments b.code).bind fun z =>
 if z = e.result then
   match accept b e with
   | none => none
   | some n => BoundedIdentifiers.runOne limits (fun _ => authorize s b (.allocate b.principal n)) s (.allocate b.principal n)
 else none

theorem run_exact (limits : BoundedIdentifiers.Limits) (lo hi : Int)
 (authorize : State → Binding → Action → Bool)
 (s : State) (b : Binding) (e : Envelope) (out : State × List Handle) :
 run limits lo hi authorize s b e = some out ↔
 CheckedArithmetic.evaluate lo hi b.arguments b.code = some e.result ∧
 ∃ n, accept b e = some n ∧ authorize s b (.allocate b.principal n) = true ∧
 BoundedIdentifiers.Capacity limits s (.allocate b.principal n) ∧
 out = raw s (.allocate b.principal n) := by
 unfold run
 rw [Option.bind_eq_some_iff]
 cases ha : accept b e with
 | none => simp
 | some n =>
   have pos := (accept_bound ha).1
   simp [BoundedIdentifiers.runOne,
     execute,ResourceBoundary.check,pos,BoundedIdentifiers.capacity_exact]
   constructor
   · rintro ⟨z,hz,he,cap,auth,ho⟩
     subst z
     exact ⟨hz,auth,cap,ho.symm⟩
   · rintro ⟨hz,auth,cap,ho⟩
     exact ⟨_,hz,rfl,cap,auth,ho.symm⟩

theorem accepted_result {b : Binding} {e : Envelope} {n : Nat}
 (accepted : accept b e = some n) : Int.ofNat n = e.result := by
 rw [accepted_integer_length accepted]
 exact LocalContract.denotes_eval (accept_sound accepted).common.result

theorem lawful_completes {limits : BoundedIdentifiers.Limits} {lo hi : Int}
 {authorize : State → Binding → Action → Bool} {s : State} {b : Binding} {e : Envelope} {n : Nat}
 (accepted : accept b e = some n)
 (bounds : CheckedArithmetic.Bounds lo hi b.arguments b.code)
 (auth : authorize s b (.allocate b.principal n) = true)
 (capacity : BoundedIdentifiers.Capacity limits s (.allocate b.principal n)) :
 run limits lo hi authorize s b e = some (raw s (.allocate b.principal n)) := by
 apply (run_exact _ _ _ _ _ _ _ _).mpr
 have h := CheckedArithmetic.accepted_machine_value accepted bounds
 rw [accepted_result accepted] at h
 exact ⟨h,n,accepted,auth,capacity,rfl⟩

theorem successful {limits : BoundedIdentifiers.Limits} {lo hi : Int}
 {authorize : State → Binding → Action → Bool} {s : State} (w : WF s)
 {b : Binding} {e : Envelope} {out : State × List Handle}
 (ok : run limits lo hi authorize s b e = some out) :
 ∃ n, PositiveExport b e n ∧ Int.ofNat n = e.result ∧
 CheckedArithmetic.Denotes lo hi b.arguments b.code e.result ∧
 authorize s b (.allocate b.principal n) = true ∧
 WF out.1 ∧ BoundedIdentifiers.Within limits out.1 ∧
 out = raw s (.allocate b.principal n) := by
 obtain ⟨machine,n,accepted,auth,cap,rfl⟩ := (run_exact _ _ _ _ _ _ _ _).mp ok
 exact ⟨n,accept_sound accepted,accepted_result accepted,
   (CheckedArithmetic.exact _ _ _ _ _).mp machine,auth,
   allocate_wf w _ _ (accept_bound accepted).1,
   (BoundedIdentifiers.raw_within _ _ _).mpr cap,rfl⟩

namespace Controls
open ContractExport.Controls
example : (run ⟨1,1⟩ (-20) 20 RequestAllocation.Controls.exactGrant empty b e).map
 (fun out => out.2.map (fun h => (h.id,h.region.hi,h.region.holder))) = some [(0,10,1)] := by decide
example : run ⟨0,1⟩ (-20) 20 RequestAllocation.Controls.exactGrant empty b e = none := by decide
example : run ⟨1,1⟩ (-5) 5 RequestAllocation.Controls.exactGrant empty b e = none := by decide
example : run ⟨1,1⟩ (-20) 20 (fun _ _ _ => false) empty b e = none := by decide
def cancellationBinding : Binding := {b with profile := .checkedValue, code := CheckedArithmetic.Controls.wideCancellation, arguments := [4611686018427387904]}
def cancellationEnvelope : Envelope := {e with binding := cancellationBinding,result := 9}
example : accept cancellationBinding cancellationEnvelope = some 9 := by decide
example : run ⟨1,1⟩ (-9223372036854775808) 9223372036854775807 (fun _ _ _ => true)
 empty cancellationBinding cancellationEnvelope = none := by decide
example : run ⟨1,1⟩ (-20) 20 RequestAllocation.Controls.exactGrant empty b {e with result := 11} = none := by decide
end Controls
#print axioms run_exact
#print axioms lawful_completes
#print axioms successful
end MirroreaProofFirst.ContractExport.CheckedAllocation
