import MixedReferenceSourceData
import MixedOperationDefinitions
namespace MirroreaProofFirst.MixedNamedOwnerSource
open OwnerCheckedArithmetic ReferenceSourceData

-- A private checked-IR name-resolution cut, reusing the real five-form owner
-- expression. No source send/event/receipt annotations or integer-key syntax.
-- Context authenticity and current physical field lifetime are separate gates.
structure Fields where
 names : List (String × Nat)
 metadata : List (Nat × (Nat × Nat)) -- unique canonical key -> owner locus,label
 deriving DecidableEq, Repr

def field (ctx : Fields) (name : String) : Option Nat := do
 let key ← ctx.names.lookup name
 let _ ← ctx.metadata.lookup key
 return key

def FieldAt (ctx : Fields) (name : String) (key : Nat) : Prop :=
 ctx.names.lookup name = some key ∧ ∃ info, ctx.metadata.lookup key = some info

theorem field_exact : field ctx name = some key ↔ FieldAt ctx name key := by
 cases hn : ctx.names.lookup name with
 | none => simp [field,FieldAt,hn]
 | some resolved =>
   constructor
   · intro h
     cases hm : ctx.metadata.lookup resolved with
     | none => simp [field,hn,hm] at h
     | some info =>
       have same : resolved = key := by simpa [field,hn,hm] using h
       subst key
       exact ⟨hn,info,hm⟩
   · rintro ⟨found,info,hm⟩
     simp [field,found,hm]

structure Capture where
 value : Int
 label : Nat
 deriving DecidableEq, Repr
abbrev Captures := List (String × Capture)

def integer (values : Values) (name : String) : Option Int :=
 match lookup values name with | some (.plain (.integer value _)) => some value | _ => none

def captures (values : Values) (labels : List (String × Nat)) : List String → Option Captures
 | [] => some []
 | name::rest => do
   let value ← integer values name
   let label ← labels.lookup name
   let tail ← captures values labels rest
   return (name,⟨value,label⟩)::tail

inductive Captured (values : Values) (labels : List (String × Nat)) : List String → Captures → Prop where
 | nil : Captured values labels [] []
 | cons : integer values name = some value → labels.lookup name = some label →
    Captured values labels rest tail → Captured values labels (name::rest) ((name,⟨value,label⟩)::tail)

theorem captures_exact : captures values labels names = some out ↔ Captured values labels names out := by
 induction names generalizing out with
 | nil => constructor <;> intro h <;> cases h <;> constructor
 | cons name rest ih =>
   constructor
   · intro h
     unfold captures at h
     cases hv : integer values name with
     | none => simp [hv] at h
     | some value =>
       cases hl : labels.lookup name with
       | none => simp [hv,hl] at h
       | some label =>
         cases ht : captures values labels rest with
         | none => simp [hv,hl,ht] at h
         | some tail =>
           simp only [hv,hl,ht,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at h
           subst out
           exact .cons hv hl (ih.mp ht)
   · intro h
     cases h with
     | cons hv hl ht => simp [captures,hv,hl,ih.mpr ht]

def parameters : Checked K String → List String
 | .state _ | .integer _ => []
 | .parameter name => [name]
 | .add left right | .sub left right => parameters left ++ parameters right

-- Occurrence-order capture list. Repeated names are harmless repeated payloads;
-- lookup selects the first matching occurrence, with value AND label retained.
-- Each call captures current local values only, never a multi-owner snapshot.
def locate : Captures → String → Option (Nat × Capture)
 | [],_ => none
 | (name,value)::rest,key =>
   if key = name then some (0,value) else (locate rest key).map fun (i,v) => (i+1,v)

inductive LocatedCapture : Captures → String → Nat → Capture → Prop where
 | here : LocatedCapture ((name,value)::rest) name 0 value
 | there : key ≠ name → LocatedCapture rest key i value →
    LocatedCapture ((name,head)::rest) key (i+1) value

theorem locate_exact : locate cs name = some (i,value) ↔ LocatedCapture cs name i value := by
 induction cs generalizing i value with
 | nil => constructor <;> intro h <;> cases h
 | cons head rest ih =>
   obtain ⟨other,v⟩ := head
   by_cases same : name = other
   · subst name
     simp only [locate,ite_true,Option.some.injEq,Prod.mk.injEq]
     constructor
     · rintro ⟨rfl,rfl⟩; exact .here
     · intro h; cases h with
       | here => exact ⟨rfl,rfl⟩
       | there different _ => exact False.elim (different rfl)
   · constructor
     · intro h
       simp only [locate,if_neg same] at h
       cases run : locate rest name with
       | none => simp [run] at h
       | some pair =>
         obtain ⟨j,c⟩ := pair
         simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at h
         obtain ⟨rfl,rfl⟩ := h
         exact .there same (ih.mp run)
     · intro h
       cases h with
       | here => exact False.elim (same rfl)
       | there _ tail => simp [locate,same,ih.mpr tail]

theorem located_get (h : LocatedCapture cs name i value) : cs[i]? = some (name,value) := by
 induction h with
 | here => rfl
 | there _ _ ih => simpa using ih

def argument (cs : Captures) (i : Nat) : Option Int := (cs[i]?).map (fun row => row.2.value)
def namedArgument (cs : Captures) (name : String) : Option Int := (locate cs name).map (fun row => row.2.value)

def resolve (ctx : Fields) (cs : Captures) : Checked String String → Option (Checked Nat Nat)
 | .state name => (field ctx name).map Checked.state
 | .parameter name => (locate cs name).map (fun row => .parameter row.1)
 | .integer value => some (.integer value)
 | .add l r => do return .add (← resolve ctx cs l) (← resolve ctx cs r)
 | .sub l r => do return .sub (← resolve ctx cs l) (← resolve ctx cs r)

inductive Resolves (ctx : Fields) (cs : Captures) : Checked String String → Checked Nat Nat → Prop where
 | state : FieldAt ctx name key → Resolves ctx cs (.state name) (.state key)
 | parameter : LocatedCapture cs name i value → Resolves ctx cs (.parameter name) (.parameter i)
 | integer : Resolves ctx cs (.integer value) (.integer value)
 | add : Resolves ctx cs a x → Resolves ctx cs b y → Resolves ctx cs (.add a b) (.add x y)
 | sub : Resolves ctx cs a x → Resolves ctx cs b y → Resolves ctx cs (.sub a b) (.sub x y)

theorem resolve_sound (h : resolve ctx cs tree = some out) : Resolves ctx cs tree out := by
 induction tree generalizing out with
 | state name =>
   cases found : field ctx name <;> simp [resolve,found] at h
   subst out; exact .state (field_exact.mp found)
 | parameter name =>
   cases found : locate cs name with
   | none => simp [resolve,found] at h
   | some row =>
     obtain ⟨i,value⟩ := row
     simp only [resolve,found,Option.map_some,Option.some.injEq] at h
     subst out; exact .parameter (locate_exact.mp found)
 | integer v => cases h; exact .integer
 | add a b ia ib =>
   cases ha : resolve ctx cs a <;> cases hb : resolve ctx cs b <;> simp [resolve,ha,hb] at h
   subst out; exact .add (ia ha) (ib hb)
 | sub a b ia ib =>
   cases ha : resolve ctx cs a <;> cases hb : resolve ctx cs b <;> simp [resolve,ha,hb] at h
   subst out; exact .sub (ia ha) (ib hb)

theorem resolve_complete (h : Resolves ctx cs tree out) : resolve ctx cs tree = some out := by
 induction h with
 | state h => simp [resolve,field_exact.mpr h]
 | parameter h => simp [resolve,locate_exact.mpr h]
 | integer => rfl
 | add _ _ ia ib => simp [resolve,ia,ib]
 | sub _ _ ia ib => simp [resolve,ia,ib]

theorem resolve_exact : resolve ctx cs tree = some out ↔ Resolves ctx cs tree out :=
 ⟨resolve_sound,resolve_complete⟩

-- General evaluation equality, including lookup failure and overflow. Aliases
-- use the same canonical store key. Captured labels are kept for the next gate.
theorem evaluation (h : Resolves ctx cs tree out) (ops : FallibleFlow.Arithmetic)
 (store : Nat → Option Int) :
 evaluate ops store (argument cs) out =
 evaluate ops (fun name => (field ctx name).bind store) (namedArgument cs) tree := by
 induction h with
 | state h => simp [evaluate,field_exact.mpr h]
 | parameter h => simp [evaluate,argument,namedArgument,located_get h,locate_exact.mpr h]
 | integer => rfl
 | add _ _ ia ib => simp only [evaluate,ia,ib]
 | sub _ _ ia ib => simp only [evaluate,ia,ib]

structure Assignment where
 site : Site
 target : String
 rhs : Checked String String

def definition (ctx : Fields) (cs : Captures) (control target : Nat) (rhs : Checked Nat Nat) :
 MixedOperationDefinitions.OwnerDefinition :=
 ⟨⟨target,rhs⟩,⟨ctx.metadata,cs.map (fun row => row.2.label),control⟩⟩

-- No independently supplied body or argument vector survives this elaboration.
structure Plan where
 site : Site
 operation : MixedOperationDefinitions.OwnerDefinition
 captures : Captures
 deriving DecidableEq

def elaborate (ctx : Fields) (values : Values) (labels : List (String × Nat))
 (control : Nat) (source : Assignment) : Option Plan := do
 let target ← field ctx source.target
 let cs ← captures values labels (parameters source.rhs)
 let tree ← resolve ctx cs source.rhs
 let op := definition ctx cs control target tree
 if MixedOperationDefinitions.ownerCheck op then some ⟨source.site,op,cs⟩ else none

def Elaborates (ctx : Fields) (values : Values) (labels : List (String × Nat))
 (control : Nat) (source : Assignment) (plan : Plan) : Prop :=
 ∃ target tree cs, FieldAt ctx source.target target ∧
 Captured values labels (parameters source.rhs) cs ∧ Resolves ctx cs source.rhs tree ∧
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
     cases hc : captures values labels (parameters source.rhs) with
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

#print axioms captures_exact
#print axioms locate_exact
#print axioms located_get
#print axioms resolve_exact
#print axioms evaluation
#print axioms elaborate_exact
end MirroreaProofFirst.MixedNamedOwnerSource
