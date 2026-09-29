import OwnerStatementAdmission
namespace MirroreaProofFirst.OwnerStatementHistoryOrigin
open MixedOwnerAttemptQueue

def committed : OwnerSavedPending.Saved × Result → Option Record
 | (_, .committed written) => some written
 | (_, .refused _) => none
-- This is an internal attempt/history correspondence, not equality of owner
-- history with acknowledged source writes. Unacknowledged commits remain legal.
def Exact (s : State) : Prop := s.history = s.attempts.filterMap committed

theorem enqueue_exact_history (valid : Exact s) (accepted : enqueue s saved = some next) : Exact next := by
 obtain ⟨_,_,rfl⟩ := enqueue_exact.mp accepted
 exact valid

theorem advance_exact_history (valid : Exact s) : Exact (advance ops catalog view current s).1 := by
 cases run : (advance ops catalog view current s).2 with
 | none => rw [not_attempted_retains run]; exact valid
 | some result =>
   obtain ⟨_,_,_,_,_,_,_,_,attempts,_,history⟩ := advance_parts run
   unfold Exact
   rw [history,attempts,List.filterMap_append,←valid]
   cases result <;> simp [historyAfter,committed]

theorem trace_step (valid : Exact s.owner) (step : MixedOwnerSourceTrace.Step s next) : Exact next.owner := by
 cases step with
 | issue accepted => obtain ⟨_,_,_,rfl⟩ := MixedOwnerSourceTrace.issue_parts accepted; exact valid
 | transfer accepted =>
   obtain ⟨_,_,_,_,enqueued,rfl⟩ := MixedOwnerSourceTrace.transfer_parts accepted
   exact enqueue_exact_history valid enqueued
 | service =>
   unfold MixedOwnerSourceTrace.service
   split
   · exact advance_exact_history valid
   · exact valid
 | receive accepted => obtain ⟨_,_,_,_,_,rfl⟩ := MixedOwnerSourceTrace.receive_parts accepted; exact valid
 | declaration => exact valid
 | pureAdvance => exact valid
 | pureComplete => exact valid
 | pureCancel => exact valid
 | control accepted => obtain ⟨_,rfl⟩ := MixedOwnerSourceTrace.control_parts accepted; exact valid
 | authority accepted => obtain ⟨_,rfl⟩ := MixedOwnerSourceTrace.authority_parts accepted; exact valid

theorem trace_rooted (path : MixedOwnerSourceTrace.Rooted source store s) : Exact s.owner := by
 induction path with
 | initial => rfl
 | step _ transition ih => exact trace_step ih transition

theorem session_rooted (path : MixedOwnerContinuation.Rooted realm view policy store s) : Exact s.state.owner :=
 trace_rooted (MixedOwnerContinuation.rooted_source path)

theorem initial_exact (entered : OwnerStatementAdmission.Initial realm view policy store s) :
 Exact s.session.state.owner := by
 obtain ⟨session,registry,addresses,path,_,attached⟩ := entered
 unfold OwnerMetadataSession.attach at attached
 split at attached
 · simp only [Option.some.injEq] at attached
   subst s
   exact session_rooted path
 · cases attached

theorem forged_initial_excluded
 (noAttempts : s.session.state.owner.attempts = [])
 (hasHistory : s.session.state.owner.history ≠ []) :
 ¬OwnerStatementAdmission.Initial realm view policy store s := by
 intro entered
 have exactHistory := initial_exact entered
 exact hasHistory (by simpa [Exact,noAttempts] using exactHistory)

-- A stronger raw-image counterexample than a mismatched cursor partition:
-- deleting all attempts retains ALL previous preservation predicates, including
-- the compiler witness and the origin-matching history predicate.
def eraseAttempts (s : OwnerMetadataSession.State p a) : OwnerMetadataSession.State p a :=
 {s with session := {s.session with state := {s.session.state with
   owner := {s.session.state.owner with attempts := []}}}}

theorem erased_weak_facts (valid : OwnerStatementAdmission.Facts s) :
 OwnerStatementAdmission.Facts (eraseAttempts s) := by
 obtain ⟨compiled,cursor,binding,history,joint⟩ := valid
 refine ⟨compiled,cursor,binding,history,⟨joint.1.1,joint.1.2.1,?_,joint.1.2.2.2⟩,joint.2⟩
 simp [eraseAttempts,Invariant,Seen]

theorem erased_not_initial (hasHistory : s.session.state.owner.history ≠ []) :
 ¬OwnerStatementAdmission.Initial realm view policy store (eraseAttempts s) :=
 forged_initial_excluded rfl hasHistory

#print axioms enqueue_exact_history
#print axioms advance_exact_history
#print axioms trace_step
#print axioms trace_rooted
#print axioms session_rooted
#print axioms initial_exact
#print axioms forged_initial_excluded
#print axioms erased_weak_facts
#print axioms erased_not_initial
end MirroreaProofFirst.OwnerStatementHistoryOrigin
