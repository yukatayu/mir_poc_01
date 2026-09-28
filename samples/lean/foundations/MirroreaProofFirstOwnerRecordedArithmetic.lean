import MirroreaProofFirstOwnerCheckedArithmetic
import MirroreaProofFirstOwnerReadReport
namespace MirroreaProofFirst.OwnerCheckedArithmetic.Recorded
variable {K J : Type}

-- Extraction study of the five strict Rust tree cases. The list retains read
-- multiplicity and order; the concrete BTreeMap quotient is a separate boundary.
def keys : Checked K J → List K
 | .state k => [k]
 | .parameter _ | .integer _ => []
 | .add l r | .sub l r => keys l ++ keys r

def actualReads (store : K → Option Int) (tree : Checked K J) : List (K × Int) :=
 (keys tree).filterMap fun k => (store k).map fun v => (k,v)

def evaluateRecorded (ops : FallibleFlow.Arithmetic) (store : K → Option Int)
 (args : J → Option Int) : Checked K J → Option (Int × List (K × Int))
 | .state k => do let v ← store k; return (v,[(k,v)])
 | .parameter j => do let v ← args j; return (v,[])
 | .integer n => some (n,[])
 | .add l r => do
     let (x,lr) ← evaluateRecorded ops store args l
     let (y,rr) ← evaluateRecorded ops store args r
     let v ← ops.add x y
     return (v,lr ++ rr)
 | .sub l r => do
     let (x,lr) ← evaluateRecorded ops store args l
     let (y,rr) ← evaluateRecorded ops store args r
     let v ← ops.sub x y
     return (v,lr ++ rr)

-- An independent static traversal retrieves values from the same immutable
-- service-time store. This equality includes failed/missing inputs, not just a
-- hypothesis equating the recorded reads to the desired reads.
theorem exact_recording (ops : FallibleFlow.Arithmetic) (store : K → Option Int)
 (args : J → Option Int) (tree : Checked K J) :
 evaluateRecorded ops store args tree =
 (evaluate ops store args tree).map (fun value => (value,actualReads store tree)) := by
 induction tree with
 | state k => cases h : store k <;> simp [evaluateRecorded,evaluate,actualReads,keys,h]
 | parameter j => cases h : args j <;> simp [evaluateRecorded,evaluate,actualReads,keys,h]
 | integer n => rfl
 | add l r hl hr =>
   simp only [evaluateRecorded,hl,hr,evaluate]
   cases left : evaluate ops store args l <;> cases right : evaluate ops store args r <;>
     simp [actualReads,keys,List.filterMap_append]
   case some.some x y => cases ops.add x y <;> rfl
 | sub l r hl hr =>
   simp only [evaluateRecorded,hl,hr,evaluate]
   cases left : evaluate ops store args l <;> cases right : evaluate ops store args r <;>
     simp [actualReads,keys,List.filterMap_append]
   case some.some x y => cases ops.sub x y <;> rfl

theorem success_parts (ops : FallibleFlow.Arithmetic) (store : K → Option Int)
 (args : J → Option Int) (tree : Checked K J) (value : Int) (reads : List (K × Int))
 (ok : evaluateRecorded ops store args tree = some (value,reads)) :
 evaluate ops store args tree = some value ∧ reads = actualReads store tree := by
 rw [exact_recording] at ok
 cases result : evaluate ops store args tree with
 | none => simp [result] at ok
 | some actual =>
   simp only [result,Option.map_some,Option.some.injEq,Prod.mk.injEq] at ok
   exact ⟨congrArg some ok.1,ok.2.symm⟩

theorem actualReads_exact (store : K → Option Int) (tree : Checked K J) (k : K) (v : Int) :
 (k,v) ∈ actualReads store tree ↔ k ∈ keys tree ∧ store k = some v :=
 OwnerReadReport.selected_exact (keys tree) store k v

theorem recorded_domain_exact (ops : FallibleFlow.Arithmetic) (store : K → Option Int)
 (args : J → Option Int) (tree : Checked K J) (value : Int) (reads : List (K × Int))
 (ok : evaluateRecorded ops store args tree = some (value,reads)) (k : K) (v : Int) :
 (k,v) ∈ reads ↔ k ∈ keys tree ∧ store k = some v := by
 rw [(success_parts _ _ _ _ _ _ ok).2,actualReads_exact]

theorem key_in_translation (tree : Checked K J) (k : K) (member : k ∈ keys tree) :
 Sum.inl k ∈ OwnerPartial.readKeys (translate tree) := by
 induction tree with
 | state key => simpa [keys,translate,OwnerPartial.readKeys] using member
 | parameter => simp [keys] at member
 | integer => simp [keys] at member
 | add l r hl hr =>
   cases List.mem_append.mp member with
   | inl h => exact List.mem_append_left _ (hl h)
   | inr h => exact List.mem_append_right _ (hr h)
 | sub l r hl hr =>
   cases List.mem_append.mp member with
   | inl h => exact List.mem_append_left _ (hl h)
   | inr h => exact List.mem_append_right _ (hr h)

theorem successful_state_coverage (ops : FallibleFlow.Arithmetic) (store : K → Option Int)
 (args : J → Option Int) (tree : Checked K J) (value : Int)
 (ok : evaluate ops store args tree = some value) (k : K) (member : k ∈ keys tree) :
 ∃ v, store k = some v := by
 have extracted : OwnerPartial.eval ops (environment store args) (translate tree) =
   some (.int value) := by rw [extraction,ok]; rfl
 obtain ⟨v,present⟩ := OwnerPartial.strict_success_covers ops _ _ (strict_translation tree)
   _ extracted (.inl k) (key_in_translation tree k member)
 cases found : store k with
 | none => simp [environment,found] at present
 | some actual => exact ⟨actual,rfl⟩

theorem repeated_key_same_value (store : K → Option Int) (tree : Checked K J)
 (k : K) (first second : Int) (left : (k,first) ∈ actualReads store tree)
 (right : (k,second) ∈ actualReads store tree) : first = second :=
 Option.some.inj (((actualReads_exact _ _ _ _).mp left).2.symm.trans
   ((actualReads_exact _ _ _ _).mp right).2)

-- Fixed examples are nonvacuity controls, not substitutes for the general laws.
example : evaluateRecorded (FallibleFlow.signed 63) (fun (_ : Nat) => some 0)
 (fun (_ : Nat) => none) (.add (.state 0) (.state 0)) = some (0,[(0,0),(0,0)]) := by decide
example : evaluateRecorded (FallibleFlow.signed 63) (fun (_ : Nat) => some 1)
 (fun (_ : Nat) => none) (.add (.state 0) (.parameter 0)) = none := by decide
#print axioms exact_recording
#print axioms success_parts
#print axioms actualReads_exact
#print axioms recorded_domain_exact
#print axioms successful_state_coverage
#print axioms repeated_key_same_value
end MirroreaProofFirst.OwnerCheckedArithmetic.Recorded
