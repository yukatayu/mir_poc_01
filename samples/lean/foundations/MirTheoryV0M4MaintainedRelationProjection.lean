/-!
Mir Theory v0 M4 finite maintained-relation / late-projection fragment.

This self-contained model fixes one finite relation owner, two anchors, one
consumer, and a three-label restriction order.  It proves properties of that
finite fragment only; it does not claim a general Core, save/load, transport,
or rendering theorem.
-/

namespace MirTheoryV0M4MaintainedRelationProjection

inductive Locus where
  | relationOwner
  | consumer
deriving DecidableEq, Repr

/- Domain-neutral object and anchor references.  Scenario-specific vocabulary
   intentionally lives outside this foundation. -/
inductive RefId where
  | subject
  | primaryAnchor
  | fallbackAnchor
deriving DecidableEq, Repr

inductive BindingSlot where
  | primary
  | fallback
deriving DecidableEq, Repr

inductive RestrictionLabel where
  | publicLabel
  | restrictedLabel
  | privateLabel
deriving DecidableEq, Repr

def restrictionRank : RestrictionLabel → Nat
  | .publicLabel => 0
  | .restrictedLabel => 1
  | .privateLabel => 2

/- `candidate` may release `baseline` only when it is at least as restrictive. -/
def atLeastAsRestricted (candidate baseline : RestrictionLabel) : Bool :=
  decide (restrictionRank baseline ≤ restrictionRank candidate)

def mostRestricted (left right : RestrictionLabel) : RestrictionLabel :=
  if restrictionRank left < restrictionRank right then right else left

def greatestRestriction (relation primary fallback : RestrictionLabel) : RestrictionLabel :=
  mostRestricted (mostRestricted relation primary) fallback

structure RelationDef where
  owner : Locus
  subject : RefId
  primaryAnchor : RefId
  fallbackAnchor : RefId
  relativeOffset : Int
  relationLabel : RestrictionLabel
deriving DecidableEq, Repr

/- `BindingState` is semantic state owned by `RelationDef.owner`; the consumer
   receives neither ownership nor an absolute evaluated pose. -/
structure BindingState where
  selected : BindingSlot
  lineage : Nat
  bindingEpoch : Nat
  witnessEpoch : Nat
  activationFrontier : Nat
  primaryAnchorEpoch : Nat
  fallbackAnchorEpoch : Nat
deriving DecidableEq, Repr

structure PresentationSample where
  anchor : RefId
  epoch : Nat
  frontier : Nat
  coordinate : Int
  label : RestrictionLabel
deriving DecidableEq, Repr

/- The finite context carries both admitted anchor samples.  Every sample
   needed by the relation must agree on the one context frontier. -/
structure PresentationContext where
  frontier : Nat
  primary : Option PresentationSample
  fallback : Option PresentationSample
deriving DecidableEq, Repr

def relationAcyclic (relation : RelationDef) : Bool :=
  (relation.subject != relation.primaryAnchor) &&
    (relation.subject != relation.fallbackAnchor)

def selectedAnchor (relation : RelationDef) : BindingSlot → RefId
  | .primary => relation.primaryAnchor
  | .fallback => relation.fallbackAnchor

def selectedAnchorEpoch (binding : BindingState) : BindingSlot → Nat
  | .primary => binding.primaryAnchorEpoch
  | .fallback => binding.fallbackAnchorEpoch

def sampleAt (context : PresentationContext) : BindingSlot → Option PresentationSample
  | .primary => context.primary
  | .fallback => context.fallback

def sampleMatches (anchor : RefId) (epoch frontier : Nat) : Option PresentationSample → Bool
  | none => false
  | some sample =>
      (sample.anchor == anchor) && (sample.epoch == epoch) && (sample.frontier == frontier)

def contextHasSingleFrontier (context : PresentationContext) : Bool :=
  match context.primary, context.fallback with
  | some primary, some fallback =>
      (primary.frontier == context.frontier) && (fallback.frontier == context.frontier)
  | _, _ => false

def coherentContext (relation : RelationDef) (binding : BindingState)
    (context : PresentationContext) : Bool :=
  relationAcyclic relation &&
    (context.frontier == binding.activationFrontier) &&
    contextHasSingleFrontier context &&
    sampleMatches relation.primaryAnchor binding.primaryAnchorEpoch context.frontier context.primary &&
    sampleMatches relation.fallbackAnchor binding.fallbackAnchorEpoch context.frontier context.fallback

inductive RelationMaterialization where
  | publishRelation
  | publishValue
  | adapterStream
deriving DecidableEq, Repr

/- The M4 relation projection selects the M3 `publish-relation` target. -/
def projectedRelationMaterialization : RelationMaterialization := .publishRelation

structure ProjectedRelation where
  slot : BindingSlot
  anchor : RefId
  anchorEpoch : Nat
  primaryAnchor : RefId
  primaryAnchorEpoch : Nat
  fallbackAnchor : RefId
  fallbackAnchorEpoch : Nat
  relativeOffset : Int
  bindingEpoch : Nat
  activationFrontier : Nat
  relationLabel : RestrictionLabel
deriving DecidableEq, Repr

def inputLabel (context : PresentationContext) : BindingSlot → RestrictionLabel
  | .primary => context.primary.map PresentationSample.label |>.getD .privateLabel
  | .fallback => context.fallback.map PresentationSample.label |>.getD .privateLabel

def projectRelation (relation : RelationDef) (binding : BindingState) : Option ProjectedRelation :=
  if relation.owner == .relationOwner && relationAcyclic relation then
    some
      { slot := binding.selected
        anchor := selectedAnchor relation binding.selected
        anchorEpoch := selectedAnchorEpoch binding binding.selected
        primaryAnchor := relation.primaryAnchor
        primaryAnchorEpoch := binding.primaryAnchorEpoch
        fallbackAnchor := relation.fallbackAnchor
        fallbackAnchorEpoch := binding.fallbackAnchorEpoch
        relativeOffset := relation.relativeOffset
        bindingEpoch := binding.bindingEpoch
        activationFrontier := binding.activationFrontier
        relationLabel := relation.relationLabel }
  else
    none

def evaluateAt (slot : BindingSlot) (anchor : RefId) (anchorEpoch frontier : Nat)
    (relativeOffset : Int) (context : PresentationContext) : Option Int :=
  if contextHasSingleFrontier context && (context.frontier == frontier) &&
      sampleMatches anchor anchorEpoch frontier (sampleAt context slot) then
    (sampleAt context slot).map (fun sample => sample.coordinate + relativeOffset)
  else
    none

def coherentProjectedContext (projection : ProjectedRelation)
    (context : PresentationContext) : Bool :=
  (context.frontier == projection.activationFrontier) &&
    contextHasSingleFrontier context &&
    sampleMatches projection.primaryAnchor projection.primaryAnchorEpoch context.frontier context.primary &&
    sampleMatches projection.fallbackAnchor projection.fallbackAnchorEpoch context.frontier context.fallback

def evaluateRelation (relation : RelationDef) (binding : BindingState)
    (context : PresentationContext) : Option Int :=
  if coherentContext relation binding context then
    evaluateAt binding.selected (selectedAnchor relation binding.selected)
      (selectedAnchorEpoch binding binding.selected) binding.activationFrontier
      relation.relativeOffset context
  else
    none

def evaluateProjected (projection : ProjectedRelation)
    (context : PresentationContext) : Option Int :=
  if coherentProjectedContext projection context then
    evaluateAt projection.slot projection.anchor projection.anchorEpoch
      projection.activationFrontier projection.relativeOffset context
  else
    none

/- Semantic invalidation advances at most to the fallback position. -/
def semanticFallback (binding : BindingState) : BindingState :=
  match binding.selected with
  | .primary => { binding with selected := .fallback }
  | .fallback => binding

def slotRank : BindingSlot → Nat
  | .primary => 0
  | .fallback => 1

def freshReacquire (binding : BindingState) (newBindingEpoch newWitnessEpoch : Nat) : BindingState :=
  if decide (binding.bindingEpoch < newBindingEpoch) &&
      decide (binding.witnessEpoch < newWitnessEpoch) then
    { binding with
      selected := .primary
      lineage := binding.lineage + 1
      bindingEpoch := newBindingEpoch
      witnessEpoch := newWitnessEpoch
      activationFrontier := binding.activationFrontier + 1
      primaryAnchorEpoch := newBindingEpoch }
  else
    binding

inductive PresentationResult where
  | rendered
  | gap
deriving DecidableEq, Repr

/- Presentation fallback is explicitly read-side: it returns the unchanged
   semantic binding alongside a local renderer decision. -/
def presentationFallback (binding : BindingState)
    (context : PresentationContext) : BindingState × PresentationResult :=
  (binding, if contextHasSingleFrontier context then .rendered else .gap)

def mutateBinding (actor : Locus) (relation : RelationDef)
    (binding : BindingState) : Option BindingState :=
  if actor == relation.owner then some (semanticFallback binding) else none

def releaseAllowed (relation : RelationDef) (context : PresentationContext)
    (requested : RestrictionLabel) : Bool :=
  atLeastAsRestricted requested
    (greatestRestriction relation.relationLabel (inputLabel context .primary)
      (inputLabel context .fallback))

def derivedLabel (relation : RelationDef) (context : PresentationContext) : RestrictionLabel :=
  greatestRestriction relation.relationLabel (inputLabel context .primary)
    (inputLabel context .fallback)

def canonicalRelation : RelationDef :=
  { owner := .relationOwner
    subject := .subject
    primaryAnchor := .primaryAnchor
    fallbackAnchor := .fallbackAnchor
    relativeOffset := 5
    relationLabel := .restrictedLabel }

def canonicalBinding : BindingState :=
  { selected := .primary
    lineage := 7
    bindingEpoch := 2
    witnessEpoch := 1
    activationFrontier := 10
    primaryAnchorEpoch := 2
    fallbackAnchorEpoch := 3 }

def canonicalPrimary : PresentationSample :=
  { anchor := .primaryAnchor, epoch := 2, frontier := 10, coordinate := 12, label := .restrictedLabel }

def canonicalFallback : PresentationSample :=
  { anchor := .fallbackAnchor, epoch := 3, frontier := 10, coordinate := 7, label := .publicLabel }

def canonicalContext : PresentationContext :=
  { frontier := 10, primary := some canonicalPrimary, fallback := some canonicalFallback }

def canonicalProjection : ProjectedRelation :=
  { slot := .primary
    anchor := .primaryAnchor
    anchorEpoch := 2
    primaryAnchor := .primaryAnchor
    primaryAnchorEpoch := 2
    fallbackAnchor := .fallbackAnchor
    fallbackAnchorEpoch := 3
    relativeOffset := 5
    bindingEpoch := 2
    activationFrontier := 10
    relationLabel := .restrictedLabel }

def splitFrameContext : PresentationContext :=
  { frontier := 10
    primary := some canonicalPrimary
    fallback := some { canonicalFallback with frontier := 11 } }

def staleAnchorContext : PresentationContext :=
  { frontier := 10
    primary := some { canonicalPrimary with epoch := 1 }
    fallback := some canonicalFallback }

def privateInputContext : PresentationContext :=
  { frontier := 10
    primary := some { canonicalPrimary with label := .privateLabel }
    fallback := some canonicalFallback }

def cyclicRelation : RelationDef :=
  { canonicalRelation with primaryAnchor := .subject }

def canonicalFallbackBinding : BindingState :=
  { canonicalBinding with selected := .fallback }

theorem canonical_context_is_coherent :
    coherentContext canonicalRelation canonicalBinding canonicalContext = true := by
  rfl

theorem projection_materializes_a_relation_only :
    projectedRelationMaterialization = .publishRelation := by
  rfl

theorem projection_of_canonical_relation_is_relation_not_value :
    projectRelation canonicalRelation canonicalBinding = some canonicalProjection := by
  rfl

theorem project_then_evaluate_equals_evaluate_relation :
    evaluateProjected canonicalProjection canonicalContext =
      evaluateRelation canonicalRelation canonicalBinding canonicalContext := by
  rfl

theorem relative_offset_is_preserved_by_projection :
    evaluateProjected canonicalProjection canonicalContext = some 17 := by
  rfl

theorem semantic_fallback_is_monotone (binding : BindingState) :
    slotRank binding.selected ≤ slotRank (semanticFallback binding).selected := by
  cases binding with
  | mk selected lineage bindingEpoch witnessEpoch activationFrontier primaryAnchorEpoch fallbackAnchorEpoch =>
      cases selected <;> simp [semanticFallback, slotRank]

theorem semantic_fallback_has_no_auto_repromotion :
    (semanticFallback canonicalFallbackBinding).selected = .fallback := by
  rfl

theorem fresh_reacquire_starts_a_new_lineage :
    let reacquired := freshReacquire canonicalFallbackBinding 3 2
    reacquired.selected = .primary ∧
      reacquired.lineage = canonicalFallbackBinding.lineage + 1 ∧
      reacquired.bindingEpoch = 3 ∧ reacquired.witnessEpoch = 2 := by
  exact ⟨rfl, rfl, rfl, rfl⟩

theorem nonfresh_reacquire_does_not_repromote :
    freshReacquire canonicalFallbackBinding 2 2 = canonicalFallbackBinding := by
  rfl

theorem presentation_gap_does_not_mutate_semantic_binding :
    presentationFallback canonicalBinding splitFrameContext = (canonicalBinding, .gap) := by
  rfl

theorem presentation_fallback_preserves_binding (binding : BindingState)
    (context : PresentationContext) :
    (presentationFallback binding context).1 = binding := by
  rfl

theorem stale_anchor_is_rejected :
    evaluateRelation canonicalRelation canonicalBinding staleAnchorContext = none := by
  rfl

theorem projected_relation_rejects_stale_context :
    evaluateProjected canonicalProjection staleAnchorContext = none := by
  rfl

theorem split_frame_is_rejected :
    evaluateRelation canonicalRelation canonicalBinding splitFrameContext = none := by
  rfl

theorem projected_relation_rejects_split_context :
    evaluateProjected canonicalProjection splitFrameContext = none := by
  rfl

theorem relation_cycle_is_rejected :
    projectRelation cyclicRelation canonicalBinding = none := by
  rfl

theorem consumer_cannot_mutate_relation_binding :
    mutateBinding .consumer canonicalRelation canonicalBinding = none := by
  rfl

theorem derived_label_is_the_greatest_restriction :
    greatestRestriction .restrictedLabel .publicLabel .privateLabel = .privateLabel := by
  rfl

theorem canonical_derived_label_is_restricted :
    derivedLabel canonicalRelation canonicalContext = .restrictedLabel := by
  rfl

theorem derived_label_dominates_each_input (relation primary fallback : RestrictionLabel) :
    atLeastAsRestricted (greatestRestriction relation primary fallback) relation = true ∧
      atLeastAsRestricted (greatestRestriction relation primary fallback) primary = true ∧
      atLeastAsRestricted (greatestRestriction relation primary fallback) fallback = true := by
  cases relation <;> cases primary <;> cases fallback <;> decide

theorem private_input_cannot_be_released_as_public :
    releaseAllowed canonicalRelation privateInputContext .publicLabel = false := by
  rfl

/- Trusted-kernel evidence for the five ledger obligations.  These commands
   remain in the executable foundation so `lean --trust=0` reports precisely
   which standard Lean axioms, if any, each finite statement depends on. -/
#print axioms project_then_evaluate_equals_evaluate_relation
#print axioms semantic_fallback_is_monotone
#print axioms presentation_gap_does_not_mutate_semantic_binding
#print axioms derived_label_dominates_each_input
#print axioms relation_cycle_is_rejected

end MirTheoryV0M4MaintainedRelationProjection
