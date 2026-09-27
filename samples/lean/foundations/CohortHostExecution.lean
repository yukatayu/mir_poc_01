import CohortHostStartup
open MirroreaProofFirst
namespace CohortHostExecution
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}

abbrev Snapshot (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) :=
  (SharedWireLifetime.native base).driver.source.map SourcePublicationWorker.project

-- The actual startup journal is retained as the origin of the SAME running
-- prefix. A separately replayed projection cannot replace this current state.
structure Live (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  callId : Nat
  startup : CohortHostStartup.Certified (Snapshot base)
  handed : startup.state = ⟨.launched,(CohortHostStartup.anchor base callId).cohort⟩
  current : CohortHostPrefix.Certified (CohortHostStartup.anchor base callId)

variable {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

def openLive (startup : CohortHostStartup.Certified (Snapshot base)) : Option (Live base) :=
  if phase : startup.state.phase = .launched then
    match call : startup.state.journal.call with
    | none => none
    | some callId =>
      have handed : startup.state = ⟨.launched,(CohortHostStartup.anchor base callId).cohort⟩ := by
        obtain ⟨actual,equal⟩ := CohortHostStartup.launched_shape startup.path phase
        have idEq : actual = callId := by simpa [equal] using call
        subst actual
        exact equal
      some ⟨callId,startup,handed,⟨⟨.start base,startup.state.journal⟩,by rw [handed]; exact .nil⟩⟩
  else none

theorem openLive_keeps (checked : openLive startup = some next) :
    next.startup = startup ∧ next.current.state.cohort = startup.state.journal ∧
    next.current.state.inner = .start base := by
  unfold openLive at checked
  split at checked
  · split at checked
    · cases checked
    · cases Option.some.inj checked; exact ⟨rfl,rfl,rfl⟩
  · cases checked

def Live.advance (before : Live base) (event : CohortHostPrefix.Event p a) : Option (Live base) :=
  (before.current.advance event).map fun next => {before with current:=next}

theorem Live.advance_step (checked : Live.advance before event = some after) :
    CohortHostPrefix.Step before.current.state event after.current.state := by
  obtain ⟨next,ran,equal⟩ := Option.map_eq_some_iff.mp checked
  cases equal
  exact CohortHostPrefix.Certified.advance_step ran

def Live.source (live : Live base) : SourceEntryPrefix.Certified base := live.current.source rfl

theorem Live.valid (live : Live base) : CohortHostPrefix.Invariant live.current.state :=
  live.current.valid (CohortHostStartup.anchor_valid base live.callId)

inductive Cursor (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  | startup (checked : CohortHostStartup.Certified (Snapshot base))
  | live (checked : Live base)

def Cursor.start (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) : Cursor base :=
  .startup (.start (Snapshot base))

inductive Event (p a : Nat) where
  | enter (callId : Nat)
  | bootstrapReturned
  | launchReturned
  | inner (event : SourceEntryPrefix.Event p a)
  | commit
  | close
  | retire
  | releaseRetired

def Cursor.advance (before : Cursor base) : Event p a → Option (Cursor base)
  | .enter callId => match before with
    | .startup checked => (checked.advance (.enter callId)).map .startup
    | .live checked => (checked.advance (.enter callId)).map .live
  | .bootstrapReturned => match before with
    | .startup checked => (checked.advance .bootstrapReturned).map .startup
    | .live _ => none
  | .launchReturned => match before with
    | .startup checked => do
      let next ← checked.advance .launchReturned
      let live ← openLive next
      some (.live live)
    | .live _ => none
  | .inner event => match before with
    | .startup _ => none
    | .live checked => (checked.advance (.inner event)).map .live
  | .commit => match before with
    | .startup checked => (checked.advance .commit).map .startup
    | .live checked => (checked.advance .commit).map .live
  | .close => match before with
    | .startup checked => (checked.advance .close).map .startup
    | .live checked => (checked.advance .close).map .live
  | .retire => match before with
    | .startup _ => none -- prelaunch fault prefixes are outside this first consumer cut
    | .live checked => (checked.advance .retire).map .live
  | .releaseRetired => match before with
    | .startup _ => none
    | .live checked => (checked.advance .releaseRetired).map .live

def Cursor.journal : Cursor base → CohortCommitJournal.State p a
  | .startup checked => checked.state.journal
  | .live checked => checked.current.state.cohort

def Cursor.source : Cursor base → Option (SourceEntryPrefix.Certified base)
  | .startup _ => none
  | .live checked => some checked.source

theorem Cursor.wellFormed (cursor : Cursor base) : CohortCommitJournal.WellFormed cursor.journal := by
  cases cursor with
  | startup checked => exact checked.path.valid.1
  | live checked => exact checked.valid.2.2.1

theorem Cursor.no_inner_before_launch (checked : CohortHostStartup.Certified (Snapshot base)) :
    (Cursor.startup checked).advance (.inner event) = none := rfl

#print axioms openLive_keeps
#print axioms Live.advance_step
#print axioms Live.valid
#print axioms Cursor.wellFormed
#print axioms Cursor.no_inner_before_launch
end CohortHostExecution
