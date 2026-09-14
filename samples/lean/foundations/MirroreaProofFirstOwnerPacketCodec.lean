import MirroreaProofFirstOwnerTreeBytes

namespace MirroreaProofFirst.OwnerPacketCodec
open OwnerCodecTree

def encode (codec : Codec T) (value : T) : List UInt8 :=
  OwnerTreeBytes.encode (codec.encode value)
def decode (codec : Codec T) (bytes : List UInt8) : Option T :=
  OwnerTreeBytes.decode bytes >>= codec.decode

def decodeAt (codec : Codec T) (fuel : Nat) (bytes : List UInt8) : Option T :=
  OwnerTreeBytes.decodeAt fuel bytes >>= codec.decode

theorem bounded_roundtrip (codec : Codec T) (fuel : Nat) (value : T)
    (fits : OwnerTreeBytes.treeCost (codec.encode value) ≤ fuel) :
    decodeAt codec fuel (encode codec value) = some value := by
  simp [decodeAt,encode,OwnerTreeBytes.decodeAt_encode fuel _ fits,codec.roundtrip]

theorem bounded_canonical (codec : Codec T) (fuel : Nat) (bytes : List UInt8) (value : T)
    (decoded : decodeAt codec fuel bytes = some value) : bytes = encode codec value := by
  cases h : OwnerTreeBytes.decodeAt fuel bytes with
  | none => simp [decodeAt,h] at decoded
  | some tree =>
    have atValue : codec.decode tree = some value := by simpa [decodeAt,h] using decoded
    rw [encode,codec.canonical tree value atValue]
    exact (OwnerTreeBytes.decodeAt_canonical fuel bytes tree h).symm

theorem roundtrip (codec : Codec T) (value : T) : decode codec (encode codec value) = some value := by
  simp [decode,encode,OwnerTreeBytes.decode_encode,codec.roundtrip]

theorem canonical (codec : Codec T) (bytes : List UInt8) (value : T)
    (decoded : decode codec bytes = some value) : bytes = encode codec value := by
  cases h : OwnerTreeBytes.decode bytes with
  | none => simp [decode,h] at decoded
  | some tree =>
    have valueAt : codec.decode tree = some value := by simpa [decode,h] using decoded
    have representation := codec.canonical tree value valueAt
    rw [encode,representation]
    exact (OwnerTreeBytes.decode_canonical bytes tree h).symm

theorem exact (codec : Codec T) (bytes : List UInt8) (value : T) :
    decode codec bytes = some value ↔ bytes = encode codec value :=
  ⟨canonical codec bytes value,fun same => same ▸ roundtrip codec value⟩

def failure : Codec OwnerEvaluator.Failure := iso (product boolean boolean)
  (fun reason => match reason with
    | .wrongRealm => (false,false)
    | .invalidConfiguration => (false,true)
    | .unauthorized => (true,false)
    | .executionFailure => (true,true))
  (fun pair => match pair with
    | (false,false) => .wrongRealm
    | (false,true) => .invalidConfiguration
    | (true,false) => .unauthorized
    | (true,true) => .executionFailure)
  (by intro reason; cases reason <;> rfl)
  (by intro pair; rcases pair with ⟨a,b⟩; cases a <;> cases b <;> rfl)

def result : Codec OwnerEvaluator.Result := iso (sum failure integer)
  (fun result => match result with | .rejected reason => .inl reason | .value n => .inr n)
  (fun result => match result with | .inl reason => .rejected reason | .inr n => .value n)
  (by intro result; cases result <;> rfl) (by intro result; cases result <;> rfl)

def evaluate (assigned : OwnerEvaluator.Assignment p) (a : Nat) (bytes : List UInt8) :
    Option (List UInt8) := do
  let input ← decode (OwnerFullCodec.request p a) bytes
  return encode result (OwnerEvaluator.run assigned input.1 input.2)

theorem exchange_roundtrip (assigned : OwnerEvaluator.Assignment p)
    (input : OwnerImage.Image p a × InvocationBoundary.Ticket) :
    evaluate assigned a (encode (OwnerFullCodec.request p a) input) =
      some (encode result (OwnerEvaluator.run assigned input.1 input.2)) := by
  simp [evaluate,roundtrip]

#print axioms roundtrip
#print axioms canonical
#print axioms exact
#print axioms exchange_roundtrip
#print axioms bounded_roundtrip
#print axioms bounded_canonical
end MirroreaProofFirst.OwnerPacketCodec
