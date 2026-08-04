/-!
Mir Theory v0 M3 finite evaluation/materialization fragment.

This is a finite, self-contained model of the M3 carrier.  It proves facts
about this declared fragment only; it does not claim a general distributed
runtime, relation projection, save/load, or patch theorem.
-/

namespace MirTheoryV0M3EvaluationMaterialization

inductive Locus where
  | owner
  | foreign
  | evaluator
deriving DecidableEq, Repr

inductive Principal where
  | attacker
  | viewer
deriving DecidableEq, Repr

inductive Provider where
  | renderer
deriving DecidableEq, Repr

inductive SemanticForm where
  | value
  | state
  | relation
  | computation
deriving DecidableEq, Repr

inductive EvaluationSite where
  | owner (locus : Locus)
  | locus (locus : Locus)
  | designated (locus : Locus)
  | consumer (principal : Principal)
  | provider (provider : Provider)
deriving DecidableEq, Repr

inductive TriggerClock where
  | onRequest
  | onEvent
  | onChange
  | logicalTick
  | frontierAdvance
  | presentationFrame
  | explicit
deriving DecidableEq, Repr

inductive AuthorityOrigin where
  | caller (principal : Principal)
  | ownerTransition (locus : Locus)
  | admittedEvaluator (locus : Locus)
  | admittedProvider (provider : Provider)
deriving DecidableEq, Repr

inductive Materialization where
  | localOnly
  | store
  | publishValue
  | publishRelation
  | adapterStream
  | persist
deriving DecidableEq, Repr

inductive EvaluationPolicy where
  | deterministic
deriving DecidableEq, Repr

inductive ObservationPolicy where
  | ownerOnly
  | authorized
  | localOnly
deriving DecidableEq, Repr

/- A frontier is stored as a canonical finite producer list.  `normalizeFrontier`
sorts and deduplicates it, so permutations denote the same semantic set. -/
abbrev Frontier := List Nat

def insertFrontier (producer : Nat) : Frontier → Frontier
  | [] => [producer]
  | existing :: rest =>
      if producer < existing then producer :: existing :: rest
      else if producer == existing then existing :: rest
      else existing :: insertFrontier producer rest

def normalizeFrontier (frontier : Frontier) : Frontier :=
  frontier.foldr insertFrontier []

def canonicalTargets (targets : List Materialization) : Prop :=
  targets ≠ [] ∧ targets.Nodup ∧
  (Materialization.localOnly ∈ targets → targets = [Materialization.localOnly]) ∧
  (Materialization.publishValue ∈ targets → Materialization.publishRelation ∉ targets) ∧
  (Materialization.publishRelation ∈ targets → Materialization.publishValue ∉ targets) ∧
  (Materialization.adapterStream ∈ targets → Materialization.persist ∉ targets) ∧
  (Materialization.persist ∈ targets → Materialization.adapterStream ∉ targets)

structure EvalPlan where
  originKey : Nat
  form : SemanticForm
  site : EvaluationSite
  trigger : TriggerClock
  authority : AuthorityOrigin
  targets : List Materialization
  policy : EvaluationPolicy
  observationPolicy : ObservationPolicy
  frontier : Option Frontier
deriving DecidableEq, Repr

def ownerRmwPlan : EvalPlan :=
  { originKey := 101
    form := .state
    site := .owner .owner
    trigger := .onRequest
    authority := .caller .attacker
    targets := [.store]
    policy := .deterministic
    observationPolicy := .ownerOnly
    frontier := none }

def designatedPlan (frontier : Frontier) : EvalPlan :=
  { originKey := 200 + (normalizeFrontier frontier).length
    form := .computation
    site := .designated .evaluator
    trigger := .logicalTick
    authority := .admittedEvaluator .evaluator
    targets := [.publishValue]
    policy := .deterministic
    observationPolicy := .authorized
    frontier := some (normalizeFrontier frontier) }

def consumerPlan : EvalPlan :=
  { originKey := 301
    form := .relation
    site := .consumer .viewer
    trigger := .presentationFrame
    authority := .caller .viewer
    targets := [.localOnly]
    policy := .deterministic
    observationPolicy := .localOnly
    frontier := none }

def providerPlan : EvalPlan :=
  { originKey := 401
    form := .computation
    site := .provider .renderer
    trigger := .logicalTick
    authority := .admittedProvider .renderer
    targets := [.adapterStream]
    policy := .deterministic
    observationPolicy := .authorized
    frontier := none }

def validPlan (plan : EvalPlan) : Prop :=
  canonicalTargets plan.targets ∧
  match plan.site with
  | .owner _ => plan.form = .state ∧ plan.trigger = .onRequest ∧
      Materialization.store ∈ plan.targets
  | .designated _ => plan.frontier.isSome ∧ Materialization.publishValue ∈ plan.targets
  | .consumer _ => plan.targets = [.localOnly]
  | .provider _ => plan.targets = [.localOnly] ∨ plan.targets = [.adapterStream]
  | .locus _ => plan.targets = [.localOnly]

inductive ReceiptStatus where
  | success (value : Int)
  | failure
deriving DecidableEq, Repr

structure Receipt where
  caller : Principal
  producer : Locus
  target : Locus
  requestOccurrence : Nat
  serveOccurrence : Nat
  replyOccurrence : Nat
  receiveOccurrence : Nat
  frontier : Frontier
  label : Nat
  releaseAdmitted : Bool
  status : ReceiptStatus
deriving DecidableEq, Repr

def usableReceipt (target : Locus) (receipt : Receipt) : Bool :=
  (receipt.target == target) && (0 < receipt.requestOccurrence) &&
  (receipt.requestOccurrence < receipt.serveOccurrence) &&
  (receipt.serveOccurrence < receipt.replyOccurrence) &&
  (receipt.replyOccurrence < receipt.receiveOccurrence) &&
  receipt.releaseAdmitted &&
  match receipt.status with
  | .success _ => true
  | .failure => false

inductive Input where
  | sameOwnerRmw
  | crossOwnerUnannotated
  | crossOwnerReceipt (receipt : Receipt)
  | designated (frontier : Frontier)
  | consumerRelation
  | providerComputation
  | ambiguous
deriving DecidableEq, Repr

inductive Diagnostic where
  | crossOwnerOperand
  | ambiguousEvaluation
  | invalidReceipt
deriving DecidableEq, Repr

inductive Elaborated where
  | accepted (plan : EvalPlan)
  | rejected (diagnostic : Diagnostic)
deriving DecidableEq, Repr

def elaborate : Input → Elaborated
  | .sameOwnerRmw => .accepted ownerRmwPlan
  | .crossOwnerUnannotated => .rejected .crossOwnerOperand
  | .crossOwnerReceipt receipt =>
      if usableReceipt .owner receipt then .accepted ownerRmwPlan else .rejected .invalidReceipt
  | .designated frontier => .accepted (designatedPlan frontier)
  | .consumerRelation => .accepted consumerPlan
  | .providerComputation => .accepted providerPlan
  | .ambiguous => .rejected .ambiguousEvaluation

theorem elaboration_deterministic {input : Input} {first second : Elaborated}
    (firstResult : elaborate input = first)
    (secondResult : elaborate input = second) :
    first = second := by
  exact firstResult.symm.trans secondResult

theorem unannotated_cross_owner_is_rejected :
    elaborate .crossOwnerUnannotated = .rejected .crossOwnerOperand := by
  rfl

theorem failed_receipt_is_not_an_operand :
    elaborate (.crossOwnerReceipt
      { caller := .attacker, producer := .foreign, target := .owner, requestOccurrence := 1,
        serveOccurrence := 2, replyOccurrence := 3, receiveOccurrence := 4,
        frontier := [1], label := 0, releaseAdmitted := true,
        status := .failure }) = .rejected .invalidReceipt := by
  rfl

theorem successful_receipt_is_explicit :
    elaborate (.crossOwnerReceipt
      { caller := .attacker, producer := .foreign, target := .owner, requestOccurrence := 1,
        serveOccurrence := 2, replyOccurrence := 3, receiveOccurrence := 4,
        frontier := [1], label := 0, releaseAdmitted := true,
        status := .success 7 }) = .accepted ownerRmwPlan := by
  rfl

theorem receipt_with_wrong_target_is_rejected :
    elaborate (.crossOwnerReceipt
      { caller := .attacker, producer := .foreign, target := .foreign, requestOccurrence := 1,
        serveOccurrence := 2, replyOccurrence := 3, receiveOccurrence := 4,
        frontier := [1], label := 0, releaseAdmitted := true,
        status := .success 7 }) = .rejected .invalidReceipt := by
  rfl

theorem receipt_without_admitted_release_is_rejected :
    elaborate (.crossOwnerReceipt
      { caller := .attacker, producer := .foreign, target := .owner, requestOccurrence := 1,
        serveOccurrence := 2, replyOccurrence := 3, receiveOccurrence := 4,
        frontier := [1], label := 0, releaseAdmitted := false,
        status := .success 7 }) = .rejected .invalidReceipt := by
  rfl

theorem receipt_without_complete_causal_chain_is_rejected :
    elaborate (.crossOwnerReceipt
      { caller := .attacker, producer := .foreign, target := .owner, requestOccurrence := 1,
        serveOccurrence := 3, replyOccurrence := 2, receiveOccurrence := 4,
        frontier := [1], label := 0, releaseAdmitted := true,
        status := .success 7 }) = .rejected .invalidReceipt := by
  rfl

theorem owner_rmw_plan_is_valid : validPlan ownerRmwPlan := by
  simp [validPlan, canonicalTargets, ownerRmwPlan]

theorem designated_plan_is_valid (frontier : Frontier) :
    validPlan (designatedPlan frontier) := by
  simp [validPlan, canonicalTargets, designatedPlan]

theorem consumer_plan_cannot_materialize_semantic_state :
    consumerPlan.targets = [.localOnly] := by
  rfl

theorem provider_plan_cannot_materialize_semantic_state :
    providerPlan.targets = [.adapterStream] := by
  rfl

structure OwnerState where
  hp : Int
  resultVersion : Nat
  published : Bool
deriving DecidableEq, Repr

inductive OwnerRequest where
  | attack (caller : Principal) (hasCapability : Bool)
deriving DecidableEq, Repr

inductive TraceRow where
  | request (plan : EvalPlan) (origin : AuthorityOrigin)
  | dependency (originKey : Nat) (locus : Locus)
  | evaluation (originKey : Nat) (site : EvaluationSite)
  | write (originKey : Nat) (locus : Locus) (after : Int)
  | failure (originKey : Nat) (diagnostic : Diagnostic)
  | receipt (originKey : Nat) (producer target : Locus)
  | publish (originKey version : Nat)
  | consume (originKey version : Nat) (consumer : Principal)
deriving DecidableEq, Repr

def wellFormed (state : OwnerState) : Prop := 0 ≤ state.hp

def serve (state : OwnerState) : OwnerRequest → OwnerState × List TraceRow
  | .attack caller true =>
      if state.hp < 10 then
        (state, [.request ownerRmwPlan (.caller caller),
          .failure ownerRmwPlan.originKey .invalidReceipt])
      else
        ({ state with hp := state.hp - 10 },
          [.request ownerRmwPlan (.caller caller), .dependency ownerRmwPlan.originKey .owner,
           .evaluation ownerRmwPlan.originKey (.owner .owner),
           .write ownerRmwPlan.originKey .owner (state.hp - 10)])
  | .attack caller false =>
      (state, [.request ownerRmwPlan (.caller caller),
        .failure ownerRmwPlan.originKey .invalidReceipt])

def serveAll : OwnerState → List OwnerRequest → OwnerState
  | state, [] => state
  | state, request :: requests => serveAll (serve state request).1 requests

def attack : OwnerRequest := .attack .attacker true
def missingCapabilityAttack : OwnerRequest := .attack .attacker false

theorem accepted_owner_rmw_is_served_at_owner (state : OwnerState) (enoughHp : 10 ≤ state.hp) :
    (serve state attack).2.contains (.evaluation ownerRmwPlan.originKey (.owner .owner)) := by
  have notLess : ¬ state.hp < 10 := by omega
  simp [serve, attack, notLess]

theorem owner_rmw_keeps_caller_authority (state : OwnerState) :
    (serve state attack).2.contains (.request ownerRmwPlan (.caller .attacker)) := by
  by_cases h : state.hp < 10
  · simp [serve, attack, h]
  · simp [serve, attack, h]

theorem two_attacks_are_serial_owner_rmw :
    (serveAll { hp := 100, resultVersion := 0, published := false } [attack, attack]).hp = 80 := by
  rfl

theorem missing_capability_does_not_mutate (state : OwnerState) :
    (serve state missingCapabilityAttack).1 = state := by
  rfl

theorem failed_owner_service_preserves_well_formed (state : OwnerState)
    (wf : wellFormed state) :
    wellFormed (serve state missingCapabilityAttack).1 := by
  exact wf

theorem successful_owner_service_preserves_well_formed (state : OwnerState)
    (wf : wellFormed state) :
    wellFormed (serve state attack).1 := by
  simp only [attack, serve]
  split
  · exact wf
  · dsimp [wellFormed] at wf ⊢
    omega

structure DecidedResult where
  frontier : Frontier
  version : Nat
  value : Int
  policy : EvaluationPolicy
  observationPolicy : ObservationPolicy
  consumed : Bool
deriving DecidableEq, Repr

structure DesignatedStore where
  nextVersion : Nat
  result : Option DecidedResult
deriving DecidableEq, Repr

def decide (store : DesignatedStore) (frontier : Frontier) (value : Int)
    (policy : EvaluationPolicy) (observationPolicy : ObservationPolicy) :
    DesignatedStore × DecidedResult × Bool :=
  match store.result with
  | some result => (store, result, false)
  | none =>
      let result : DecidedResult :=
        { frontier := normalizeFrontier frontier
          version := store.nextVersion + 1
          value := value
          policy := policy
          observationPolicy := observationPolicy
          consumed := false }
      ({ nextVersion := result.version, result := some result }, result, true)

/- The finite M3 profile permits one named semantic consumer for a decided
result.  A repeated or competing consume does not create another consume row. -/
def consume (store : DesignatedStore) (_consumer : Principal) : DesignatedStore :=
  match store.result with
  | none => store
  | some result =>
      if result.consumed then store
      else { store with result := some { result with consumed := true } }

theorem duplicate_designated_decision_is_stable :
    let initial := { nextVersion := 0, result := none }
    let first := decide initial [4, 9] 42 .deterministic .authorized
    let duplicate := decide first.1 [9, 4] 42 .deterministic .authorized
    duplicate.1 = first.1 ∧ duplicate.2.1 = first.2.1 ∧ duplicate.2.2 = false := by
  simp [decide]

theorem failed_designated_input_has_no_success_value :
    elaborate (.crossOwnerReceipt
      { caller := .attacker, producer := .foreign, target := .evaluator, requestOccurrence := 2,
        serveOccurrence := 3, replyOccurrence := 4, receiveOccurrence := 5,
        frontier := [2], label := 0, releaseAdmitted := true,
        status := .failure }) = .rejected .invalidReceipt := by
  rfl

theorem consume_is_idempotent (store : DesignatedStore) :
    consume (consume store .viewer) .viewer = consume store .viewer := by
  cases store with
  | mk nextVersion result =>
      cases result with
      | none => rfl
      | some result =>
          cases consumed : result.consumed <;> simp [consume, consumed]

end MirTheoryV0M3EvaluationMaterialization
