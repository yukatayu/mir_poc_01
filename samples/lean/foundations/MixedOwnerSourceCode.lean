import MixedOwnerSourceFootprint
namespace MirroreaProofFirst.MixedOwnerSourceCode
open OwnerCheckedArithmetic MixedNamedOwnerSource

-- A declaration layout contains names and labels ONLY. Compilation does not
-- read current source values, manufacture zero captures, or execute an owner
-- read. Actual capture values remain an issue-time operation in the same source.
abbrev Layout := List (String × Nat)
def layout (captures : Captures) : Layout := captures.map fun row => (row.1,row.2.label)

def readTypes (labels : List (String × Nat)) : List String → Option Layout
 | [] => some []
 | name::rest => do return (name,←labels.lookup name)::(←readTypes labels rest)

inductive Types (labels : List (String × Nat)) : List String → Layout → Prop where
 | nil : Types labels [] []
 | cons : labels.lookup name = some label → Types labels names rest → Types labels (name::names) ((name,label)::rest)

theorem types_exact : readTypes labels names = some out ↔ Types labels names out := by
 induction names generalizing out with
 | nil => constructor <;> intro h <;> cases h <;> constructor
 | cons name names ih =>
   constructor
   · intro accepted
     cases label : labels.lookup name with
     | none => simp [readTypes,label] at accepted
     | some value =>
       cases tail : readTypes labels names with
       | none => simp [readTypes,label,tail] at accepted
       | some rest =>
         simp only [readTypes,label,tail,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
         subst out
         exact .cons label (ih.mp tail)
   · intro typed
     cases typed with
     | cons label tail => simp [readTypes,label,ih.mpr tail]

def slot : Layout → String → Option Nat
 | [],_ => none
 | (name,_)::rest,key => if key=name then some 0 else (slot rest key).map (·+1)

inductive SlotAt : Layout → String → Nat → Prop where
 | here : SlotAt ((name,label)::rest) name 0
 | there : key ≠ name → SlotAt rest key i → SlotAt ((name,label)::rest) key (i+1)

theorem slot_exact : slot ls name = some index ↔ SlotAt ls name index := by
 induction ls generalizing index with
 | nil => constructor <;> intro h <;> cases h
 | cons row rest ih =>
   obtain ⟨key,label⟩ := row
   by_cases same : name=key
   · subst name
     constructor
     · intro h; simp [slot] at h; subst index; exact .here
     · intro h; cases h with
       | here => simp [slot]
       | there different _ => exact False.elim (different rfl)
   · constructor
     · intro h
       simp only [slot,if_neg same] at h
       cases run : slot rest name with
       | none => simp [run] at h
       | some prior =>
         simp only [run,Option.map_some,Option.some.injEq] at h
         subst index
         exact .there same (ih.mp run)
     · intro h
       cases h with
       | here => exact False.elim (same rfl)
       | there _ tail => simp [slot,same,ih.mpr tail]

def resolve (ctx : Fields) (ls : Layout) : Checked String String → Option (Checked Nat Nat)
 | .state name => (field ctx name).map Checked.state
 | .parameter name => (slot ls name).map Checked.parameter
 | .integer value => some (.integer value)
 | .add left right => do return .add (←resolve ctx ls left) (←resolve ctx ls right)
 | .sub left right => do return .sub (←resolve ctx ls left) (←resolve ctx ls right)

inductive Resolves (ctx : Fields) (ls : Layout) : Checked String String → Checked Nat Nat → Prop where
 | state : FieldAt ctx name key → Resolves ctx ls (.state name) (.state key)
 | parameter : SlotAt ls name index → Resolves ctx ls (.parameter name) (.parameter index)
 | integer : Resolves ctx ls (.integer value) (.integer value)
 | add : Resolves ctx ls left x → Resolves ctx ls right y → Resolves ctx ls (.add left right) (.add x y)
 | sub : Resolves ctx ls left x → Resolves ctx ls right y → Resolves ctx ls (.sub left right) (.sub x y)

theorem resolve_sound (accepted : resolve ctx ls tree = some out) : Resolves ctx ls tree out := by
 induction tree generalizing out with
 | state name =>
   cases run : field ctx name <;> simp [resolve,run] at accepted
   subst out; exact .state (field_exact.mp run)
 | parameter name =>
   cases run : slot ls name <;> simp [resolve,run] at accepted
   subst out; exact .parameter (slot_exact.mp run)
 | integer value => cases accepted; exact .integer
 | add left right il ir =>
   cases l : resolve ctx ls left <;> cases r : resolve ctx ls right <;> simp [resolve,l,r] at accepted
   subst out; exact .add (il l) (ir r)
 | sub left right il ir =>
   cases l : resolve ctx ls left <;> cases r : resolve ctx ls right <;> simp [resolve,l,r] at accepted
   subst out; exact .sub (il l) (ir r)

theorem resolve_complete (meaning : Resolves ctx ls tree out) : resolve ctx ls tree = some out := by
 induction meaning with
 | state found => simp [resolve,field_exact.mpr found]
 | parameter found => simp [resolve,slot_exact.mpr found]
 | integer => rfl
 | add _ _ il ir => simp [resolve,il,ir]
 | sub _ _ il ir => simp [resolve,il,ir]

def definition (ctx : Fields) (ls : Layout) (control target : Nat) (rhs : Checked Nat Nat) :
 MixedOperationDefinitions.OwnerDefinition :=
 ⟨⟨target,rhs⟩,⟨MixedOwnerSourceFootprint.fields ctx target rhs,ls.map Prod.snd,control⟩⟩

structure Code where
 site : ReferenceSourceData.Site
 operation : MixedOperationDefinitions.OwnerDefinition
 captures : Layout
 deriving DecidableEq

def compile (ctx : Fields) (labels : List (String × Nat)) (control : Nat) (source : Assignment) : Option Code := do
 let target ← field ctx source.target
 let ls ← readTypes labels (parameters source.rhs).eraseDups
 let tree ← resolve ctx ls source.rhs
 let op := definition ctx ls control target tree
 if MixedOperationDefinitions.ownerCheck op then some ⟨source.site,op,ls⟩ else none

def erase (plan : Plan) : Code := ⟨plan.site,plan.operation,layout plan.captures⟩

-- Declarative compilation, with structural name/layout judgments and the prior
-- independent owner typing/flow judgment. It does not mention compile success.
def Compiles (ctx : Fields) (labels : List (String × Nat)) (control : Nat)
 (source : Assignment) (code : Code) : Prop :=
 ∃ target ls tree, FieldAt ctx source.target target ∧
 Types labels (parameters source.rhs).eraseDups ls ∧ Resolves ctx ls source.rhs tree ∧
 MixedOperationDefinitions.OwnerSatisfies (definition ctx ls control target tree) ∧
 code = ⟨source.site,definition ctx ls control target tree,ls⟩

theorem compile_exact (control : Nat) : compile ctx labels control source = some code ↔
 Compiles ctx labels control source code := by
 constructor
 · intro accepted
   unfold compile at accepted
   cases f : field ctx source.target with
   | none => simp [f] at accepted
   | some target =>
     cases types : readTypes labels (parameters source.rhs).eraseDups with
     | none => simp [f,types] at accepted
     | some ls =>
       cases body : resolve ctx ls source.rhs with
       | none => simp [f,types,body] at accepted
       | some tree =>
         simp only [f,types,body,Option.bind_eq_bind,Option.bind_some] at accepted
         split at accepted
         · exact ⟨target,ls,tree,field_exact.mp f,types_exact.mp types,resolve_sound body,
             MixedOperationDefinitions.owner_check_exact.mp ‹_›,(Option.some.inj accepted).symm⟩
         · cases accepted
 · rintro ⟨target,ls,tree,fieldAt,types,body,typed,rfl⟩
   simp [compile,field_exact.mpr fieldAt,types_exact.mpr types,resolve_complete body,
     MixedOperationDefinitions.owner_check_exact.mpr typed]

theorem captured_types (captured : Captured values labels names cs) : Types labels names (layout cs) := by
 induction captured with
 | nil => exact .nil
 | cons _ label _ ih => exact .cons label ih

theorem slot_erasure : slot (layout cs) name = (MixedNamedOwnerSource.locate cs name).map Prod.fst := by
 induction cs with
 | nil => rfl
 | cons row rest ih =>
   obtain ⟨key,capture⟩ := row
   by_cases same : name=key
   · simp [slot,layout,MixedNamedOwnerSource.locate,same]
   · simp only [layout,List.map_cons,slot,if_neg same,MixedNamedOwnerSource.locate]
     rw [show slot (rest.map fun row => (row.1,row.2.label)) name =
       (MixedNamedOwnerSource.locate rest name).map Prod.fst from ih]
     cases found : MixedNamedOwnerSource.locate rest name <;> rfl

theorem resolve_erasure (ctx : Fields) (cs : Captures) (tree : Checked String String) :
 resolve ctx (layout cs) tree = MixedNamedOwnerSource.resolve ctx cs tree := by
 induction tree with
 | state name => rfl
 | parameter name =>
   simp only [resolve,slot_erasure,MixedNamedOwnerSource.resolve,Option.map_map]
   rfl
 | integer value => rfl
 | add left right il ir => simp only [resolve,MixedNamedOwnerSource.resolve,il,ir]
 | sub left right il ir => simp only [resolve,MixedNamedOwnerSource.resolve,il,ir]

theorem definition_erasure {control : Nat} : definition ctx (layout cs) control target tree =
 MixedOwnerSourceFootprint.definition ctx cs control target tree := by
 simp only [definition,layout,List.map_map,MixedOwnerSourceFootprint.definition]
 rfl

-- The actual source issue elaborator compiles to exactly this declaration.
-- Code/contract/layout do not depend on replacing captures with their current
-- values. Retained labels/field context/control still matter and are not erased.
theorem elaboration_compiles (control : Nat)
 (accepted : MixedOwnerSourceFootprint.elaborate ctx values labels control source = some plan) :
 compile ctx labels control source = some (erase plan) := by
 obtain ⟨target,tree,cs,fieldAt,captured,resolved,valid,rfl⟩ := (MixedOwnerSourceFootprint.elaborate_exact control).mp accepted
 have types := types_exact.mpr (captured_types captured)
 have body : resolve ctx (layout cs) source.rhs = some tree := by
   rw [resolve_erasure]; exact MixedNamedOwnerSource.resolve_complete resolved
 simp [compile,field_exact.mpr fieldAt,types,body,definition_erasure,
   MixedOperationDefinitions.owner_check_exact.mpr valid,erase]

theorem value_independent_code (control : Nat)
 (first : MixedOwnerSourceFootprint.elaborate ctx left labels control source = some firstPlan)
 (second : MixedOwnerSourceFootprint.elaborate ctx right labels control source = some secondPlan) :
 erase firstPlan = erase secondPlan :=
 Option.some.inj ((elaboration_compiles control first).symm.trans (elaboration_compiles control second))

-- Conversely, a statically compiled declaration can be instantiated with ANY
-- actual captured values satisfying the independent current capture judgment.
-- The resulting source plan has the SAME complete code/contract/layout. No
-- successful issue, owner service or authorization is assumed or manufactured.
theorem compiled_with_values (control : Nat)
 (compiled : Compiles ctx labels control source code)
 (captured : Captured values labels (parameters source.rhs).eraseDups cs) :
 ∃ plan, MixedOwnerSourceFootprint.Elaborates ctx values labels control source plan ∧ erase plan = code := by
 obtain ⟨target,ls,tree,fieldAt,types,body,typed,rfl⟩ := compiled
 have same : layout cs = ls := Option.some.inj ((types_exact.mpr (captured_types captured)).symm.trans (types_exact.mpr types))
 have resolved : MixedNamedOwnerSource.Resolves ctx cs source.rhs tree := by
   apply MixedNamedOwnerSource.resolve_sound
   rw [←resolve_erasure,same]
   exact resolve_complete body
 have op : definition ctx ls control target tree = MixedOwnerSourceFootprint.definition ctx cs control target tree := by
   rw [←same,definition_erasure]
 refine ⟨⟨source.site,MixedOwnerSourceFootprint.definition ctx cs control target tree,cs⟩,
   ⟨target,tree,cs,fieldAt,captured,resolved,?_,rfl⟩,?_⟩
 · rw [←op]; exact typed
 · simp [erase,op,same]

namespace Controls
open MixedNamedOwnerSource.Controls (ctx values labels source)
#guard (compile ctx labels 0 source).isSome
#guard (MixedOwnerSourceFootprint.elaborate ctx [] labels 0 source).isNone
#guard (compile ctx labels 0 source |>.map fun code => code.operation.body.tree) = some (.add (.state 0) (.parameter 0))
#guard (compile ctx labels 0 source |>.map fun code => code.captures) = some [("amount",0)]
#guard (compile ctx labels 0 source) = (MixedOwnerSourceFootprint.elaborate ctx values labels 0 source).map erase
#guard (compile ctx [] 0 source).isNone
#guard (compile ctx [("amount",1)] 0 source).isNone
end Controls

#print axioms types_exact
#print axioms slot_exact
#print axioms resolve_sound
#print axioms resolve_complete
#print axioms compile_exact
#print axioms captured_types
#print axioms slot_erasure
#print axioms resolve_erasure
#print axioms definition_erasure
#print axioms elaboration_compiles
#print axioms value_independent_code
#print axioms compiled_with_values
end MirroreaProofFirst.MixedOwnerSourceCode
