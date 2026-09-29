import OwnerStatementCompletion
import OwnerStatementHistoryOrigin
import OwnerStatementExecutionControls
namespace MirroreaProofFirst.OwnerStatementBoundaryControls
open MixedOwnerContinuation MixedOwnerProgram
open OwnerStatementExecutionControls (first middle last fields metadata store program begun waiting1 served1 ack1)

-- Interleaved ordinary items retain sparse ORIGINAL positions, and the last
-- assignment does not complete trailing ordinary work.
def ordinary : Item := .ordinary ⟨⟨"sequence.mir",5⟩,.plain (.localValue "z" true (.integer 8))⟩
def update : Item := .ordinary ⟨⟨"sequence.mir",15⟩,.plain (.assign "z" (.integer 9))⟩
def sparse : Program 3 := {program with items := [ordinary,.assignment first,update,.assignment middle,update]}
#guard (OwnerStatementPosition.indexed 0 sparse.items).map Prod.fst = [1,3]
#guard ((MixedOwnerProgram.compile [] sparse).map fun pair =>
 (OwnerStatementPosition.writes pair.1).map Prod.fst) = some [1,3]
def sparseBegun := launch 91 MixedManagementControls.view (fun _ => ManagementEntry.Controls.policy) store sparse
def sparseFirst := ((sparseBegun.map fun s => drive 4 s 0 7).bind transfer).map fun s => tick (service s (FallibleFlow.signed 63) metadata) 0 7
def sparseLast := ((sparseFirst.map fun s => drive 4 s 0 7).bind transfer).map fun s => tick (service s (FallibleFlow.signed 63) metadata) 0 7
#guard (sparseLast.map fun s => ((OwnerStatementPosition.writes s.cursor.completed).map Prod.fst,s.cursor.remaining.length,s.status)) = some ([1,3],1,.ready)
#guard (sparseLast.bind fun s => continueWith s sparse).isNone
#guard (sparseLast.map fun s => (tick s 0 7).cursor.remaining.length) = some 0

-- Equal sites and payloads are legal in this IR; occurrence order distinguishes
-- both actual service attempts. Site equality never becomes request identity.
def equalProgram : Program 3 := {program with items := [.assignment first,.assignment first]}
def equalBegun := launch 91 MixedManagementControls.view (fun _ => ManagementEntry.Controls.policy) store equalProgram
def equalFirst := ((equalBegun.map fun s => drive 3 s 0 7).bind transfer).map fun s => tick (service s (FallibleFlow.signed 63) metadata) 0 7
def equalLast := ((equalFirst.map fun s => drive 3 s 0 7).bind transfer).map fun s => tick (service s (FallibleFlow.signed 63) metadata) 0 7
#guard (equalLast.map fun s => s.state.owner.history.map fun r => (r.pending.origin.ordinal,r.pending.origin.byteOffset)) = some [(0,10),(1,10)]
#guard (equalLast.map fun s => (s.state.owner.attempts.length,s.status)) = some (2,.ready)

-- Admitted lower arithmetic refusal consumes the queued attempt; this differs
-- from the metadata boundary refusal that retains its queue without an attempt.
def overflowProgram : Program 3 := {program with items := [.assignment first,
 .assignment {middle with rhs := .add (.integer 31) (.integer 4)},.assignment last]}
def overflowBegun := launch 91 MixedManagementControls.view (fun _ => ManagementEntry.Controls.policy) store overflowProgram
def overflowFirst := ((overflowBegun.map fun s => drive 3 s 0 7).bind transfer).map fun s => tick (service s (FallibleFlow.signed 63) metadata) 0 7
def overflow := ((overflowFirst.map fun s => drive 3 s 0 7).bind transfer).map fun s => service s (FallibleFlow.signed 5) metadata
#guard (overflow.map fun s => (s.state.owner.history.length,s.state.owner.attempts.length,s.state.owner.queued.isNone)) = some (1,2,true)
#guard (overflow.map fun s => (s.state.owner.attempts.getLast?).map fun row => match row.2 with | .refused _ => true | _ => false) = some (some true)
#guard (overflow.map fun s => ((OwnerStatementPosition.writes (drive 6 s 0 7).cursor.completed).length,(drive 6 s 0 7).status)) = some (1,.waiting)

-- Metadata refusal is before publishing tentative source events/request IDs;
-- comparing the WHOLE source and owner history retains every field.
#guard (OwnerMetadataSessionControls.refused.map fun s => s.session.state.source.nextRequest) =
 OwnerMetadataSessionControls.attached.map fun s => s.session.state.source.nextRequest
#guard (OwnerMetadataSessionControls.refused.map fun s => s.session.cursor) =
 OwnerMetadataSessionControls.attached.map fun s => s.session.cursor
#guard (OwnerMetadataSessionControls.refused.map fun s => s.session.state.source.waiting) =
 OwnerMetadataSessionControls.attached.map fun s => s.session.state.source.waiting
#guard (OwnerMetadataSessionControls.refused.map fun s => s.session.state.owner.attempts) =
 OwnerMetadataSessionControls.attached.map fun s => s.session.state.owner.attempts

-- Expose the actual current policy outcome at service/change/ack; this control
-- does not choose a new policy or infer metadata currentness from history.
def changedAfterService := OwnerMetadataSessionControls.served.bind fun s =>
 OwnerMetadataSession.changeMetadata s 0 0 7 ⟨"metadata-session.mir",8⟩ (.retire OwnerMetadataManagementControls.key)
def acknowledgedAfterChange := changedAfterService.map fun s => OwnerMetadataSession.tick s 0 7
#guard (acknowledgedAfterChange.map fun s => (s.session.status,s.session.state.owner.history.length,
 (OwnerStatementPosition.writes s.session.cursor.completed).length)) = some (.ready,1,1)

theorem driven_rooted (path : Rooted realm view policy store s) : Rooted realm view policy store (drive fuel s member principal) := by
 induction fuel generalizing s with
 | zero => exact path
 | succ fuel ih => exact ih (.tick path)

theorem first_rooted (run : ack1 = some s) :
 Rooted 91 MixedManagementControls.view (fun _ => ManagementEntry.Controls.policy) store s := by
 unfold OwnerStatementExecutionControls.ack1 OwnerStatementExecutionControls.served1
   OwnerStatementExecutionControls.waiting1 at run
 cases start : begun with
 | none => simp [start] at run
 | some initial =>
   simp only [start,Option.map_some,Option.bind_some] at run
   cases moved : transfer (drive 3 initial 0 7) with
   | none => simp [moved] at run
   | some queued =>
     simp only [moved,Option.map_some,Option.some.injEq] at run
     subst s
     exact .tick (.service (.transfer (driven_rooted (.launch start)) moved))

def actual := ack1.get (by decide)
theorem actual_rooted : Rooted 91 MixedManagementControls.view
 (fun _ => ManagementEntry.Controls.policy) store actual :=
 first_rooted (Option.some_get _).symm
def registry : OwnerMetadataRegistry.Registry :=
 {instanceId := 91, ownerKey := 0, ownerIdentity := ⟨.locus,0,0⟩,
  moduleKey := 0, moduleIdentity := ⟨.module,0,0⟩, code := 0, contract := 0,
  ownerName := "S", schema := [], serial := 0, slots := [], used := []}
def attached : OwnerMetadataSession.State 3 1 := ⟨actual,registry,[],none⟩
-- The finite drained/nonempty premises are evaluated below. They are not
-- expanded into a giant kernel decide proof of the entire execution fixture.
theorem actual_initial (empty : drained actual = true) : OwnerStatementAdmission.Initial 91 MixedManagementControls.view
 (fun _ => ManagementEntry.Controls.policy) store attached := by
 refine ⟨actual,registry,[],actual_rooted,OwnerMetadataRegistry.empty_consistent rfl,?_⟩
 simp [OwnerMetadataSession.attach,empty,attached]

-- The initializer is inhabited after a real commit; it is not all-rejecting.
theorem actual_facts (empty : drained actual = true) : OwnerStatementAdmission.Facts attached :=
 OwnerStatementAdmission.initial_facts (actual_initial empty)
def forged := OwnerStatementHistoryOrigin.eraseAttempts attached
theorem forged_all_predicates (empty : drained actual = true) : OwnerStatementAdmission.Facts forged :=
 OwnerStatementHistoryOrigin.erased_weak_facts (actual_facts empty)
theorem forged_excluded (nonempty : attached.session.state.owner.history ≠ []) :
 ¬OwnerStatementAdmission.Initial realm view policy store forged := by
 exact OwnerStatementHistoryOrigin.erased_not_initial (realm:=realm) (view:=view)
   (policy:=policy) (store:=store) (s:=attached) nonempty
#guard drained actual = true
#guard actual.state.owner.history ≠ []
#guard forged.session.state.owner.attempts = []
#guard forged.session.state.owner.history.length = 1
#guard OwnerStatementPosition.writes forged.session.cursor.completed = [(0,first)]

#print axioms driven_rooted
#print axioms first_rooted
#print axioms actual_rooted
#print axioms actual_initial
#print axioms actual_facts
#print axioms forged_all_predicates
#print axioms forged_excluded

#print axioms OwnerStatementHistoryOrigin.erased_weak_facts
#print axioms OwnerStatementHistoryOrigin.erased_not_initial
end MirroreaProofFirst.OwnerStatementBoundaryControls
