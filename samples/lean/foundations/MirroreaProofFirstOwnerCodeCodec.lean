import MirroreaProofFirstOwnerCodecTree
import MirroreaProofFirstOwnerValidity

namespace MirroreaProofFirst.OwnerCodeCodec
open OwnerCodecTree LocalContract

def encodeTerm : Term → Tree
  | .input index => .node 0 [.natural index]
  | .integer value => .node 1 [.integer value]
  | .add left right => .node 2 [encodeTerm left,encodeTerm right]
  | .mul left right => .node 3 [encodeTerm left,encodeTerm right]
  | .square term => .node 4 [encodeTerm term]

def decodeTerm (tree : Tree) : Option Term := match tree with
  | .node 0 [.natural index] => some (.input index)
  | .node 1 [.integer value] => some (.integer value)
  | .node 2 [left,right] => do return .add (← decodeTerm left) (← decodeTerm right)
  | .node 3 [left,right] => do return .mul (← decodeTerm left) (← decodeTerm right)
  | .node 4 [term] => .square <$> decodeTerm term
  | _ => none
termination_by sizeOf tree

theorem term_roundtrip (term : Term) : decodeTerm (encodeTerm term) = some term := by
  induction term <;> simp_all [encodeTerm,decodeTerm]

theorem tree_induction {P : Tree → Prop}
    (hn : ∀ n, P (.natural n)) (hi : ∀ i, P (.integer i)) (hs : ∀ s, P (.text s))
    (node : ∀ tag fields, (∀ field ∈ fields, P field) → P (.node tag fields)) :
    ∀ tree, P tree := by
  intro tree
  refine Tree.rec (motive_1 := P) (motive_2 := fun fields => ∀ field ∈ fields, P field) hn hi hs node ?_ ?_ tree
  · simp
  · intro head tail hh ht field member
    rcases List.mem_cons.mp member with rfl | belongs
    · exact hh
    · exact ht field belongs

theorem term_canonical (tree : Tree) (term : Term) (decoded : decodeTerm tree = some term) :
    encodeTerm term = tree := by
  induction tree using tree_induction generalizing term with
  | hn n => simp [decodeTerm] at decoded
  | hi i => simp [decodeTerm] at decoded
  | hs s => simp [decodeTerm] at decoded
  | node tag fields ih =>
    unfold decodeTerm at decoded
    split at decoded
    · rename_i index shape
      simp only [Option.some.injEq] at decoded
      subst term
      exact shape.symm
    · rename_i value shape
      simp only [Option.some.injEq] at decoded
      subst term
      exact shape.symm
    · rename_i left right shape
      rcases Tree.node.inj shape with ⟨rfl,rfl⟩
      cases hl : decodeTerm left with
      | none => simp [hl] at decoded
      | some a =>
        cases hr : decodeTerm right with
        | none => simp [hl,hr] at decoded
        | some b =>
          simp [hl,hr] at decoded
          subst term
          simp only [encodeTerm]
          rw [ih left (by simp) a hl,ih right (by simp) b hr]
    · rename_i left right shape
      rcases Tree.node.inj shape with ⟨rfl,rfl⟩
      cases hl : decodeTerm left with
      | none => simp [hl] at decoded
      | some a =>
        cases hr : decodeTerm right with
        | none => simp [hl,hr] at decoded
        | some b =>
          simp [hl,hr] at decoded
          subst term
          simp only [encodeTerm]
          rw [ih left (by simp) a hl,ih right (by simp) b hr]
    · rename_i inner shape
      rcases Tree.node.inj shape with ⟨rfl,rfl⟩
      cases ht : decodeTerm inner <;> simp [ht] at decoded
      subst term
      simp only [encodeTerm]
      rw [ih inner (by simp) _ ht]
    · cases decoded

def term : Codec Term := ⟨encodeTerm,decodeTerm,term_roundtrip,term_canonical⟩

def contract : Codec InstancePrograms.Contract := iso
  (product (list integer) (product integer integer))
  (fun c => (c.inputs,c.lower,c.upper)) (fun c => ⟨c.1,c.2.1,c.2.2⟩)
  (by intro c; cases c; rfl) (by intro c; rfl)

def definition : Codec InstancePrograms.Definition := iso (product term contract)
  (fun d => (d.code,d.contract)) (fun d => ⟨d.1,d.2⟩)
  (by intro d; cases d; rfl) (by intro d; rfl)

#print axioms term_roundtrip
#print axioms term_canonical
#print axioms definition
end MirroreaProofFirst.OwnerCodeCodec
