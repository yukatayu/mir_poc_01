import MirroreaProofFirstOwnerCommitJournal
import GateCaptureReplay
open MirroreaProofFirst
namespace WriterJournalCaptureReplay
open OwnerCommitJournal
set_option maxHeartbeats 800000

-- Actual captured host bytes/fields, not expected model output. All images and
-- tickets decode through the real typed codecs. The hash-looking filename is
-- merely the binder's content identity; image equality uses complete values.
structure Observation where
  credits : Nat
  revision : Nat
  image : Option String
  keys : Option (List (Nat × Nat))
  lease : Option String
  entered : Bool

inductive Event where
  | beginWriter (owner ordinal : Nat) (before : Observation)
  | observe (owner : Nat) (memory : Observation)
  | endWriter (owner : Nat) (memory : Observation)

def decodeObservation (p a : Nat) (trees : String) (o : Observation) : IO (Memory p a) := do
  let image ← match o.image with
    | none => pure none
    | some file => do
      pure (some (← SourceWorker.decodeFrame (OwnerFullCodec.image p a) (← IO.FS.readBinFile (trees ++ "/" ++ file ++ ".bin"))))
  let lease ← match o.lease with
    | none => pure none
    | some file => do
      pure (some (← SourceWorker.decodeFrame OwnerFullCodec.ticket (← IO.FS.readBinFile (trees ++ "/" ++ file ++ ".bin"))))
  return ⟨⟨o.credits,image,o.revision,o.keys⟩,lease,o.entered⟩

structure OpenWriter (p a : Nat) where
  owner : Fin p
  nativeAfter : OwnerEndpointBudget.State p a
  before : Memory p a
  command : OwnerEndpoint.Command p a
  reply : Sum Nat OwnerReceipt.Envelope
  current : Memory p a
  pending : List (Store p a)
  path : Runs ⟨before,stores before command reply,false⟩ ⟨current,pending,false⟩
  nativeTarget : (stores before command reply).foldl write before = ⟨project nativeAfter,none,false⟩

def OpenWriter.observe (w : OpenWriter p a) (observed : Memory p a) : Option (OpenWriter p a) :=
  match checked : seek w.current w.pending observed with
  | none => none
  | some count =>
    some ⟨w.owner,w.nativeAfter,w.before,w.command,w.reply,observed,w.pending.drop count,
      w.path.trans (seek_sound checked).2.2,w.nativeTarget⟩

-- A successful actual writer return, unlike an arbitrary intermediate sample,
-- supplies the completion boundary. Remaining idempotent stores are represented
-- by discharge; this does not claim an exact instruction cursor from field data.
def OpenWriter.finish (w : OpenWriter p a) (observed : Memory p a) : Option Unit :=
  if same : memoryEq observed (w.pending.foldl write w.current) = true then
    let path := w.path.trans (discharge w.current w.pending)
    have target := runs_target path
    have physical : observed = ⟨project w.nativeAfter,none,false⟩ := by
      have equal := memoryEq_exact.mp same
      exact equal.trans (target.trans w.nativeTarget)
    some ()
  else none

def check (root trees : String) (realm p a scope : Nat) (capacities : Vector Nat p)
    (events : List Event) : IO Unit := do
  let mut owners : Fin p → OwnerEndpointBudget.State p a := fun _ => OwnerEndpointBudget.initial 512
  let mut ordinals : Fin p → Nat := fun _ => 0
  let mut active : Option (OpenWriter p a) := none
  let mut replies := 0
  let mut samples := 0
  for event in events do
    match event with
    | .beginWriter owner ordinal before =>
      if index : owner < p then
        let i : Fin p := ⟨owner,index⟩
        unless active.isNone && ordinal == ordinals i+1 do throw (IO.userError "writer occurrence/order")
        let command ← SourceWorker.decodeFrame (OwnerEndpointWorker.command p a)
          (← GateCaptureReplay.read root s!"owner{owner}" ordinal "input")
        let raw ← GateCaptureReplay.read root s!"owner{owner}" ordinal "output"
        let before ← decodeObservation p a trees before
        unless maySend before command do throw (IO.userError "writer command contradicts retained lease admission")
        if matching : dataEq before.data (project (owners i)) = true then
          match ran : OwnerEndpointBudget.transition ⟨realm,i⟩ scope capacities[i.val] (owners i) command with
          | (next,reply) =>
            unless raw.data.toList == OwnerPacketCodec.encode OwnerReservationWorker.replyCodec reply do
              throw (IO.userError "writer reply differs from actual complete native state")
            have binding := dataEq_exact.mp matching
            let opened : OpenWriter p a := ⟨i,next,before,command,reply,before,stores before command reply,.nil,by
              rw [stores_effects,binding,effects_project ran]⟩
            active := some opened
            owners := GateCaptureReplay.put owners i next
            ordinals := GateCaptureReplay.put ordinals i ordinal
            replies := replies+1
        else throw (IO.userError "writer before-memory differs from actual native projection")
      else throw (IO.userError "writer owner index")
    | .observe owner memory =>
      let some opened := active | throw (IO.userError "writer sample outside occurrence")
      unless opened.owner.val == owner do throw (IO.userError "writer sample borrowed occurrence")
      let memory ← decodeObservation p a trees memory
      let some advanced := opened.observe memory | throw (IO.userError "writer sample is not an ordered recipe prefix")
      active := some advanced
      samples := samples+1
    | .endWriter owner memory =>
      let some opened := active | throw (IO.userError "writer return outside occurrence")
      unless opened.owner.val == owner do throw (IO.userError "writer return borrowed occurrence")
      let memory ← decodeObservation p a trees memory
      unless (opened.finish memory).isSome do throw (IO.userError "writer returned before required stores")
      active := none
  unless active.isNone && replies>0 && samples>0 do throw (IO.userError "incomplete/nonvacuous writer journal")
  IO.println s!"WRITER_JOURNAL_CAPTURE_OK actualReplies={replies} observedStorePrefixes={samples}; full values and native projection; sampled-prefix correspondence, not exact instruction/whole-host proof"

end WriterJournalCaptureReplay
