import MirroreaProofFirstOwnerCheckedSchema
namespace MirroreaProofFirst.OwnerRequiredSchema
open OwnerCheckedSchema OwnerSourceContext OwnerStructuredKeys OwnerCheckedArithmetic

def retain (states : List StateDeclaration) (names : List String) : List StateDeclaration :=
 states.filter fun s => names.contains s.name

def needed (source : Assignment) : List String :=
 source.target.namespaceName :: (references source.rhs).map Read.namespaceName

-- Keep the complete declaration once a namespace is required. Do not filter by
-- owner first, hide duplicate declarations, or infer a classification/grant.
theorem filterTrue {α : Type} (xs : List α) : xs.filter (fun _ => true) = xs := by
 induction xs <;> simp_all [List.filter]
theorem filterFalse {α : Type} (xs : List α) : xs.filter (fun _ => false) = [] := by
 induction xs <;> simp_all [List.filter]

theorem projection_retain : project (retain states names) =
 (project states).filter (fun d => names.contains d.namespaceName) := by
 induction states with
 | nil => rfl
 | cons state rest ih =>
   by_cases member : state.name ∈ names
   · simpa [retain,project,List.filter_append,List.filter_map,Function.comp_def,row,member,filterTrue]
       using congrArg (fun tail => state.fields.map (row state) ++ tail) ih
   · simpa [retain,project,List.filter_append,List.filter_map,Function.comp_def,row,member,filterFalse]
       using ih

theorem candidates_retain (required : key.namespaceName ∈ names) :
 candidates (project (retain states names)) key = candidates (project states) key := by
 rw [projection_retain]
 unfold candidates
 rw [List.filter_filter]
 apply List.filter_congr
 intro d _
 by_cases same : d.namespaceName=key.namespaceName
 · simp [same,required]
 · simp [same]

theorem select_retain (required : key.namespaceName ∈ names) :
 select (project (retain states names)) key = select (project states) key := by
 unfold select
 rw [candidates_retain required]

theorem bind_retain {read : Read} (required : read.namespaceName ∈ names) :
 bind (project (retain states names)) current arguments read =
 bind (project states) current arguments read := by
 unfold OwnerSourceContext.bind
 cases materialized : materialize arguments read with
 | none => simp
 | some key =>
   have keyRequired : key.namespaceName ∈ names := by rw [namespace_retained materialized]; exact required
   simp only [Option.bind_eq_bind,Option.bind_some,select_retain keyRequired]

theorem tree_retain (source : Checked Read String)
 (required : ∀ read ∈ references source, read.namespaceName ∈ names) :
 bindTree (project (retain states names)) current arguments source =
 bindTree (project states) current arguments source := by
 induction source with
 | state read => simp only [bindTree,bind_retain (required read (by simp [references]))]
 | parameter | integer => rfl
 | add l r il ir | sub l r il ir =>
   have left := il (fun read member => required read (List.mem_append_left _ member))
   have right := ir (fun read member => required read (List.mem_append_right _ member))
   simp only [bindTree,left,right]

theorem assignment_retain :
 bindAssignment (project (retain states (needed source))) current arguments source =
 bindAssignment (project states) current arguments source := by
 have target : source.target.namespaceName ∈ needed source := by simp [needed]
 have reads : ∀ read ∈ references source.rhs, read.namespaceName ∈ needed source := by
   intro read member
   simp only [needed,List.mem_cons]
   exact Or.inr (List.mem_map.mpr ⟨read,member,rfl⟩)
 simp only [bindAssignment,bind_retain target,tree_retain source.rhs reads]

theorem assignment_relative_exact :
 bindAssignment (project (retain states (needed source))) current arguments source = some result ↔
 AssignmentBinds (project states) current arguments source result := by
 rw [assignment_retain,assignment_exact]

namespace Controls
open OwnerCheckedSchema.Controls (hp a b key)
def shield : StateDeclaration := ⟨"shield","id","Player","S",[hp]⟩
def unused : StateDeclaration := ⟨"unused","id","Player","S",[hp]⟩
def source : Assignment := ⟨⟨"player",some "self",some "hp","S"⟩,
 .add (.state ⟨"shield",some "self",some "hp","S"⟩) (.parameter "amount")⟩
#guard retain [a,shield,unused] (needed source) = [a,shield]
def current (k : Key) : Option Metadata := (select (project [a,shield,unused]) k).map fun d => ⟨d,1,3⟩
#guard (bindAssignment (project [a,shield]) current (fun _ => none) source).isSome
-- Existing target-only projection loses a real same-owner source dependency.
#guard (bindAssignment (project [a]) current (fun _ => none) source).isNone
#guard select (project (retain [a,{a with owner := "T"},unused] ["player"])) key = none
end Controls
#print axioms projection_retain
#print axioms candidates_retain
#print axioms select_retain
#print axioms bind_retain
#print axioms tree_retain
#print axioms assignment_retain
#print axioms assignment_relative_exact
end MirroreaProofFirst.OwnerRequiredSchema
