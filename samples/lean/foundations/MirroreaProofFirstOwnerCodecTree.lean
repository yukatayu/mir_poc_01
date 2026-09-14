import Std

namespace MirroreaProofFirst.OwnerCodecTree

-- Total structural encoding candidate for finite owner data. No bytes, JSON
-- parser, authority, current installation or public encoding is selected here.
inductive Tree where
  | natural (value : Nat)
  | integer (value : Int)
  | text (value : String)
  | node (tag : Nat) (fields : List Tree)
  deriving Repr

structure Codec (T : Type) where
  encode : T → Tree
  decode : Tree → Option T
  roundtrip : ∀ value, decode (encode value) = some value
  canonical : ∀ tree value, decode tree = some value → encode value = tree

theorem decode_exact (c : Codec T) (tree : Tree) (value : T) :
    c.decode tree = some value ↔ tree = c.encode value :=
  ⟨fun decoded => (c.canonical tree value decoded).symm,fun equal => equal ▸ c.roundtrip value⟩

def natural : Codec Nat where
  encode := .natural
  decode := fun tree => match tree with | .natural value => some value | _ => none
  roundtrip := fun _ => rfl
  canonical := by intro tree value decoded; cases tree <;> simp_all

def integer : Codec Int where
  encode := .integer
  decode := fun tree => match tree with | .integer value => some value | _ => none
  roundtrip := fun _ => rfl
  canonical := by intro tree value decoded; cases tree <;> simp_all

def text : Codec String where
  encode := .text
  decode := fun tree => match tree with | .text value => some value | _ => none
  roundtrip := fun _ => rfl
  canonical := by intro tree value decoded; cases tree <;> simp_all

def finite (n : Nat) : Codec (Fin n) where
  encode := fun value => .natural value.val
  decode := fun tree => match tree with
    | .natural value => if h : value < n then some ⟨value,h⟩ else none
    | _ => none
  roundtrip := by intro value; simp [value.isLt]
  canonical := by
    intro tree value decoded
    cases tree <;> simp_all
    obtain ⟨_,equal⟩ := decoded
    cases equal
    rfl

def product (left : Codec A) (right : Codec B) : Codec (A × B) where
  encode := fun value => .node 0 [left.encode value.1,right.encode value.2]
  decode := fun tree => match tree with
    | .node 0 [a,b] => do return (← left.decode a,← right.decode b)
    | _ => none
  roundtrip := by intro value; simp [left.roundtrip,right.roundtrip]
  canonical := by
    intro tree value decoded
    split at decoded
    · rename_i a b
      cases ha : left.decode a with
      | none => simp [ha] at decoded
      | some va =>
        cases hb : right.decode b with
        | none => simp [ha,hb] at decoded
        | some vb =>
          simp [ha,hb] at decoded
          subst value
          simp [left.canonical a va ha,right.canonical b vb hb]
    · cases decoded

theorem list_roundtrip (c : Codec A) (values : List A) :
    (values.map c.encode).mapM c.decode = some values := by
  induction values with
  | nil => rfl
  | cons value rest ih => simp [c.roundtrip,ih]

theorem list_canonical (c : Codec A) (trees : List Tree) (values : List A)
    (decoded : trees.mapM c.decode = some values) : values.map c.encode = trees := by
  induction trees generalizing values with
  | nil => simpa using decoded.symm
  | cons tree rest ih =>
    cases ht : c.decode tree with
    | none => simp [ht] at decoded
    | some value =>
      cases hr : rest.mapM c.decode with
      | none => simp [ht,hr] at decoded
      | some others =>
        simp [ht,hr] at decoded
        subst values
        simp [c.canonical tree value ht,ih others hr]

def list (element : Codec A) : Codec (List A) where
  encode := fun values => .node 0 (values.map element.encode)
  decode := fun tree => match tree with | .node 0 fields => fields.mapM element.decode | _ => none
  roundtrip := list_roundtrip element
  canonical := by
    intro tree values decoded
    split at decoded
    · rename_i fields
      simp [list_canonical element fields values decoded]
    · cases decoded

def unit : Codec Unit where
  encode := fun _ => .node 0 []
  decode := fun tree => match tree with | .node 0 [] => some () | _ => none
  roundtrip := by intro value; cases value; rfl
  canonical := by intro tree value decoded; split at decoded <;> simp_all

def sum (left : Codec A) (right : Codec B) : Codec (Sum A B) where
  encode := fun value => match value with
    | .inl a => .node 0 [left.encode a]
    | .inr b => .node 1 [right.encode b]
  decode := fun tree => match tree with
    | .node 0 [a] => Sum.inl <$> left.decode a
    | .node 1 [b] => Sum.inr <$> right.decode b
    | _ => none
  roundtrip := by intro value; cases value <;> simp [left.roundtrip,right.roundtrip]
  canonical := by
    intro tree value decoded
    split at decoded
    · rename_i a
      cases ha : left.decode a <;> simp [ha] at decoded
      subst value
      simp [left.canonical _ _ ha]
    · rename_i b
      cases hb : right.decode b <;> simp [hb] at decoded
      subst value
      simp [right.canonical _ _ hb]
    · cases decoded

-- Transport a proved codec through a full two-sided structural isomorphism.
-- Both laws are required; a lossy projection cannot enter as serialization.
def iso (base : Codec A) (forward : B → A) (backward : A → B)
    (back_forward : ∀ b, backward (forward b) = b)
    (forward_back : ∀ a, forward (backward a) = a) : Codec B where
  encode := fun b => base.encode (forward b)
  decode := fun tree => backward <$> base.decode tree
  roundtrip := by intro b; simp [base.roundtrip,back_forward]
  canonical := by
    intro tree b decoded
    cases h : base.decode tree <;> simp [h] at decoded
    subst b
    simp [forward_back,base.canonical _ _ h]

def boolean : Codec Bool := iso (sum unit unit)
  (fun b => if b then .inr () else .inl ())
  (fun value => match value with | .inl _ => false | .inr _ => true)
  (by intro b; cases b <;> rfl)
  (by intro value; cases value <;> rfl)

def optional (base : Codec A) : Codec (Option A) := iso (sum unit base)
  (fun value => match value with | none => .inl () | some a => .inr a)
  (fun value => match value with | .inl _ => none | .inr a => some a)
  (by intro value; cases value <;> rfl)
  (by intro value; cases value <;> rfl)

def vector (element : Codec A) (n : Nat) : Codec (Vector A n) where
  encode := fun value => (list element).encode value.toArray.toList
  decode := fun tree => do
    let values ← (list element).decode tree
    if h : values.length = n then some ⟨values.toArray,by simpa using h⟩ else none
  roundtrip := by intro value; simp [Codec.roundtrip]
  canonical := by
    intro tree value decoded
    cases h : (list element).decode tree with
    | none => simp [h] at decoded
    | some values =>
      simp [h] at decoded
      obtain ⟨_,equal⟩ := decoded
      have same : value.toArray.toList = values := by
        simpa using congrArg Array.toList equal.symm
      rw [same]
      exact (list element).canonical tree values h

def dependent {A : Type} {B : A → Type} (head : Codec A) (tail : (a : A) → Codec (B a)) : Codec ((a : A) × B a) where
  encode := fun value => .node 0 [head.encode value.1,(tail value.1).encode value.2]
  decode := fun tree => match tree with
    | .node 0 [h,t] => do
      let a ← head.decode h
      let b ← (tail a).decode t
      return ⟨a,b⟩
    | _ => none
  roundtrip := by intro value; cases value; simp [head.roundtrip,Codec.roundtrip]
  canonical := by
    intro tree value decoded
    split at decoded
    · rename_i h t
      cases hh : head.decode h with
      | none => simp [hh] at decoded
      | some a =>
        cases ht : (tail a).decode t with
        | none => simp [hh,ht] at decoded
        | some b =>
          simp [hh,ht] at decoded
          subst value
          simp [head.canonical h a hh,(tail a).canonical t b ht]
    · cases decoded

#print axioms decode_exact
#print axioms finite
#print axioms product
#print axioms list
#print axioms iso
#print axioms vector
#print axioms dependent
end MirroreaProofFirst.OwnerCodecTree
