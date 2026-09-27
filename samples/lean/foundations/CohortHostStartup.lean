import CohortHostObservation
open MirroreaProofFirst
namespace CohortHostStartup
open CohortCommitJournal
set_option maxHeartbeats 1600000

-- Startup ends at the actual launch reply, before its local snapshot store.
-- The following paired prefix inherits that store debt and the still-held gate.
-- Native startup/launch bytes and actual gate transitions must be bound by the
-- consumer; these labels alone neither authenticate IO nor create authority.
inductive Phase where
  | fresh | bootstrapped | launched
  deriving DecidableEq, BEq

def empty : Memory p a := ⟨false,none,[],[],[],[],none⟩
def expected (snapshot : Option (SourcePublicationWorker.PrivateOutput p a)) : Phase → Memory p a
  | .fresh => empty
  | .bootstrapped => write empty .bootstrap
  | .launched => write (write empty .bootstrap) (.snapshot snapshot)

structure State (p a : Nat) where
  phase : Phase
  journal : CohortCommitJournal.State p a

def initial : State p a := ⟨.fresh,⟨empty,[],none,false,false⟩⟩
inductive Event where
  | enter (callId : Nat)
  | commit
  | close
  | bootstrapReturned
  | launchReturned

def advance (snapshot : Option (SourcePublicationWorker.PrivateOutput p a))
    (s : State p a) (event : Event) : Option (State p a) :=
  if s.phase = .launched then none else
  match event with
  | .enter callId => (CohortCommitJournal.advance s.journal (.enter callId)).map (⟨s.phase,·⟩)
  | .commit => (CohortCommitJournal.advance s.journal .commit).map (⟨s.phase,·⟩)
  | .close => (CohortCommitJournal.advance s.journal .returnCall).map (⟨s.phase,·⟩)
  | .bootstrapReturned =>
    if s.phase = .fresh then
      (CohortCommitJournal.advance s.journal (.arm .bootstrapReturned)).map (⟨.bootstrapped,·⟩)
    else none
  | .launchReturned =>
    if s.phase = .bootstrapped then
      (CohortCommitJournal.advance s.journal (.arm (.sourceReply snapshot false))).map (⟨.launched,·⟩)
    else none

inductive Step (snapshot : Option (SourcePublicationWorker.PrivateOutput p a)) :
    State p a → Event → State p a → Prop where
  | enter (prior : phase ≠ .launched)
      (localStep : CohortCommitJournal.Step journal (.enter callId) next) :
      Step snapshot ⟨phase,journal⟩ (.enter callId) ⟨phase,next⟩
  | commit (prior : phase ≠ .launched)
      (localStep : CohortCommitJournal.Step journal .commit next) :
      Step snapshot ⟨phase,journal⟩ .commit ⟨phase,next⟩
  | close (prior : phase ≠ .launched)
      (localStep : CohortCommitJournal.Step journal .returnCall next) :
      Step snapshot ⟨phase,journal⟩ .close ⟨phase,next⟩
  | bootstrap (localStep : CohortCommitJournal.Step journal (.arm .bootstrapReturned) next) :
      Step snapshot ⟨.fresh,journal⟩ .bootstrapReturned ⟨.bootstrapped,next⟩
  | launch (localStep : CohortCommitJournal.Step journal (.arm (.sourceReply snapshot false)) next) :
      Step snapshot ⟨.bootstrapped,journal⟩ .launchReturned ⟨.launched,next⟩

theorem advance_complete (step : Step snapshot s event next) : advance snapshot s event = some next := by
  cases step <;> simp_all [advance,CohortCommitJournal.advance_exact]

theorem advance_sound (checked : advance snapshot s event = some next) : Step snapshot s event next := by
  rcases s with ⟨phase,journal⟩
  unfold advance at checked
  split at checked
  · cases checked
  · rename_i prior
    have prior : phase ≠ .launched := by simpa using prior
    cases event <;> simp only at checked
    case enter callId =>
      obtain ⟨next,localStep,equal⟩ := Option.map_eq_some_iff.mp checked
      cases equal
      exact .enter prior (CohortCommitJournal.advance_exact.mp localStep)
    case commit =>
      obtain ⟨next,localStep,equal⟩ := Option.map_eq_some_iff.mp checked
      cases equal
      exact .commit prior (CohortCommitJournal.advance_exact.mp localStep)
    case close =>
      obtain ⟨next,localStep,equal⟩ := Option.map_eq_some_iff.mp checked
      cases equal
      exact .close prior (CohortCommitJournal.advance_exact.mp localStep)
    case bootstrapReturned =>
      split at checked
      · rename_i phaseEq
        have phaseEq : phase = .fresh := by simpa using phaseEq
        subst phase
        obtain ⟨next,localStep,equal⟩ := Option.map_eq_some_iff.mp checked
        cases equal
        exact .bootstrap (CohortCommitJournal.advance_exact.mp localStep)
      · cases checked
    case launchReturned =>
      split at checked
      · rename_i phaseEq
        have phaseEq : phase = .bootstrapped := by simpa using phaseEq
        subst phase
        obtain ⟨next,localStep,equal⟩ := Option.map_eq_some_iff.mp checked
        cases equal
        exact .launch (CohortCommitJournal.advance_exact.mp localStep)
      · cases checked

theorem advance_exact : advance snapshot s event = some next ↔ Step snapshot s event next :=
  ⟨advance_sound,advance_complete⟩

def Invariant (snapshot : Option (SourcePublicationWorker.PrivateOutput p a)) (s : State p a) : Prop :=
  WellFormed s.journal ∧ s.journal.retired = false ∧ target s.journal = expected snapshot s.phase

theorem initial_valid (snapshot : Option (SourcePublicationWorker.PrivateOutput p a)) :
    Invariant snapshot (initial : State p a) := by
  simp [Invariant,initial,WellFormed,target,expected]

theorem preserves (valid : Invariant snapshot s) (step : Step snapshot s event next) :
    Invariant snapshot next := by
  cases step with
  | enter prior localStep | commit prior localStep | close prior localStep =>
    cases localStep <;> simpa [Invariant,target,WellFormed] using valid
  | bootstrap localStep =>
    cases localStep
    simp only [Invariant,target,List.foldl_nil,expected] at valid
    simp [Invariant,WellFormed,target,recipe,valid.2.2,expected]
  | launch localStep =>
    cases localStep
    simp only [Invariant,target,List.foldl_nil,expected] at valid
    simp [Invariant,WellFormed,target,recipe,valid.2.2,expected]

inductive Runs (snapshot : Option (SourcePublicationWorker.PrivateOutput p a)) : State p a → Prop where
  | nil : Runs snapshot initial
  | step : Runs snapshot s → Step snapshot s event next → Runs snapshot next

theorem Runs.valid (path : Runs snapshot s) : Invariant snapshot s := by
  induction path with
  | nil => exact initial_valid snapshot
  | step path step ih => exact preserves ih step

-- This anchor contains an OWED snapshot, not a manufactured completed write.
-- Its full source/writer projection starts at the SAME actual launch certificate.
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}

def anchor (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (callId : Nat) : CohortHostPrefix.State base :=
  ⟨.start base,⟨write empty .bootstrap,
    [.snapshot ((SharedWireLifetime.native base).driver.source.map SourcePublicationWorker.project)],
    some callId,true,false⟩⟩

theorem anchor_valid (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (callId : Nat) : CohortHostPrefix.Invariant (anchor base callId) := by
  refine ⟨SourceEntryPrefix.initial_valid,SharedHostCaptureReplay.initial_valid,?_,rfl⟩
  simp [anchor,WellFormed]

theorem launch_exact (path : Runs snapshot s) (step : Step snapshot s .launchReturned next) :
    ∃ callId, next = ⟨.launched,⟨write empty .bootstrap,[.snapshot snapshot],some callId,true,false⟩⟩ := by
  have valid := path.valid
  cases step with
  | launch localStep =>
    cases localStep
    simp only [Invariant,target,List.foldl_nil,expected] at valid
    simp [recipe,valid.2.2]

-- No second bootstrap or launch is licensed; stores after launch are checked
-- by the paired product, so there is one owner of each local commit.
theorem launch_barrier (s : State p a) (phase : s.phase = .launched) : advance snapshot s event = none := by
  simp [advance,phase]

theorem launched_shape (path : Runs snapshot s) (phase : s.phase = .launched) :
    ∃ callId, s = ⟨.launched,⟨write empty .bootstrap,[.snapshot snapshot],some callId,true,false⟩⟩ := by
  cases path with
  | nil => cases phase
  | step path step =>
    cases step with
    | enter prior | commit prior | close prior => exact False.elim (prior phase)
    | bootstrap => cases phase
    | launch localStep => exact launch_exact path (.launch localStep)

structure Certified (snapshot : Option (SourcePublicationWorker.PrivateOutput p a)) where
  state : State p a
  path : Runs snapshot state

def Certified.start (snapshot : Option (SourcePublicationWorker.PrivateOutput p a)) : Certified snapshot :=
  ⟨initial,.nil⟩

def Certified.advance (before : Certified snapshot) (event : Event) : Option (Certified snapshot) :=
  match checked : CohortHostStartup.advance snapshot before.state event with
  | none => none
  | some next => some ⟨next,.step before.path (advance_sound checked)⟩

-- The bridge uses the actual checked journal, not an independently replayed
-- expected state. All equalities here follow from its rooted startup path.
def Certified.bridge
    (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (checked : Certified ((SharedWireLifetime.native base).driver.source.map SourcePublicationWorker.project)) :
    Option (Σ callId, CohortHostPrefix.Certified (anchor base callId)) :=
  if phase : checked.state.phase = .launched then
    match call : checked.state.journal.call with
    | none => none
    | some callId =>
      some ⟨callId,⟨⟨.start base,checked.state.journal⟩,by
        obtain ⟨actual,equal⟩ := launched_shape checked.path phase
        have idEq : actual = callId := by simpa [equal] using call
        subst actual
        rw [equal]
        exact .nil⟩⟩
  else none

theorem Certified.bridge_complete
    (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (checked : Certified ((SharedWireLifetime.native base).driver.source.map SourcePublicationWorker.project))
    (phase : checked.state.phase = .launched) : (checked.bridge base).isSome = true := by
  obtain ⟨callId,equal⟩ := launched_shape checked.path phase
  have call : checked.state.journal.call = some callId := by rw [equal]
  simp only [Certified.bridge,dif_pos phase]
  split
  · rename_i absent
    rw [call] at absent
    cases absent
  · rfl

theorem Certified.bridge_memory
    (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (before : Certified ((SharedWireLifetime.native base).driver.source.map SourcePublicationWorker.project))
    (after : Σ callId, CohortHostPrefix.Certified (anchor base callId))
    (checked : before.bridge base = some after) :
    after.2.state.cohort = before.state.journal ∧ after.2.state.inner = .start base := by
  unfold Certified.bridge at checked
  split at checked
  · split at checked
    · cases checked
    · cases Option.some.inj checked
      exact ⟨rfl,rfl⟩
  · cases checked

theorem normal_startup (snapshot : Option (SourcePublicationWorker.PrivateOutput p a))
    (bootCall launchCall : Nat) :
    Runs snapshot ⟨.launched,⟨write empty .bootstrap,[.snapshot snapshot],some launchCall,true,false⟩⟩ := by
  have opened : Runs snapshot ⟨.fresh,⟨empty,[],some bootCall,true,false⟩⟩ :=
    .step .nil (.enter (by decide) .enter)
  have received : Runs snapshot ⟨.bootstrapped,⟨empty,[.bootstrap],some bootCall,true,false⟩⟩ := by
    simpa [recipe] using Runs.step opened (Step.bootstrap (.arm (receipt:=.bootstrapReturned)))
  have stored := Runs.step received (Step.commit (by decide) .commit)
  have closed := Runs.step stored (Step.close (by decide) .returnCall)
  have reopened := Runs.step closed (Step.enter (callId:=launchCall) (by decide) .enter)
  simpa [recipe] using Runs.step reopened (Step.launch (.arm (receipt:=.sourceReply snapshot false)))

theorem unpaid_bootstrap_cannot_launch (snapshot : Option (SourcePublicationWorker.PrivateOutput p a))
    (callId : Nat) :
    advance snapshot ⟨.bootstrapped,⟨empty,[.bootstrap],some callId,true,false⟩⟩ .launchReturned = none := by
  simp [advance,CohortCommitJournal.advance]

theorem unpaid_bootstrap_cannot_return (snapshot : Option (SourcePublicationWorker.PrivateOutput p a))
    (callId : Nat) :
    advance snapshot ⟨.bootstrapped,⟨empty,[.bootstrap],some callId,true,false⟩⟩ .close = none := by
  simp [advance,CohortCommitJournal.advance]

theorem bootstrap_once (snapshot : Option (SourcePublicationWorker.PrivateOutput p a))
    (journal : CohortCommitJournal.State p a) :
    advance snapshot ⟨.bootstrapped,journal⟩ .bootstrapReturned = none := by
  simp [advance]

#print axioms advance_exact
#print axioms initial_valid
#print axioms preserves
#print axioms Runs.valid
#print axioms anchor_valid
#print axioms launch_exact
#print axioms launch_barrier
#print axioms launched_shape
#print axioms Certified.bridge
#print axioms Certified.bridge_complete
#print axioms Certified.bridge_memory
#print axioms normal_startup
#print axioms unpaid_bootstrap_cannot_launch
#print axioms unpaid_bootstrap_cannot_return
#print axioms bootstrap_once
end CohortHostStartup
