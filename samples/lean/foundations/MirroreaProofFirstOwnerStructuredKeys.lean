import MirroreaProofFirstOwnerEffectService
namespace MirroreaProofFirst.OwnerStructuredKeys
open OwnerCheckedArithmetic
variable {J V : Type}

-- Exact product used by M8StateKey. Owner locus is metadata, not an extra
-- coordinate smuggled into the field identity. No display-string concatenation.
structure Key where
 namespaceName : String
 entity : String
 field : String
 deriving DecidableEq, BEq, ReflBEq, LawfulBEq, Repr
structure Read where
 namespaceName : String
 index : Option String
 field : Option String
 owner : String
 deriving DecidableEq, Repr

-- Mirrors the existing materialize_entity_identity/materialize_key behavior:
-- substitution if present, otherwise the literal retained index. Missing index
-- or field is refused; this operation does not fetch any field value.
def materialize (arguments : String → Option String) (read : Read) : Option Key := do
 let index ← read.index
 let field ← read.field
 return ⟨read.namespaceName,(arguments index).getD index,field⟩

def Materializes (arguments : String → Option String) (read : Read) (key : Key) : Prop :=
 ∃ index field, read.index = some index ∧ read.field = some field ∧
 key.namespaceName = read.namespaceName ∧ key.entity = (arguments index).getD index ∧ key.field = field

theorem materialize_exact {arguments : String → Option String} {read : Read} {key : Key} : materialize arguments read = some key ↔ Materializes arguments read key := by
 cases read with
 | mk namespaceName index field owner =>
   cases index <;> cases field <;> cases key <;> simp [materialize,Materializes,Key.mk.injEq,eq_comm]

theorem namespace_retained {arguments : String → Option String} {read : Read} {key : Key} (accepted : materialize arguments read = some key) : key.namespaceName = read.namespaceName := by
 obtain ⟨_,_,_,_,h,_⟩ := materialize_exact.mp accepted; exact h

theorem distinct_namespaces (left : materialize args r = some x) (right : materialize args s = some y)
 (different : r.namespaceName ≠ s.namespaceName) : x ≠ y := by
 intro same
 exact different ((namespace_retained left).symm.trans (same ▸ namespace_retained right))

-- Finite correspondence address table, NOT a new runtime allocator. The real
-- runtime uses structured Key directly. Model numbers denote positions in one
-- retained table, including across added source blocks; rows are never recycled.
abbrev Addresses := List Key
def encode (a : Addresses) (key : Key) : Nat := a.idxOf key
def decode (a : Addresses) (i : Nat) : Option Key := a[i]?
def pullback (a : Addresses) (store : Key → Option V) (i : Nat) : Option V := (decode a i).bind store

theorem roundtrip (present : key ∈ a) : decode a (encode a key) = some key := by
 induction a with
 | nil => cases present
 | cons head tail ih =>
   by_cases same : head=key
   · subst head; simp [decode,encode]
   · have tailPresent : key ∈ tail := (List.mem_cons.mp present).resolve_left (Ne.symm same)
     have different : (head == key) = false := beq_eq_false_iff_ne.mpr same
     simpa [decode,encode,List.idxOf_cons,different] using ih tailPresent

theorem encoded_injective (left : x ∈ a) (right : y ∈ a) (same : encode a x = encode a y) : x=y := by
 have h := roundtrip left
 rw [same,roundtrip right] at h
 exact (Option.some.inj h).symm

theorem pullback_exact (present : key ∈ a) : pullback a store (encode a key) = store key := by
 simp [pullback,roundtrip present]

theorem old_address_retained (present : key ∈ a) : encode (a++extra) key = encode a key := by
 simp [encode,List.idxOf_append,present]

theorem old_decode_retained (present : key ∈ a) : decode (a++extra) (encode a key) = some key := by
 rw [←old_address_retained (extra:=extra) present]
 exact roundtrip (List.mem_append_left _ present)

theorem old_value_retained (present : key ∈ a) : pullback (a++extra) store (encode a key) = pullback a store (encode a key) := by
 simp [pullback,old_decode_retained present,roundtrip present]

-- The same pullback applies to owner/label metadata AND values: erasing an
-- address cannot silently turn one key's label into another key's label.
theorem value_and_metadata (present : key ∈ a) :
 pullback a store (encode a key) = store key ∧ pullback a metadata (encode a key) = metadata key :=
 ⟨pullback_exact present,pullback_exact present⟩

def put (store : Key → Option Int) (target : Key) (value : Int) : Key → Option Int :=
 fun key => if key=target then some value else store key

theorem write_refines (targetPresent : target ∈ a) (keyPresent : key ∈ a) :
 OwnerEffectService.put (pullback a store) (encode a target) value (encode a key) =
 put store target value key := by
 by_cases same : key=target
 · subst key; simp [OwnerEffectService.put,put]
 · have separated : encode a key ≠ encode a target := fun equal => same (encoded_injective keyPresent targetPresent equal)
   simp [OwnerEffectService.put,put,same,separated,pullback_exact keyPresent]

def Scoped (a : Addresses) : Checked Key J → Prop
 | .state key => key ∈ a
 | .parameter _ | .integer _ => True
 | .add left right | .sub left right => Scoped a left ∧ Scoped a right

def scopeCheck (a : Addresses) : Checked Key J → Bool
 | .state key => a.contains key
 | .parameter _ | .integer _ => true
 | .add left right | .sub left right => scopeCheck a left && scopeCheck a right

theorem scope_exact (tree : Checked Key J) : scopeCheck a tree = true ↔ Scoped a tree := by
 induction tree <;> simp_all [scopeCheck,Scoped]

def mapKeys (a : Addresses) : Checked Key J → Checked Nat J
 | .state key => .state (encode a key)
 | .parameter name => .parameter name
 | .integer value => .integer value
 | .add left right => .add (mapKeys a left) (mapKeys a right)
 | .sub left right => .sub (mapKeys a left) (mapKeys a right)

def lower (a : Addresses) (tree : Checked Key J) : Option (Checked Nat J) :=
 if scopeCheck a tree then some (mapKeys a tree) else none

-- This judgment checks address renaming only; it is not a substitute for the
-- independent source typing/authority/resource judgments of the consumer.
def Encodes (a : Addresses) (tree : Checked Key J) (out : Checked Nat J) : Prop :=
 Scoped a tree ∧ out = mapKeys a tree

theorem lower_exact : lower a tree = some out ↔ Encodes a tree out := by
 unfold lower Encodes
 by_cases scope : Scoped a tree
 · simp [(scope_exact tree).mpr scope,scope,eq_comm]
 · have no : scopeCheck a tree = false := by cases h : scopeCheck a tree <;> simp_all [scope_exact]
   simp [no,scope]

-- Partial lookup and bounded arithmetic failures are preserved, not replaced
-- with zero. This is general for every scoped tree and supplied arithmetic.
theorem evaluation (scope : Scoped a tree) (ops : FallibleFlow.Arithmetic)
 (store : Key → Option Int) (args : J → Option Int) :
 evaluate ops (pullback a store) args (mapKeys a tree) = evaluate ops store args tree := by
 induction tree with
 | state key => exact pullback_exact scope
 | parameter name => rfl
 | integer value => rfl
 | add left right il ir => simp only [mapKeys,evaluate,il scope.1,ir scope.2]
 | sub left right il ir => simp only [mapKeys,evaluate,il scope.1,ir scope.2]

theorem lowered_evaluation (accepted : lower a tree = some out) (ops : FallibleFlow.Arithmetic)
 (store : Key → Option Int) (args : J → Option Int) :
 evaluate ops (pullback a store) args out = evaluate ops store args tree := by
 obtain ⟨scope,rfl⟩ := lower_exact.mp accepted
 exact evaluation scope ops store args

theorem lower_exists_iff : (∃ out, lower a tree = some out) ↔ Scoped a tree := by
 simp only [lower_exact,Encodes]; constructor
 · rintro ⟨_,scope,_⟩; exact scope
 · intro scope; exact ⟨_,scope,rfl⟩

namespace Controls
def left : Key := ⟨"a","self","hp"⟩
def right : Key := ⟨"b","self","hp"⟩
def args (name : String) : Option String := if name="target" then some "self" else none
#guard materialize args ⟨"a",some "target",some "hp","S"⟩ = some left
#guard materialize args ⟨"a",some "self",some "hp","S"⟩ = some left
#guard materialize args ⟨"b",some "self",some "hp","T"⟩ = some right
#guard materialize args ⟨"a",none,some "hp","S"⟩ = none
#guard encode [left] left = 0 && encode [right] right = 0
#guard encode [left,right] left = 0 && encode [left,right] right = 1
#guard (pullback [left,right] (fun k => if k=left then some 10 else none) 1).isNone
#guard evaluate (FallibleFlow.signed 63) (pullback [left,right] (fun k => if k=left then some 10 else none))
 (fun (_ : Nat) => none) (mapKeys [left,right] (.add (.state left) (.state right))) = none
end Controls
#print axioms materialize_exact
#print axioms distinct_namespaces
#print axioms roundtrip
#print axioms encoded_injective
#print axioms old_address_retained
#print axioms old_decode_retained
#print axioms old_value_retained
#print axioms value_and_metadata
#print axioms write_refines
#print axioms evaluation
#print axioms scope_exact
#print axioms lower_exact
#print axioms lowered_evaluation
#print axioms lower_exists_iff
end MirroreaProofFirst.OwnerStructuredKeys
