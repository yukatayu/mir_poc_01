import CohortHostExecution
open MirroreaProofFirst
namespace CohortHostCaptureValues

-- All fields come from the real privileged host capture. String keys name
-- bound raw-value files only; neither counts nor digest equality is a value.
structure Observation where
  bootstrapped : Bool
  snapshot : Option String
  initialized : List Nat
  freezes : List (Nat × Nat)
  installs : List (Nat × Nat)
  produced : List String
  payment : Option (String × Nat)
  gate : Bool
  retired : Bool
  sourceOrdinal : Nat

def endpoint (p index : Nat) : IO (Fin p) :=
  if bounded : index < p then pure ⟨index,bounded⟩ else throw (IO.userError "cohort observation endpoint range")

def decode (p a : Nat) (trees : String) (observed : Observation) : IO (CohortCommitJournal.Memory p a) := do
  let snapshot ← match observed.snapshot with
    | none => pure none
    | some file => SourceEntryCapture.readSnapshot trees file p a
  let initialized ← observed.initialized.mapM (endpoint p)
  let freezes ← observed.freezes.mapM fun (owner,revision) => do pure (← endpoint p owner,revision)
  let installs ← observed.installs.mapM fun (owner,revision) => do pure (← endpoint p owner,revision)
  let produced ← observed.produced.mapM fun file => do
    SourceWorker.decodeFrame OwnerReceipt.codec (← IO.FS.readBinFile (trees++"/"++file++".bin"))
  let payment ← match observed.payment with
    | none => pure none
    | some (file,ordinal) => do
      let input ← SourceWorker.decodeFrame (PublicationInput.input p a)
        (← IO.FS.readBinFile (trees++"/"++file++".bin"))
      match input with
      | .step (.freeze target revision) => pure (some ⟨true,target,revision,ordinal⟩)
      | .step (.install target revision) => pure (some ⟨false,target,revision,ordinal⟩)
      | _ => throw (IO.userError "cohort observed payment is not freeze/install notification")
  pure ⟨observed.bootstrapped,snapshot,initialized,freezes,installs,produced,payment⟩

end CohortHostCaptureValues
