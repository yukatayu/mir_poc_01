import OwnerStatementMetadataCursor
import OwnerMetadataSessionInvariant
namespace MirroreaProofFirst.OwnerStatementAcceptance
open MixedOwnerContinuation MixedOwnerProgram

-- Setup entries are not completed source assignments. This projection keeps
-- only original assignment payloads at their generated WRITE positions.
def entryAssignment : Entry → Option MixedNamedOwnerSource.Assignment
 | .write _ _ _ source => some source
 | _ => none
def itemAssignment : Item → Option MixedNamedOwnerSource.Assignment
 | .assignment source => some source
 | _ => none
def writes (entries : List Entry) := entries.filterMap entryAssignment
def assignments (items : List Item) := items.filterMap itemAssignment

theorem lower_assignments {control : Nat} (typed : Lowers p ctx labels control ordinal used env items entries final) :
 writes entries = assignments items := by
 induction typed with
 | nil => rfl
 | ordinary _ _ ih => simpa [writes,assignments,entryAssignment,itemAssignment] using ih
 | assignment _ _ _ _ ih => simpa [writes,assignments,entryAssignment,itemAssignment,construction] using ih

theorem write_prefix {control : Nat} (partition : OwnerStatementCursor.Partition entries cursor)
 (typed : Lowers p ctx labels control ordinal used env items entries final) :
 ∃ suffix, assignments items = writes cursor.completed ++ suffix := by
 refine ⟨writes (cursor.stopped.toList ++ cursor.remaining),?_⟩
 rw [←lower_assignments typed,←partition]
 simp [writes,List.filterMap_append,List.append_assoc]

-- The actual acknowledgement checks full saved binding before clearing wait.
-- This entails request/incarnation/origin equality, not just equal operation.
theorem acknowledged_saved_equal (accepted : MixedOwnerSourceIssue.acknowledge state received evidence = some next)
 (waiting : MixedOwnerSourceIssue.ownerWaiting state.waiting = some held) : received = held.saved := by
 obtain ⟨w,sw,finished,_⟩ := MixedOwnerSourceIssue.acknowledge_parts accepted
 have same : w = held := by simpa [MixedOwnerSourceIssue.ownerWaiting,sw] using waiting
 subst w
 have store := (MixedReferenceExecution.finishOwner_parts _ finished).1
 obtain ⟨_,core,_⟩ := MixedReferenceStore.finishOwner_parts _ store
 obtain ⟨_,_,pending,delivered,saved,receivedSaved,ok,_⟩ := MixedRequestCore.finishOwner_parts core
 exact receivedSaved.symm.trans ((congrArg OwnerSavedPending.save ok.1).trans saved)

theorem owner_complete_parts (waiting : MixedOwnerSourceIssue.ownerWaiting s.state.source.waiting = some held)
 (ready : (complete s).status = .ready) :
 ∃ next, MixedOwnerSourceTrace.receive s.state = some next ∧ (complete s).state = next := by
 unfold complete at ready ⊢
 simp only [waiting] at ready ⊢
 split at ready
 · cases ready
 · rename_i nonempty
   rw [if_neg nonempty]
   cases run : MixedOwnerSourceTrace.receive s.state with
   | none => simp [run] at ready
   | some next => exact ⟨next,rfl,rfl⟩

-- A successful owner completion consumes a reply for an ACTUAL prior owner
-- history record with the exact current saved request. Provenance is supplied
-- by the existing rooted owner/metadata invariant, not assumed as completion.
theorem owner_complete_committed
 (provenance : MixedOwnerSourceTrace.Provenance s.state)
 (waiting : MixedOwnerSourceIssue.ownerWaiting s.state.source.waiting = some held)
 (ready : (complete s).status = .ready) :
 (∃ written ∈ s.state.owner.history, written.pending = held.saved) ∧
 (complete s).state.owner = s.state.owner := by
 obtain ⟨next,received,nextState⟩ := owner_complete_parts waiting ready
 obtain ⟨reply,rest,source,box,ack,eq⟩ := MixedOwnerSourceTrace.receive_parts received
 obtain ⟨written,member,replyEq⟩ := provenance reply (by simp [box])
 have saved := acknowledged_saved_equal ack waiting
 constructor
 · exact ⟨written,member,by simpa [replyEq,MixedOwnerSourceTrace.reply] using saved⟩
 · rw [nextState,eq]

-- Applies to both previously launched sessions and the current metadata
-- attachment. Raw populated cursor/image imports are outside these relations.
theorem rooted_owner_complete_committed
 (path : Rooted realm view policy store s)
 (waiting : MixedOwnerSourceIssue.ownerWaiting s.state.source.waiting = some held)
 (ready : (complete s).status = .ready) :
 ∃ written ∈ s.state.owner.history, written.pending = held.saved :=
 (owner_complete_committed (rooted_joint path).2.2.2 waiting ready).1

theorem metadata_owner_complete_committed
 (valid : OwnerMetadataSessionInvariant.Valid initial)
 (path : OwnerMetadataSession.Rooted initial s)
 (waiting : MixedOwnerSourceIssue.ownerWaiting s.session.state.source.waiting = some held)
 (ready : (complete s.session).status = .ready) :
 ∃ written ∈ s.session.state.owner.history, written.pending = held.saved :=
 (owner_complete_committed (OwnerMetadataSessionInvariant.rooted_joint valid path).1.2.2.2 waiting ready).1

#print axioms lower_assignments
#print axioms write_prefix
#print axioms acknowledged_saved_equal
#print axioms owner_complete_parts
#print axioms owner_complete_committed
#print axioms rooted_owner_complete_committed
#print axioms metadata_owner_complete_committed
end MirroreaProofFirst.OwnerStatementAcceptance
