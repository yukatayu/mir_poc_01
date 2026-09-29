import MixedNamedOwnerSourceEvidence
import MixedOwnerMaterialization
namespace MirroreaProofFirst.MixedOwnerSourceFootprint
open OwnerCheckedArithmetic ReferenceSourceData MixedNamedOwnerSource

def keys : Checked Nat J → List Nat
 | .state k => [k]
 | .parameter _ | .integer _ => []
 | .add a b | .sub a b => keys a ++ keys b

def fields (ctx : Fields) (target : Nat) (rhs : Checked Nat Nat) : List (Nat × (Nat × Nat)) :=
 ctx.metadata.filter fun row => (target::keys rhs).contains row.1

theorem fields_exact : (key,info) ∈ fields ctx target rhs ↔
 (key,info) ∈ ctx.metadata ∧ (key=target ∨ key ∈ keys rhs) := by simp [fields]

theorem filter_lookup (rows : List (Nat × (Nat × Nat))) (keep : List Nat) (present : key ∈ keep) :
 (rows.filter fun row => keep.contains row.1).lookup key = rows.lookup key := by
 induction rows with
 | nil => rfl
 | cons row rest ih =>
   obtain ⟨other,info⟩ := row
   by_cases same : key=other
   · subst key; simp [present]
   · have ne : (key == other) = false := beq_eq_false_iff_ne.mpr same
     by_cases included : other ∈ keep <;> simp [included,ne,List.lookup_cons] <;> simpa using ih

theorem relevant_lookup (relevant : key=target ∨ key ∈ keys rhs) :
 (fields ctx target rhs).lookup key = ctx.metadata.lookup key :=
 filter_lookup _ _ (by simpa using relevant)

def definition (ctx : Fields) (cs : Captures) (control target : Nat) (rhs : Checked Nat Nat) :
 MixedOperationDefinitions.OwnerDefinition :=
 ⟨⟨target,rhs⟩,⟨fields ctx target rhs,cs.map (fun row => row.2.label),control⟩⟩

-- Only the capture layout is deduplicated. Values/labels remain paired; code
-- never receives captured values by literal substitution. No new syntax chosen.
def elaborate (ctx : Fields) (values : Values) (labels : List (String × Nat))
 (control : Nat) (source : Assignment) : Option Plan := do
 let target ← field ctx source.target
 let cs ← captures values labels (parameters source.rhs).eraseDups
 let tree ← resolve ctx cs source.rhs
 let op := definition ctx cs control target tree
 if MixedOperationDefinitions.ownerCheck op then some ⟨source.site,op,cs⟩ else none

def Elaborates (ctx : Fields) (values : Values) (labels : List (String × Nat))
 (control : Nat) (source : Assignment) (plan : Plan) : Prop :=
 ∃ target tree cs, FieldAt ctx source.target target ∧
 Captured values labels (parameters source.rhs).eraseDups cs ∧ Resolves ctx cs source.rhs tree ∧
 MixedOperationDefinitions.OwnerSatisfies (definition ctx cs control target tree) ∧
 plan = ⟨source.site,definition ctx cs control target tree,cs⟩

theorem elaborate_exact (control : Nat) : elaborate ctx values labels control source = some plan ↔
 Elaborates ctx values labels control source plan := by
 constructor
 · intro h
   unfold elaborate at h
   cases hf : field ctx source.target with
   | none => simp [hf] at h
   | some target =>
     cases hc : captures values labels (parameters source.rhs).eraseDups with
     | none => simp [hf,hc] at h
     | some cs =>
       cases hr : resolve ctx cs source.rhs with
       | none => simp [hf,hc,hr] at h
       | some tree =>
         simp only [hf,hc,hr,Option.bind_eq_bind,Option.bind_some] at h
         split at h
         · exact ⟨target,tree,cs,field_exact.mp hf,captures_exact.mp hc,resolve_sound hr,
             MixedOperationDefinitions.owner_check_exact.mp ‹_›,(Option.some.inj h).symm⟩
         · cases h
 · rintro ⟨target,tree,cs,hf,hc,hr,valid,rfl⟩
   simp [elaborate,field_exact.mpr hf,captures_exact.mpr hc,resolve_complete hr,
     MixedOperationDefinitions.owner_check_exact.mpr valid]

theorem source_evaluation (control : Nat) (accepted : elaborate ctx values labels control source = some plan)
 (ops : FallibleFlow.Arithmetic) (store : Nat → Option Int) :
 evaluate ops store (argument plan.captures) plan.operation.body.tree =
 evaluate ops (fun name => (field ctx name).bind store) (integer values) source.rhs := by
 obtain ⟨target,tree,cs,_,hc,hr,_,rfl⟩ := (elaborate_exact control).mp accepted
 change evaluate ops store (argument cs) tree = _
 rw [evaluation hr]
 exact parameters_congr _ _ _ _ _ (fun name member => captured_argument hc (by simpa using member))

theorem source_payload (control : Nat) (accepted : elaborate ctx values labels control source = some plan) :
 plan.site = source.site ∧ FieldAt ctx source.target plan.operation.body.target ∧
 MixedOperationDefinitions.OwnerSatisfies plan.operation ∧
 plan.operation.contract.control = control ∧
 plan.operation.contract.arguments = plan.captures.map (fun row => row.2.label) ∧
 ∀ name c, (name,c) ∈ plan.captures →
 integer values name = some c.value ∧ labels.lookup name = some c.label := by
 obtain ⟨target,tree,cs,hf,hc,_,valid,rfl⟩ := (elaborate_exact control).mp accepted
 exact ⟨rfl,hf,valid,rfl,rfl,fun _ _ member => captured_member hc member⟩

theorem no_extra_fields (control : Nat) (accepted : elaborate ctx values labels control source = some plan)
 (present : (key,info) ∈ plan.operation.contract.fields) :
 (key,info) ∈ ctx.metadata ∧ (key=plan.operation.body.target ∨ key ∈ keys plan.operation.body.tree) := by
 obtain ⟨target,tree,cs,_,_,_,_,rfl⟩ := (elaborate_exact control).mp accepted
 exact fields_exact.mp present

theorem metadata_change_irrelevant (control : Nat) (same : ∀ key, key=target ∨ key ∈ keys rhs → old key = next key) :
 MixedOwnerMaterialization.fieldsCheck (definition ctx cs control target rhs) old =
 MixedOwnerMaterialization.fieldsCheck (definition ctx cs control target rhs) next := by
 apply Bool.eq_iff_iff.mpr
 simp only [MixedOwnerMaterialization.fields_exact,MixedOwnerMaterialization.FieldsAt]
 constructor
 · intro allFields key info member
   rw [←same key (fields_exact.mp member).2]
   exact allFields key info member
 · intro allFields key info member
   rw [same key (fields_exact.mp member).2]
   exact allFields key info member

theorem eraseDups_nodup (xs : List String) : xs.eraseDups.Nodup := by
 match xs with
 | [] => simp
 | x::xs =>
   rw [List.eraseDups_cons,List.nodup_cons]
   refine ⟨?_,eraseDups_nodup (xs.filter fun b => !b == x)⟩
   simp
termination_by xs.length
decreasing_by
 have bound := List.length_filter_le (fun b : String => !b == x) xs
 simp only [List.length_cons]
 omega

theorem captures_names_unique (control : Nat) (accepted : elaborate ctx values labels control source = some plan) :
 (plan.captures.map Prod.fst).Nodup := by
 obtain ⟨target,tree,cs,_,hc,_,_,rfl⟩ := (elaborate_exact control).mp accepted
 rw [captured_names hc]
 exact eraseDups_nodup _

namespace Controls
open MixedNamedOwnerSource.Controls (ctx values labels source)
def broad := MixedNamedOwnerSource.elaborate ctx values labels 0 source
def small := elaborate ctx values labels 0 source
def onlyTarget (k : Nat) : Option (Nat × Nat) := if k=0 then some (7,0) else none
#guard broad.isSome && small.isSome
#guard (broad.map fun p => MixedOwnerMaterialization.fieldsCheck p.operation onlyTarget) = some false
#guard (small.map fun p => MixedOwnerMaterialization.fieldsCheck p.operation onlyTarget) = some true
#guard (small.map fun p => p.operation.contract.fields) = some [(0,(7,0))]
#guard (small.map fun p => p.operation.body.tree) = some (.add (.state 0) (.parameter 0))
#guard (small.bind fun p => evaluate (FallibleFlow.signed 63) (fun _ => some 10)
 (argument p.captures) p.operation.body.tree) = some 15
-- Repeated same named capture becomes one retained value/label slot.
def twice : Assignment := {source with rhs := .add (.parameter "amount") (.parameter "amount")}
#guard (MixedNamedOwnerSource.elaborate ctx values labels 0 twice |>.map fun p => p.captures.length) = some 2
#guard (elaborate ctx values labels 0 twice |>.map fun p => p.captures.length) = some 1
#guard (elaborate ctx values labels 0 twice |>.bind fun p =>
 evaluate (FallibleFlow.signed 63) (fun _ => none) (argument p.captures) p.operation.body.tree) = some 10
#guard !(elaborate ctx values [("amount",1)] 0 source).isSome
#guard (elaborate {ctx with metadata := [(0,(7,1)),(1,(8,0))]} values [("amount",1)] 1 source).isSome
#guard !(elaborate ctx values labels 0 {source with rhs := .state "foreign"}).isSome
end Controls
#print axioms source_payload
#print axioms relevant_lookup
#print axioms elaborate_exact
#print axioms source_evaluation
#print axioms no_extra_fields
#print axioms metadata_change_irrelevant
#print axioms captures_names_unique
end MirroreaProofFirst.MixedOwnerSourceFootprint
