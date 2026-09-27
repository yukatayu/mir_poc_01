import MirroreaProofFirstSharedHostJoin
import WriterJournalCaptureReplay
import SharedCaptureReplay
open MirroreaProofFirst
namespace SharedHostCaptureReplay
open OwnerCommitJournal
set_option maxHeartbeats 800000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- Each local interval keeps its EXACT known wire extension and complete native
-- owner transition. Matching an erased Data projection or a receipt hash alone
-- cannot construct this record. Privileged observed call/frame provenance is
-- still supplied by the physical binder, not manufactured by this type.
structure BoundWriter (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  earlier : SharedWireLifetime.History base
  later : SharedWireLifetime.History base
  writer : WriterJournalCaptureReplay.OpenWriter p a
  headPayment : Bool
  bytes : List UInt8
  known : SharedWireLifetime.History.knownStep earlier (.owner writer.owner writer.command headPayment) bytes = some later
  nativeReply : OwnerEndpointBudget.transition ⟨assigned.realm,writer.owner⟩ scope (capacity writer.owner)
    ((SharedWireLifetime.native earlier.current).owners writer.owner) writer.command = (writer.nativeAfter,writer.reply)
  beforeBinding : writer.before.data = project ((SharedWireLifetime.native earlier.current).owners writer.owner)
  afterBinding : writer.nativeAfter = (SharedWireLifetime.native later.current).owners writer.owner

def BoundWriter.observe (bound : BoundWriter base) (observed : Memory p a) : Option (BoundWriter base) :=
  let w := bound.writer
  match checked : seek w.current w.pending observed with
  | none => none
  | some count =>
    some {bound with writer:=⟨w.owner,w.nativeAfter,w.before,w.command,w.reply,observed,w.pending.drop count,
      w.path.trans (seek_sound checked).2.2,w.nativeTarget⟩}

structure ClosedWriter (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  bound : BoundWriter base
  observed : Memory p a
  path : Runs ⟨bound.writer.before,stores bound.writer.before bound.writer.command bound.writer.reply,false⟩
    ⟨observed,[],false⟩
  nativeData : observed.data = project ((SharedWireLifetime.native bound.later.current).owners bound.writer.owner)
  leaseReleased : observed.lease = none
  enteredCleared : observed.entered = false

def BoundWriter.finish (bound : BoundWriter base) (observed : Memory p a) : Option (ClosedWriter base) :=
  let w := bound.writer
  if same : memoryEq observed (w.pending.foldl write w.current) = true then
    let path := w.path.trans (discharge w.current w.pending)
    have equal := memoryEq_exact.mp same
    have physical : observed = ⟨project w.nativeAfter,none,false⟩ :=
      equal.trans ((runs_target path).trans w.nativeTarget)
    some ⟨bound,observed,by simpa only [←equal] using path,by rw [physical,bound.afterBinding],
      by rw [physical],by rw [physical]⟩
  else none

end SharedHostCaptureReplay
