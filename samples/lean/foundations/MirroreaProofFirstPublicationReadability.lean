import SourcePublicationWorkerSupport

namespace MirroreaProofFirst.PublicationReadability
open OwnerCodecTree

-- Finite private recipient profile. This does not change mathematical Nat,
-- authorize the recipient, or prove the external Python reader implementation.
def payload (p a : Nat) := optional (SourcePublicationWorker.output p a)
def wire (p a : Nat) := product natural (payload p a)

def PayloadFits (output : Option (SourcePublicationWorker.PrivateOutput p a)) : Prop :=
  let bytes := OwnerPacketCodec.encode (payload p a) output
  OwnerPayload.DigitBound 128 0 bytes ∧ OwnerPayload.TextBound 4096 bytes ∧
    OwnerTreeBytes.treeCost ((payload p a).encode output) + 3 ≤ 256

def payloadCheck (output : Option (SourcePublicationWorker.PrivateOutput p a)) : Bool :=
  let tree := (payload p a).encode output
  let bytes := OwnerTreeBytes.encode tree
  OwnerPayload.digitCheck 128 0 bytes && OwnerPayload.textCheck 4096 bytes &&
    decide (OwnerTreeBytes.treeCost tree + 3 ≤ 256)

theorem payloadCheck_exact : payloadCheck output = true ↔ PayloadFits output := by
  simp [payloadCheck,PayloadFits,OwnerPacketCodec.encode,OwnerPayload.digits_exact,
    OwnerPayload.text_exact,and_assoc]

-- Every concrete private status wrapper (0,1,2) is bounded independently of
-- the output value. Checking the shared payload once is sufficient.
def WireReadable (tag : Nat) (output : Option (SourcePublicationWorker.PrivateOutput p a)) : Prop :=
  let bytes := OwnerPacketCodec.encode (wire p a) (tag,output)
  OwnerPayload.DigitBound 128 0 bytes ∧ OwnerPayload.TextBound 4096 bytes ∧
    OwnerPacketCodec.decodeAt (wire p a) 256 bytes = some (tag,output)

theorem wrapper_digits (tag : Nat) (small : tag < 3)
    (output : Option (SourcePublicationWorker.PrivateOutput p a)) :
    OwnerPayload.digitCheck 128 0 (OwnerPacketCodec.encode (wire p a) (tag,output)) =
      OwnerPayload.digitCheck 128 0 (OwnerPacketCodec.encode (payload p a) output) := by
  have choices : tag = 0 ∨ tag = 1 ∨ tag = 2 := by omega
  rcases choices with rfl | rfl | rfl <;>
    simp [wire,OwnerPacketCodec.encode,OwnerCodecTree.product,OwnerCodecTree.natural,
      OwnerTreeBytes.encode,OwnerTreeBytes.writeTree,OwnerTreeBytes.writeTrees,
      OwnerByteCodec.writeNat,OwnerPayload.digitCheck,OwnerPayload.nextDigits]

theorem wrapper_text (tag : Nat) (small : tag < 3)
    (output : Option (SourcePublicationWorker.PrivateOutput p a)) :
    OwnerPayload.textCheck 4096 (OwnerPacketCodec.encode (wire p a) (tag,output)) =
      OwnerPayload.textCheck 4096 (OwnerPacketCodec.encode (payload p a) output) := by
  have choices : tag = 0 ∨ tag = 1 ∨ tag = 2 := by omega
  rcases choices with rfl | rfl | rfl <;>
    simp [wire,OwnerPacketCodec.encode,OwnerCodecTree.product,OwnerCodecTree.natural,
      OwnerTreeBytes.encode,OwnerTreeBytes.writeTree,OwnerTreeBytes.writeTrees,
      OwnerByteCodec.writeNat,OwnerPayload.textCheck,OwnerPayload.textHeadCheck]

theorem wrapper_cost (tag : Nat)
    (output : Option (SourcePublicationWorker.PrivateOutput p a)) :
    OwnerTreeBytes.treeCost ((wire p a).encode (tag,output)) ≤
      OwnerTreeBytes.treeCost ((payload p a).encode output) + 3 := by
  simp [wire,OwnerCodecTree.product,OwnerCodecTree.natural,OwnerTreeBytes.treeCost,
    OwnerTreeBytes.treesCost]
  omega

theorem payload_readable (fit : PayloadFits output) (small : tag < 3) :
    WireReadable tag output := by
  obtain ⟨digits,text,cost⟩ := fit
  refine ⟨?_,?_,?_⟩
  · apply (OwnerPayload.digits_exact _ _ _).mp
    rw [wrapper_digits tag small]
    exact (OwnerPayload.digits_exact _ _ _).mpr digits
  · apply (OwnerPayload.text_exact _ _).mp
    rw [wrapper_text tag small]
    exact (OwnerPayload.text_exact _ _).mpr text
  · exact OwnerPacketCodec.bounded_roundtrip _ _ _ (Nat.le_trans (wrapper_cost _ _) cost)

theorem none_fits : PayloadFits (p:=p) (a:=a) none := by
  apply payloadCheck_exact.mp
  simp [payloadCheck,payload,OwnerCodecTree.optional,OwnerCodecTree.iso,
    OwnerCodecTree.sum,OwnerCodecTree.unit,OwnerTreeBytes.encode,
    OwnerTreeBytes.writeTree,OwnerTreeBytes.writeTrees,OwnerTreeBytes.treeCost,
    OwnerTreeBytes.treesCost,OwnerByteCodec.writeNat,OwnerPayload.digitCheck,
    OwnerPayload.nextDigits,OwnerPayload.textCheck,OwnerPayload.textHeadCheck]

#print axioms none_fits
#print axioms payloadCheck_exact
#print axioms wrapper_digits
#print axioms wrapper_text
#print axioms wrapper_cost
#print axioms payload_readable
end MirroreaProofFirst.PublicationReadability
