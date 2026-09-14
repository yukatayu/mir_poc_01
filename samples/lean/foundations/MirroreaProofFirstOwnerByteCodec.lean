import MirroreaProofFirstOwnerFullCodec

namespace MirroreaProofFirst.OwnerByteCodec

-- Small total self-delimiting binary-digit codec candidate. The digit bytes
-- are 1 (bit0), 2 (bit1), followed by 0. This is an external proof experiment,
-- not selected public wire. General prefix/suffix laws precede worker changes.
def writeNat (n : Nat) : List UInt8 :=
  if n = 0 then [0] else (if n % 2 = 0 then 1 else 2) :: writeNat (n / 2)
termination_by n

def readNat : List UInt8 → Option (Nat × List UInt8)
  | [] => none
  | b :: rest =>
    if b = 0 then some (0,rest)
    else if b = 1 ∨ b = 2 then do
      let (n,tail) ← readNat rest
      let value := (if b = 1 then 0 else 1) + 2*n
      if value = 0 then none else some (value,tail)
    else none

theorem nat_roundtrip (n : Nat) (suffix : List UInt8) :
    readNat (writeNat n ++ suffix) = some (n,suffix) := by
  induction n using Nat.strongRecOn with
  | ind n ih =>
    by_cases zero : n = 0
    · subst n; simp [writeNat,readNat]
    · have small : n / 2 < n := Nat.div_lt_self (Nat.pos_of_ne_zero zero) (by decide)
      have divided := ih (n/2) small
      have eq := Nat.mod_add_div n 2
      by_cases even : n % 2 = 0
      · have value : 2*(n/2) = n := by omega
        rw [writeNat,if_neg zero,if_pos even,List.cons_append,readNat]
        simp [divided,value,zero]
      · have value : 1+2*(n/2) = n := by have := Nat.mod_lt n (by decide : 0 < 2); omega
        rw [writeNat,if_neg zero,if_neg even,List.cons_append,readNat]
        simp [divided,value]

theorem nat_canonical (bytes : List UInt8) (result : Nat × List UInt8)
    (decoded : readNat bytes = some result) : bytes = writeNat result.1 ++ result.2 := by
  induction bytes generalizing result with
  | nil => simp [readNat] at decoded
  | cons b rest ih =>
    by_cases zero : b = 0
    · subst b
      simp [readNat] at decoded
      subst result
      simp [writeNat]
    · by_cases bit : b = 1 ∨ b = 2
      · cases hr : readNat rest with
        | none => simp [readNat,zero,bit,hr] at decoded
        | some pair =>
          rcases pair with ⟨n,tail⟩
          have suffix := ih _ hr
          rcases bit with rfl | rfl
          · simp [readNat,hr] at decoded
            obtain ⟨ne,rfl⟩ := decoded
            have even : (2*n)%2 = 0 := by omega
            have half : (2*n)/2 = n := by omega
            rw [writeNat,if_neg ne,if_pos even,half]
            simp [suffix]
          · simp [readNat,hr] at decoded
            subst result
            have ne : 1+2*n ≠ 0 := by omega
            have odd : (1+2*n)%2 ≠ 0 := by omega
            have half : (1+2*n)/2 = n := by omega
            rw [writeNat,if_neg ne,if_neg odd,half]
            simp [suffix]
      · simp [readNat,zero,bit] at decoded

theorem nat_consumes (bytes : List UInt8) (result : Nat × List UInt8)
    (decoded : readNat bytes = some result) : result.2.length < bytes.length := by
  have positive : 0 < (writeNat result.1).length := by rw [writeNat]; split <;> simp
  rw [nat_canonical bytes result decoded,List.length_append]
  omega

def writeNats (values : List Nat) : List UInt8 := values.flatMap writeNat

def readNats : Nat → List UInt8 → Option (List Nat × List UInt8)
  | 0,bytes => some ([],bytes)
  | count+1,bytes => do
    let (value,rest) ← readNat bytes
    let (values,tail) ← readNats count rest
    return (value :: values,tail)

theorem nats_roundtrip (values : List Nat) (suffix : List UInt8) :
    readNats values.length (writeNats values ++ suffix) = some (values,suffix) := by
  induction values with
  | nil => rfl
  | cons value rest ih =>
    simp only [List.length_cons,writeNats,List.flatMap_cons,List.append_assoc,readNats,
      nat_roundtrip,Option.bind_eq_bind,Option.bind_some]
    simp only [writeNats] at ih
    rw [ih]
    rfl

theorem nats_canonical (count : Nat) (bytes : List UInt8) (result : List Nat × List UInt8)
    (decoded : readNats count bytes = some result) :
    result.1.length = count ∧ bytes = writeNats result.1 ++ result.2 := by
  induction count generalizing bytes result with
  | zero => simp [readNats] at decoded; subst result; simp [writeNats]
  | succ count ih =>
    cases hn : readNat bytes with
    | none => simp [readNats,hn] at decoded
    | some pair =>
      rcases pair with ⟨value,rest⟩
      cases ht : readNats count rest with
      | none => simp [readNats,hn,ht] at decoded
      | some pair =>
        rcases pair with ⟨values,tail⟩
        simp [readNats,hn,ht] at decoded
        subst result
        obtain ⟨length,suffix⟩ := ih rest (values,tail) ht
        have headEq := nat_canonical bytes (value,rest) hn
        simp_all [writeNats,List.append_assoc]

def writeText (s : String) : List UInt8 :=
  writeNat s.toList.length ++ writeNats (s.toList.map Char.toNat)

def readText (bytes : List UInt8) : Option (String × List UInt8) := do
  let (count,rest) ← readNat bytes
  let (values,tail) ← readNats count rest
  if values.all (fun n => decide ((Char.ofNat n).toNat = n)) then
    some (String.ofList (values.map Char.ofNat),tail)
  else none

theorem text_roundtrip (s : String) (suffix : List UInt8) :
    readText (writeText s ++ suffix) = some (s,suffix) := by
  simp only [writeText,List.append_assoc,readText,nat_roundtrip,
    Option.bind_eq_bind,Option.bind_some]
  have round := nats_roundtrip (s.toList.map Char.toNat) suffix
  simp only [List.length_map] at round
  rw [round]
  simp [List.map_map,Function.comp_def,String.ofList_toList]

theorem text_canonical (bytes : List UInt8) (result : String × List UInt8)
    (decoded : readText bytes = some result) : bytes = writeText result.1 ++ result.2 := by
  cases hn : readNat bytes with
  | none => simp [readText,hn] at decoded
  | some pair =>
    rcases pair with ⟨count,rest⟩
    cases ht : readNats count rest with
    | none => simp [readText,hn,ht] at decoded
    | some pair =>
      rcases pair with ⟨values,tail⟩
      simp only [readText,hn,ht,Option.bind_eq_bind,Option.bind_some] at decoded
      split at decoded
      · rename_i valid
        cases decoded
        have same : (values.map Char.ofNat).map Char.toNat = values := by
          rw [List.map_map]
          have mapped : values.map (fun n => (Char.ofNat n).toNat) = values.map id :=
            List.map_congr_left (fun n member => of_decide_eq_true (List.all_eq_true.mp valid n member))
          simpa using mapped
        obtain ⟨length,body⟩ := nats_canonical count rest (values,tail) ht
        have headEq := nat_canonical bytes (count,rest) hn
        simp only [writeText,String.toList_ofList,List.length_map,same]
        simpa [length,List.append_assoc] using headEq.trans (by rw [body])
      · cases decoded

#print axioms nat_roundtrip
#print axioms nat_canonical
#print axioms nat_consumes
#print axioms nats_roundtrip
#print axioms nats_canonical
#print axioms text_roundtrip
#print axioms text_canonical
end MirroreaProofFirst.OwnerByteCodec
