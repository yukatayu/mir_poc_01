import MirroreaProofFirstOwnerInstallationHistory

namespace MirroreaProofFirst.SourceRegistration

def actionInstalled (old : Fin p → Nat) :
    PublicationUse.Action p (QualifiedPublication.Command p a) → Fin p → Nat
  | .administrative (.install endpoint revision) => Publication.put old endpoint revision
  | _ => old

def commandInstalled (old : Fin p → Nat) : PublicationInput.Command p a → Fin p → Nat
  | .install endpoint revision => Publication.put old endpoint revision
  | _ => old

theorem image_installed
    (ran : PublicationImage.execute CustodyPublication.image QualifiedCustody.evaluate source action = some next) :
    next.barrier.installed = actionInstalled source.barrier.installed action := by
  unfold PublicationImage.execute at ran
  split at ran
  · cases ran
    cases action with
    | enter endpoint => rfl
    | finish endpoint => rfl
    | administrative action => cases action <;> rfl
  · cases ran

theorem selected_installed
    (selected : PublicationInput.select source command = some action) :
    actionInstalled source.publication.barrier.installed action = commandInstalled source.publication.barrier.installed command := by
  cases command with
  | stage input => cases input <;> cases selected <;> rfl
  | freeze endpoint revision => cases selected; rfl
  | acknowledge endpoint revision => cases selected; rfl
  | publish => cases selected; rfl
  | install endpoint revision => cases selected; rfl
  | finish endpoint => cases selected; rfl
  | enter endpoint =>
    simp only [PublicationInput.select] at selected
    split at selected
    · cases selected
    · cases waiting : source.publication.current.privateState.session.state.source.waiting with
      | none => simp [waiting] at selected
      | some saved =>
        simp only [waiting,Option.bind_eq_bind,Option.bind_some] at selected
        split at selected <;> cases selected <;> rfl
  | arrival envelope =>
    cases dispatch : source.dispatch with
    | none => simp [PublicationInput.select,dispatch] at selected
    | some dispatched =>
      by_cases ticket : envelope.ticket = dispatched.ticket
      · cases accepted : QualifiedReceipt.accept dispatched.context source.publication.current envelope with
        | none => simp [PublicationInput.select,dispatch,ticket,accepted] at selected
        | some privateState =>
          cases result : envelope.result with
          | rejected reason => simp [PublicationInput.select,dispatch,ticket,result] at selected
          | value value =>
            have equal : action = .administrative (.stage (.receive envelope.ticket value)) := by
              simpa [PublicationInput.select,dispatch,ticket,accepted,result] using selected.symm
            rw [equal]; rfl
      · simp [PublicationInput.select,dispatch,ticket] at selected

theorem source_installed (ran : PublicationInput.execute scopeId source command = some next) :
    next.publication.barrier.installed = commandInstalled source.publication.barrier.installed command := by
  obtain ⟨action,selected,executed,_⟩ := PublicationInput.executed_action ran
  exact (image_installed executed).trans (selected_installed selected)


#print axioms image_installed
#print axioms selected_installed
#print axioms source_installed
-- Absent workers have no installation facts and a zero numerical lower bound.
-- This is not an assertion that all workers already exist at fresh launch.
def actualRevision (state : OwnerEndpointBudget.State p a) : Nat :=
  match state.owner with
  | none => 0
  | some owner => owner.owner.core.revision

theorem revision_mono
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply)) :
    actualRevision state ≤ actualRevision next := by
  cases present : state.owner with
  | none => simp [actualRevision,present]
  | some owner =>
    obtain ⟨after,retained,monotone⟩ := OwnerRevisionHistory.budget_mono present ran
    simpa [actualRevision,present,retained] using monotone

structure State (p a : Nat) where
  actual : SourceOwnerFloor.State p a
  installs : Fin p → List Nat

def initial (seed : QualifiedCustody.State p a) (budget : Nat) : State p a :=
  ⟨SourceOwnerFloor.initial seed budget, fun _ => []⟩

def ownerResult (s : State p a) (i : Fin p) (command : OwnerEndpoint.Command p a)
    (next : OwnerEndpointBudget.State p a) (reply : Sum Nat OwnerReceipt.Envelope) : State p a :=
  ⟨SourceOwnerFloor.ownerResult s.actual i command next reply,
    PublicationPayload.put s.installs i (OwnerInstallationHistory.observe (s.installs i) command reply)⟩

def sourceAllowed (s : State p a) : PublicationInput.Command p a → Prop
  | .install i revision => revision ∈ s.installs i
  | _ => True

def sourceCheck (s : State p a) : PublicationInput.Command p a → Bool
  | .install i revision => (s.installs i).contains revision
  | _ => true

theorem source_check_exact : sourceCheck s command = true ↔ sourceAllowed s command := by
  cases command <;> simp [sourceCheck,sourceAllowed]

-- The same joint history projects to the earlier image/floor relation. It adds
-- only the host's actual retained-install-fact guard; it does not assume the
-- registration/currentness conclusion, idle state, free room or funding.
inductive Runs (assigned : Fin p → OwnerEvaluator.Assignment p) (scopeId : Nat)
    (capacity : Fin p → Nat) (budget : Nat) (seed : QualifiedCustody.State p a) : State p a → Prop where
  | fresh : Runs assigned scopeId capacity budget seed (initial seed budget)
  | owner : Runs assigned scopeId capacity budget seed s → s.actual.pending = none →
      SourceOwnerImage.ImageAllowed s.actual.source command →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.actual.owners i) (.owner command) = (next,reply) →
      Runs assigned scopeId capacity budget seed (ownerResult s i (.owner command) next reply)
  | extraFreeze : Runs assigned scopeId capacity budget seed s → s.actual.pending = none →
      revision ≤ s.actual.source.publication.barrier.published →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.actual.owners i) (.freeze revision) = (next,reply) →
      Runs assigned scopeId capacity budget seed (ownerResult s i (.freeze revision) next reply)
  | failedFreeze : Runs assigned scopeId capacity budget seed s → s.actual.pending = none → reply ≠ .inl 12 →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.actual.owners i) (.freeze revision) = (next,reply) →
      Runs assigned scopeId capacity budget seed (ownerResult s i (.freeze revision) next reply)
  | paidFreeze : Runs assigned scopeId capacity budget seed s → s.actual.pending = none →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.actual.owners i) (.freeze revision) = (next,.inl 12) →
      Runs assigned scopeId capacity budget seed
        {ownerResult s i (.freeze revision) next (.inl 12) with
          actual.pending := some (i,revision)}
  | source : Runs assigned scopeId capacity budget seed s → s.actual.pending = none →
      SourceOwnerFloor.sourceAllowed s.actual command → sourceAllowed s command →
      PublicationInput.execute scopeId s.actual.source command = some next →
      Runs assigned scopeId capacity budget seed {s with actual.source := next}
  | notify : Runs assigned scopeId capacity budget seed s → s.actual.pending = some (i,revision) →
      PublicationInput.execute scopeId s.actual.source (.freeze i revision) = some next →
      Runs assigned scopeId capacity budget seed {s with actual.source := next,actual.pending := none}

theorem image_path (path : Runs assigned scopeId capacity budget seed s) :
    SourceOwnerImage.Runs assigned scopeId capacity budget seed s.actual := by
  induction path with
  | fresh => exact .fresh
  | owner prior clear allowed ran ih => exact .owner ih clear allowed ran
  | extraFreeze prior clear bound ran ih => exact .extraFreeze ih clear bound ran
  | failedFreeze prior clear failed ran ih => exact .failedFreeze ih clear failed ran
  | paidFreeze prior clear ran ih => exact .paidFreeze ih clear ran
  | source prior clear floor install ran ih => exact .source ih clear floor ran
  | notify prior pending ran ih => exact .notify ih pending ran

def Invariant (s : State p a) : Prop := ∀ i,
  OwnerInstallationHistory.Authentic (s.actual.owners i) (s.installs i) ∧
    s.actual.source.publication.barrier.installed i ≤ actualRevision (s.actual.owners i)

theorem owner_preserves {p a : Nat} {s : State p a} {i : Fin p}
    {assigned : OwnerEvaluator.Assignment p} {scopeId capacity : Nat}
    {command : OwnerEndpoint.Command p a} {next : OwnerEndpointBudget.State p a}
    {reply : Sum Nat OwnerReceipt.Envelope}
    (old : Invariant s)
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity (s.actual.owners i) command = (next,reply)) :
    Invariant (ownerResult s i command next reply) := by
  intro j
  by_cases equal : j = i
  · subst j
    have facts := OwnerInstallationHistory.observe_preserves (old i).1 ran
    have bound := Nat.le_trans (old i).2 (revision_mono ran)
    simpa [ownerResult,SourceOwnerFloor.ownerResult,PublicationPayload.put] using And.intro facts bound
  · simpa [ownerResult,SourceOwnerFloor.ownerResult,PublicationPayload.put,equal] using old j

theorem source_preserves
    (old : Invariant s) (allowed : sourceAllowed s command)
    (ran : PublicationInput.execute scopeId s.actual.source command = some next) :
    Invariant {s with actual.source := next} := by
  intro j
  refine ⟨(old j).1,?_⟩
  have changed := source_installed ran
  cases command with
  | install i r =>
    obtain ⟨owner,present,bound⟩ := (old i).1 r allowed
    by_cases equal : j = i
    · subst j
      simpa [changed,commandInstalled,Publication.put,actualRevision,present] using bound
    · simpa [changed,commandInstalled,Publication.put,equal] using (old j).2
  | stage input => simpa [changed,commandInstalled] using (old j).2
  | arrival envelope => simpa [changed,commandInstalled] using (old j).2
  | freeze i r => simpa [changed,commandInstalled] using (old j).2
  | acknowledge i r => simpa [changed,commandInstalled] using (old j).2
  | publish => simpa [changed,commandInstalled] using (old j).2
  | enter i => simpa [changed,commandInstalled] using (old j).2
  | finish i => simpa [changed,commandInstalled] using (old j).2

theorem reached_invariant (path : Runs assigned scopeId capacity budget seed s) : Invariant s := by
  induction path with
  | fresh =>
    intro i
    constructor
    · intro r impossible; cases impossible
    · exact Nat.le_refl 0
  | owner prior clear allowed ran ih => exact owner_preserves ih ran
  | extraFreeze prior clear bound ran ih => exact owner_preserves ih ran
  | failedFreeze prior clear failed ran ih => exact owner_preserves ih ran
  | paidFreeze prior clear ran ih => exact owner_preserves ih ran
  | source prior clear floor install ran ih => exact source_preserves ih install ran
  | @notify s i r next prior pending ran ih =>
    exact source_preserves (command:=.freeze i r) ih trivial ran

-- No current-image preflight is assumed: registration, actual revision, image
-- and fence now follow from ONE history plus an open logical gate and an actual
-- initialized owner. Presence remains separate at initial zero revision.
theorem open_gate_current
    (path : Runs assigned scopeId capacity budget seed s)
    (clear : s.actual.pending = none)
    (present : (s.actual.owners i).owner = some owner)
    (openGate : s.actual.source.publication.barrier.installed i =
      s.actual.source.publication.barrier.fence i) :
    OwnerImageMonitor.Usable (s.actual.owners i) s.actual.source.publication.barrier.published
      (CustodyPublication.image s.actual.source.publication.current) := by
  have images := image_path path
  have floors := SourceOwnerImage.floor_path images
  have registered := (reached_invariant path i).2
  have compatible := SourceOwnerImage.reached_invariant images i owner.owner.core.revision
    owner.owner.core.image (by simp [OwnerImageMonitor.project,OwnerImageMonitor.view,present])
  have pubFloor := (SourceOwnerFloor.continuing_bounds floors clear i).1
  have fence := SourceOwnerFloor.continuing_floors floors clear i
  have coreBound : s.actual.source.publication.barrier.installed i ≤ owner.owner.core.revision := by
    simpa [actualRevision,present] using registered
  have physicalFence : owner.fence = s.actual.source.publication.barrier.fence i := by
    simpa [OwnerFenceMonitor.floor,present] using fence
  have lower : s.actual.source.publication.barrier.published ≤ owner.fence := by
    simpa [OwnerFenceMonitor.floor,present] using pubFloor
  have equal : owner.owner.core.revision = s.actual.source.publication.barrier.published := by
    have := compatible.1
    omega
  exact ⟨owner,present,equal,compatible.2 equal,by omega⟩

#print axioms revision_mono
#print axioms source_check_exact
#print axioms image_path
#print axioms owner_preserves
#print axioms source_preserves
#print axioms reached_invariant
#print axioms open_gate_current


-- Construct successful actual-model installation and its admitted notification
-- while preserving the retained-fact relation; no success premise is assumed.
theorem funded_head_install
    {p a : Nat} {assigned : SourceInput.Assignment p a} {scopeId budget : Nat}
    {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a} {s : State p a}
    {bootstrap : SourceInput.Bootstrap a} {old : PublicationCapacityDriver.State p a}
    {target : Fin p} {revision : Nat} {rest : List (PublicationInput.Command p a)}
    {before : Vector (Fin 513) p} {owner : OwnerEndpoint.State p a}
    (path : Runs (fun i => ⟨assigned.realm,i⟩) scopeId capacity budget seed s)
    (clear : s.actual.pending = none)
    (valid : PublicationLifecycle.Invariant assigned scopeId bootstrap old)
    (sourceAt : old.source = some s.actual.source) (head : old.suffix = .install target revision :: rest)
    (covered : PublicationOwnerBudget.Covered old.suffix (fun i => (before[i.val]).val))
    (ownerAt : s.actual.owners target = ⟨some owner,(before[target.val]).val⟩)
    (idle : owner.owner.active = none) :
    ∃ (next : PublicationCapacityDriver.State p a) (last : OwnerEndpoint.State p a)
        (after : PublicationInput.State p a),
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scopeId (capacity target) (s.actual.owners target)
        (.owner (.install revision (CustodyPublication.image s.actual.source.publication.current))) =
        (⟨some last,((SourceFundingAdministration.paidVector before target)[target.val]).val⟩,.inl 7) ∧
      last.fence = revision ∧ last.owner.core.revision = revision ∧
      last.owner.core.image = CustodyPublication.image s.actual.source.publication.current ∧
      last.owner.active = none ∧ last.owner.reserved = owner.owner.reserved ∧
      last.owner.core.records = owner.owner.core.records ∧
      SourceFundingInput.execute assigned scopeId bootstrap old
        (SourceFundingAdministration.paidVector before target,.step (.install target revision)) = (next,.accepted) ∧
      next.source = some after ∧ next.suffix = rest ∧
      PublicationLifecycle.Invariant assigned scopeId bootstrap next ∧
      PublicationOwnerBudget.Covered next.suffix
        (fun i => ((SourceFundingAdministration.paidVector before target)[i.val]).val) ∧
      Runs (fun i => ⟨assigned.realm,i⟩) scopeId capacity budget seed
        {ownerResult s target (.owner (.install revision (CustodyPublication.image s.actual.source.publication.current)))
          ⟨some last,((SourceFundingAdministration.paidVector before target)[target.val]).val⟩ (.inl 7)
          with actual.source := after} := by
  obtain ⟨logicalNext,logical⟩ := SourceOwnerImage.certified_head_step valid sourceAt head
  have present : (s.actual.owners target).owner = some owner := by rw [ownerAt]
  obtain ⟨published,fence,ready⟩ := SourceOwnerImage.install_ready (image_path path) clear logical present
  obtain ⟨next,last,ownerRan,fenceAt,revisionAt,imageAt,stillIdle,keys,records,sourceRan,tail,preserved,funded⟩ :=
    SourceFundingAdministration.install_then_notify (capacity:=capacity target)
      valid sourceAt head covered idle fence ready
  have actualOwner := ownerRan
  rw [←ownerAt] at actualOwner
  obtain ⟨after,atAfter,actualSource⟩ := SourceOwnerFloor.funded_step_actual valid sourceAt sourceRan
  refine ⟨next,last,after,actualOwner,fenceAt,revisionAt,imageAt,stillIdle,keys,records,
    sourceRan,atAfter,tail,preserved,funded,?_⟩
  have allowed : SourceOwnerImage.ImageAllowed s.actual.source (.install revision (CustodyPublication.image s.actual.source.publication.current)) :=
    ⟨published,rfl⟩
  have middlePath := Runs.owner path clear allowed actualOwner
  have observed : sourceAllowed
      (ownerResult s target (.owner (.install revision (CustodyPublication.image s.actual.source.publication.current)))
        ⟨some last,((SourceFundingAdministration.paidVector before target)[target.val]).val⟩ (.inl 7))
      (.install target revision) := by
    simp [sourceAllowed,ownerResult,PublicationPayload.put,OwnerInstallationHistory.observe,OwnerImageMonitor.observe]
  exact Runs.source (command:=.install target revision) middlePath clear trivial observed actualSource

#print axioms funded_head_install

-- The ordinary accepted source-entry consumer no longer needs the independent
-- current-image preflight as a premise. Actual existence is still explicit.
theorem entered_current
    (path : Runs assigned scopeId capacity budget seed s)
    (clear : s.actual.pending = none)
    (present : (s.actual.owners i).owner = some owner)
    (entered : PublicationInput.execute scopeId s.actual.source (.enter i) = some next) :
    OwnerImageMonitor.Usable (s.actual.owners i) s.actual.source.publication.barrier.published
      (CustodyPublication.image s.actual.source.publication.current) :=
  open_gate_current path clear present (SourceOwnerGate.entered_logical_gate entered)

-- General counter to dropping existence: fresh numerical zero equality does
-- not mean that an owner process/state already exists, for any nonempty slot.
theorem fresh_gate_without_owner (seed : QualifiedCustody.State p a) (budget : Nat) (i : Fin p) :
    (initial seed budget).actual.source.publication.barrier.installed i =
      (initial seed budget).actual.source.publication.barrier.fence i ∧
    ¬ OwnerImageMonitor.Usable ((initial seed budget).actual.owners i) 0
      (CustodyPublication.image seed) := by
  constructor
  · rfl
  · rintro ⟨owner,present,rest⟩
    cases present

#print axioms entered_current
#print axioms fresh_gate_without_owner

theorem initialize_success_absent
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity state
      (.owner (.initialize image)) = (next,.inl 10)) : state.owner = none := by
  cases present : state.owner with
  | none => rfl
  | some owner =>
    simp only [OwnerEndpointBudget.transition,present,
      OwnerEndpointProfile.transition,OwnerEndpointProfile.check,
      OwnerEndpoint.transition,OwnerEndpoint.confirms,OwnerEndpoint.permitted,
      OwnerReservationWorker.transition] at ran
    split at ran <;> cases ran
#print axioms initialize_success_absent

-- At a clear public boundary the existing joint floor theorem already forces
-- zero publication for any absent owner. No stronger Runs relation is needed.
theorem absent_publication_zero
    (path : Runs assigned scopeId capacity budget seed s)
    (clear : s.actual.pending = none)
    (absent : (s.actual.owners i).owner = none) :
    s.actual.source.publication.barrier.published = 0 := by
  have lower := (SourceOwnerFloor.continuing_bounds
    (SourceOwnerImage.floor_path (image_path path)) clear i).1
  simpa [OwnerFenceMonitor.floor,absent] using lower

theorem initialize_publication_zero
    (path : Runs assigned scopeId capacity budget seed s)
    (clear : s.actual.pending = none)
    (ran : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.actual.owners i)
      (.owner (.initialize image)) = (next,.inl 10)) :
    s.actual.source.publication.barrier.published = 0 :=
  absent_publication_zero path clear (initialize_success_absent ran)

theorem initialize_record_matches
    (path : Runs assigned scopeId capacity budget seed s)
    (clear : s.actual.pending = none)
    (ran : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.actual.owners i)
      (.owner (.initialize image)) = (next,.inl 10)) :
    OwnerInstallationHistory.observe (s.installs i) (.owner (.initialize image)) (.inl 10) =
      s.actual.source.publication.barrier.published :: s.installs i := by
  rw [initialize_publication_zero path clear ran]
  rfl
#print axioms absent_publication_zero
#print axioms initialize_publication_zero
#print axioms initialize_record_matches
-- Every coordinate is tied to this same actual model state. This is not a
-- claim that a caller-supplied vector is authenticated or physically current.
def CreditsAt (s : State p a) (credits : Vector (Fin 513) p) : Prop :=
  ∀ i, (s.actual.owners i).remaining = (credits[i.val]).val

theorem credits_owner_paid {p a : Nat} {s : State p a} {before : Vector (Fin 513) p}
    {i : Fin p} {next : OwnerEndpointBudget.State p a}
    {command : OwnerEndpoint.Command p a} {reply : Sum Nat OwnerReceipt.Envelope}
    (bound : CreditsAt s before)
    (debit : next.remaining = (before[i.val]).val - 1) :
    CreditsAt (ownerResult s i command next reply)
      (SourceFundingAdministration.paidVector before i) := by
  intro j
  by_cases equal : j = i
  · subst j
    simpa [ownerResult,SourceOwnerFloor.ownerResult,PublicationPayload.put,
      SourceFundingAdministration.paidVector] using debit
  · simpa [ownerResult,SourceOwnerFloor.ownerResult,PublicationPayload.put,
      SourceFundingAdministration.paidVector,equal] using bound j

theorem coverage_actual {p a : Nat} {s : State p a} {before : Vector (Fin 513) p}
    {suffix : List (PublicationInput.Command p a)} (bound : CreditsAt s before)
    (covered : PublicationOwnerBudget.Covered suffix (fun i => (before[i.val]).val)) :
    PublicationOwnerBudget.Covered suffix (fun i => (s.actual.owners i).remaining) := by
  intro i
  change PublicationOwnerBudget.demand suffix i ≤ (s.actual.owners i).remaining
  rw [bound i]
  exact covered i

theorem funded_head_install_actual
    {p a : Nat} {assigned : SourceInput.Assignment p a} {scopeId budget : Nat}
    {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a} {s : State p a}
    {bootstrap : SourceInput.Bootstrap a} {old : PublicationCapacityDriver.State p a}
    {target : Fin p} {revision : Nat} {rest : List (PublicationInput.Command p a)}
    {before : Vector (Fin 513) p} {owner : OwnerEndpoint.State p a}
    (path : Runs (fun i => ⟨assigned.realm,i⟩) scopeId capacity budget seed s)
    (clear : s.actual.pending = none)
    (valid : PublicationLifecycle.Invariant assigned scopeId bootstrap old)
    (sourceAt : old.source = some s.actual.source) (head : old.suffix = .install target revision :: rest)
    (bound : CreditsAt s before)
    (covered : PublicationOwnerBudget.Covered old.suffix (fun i => (s.actual.owners i).remaining))
    (present : (s.actual.owners target).owner = some owner)
    (idle : owner.owner.active = none) :
    ∃ (next : PublicationCapacityDriver.State p a) (last : OwnerEndpoint.State p a)
        (after : PublicationInput.State p a),
      let actualNext := {ownerResult s target
          (.owner (.install revision (CustodyPublication.image s.actual.source.publication.current)))
          ⟨some last,((SourceFundingAdministration.paidVector before target)[target.val]).val⟩ (.inl 7)
          with actual.source := after}
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scopeId (capacity target) (s.actual.owners target)
        (.owner (.install revision (CustodyPublication.image s.actual.source.publication.current))) =
        (actualNext.actual.owners target,.inl 7) ∧
      SourceFundingInput.execute assigned scopeId bootstrap old
        (SourceFundingAdministration.paidVector before target,.step (.install target revision)) = (next,.accepted) ∧
      next.source = some after ∧ next.suffix = rest ∧
      PublicationLifecycle.Invariant assigned scopeId bootstrap next ∧
      Runs (fun i => ⟨assigned.realm,i⟩) scopeId capacity budget seed actualNext ∧
      CreditsAt actualNext (SourceFundingAdministration.paidVector before target) ∧
      PublicationOwnerBudget.Covered next.suffix (fun i => (actualNext.actual.owners i).remaining) := by
  have supplied : PublicationOwnerBudget.Covered old.suffix (fun i => (before[i.val]).val) := by
    intro i; simpa only [bound i] using covered i
  have ownerAt : s.actual.owners target = ⟨some owner,(before[target.val]).val⟩ := by
    cases h : s.actual.owners target with
    | mk value remaining =>
      have balance := bound target
      simp only [h] at present balance
      simp [present,balance]
  obtain ⟨next,last,after,ownerRan,fenceAt,revisionAt,imageAt,stillIdle,keys,records,
    sourceRan,afterAt,tail,preserved,funded,newPath⟩ :=
    funded_head_install path clear valid sourceAt head supplied ownerAt idle
  have balances := credits_owner_paid (i:=target) (command:=.owner (.install revision
    (CustodyPublication.image s.actual.source.publication.current))) (reply:=.inl 7)
    (next:=⟨some last,((SourceFundingAdministration.paidVector before target)[target.val]).val⟩)
    bound (by simp [SourceFundingAdministration.paidVector])
  refine ⟨next,last,after,?_,sourceRan,afterAt,tail,preserved,newPath,balances,?_⟩
  · simpa [ownerResult,SourceOwnerFloor.ownerResult,PublicationPayload.put] using ownerRan
  · exact coverage_actual balances funded

#print axioms credits_owner_paid
#print axioms coverage_actual
#print axioms funded_head_install_actual

end MirroreaProofFirst.SourceRegistration
