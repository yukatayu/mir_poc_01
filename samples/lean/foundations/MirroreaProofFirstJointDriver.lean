import MirroreaProofFirstWorkOccurrenceReplay
namespace MirroreaProofFirst.JointDriver
open WorkOccurrence

-- One retained complete driver and Joint history. No fresh replacement with
-- an equal public snapshot is accepted. This remains a private replay object;
-- native bytes, lifetime/pipe custody and host commit correspondence are TCB.
structure Closed (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat)
    (budget : Nat) (seed : QualifiedCustody.State p a) where
  actual : State p a
  valid : PublicationLifecycle.Invariant assigned scope bootstrap actual.driver
  history : PublicJointHistory.Runs (fun i => ⟨assigned.realm,i⟩) scope capacity budget seed actual.joint
  coupled : actual.driver.source = some actual.joint.actual.source

theorem accepted_launch
    (launched : SourceInput.launch assigned bootstrap program = some seed)
    (ran : SourceFundingInput.execute assigned scope bootstrap (PublicationCapacityDriver.initial sourceBudget)
      (vector,.launch program) = (next,.accepted)) :
    next.source = some (PublicationInput.initial seed) := by
  have valid := PublicationLifecycle.initial_invariant assigned scope bootstrap sourceBudget
  have full : PublicationLifecycle.transition assigned scope bootstrap (PublicationCapacityDriver.initial sourceBudget)
      (.launch program) = (next,.accepted) := by
    rw [← PublicationLifecycle.transitionFast_exact valid]
    exact (SourceFundingInput.accepted_exact.mp ran).1
  have erased := (PublicationLifecycle.accepted_refines assigned scope bootstrap (PublicationCapacityDriver.initial sourceBudget)
    (.launch program) (by rw [full])).1
  rw [full] at erased
  simpa [PublicationCapacityDriver.initial,PublicationInput.transition,launched] using (congrArg Prod.fst erased).symm

-- Host pre-send provenance is independent of the native success/refusal.
-- Retention timing is supplied by the caller's consumed host/work events.
def SourceAdmissible (s : SourceRegistration.State p a)
    (retained : List OwnerReceipt.Envelope) : PublicationInput.Command p a → Prop
  | .freeze i revision | .acknowledge i revision => revision ∈ s.actual.records i
  | .install i revision => revision ∈ s.installs i
  | .arrival envelope => envelope ∈ retained
  | _ => True

def sourcePreSend (s : SourceRegistration.State p a)
    (retained : List OwnerReceipt.Envelope) : PublicationInput.Command p a → Bool
  | .freeze i revision | .acknowledge i revision => (s.actual.records i).contains revision
  | .install i revision => (s.installs i).contains revision
  | .arrival envelope => retained.contains envelope
  | _ => true

theorem source_presend_exact : sourcePreSend s retained command = true ↔ SourceAdmissible s retained command := by
  cases command <;> simp [sourcePreSend,SourceAdmissible]

#print axioms source_presend_exact

def fromLaunch {p a : Nat} {assigned : SourceInput.Assignment p a} {scope : Nat}
    {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
    {program : QualifiedSession.Program p} {sourceBudget budget : Nat}
    {vector : Vector (Fin 513) p} {next : PublicationCapacityDriver.State p a}
    (launched : SourceInput.launch assigned bootstrap program = some seed)
    (ran : SourceFundingInput.execute assigned scope bootstrap (PublicationCapacityDriver.initial sourceBudget)
      (vector,.launch program) = (next,.accepted)) : Closed assigned scope bootstrap capacity budget seed :=
  ⟨⟨next,SourceRegistration.initial seed budget,2⟩,
    by
      have preserved := SourceFundingInput.preserves (PublicationLifecycle.initial_invariant assigned scope bootstrap sourceBudget)
        (value:=(vector,.launch program))
      simpa [ran] using preserved,
    .fresh,accepted_launch launched ran⟩

-- Ordinal2 after bootstrap(no reply) at1 and launch at2. Queries thereafter
-- increment the actual ordinal although they leave the complete driver intact.
def query (s : Closed assigned scope bootstrap capacity budget seed) : Closed assigned scope bootstrap capacity budget seed :=
  {s with actual.ordinal := s.actual.ordinal+1}

def source {p a : Nat} {assigned : SourceInput.Assignment p a} {scope budget : Nat}
    {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
    (s : Closed assigned scope bootstrap capacity budget seed) (vector : Vector (Fin 513) p)
    (command : PublicationInput.Command p a) : Option (Closed assigned scope bootstrap capacity budget seed) := by
  if bound : vectorAt s.actual vector then
    let result := SourceFundingInput.execute assigned scope bootstrap s.actual.driver (vector,.step command)
    if accepted : result.2 = .accepted then
      have ran : SourceFundingInput.execute assigned scope bootstrap s.actual.driver (vector,.step command) = (result.1,.accepted) := by rw [←accepted]
      have preserved : PublicationLifecycle.Invariant assigned scope bootstrap result.1 := SourceFundingInput.preserves s.valid
      have semanticExists := accepted_semantic s.valid s.coupled ran
      -- The existential semantic successor cannot be eliminated into runtime
      -- data. Inspect the same deterministic result, then identify its value.
      match atSource : result.1.source with
      | none => exact False.elim (by obtain ⟨after,present,_⟩ := semanticExists; rw [present] at atSource; cases atSource)
      | some actualSource =>
        have semantic : PublicationInput.execute scope s.actual.joint.actual.source command = some actualSource := by
          obtain ⟨after,present,semantic⟩ := semanticExists
          have equal := Option.some.inj (present.symm.trans atSource)
          subst after
          exact semantic
        match pending : s.actual.joint.actual.pending with
        | none =>
          if allowedPublic : PublicJointReplay.publicSourceCheck command = true then
           if floor : PublicJointReplay.floorCheck s.actual.joint command = true then
            if registered : SourceRegistration.sourceCheck s.actual.joint command = true then
              exact some ⟨onSource s.actual result.1 actualSource,preserved,
                .step s.history (.source pending (PublicJointReplay.public_source_exact.mp allowedPublic)
                  (PublicJointReplay.floor_exact.mp floor) (SourceRegistration.source_check_exact.mp registered) semantic),atSource⟩
            else exact none
           else exact none
          else exact none
        | some (i,revision) =>
          if matching : WorkOccurrenceReplay.same (PublicationInput.command p a) command (.freeze i revision) = true then
            have equal := WorkOccurrenceReplay.same_exact.mp matching
            subst command
            exact some ⟨⟨result.1,{s.actual.joint with actual.source := actualSource,actual.pending := none},s.actual.ordinal+1⟩,
              preserved,.step s.history (.notify pending semantic),atSource⟩
          else exact none
    else
      have unchanged := SourceFundingWork.known_refusal_frames s.valid accepted
      exact some ⟨⟨result.1,s.actual.joint,s.actual.ordinal+1⟩,
        SourceFundingInput.preserves s.valid,s.history,(congrArg PublicationCapacityDriver.State.source unchanged).trans s.coupled⟩
  else exact none

def owner {p a : Nat} {assigned : SourceInput.Assignment p a} {scope budget : Nat}
    {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
    (s : Closed assigned scope bootstrap capacity budget seed) (i : Fin p)
    (command : OwnerEndpoint.Command p a) (headPayment : Bool) :
    Option (Closed assigned scope bootstrap capacity budget seed) := by
  if clear : s.actual.joint.actual.pending = none then
    match command with
    | .owner command =>
      let result := OwnerEndpointBudget.transition ⟨assigned.realm,i⟩ scope (capacity i) (s.actual.joint.actual.owners i) (.owner command)
      if admin : PublicOwnerReplay.adminCheck (.owner command) = true then
       if image : SourceOwnerImage.imageCheck s.actual.joint.actual.source command = true then
        exact some ⟨onOwner s.actual i (.owner command) result.1 result.2,s.valid,
          .step s.history (.owner clear (PublicOwnerReplay.admin_exact.mp admin) (SourceOwnerImage.image_check_exact.mp image) rfl),s.coupled⟩
       else exact none
      else exact none
    | .freeze revision =>
      let result := OwnerEndpointBudget.transition ⟨assigned.realm,i⟩ scope (capacity i) (s.actual.joint.actual.owners i) (.freeze revision)
      if paid : result.2 = .inl 12 then
        have ran : OwnerEndpointBudget.transition ⟨assigned.realm,i⟩ scope (capacity i) (s.actual.joint.actual.owners i) (.freeze revision) = (result.1,.inl 12) := by rw [←paid]
        if headPayment then
          exact some ⟨⟨s.actual.driver,{SourceRegistration.ownerResult s.actual.joint i (.freeze revision) result.1 (.inl 12) with actual.pending := some (i,revision)},s.actual.ordinal⟩,
            s.valid,.step s.history (.paidFreeze clear ran),s.coupled⟩
        else
          if bound : revision ≤ s.actual.joint.actual.source.publication.barrier.published then
            exact some ⟨onOwner s.actual i (.freeze revision) result.1 (.inl 12),s.valid,
              .step s.history (.extraFreeze clear bound ran),s.coupled⟩
          else exact none
      else
        exact some ⟨onOwner s.actual i (.freeze revision) result.1 result.2,s.valid,
          .step s.history (.failedFreeze clear paid rfl),s.coupled⟩
  else exact none

-- Includes a repeated launch on an existing source. A known native refusal
-- frames the entire retained driver; an unknown outcome is never this branch.
def refused {p a : Nat} {assigned : SourceInput.Assignment p a} {scope budget : Nat}
    {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
    (s : Closed assigned scope bootstrap capacity budget seed) (vector : Vector (Fin 513) p)
    (input : PublicationInput.Input p a) : Option (Closed assigned scope bootstrap capacity budget seed) := by
  if bound : vectorAt s.actual vector then
    let result := SourceFundingInput.execute assigned scope bootstrap s.actual.driver (vector,input)
    if rejected : result.2 ≠ .accepted then
      have unchanged := SourceFundingWork.known_refusal_frames s.valid rejected
      exact some ⟨⟨result.1,s.actual.joint,s.actual.ordinal+1⟩,
        SourceFundingInput.preserves s.valid,s.history,
        (congrArg PublicationCapacityDriver.State.source unchanged).trans s.coupled⟩
    else exact none
  else exact none

#print axioms refused

-- An enabled semantic finish cannot be passed here. The caller must supply
-- a prefix certificate whose event list includes the actual funded finish.
def closeWork {p a : Nat} {assigned : SourceInput.Assignment p a} {scope budget : Nat}
    {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
    (s : Closed assigned scope bootstrap capacity budget seed)
    {target : Fin p} {ticket : OwnerOccurrence.Ticket} {events : List (Event p a)}
    {envelope : OwnerReceipt.Envelope} {after : State p a}
    (trace : Trace assigned scope bootstrap capacity s.actual target ticket events (.finished envelope) after) :
    Closed assigned scope bootstrap capacity budget seed :=
  ⟨after,driver_invariant s.valid trace,finished_history s.history s.valid s.coupled trace,source_coupled s.coupled trace⟩

#print axioms accepted_launch
#print axioms fromLaunch
#print axioms source
#print axioms owner
#print axioms closeWork
end MirroreaProofFirst.JointDriver
