import OwnerStatementCompletion
import OwnerStatementHistoryOrigin
namespace MirroreaProofFirst.OwnerStatementRetainedHistory
open MixedOwnerAttemptQueue

-- Exact committed-result projection alone does not bind an attempt's request
-- to the record it purports to have committed. Derive that binding separately.
def RowsBound (s : State) : Prop :=
 ∀ saved written, (saved,Result.committed written) ∈ s.attempts → written.pending = saved
def Valid (s : State) : Prop := OwnerStatementHistoryOrigin.Exact s ∧ RowsBound s

theorem enqueue_bound (valid : RowsBound s) (accepted : enqueue s saved = some next) : RowsBound next := by
 obtain ⟨_,_,rfl⟩ := enqueue_exact.mp accepted
 exact valid

theorem advance_bound (valid : RowsBound s) : RowsBound (advance ops catalog view current s).1 := by
 cases run : (advance ops catalog view current s).2 with
 | none => rw [not_attempted_retains run]; exact valid
 | some answer =>
   obtain ⟨entry,_,_,queued,_,_,_,_,attempts,_⟩ := advance_parts run
   intro saved written member
   rw [attempts] at member
   rcases List.mem_append.mp member with old | fresh
   · exact valid _ _ old
   · have pair : (saved,Result.committed written) = (entry,answer) := List.mem_singleton.mp fresh
     have savedEq := congrArg Prod.fst pair
     have resultEq : answer = Result.committed written := (congrArg Prod.snd pair).symm
     obtain ⟨actual,_,_,atQueue,_,_,_,same,_⟩ := actual_commit (run.trans (congrArg some resultEq))
     have entryEq : actual = entry := Option.some.inj (atQueue.symm.trans queued)
     exact same.trans (entryEq.trans savedEq.symm)

theorem trace_step_bound (valid : RowsBound s.owner) (step : MixedOwnerSourceTrace.Step s next) : RowsBound next.owner := by
 cases step with
 | issue accepted => obtain ⟨_,_,_,rfl⟩ := MixedOwnerSourceTrace.issue_parts accepted; exact valid
 | transfer accepted =>
   obtain ⟨_,_,_,_,enqueued,rfl⟩ := MixedOwnerSourceTrace.transfer_parts accepted
   exact enqueue_bound valid enqueued
 | service =>
   unfold MixedOwnerSourceTrace.service
   split
   · exact advance_bound valid
   · exact valid
 | receive accepted => obtain ⟨_,_,_,_,_,rfl⟩ := MixedOwnerSourceTrace.receive_parts accepted; exact valid
 | declaration => exact valid
 | pureAdvance => exact valid
 | pureComplete => exact valid
 | pureCancel => exact valid
 | control accepted => obtain ⟨_,rfl⟩ := MixedOwnerSourceTrace.control_parts accepted; exact valid
 | authority accepted => obtain ⟨_,rfl⟩ := MixedOwnerSourceTrace.authority_parts accepted; exact valid

theorem trace_step_valid (valid : Valid s.owner) (step : MixedOwnerSourceTrace.Step s next) : Valid next.owner :=
 ⟨OwnerStatementHistoryOrigin.trace_step valid.1 step,trace_step_bound valid.2 step⟩

theorem trace_rooted (path : MixedOwnerSourceTrace.Rooted source store s) : Valid s.owner := by
 induction path with
 | initial => exact ⟨rfl,by intro _ _ impossible; cases impossible⟩
 | step _ transition ih => exact trace_step_valid ih transition

theorem initial_valid (entered : OwnerStatementAdmission.Initial realm view policy store s) :
 Valid s.session.state.owner := by
 obtain ⟨session,registry,addresses,path,_,attached⟩ := entered
 unfold OwnerMetadataSession.attach at attached
 split at attached
 · simp only [Option.some.injEq] at attached
   subst s
   exact trace_rooted (MixedOwnerContinuation.rooted_source path)
 · cases attached

theorem metadata_tick (valid : Valid s.session.state.owner) :
 Valid (OwnerMetadataSession.tick s member principal).session.state.owner := by
 rcases OwnerStatementCompletion.metadata_tick_route (s:=s) (member:=member) (principal:=principal) with same | refused
 · rw [same,OwnerStatementHistory.tick_owner]; exact valid
 · rw [refused]; exact valid

theorem metadata_transfer (valid : Valid s.session.state.owner)
 (accepted : OwnerMetadataSession.transfer s = some next) : Valid next.session.state.owner := by
 unfold OwnerMetadataSession.transfer at accepted
 cases outer : MixedOwnerContinuation.transfer s.session with
 | none => simp [outer] at accepted
 | some session =>
   simp only [outer,Option.map_some,Option.some.injEq] at accepted
   subst next
   unfold MixedOwnerContinuation.transfer at outer
   cases run : MixedOwnerSourceTrace.transfer s.session.state with
   | none => simp [run] at outer
   | some trace =>
     simp only [run,Option.map_some,Option.some.injEq] at outer
     subst session
     exact trace_step_valid valid (.transfer run)

theorem metadata_service (valid : Valid s.session.state.owner) :
 Valid (OwnerMetadataSession.service s ops).session.state.owner := by
 rcases OwnerMetadataSession.service_route (s:=s) (ops:=ops) with same | ⟨_,_,_,_,_,_,same⟩
 · rw [same]; exact valid
 · rw [same]; exact trace_step_valid valid .service

theorem metadata_change (valid : Valid s.session.state.owner)
 (accepted : OwnerMetadataSession.changeMetadata s member place principal site change = some next) :
 Valid next.session.state.owner := by
 rw [(OwnerMetadataSession.metadata_keeps_packet accepted).2.2.1]
 exact valid

theorem metadata_authority (valid : Valid s.session.state.owner)
 (accepted : OwnerMetadataSession.authorityHead s view = some next) : Valid next.session.state.owner := by
 unfold OwnerMetadataSession.authorityHead at accepted
 cases outer : MixedOwnerContinuation.authorityHead s.session view with
 | none => simp [outer] at accepted
 | some session =>
   simp only [outer,Option.map_some,Option.some.injEq] at accepted
   subst next
   unfold MixedOwnerContinuation.authorityHead at outer
   cases run : MixedOwnerSourceTrace.authorityHead s.session.state view with
   | none => simp [run] at outer
   | some trace =>
     simp only [run,Option.map_some,Option.some.injEq] at outer
     subst session
     exact trace_step_valid valid (.authority run)

theorem metadata_control (valid : Valid s.session.state.owner)
 (accepted : OwnerMetadataSession.controlInput s member place principal raw = some (next,key)) : Valid next.session.state.owner := by
 unfold OwnerMetadataSession.controlInput at accepted
 cases outer : MixedOwnerContinuation.controlInput s.session member place principal raw with
 | none => simp [outer] at accepted
 | some pair =>
   obtain ⟨session,created⟩ := pair
   simp only [outer,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   unfold MixedOwnerContinuation.controlInput at outer
   cases run : MixedOwnerSourceTrace.controlInput s.session.state member place principal raw with
   | none => simp [run] at outer
   | some pair =>
     obtain ⟨trace,key⟩ := pair
     simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at outer
     obtain ⟨rfl,rfl⟩ := outer
     exact trace_step_valid valid (.control run)

theorem metadata_continue (valid : Valid s.session.state.owner)
 (accepted : OwnerMetadataSession.continueWith s program = some next) : Valid next.session.state.owner := by
 unfold OwnerMetadataSession.continueWith at accepted
 cases run : MixedOwnerContinuation.continueWith s.session program with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   rw [(MixedOwnerContinuation.continued_keeps run).1]
   exact valid

theorem metadata_replace (valid : Valid s.session.state.owner)
 (accepted : OwnerMetadataSession.replaceResidual s program = some next) : Valid next.session.state.owner := by
 unfold OwnerMetadataSession.replaceResidual at accepted
 cases run : MixedOwnerContinuation.replaceResidual s.session program with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   rw [(MixedOwnerContinuation.replaced_keeps run).1]
   exact valid

theorem metadata_cancel (valid : Valid s.session.state.owner) :
 Valid (OwnerMetadataSession.cancel s member principal).session.state.owner := by
 unfold OwnerMetadataSession.cancel MixedOwnerContinuation.cancel
 dsimp only
 split <;> exact valid

-- Metadata transitions are NOT assumed to be base-session transitions.
-- Each actual wrapper route above is inspected, including refusal.
theorem metadata_rooted {initial : OwnerMetadataSession.State p a} (valid : Valid initial.session.state.owner)
 (path : OwnerMetadataSession.Rooted initial s) : Valid s.session.state.owner := by
 induction path with
 | initial => exact valid
 | tick _ ih => exact metadata_tick ih
 | transfer _ accepted ih => exact metadata_transfer ih accepted
 | service _ ih => exact metadata_service ih
 | metadata _ accepted ih => exact metadata_change ih accepted
 | authority _ accepted ih => exact metadata_authority ih accepted
 | control _ accepted ih => exact metadata_control ih accepted
 | continued _ accepted ih => exact metadata_continue ih accepted
 | replaced _ accepted ih => exact metadata_replace ih accepted
 | cancel _ ih => exact metadata_cancel ih

theorem admitted_valid (admitted : OwnerStatementAdmission.Admitted realm view policy store s) :
 Valid s.session.state.owner := by
 obtain ⟨initial,entered,path⟩ := admitted
 exact metadata_rooted (initial_valid entered) path

theorem erased_not_admitted (nonempty : s.session.state.owner.history ≠ []) :
 ¬OwnerStatementAdmission.Admitted realm view policy store (OwnerStatementHistoryOrigin.eraseAttempts s) := by
 intro admitted
 have exactHistory := (admitted_valid admitted).1
 exact nonempty (by simpa [OwnerStatementHistoryOrigin.Exact,OwnerStatementHistoryOrigin.eraseAttempts] using exactHistory)

-- Even Facts AND Exact do not authenticate an arbitrary attempt row.
def replaceOne (s : OwnerMetadataSession.State p a) (saved : OwnerSavedPending.Saved)
 (written : Record) : OwnerMetadataSession.State p a :=
 {s with session := {s.session with state := {s.session.state with
   owner := {s.session.state.owner with attempts := [(saved,Result.committed written)]}}}}

theorem replaced_weak_facts (valid : OwnerStatementAdmission.Facts s)
 (empty : s.session.state.owner.queued = none) : OwnerStatementAdmission.Facts (replaceOne s saved written) := by
 obtain ⟨compiled,cursor,binding,history,joint⟩ := valid
 refine ⟨compiled,cursor,binding,history,⟨joint.1.1,joint.1.2.1,?_,joint.1.2.2.2⟩,joint.2⟩
 simp [replaceOne,Invariant,empty]

theorem replaced_exact (valid : OwnerStatementHistoryOrigin.Exact s.session.state.owner)
 (attempt : s.session.state.owner.attempts = [(prior,Result.committed written)]) :
 OwnerStatementHistoryOrigin.Exact (replaceOne s saved written).session.state.owner := by
 simpa [OwnerStatementHistoryOrigin.Exact,attempt,replaceOne,OwnerStatementHistoryOrigin.committed] using valid

theorem replaced_not_admitted (different : written.pending ≠ saved) :
 ¬OwnerStatementAdmission.Admitted realm view policy store (replaceOne s saved written) := by
 intro admitted
 exact different ((admitted_valid admitted).2 saved written (by simp [replaceOne]))

-- The actual successor remains in the statement; Accepted alone does not
-- validate an arbitrary supplied next session/status/program/cursor remainder.
theorem completion_changed_iff (admitted : OwnerStatementAdmission.Admitted realm view policy store s) :
 (OwnerStatementPosition.writes (OwnerMetadataSession.tick s member principal).session.cursor.completed ≠
  OwnerStatementPosition.writes s.session.cursor.completed) ↔
 OwnerStatementCompletion.Accepted s.session (OwnerMetadataSession.tick s member principal).session := by
 constructor
 · intro changed
   exact (OwnerStatementCompletion.admitted_tick_delta admitted).resolve_left changed
 · intro accepted same
   obtain ⟨_,_,append⟩ := OwnerStatementCompletion.accepted_projection accepted
   have lengths := congrArg List.length same
   rw [append] at lengths
   simp at lengths

#print axioms enqueue_bound
#print axioms advance_bound
#print axioms trace_step_valid
#print axioms trace_rooted
#print axioms initial_valid
#print axioms metadata_rooted
#print axioms admitted_valid
#print axioms erased_not_admitted
#print axioms completion_changed_iff
#print axioms replaced_weak_facts
#print axioms replaced_exact
#print axioms replaced_not_admitted
end MirroreaProofFirst.OwnerStatementRetainedHistory
