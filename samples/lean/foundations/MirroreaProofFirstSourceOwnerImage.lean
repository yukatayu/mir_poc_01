import MirroreaProofFirstSourceCurrentFrame

namespace MirroreaProofFirst.SourceOwnerImage
open SourceOwnerFloor (State ownerResult sourceAllowed)

def Compatible (published : Nat) (image : OwnerImage.Image p a)
    (known : Option (OwnerImageMonitor.Current p a)) : Prop :=
  ∀ revision actual, known = some (revision,actual) →
    revision ≤ published ∧ (revision = published → actual = image)

def ImageAllowed (source : PublicationInput.State p a) : OwnerReservationWorker.Command p a → Prop
  | .initialize image => image = CustodyPublication.image source.publication.current
  | .install revision image => revision = source.publication.barrier.published ∧
      image = CustodyPublication.image source.publication.current
  | _ => True

def imageCheck (source : PublicationInput.State p a) : OwnerReservationWorker.Command p a → Bool
  | .initialize image => OwnerEndpoint.sameImage image (CustodyPublication.image source.publication.current)
  | .install revision image => decide (revision = source.publication.barrier.published) &&
      OwnerEndpoint.sameImage image (CustodyPublication.image source.publication.current)
  | _ => true

theorem image_check_exact : imageCheck source command = true ↔ ImageAllowed source command := by
  cases command <;> simp [imageCheck,ImageAllowed,OwnerEndpoint.sameImage_exact]

theorem owner_observation_preserves
    (old : Compatible source.publication.barrier.published (CustodyPublication.image source.publication.current) known)
    (allowed : ImageAllowed source command) :
    Compatible source.publication.barrier.published (CustodyPublication.image source.publication.current)
      (OwnerImageMonitor.observe known (.owner command) reply) := by
  cases command with
  | reserve ticket => exact old
  | compute => exact old
  | abandon => exact old
  | «initialize» image =>
    cases reply with
    | inr envelope => exact old
    | inl code =>
      by_cases success : code = 10
      · subst code
        intro revision actual equal
        have fields : (0,image) = (revision,actual) := Option.some.inj equal
        obtain ⟨rfl,rfl⟩ := Prod.mk.inj fields
        exact ⟨Nat.zero_le _,fun _ => allowed⟩
      · simpa [OwnerImageMonitor.observe,success] using old
  | install revision image =>
    cases reply with
    | inr envelope => exact old
    | inl code =>
      by_cases success : code = 7
      · subst code
        intro stored actual equal
        have fields : (revision,image) = (stored,actual) := Option.some.inj equal
        obtain ⟨rfl,rfl⟩ := Prod.mk.inj fields
        exact ⟨Nat.le_of_eq allowed.1,fun _ => allowed.2⟩
      · simpa [OwnerImageMonitor.observe,success] using old

theorem source_preserves
    (old : Compatible source.publication.barrier.published (CustodyPublication.image source.publication.current) known)
    (ran : PublicationInput.execute scopeId source command = some next) :
    Compatible next.publication.barrier.published (CustodyPublication.image next.publication.current) known := by
  rcases SourceCurrentFrame.source_current ran with ⟨revision,current⟩ | newer
  · simpa [revision,current] using old
  · intro revision image present
    obtain ⟨bound,_⟩ := old revision image present
    exact ⟨by omega,fun equal => by omega⟩

-- This refinement restricts only image-bearing owner inputs to the actual
-- source publication. The floor relation already covers the other operations.
-- It is still a projection model: installation receipts/registration, idle,
-- keys/capacity, funding vectors and physical call-graph refinement are separate.
inductive Runs (assigned : Fin p → OwnerEvaluator.Assignment p) (scopeId : Nat) (capacity : Fin p → Nat) (budget : Nat)
    (seed : QualifiedCustody.State p a) : State p a → Prop where
  | fresh : Runs assigned scopeId capacity budget seed (SourceOwnerFloor.initial seed budget)
  | owner : Runs assigned scopeId capacity budget seed s → s.pending = none → ImageAllowed s.source command →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.owners i) (.owner command) = (next,reply) →
      Runs assigned scopeId capacity budget seed (ownerResult s i (.owner command) next reply)
  | extraFreeze : Runs assigned scopeId capacity budget seed s → s.pending = none →
      revision ≤ s.source.publication.barrier.published →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.owners i) (.freeze revision) = (next,reply) →
      Runs assigned scopeId capacity budget seed (ownerResult s i (.freeze revision) next reply)
  | failedFreeze : Runs assigned scopeId capacity budget seed s → s.pending = none → reply ≠ .inl 12 →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.owners i) (.freeze revision) = (next,reply) →
      Runs assigned scopeId capacity budget seed (ownerResult s i (.freeze revision) next reply)
  | paidFreeze : Runs assigned scopeId capacity budget seed s → s.pending = none →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.owners i) (.freeze revision) = (next,.inl 12) →
      Runs assigned scopeId capacity budget seed
        {ownerResult s i (.freeze revision) next (.inl 12) with pending := some (i,revision)}
  | source : Runs assigned scopeId capacity budget seed s → s.pending = none → sourceAllowed s command →
      PublicationInput.execute scopeId s.source command = some next →
      Runs assigned scopeId capacity budget seed {s with source := next}
  | notify : Runs assigned scopeId capacity budget seed s → s.pending = some (i,revision) →
      PublicationInput.execute scopeId s.source (.freeze i revision) = some next →
      Runs assigned scopeId capacity budget seed {s with source := next,pending := none}

theorem floor_path (path : Runs assigned scopeId capacity budget seed s) :
    SourceOwnerFloor.Runs assigned scopeId capacity budget seed s := by
  induction path with
  | fresh => exact .fresh
  | owner prior clear allowed ran ih => exact .owner ih clear ran
  | extraFreeze prior clear bounded ran ih => exact .extraFreeze ih clear bounded ran
  | failedFreeze prior clear failed ran ih => exact .failedFreeze ih clear failed ran
  | paidFreeze prior clear ran ih => exact .paidFreeze ih clear ran
  | source prior clear allowed ran ih => exact .source ih clear allowed ran
  | notify prior pending ran ih => exact .notify ih pending ran

def Invariant (s : State p a) : Prop := ∀ i,
  Compatible s.source.publication.barrier.published (CustodyPublication.image s.source.publication.current)
    (OwnerImageMonitor.project (s.owners i).owner)

theorem owner_preserves {p a : Nat} {assigned : OwnerEvaluator.Assignment p} {s : State p a} {i : Fin p}
    {command : OwnerReservationWorker.Command p a} {next : OwnerEndpointBudget.State p a}
    {reply : Sum Nat OwnerReceipt.Envelope}
    (old : Invariant s) (allowed : ImageAllowed s.source command)
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity (s.owners i) (.owner command) = (next,reply)) :
    Invariant (ownerResult s i (.owner command) next reply) := by
  intro j
  by_cases equal : j = i
  · subst j
    have compatible := owner_observation_preserves (old i) allowed (reply:=reply)
    rw [←OwnerImageMonitor.budget_exact ran] at compatible
    simpa [ownerResult,PublicationPayload.put] using compatible
  · simpa [ownerResult,PublicationPayload.put,equal] using old j

theorem freeze_preserves {p a : Nat} {assigned : OwnerEvaluator.Assignment p} {s : State p a} {i : Fin p}
    {next : OwnerEndpointBudget.State p a} {reply : Sum Nat OwnerReceipt.Envelope}
    (old : Invariant s)
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity (s.owners i) (.freeze revision) = (next,reply)) :
    Invariant (ownerResult s i (.freeze revision) next reply) := by
  intro j
  by_cases equal : j = i
  · subst j
    have same : OwnerImageMonitor.project next.owner = OwnerImageMonitor.project (s.owners i).owner :=
      OwnerImageMonitor.budget_exact ran
    have compatible := old i
    rw [←same] at compatible
    simpa [ownerResult,PublicationPayload.put] using compatible
  · simpa [ownerResult,PublicationPayload.put,equal] using old j

theorem reached_invariant (path : Runs assigned scopeId capacity budget seed s) : Invariant s := by
  induction path with
  | fresh => intro i revision actual impossible; cases impossible
  | owner prior clear allowed ran ih => exact owner_preserves ih allowed ran
  | extraFreeze prior clear bounded ran ih => exact freeze_preserves ih ran
  | failedFreeze prior clear failed ran ih => exact freeze_preserves ih ran
  | paidFreeze prior clear ran ih => exact freeze_preserves ih ran
  | source prior clear allowed ran ih => exact fun i => source_preserves (ih i) ran
  | notify prior pending ran ih => exact fun i => source_preserves (ih i) ran

-- Certification supplies an actual logical next step; no head-query response
-- substitutes for this declaration-level path.
theorem certified_head_step
    (valid : PublicationLifecycle.Invariant assigned scopeId bootstrap old)
    (present : old.source = some source) (head : old.suffix = command :: rest) :
    ∃ next, PublicationInput.execute scopeId source command = some next := by
  obtain ⟨_,final,planned,_⟩ := valid.2 source present
  rw [head] at planned
  cases planned with
  | cons fits ran tail => exact ⟨_,ran⟩

-- Both the source phase and the actual core/image alternative come from the
-- SAME guarded history. Idleness/freshness/room/auth are not inferred here.
theorem install_ready
    {p a : Nat} {assigned : Fin p → OwnerEvaluator.Assignment p}
    {scopeId budget : Nat} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
    {s : State p a} {i : Fin p} {revision : Nat} {next : PublicationInput.State p a}
    {owner : OwnerEndpoint.State p a}
    (path : Runs assigned scopeId capacity budget seed s) (clear : s.pending = none)
    (logical : PublicationInput.execute scopeId s.source (.install i revision) = some next)
    (present : (s.owners i).owner = some owner) :
    revision = s.source.publication.barrier.published ∧ owner.fence = revision ∧
      (owner.owner.core.revision < revision ∨
        (owner.owner.core.revision = revision ∧
          owner.owner.core.image = CustodyPublication.image s.source.publication.current)) := by
  have floorHistory := floor_path path
  have sourcePath := (SourceOwnerFloor.reached_invariant floorHistory).sourcePath
  obtain ⟨published,fence⟩ := SourceCurrentFrame.install_current sourcePath logical
  have aligned := SourceOwnerFloor.continuing_floors floorHistory clear i
  have actualFence : owner.fence = revision := by
    simpa [present,OwnerFenceMonitor.floor,fence] using aligned
  have core := reached_invariant path i owner.owner.core.revision owner.owner.core.image
    (by simp [OwnerImageMonitor.project,OwnerImageMonitor.view,present])
  refine ⟨published,actualFence,?_⟩
  by_cases equal : owner.owner.core.revision = revision
  · exact Or.inr ⟨equal,core.2 (equal.trans published)⟩
  · exact Or.inl (by have := core.1; omega)

-- Construct real model installation (new or exact confirmation) and accepted
-- metered notification without assuming physical phase/image readiness or any
-- successful operation. Whole actual driver/vector custody and idle remain
-- independent premises of the eventual physical consumer.
theorem funded_head_install
    {p a : Nat} {assigned : SourceInput.Assignment p a} {scopeId budget : Nat}
    {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a} {s : State p a}
    {bootstrap : SourceInput.Bootstrap a} {old : PublicationCapacityDriver.State p a}
    {target : Fin p} {revision : Nat} {rest : List (PublicationInput.Command p a)}
    {before : Vector (Fin 513) p} {owner : OwnerEndpoint.State p a}
    (path : Runs (fun i => ⟨assigned.realm,i⟩) scopeId capacity budget seed s)
    (clear : s.pending = none)
    (valid : PublicationLifecycle.Invariant assigned scopeId bootstrap old)
    (sourceAt : old.source = some s.source) (head : old.suffix = .install target revision :: rest)
    (covered : PublicationOwnerBudget.Covered old.suffix (fun i => (before[i.val]).val))
    (ownerAt : s.owners target = ⟨some owner,(before[target.val]).val⟩)
    (idle : owner.owner.active = none) :
    ∃ (next : PublicationCapacityDriver.State p a) (last : OwnerEndpoint.State p a)
        (after : PublicationInput.State p a),
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scopeId (capacity target) (s.owners target)
        (.owner (.install revision (CustodyPublication.image s.source.publication.current))) =
        (⟨some last,((SourceFundingAdministration.paidVector before target)[target.val]).val⟩,.inl 7) ∧
      last.fence = revision ∧ last.owner.core.revision = revision ∧
      last.owner.core.image = CustodyPublication.image s.source.publication.current ∧
      last.owner.active = none ∧ last.owner.reserved = owner.owner.reserved ∧
      last.owner.core.records = owner.owner.core.records ∧
      SourceFundingInput.execute assigned scopeId bootstrap old
        (SourceFundingAdministration.paidVector before target,.step (.install target revision)) = (next,.accepted) ∧
      next.source = some after ∧ next.suffix = rest ∧
      PublicationLifecycle.Invariant assigned scopeId bootstrap next ∧
      PublicationOwnerBudget.Covered next.suffix
        (fun i => ((SourceFundingAdministration.paidVector before target)[i.val]).val) ∧
      Runs (fun i => ⟨assigned.realm,i⟩) scopeId capacity budget seed
        {ownerResult s target (.owner (.install revision (CustodyPublication.image s.source.publication.current)))
          ⟨some last,((SourceFundingAdministration.paidVector before target)[target.val]).val⟩ (.inl 7)
          with source := after} := by
  obtain ⟨logicalNext,logical⟩ := certified_head_step valid sourceAt head
  have present : (s.owners target).owner = some owner := by rw [ownerAt]
  obtain ⟨published,fence,ready⟩ := install_ready path clear logical present
  obtain ⟨next,last,ownerRan,fenceAt,revisionAt,imageAt,stillIdle,keys,records,sourceRan,tail,preserved,funded⟩ :=
    SourceFundingAdministration.install_then_notify (capacity:=capacity target)
      valid sourceAt head covered idle fence ready
  have actualOwner := ownerRan
  rw [←ownerAt] at actualOwner
  obtain ⟨after,atAfter,actualSource⟩ := SourceOwnerFloor.funded_step_actual valid sourceAt sourceRan
  refine ⟨next,last,after,actualOwner,fenceAt,revisionAt,imageAt,stillIdle,keys,records,
    sourceRan,atAfter,tail,preserved,funded,?_⟩
  have allowed : ImageAllowed s.source (.install revision (CustodyPublication.image s.source.publication.current)) :=
    ⟨published,rfl⟩
  have middlePath := Runs.owner path clear allowed actualOwner
  exact Runs.source (command:=.install target revision) middlePath clear trivial actualSource

#print axioms certified_head_step
#print axioms install_ready
#print axioms funded_head_install

#print axioms image_check_exact
#print axioms owner_observation_preserves
#print axioms source_preserves
#print axioms floor_path
#print axioms owner_preserves
#print axioms freeze_preserves
#print axioms reached_invariant
end MirroreaProofFirst.SourceOwnerImage
