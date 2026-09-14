import MirroreaProofFirstOwnerRecordCodecs

namespace MirroreaProofFirst.OwnerTreeCodecs
open OwnerCodecTree

def encodeFormula (element : Codec K) : Support.Formula K → Tree
  | .top => .node 0 []
  | .bottom => .node 1 []
  | .ref value => .node 2 [element.encode value]
  | .both left right => .node 3 [encodeFormula element left,encodeFormula element right]
  | .either left right => .node 4 [encodeFormula element left,encodeFormula element right]

def decodeFormula (element : Codec K) (tree : Tree) : Option (Support.Formula K) := match tree with
  | .node 0 [] => some .top
  | .node 1 [] => some .bottom
  | .node 2 [value] => .ref <$> element.decode value
  | .node 3 [left,right] => do return .both (← decodeFormula element left) (← decodeFormula element right)
  | .node 4 [left,right] => do return .either (← decodeFormula element left) (← decodeFormula element right)
  | _ => none
termination_by sizeOf tree

theorem formula_roundtrip (element : Codec K) (value : Support.Formula K) : decodeFormula element (encodeFormula element value) = some value := by
  induction value <;> simp_all [encodeFormula,decodeFormula,Codec.roundtrip]

theorem formula_canonical (element : Codec K) (tree : Tree) (value : Support.Formula K)
    (decoded : decodeFormula element tree = some value) : encodeFormula element value = tree := by
  induction tree using OwnerCodeCodec.tree_induction generalizing value with
  | hn n => simp [decodeFormula] at decoded
  | hi i => simp [decodeFormula] at decoded
  | hs t => simp [decodeFormula] at decoded
  | node tag fields ih =>
    unfold decodeFormula at decoded
    split at decoded
    · rename_i shape
      simp only [Option.some.injEq] at decoded
      subst value
      exact shape.symm
    · rename_i shape
      simp only [Option.some.injEq] at decoded
      subst value
      exact shape.symm
    · rename_i encoded shape
      cases hc : element.decode encoded <;> simp [hc] at decoded
      subst value
      simpa [encodeFormula,element.canonical _ _ hc] using shape.symm
    · rename_i left right shape
      rcases Tree.node.inj shape with ⟨rfl,rfl⟩
      cases hl : decodeFormula element left with
      | none => simp [hl] at decoded
      | some a =>
        cases hr : decodeFormula element right with
        | none => simp [hl,hr] at decoded
        | some b =>
          simp [hl,hr] at decoded
          subst value
          simp only [encodeFormula]
          rw [ih left (by simp) a hl,ih right (by simp) b hr]
    · rename_i left right shape
      rcases Tree.node.inj shape with ⟨rfl,rfl⟩
      cases hl : decodeFormula element left with
      | none => simp [hl] at decoded
      | some a =>
        cases hr : decodeFormula element right with
        | none => simp [hl,hr] at decoded
        | some b =>
          simp [hl,hr] at decoded
          subst value
          simp only [encodeFormula]
          rw [ih left (by simp) a hl,ih right (by simp) b hr]
    · cases decoded

def formula (element : Codec K) : Codec (Support.Formula K) :=
  ⟨encodeFormula element,decodeFormula element,formula_roundtrip element,formula_canonical element⟩

#print axioms formula

def encodePolicy  : CurrentUse.PolicyExpr → Tree
  | .leaf value => .node 0 [OwnerRecordCodecs.need.encode value]
  | .both left right => .node 1 [encodePolicy left,encodePolicy right]
  | .either left right => .node 2 [encodePolicy left,encodePolicy right]

def decodePolicy  (tree : Tree) : Option (CurrentUse.PolicyExpr) := match tree with
  | .node 0 [value] => .leaf <$> OwnerRecordCodecs.need.decode value
  | .node 1 [left,right] => do return .both (← decodePolicy left) (← decodePolicy right)
  | .node 2 [left,right] => do return .either (← decodePolicy left) (← decodePolicy right)
  | _ => none
termination_by sizeOf tree

theorem policy_roundtrip  (value : CurrentUse.PolicyExpr) : decodePolicy (encodePolicy value) = some value := by
  induction value <;> simp_all [encodePolicy,decodePolicy,Codec.roundtrip]

theorem policy_canonical  (tree : Tree) (value : CurrentUse.PolicyExpr)
    (decoded : decodePolicy tree = some value) : encodePolicy value = tree := by
  induction tree using OwnerCodeCodec.tree_induction generalizing value with
  | hn n => simp [decodePolicy] at decoded
  | hi i => simp [decodePolicy] at decoded
  | hs t => simp [decodePolicy] at decoded
  | node tag fields ih =>
    unfold decodePolicy at decoded
    split at decoded
    · rename_i encoded shape
      cases hc : OwnerRecordCodecs.need.decode encoded <;> simp [hc] at decoded
      subst value
      simpa [encodePolicy,OwnerRecordCodecs.need.canonical _ _ hc] using shape.symm
    · rename_i left right shape
      rcases Tree.node.inj shape with ⟨rfl,rfl⟩
      cases hl : decodePolicy left with
      | none => simp [hl] at decoded
      | some a =>
        cases hr : decodePolicy right with
        | none => simp [hl,hr] at decoded
        | some b =>
          simp [hl,hr] at decoded
          subst value
          simp only [encodePolicy]
          rw [ih left (by simp) a hl,ih right (by simp) b hr]
    · rename_i left right shape
      rcases Tree.node.inj shape with ⟨rfl,rfl⟩
      cases hl : decodePolicy left with
      | none => simp [hl] at decoded
      | some a =>
        cases hr : decodePolicy right with
        | none => simp [hl,hr] at decoded
        | some b =>
          simp [hl,hr] at decoded
          subst value
          simp only [encodePolicy]
          rw [ih left (by simp) a hl,ih right (by simp) b hr]
    · cases decoded

def policy  : Codec (CurrentUse.PolicyExpr) :=
  ⟨encodePolicy,decodePolicy,policy_roundtrip,policy_canonical⟩

#print axioms policy

def encodeWitness  : CurrentUse.Witness → Tree
  | .leaf value => .node 0 [OwnerRecordCodecs.claim.encode value]
  | .both left right => .node 1 [encodeWitness left,encodeWitness right]
  | .left inner => .node 2 [encodeWitness inner]
  | .right inner => .node 3 [encodeWitness inner]

def decodeWitness  (tree : Tree) : Option (CurrentUse.Witness) := match tree with
  | .node 0 [value] => .leaf <$> OwnerRecordCodecs.claim.decode value
  | .node 1 [left,right] => do return .both (← decodeWitness left) (← decodeWitness right)
  | .node 2 [inner] => .left <$> decodeWitness inner
  | .node 3 [inner] => .right <$> decodeWitness inner
  | _ => none
termination_by sizeOf tree

theorem witness_roundtrip  (value : CurrentUse.Witness) : decodeWitness (encodeWitness value) = some value := by
  induction value <;> simp_all [encodeWitness,decodeWitness,Codec.roundtrip]

theorem witness_canonical  (tree : Tree) (value : CurrentUse.Witness)
    (decoded : decodeWitness tree = some value) : encodeWitness value = tree := by
  induction tree using OwnerCodeCodec.tree_induction generalizing value with
  | hn n => simp [decodeWitness] at decoded
  | hi i => simp [decodeWitness] at decoded
  | hs t => simp [decodeWitness] at decoded
  | node tag fields ih =>
    unfold decodeWitness at decoded
    split at decoded
    · rename_i encoded shape
      cases hc : OwnerRecordCodecs.claim.decode encoded <;> simp [hc] at decoded
      subst value
      simpa [encodeWitness,OwnerRecordCodecs.claim.canonical _ _ hc] using shape.symm
    · rename_i left right shape
      rcases Tree.node.inj shape with ⟨rfl,rfl⟩
      cases hl : decodeWitness left with
      | none => simp [hl] at decoded
      | some a =>
        cases hr : decodeWitness right with
        | none => simp [hl,hr] at decoded
        | some b =>
          simp [hl,hr] at decoded
          subst value
          simp only [encodeWitness]
          rw [ih left (by simp) a hl,ih right (by simp) b hr]
    · rename_i inner shape
      rcases Tree.node.inj shape with ⟨rfl,rfl⟩
      cases ht : decodeWitness inner <;> simp [ht] at decoded
      subst value
      simp only [encodeWitness]
      rw [ih inner (by simp) _ ht]
    · rename_i inner shape
      rcases Tree.node.inj shape with ⟨rfl,rfl⟩
      cases ht : decodeWitness inner <;> simp [ht] at decoded
      subst value
      simp only [encodeWitness]
      rw [ih inner (by simp) _ ht]
    · cases decoded

def witness  : Codec (CurrentUse.Witness) :=
  ⟨encodeWitness,decodeWitness,witness_roundtrip,witness_canonical⟩

#print axioms witness

end MirroreaProofFirst.OwnerTreeCodecs
