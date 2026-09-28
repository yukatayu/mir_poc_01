import MirroreaProofFirstOwnerCheckedSchema
import MirroreaProofFirstOwnerRequiredSchema
namespace MirroreaProofFirst.OwnerTypedIndex
open OwnerCheckedSchema OwnerStructuredKeys

-- Nonproduction refinement for an explicitly declared index parameter. A
-- principal/literal index is a separate source form, not a missing parameter.
-- Neither a type tag nor a checked result authenticates entity ownership.
structure Parameter where
 name : String
 typeName : String
 deriving DecidableEq, Repr
structure Entity where
 typeName : String
 identity : String
 deriving DecidableEq, Repr
structure Binding where
 declaration : StateDeclaration
 parameter : Parameter
 entity : Entity
 deriving DecidableEq, Repr

def unique {α : Type} : List α → Option α
 | [value] => some value
 | _ => none
theorem unique_exact {α : Type} {xs : List α} {x : α} :
 unique xs = some x ↔ xs = [x] := by
 cases xs with
 | nil => simp [unique]
 | cons head tail => cases tail <;> simp [unique,eq_comm]

def declarations (states : List StateDeclaration) (name : String) :=
 states.filter fun s => s.name == name
def parameters (signature : List Parameter) (name : String) :=
 signature.filter fun p => p.name == name

def checkIndex (states : List StateDeclaration) (signature : List Parameter)
 (namespaceName index : String) : Option (StateDeclaration × Parameter) := do
 let state ← unique (declarations states namespaceName)
 let param ← unique (parameters signature index)
 if param.typeName = state.indexType then some (state,param) else none

def IndexTyped (states : List StateDeclaration) (signature : List Parameter)
 (namespaceName index : String) (state : StateDeclaration) (param : Parameter) : Prop :=
 declarations states namespaceName = [state] ∧
 parameters signature index = [param] ∧ param.typeName = state.indexType

theorem check_index_exact : checkIndex states signature namespaceName index = some (state,param) ↔
 IndexTyped states signature namespaceName index state param := by
 constructor
 · intro accepted
   unfold checkIndex at accepted
   cases found : unique (declarations states namespaceName) with
   | none => simp [found] at accepted
   | some declaration =>
     cases resolved : unique (parameters signature index) with
     | none => simp [found,resolved] at accepted
     | some parameter =>
       simp only [found,resolved,Option.bind_eq_bind,Option.bind_some] at accepted
       split at accepted
       · rename_i sameType
         cases accepted
         exact ⟨unique_exact.mp found,unique_exact.mp resolved,sameType⟩
       · cases accepted
 · rintro ⟨found,resolved,sameType⟩
   simp [checkIndex,found,resolved,unique,sameType]

-- Extension predicate for the existing finite source profile: a non-parameter
-- index remains outside this new check. True there is NOT a typing certificate.
def explicitCheck (states : List StateDeclaration) (signature : List Parameter)
 (namespaceName index : String) : Bool :=
 if parameters signature index = [] then true else
 (checkIndex states signature namespaceName index).isSome

def ExplicitlyCompatible (states : List StateDeclaration) (signature : List Parameter)
 (namespaceName index : String) : Prop :=
 parameters signature index = [] ∨
 ∃ state param, IndexTyped states signature namespaceName index state param

theorem explicit_exact : explicitCheck states signature namespaceName index = true ↔
 ExplicitlyCompatible states signature namespaceName index := by
 unfold explicitCheck ExplicitlyCompatible
 by_cases absent : parameters signature index = []
 · simp [absent]
 · simp only [absent,if_false,false_or]
   constructor
   · intro present
     cases result : checkIndex states signature namespaceName index with
     | none => simp [result] at present
     | some pair => exact ⟨pair.1,pair.2,check_index_exact.mp result⟩
   · rintro ⟨state,param,typed⟩
     simp [check_index_exact.mpr typed]

def explicitReadCheck (states : List StateDeclaration) (signature : List Parameter) (read : Read) : Bool :=
 match read.index with
 | none => true
 | some index => explicitCheck states signature read.namespaceName index
def ExplicitReadCompatible (states : List StateDeclaration) (signature : List Parameter) (read : Read) : Prop :=
 match read.index with
 | none => True
 | some index => ExplicitlyCompatible states signature read.namespaceName index

theorem read_exact {read : Read} : explicitReadCheck states signature read = true ↔
 ExplicitReadCompatible states signature read := by
 cases held : read.index <;> simp [explicitReadCheck,ExplicitReadCompatible,held,explicit_exact]

def footprintCheck (states : List StateDeclaration) (signature : List Parameter) (reads : List Read) : Bool :=
 reads.all (explicitReadCheck states signature)
def FootprintCompatible (states : List StateDeclaration) (signature : List Parameter) (reads : List Read) : Prop :=
 ∀ read ∈ reads, ExplicitReadCompatible states signature read
theorem footprint_exact : footprintCheck states signature reads = true ↔
 FootprintCompatible states signature reads := by
 simp [footprintCheck,FootprintCompatible,read_exact]

def bind (states : List StateDeclaration) (signature : List Parameter)
 (values : String → Option Entity) (namespaceName index : String) : Option Binding := do
 let state ← unique (declarations states namespaceName)
 let param ← unique (parameters signature index)
 let entity ← values index
 if param.typeName = state.indexType ∧ entity.typeName = param.typeName then
   some ⟨state,param,entity⟩
 else none

-- Independent premises over original declaration/capture inventories, including
-- multiplicity; no call to bind and no success bit taken as a premise.
def Binds (states : List StateDeclaration) (signature : List Parameter)
 (values : String → Option Entity) (namespaceName index : String) (out : Binding) : Prop :=
 declarations states namespaceName = [out.declaration] ∧
 parameters signature index = [out.parameter] ∧
 values index = some out.entity ∧
 out.parameter.typeName = out.declaration.indexType ∧
 out.entity.typeName = out.parameter.typeName

theorem bind_exact : bind states signature values namespaceName index = some out ↔
 Binds states signature values namespaceName index out := by
 constructor
 · intro accepted
   unfold bind at accepted
   cases state : unique (declarations states namespaceName) with
   | none => simp [state] at accepted
   | some declaration =>
     cases param : unique (parameters signature index) with
     | none => simp [state,param] at accepted
     | some parameter =>
       cases capture : values index with
       | none => simp [state,param,capture] at accepted
       | some entity =>
         simp only [state,param,capture,Option.bind_eq_bind,Option.bind_some] at accepted
         split at accepted
         · rename_i types
           cases accepted
           exact ⟨unique_exact.mp state,unique_exact.mp param,capture,types⟩
         · cases accepted
 · rintro ⟨state,param,capture,types⟩
   cases out
   simp_all [bind,unique]

theorem binding_names (bound : Binds states signature values namespaceName index out) :
 out.declaration.name = namespaceName ∧ out.parameter.name = index := by
 have smem : out.declaration ∈ declarations states namespaceName := by rw [bound.1]; simp
 have pmem : out.parameter ∈ parameters signature index := by rw [bound.2.1]; simp
 exact ⟨by simpa [declarations] using (List.mem_filter.mp smem).2,
   by simpa [parameters] using (List.mem_filter.mp pmem).2⟩

def erase (values : String → Option Entity) (name : String) : Option String :=
 (values name).map Entity.identity

-- The low-level substitution-or-literal behavior is unchanged. An admitted
-- explicit parameter always takes substitution, so fallback cannot choose a
-- coincidentally live entity named after the source variable.
theorem no_index_fallback
 (bound : Binds states signature values namespaceName index out) (field owner : String) :
 materialize (erase values) ⟨namespaceName,some index,some field,owner⟩ =
 some ⟨namespaceName,out.entity.identity,field⟩ := by
 simp [materialize,erase,bound.2.2.1]

theorem missing_capture_refused (absent : values index = none) :
 bind states signature values namespaceName index = none := by
 unfold bind
 cases unique (declarations states namespaceName) <;>
 cases unique (parameters signature index) <;> simp [absent]

theorem wrong_index_type_refused
 (state : declarations states namespaceName = [declaration])
 (param : parameters signature index = [parameter])
 (wrong : parameter.typeName ≠ declaration.indexType) :
 bind states signature values namespaceName index = none := by
 simp only [bind,unique_exact.mpr state,unique_exact.mpr param,Option.bind_eq_bind,Option.bind_some]
 cases values index <;> simp [wrong]

-- Exact environment binding is required for re-use; a type-preserving capture
-- change must not inherit the old concrete key. This is structural evidence,
-- not cryptographic custody or a current authorization claim.
theorem current_capture
 (before : Binds states signature values namespaceName index out)
 (after : Binds states signature changed namespaceName index out) :
 changed index = values index := after.2.2.1.trans before.2.2.1.symm

theorem bound_index_typed (bound : Binds states signature values namespaceName index out) :
 IndexTyped states signature namespaceName index out.declaration out.parameter :=
 ⟨bound.1,bound.2.1,bound.2.2.2.1⟩

-- The preceding required-schema repair retains the complete index declaration,
-- so it preserves this stronger check too, including ambiguous refusals.
theorem declarations_retain (required : namespaceName ∈ names) :
 declarations (OwnerRequiredSchema.retain states names) namespaceName =
 declarations states namespaceName := by
 unfold declarations OwnerRequiredSchema.retain
 rw [List.filter_filter]
 apply List.filter_congr
 intro state _
 by_cases same : state.name = namespaceName <;> simp_all

theorem index_retain (required : namespaceName ∈ names) :
 checkIndex (OwnerRequiredSchema.retain states names) signature namespaceName index =
 checkIndex states signature namespaceName index := by
 simp only [checkIndex,declarations_retain required]

theorem binding_retain (required : namespaceName ∈ names) :
 bind (OwnerRequiredSchema.retain states names) signature values namespaceName index =
 bind states signature values namespaceName index := by
 simp only [bind,declarations_retain required]

namespace Controls
def state : StateDeclaration := ⟨"player","id","Player","S",[⟨"hp","Int",none⟩]⟩
def parameter : Parameter := ⟨"target","Player"⟩
def values : String → Option Entity := fun name =>
 if name="target" then some ⟨"Player","alice"⟩ else none
#guard (bind [state] [parameter] values "player" "target").isSome
#guard (bind [state] [parameter] (fun _ => none) "player" "target").isNone
#guard (bind [state] [{parameter with typeName := "Team"}] values "player" "target").isNone
#guard (bind [state] [parameter] (fun _ => some ⟨"Team","alice"⟩) "player" "target").isNone
#guard (bind [state,state] [parameter] values "player" "target").isNone
#guard (bind [state] [parameter,parameter] values "player" "target").isNone
#guard explicitCheck [state] [parameter] "player" "self"
#guard explicitCheck [state] [parameter] "player" "target"
#guard !explicitCheck [state] [{parameter with typeName := "Team"}] "player" "target"
#guard !explicitCheck [state] [parameter,parameter] "player" "target"
#guard materialize (erase (fun _ => none)) ⟨"player",some "target",some "hp","S"⟩ =
 some ⟨"player","target","hp"⟩
end Controls
#print axioms bind_exact
#print axioms check_index_exact
#print axioms explicit_exact
#print axioms no_index_fallback
#print axioms missing_capture_refused
#print axioms wrong_index_type_refused
#print axioms current_capture
end MirroreaProofFirst.OwnerTypedIndex
