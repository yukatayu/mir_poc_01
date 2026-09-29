import MixedReferenceTrace

namespace MirroreaProofFirst.MixedReferenceChronology
open MixedReferenceStore

def Recorded (before after : MixedReferenceStore.Machine p a) : Prop :=
  after.history = before.history ++ [frame after.core] ∧ ∃ event, after.events = event :: before.events

theorem transition_recorded (m next : MixedReferenceStore.Machine p a) (step : MixedReferenceTrace.Transition m next) :
    next = m ∨ Recorded m next := by
  cases step with
  | management run =>
      obtain ⟨core,created,_,_,equal⟩ := manage_parts _ _ _ _ _ _ _ run
      obtain ⟨rfl,rfl⟩ := Prod.mk.inj equal
      exact Or.inr ⟨rfl,_,rfl⟩
  | authority => exact Or.inr ⟨rfl,_,rfl⟩
  | start run =>
      simp only [start,Option.bind_eq_bind,Option.bind,Option.pure_def] at run
      split at run
      · cases run
      · rename_i pair started
        obtain ⟨core,ticket⟩ := pair
        simp only [Option.some.injEq,Prod.mk.injEq] at run
        obtain ⟨rfl,rfl⟩ := run
        exact Or.inr ⟨rfl,_,rfl⟩
  | startOwner run =>
      simp only [startOwner,Option.bind_eq_bind,Option.bind,Option.pure_def] at run
      split at run
      · cases run
      · rename_i pair started
        obtain ⟨core,saved⟩ := pair
        simp only [Option.some.injEq,Prod.mk.injEq] at run
        obtain ⟨rfl,rfl⟩ := run
        exact Or.inr ⟨rfl,_,rfl⟩
  | cancellation run =>
      obtain ⟨_,core,_,rfl⟩ := cancel_parts _ _ _ _ _ _ _ _ run
      exact Or.inr ⟨rfl,_,rfl⟩
  | finishOwner run =>
      obtain ⟨_,_,rfl⟩ := finishOwner_parts _ run
      exact Or.inr ⟨rfl,_,rfl⟩
  | finishPlain run =>
      simp only [finishPlain,Option.bind_eq_bind,Option.bind,Option.pure_def] at run
      split at run
      · cases run
      · cases run
        exact Or.inr ⟨rfl,_,rfl⟩
  | acquire run => exact Or.inr (MixedReferenceMutation.acquire_recorded _ _ _ _ _ _ _ run)
  | reacquire run => exact Or.inr (MixedReferenceMutation.reacquire_recorded _ _ _ _ _ _ _ run)
  | release run => exact Or.inr (MixedReferenceMutation.release_recorded _ _ _ _ _ _ _ run)
  | normalize => exact MixedReferenceMutation.normalize_recorded _ _ _ _ _ _

theorem changed_recorded (m next : MixedReferenceStore.Machine p a) (step : MixedReferenceTrace.Transition m next)
    (changed : next ≠ m) : Recorded m next := by
  rcases transition_recorded _ _ step with same | recorded
  · exact False.elim (changed same)
  · exact recorded

-- This relation records the actual resulting state at every non-stuttering
-- admitted transition. Its constructors do not assume anything about a stored
-- history list, a stored event list, or equality of serial and event count.
inductive Chronological : MixedReferenceStore.Machine p a → MixedReferenceStore.Machine p a →
    List (MixedReferenceHistory.Frame p a) → Prop where
  | refl : Chronological m m []
  | idle : Chronological first m frames → MixedReferenceTrace.Transition m m → Chronological first m frames
  | step : Chronological first m frames → MixedReferenceTrace.Transition m next → next ≠ m →
      Chronological first next (frames ++ [frame next.core])

theorem reached_chronological (first last : MixedReferenceStore.Machine p a) (path : MixedReferenceTrace.Reached first last) :
    ∃ frames, Chronological first last frames := by
  induction path with
  | refl => exact ⟨[],.refl⟩
  | @step m next previous step ih =>
      obtain ⟨frames,record⟩ := ih
      by_cases same : next = m
      · subst next; exact ⟨frames,.idle record step⟩
      · exact ⟨_,.step record step same⟩

theorem chronological_history (first last : MixedReferenceStore.Machine p a)
    (frames : List (MixedReferenceHistory.Frame p a)) (record : Chronological first last frames) :
    last.history = first.history ++ frames ∧ last.events.length = first.events.length + frames.length := by
  induction record with
  | refl => simp
  | idle previous step ih => exact ih
  | @step m next previousFrames previous step changed ih =>
      obtain ⟨historyAt,event,eventAt⟩ := changed_recorded _ _ step changed
      refine ⟨?_,?_⟩
      · rw [historyAt,ih.1,List.append_assoc]
      · simp only [eventAt,List.length_cons,ih.2,List.length_append,List.length_nil]
        omega

theorem reached_history (first last : MixedReferenceStore.Machine p a) (path : MixedReferenceTrace.Reached first last) :
    ∃ frames, Chronological first last frames ∧ last.history = first.history ++ frames ∧
      last.events.length = first.events.length + frames.length := by
  obtain ⟨frames,record⟩ := reached_chronological _ _ path
  exact ⟨frames,record,chronological_history _ _ _ record⟩

theorem rooted_history_count (realm : Nat) (view : WorldProjection.AuthorityView a)
 (policy : Nat → CurrentUse.Policy) (m : MixedReferenceStore.Machine p a)
 (path : MixedReferenceTrace.Rooted realm view policy m) : m.history.length = m.events.length + 1 := by
 obtain ⟨frames,_,historyAt,countAt⟩ := reached_history _ _ path
 simp only [MixedReferenceStore.initial,List.length_nil,Nat.zero_add] at countAt historyAt
 simp [historyAt,countAt,Nat.add_comm]
#print axioms transition_recorded
#print axioms chronological_history
#print axioms rooted_history_count
end MirroreaProofFirst.MixedReferenceChronology
