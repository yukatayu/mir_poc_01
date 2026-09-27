import CohortHostDebt
open MirroreaProofFirst SharedHostCaptureReplay
namespace CohortHostPrefix
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- Relative prefix product. Initial physical creation, full observation decoding
-- and gate-acquisition/exception restoration bindings are still obligations.
-- A caller cannot inject an arbitrary cohort receipt into this transition API.
structure State (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  inner : SourceEntryPrefix.State base
  cohort : CohortCommitJournal.State p a

inductive Event (p a : Nat) where
  | enter (callId : Nat)
  | inner (event : SourceEntryPrefix.Event p a)
  | commit
  | close
  | retire
  | releaseRetired

def idle (s : SourceEntryPrefix.State base) : Bool := s.active.isNone && s.joined.active.isNone

theorem idle_exact : idle s = true ↔ s.active = none ∧ s.joined.active = none := by
  simp [idle]

-- This selected prefix starts after launch. Every actual outer close runs
-- the existing local completion step: even a closed Joint may still have an
-- initialization prelude whose final obligation has just been discharged.
-- Skipping localComplete merely because no work is open loses that transition.
def finish (s : SourceEntryPrefix.State base) : Option (SourceEntryPrefix.State base) :=
  if idle s then SourceEntryPrefix.advance s (.ordinary .localComplete) else none

inductive Finish : SourceEntryPrefix.State base → SourceEntryPrefix.State base → Prop where
  | completed (noInner : idle s = true)
      (step : SourceEntryPrefix.Step s (.ordinary .localComplete) next) : Finish s next

theorem finish_exact : finish s = some next ↔ Finish s next := by
  constructor
  · intro checked
    unfold finish at checked
    split at checked
    · rename_i noInner
      exact .completed noInner (SourceEntryPrefix.advance_sound checked)
    · cases checked
  · intro step
    cases step with
    | completed noInner ran => simp [finish,noInner,SourceEntryPrefix.advance_complete ran]

def advance (s : State base) : Event p a → Option (State base)
  | .inner event => do
    let next ← SourceEntryPrefix.advance s.inner event
    let cohort ← CohortHostDebt.advance s.inner next s.cohort event
    some ⟨next,cohort⟩
  | .enter callId =>
    if idle s.inner then (CohortCommitJournal.advance s.cohort (.enter callId)).map (⟨s.inner,·⟩) else none
  | .commit =>
    if idle s.inner then (CohortCommitJournal.advance s.cohort .commit).map (⟨s.inner,·⟩) else none
  | .close => do
    let cohort ← CohortCommitJournal.advance s.cohort .returnCall
    let next ← finish s.inner
    some ⟨next,cohort⟩
  | .retire => do
    let next ← SourceEntryPrefix.advance s.inner .retire
    let cohort ← CohortCommitJournal.advance s.cohort .retire
    some ⟨next,cohort⟩
  | .releaseRetired =>
    (CohortCommitJournal.advance s.cohort .releaseRetired).map (⟨s.inner,·⟩)

-- Separate declarative rules: the same inner successor is used both by the
-- native/source journal and the receipt derivation. Generic cohort commits are
-- forbidden while a source-entry or owner journal is open; the one source
-- snapshot store is consumed in BOTH journals by CohortHostDebt.Step.snapshot.
inductive Step : State base → Event p a → State base → Prop where
  | inner (native : SourceEntryPrefix.Step s.inner event next)
      (localStores : CohortHostDebt.Step s.inner next s.cohort event cohort) :
      Step s (.inner event) ⟨next,cohort⟩
  | enter (noInner : idle s.inner = true)
      (localStores : CohortCommitJournal.Step s.cohort (.enter callId) cohort) :
      Step s (.enter callId) ⟨s.inner,cohort⟩
  | commit (noInner : idle s.inner = true)
      (localStores : CohortCommitJournal.Step s.cohort .commit cohort) :
      Step s .commit ⟨s.inner,cohort⟩
  | close (localStores : CohortCommitJournal.Step s.cohort .returnCall cohort)
      (native : Finish s.inner next) : Step s .close ⟨next,cohort⟩
  | retire (native : SourceEntryPrefix.Step s.inner .retire next)
      (localStores : CohortCommitJournal.Step s.cohort .retire cohort) :
      Step s .retire ⟨next,cohort⟩
  | release (localStores : CohortCommitJournal.Step s.cohort .releaseRetired cohort) :
      Step s .releaseRetired ⟨s.inner,cohort⟩

theorem advance_complete (step : Step s event next) : advance s event = some next := by
  cases step with
  | inner native localStores =>
    simp [advance,SourceEntryPrefix.advance_complete native,CohortHostDebt.advance_complete localStores]
  | enter noInner localStores => simp [advance,noInner,CohortCommitJournal.advance_exact.mpr localStores]
  | commit noInner localStores => simp [advance,noInner,CohortCommitJournal.advance_exact.mpr localStores]
  | close localStores native => simp [advance,CohortCommitJournal.advance_exact.mpr localStores,finish_exact.mpr native]
  | retire native localStores => simp [advance,SourceEntryPrefix.advance_complete native,CohortCommitJournal.advance_exact.mpr localStores]
  | release localStores => simp [advance,CohortCommitJournal.advance_exact.mpr localStores]

theorem advance_sound (checked : advance s event = some next) : Step s event next := by
  cases event <;> simp only [advance] at checked
  case inner event =>
    obtain ⟨after,first,tail⟩ := Option.bind_eq_some_iff.mp checked
    obtain ⟨cohort,second,equal⟩ := Option.bind_eq_some_iff.mp tail
    cases Option.some.inj equal
    exact .inner (SourceEntryPrefix.advance_sound first) (CohortHostDebt.advance_sound second)
  case enter callId =>
    split at checked
    · rename_i noInner
      obtain ⟨cohort,ran,equal⟩ := Option.map_eq_some_iff.mp checked
      cases equal
      exact .enter noInner (CohortCommitJournal.advance_exact.mp ran)
    · cases checked
  case commit =>
    split at checked
    · rename_i noInner
      obtain ⟨cohort,ran,equal⟩ := Option.map_eq_some_iff.mp checked
      cases equal
      exact .commit noInner (CohortCommitJournal.advance_exact.mp ran)
    · cases checked
  case close =>
    obtain ⟨cohort,localStores,tail⟩ := Option.bind_eq_some_iff.mp checked
    obtain ⟨after,native,equal⟩ := Option.bind_eq_some_iff.mp tail
    cases Option.some.inj equal
    exact .close (CohortCommitJournal.advance_exact.mp localStores) (finish_exact.mp native)
  case retire =>
    obtain ⟨after,native,tail⟩ := Option.bind_eq_some_iff.mp checked
    obtain ⟨cohort,localStores,equal⟩ := Option.bind_eq_some_iff.mp tail
    cases Option.some.inj equal
    exact .retire (SourceEntryPrefix.advance_sound native) (CohortCommitJournal.advance_exact.mp localStores)
  case releaseRetired =>
    obtain ⟨cohort,ran,equal⟩ := Option.map_eq_some_iff.mp checked
    cases equal
    exact .release (CohortCommitJournal.advance_exact.mp ran)

theorem advance_exact : advance s event = some next ↔ Step s event next := ⟨advance_sound,advance_complete⟩

-- Necessary closure facts are derived from the rules, not invariant guards.
theorem close_discharged (step : Step s .close next) :
    s.cohort.pending = [] ∧ s.cohort.retired = false ∧
    s.inner.active = none ∧ s.inner.joined.active = none ∧
    next.cohort.memory = s.cohort.memory := by
  cases step with
  | close localStores native =>
    have facts := CohortCommitJournal.return_discharged localStores
    cases native <;> exact ⟨facts.1,facts.2.1,(idle_exact.mp (by assumption)).1,
      (idle_exact.mp (by assumption)).2,facts.2.2⟩

theorem commit_cannot_bypass_entry (held : s.inner.active = some opened) : advance s .commit = none := by
  simp [advance,idle,held]

theorem cleanup_keeps_debt (step : Step s .releaseRetired next) :
    next.inner = s.inner ∧ next.cohort.memory = s.cohort.memory ∧
    next.cohort.pending = s.cohort.pending ∧ next.cohort.retired = true := by
  cases step with
  | release ran =>
    have retained := CohortCommitJournal.cleanup_retains ran
    exact ⟨rfl,retained.1,retained.2.1,retained.2.2.1⟩

theorem inner_stop_frame (native : SourceEntryPrefix.Step before event after)
    (localStores : CohortHostDebt.Step before after s event next) :
    after.joined.stopped = before.joined.stopped := by
  have allowed : CohortHostDebt.kind event ≠ .forbidden := by
    cases localStores <;> simp_all [CohortHostDebt.kind]
  cases native with
  | ordinary live noInner permitted bound ran =>
    cases SharedHostCaptureReplay.advance_sound ran <;> simp_all [CohortHostDebt.kind]
  | claim => rfl
  | reply live opened bound localStep wireStep =>
    cases SharedHostCaptureReplay.advance_sound wireStep; rfl
  | store => rfl
  | retireIdle => simp [CohortHostDebt.kind] at allowed
  | retireActive => simp [CohortHostDebt.kind] at allowed

theorem finish_stop_frame (step : Finish before after) : after.joined.stopped = before.joined.stopped := by
  cases step with
  | completed noInner native =>
    cases native with
    | ordinary live noInner permitted bound ran =>
      cases SharedHostCaptureReplay.advance_sound ran; rfl

theorem retire_stopped (step : SourceEntryPrefix.Step before .retire after) : after.joined.stopped = true := by
  cases step with
  | retireIdle live noInner ran => cases SharedHostCaptureReplay.advance_sound ran; rfl
  | retireActive live opened ran localStep => cases SharedHostCaptureReplay.advance_sound ran; rfl

theorem inner_path (step : Step s event next) :
    ∃ events, SourceEntryPrefix.Runs s.inner events next.inner := by
  cases step with
  | inner native => exact ⟨[_],.step .nil native⟩
  | enter | commit | release => exact ⟨[],.nil⟩
  | retire native => exact ⟨[.retire],.step .nil native⟩
  | close localStores native =>
    cases native with
    | completed noInner native => exact ⟨[.ordinary .localComplete],.step .nil native⟩

-- These are source/writer history and local lifecycle invariants. Full cohort
-- memory/native correspondence requires receipt-origin and observation binding;
-- it is not asserted merely by this conjunction.
def Invariant (s : State base) : Prop :=
  SourceEntryPrefix.Invariant s.inner ∧ HistoryInvariant s.inner.joined ∧
  CohortCommitJournal.WellFormed s.cohort ∧ s.inner.joined.stopped = s.cohort.retired

theorem preserves (valid : Invariant s) (step : Step s event next) : Invariant next := by
  rcases s with ⟨inner,cohort⟩
  obtain ⟨events,path⟩ := inner_path step
  refine ⟨SourceEntryPrefix.runs_preserves valid.1 path,
    runs_history valid.2.1 (SourceEntryPrefix.runs_projection path),?_,?_⟩
  · cases step with
    | inner native localStores => exact CohortHostDebt.preserves valid.2.2.1 localStores
    | enter _ localStores | commit _ localStores | close localStores _ | retire _ localStores | release localStores =>
      exact CohortCommitJournal.preserves valid.2.2.1 localStores
  · cases step with
    | inner native localStores =>
      exact (inner_stop_frame native localStores).trans (valid.2.2.2.trans (CohortHostDebt.frames_gate localStores).2.2.symm)
    | enter noInner localStores | commit noInner localStores =>
      cases localStores
      exact valid.2.2.2
    | close localStores native =>
      have retained := finish_stop_frame native
      cases localStores
      exact retained.trans valid.2.2.2
    | retire native localStores =>
      cases localStores
      exact retire_stopped native
    | release localStores => cases localStores; exact valid.2.2.2

-- Constructive relative admission: every already allowed plain native step is
-- admitted with its exact successor under the real outer entry and empty local
-- debt. No proposed invariant or successful future outcome is an input.
theorem inner_lift (native : SourceEntryPrefix.Step inner event after)
    (plain : CohortHostDebt.kind event = .plain) (memory : CohortCommitJournal.Memory p a) (callId : Nat) :
    Step ⟨inner,⟨memory,[],some callId,true,false⟩⟩ (.inner event)
      ⟨after,⟨memory,CohortHostDebt.compileReceipts memory (CohortHostDebt.receipts inner after event),some callId,true,false⟩⟩ :=
  .inner native (.plain rfl rfl rfl rfl plain)

inductive Runs : State base → State base → Prop where
  | nil : Runs s s
  | step : Runs first s → Step s event next → Runs first next

theorem Runs.trans (left : Runs before middle) (right : Runs middle after) : Runs before after := by
  induction right with
  | nil => exact left
  | step prior step ih => exact .step ih step

theorem discharge (noInner : idle inner = true) (memory : CohortCommitJournal.Memory p a)
    (pending : List (CohortCommitJournal.Store p a)) (callId : Nat) :
    Runs ⟨inner,⟨memory,pending,some callId,true,false⟩⟩
      ⟨inner,⟨pending.foldl CohortCommitJournal.write memory,[],some callId,true,false⟩⟩ := by
  induction pending generalizing memory with
  | nil => exact .nil
  | cons store rest ih =>
    exact (Runs.step Runs.nil (.commit noInner .commit)).trans (ih (CohortCommitJournal.write memory store))

-- Payment is protocol debt in memory, not unfinished local stores. This general
-- positive path accepts arbitrary retained payment/fields while enforcing each
-- prescribed store; physical memory-to-native correspondence remains separate.
theorem discharge_and_close (completed : Finish inner after) (memory : CohortCommitJournal.Memory p a)
    (pending : List (CohortCommitJournal.Store p a)) (callId : Nat) :
    Runs ⟨inner,⟨memory,pending,some callId,true,false⟩⟩
      ⟨after,⟨pending.foldl CohortCommitJournal.write memory,[],none,false,false⟩⟩ := by
  have noInner : idle inner = true := by cases completed <;> assumption
  exact (discharge noInner memory pending callId).trans (Runs.step Runs.nil (.close .returnCall completed))

theorem Runs.inner_history (path : Runs before after) :
    ∃ events, SourceEntryPrefix.Runs before.inner events after.inner := by
  induction path with
  | nil => exact ⟨[],.nil⟩
  | step prior step ih =>
    obtain ⟨earlier,left⟩ := ih
    obtain ⟨later,right⟩ := inner_path step
    exact ⟨earlier++later,left.append right⟩

theorem Runs.preserves (valid : Invariant before) (path : Runs before after) : Invariant after := by
  induction path with
  | nil => exact valid
  | step prior step ih => exact CohortHostPrefix.preserves ih step

-- The anchor is an explicit checked physical cut, not a manufactured fresh
-- process. Creation/launch must separately establish its initial memory. All
-- subsequent source and host stores then update this single paired state.
structure Certified (anchor : State base) where
  state : State base
  path : Runs anchor state

variable {anchor : State base}

def Certified.start (anchor : State base) : Certified anchor := ⟨anchor,.nil⟩
def Certified.advance (before : Certified anchor) (event : Event p a) : Option (Certified anchor) :=
  match checked : CohortHostPrefix.advance before.state event with
  | none => none
  | some next => some ⟨next,.step before.path (advance_sound checked)⟩

theorem Certified.advance_step (checked : Certified.advance before event = some after) :
    Step before.state event after.state := by
  unfold Certified.advance at checked
  split at checked
  · cases checked
  · rename_i next ran
    cases Option.some.inj checked
    exact advance_sound ran

theorem Certified.advance_complete (step : Step before.state event after) :
    (Certified.advance before event).map Certified.state = some after := by
  unfold Certified.advance
  split
  · rename_i missing
    have present := CohortHostPrefix.advance_complete step
    rw [missing] at present
    cases present
  · rename_i next ran
    have equal := Option.some.inj (ran.symm.trans (CohortHostPrefix.advance_complete step))
    subst next
    rfl

theorem Certified.valid (checked : Certified anchor) (initial : Invariant anchor) : Invariant checked.state :=
  checked.path.preserves initial

-- View of the SAME recorded inner state/path for existing pending-wire and
-- current-writer consumers. It is not a separately replayed equal projection.
def Certified.source (checked : Certified anchor) (rooted : anchor.inner = .start base) :
    SourceEntryPrefix.Certified base := by
  refine ⟨checked.state.inner, ?_⟩
  obtain ⟨events,path⟩ := checked.path.inner_history
  rw [rooted] at path
  have same := SourceEntryPrefix.runs_events path
  simp only [SourceEntryPrefix.State.start,List.nil_append] at same
  simpa only [same] using path

#print axioms finish_exact
#print axioms advance_exact
#print axioms close_discharged
#print axioms commit_cannot_bypass_entry
#print axioms cleanup_keeps_debt
#print axioms preserves
#print axioms inner_lift
#print axioms discharge_and_close
#print axioms Certified.advance_complete
#print axioms Certified.valid
#print axioms Certified.source
end CohortHostPrefix
