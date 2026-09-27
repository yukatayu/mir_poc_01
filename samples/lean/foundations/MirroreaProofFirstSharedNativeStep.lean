import MirroreaProofFirstCohortFundingReplay
namespace MirroreaProofFirst.SharedNativeStep

-- Exact common native data for joining funding and Joint/work certificates.
-- This projection does not erase data to public snapshots. Host mirrors,
-- owner transport counters, byte custody and unknown IO are separate fields
-- of the unfinished outer lifetime construction, not facts proved here.
structure State (p a : Nat) where
  driver : PublicationCapacityDriver.State p a
  owners : Fin p → OwnerEndpointBudget.State p a
  ordinal : Nat

theorem ext {s t : State p a} (driver : s.driver=t.driver)
    (owners : s.owners=t.owners) (ordinal : s.ordinal=t.ordinal) : s=t := by
  cases s; cases t; cases driver; cases owners; cases ordinal; rfl

def fromFunding (s : CohortPhase.Live p a) : State p a := ⟨s.driver,s.owners,s.ordinal⟩
def fromWork (s : WorkOccurrence.State p a) : State p a := ⟨s.driver,s.joint.actual.owners,s.ordinal⟩

-- Calculation is total; event alignment is an independent admission premise.
-- No native outcome is inferred from an unresolved transport request.
def execute (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat)
    (s : State p a) : WorkOccurrence.Event p a → State p a
  | .source _ request => {s with driver := (SourceFundingInput.execute assigned scope bootstrap s.driver request).1, ordinal := s.ordinal+1}
  | .query _ _ => {s with ordinal := s.ordinal+1}
  | .owner target request => {s with owners := (PublicOwnerBoundary.put s.owners target
      (OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.owners target) request).1)}

def Aligned (s : State p a) : WorkOccurrence.Event p a → Prop
  | .source ordinal _ | .query ordinal _ => ordinal=s.ordinal
  | .owner _ _ => True

inductive Runs (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat) (base : State p a) :
    List (WorkOccurrence.Event p a) → State p a → Prop where
  | nil : Runs assigned scope bootstrap capacity base [] base
  | step : Runs assigned scope bootstrap capacity base events s → Aligned s event →
      Runs assigned scope bootstrap capacity base (events ++ [event]) (execute assigned scope bootstrap capacity s event)

theorem runs_exact
    (path : Runs assigned scope bootstrap capacity base events s) :
    s = events.foldl (execute assigned scope bootstrap capacity) base := by
  induction path with
  | nil => rfl
  | step prior aligned ih => simp [List.foldl_append,ih]

theorem runs_deterministic
    (left : Runs assigned scope bootstrap capacity base events s)
    (right : Runs assigned scope bootstrap capacity base events t) : s=t :=
  (runs_exact left).trans (runs_exact right).symm

theorem funding_source
    (valid : PublicationLifecycle.Invariant assigned scope bootstrap s.driver)
    (step : CohortPhase.Step assigned scope bootstrap capacity (some s) (.source request) (some next)) :
    fromFunding next = execute assigned scope bootstrap capacity (fromFunding s) (.source s.ordinal request) := by
  obtain ⟨driver,owners,ordinal⟩ := CohortFundingReplay.source_physics valid step
  exact ext driver owners ordinal

theorem funding_owner
    (step : CohortPhase.Step assigned scope bootstrap capacity (some s) (.owner target request) (some next)) :
    fromFunding next = execute assigned scope bootstrap capacity (fromFunding s) (.owner target request) := by
  obtain ⟨driver,owners,ordinal⟩ := CohortFundingReplay.owner_physics step
  exact ext driver owners ordinal

theorem funding_query
    (step : CohortPhase.Step assigned scope bootstrap capacity (some s) (.query request) (some next)) :
    fromFunding next = execute assigned scope bootstrap capacity (fromFunding s) (.query s.ordinal request) := by
  cases step
  rfl

theorem work_source
    (ran : SourceFundingInput.execute assigned scope bootstrap s.driver (vector,.step command) = (next,.accepted)) :
    fromWork (WorkOccurrence.onSource s next after) =
      execute assigned scope bootstrap capacity (fromWork s) (.source s.ordinal (vector,.step command)) := by
  apply ext
  · simp [fromWork,execute,WorkOccurrence.onSource,ran]
  · rfl
  · rfl

theorem work_owner
    (ran : OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.joint.actual.owners target) request = (next,reply)) :
    fromWork (WorkOccurrence.onOwner s target request next reply) =
      execute assigned scope bootstrap capacity (fromWork s) (.owner target request) := by
  apply ext
  · rfl
  · simp only [fromWork,WorkOccurrence.onOwner,SourceRegistration.ownerResult,SourceOwnerFloor.ownerResult,execute,ran]
    rfl
  · rfl

theorem work_trace
    (trace : WorkOccurrence.Trace assigned scope bootstrap capacity base target ticket events phase s) :
    Runs assigned scope bootstrap capacity (fromWork base) events (fromWork s) := by
  induction trace with
  | start => exact .nil
  | enter clear bound ran present dispatched =>
    rw [work_source ran]
    exact .step .nil rfl
  | query prior ih =>
    exact .step ih rfl
  | reserve prior ran ih =>
    rw [work_owner ran]
    exact .step ih trivial
  | probe prior ran ih =>
    rw [work_owner ran]
    exact .step ih trivial
  | compute prior ran ticket scopeAt revision ih =>
    rw [work_owner ran]
    exact .step ih trivial
  | finish prior bound ran present ih =>
    rw [work_source ran]
    exact .step ih rfl

-- Any accepted extension of this certified work prefix computes from its
-- actual previous full state. It cannot silently switch to an equal snapshot.
theorem work_extension
    (prior : WorkOccurrence.Trace assigned scope bootstrap capacity base target ticket events phase s)
    (next : WorkOccurrence.Trace assigned scope bootstrap capacity base target ticket (events ++ [event]) nextPhase t) :
    fromWork t = execute assigned scope bootstrap capacity (fromWork s) event := by
  have before := runs_exact (work_trace prior)
  have after := runs_exact (work_trace next)
  simpa [List.foldl_append,←before] using after

theorem stale_source_excluded (wrong : ordinal ≠ s.ordinal) :
    ¬ Aligned s (.source ordinal request) := wrong

section ClosedDriver
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope budget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {s next : JointDriver.Closed assigned scope bootstrap capacity budget seed}

theorem closed_query (request : Sum (SourceFundingQuery.HeadRequest p) (Fin p)) :
    fromWork (JointDriver.query s).actual =
      execute assigned scope bootstrap capacity (fromWork s.actual) (.query s.actual.ordinal request) := rfl

theorem closed_refused (checked : JointDriver.refused s vector input = some next) :
    fromWork next.actual =
      execute assigned scope bootstrap capacity (fromWork s.actual) (.source s.actual.ordinal (vector,input)) := by
  unfold JointDriver.refused at checked
  split at checked
  · dsimp only at checked
    split at checked
    · cases Option.some.inj checked
      rfl
    · cases checked
  · cases checked

theorem closed_source (checked : JointDriver.source s vector command = some next) :
    fromWork next.actual =
      execute assigned scope bootstrap capacity (fromWork s.actual) (.source s.actual.ordinal (vector,.step command)) := by
  unfold JointDriver.source at checked
  split at checked
  · dsimp only at checked
    split at checked
    · split at checked
      · rename_i bound accepted absent
        have ran : SourceFundingInput.execute assigned scope bootstrap s.actual.driver (vector,.step command) =
            ((SourceFundingInput.execute assigned scope bootstrap s.actual.driver (vector,.step command)).1,.accepted) := by rw [←accepted]
        obtain ⟨after,present,_⟩ := WorkOccurrence.accepted_semantic s.valid s.coupled ran
        rw [present] at absent
        cases absent
      · split at checked
        · split at checked
          · split at checked
            · split at checked
              · cases Option.some.inj checked
                rfl
              · cases checked
            · cases checked
          · cases checked
        · split at checked
          · rename_i matching
            have equal := WorkOccurrenceReplay.same_exact.mp matching
            subst command
            cases Option.some.inj checked
            rfl
          · cases checked
    · cases Option.some.inj checked
      rfl
  · cases checked

theorem closed_owner (checked : JointDriver.owner s target request headPayment = some next) :
    fromWork next.actual =
      execute assigned scope bootstrap capacity (fromWork s.actual) (.owner target request) := by
  unfold JointDriver.owner at checked
  split at checked
  · cases request with
    | owner command =>
      simp only at checked
      split at checked
      · split at checked
        · cases Option.some.inj checked
          rfl
        · cases checked
      · cases checked
    | freeze revision =>
      simp only at checked
      split at checked
      · split at checked
        · cases Option.some.inj checked
          rfl
        · split at checked
          · cases Option.some.inj checked
            rfl
          · cases checked
      · cases Option.some.inj checked
        rfl
  · cases checked

#print axioms closed_query
#print axioms closed_refused
#print axioms closed_source
#print axioms closed_owner
end ClosedDriver

#print axioms funding_source
#print axioms funding_owner
#print axioms funding_query
#print axioms work_trace
#print axioms work_extension
#print axioms runs_deterministic
end MirroreaProofFirst.SharedNativeStep
