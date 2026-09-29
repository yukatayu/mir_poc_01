import MixedNamedOwnerSource
namespace MirroreaProofFirst.MixedNamedOwnerSource
open OwnerCheckedArithmetic ReferenceSourceData

theorem captured_member (h : Captured values labels names cs) (member : (name,c) ∈ cs) :
 integer values name = some c.value ∧ labels.lookup name = some c.label := by
 induction h with
 | nil => simp at member
 | cons hv hl ht ih =>
   rcases List.mem_cons.mp member with same | later
   · cases same; exact ⟨hv,hl⟩
   · exact ih later

theorem captured_names (h : Captured values labels names cs) : cs.map Prod.fst = names := by
 induction h with
 | nil => rfl
 | cons _ _ _ ih => simp [ih]

theorem locate_of_mem (h : name ∈ cs.map Prod.fst) : ∃ i c, LocatedCapture cs name i c := by
 induction cs with
 | nil => simp at h
 | cons head rest ih =>
   obtain ⟨other,c⟩ := head
   by_cases same : name = other
   · subst name; exact ⟨0,c,.here⟩
   · have later : name ∈ rest.map Prod.fst := by simpa [same] using h
     obtain ⟨i,c,found⟩ := ih later
     exact ⟨i+1,c,.there same found⟩

theorem located_member (h : LocatedCapture cs name i c) : (name,c) ∈ cs := by
 induction h with
 | here => exact List.mem_cons_self
 | there _ _ ih => exact List.mem_cons_of_mem _ ih

theorem captured_argument (h : Captured values labels names cs) (member : name ∈ names) :
 namedArgument cs name = integer values name := by
 have hn : name ∈ cs.map Prod.fst := (captured_names h).symm ▸ member
 obtain ⟨i,c,found⟩ := locate_of_mem hn
 have actual := captured_member h (located_member found)
 simp [namedArgument,locate_exact.mpr found,actual.1]

-- The general named evaluator really consumes current SOURCE values. Its state
-- reads remain deferred owner reads; no source-side evaluation of those cells.
theorem parameters_congr (tree : Checked K String) (ops : FallibleFlow.Arithmetic)
 (store : K → Option Int) (left right : String → Option Int)
 (same : ∀ name ∈ parameters tree, left name = right name) :
 evaluate ops store left tree = evaluate ops store right tree := by
 induction tree with
 | integer => rfl
 | state => rfl
 | parameter name => exact same name (by simp [parameters])
 | add a b ia ib =>
   rw [evaluate,evaluate,ia (fun name mem => same name (by simp [parameters,mem])),
     ib (fun name mem => same name (by simp [parameters,mem]))]
 | sub a b ia ib =>
   rw [evaluate,evaluate,ia (fun name mem => same name (by simp [parameters,mem])),
     ib (fun name mem => same name (by simp [parameters,mem]))]

theorem source_evaluation (control : Nat) (accepted : elaborate ctx values labels control source = some plan)
 (ops : FallibleFlow.Arithmetic) (store : Nat → Option Int) :
 evaluate ops store (argument plan.captures) plan.operation.body.tree =
 evaluate ops (fun name => (field ctx name).bind store) (integer values) source.rhs := by
 obtain ⟨target,tree,cs,_,hc,hr,_,rfl⟩ := elaborate_exact control |>.mp accepted
 change evaluate ops store (argument cs) tree = _
 rw [evaluation hr]
 exact parameters_congr _ _ _ _ _ (fun name member => captured_argument hc member)

theorem source_payload (control : Nat)
 (accepted : elaborate ctx values labels control source = some plan) :
 plan.site = source.site ∧ FieldAt ctx source.target plan.operation.body.target ∧
 MixedOperationDefinitions.OwnerSatisfies plan.operation ∧
 plan.operation.contract.fields = ctx.metadata ∧ plan.operation.contract.control = control ∧
 plan.operation.contract.arguments = plan.captures.map (fun row => row.2.label) ∧
 ∀ name c, (name,c) ∈ plan.captures →
   integer values name = some c.value ∧ labels.lookup name = some c.label := by
 obtain ⟨target,tree,cs,hf,hc,_,valid,rfl⟩ := (elaborate_exact control).mp accepted
 exact ⟨rfl,hf,valid,rfl,rfl,rfl,fun _ _ member => captured_member hc member⟩

-- Kind checking rejects references rather than coercing their numeric keys.
theorem reference_not_capture (reference : lookup values name = some (.reference key)) :
 integer values name = none := by simp [integer,reference]

namespace Controls
def ctx : Fields := ⟨[("health",0),("alias",0),("foreign",1)],[(0,(7,0)),(1,(8,0))]⟩
def values : Values := [("amount",.plain (.integer 5 false))]
def labels := [("amount",0)]
def source : Assignment := ⟨⟨"ordinary.mir",42⟩,"health",.add (.state "alias") (.parameter "amount")⟩
def plan := elaborate ctx values labels 0 source
#guard plan.isSome
#guard (plan.map fun p => p.site) = some source.site
#guard (plan.map fun p => p.operation.body.target) = some 0
#guard (plan.map fun p => p.operation.body.tree) = some (.add (.state 0) (.parameter 0))
#guard (plan.bind fun p => evaluate (FallibleFlow.signed 63) (fun _ => some 10)
 (argument p.captures) p.operation.body.tree) = some 15
-- The old captured five remains five even if the local source value changes.
#guard (plan.bind fun p => argument p.captures 0) = some 5
#guard (elaborate ctx [("amount",.plain (.integer 99 true))] labels 0 source |>.bind fun p => argument p.captures 0) = some 99
#guard !(elaborate ctx values [("amount",1)] 0 source).isSome
#guard !(elaborate ctx values [] 0 source).isSome
#guard !(elaborate ctx [] labels 0 source).isSome
#guard !(elaborate ctx [("amount",.reference 5)] labels 0 source).isSome
#guard !(elaborate ctx values labels 0 {source with rhs := .state "foreign"}).isSome
#guard !(elaborate {ctx with metadata := [(1,(8,0))]} values labels 0 source).isSome
#guard !(elaborate ctx values labels 0 {source with target := "absent"}).isSome
#guard !(elaborate ctx values labels 0 {source with rhs := .integer 9223372036854775808}).isSome
-- Capture erasure would falsely allow the secret to flow into a public target.
#guard (elaborate ctx values [("amount",1)] 0 {source with rhs := .integer 5}).isSome
#guard !(elaborate ctx values labels 1 source).isSome
-- Secret-to-secret is a positive, not an all-secret-rejection policy.
#guard (elaborate {ctx with metadata := [(0,(7,1)),(1,(8,0))]} values [("amount",1)] 1 source).isSome
-- Static construction is not target liveness: raw evaluator can return a
-- constant for an absent cell; the owner entry must check materialization.
#guard (elaborate ctx values labels 0 {source with rhs := .integer 5}).isSome
#guard (elaborate ctx values labels 0 {source with rhs := .integer 5} |>.bind fun p =>
 evaluate (FallibleFlow.signed 63) (fun _ => none) (argument p.captures) p.operation.body.tree) = some 5
end Controls
#print axioms captured_member
#print axioms captured_argument
#print axioms source_evaluation
#print axioms source_payload
#print axioms reference_not_capture
end MirroreaProofFirst.MixedNamedOwnerSource
