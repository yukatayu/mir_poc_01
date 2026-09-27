import Lean.Data.Json
namespace CohortWireEvidence
open Lean

-- Independent actual-custody input, read from the privileged raw capture,
-- not inferred from the normalized event's proposed interruption label.
-- Capture truthfulness and exclusive private process custody remain TCB.
structure Evidence where
  source : Bool
  endpoint : Nat
  ordinal : Nat
  applied : Bool
  payload : List UInt8
  raw : Option (List UInt8)
  deriving DecidableEq

def check (actual proposed : Evidence) : Bool := decide (actual = proposed)

theorem check_exact : check actual proposed = true ↔ actual = proposed := by
  simp [check]

theorem lost_raw_rejected (actual : Evidence) (bytes : List UInt8)
    (retained : actual.raw = some bytes) :
    check actual {actual with raw:=none} = false := by
  apply Bool.eq_false_iff.mpr
  intro accepted
  have equal := congrArg Evidence.raw (check_exact.mp accepted)
  simp [retained] at equal

theorem applied_rollback_rejected (actual : Evidence) (applied : actual.applied = true) :
    check actual {actual with applied:=false} = false := by
  apply Bool.eq_false_iff.mpr
  intro accepted
  have equal := congrArg Evidence.applied (check_exact.mp accepted)
  simp [applied] at equal

def field [FromJson α] (value : Json) (name : String) : Except String α := value.getObjValAs? α name

def digit (c : Char) : Except String Nat :=
  if '0' ≤ c && c ≤ '9' then pure (c.toNat-'0'.toNat)
  else if 'a' ≤ c && c ≤ 'f' then pure (c.toNat-'a'.toNat+10)
  else throw "noncanonical capture hex digit"

def hex : List Char → Except String (List UInt8)
  | [] => pure []
  | a::b::rest => do
    let high ← digit a
    let low ← digit b
    pure (UInt8.ofNat (16*high+low)::(← hex rest))
  | [_] => throw "odd capture hex length"

def parse (receipt : Json) : Except String Evidence := do
  let fault ← receipt.getObjVal? "wire_fault"
  let expected ← receipt.getObjVal? "wire_custody_expected"
  let slot ← expected.getObjVal? "outstanding"
  let profile : String ← field fault "target_profile"
  let endpointName : String ← field fault "endpoint"
  let actualEndpoint : String ← field slot "endpoint"
  unless actualEndpoint == endpointName do throw "raw custody endpoint differs"
  let source ← match profile with
    | "ENTRY" => pure true
    | "OWNER" => pure false
    | _ => throw "unselected raw-capture endpoint profile"
  let endpoint ← if source then do
      unless endpointName == "source" do throw "raw source endpoint"
      field fault "entry_owner"
    else do
      match endpointName.splitOn "owner" with
      | ["",number] =>
        let some n := number.toNat? | throw "raw owner endpoint index"
        pure n
      | _ => throw "raw owner endpoint"
  let ordinal : Nat ← field fault "attempted_ordinal"
  let slotOrdinal : Nat ← field slot "ordinal"
  let expectsReply : Bool ← field slot "expect_reply"
  unless slotOrdinal == ordinal && expectsReply do throw "raw attempted occurrence differs"
  let payloadHex : String ← field slot "payload_hex"
  let rawHex : Option String ← field slot "reply_hex"
  let payload ← hex payloadHex.toList
  let raw ← rawHex.mapM fun value => hex value.toList
  let retained : Bool ← field fault "raw_retained"
  unless retained == raw.isSome do throw "raw custody body presence differs"
  let kind : String ← field fault "cut"
  let applied ← match kind with
    | "BEFORE_WRITE" =>
      unless !retained do throw "body before physical delivery"
      pure false
    | "BODY_LOST" =>
      unless !retained do throw "lost body still retained"
      pure true
    | "RAW_CAPTURE" =>
      unless retained do throw "captured body lost"
      pure true
    | _ => throw "unselected raw-capture interruption cut"
  pure ⟨source,endpoint,ordinal,applied,payload,raw⟩

def read (path : String) : IO Evidence := do
  let value ← IO.ofExcept (Json.parse (← IO.FS.readFile path))
  IO.ofExcept (parse value)

#print axioms check_exact
#print axioms lost_raw_rejected
#print axioms applied_rollback_rejected
end CohortWireEvidence
