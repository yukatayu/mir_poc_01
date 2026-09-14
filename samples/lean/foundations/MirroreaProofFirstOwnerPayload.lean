import MirroreaProofFirstOwnerPacketCodec

namespace MirroreaProofFirst.OwnerPayload

-- Private experimental operational profile, separate from the unbounded
-- canonical byte laws and from semantic/auth admission. These bounds do not
-- purport to bound least-fixed-point/checker CPU work or OS scheduling.
def maxBytes : Nat := 65536
def maxFuel : Nat := 256
def maxRecords : Nat := 64
def maxDigits : Nat := 128
def maxTextScalars : Nat := 256
def reply := OwnerCodecTree.sum OwnerCodecTree.natural OwnerPacketCodec.result

def nextDigits (count : Nat) (byte : UInt8) : Nat :=
  if byte = 1 ∨ byte = 2 then count+1 else 0

inductive DigitBound (limit : Nat) : Nat → List UInt8 → Prop where
  | nil : count ≤ limit → DigitBound limit count []
  | cons : count ≤ limit → DigitBound limit (nextDigits count byte) rest → DigitBound limit count (byte :: rest)

def digitCheck (limit count : Nat) : List UInt8 → Bool
  | [] => decide (count ≤ limit)
  | byte :: rest => if count ≤ limit then digitCheck limit (nextDigits count byte) rest else false

theorem digits_exact (limit count : Nat) (bytes : List UInt8) :
    digitCheck limit count bytes = true ↔ DigitBound limit count bytes := by
  induction bytes generalizing count with
  | nil =>
      simp only [digitCheck,decide_eq_true_eq]
      exact ⟨DigitBound.nil,fun bound => by cases bound; assumption⟩
  | cons byte rest ih =>
      constructor
      · intro checked
        by_cases fits : count ≤ limit
        · exact .cons fits ((ih _).mp (by simpa [digitCheck,fits] using checked))
        · simp [digitCheck,fits] at checked
      · intro bound
        cases bound with
        | cons fits rest => simp [digitCheck,fits,(ih _).mpr rest]

def TextHead (limit : Nat) (bytes : List UInt8) : Prop :=
  match bytes with
  | [] => True
  | byte :: rest => if byte = 6 then
      ∃ count tail, OwnerByteCodec.readNat rest = some (count,tail) ∧ count ≤ limit
    else True

def textHeadCheck (limit : Nat) : List UInt8 → Bool
  | [] => true
  | byte :: rest => if byte = 6 then
      match OwnerByteCodec.readNat rest with
      | none => false
      | some pair => decide (pair.1 ≤ limit)
    else true

theorem text_head_exact (limit : Nat) (bytes : List UInt8) : textHeadCheck limit bytes = true ↔ TextHead limit bytes := by
  cases bytes with
  | nil => simp [textHeadCheck,TextHead]
  | cons byte rest =>
      by_cases text : byte = 6
      · cases parsed : OwnerByteCodec.readNat rest with
        | none => simp [textHeadCheck,TextHead,text,parsed]
        | some pair =>
            rcases pair with ⟨count,tail⟩
            simp [textHeadCheck,TextHead,text,parsed,Prod.mk.injEq]
      · simp [textHeadCheck,TextHead,text]

inductive TextBound (limit : Nat) : List UInt8 → Prop where
  | nil : TextBound limit []
  | cons : TextHead limit (byte :: rest) → TextBound limit rest → TextBound limit (byte :: rest)

-- Invoked only after digitCheck: calls to readNat see bounded runs of digit
-- bytes. This guards text's scalar-list recursion, independently of tree fuel.
def textCheck (limit : Nat) : List UInt8 → Bool
  | [] => true
  | byte :: rest => if textHeadCheck limit (byte :: rest) then textCheck limit rest else false

theorem text_exact (limit : Nat) (bytes : List UInt8) : textCheck limit bytes = true ↔ TextBound limit bytes := by
  induction bytes with
  | nil => exact ⟨fun _ => .nil,fun _ => rfl⟩
  | cons byte rest ih =>
      constructor
      · intro checked
        by_cases head : textHeadCheck limit (byte :: rest) = true
        · exact .cons ((text_head_exact _ _).mp head) (ih.mp (by simpa [textCheck,head] using checked))
        · simp [textCheck,head] at checked
      · intro bound
        cases bound with
        | cons head rest => simp [textCheck,(text_head_exact _ _).mpr head,ih.mpr rest]

def FitsBytes (bytes : List UInt8) : Prop :=
  DigitBound maxDigits 0 bytes ∧ TextBound maxTextScalars bytes

def payload (assigned : OwnerEvaluator.Assignment p) (a : Nat) (bytes : List UInt8) : Sum Nat OwnerEvaluator.Result :=
  if bytes.length > maxBytes then .inl 1
  else if !digitCheck maxDigits 0 bytes then .inl 3
  else if !textCheck maxTextScalars bytes then .inl 3
  else match OwnerPacketCodec.decodeAt (OwnerFullCodec.request p a) maxFuel bytes with
  | none => .inl 0
  | some request =>
      if request.1.definitions > maxRecords || request.1.count > maxRecords then .inl 2
      else .inr (OwnerEvaluator.run assigned request.1 request.2)

theorem encoded_payload (assigned : OwnerEvaluator.Assignment p)
    (input : OwnerImage.Image p a × InvocationBoundary.Ticket)
    (size : (OwnerPacketCodec.encode (OwnerFullCodec.request p a) input).length ≤ maxBytes)
    (fits : FitsBytes (OwnerPacketCodec.encode (OwnerFullCodec.request p a) input))
    (fuel : OwnerTreeBytes.treeCost ((OwnerFullCodec.request p a).encode input) ≤ maxFuel)
    (definitions : input.1.definitions ≤ maxRecords) (instances : input.1.count ≤ maxRecords) :
    payload assigned a (OwnerPacketCodec.encode (OwnerFullCodec.request p a) input) =
      .inr (OwnerEvaluator.run assigned input.1 input.2) := by
  have digits := (digits_exact _ _ _).mpr fits.1
  have text := (text_exact _ _).mpr fits.2
  simp [payload,Nat.not_lt.mpr size,digits,text,OwnerPacketCodec.bounded_roundtrip _ _ _ fuel,
    Nat.not_lt.mpr definitions,Nat.not_lt.mpr instances]

#print axioms digits_exact
#print axioms text_head_exact
#print axioms text_exact
#print axioms encoded_payload
end MirroreaProofFirst.OwnerPayload
