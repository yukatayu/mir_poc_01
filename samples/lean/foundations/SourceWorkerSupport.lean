-- External factoring of the existing private protocol/IO support; no entrypoint.
import MirroreaProofFirstSourceInput

open MirroreaProofFirst
namespace SourceWorker
open OwnerCodecTree

def failure : Codec ReferenceSource.Failure where
  encode := fun value => .natural (match value with
    | .awaiting => 0 | .staticType => 1 | .elaboration => 2 | .rejected => 3
    | .missingCreated => 4 | .cancelled => 5
    | .normalization .absent => 6 | .normalization .exhausted => 7
    | .normalization .ownerDenied => 8 | .normalization .ownerNotLive => 9
    | .normalization .mutationRejected => 10)
  decode := fun tree => match tree with
    | .natural 0 => some .awaiting | .natural 1 => some .staticType
    | .natural 2 => some .elaboration | .natural 3 => some .rejected
    | .natural 4 => some .missingCreated | .natural 5 => some .cancelled
    | .natural 6 => some (.normalization .absent) | .natural 7 => some (.normalization .exhausted)
    | .natural 8 => some (.normalization .ownerDenied) | .natural 9 => some (.normalization .ownerNotLive)
    | .natural 10 => some (.normalization .mutationRejected) | _ => none
  roundtrip := by
    intro value
    cases value <;> try rfl
    case normalization reason => cases reason <;> rfl
  canonical := by intro tree value decoded; split at decoded <;> cases decoded <;> rfl

def status : Codec ReferenceSource.Status := iso (sum boolean failure)
  (fun value => match value with | .ready => .inl false | .waiting => .inl true | .failed error => .inr error)
  (fun value => match value with | .inl false => .ready | .inl true => .waiting | .inr error => .failed error)
  (by intro value; cases value <;> rfl)
  (by intro value; rcases value with flag | error; cases flag <;> rfl; rfl)

def plainValue : Codec SourceAuthoring.Value := iso
  (sum unit (sum natural (sum natural (product integer boolean))))
  (fun value => match value with
    | .unit => .inl () | .definition key => .inr (.inl key)
    | .callable key => .inr (.inr (.inl key)) | .integer value mutable => .inr (.inr (.inr (value,mutable))))
  (fun value => match value with
    | .inl _ => .unit | .inr (.inl key) => .definition key
    | .inr (.inr (.inl key)) => .callable key | .inr (.inr (.inr (value,mutable))) => .integer value mutable)
  (by intro value; cases value <;> rfl)
  (by intro value; rcases value with ⟨⟩ | key | key | ⟨value,mutable⟩ <;> rfl)
def value : Codec ReferenceSourceData.Value := iso (sum plainValue natural)
  (fun value => match value with | .plain value => .inl value | .reference key => .inr key)
  (fun value => match value with | .inl value => .plain value | .inr key => .reference key)
  (by intro value; cases value <;> rfl) (by intro value; cases value <;> rfl)

-- Privileged pipe reply for this nonproduction experiment. It includes private
-- values and authority-bearing owner input. It is NOT a public observer stream,
-- a disclosure grant, an authenticated image or a populated-state import.
-- Only the assigned trusted launcher may read it in the component experiment.
structure PrivateOutput (p a : Nat) where
  status : ReferenceSource.Status
  completed : Nat
  remaining : Nat
  writes : Nat
  replies : Nat
  values : ReferenceSourceData.Values
  pending : Option InvocationBoundary.Ticket
  image : OwnerImage.Image p a

def output (p a : Nat) : Codec (PrivateOutput p a) := iso
  (product status (product natural (product natural (product natural (product natural
    (product (list (product text value)) (product (optional OwnerFullCodec.ticket) (OwnerFullCodec.image p a))))))))
  (fun s => (s.status,s.completed,s.remaining,s.writes,s.replies,s.values,s.pending,s.image))
  (fun (status,completed,remaining,writes,replies,values,pending,image) =>
    ⟨status,completed,remaining,writes,replies,values,pending,image⟩)
  (by intro s; cases s; rfl) (by intro s; rfl)

def project (s : QualifiedCustody.State p a) : PrivateOutput p a :=
  let session := s.privateState.session
  ⟨session.status,session.completed.length,session.remaining.length,session.state.source.writes.length,
    s.privateState.replies.length,session.state.source.values,
    session.state.source.waiting.map (fun saved => saved.entry.ticket),QualifiedPublication.image s.privateState⟩

def reply (p a : Nat) := product boolean (optional (output p a))
def exchange (assigned : SourceInput.Assignment p a) (seed : SourceInput.Bootstrap a)
    (s : Option (QualifiedCustody.State p a)) (command : SourceInput.Input p a) :=
  let (next,accepted) := SourceInput.transition assigned seed s command
  (next,(accepted,next.map project))

theorem exchange_preserves (valid : SourceInput.Invariant assigned seed s) :
    SourceInput.Invariant assigned seed (exchange assigned seed s command).1 :=
  SourceInput.transition_preserves valid

theorem reply_bytes (message : Bool × Option (PrivateOutput p a)) :
    OwnerPacketCodec.decode (reply p a) (OwnerPacketCodec.encode (reply p a) message) = some message :=
  OwnerPacketCodec.roundtrip _ _

#print axioms failure
#print axioms status
#print axioms value
#print axioms output
#print axioms exchange_preserves
#print axioms reply_bytes

def readExact (input : IO.FS.Stream) (count : Nat) : IO ByteArray := do
  let mut bytes := ByteArray.empty
  for _ in [:count] do
    if bytes.size == count then break
    let next ← input.read (min 4096 (count-bytes.size)).toUSize
    if next.isEmpty then throw (IO.userError "short frame")
    bytes := bytes ++ next
  return bytes

def readFrame (input : IO.FS.Stream) : IO (Option ByteArray) := do
  let first ← input.read 1
  if first.isEmpty then return none
  let rest ← readExact input 3
  let size := (first++rest).data.foldl (fun n b => n*256+b.toNat) 0
  if size > 65536 then throw (IO.userError "oversized frame")
  return some (← readExact input size)

def decodeFrame (codec : Codec T) (bytes : ByteArray) : IO T := do
  unless SourceCodec.compactFits bytes.data.toList do throw (IO.userError "source preflight refused")
  let some value := OwnerPacketCodec.decodeAt codec 256 bytes.data.toList |
    throw (IO.userError "invalid source input")
  return value

def respond (outputStream : IO.FS.Stream) (message : Bool × Option (PrivateOutput p a)) : IO Unit := do
  let bytes := OwnerPacketCodec.encode (reply p a) message
  if bytes.length > 65536 then throw (IO.userError "private reply exceeds component frame bound")
  let header := [24,16,8,0].map (fun shift => UInt8.ofNat (bytes.length / 2^shift % 256))
  outputStream.write ⟨(header++bytes).toArray⟩
  outputStream.flush


end SourceWorker
