import MirroreaProofFirstSharedJointDriver
namespace MirroreaProofFirst.SharedFundedDriver

-- One full current native state, retained inside Joint/work. Funding retains
-- only its phase; its rooted certificate is indexed by that SAME native state.
-- This does not yet model transport or per-occurrence host commit journals.
def ofNative (s : SharedNativeStep.State p a) (mode : CohortPhase.Mode p) : CohortPhase.Live p a :=
  ⟨s.driver,s.owners,s.ordinal,mode⟩

@[simp] theorem of_projection (s : CohortPhase.Live p a) :
    ofNative (SharedNativeStep.fromFunding s) s.mode = s := by cases s; rfl

@[simp] theorem project_of (s : SharedNativeStep.State p a) (mode : CohortPhase.Mode p) :
    SharedNativeStep.fromFunding (ofNative s mode) = s := by cases s; rfl

structure State (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat)
    (sourceBudget ownerBudget : Nat) (seed : QualifiedCustody.State p a) where
  joint : SharedJointDriver.Running assigned scope bootstrap capacity ownerBudget seed
  mode : CohortPhase.Mode p
  events : List (CohortPhase.Event p a)
  path : CohortPhysicalOrdinal.Runs assigned scope bootstrap capacity sourceBudget ownerBudget events
    (some (ofNative (SharedJointDriver.native joint) mode))

variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}

def funding (s : State assigned scope bootstrap capacity sourceBudget ownerBudget seed) : CohortPhase.Live p a :=
  ofNative (SharedJointDriver.native s.joint) s.mode

def fundingEvent : WorkOccurrence.Event p a → CohortPhase.Event p a
  | .source _ request => .source request
  | .query _ request => .query request
  | .owner target request => .owner target request

theorem invariant (s : State assigned scope bootstrap capacity sourceBudget ownerBudget seed) :
    CohortPhase.Invariant assigned scope bootstrap (some (funding s)) :=
  CohortPhysicalOrdinal.runs_invariant s.path

structure Next (s : State assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (event : CohortPhase.Event p a) where
  val : State assigned scope bootstrap capacity sourceBudget ownerBudget seed
  appended : val.events = s.events ++ [event]
  step : CohortPhase.Step assigned scope bootstrap capacity (some (funding s)) event (some (funding val))

-- The equation is derived from independently certified native computations;
-- no equality of desired invariants, snapshots or outcome flags is checked.
private def merge (s : State assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    {event : WorkOccurrence.Event p a} (joint : SharedJointDriver.Next s.joint event)
    (next : CohortPhase.Live p a)
    (step : CohortPhase.Step assigned scope bootstrap capacity (some (funding s)) (fundingEvent event) (some next))
    (physical : SharedNativeStep.fromFunding next =
      SharedNativeStep.execute assigned scope bootstrap capacity (SharedJointDriver.native s.joint) event) :
    Next s (fundingEvent event) := by
  have same := physical.trans joint.physics.symm
  have whole : ofNative (SharedJointDriver.native joint.val) next.mode = next := by
    rw [←same]; exact of_projection next
  let combined : State assigned scope bootstrap capacity sourceBudget ownerBudget seed :=
    ⟨joint.val,next.mode,s.events ++ [fundingEvent event],by
      rw [whole]; exact .step s.path step⟩
  exact ⟨combined,rfl,by simpa only [funding,combined,whole] using step⟩

def source (s : State assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (input : SourceFundingQuery.CheckedInput p a) :
    Option (Next s (fundingEvent (SharedJointDriver.inputEvent (SharedJointDriver.actual s.joint).ordinal input))) := by
  match SharedJointDriver.source s.joint input with
  | none => exact none
  | some joint =>
    cases input with
    | inl request =>
      match CohortFundingReplay.query (assigned:=assigned) (scope:=scope) (bootstrap:=bootstrap) (capacity:=capacity) (funding s) (.inl request) with
      | none => exact none
      | some next => exact some (merge s joint next.val next.property (by
          simpa [funding] using SharedNativeStep.funding_query next.property))
    | inr input =>
      cases input with
      | inl request =>
        match CohortFundingReplay.query (assigned:=assigned) (scope:=scope) (bootstrap:=bootstrap) (capacity:=capacity) (funding s) (.inr request) with
        | none => exact none
        | some next => exact some (merge s joint next.val next.property (by
            simpa [funding] using SharedNativeStep.funding_query next.property))
      | inr request =>
        match CohortFundingReplay.source (assigned:=assigned) (scope:=scope) (bootstrap:=bootstrap) (capacity:=capacity) (funding s) request with
        | none => exact none
        | some next => exact some (merge s joint next.val next.step (by
            simpa [funding] using SharedNativeStep.funding_source ((invariant s) _ rfl).1 next.step))

def owner (s : State assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (target : Fin p) (request : OwnerEndpoint.Command p a) (headPayment : Bool) :
    Option (Next s (.owner target request)) := by
  match SharedJointDriver.owner s.joint target request headPayment with
  | none => exact none
  | some joint =>
    match CohortFundingReplay.owner (assigned:=assigned) (scope:=scope) (bootstrap:=bootstrap) (capacity:=capacity) (funding s) target request with
    | none => exact none
    | some next => exact some (merge s joint next.val next.step (by
        simpa [funding] using SharedNativeStep.funding_owner next.step))

-- Relative completeness of composition: given a recognized Joint/work
-- transition and an INDEPENDENT declarative funding step on this same state,
-- their composition cannot reject. This does not claim completeness of the
-- older Joint admission policy or of unmodeled host/transport actions.
theorem source_complete {s : State assigned scope bootstrap capacity sourceBudget ownerBudget seed}
    {input : SourceFundingQuery.CheckedInput p a} {next : CohortPhase.Live p a}
    {joint : SharedJointDriver.Next s.joint (SharedJointDriver.inputEvent (SharedJointDriver.actual s.joint).ordinal input)}
    (recognized : SharedJointDriver.source s.joint input = some joint)
    (step : CohortPhase.Step assigned scope bootstrap capacity (some (funding s))
      (fundingEvent (SharedJointDriver.inputEvent (SharedJointDriver.actual s.joint).ordinal input)) (some next)) :
    (source s input).isSome = true := by
  cases input with
  | inl request =>
    simp only [source,recognized]
    have complete := CohortFundingReplay.query_complete step
    cases checked : CohortFundingReplay.query (assigned:=assigned) (scope:=scope) (bootstrap:=bootstrap) (capacity:=capacity) (funding s) (.inl request) with
    | none => simp [checked] at complete
    | some next => rfl
  | inr input =>
    cases input with
    | inl request =>
      simp only [source,recognized]
      have complete := CohortFundingReplay.query_complete step
      cases checked : CohortFundingReplay.query (assigned:=assigned) (scope:=scope) (bootstrap:=bootstrap) (capacity:=capacity) (funding s) (.inr request) with
      | none => simp [checked] at complete
      | some next => rfl
    | inr request =>
      simp only [source,recognized]
      have complete := CohortFundingReplay.source_complete step
      cases checked : CohortFundingReplay.source (assigned:=assigned) (scope:=scope) (bootstrap:=bootstrap) (capacity:=capacity) (funding s) request with
      | none => simp [checked] at complete
      | some next => rfl

theorem owner_complete {s : State assigned scope bootstrap capacity sourceBudget ownerBudget seed}
    {target : Fin p} {request : OwnerEndpoint.Command p a} {headPayment : Bool} {next : CohortPhase.Live p a}
    {joint : SharedJointDriver.Next s.joint (.owner target request)}
    (recognized : SharedJointDriver.owner s.joint target request headPayment = some joint)
    (step : CohortPhase.Step assigned scope bootstrap capacity (some (funding s)) (.owner target request) (some next)) :
    (owner s target request headPayment).isSome = true := by
  simp only [owner,recognized]
  have complete := CohortFundingReplay.owner_complete step
  cases checked : CohortFundingReplay.owner (assigned:=assigned) (scope:=scope) (bootstrap:=bootstrap) (capacity:=capacity) (funding s) target request with
  | none => simp [checked] at complete
  | some next => rfl

#print axioms source_complete
#print axioms owner_complete

-- Close a finished work interval and, when enabled, finish the initialization
-- prelude. Both are local metadata changes; full native data stays unchanged.
theorem local_frame {before after : CohortPhase.Live p a}
    (step : CohortPhase.Step assigned scope bootstrap capacity (some before) .local (some after)) :
    SharedNativeStep.fromFunding after = SharedNativeStep.fromFunding before := by
  cases step <;> rfl

def finish (s : State assigned scope bootstrap capacity sourceBudget ownerBudget seed) :
    Option (State assigned scope bootstrap capacity sourceBudget ownerBudget seed) := by
  match SharedJointDriver.finish s.joint with
  | none => exact none
  | some joint =>
    match CohortFundingReplay.preludeDone (assigned:=assigned) (scope:=scope) (bootstrap:=bootstrap) (capacity:=capacity) (funding s) with
    | none => exact some ⟨joint.val,s.mode,s.events,by simpa only [joint.property] using s.path⟩
    | some next =>
      have frame : SharedNativeStep.fromFunding next.val = SharedJointDriver.native s.joint := by
        simpa [funding] using local_frame next.property
      exact some ⟨joint.val,next.val.mode,s.events ++ [.local],by
        have whole : ofNative (SharedJointDriver.native joint.val) next.val.mode = next.val := by
          rw [joint.property,←frame]; exact of_projection next.val
        rw [whole]
        exact .step s.path next.property⟩

-- Only the existing local completion checker may change certificate metadata.
-- Its entire native projection is framed; equal snapshots do not authorize
-- substituting an unrelated certificate or discarding phase/history.
theorem finish_native
    (checked : finish s = some next) :
    SharedJointDriver.native next.joint = SharedJointDriver.native s.joint := by
  unfold finish at checked
  split at checked
  · cases checked
  · rename_i joint selected
    split at checked
    · cases Option.some.inj checked
      exact joint.property
    · cases Option.some.inj checked
      exact joint.property

-- Actual startup consumes the exact launch at2, with fresh owner budgets
-- bound to the supplied vector. The owner budget is not erased by ordinal2.
def fromLaunch {program : QualifiedSession.Program p} {vector : Vector (Fin 513) p}
    {next : PublicationCapacityDriver.State p a}
    (bound : ∀ i : Fin p, ownerBudget = (vector[i.val]).val)
    (launched : SourceInput.launch assigned bootstrap program = some seed)
    (ran : SourceFundingInput.execute assigned scope bootstrap (PublicationCapacityDriver.initial sourceBudget)
      (vector,.launch program) = (next,.accepted)) :
    State assigned scope bootstrap capacity sourceBudget ownerBudget seed :=
  ⟨.closed (JointDriver.fromLaunch launched ran),.prelude (fun _ => true),[.source (vector,.launch program)],by
    exact .step .postBootstrap (.launch rfl rfl bound ran)⟩

#print axioms invariant
#print axioms source
#print axioms owner
#print axioms finish_native
#print axioms finish
#print axioms fromLaunch
end MirroreaProofFirst.SharedFundedDriver
