import CohortHostFailure
open MirroreaProofFirst
namespace CohortRetiredProbe
set_option maxHeartbeats 1600000

-- A retired public view still acquires its physical guard before discovering
-- retirement. This is not semantic entry or permission to send. Keep these
-- physical check/cleanup steps distinct from CohortCommitJournal.enter.
inductive Event where
  | acquire (callId : Nat)
  | release
  | nativeRequest
  | commitStore

inductive Step : CohortCommitJournal.State p a → Event → CohortCommitJournal.State p a → Prop where
  | acquire : Step ⟨memory,pending,none,false,true⟩ (.acquire callId)
      ⟨memory,pending,some callId,true,true⟩
  | release : Step ⟨memory,pending,some callId,true,true⟩ .release
      ⟨memory,pending,none,false,true⟩

def advance : CohortCommitJournal.State p a → Event → Option (CohortCommitJournal.State p a)
  | ⟨memory,pending,none,false,true⟩,.acquire callId => some ⟨memory,pending,some callId,true,true⟩
  | ⟨memory,pending,some _,true,true⟩,.release => some ⟨memory,pending,none,false,true⟩
  | _,_ => none

theorem advance_exact : advance s event = some next ↔ Step s event next := by
  constructor
  · intro checked
    rcases s with ⟨memory,pending,call,gate,retired⟩
    cases call <;> cases gate <;> cases retired <;> cases event <;>
      simp only [advance] at checked <;> try contradiction
    all_goals cases Option.some.inj checked; constructor
  · intro step; cases step <;> rfl

theorem frames (step : Step s event next) :
    next.memory = s.memory ∧ next.pending = s.pending ∧ next.retired = s.retired := by
  cases step <;> exact ⟨rfl,rfl,rfl⟩

theorem preserves (step : Step s event next) : CohortCommitJournal.WellFormed next := by
  cases step <;> simp [CohortCommitJournal.WellFormed]

theorem no_native (s : CohortCommitJournal.State p a) : advance s .nativeRequest = none := by
  rcases s with ⟨memory,pending,call,gate,retired⟩
  cases call <;> cases gate <;> cases retired <;> rfl

theorem no_commit (s : CohortCommitJournal.State p a) : advance s .commitStore = none := by
  rcases s with ⟨memory,pending,call,gate,retired⟩
  cases call <;> cases gate <;> cases retired <;> rfl

inductive Runs : CohortCommitJournal.State p a → CohortCommitJournal.State p a → Prop where
  | nil : Runs s s
  | step : Runs first s → Step s event next → Runs first next

theorem Runs.frames (path : Runs first last) :
    last.memory = first.memory ∧ last.pending = first.pending ∧ last.retired = first.retired := by
  induction path with
  | nil => exact ⟨rfl,rfl,rfl⟩
  | step prior step ih =>
    have one := CohortRetiredProbe.frames step
    exact ⟨one.1.trans ih.1,one.2.1.trans ih.2.1,one.2.2.trans ih.2.2⟩

theorem Runs.valid (initial : CohortCommitJournal.WellFormed first) (path : Runs first last) :
    CohortCommitJournal.WellFormed last := by
  cases path with
  | nil => exact initial
  | step _ step => exact preserves step

-- General positive, including arbitrary nonempty unfinished local-store debt:
-- the refusal path releases its physical guard without discharging that debt.
theorem refusal_path (memory : CohortCommitJournal.Memory p a)
    (pending : List (CohortCommitJournal.Store p a)) (callId : Nat) :
    Runs ⟨memory,pending,none,false,true⟩ ⟨memory,pending,none,false,true⟩ :=
  (Runs.step Runs.nil (Step.acquire (callId:=callId))).step Step.release

-- Explicit checker acceptance prevents the reflexive Runs endpoint alone
-- from serving as a vacuous positive witness for an all-reject checker.
theorem refusal_checks (memory : CohortCommitJournal.Memory p a)
    (pending : List (CohortCommitJournal.Store p a)) (callId : Nat) :
    advance ⟨memory,pending,none,false,true⟩ (.acquire callId) =
      some ⟨memory,pending,some callId,true,true⟩ ∧
    advance ⟨memory,pending,some callId,true,true⟩ .release =
      some ⟨memory,pending,none,false,true⟩ := ⟨rfl,rfl⟩

variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

structure Certified (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  origin : CohortHostExecution.Live base
  retired : origin.current.state.cohort.retired = true
  cleaned : origin.current.state.cohort.gate = false ∧ origin.current.state.cohort.call = none
  journal : CohortCommitJournal.State p a
  path : Runs origin.current.state.cohort journal

def openProbe (origin : CohortHostExecution.Live base) : Option (Certified base) :=
  if retired : origin.current.state.cohort.retired = true then
    if cleaned : origin.current.state.cohort.gate = false ∧ origin.current.state.cohort.call = none then
      some ⟨origin,retired,cleaned,origin.current.state.cohort,.nil⟩
    else none
  else none

theorem open_complete (retired : origin.current.state.cohort.retired = true)
    (cleaned : origin.current.state.cohort.gate = false ∧ origin.current.state.cohort.call = none) :
    (openProbe origin).isSome = true := by simp [openProbe,retired,cleaned]

theorem release_clean (step : CohortHostPrefix.Step before .releaseRetired after) :
    after.cohort.retired = true ∧ after.cohort.gate = false ∧ after.cohort.call = none := by
  cases step with
  | release localStep =>
    have facts := CohortCommitJournal.cleanup_retains localStep
    exact facts.2.2

theorem released_open (released : CohortHostFailure.Released base) :
    (openProbe released.current).isSome = true := by
  have facts := release_clean released.step
  exact open_complete facts.1 facts.2

theorem open_keeps (checked : openProbe origin = some opened) :
    opened.origin = origin ∧ opened.journal = origin.current.state.cohort := by
  unfold openProbe at checked
  split at checked
  · split at checked
    · cases Option.some.inj checked; exact ⟨rfl,rfl⟩
    · cases checked
  · cases checked

def Certified.advance (before : Certified base) (event : Event) : Option (Certified base) :=
  match checked : CohortRetiredProbe.advance before.journal event with
  | none => none
  | some next => some {before with journal:=next,path:=.step before.path (advance_exact.mp checked)}

theorem Certified.advance_step (checked : Certified.advance before event = some after) :
    after.origin = before.origin ∧ Step before.journal event after.journal := by
  unfold Certified.advance at checked
  split at checked
  · cases checked
  · rename_i next ran
    cases Option.some.inj checked
    exact ⟨rfl,advance_exact.mp ran⟩

theorem Certified.advance_complete (step : Step before.journal event next) :
    (Certified.advance before event).isSome = true := by
  unfold Certified.advance
  split
  · rename_i absent
    have present := advance_exact.mpr step
    rw [absent] at present; cases present
  · rfl

def Certified.source (checked : Certified base) : SourceEntryPrefix.Certified base := checked.origin.source

theorem Certified.keeps (checked : Certified base) :
    checked.journal.memory = checked.origin.current.state.cohort.memory ∧
    checked.journal.pending = checked.origin.current.state.cohort.pending ∧
    checked.journal.retired = true := by
  have kept := checked.path.frames
  exact ⟨kept.1,kept.2.1,kept.2.2.trans checked.retired⟩

theorem Certified.valid (checked : Certified base) : CohortCommitJournal.WellFormed checked.journal :=
  checked.path.valid checked.origin.valid.2.2.1

#print axioms advance_exact
#print axioms frames
#print axioms preserves
#print axioms no_native
#print axioms no_commit
#print axioms refusal_path
#print axioms refusal_checks
#print axioms open_complete
#print axioms released_open
#print axioms open_keeps
#print axioms Certified.advance_step
#print axioms Certified.advance_complete
#print axioms Certified.keeps
#print axioms Certified.valid
end CohortRetiredProbe
