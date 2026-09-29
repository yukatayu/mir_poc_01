import OwnerStatementMetadataHistory
import MixedOwnerContinuationControls
import OwnerMetadataSessionControls
namespace MirroreaProofFirst.OwnerStatementExecutionControls
open MixedOwnerContinuation MixedOwnerProgram
-- Finite model execution of the current checked-IR/source driver. These are
-- controls for the general proofs, not Rust/network or authentic-schema proof.
def first : MixedNamedOwnerSource.Assignment := ⟨⟨"sequence.mir",10⟩,"s",.integer 34⟩
def middle : MixedNamedOwnerSource.Assignment := ⟨⟨"sequence.mir",20⟩,"t",.integer 35⟩
def last : MixedNamedOwnerSource.Assignment := ⟨⟨"sequence.mir",30⟩,"s",.add (.state "s") (.integer 1)⟩
def fields : MixedNamedOwnerSource.Fields := ⟨[("s",0),("t",1)],[(0,(0,0)),(1,(1,0))]⟩
def program : Program 3 := ⟨2,fields,[],0,[.assignment first,.assignment middle,.assignment last]⟩
def metadata (k : Nat) := fields.metadata.lookup k
def store (k : Nat) : Option Int := if k < 2 then some 10 else none
def begun := launch 91 MixedManagementControls.view (fun _ => ManagementEntry.Controls.policy) store program
def waiting1 := begun.map fun s => drive 3 s 0 7
def served1 := (waiting1.bind transfer).map fun s => service s (FallibleFlow.signed 63) metadata
def ack1 := served1.map fun s => tick s 0 7
def waiting2 := ack1.map fun s => drive 3 s 0 7
def served2 := (waiting2.bind transfer).map fun s => service s (FallibleFlow.signed 63) metadata
def ack2 := served2.map fun s => tick s 0 7
def waiting3 := ack2.map fun s => drive 3 s 0 7
def served3 := (waiting3.bind transfer).map fun s => service s (FallibleFlow.signed 63) metadata
def ack3 := served3.map fun s => tick s 0 7
#guard begun.isSome
#guard (waiting1.map fun s => s.status) = some .waiting
#guard (ack1.map fun s => (s.status,s.state.owner.store 0,s.state.owner.history.length)) = some (.ready,some 34,1)
#guard (ack2.map fun s => (s.status,s.state.owner.store 1,s.state.owner.history.length)) = some (.ready,some 35,2)
#guard (ack3.map fun s => (s.status,s.state.owner.store 0,s.state.owner.history.length)) = some (.ready,some 35,3)
#guard (ack3.map fun s => s.state.owner.history.map fun r => (r.pending.origin.ordinal,r.pending.origin.byteOffset)) = some [(0,10),(1,20),(2,30)]
#guard (ack1.map drained) = some true
#guard (ack1.bind fun s => continueWith s program).isNone
#guard (ack1.bind fun s => replaceResidual s program).isNone
-- Service-time failure at T retains S's accepted prefix; later S never issues.
def refused2 := (waiting2.bind transfer).map fun s => service s (FallibleFlow.signed 63) (fun k => if k=1 then none else metadata k)
def stuck2 := refused2.map fun s => drive 8 s 0 7
#guard (stuck2.map fun s => (s.state.owner.store 0,s.state.owner.store 1,s.state.owner.history.length,s.status)) = some (some 34,some 10,1,.waiting)
#guard (stuck2.map fun s => s.state.source.nextRequest) = waiting2.map fun s => s.state.source.nextRequest
#guard (stuck2.map fun s => (OwnerStatementAcceptance.writes s.cursor.completed).length) = some 1
-- Actual commit without accepted acknowledgment is not a completed source write.
def changed := served1.bind fun s => authorityHead s
 {s.state.source.machine.store.core.system.view with generation := s.state.source.machine.store.core.system.view.generation+1}
def denied := changed.map fun s => tick s 0 7
#guard (denied.map fun s => (s.status,s.state.owner.history.length,(OwnerStatementAcceptance.writes s.cursor.completed).length)) = some (.failed .rejected,1,0)
#guard (served1.map fun s => (OwnerStatementAcceptance.writes s.cursor.completed).length) = some 0
#guard (ack1.map fun s => (OwnerStatementAcceptance.writes s.cursor.completed).length) = some 1
-- Current metadata-attached executor provides a real positive control too.
#guard (OwnerMetadataSessionControls.served.map fun s => s.session.state.owner.history.length) = some 1
#guard (OwnerMetadataSessionControls.acknowledged.map fun s => (OwnerStatementAcceptance.writes s.session.cursor.completed).length) = some 1
-- Deliberate lower inbox substitution: an earlier request's actual reply may
-- not advance another occurrence. This raw injection is outside Rooted.
def substituted := do
 let current ← waiting2
 let old ← served1
 return {current with state := {current.state with inbox := old.state.inbox}}
def substitutionResult := substituted.map fun s => tick s 0 7
#guard (substitutionResult.map fun s => (s.status,s.state.owner.history.length,(OwnerStatementAcceptance.writes s.cursor.completed).length)) = some (.failed .rejected,1,1)
-- Exact list partition can hold for a forged acknowledged prefix, without any
-- real commit. Such raw records are excluded by rooted-history admission.
def forged := begun.map fun s => {s with cursor := ⟨s.entries.take 3,s.entries.drop 3,none⟩}
#guard (forged.map fun s => decide (s.cursor.completed ++ s.cursor.remaining = s.entries)) = some true
#guard (forged.map fun s => ((OwnerStatementAcceptance.writes s.cursor.completed).length,s.state.owner.history.length)) = some (1,0)
-- Same program, new invocation: activation and request counter are retained,
-- while the new source accepted prefix starts empty.
def continued := ack3.bind fun s => continueWith s program
def nextWaiting := continued.map fun s => drive 3 s 0 7
#guard (nextWaiting.bind fun s => (MixedOwnerSourceIssue.ownerWaiting s.state.source.waiting).map fun w => (w.saved.origin.activation,w.saved.origin.ordinal)) = some (1,0)
#guard (nextWaiting.map fun s => (s.state.owner.history.length,(OwnerStatementAcceptance.writes s.cursor.completed).length)) = some (3,0)
#guard (nextWaiting.bind fun current => waiting1.map fun old => decide (old.state.source.nextRequest < current.state.source.nextRequest)) = some true
end MirroreaProofFirst.OwnerStatementExecutionControls
