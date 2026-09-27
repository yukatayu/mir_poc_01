import MirroreaProofFirstOwnerEndpointBudget
import SourceWorkerSupport

open MirroreaProofFirst
namespace OwnerBudgetWorker
open OwnerCodecTree

-- Authenticated publisher messages are a separate boundary. This experiment
-- owns a private pipe; no arbitrary network peer may obtain this command API.
-- A freeze ack is emitted only after the guarded local transition. Native
-- exceptions terminate the namespace; no catch, reset or populated restore.
-- The finite semantic credit state retains the last completion credit. Refused
-- wire attempts do not exhaust it; external CPU/fault termination remains a
-- separate fail-stop assumption, never an eventual-completion guarantee.
def serve (assigned : OwnerEvaluator.Assignment p) (a scopeId capacity : Nat) : IO Unit := do
  let inputStream ← IO.getStdin
  let outputStream ← IO.getStdout
  let mut state : OwnerEndpointBudget.State p a := OwnerEndpointBudget.initial 512
  while true do
    let some bytes ← SourceWorker.readFrame inputStream | do
      if OwnerEndpointBudget.held state.owner then
        throw (IO.userError "owner EOF with active reservation")
      return ()
    unless OwnerPayload.digitCheck 128 0 bytes.data.toList && OwnerPayload.textCheck 256 bytes.data.toList do
      throw (IO.userError "owner endpoint preflight refused")
    let some cmd := OwnerPacketCodec.decodeAt (OwnerEndpointWorker.command p a) 256 bytes.data.toList |
      throw (IO.userError "invalid owner endpoint input")
    let (next,response) := OwnerEndpointBudget.transition assigned scopeId capacity state cmd
    state := next
    let encoded := OwnerPacketCodec.encode OwnerReservationWorker.replyCodec response
    if encoded.length > 65536 then throw (IO.userError "owner endpoint response exceeds frame bound")
    let header := [24,16,8,0].map (fun shift => UInt8.ofNat (encoded.length / 2^shift % 256))
    outputStream.write ⟨(header++encoded).toArray⟩
    outputStream.flush

def main (args : List String) : IO UInt32 := do
  let [realmText,pText,aText,placeText,scopeText,capacityText] := args | return 64
  let some realm := realmText.toNat? | return 64
  let some p := pText.toNat? | return 64
  let some a := aText.toNat? | return 64
  let some place := placeText.toNat? | return 64
  let some scopeId := scopeText.toNat? | return 64
  let some capacity := capacityText.toNat? | return 64
  if p > 64 || a > 64 || capacity > 64 then return 64
  if hp : place < p then
    serve ⟨realm,⟨place,hp⟩⟩ a scopeId capacity
    return 0
  else return 64
end OwnerBudgetWorker
def main := OwnerBudgetWorker.main
