import MirroreaProofFirstOwnerByteCodec

namespace MirroreaProofFirst.OwnerTreeBytes
open OwnerCodecTree OwnerByteCodec

mutual
def writeTree : Tree → List UInt8
  | .natural n => 3 :: writeNat n
  | .integer (.ofNat n) => 4 :: writeNat n
  | .integer (.negSucc n) => 5 :: writeNat n
  | .text s => 6 :: writeText s
  | .node tag fields => 7 :: (writeNat tag ++ writeNat fields.length ++ writeTrees fields)
termination_by tree => sizeOf tree
def writeTrees : List Tree → List UInt8
  | [] => []
  | tree :: rest => writeTree tree ++ writeTrees rest
termination_by trees => sizeOf trees
end

-- Fuel gives a total parser with explicit bounded rejection. Completeness is
-- stated relative to sufficient fuel; no all-reject parser satisfies that law.
mutual
def readTree : Nat → List UInt8 → Option (Tree × List UInt8)
  | 0,_ => none
  | fuel+1,bytes => match bytes with
    | [] => none
    | b :: rest =>
      if b = 3 then do let (n,tail) ← readNat rest; return (.natural n,tail)
      else if b = 4 then do let (n,tail) ← readNat rest; return (.integer (.ofNat n),tail)
      else if b = 5 then do let (n,tail) ← readNat rest; return (.integer (.negSucc n),tail)
      else if b = 6 then do let (s,tail) ← readText rest; return (.text s,tail)
      else if b = 7 then do
        let (tag,body) ← readNat rest
        let (count,fields) ← readNat body
        let (values,tail) ← readTrees fuel count fields
        return (.node tag values,tail)
      else none
termination_by fuel => fuel
def readTrees (fuel count : Nat) (bytes : List UInt8) : Option (List Tree × List UInt8) :=
  match count,fuel with
  | 0,_ => some ([],bytes)
  | _+1,0 => none
  | count+1,fuel+1 => do
    let (tree,body) ← readTree fuel bytes
    let (trees,tail) ← readTrees fuel count body
    return (tree :: trees,tail)
termination_by fuel
end

mutual
def treeCost : Tree → Nat
  | .node _ fields => 1 + treesCost fields
  | _ => 1
termination_by tree => sizeOf tree
def treesCost : List Tree → Nat
  | [] => 0
  | tree :: rest => 1 + max (treeCost tree) (treesCost rest)
termination_by trees => sizeOf trees
end

theorem roundtrips (fuel : Nat) :
    (∀ tree suffix, treeCost tree ≤ fuel →
      readTree fuel (writeTree tree ++ suffix) = some (tree,suffix)) ∧
    (∀ trees suffix, treesCost trees ≤ fuel →
      readTrees fuel trees.length (writeTrees trees ++ suffix) = some (trees,suffix)) := by
  induction fuel with
  | zero =>
    constructor
    · intro tree suffix fits
      cases tree <;> simp [treeCost] at fits
    · intro trees suffix fits
      cases trees with
      | nil => simp [writeTrees,readTrees]
      | cons tree rest => simp [treesCost] at fits
  | succ fuel ih =>
    obtain ⟨single,many⟩ := ih
    constructor
    · intro tree suffix fits
      cases tree with
      | natural n => simp [writeTree,readTree,nat_roundtrip]
      | integer i =>
        cases i <;> rw [writeTree,List.cons_append,readTree] <;> simp [nat_roundtrip]
      | text s => simp [writeTree,readTree,text_roundtrip]
      | node tag fields =>
        have bound : treesCost fields ≤ fuel := by simp only [treeCost] at fits; omega
        have fieldsRound := many fields suffix bound
        simp [writeTree,readTree,List.append_assoc,nat_roundtrip,fieldsRound]
    · intro trees suffix fits
      cases trees with
      | nil => simp [writeTrees,readTrees]
      | cons tree rest =>
        have treeBound : treeCost tree ≤ fuel := by simp only [treesCost] at fits; omega
        have restBound : treesCost rest ≤ fuel := by simp only [treesCost] at fits; omega
        have first := single tree (writeTrees rest ++ suffix) treeBound
        have remaining := many rest suffix restBound
        simp only [writeTrees,List.append_assoc,List.length_cons,readTrees,
          first,Option.bind_eq_bind,Option.bind_some,remaining]
        rfl

theorem canonicals (fuel : Nat) :
    (∀ bytes result, readTree fuel bytes = some result →
      bytes = writeTree result.1 ++ result.2) ∧
    (∀ count bytes result, readTrees fuel count bytes = some result →
      result.1.length = count ∧ bytes = writeTrees result.1 ++ result.2) := by
  induction fuel with
  | zero =>
    constructor
    · intro bytes result decoded; simp [readTree] at decoded
    · intro count bytes result decoded
      cases count with
      | zero => simp [readTrees] at decoded; subst result; simp [writeTrees]
      | succ count => simp [readTrees] at decoded
  | succ fuel ih =>
    obtain ⟨single,many⟩ := ih
    constructor
    · intro bytes result decoded
      cases bytes with
      | nil => simp [readTree] at decoded
      | cons b rest =>
        by_cases h3 : b = 3
        · subst b
          cases hn : readNat rest with
          | none => simp [readTree,hn] at decoded
          | some pair =>
            rcases pair with ⟨n,tail⟩
            simp [readTree,hn] at decoded
            subst result
            rw [writeTree,List.cons_append,← nat_canonical rest (n,tail) hn]
        · by_cases h4 : b = 4
          · subst b
            cases hn : readNat rest with
            | none => simp [readTree,hn] at decoded
            | some pair =>
              rcases pair with ⟨n,tail⟩
              simp [readTree,hn] at decoded
              subst result
              change (4 : UInt8) :: rest = writeTree (.integer (.ofNat n)) ++ tail
              rw [writeTree,List.cons_append,← nat_canonical rest (n,tail) hn]
          · by_cases h5 : b = 5
            · subst b
              cases hn : readNat rest with
              | none => simp [readTree,hn] at decoded
              | some pair =>
                rcases pair with ⟨n,tail⟩
                simp [readTree,hn] at decoded
                subst result
                rw [writeTree,List.cons_append,← nat_canonical rest (n,tail) hn]
            · by_cases h6 : b = 6
              · subst b
                cases hs : readText rest with
                | none => simp [readTree,hs] at decoded
                | some pair =>
                  rcases pair with ⟨s,tail⟩
                  simp [readTree,hs] at decoded
                  subst result
                  rw [writeTree,List.cons_append,← text_canonical rest (s,tail) hs]
              · by_cases h7 : b = 7
                · subst b
                  cases ht : readNat rest with
                  | none => simp [readTree,ht] at decoded
                  | some pair =>
                    rcases pair with ⟨tag,body⟩
                    cases hc : readNat body with
                    | none => simp [readTree,ht,hc] at decoded
                    | some pair =>
                      rcases pair with ⟨count,fields⟩
                      cases hv : readTrees fuel count fields with
                      | none => simp [readTree,ht,hc,hv] at decoded
                      | some pair =>
                        rcases pair with ⟨values,tail⟩
                        simp [readTree,ht,hc,hv] at decoded
                        subst result
                        obtain ⟨length,valuesEq⟩ := many count fields (values,tail) hv
                        have tagEq := nat_canonical rest (tag,body) ht
                        have countEq := nat_canonical body (count,fields) hc
                        rw [writeTree,List.cons_append]
                        simp_all [List.append_assoc]
                · simp [readTree,h3,h4,h5,h6,h7] at decoded
    · intro count bytes result decoded
      cases count with
      | zero => simp [readTrees] at decoded; subst result; simp [writeTrees]
      | succ count =>
        cases ht : readTree fuel bytes with
        | none => simp [readTrees,ht] at decoded
        | some pair =>
          rcases pair with ⟨tree,body⟩
          cases hs : readTrees fuel count body with
          | none => simp [readTrees,ht,hs] at decoded
          | some pair =>
            rcases pair with ⟨trees,tail⟩
            simp [readTrees,ht,hs] at decoded
            subst result
            obtain ⟨length,tailEq⟩ := many count body (trees,tail) hs
            have headEq := single bytes (tree,body) ht
            simp_all [writeTrees,List.append_assoc]

theorem cost_bounds :
    (∀ tree, treeCost tree + 1 ≤ (writeTree tree).length) ∧
    (∀ trees, treesCost trees ≤ (writeTrees trees).length) := by
  have natPositive (n : Nat) : 0 < (writeNat n).length := by rw [writeNat]; split <;> simp
  have textPositive (s : String) : 0 < (writeText s).length := by
    have := natPositive s.toList.length
    simp only [writeText,List.length_append]
    omega
  have single : ∀ tree, treeCost tree + 1 ≤ (writeTree tree).length := by
    apply OwnerCodeCodec.tree_induction
    · intro n; simp only [treeCost,writeTree,List.length_cons]; have := natPositive n; omega
    · intro i
      cases i with
      | ofNat n =>
        have := natPositive n
        rw [treeCost.eq_def,writeTree]
        change 2 ≤ (writeNat n).length + 1
        omega
      | negSucc n =>
        have := natPositive n
        rw [treeCost.eq_def,writeTree]
        change 2 ≤ (writeNat n).length + 1
        omega
    · intro s; simp only [treeCost,writeTree,List.length_cons]; have := textPositive s; omega
    · intro tag fields ih
      have body : treesCost fields ≤ (writeTrees fields).length := by
        induction fields with
        | nil => simp [treesCost,writeTrees]
        | cons tree rest tail =>
          have first := ih tree (by simp)
          have remaining := tail (fun t member => ih t (by simp [member]))
          simp only [treesCost,writeTrees,List.length_append]
          omega
      have tagSize := natPositive tag
      have countSize := natPositive fields.length
      simp only [treeCost,writeTree,List.length_cons,List.length_append]
      omega
  refine ⟨single,?_⟩
  intro trees
  induction trees with
  | nil => simp [treesCost,writeTrees]
  | cons tree rest ih =>
    have first := single tree
    simp only [treesCost,writeTrees,List.length_append]
    omega

def encode (tree : Tree) : List UInt8 := writeTree tree
def decodeAt (fuel : Nat) (bytes : List UInt8) : Option Tree := do
  let (tree,tail) ← readTree fuel bytes
  if tail.isEmpty then some tree else none
def decode (bytes : List UInt8) : Option Tree := do
  let (tree,tail) ← readTree bytes.length bytes
  if tail.isEmpty then some tree else none

theorem decode_encode (tree : Tree) : decode (encode tree) = some tree := by
  have bound : treeCost tree ≤ (writeTree tree).length := by have := cost_bounds.1 tree; omega
  have round := (roundtrips (writeTree tree).length).1 tree [] bound
  simp only [List.append_nil] at round
  simp [decode,encode,round]

theorem decode_canonical (bytes : List UInt8) (tree : Tree)
    (decoded : decode bytes = some tree) : encode tree = bytes := by
  cases h : readTree bytes.length bytes with
  | none => simp [decode,h] at decoded
  | some pair =>
    rcases pair with ⟨value,tail⟩
    simp [decode,h] at decoded
    obtain ⟨rfl,rfl⟩ := decoded
    simpa [encode] using ((canonicals bytes.length).1 bytes (value,[]) h).symm

theorem decodeAt_encode (fuel : Nat) (tree : Tree) (fits : treeCost tree ≤ fuel) :
    decodeAt fuel (encode tree) = some tree := by
  have round := (roundtrips fuel).1 tree [] fits
  simp only [List.append_nil] at round
  simp [decodeAt,encode,round]

theorem decodeAt_canonical (fuel : Nat) (bytes : List UInt8) (tree : Tree)
    (decoded : decodeAt fuel bytes = some tree) : encode tree = bytes := by
  cases h : readTree fuel bytes with
  | none => simp [decodeAt,h] at decoded
  | some pair =>
    rcases pair with ⟨value,tail⟩
    simp [decodeAt,h] at decoded
    obtain ⟨rfl,rfl⟩ := decoded
    simpa [encode] using ((canonicals fuel).1 bytes (value,[]) h).symm

#print axioms writeTree
#print axioms readTree
#print axioms roundtrips
#print axioms canonicals
#print axioms cost_bounds
#print axioms decode_encode
#print axioms decode_canonical
#print axioms decodeAt_encode
#print axioms decodeAt_canonical
end MirroreaProofFirst.OwnerTreeBytes
