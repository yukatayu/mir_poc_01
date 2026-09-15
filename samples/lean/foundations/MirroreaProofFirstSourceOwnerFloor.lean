import MirroreaProofFirstSourceOwnerFence
import MirroreaProofFirstSourceFundingAdministration

namespace MirroreaProofFirst.SourceOwnerFloor

theorem source_numeric {p a : Nat} {initial : QualifiedCustody.State p a}
    {source : PublicationInput.State p a} (path : PublicationInput.Reached scopeId initial source) :
    Publication.Reached p 0 source.publication.barrier := by
  obtain ⟨abstract,reached,equal⟩ := PublicationImage.reached_lifts
    (PublicationInput.reached_publication path)
  rw [equal]
  exact PublicationPayload.reached_numeric (PublicationUse.reached_payload reached)

structure State (p a : Nat) where
  source : PublicationInput.State p a
  owners : Fin p → OwnerEndpointBudget.State p a
  records : Fin p → List Nat
  pending : Option (Fin p × Nat)

def initial (source : QualifiedCustody.State p a) (budget : Nat) : State p a :=
  ⟨PublicationInput.initial source,fun _ => OwnerEndpointBudget.initial budget,fun _ => [],none⟩

def pendingAt (pending : Option (Fin p × Nat)) (i : Fin p) : Nat :=
  match pending with
  | none => 0
  | some (target,revision) => if i = target then revision else 0

def sourceAllowed (s : State p a) : PublicationInput.Command p a → Prop
  | .freeze i revision => revision ∈ s.records i
  | _ => True

def ownerResult (s : State p a) (i : Fin p) (command : OwnerEndpoint.Command p a)
    (next : OwnerEndpointBudget.State p a) (reply : Sum Nat OwnerReceipt.Envelope) : State p a :=
  {s with owners := PublicationPayload.put s.owners i next
          records := PublicationPayload.put s.records i (OwnerFenceMonitor.observe (s.records i) command reply)}

-- This is a broad event relation for the floor projection only. It includes
-- ALL actual non-freeze owner commands, every source command and old truthful
-- freeze notifications. It deliberately does not assert image/idle/funding/auth
-- preservation. The selected host is narrower; its complete refinement remains
-- separate. A successful paid freeze has an explicit unnotified intermediate
-- state. No other operation can advance that state before its matching notice.
inductive Runs (assigned : Fin p → OwnerEvaluator.Assignment p) (scopeId : Nat) (capacity : Fin p → Nat) (budget : Nat)
    (seed : QualifiedCustody.State p a) : State p a → Prop where
  | fresh : Runs assigned scopeId capacity budget seed (initial seed budget)
  | owner : Runs assigned scopeId capacity budget seed s → s.pending = none →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.owners i) (.owner command) = (next,reply) →
      Runs assigned scopeId capacity budget seed (ownerResult s i (.owner command) next reply)
  | extraFreeze : Runs assigned scopeId capacity budget seed s → s.pending = none →
      revision ≤ s.source.publication.barrier.published →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.owners i) (.freeze revision) = (next,reply) →
      Runs assigned scopeId capacity budget seed (ownerResult s i (.freeze revision) next reply)
  | failedFreeze : Runs assigned scopeId capacity budget seed s → s.pending = none →
      reply ≠ .inl 12 →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.owners i) (.freeze revision) = (next,reply) →
      Runs assigned scopeId capacity budget seed (ownerResult s i (.freeze revision) next reply)
  | paidFreeze : Runs assigned scopeId capacity budget seed s → s.pending = none →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.owners i) (.freeze revision) = (next,.inl 12) →
      Runs assigned scopeId capacity budget seed
        {ownerResult s i (.freeze revision) next (.inl 12) with pending := some (i,revision)}
  | source : Runs assigned scopeId capacity budget seed s → s.pending = none →
      sourceAllowed s command → PublicationInput.execute scopeId s.source command = some next →
      Runs assigned scopeId capacity budget seed {s with source := next}
  | notify : Runs assigned scopeId capacity budget seed s → s.pending = some (i,revision) →
      PublicationInput.execute scopeId s.source (.freeze i revision) = some next →
      Runs assigned scopeId capacity budget seed {s with source := next,pending := none}

-- No desired conclusion is a premise of Runs. Exact observations are proved
-- from every actual owner transition, and numerical reachability from every
-- actual source transition.
structure Invariant (assigned : Fin p → OwnerEvaluator.Assignment p) (scopeId : Nat) (capacity : Fin p → Nat) (budget : Nat)
    (seed : QualifiedCustody.State p a) (s : State p a) : Prop where
  sourcePath : PublicationInput.Reached scopeId seed s.source
  ownerPaths : ∀ i, OwnerFenceMonitor.ObservedRuns (assigned i) scopeId (capacity i) budget (s.owners i) (s.records i)
  aligned : ∀ i, OwnerFenceMonitor.floor (s.owners i).owner =
    max (s.source.publication.barrier.fence i) (pendingAt s.pending i)

theorem actual_record_bound (path : OwnerFenceMonitor.ObservedRuns assigned scopeId capacity budget owner records)
    (member : revision ∈ records) : revision ≤ OwnerFenceMonitor.floor owner.owner := by
  have exact := OwnerFenceMonitor.reached_exact (OwnerFenceMonitor.observed_path path)
  rw [exact]
  exact OwnerFenceMonitor.maximum_bounded.mp (Nat.le_refl _) revision member

theorem owner_paths_updated {p a : Nat} {capacity : Fin p → Nat} {assigned : Fin p → OwnerEvaluator.Assignment p}
    {s : State p a} {i : Fin p} {next : OwnerEndpointBudget.State p a}
    {command : OwnerEndpoint.Command p a} {reply : Sum Nat OwnerReceipt.Envelope}
    (paths : ∀ j, OwnerFenceMonitor.ObservedRuns (assigned j) scopeId (capacity j) budget (s.owners j) (s.records j))
    (ran : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.owners i) command = (next,reply)) :
    ∀ j, OwnerFenceMonitor.ObservedRuns (assigned j) scopeId (capacity j) budget
      ((ownerResult s i command next reply).owners j) ((ownerResult s i command next reply).records j) := by
  intro j
  by_cases equal : j = i
  · subst j
    simpa [ownerResult,PublicationPayload.put] using OwnerFenceMonitor.ObservedRuns.step (paths i) ran
  · simpa [ownerResult,PublicationPayload.put,equal] using paths j

theorem reached_invariant (path : Runs assigned scopeId capacity budget seed s) :
    Invariant assigned scopeId capacity budget seed s := by
  induction path with
  | fresh => exact ⟨.initial,fun _ => .fresh,fun _ => rfl⟩
  | @owner s i command next reply prior clear ran ih =>
    refine ⟨ih.sourcePath,owner_paths_updated ih.ownerPaths ran,?_⟩
    intro j
    by_cases equal : j = i
    · subst j
      have updated := OwnerFenceMonitor.budget_exact ran
      simpa [ownerResult,PublicationPayload.put,OwnerFenceMonitor.advance] using updated.trans (ih.aligned i)
    · simpa [ownerResult,PublicationPayload.put,equal] using ih.aligned j
  | @extraFreeze s revision i next reply prior clear bounded ran ih =>
    refine ⟨ih.sourcePath,owner_paths_updated ih.ownerPaths ran,?_⟩
    intro j
    by_cases equal : j = i
    · subst j
      have numeric := (Publication.reached_invariant (source_numeric ih.sourcePath)).2.1 i
      have aligned : OwnerFenceMonitor.floor (s.owners i).owner = s.source.publication.barrier.fence i := by
        simpa [clear,pendingAt] using ih.aligned i
      have bound : revision ≤ OwnerFenceMonitor.floor (s.owners i).owner := by omega
      have same := SourceOwnerFence.bounded_owner_freeze_frames bound ran
      simpa [ownerResult,PublicationPayload.put] using same.trans (ih.aligned i)
    · simpa [ownerResult,PublicationPayload.put,equal] using ih.aligned j
  | @failedFreeze s reply i revision next prior clear failed ran ih =>
    refine ⟨ih.sourcePath,owner_paths_updated ih.ownerPaths ran,?_⟩
    intro j
    by_cases equal : j = i
    · subst j
      have same : OwnerFenceMonitor.floor next.owner = OwnerFenceMonitor.floor (s.owners i).owner := by
        rw [OwnerFenceMonitor.budget_exact ran]
        cases reply with
        | inr envelope => rfl
        | inl code =>
          have different : code ≠ 12 := by intro equal; subst code; exact failed rfl
          simp [OwnerFenceMonitor.advance,different]
      simpa [ownerResult,PublicationPayload.put] using same.trans (ih.aligned i)
    · simpa [ownerResult,PublicationPayload.put,equal] using ih.aligned j
  | @paidFreeze s i revision next prior clear ran ih =>
    refine ⟨ih.sourcePath,owner_paths_updated ih.ownerPaths ran,?_⟩
    intro j
    by_cases equal : j = i
    · subst j
      have before : OwnerFenceMonitor.floor (s.owners i).owner = s.source.publication.barrier.fence i := by
        simpa [clear,pendingAt] using ih.aligned i
      have after := OwnerFenceMonitor.budget_exact ran
      simpa [ownerResult,PublicationPayload.put,pendingAt,OwnerFenceMonitor.advance,before] using after
    · simpa [ownerResult,PublicationPayload.put,pendingAt,equal,clear] using ih.aligned j
  | @source s command next prior clear allowed ran ih =>
    refine ⟨.step ih.sourcePath ran,ih.ownerPaths,?_⟩
    intro j
    have before : OwnerFenceMonitor.floor (s.owners j).owner = s.source.publication.barrier.fence j := by
      simpa [clear,pendingAt] using ih.aligned j
    have updated := SourceOwnerFence.source_fences ran
    cases command with
    | freeze i revision =>
      have bounded := actual_record_bound (ih.ownerPaths i) allowed
      by_cases equal : j = i
      · subst j
        have le : revision ≤ s.source.publication.barrier.fence i := by omega
        simp [clear,pendingAt,updated,SourceOwnerFence.commandFences,Publication.put,Nat.max_eq_left le,before]
      · simp [clear,pendingAt,updated,SourceOwnerFence.commandFences,Publication.put,equal,before]
    | stage input => simpa [clear,pendingAt,updated,SourceOwnerFence.commandFences] using before
    | arrival envelope => simpa [clear,pendingAt,updated,SourceOwnerFence.commandFences] using before
    | acknowledge i revision => simpa [clear,pendingAt,updated,SourceOwnerFence.commandFences] using before
    | publish => simpa [clear,pendingAt,updated,SourceOwnerFence.commandFences] using before
    | install i revision => simpa [clear,pendingAt,updated,SourceOwnerFence.commandFences] using before
    | enter i => simpa [clear,pendingAt,updated,SourceOwnerFence.commandFences] using before
    | finish i => simpa [clear,pendingAt,updated,SourceOwnerFence.commandFences] using before
  | @notify s i revision next prior pending ran ih =>
    refine ⟨.step ih.sourcePath ran,ih.ownerPaths,?_⟩
    intro j
    have updated := SourceOwnerFence.source_fences ran
    by_cases equal : j = i
    · subst j
      simpa [pending,pendingAt,updated,SourceOwnerFence.commandFences,Publication.put] using ih.aligned i
    · simpa [pending,pendingAt,updated,SourceOwnerFence.commandFences,Publication.put,equal] using ih.aligned j

theorem continuing_floors (path : Runs assigned scopeId capacity budget seed s) (clear : s.pending = none) :
    ∀ i, OwnerFenceMonitor.floor (s.owners i).owner = s.source.publication.barrier.fence i := by
  intro i
  simpa [clear,pendingAt] using (reached_invariant path).aligned i

theorem continuing_bounds (path : Runs assigned scopeId capacity budget seed s) (clear : s.pending = none) :
    ∀ i, s.source.publication.barrier.published ≤ OwnerFenceMonitor.floor (s.owners i).owner ∧
      OwnerFenceMonitor.floor (s.owners i).owner ≤ s.source.publication.barrier.announced := by
  intro i
  rw [continuing_floors path clear]
  have numeric := source_numeric (reached_invariant path).sourcePath
  exact ⟨(Publication.reached_invariant numeric).2.1 i |>.2.1,(PublicationProgress.reached_upper numeric).2.2 i⟩

-- Extract the actual source step from a successful metered source operation.
-- This does not turn a caller's claim about the source into a transition.
theorem funded_step_actual
    (valid : PublicationLifecycle.Invariant assigned scopeId bootstrap old)
    (present : old.source = some source)
    (accepted : SourceFundingInput.execute assigned scopeId bootstrap old (credits,.step command) =
      (next,.accepted)) :
    ∃ after, next.source = some after ∧ PublicationInput.execute scopeId source command = some after := by
  have native := (SourceFundingInput.accepted_exact.mp accepted).1
  rw [PublicationLifecycle.transitionFast_exact valid] at native
  have sem := (PublicationLifecycle.accepted_refines assigned scopeId bootstrap old
    (.step command) (by rw [native])).1
  rw [native,present] at sem
  cases step : PublicationInput.execute scopeId source command with
  | none => simp [PublicationInput.transition,step] at sem
  | some after =>
    refine ⟨after,?_,rfl⟩
    simpa [PublicationInput.transition,step] using (Prod.mk.inj sem).1.symm

-- Positive consumer: derive the owner's floor bound from the actual joint
-- history, then construct native-model freeze12 AND accepted metered notice.
-- Idle, funding and the binding to the same physical state remain independent
-- premises. No successful freeze, successful notice or post-state is assumed.
theorem funded_head_freeze
    {p a : Nat} {assigned : SourceInput.Assignment p a} {scopeId budget : Nat} {capacity : Fin p → Nat}
    {seed : QualifiedCustody.State p a} {s : State p a}
    {bootstrap : SourceInput.Bootstrap a} {old : PublicationCapacityDriver.State p a}
    {target : Fin p} {revision : Nat} {rest : List (PublicationInput.Command p a)}
    {before : Vector (Fin 513) p} {owner : OwnerEndpoint.State p a}
    (path : Runs (fun i => ⟨assigned.realm,i⟩) scopeId capacity budget seed s)
    (clear : s.pending = none)
    (valid : PublicationLifecycle.Invariant assigned scopeId bootstrap old)
    (sourceAt : old.source = some s.source)
    (head : old.suffix = .freeze target revision :: rest)
    (announced : revision = s.source.publication.barrier.announced)
    (covered : PublicationOwnerBudget.Covered old.suffix (fun i => (before[i.val]).val))
    (ownerAt : s.owners target = ⟨some owner,(before[target.val]).val⟩)
    (idle : owner.owner.active = none) :
    ∃ (next : PublicationCapacityDriver.State p a) (after : PublicationInput.State p a),
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scopeId (capacity target) (s.owners target) (.freeze revision) =
        (⟨some {owner with fence := revision},((SourceFundingAdministration.paidVector before target)[target.val]).val⟩,.inl 12) ∧
      SourceFundingInput.execute assigned scopeId bootstrap old
        (SourceFundingAdministration.paidVector before target,.step (.freeze target revision)) = (next,.accepted) ∧
      next.source = some after ∧ next.suffix = rest ∧
      PublicationLifecycle.Invariant assigned scopeId bootstrap next ∧
      PublicationOwnerBudget.Covered next.suffix
        (fun i => ((SourceFundingAdministration.paidVector before target)[i.val]).val) ∧
      Runs (fun i => ⟨assigned.realm,i⟩) scopeId capacity budget seed
        {ownerResult s target (.freeze revision)
          ⟨some {owner with fence := revision},((SourceFundingAdministration.paidVector before target)[target.val]).val⟩
          (.inl 12) with source := after,pending := none} := by
  have bound : owner.fence ≤ revision := by
    have actual := (continuing_bounds path clear target).2
    simpa [ownerAt,OwnerFenceMonitor.floor,←announced] using actual
  obtain ⟨next,ownerRan,sourceRan,tail,preserved,funded⟩ :=
    SourceFundingAdministration.freeze_then_notify (capacity:=capacity target) valid sourceAt head covered idle bound
  have actualOwner := ownerRan
  rw [←ownerAt] at actualOwner
  obtain ⟨after,atAfter,actualSource⟩ := funded_step_actual valid sourceAt sourceRan
  refine ⟨next,after,actualOwner,sourceRan,atAfter,tail,preserved,funded,?_⟩
  have middlePath := Runs.paidFreeze path clear actualOwner
  have afterPath := Runs.notify middlePath rfl actualSource
  exact afterPath

#print axioms funded_step_actual
#print axioms funded_head_freeze

#print axioms source_numeric
#print axioms actual_record_bound
#print axioms owner_paths_updated
#print axioms reached_invariant
#print axioms continuing_floors
#print axioms continuing_bounds
end MirroreaProofFirst.SourceOwnerFloor
