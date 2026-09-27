import MirroreaProofFirstSourceFundingFrame

open MirroreaProofFirst
namespace SourceFundingQueryWorker

-- Privileged reference pipe. Read-only cost/head queries preserve state.
-- Every execute branch carries the complete bounded owner-credit vector.
-- The sole reader retains the exact request constructor through reply receipt.
-- All query data is private; cost/head metadata is not a public observation.
-- This input is not authenticated or a proof of actual physical balances.
-- Source adoption reserves its certified suffix plus first owner initialization.
-- No restore/populated import/alternate unmetered command is accepted.
-- Quota counts successful checked source transitions
-- and includes the retained completion. Refused frames do not replenish or
-- consume it. Request traffic/CPU/physical owner capacity are separate limits.
-- Read to EOF even at quota zero so the old 512th-command exception cannot
-- turn an already emitted successful completion into a failed normal close.
def serve (assigned : SourceInput.Assignment p a) (scopeId capacity : Nat) : IO Unit := do
  let inputStream ← IO.getStdin
  let outputStream ← IO.getStdout
  let some initial ← SourceWorker.readFrame inputStream | throw (IO.userError "missing bootstrap")
  let seed ← SourceWorker.decodeFrame (SourceInput.bootstrap a) initial
  let mut state := PublicationCapacityDriver.initial (p:=p) (a:=a) capacity
  repeat
    let some bytes ← SourceWorker.readFrame inputStream | return ()
    let command ← SourceWorker.decodeFrame (SourceFundingQuery.checkedInput p a) bytes
    let (next,response) := SourceFundingQuery.checkedExchange assigned scopeId seed state command
    let encoded := OwnerPacketCodec.encode (SourceFundingQuery.checkedReply command) response
    -- Backstop for compiler/runtime faults. The reachable typed model proves
    -- this bound for every accepted/refused response, including initial none.
    if encoded.length > 65536 then throw (IO.userError "capacity invariant response bound violated")
    state := next
    let header := [24,16,8,0].map (fun shift => UInt8.ofNat (encoded.length / 2^shift % 256))
    outputStream.write ⟨(header++encoded).toArray⟩
    outputStream.flush

def main (args : List String) : IO UInt32 := do
  let [realmText,pText,aText,callerText,memberText,principalText,scopeText,capacityText] := args | return 64
  let some realm := realmText.toNat? | return 64
  let some p := pText.toNat? | return 64
  let some a := aText.toNat? | return 64
  let some caller := callerText.toNat? | return 64
  let some member := memberText.toNat? | return 64
  let some principal := principalText.toNat? | return 64
  let some scopeId := scopeText.toNat? | return 64
  let some capacity := capacityText.toNat? | return 64
  if p > 64 || a > 64 || capacity > 512 then return 64
  if hc : caller < p then
    if hm : member < a then
      serve ⟨realm,⟨⟨caller,hc⟩,⟨member,hm⟩,principal⟩⟩ scopeId capacity
      return 0
    else return 64
  else return 64
end SourceFundingQueryWorker
def main := SourceFundingQueryWorker.main
