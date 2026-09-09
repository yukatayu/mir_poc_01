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
