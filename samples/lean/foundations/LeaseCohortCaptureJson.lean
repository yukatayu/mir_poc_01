import LeaseCohortCaptureReplay
import Lean.Data.Json
open Lean MirroreaProofFirst
namespace LeaseCohortCaptureJson

-- Private artifact decoder, not source syntax or a public wire contract.
-- It loads actual normalized capture data at runtime instead of elaborating a
-- multi-megabyte Lean expression. Every value still enters the same checker.
def field [FromJson α] (value : Json) (name : String) : Except String α := value.getObjValAs? α name

def writer (value : Json) : Except String WriterJournalCaptureReplay.Observation := do
  pure ⟨← field value "credits",← field value "revision",← field value "image",
    ← field value "keys",← field value "lease",← field value "entered"⟩

def observation (value : Json) : Except String CohortHostCaptureValues.Observation := do
  pure ⟨← field value "bootstrapped",← field value "snapshot",← field value "initialized",
    ← field value "freezes",← field value "installs",← field value "produced",← field value "payment",
    ← field value "gate",← field value "retired",← field value "sourceOrdinal"⟩

def event (value : Json) : Except String LeaseCohortCaptureReplay.Event := do
  let tag : String ← field value "tag"
  match tag with
  | "begin" => pure .begin
  | "finish" => pure .finish
  | "gateEnter" => pure (.gateEnter (← field value "callId"))
  | "gateRelease" => pure .gateRelease
  | "bootstrapReturned" => pure .bootstrapReturned
  | "cohortCommit" => pure .cohortCommit
  | "shutdown" => pure .shutdown
  | "rejectedReentry" => pure .rejectedReentry
  | "cohortObserve" => pure (.cohortObserve (← field value "row") (← observation (← value.getObjVal? "observation")))
  | "source" => pure (.source (← field value "ordinal"))
  | "sourceClaim" => pure (.sourceClaim (← field value "ordinal") (← field value "endpoint")
      (← writer (← value.getObjVal? "before")) (← writer (← value.getObjVal? "claimed"))
      (← field value "snapshot"))
  | "sourceStore" =>
    let kind : String ← field value "kind"
    let store : SourceEntryPrefix.StoreKind ← match kind with
      | "notify" => pure .notify
      | "cancel" => pure .cancel
      | "snapshot" => pure .snapshot
      | _ => throw "unknown source store kind"
    pure (.sourceStore store (← field value "endpoint") (← writer (← value.getObjVal? "memory"))
      (← field value "snapshot"))
  | "owner" => pure (.owner (← field value "endpoint") (← field value "ordinal")
      (← writer (← value.getObjVal? "memory")))
  | "writerStatement" => pure (.writerStatement (← field value "endpoint") (← field value "ordinal") (← field value "occurrence") (← field value "token")
      (← field value "kind") (← writer (← value.getObjVal? "memory")))
  | "writerObserve" => pure (.writerObserve (← field value "endpoint") (← writer (← value.getObjVal? "memory")))
  | "writerReturn" => pure (.writerReturn (← field value "endpoint") (← writer (← value.getObjVal? "memory")))
  | _ => throw "event is outside the selected normal whole-cohort capture profile"

def run (root trees eventsFile : String) (realm p a caller member principal scope capacity : Nat)
    (ownerCapacity : Vector Nat p) (expectedObservations : Nat) (requireProgramComplete : Bool := true) : IO Unit := do
  let json ← IO.ofExcept (Json.parse (← IO.FS.readFile eventsFile))
  let values ← IO.ofExcept json.getArr?
  let events ← IO.ofExcept (values.toList.mapM event)
  LeaseCohortCaptureReplay.cohort root trees realm p a caller member principal scope capacity ownerCapacity expectedObservations events true requireProgramComplete

end LeaseCohortCaptureJson
