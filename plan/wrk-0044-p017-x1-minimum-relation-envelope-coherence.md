# WRK-0044 P017 X1 minimum relation-envelope coherence

## Evidence role

This is the sole Markdown-held Lean source permitted by WRK-0044. It is a
candidate-local static presentation at the pinned cut, not a Mir relation
schema, dynamic state model, identity scheme, transition system, validator,
SaveObject representation, runtime component, or public interface.

`Pre`, `Rest`, and the binding, result, failure, claim, provenance, and use
types below are universally quantified opaque carriers. The source does not
declare a finite carrier, constructor, key, lookup, equality decision, record,
state-valued field, map, or restore function. The ten named witnesses are
explicit `H_nonvacuity` instances, rather than an exhaustive domain claim. The
five displayed restore facts are a candidate-local correspondence only; they
do not assert global functionality, totality, injectivity, surjectivity, or
cross-load equality.

The five witness valuations are intentionally independent facts, not stages of
one request: emitted/service-pending; typed owner failure; owner
success/result/receipt-pending; accepted/unconsumed; and accepted/consumed.
The selected external-rejection hypothesis is represented only by the absence
of any semantic rejection declaration: raw delivery remains outside this
source's semantic vocabulary and has no persistent state or typed failure.

## Static causal traceability

No causal order, occurrence, transition, or generated edge is declared in the
Lean block. If a later ordinary design relies on an order, its only candidate
mapping is to the existing theory/04 family below; this table itself creates no
order and proves no reachability.

| Conditional fact family | Existing theory/04 generator family |
| --- | --- |
| request emission to owner service | `send -> receive` |
| capability, witness, authentication-evidence, and membership lineage to validated service | `capability_grant -> capability_use`; `witness_create -> witness_use`; `auth_evidence_create -> use`; `membership_update -> dependent dispatch` |
| owner result or reply fact to semantic receipt | `send -> receive` |
| receipt acceptance to restricted-consumption enabling | `state_dependency_order` |
| consumption to a later dependent occurrence | `state_dependency_order` |

This source introduces no observer projection, export, source syntax,
elaboration, transport object, runtime object, helper, `SaveObject`, or
serialization claim. Its absence audit is a negative scope account only.

## Lean source

```lean
namespace P017X1MinimumRelationEnvelopeLab

variable
  {Pre Rest PreBinding RestBinding : Type}
  {PreFailure RestFailure PreResult RestResult : Type}
  {M1Claim Provenance PreUse RestUse : Type}

variable
  (preEmitted preFailure preReceiptPending preAccepted preConsumed : Pre)
  (restEmitted restFailure restReceiptPending restAccepted restConsumed : Rest)
  (preEmittedBinding preFailureBinding preReceiptPendingBinding
    preAcceptedBinding preConsumedBinding : PreBinding)
  (restEmittedBinding restFailureBinding restReceiptPendingBinding
    restAcceptedBinding restConsumedBinding : RestBinding)
  (preFailureValue : PreFailure)
  (restFailureValue : RestFailure)
  (preReceiptPendingResult preAcceptedResult preConsumedResult : PreResult)
  (restReceiptPendingResult restAcceptedResult restConsumedResult : RestResult)
  (preClaim restClaim : M1Claim)
  (preProvenance restProvenance : Provenance)
  (preConsumedUse : PreUse)
  (restConsumedUse : RestUse)

variable
  (preEmitted_ne_failure : preEmitted = preFailure -> False)
  (preEmitted_ne_receiptPending : preEmitted = preReceiptPending -> False)
  (preEmitted_ne_accepted : preEmitted = preAccepted -> False)
  (preEmitted_ne_consumed : preEmitted = preConsumed -> False)
  (preFailure_ne_receiptPending : preFailure = preReceiptPending -> False)
  (preFailure_ne_accepted : preFailure = preAccepted -> False)
  (preFailure_ne_consumed : preFailure = preConsumed -> False)
  (preReceiptPending_ne_accepted : preReceiptPending = preAccepted -> False)
  (preReceiptPending_ne_consumed : preReceiptPending = preConsumed -> False)
  (preAccepted_ne_consumed : preAccepted = preConsumed -> False)
  (restEmitted_ne_failure : restEmitted = restFailure -> False)
  (restEmitted_ne_receiptPending : restEmitted = restReceiptPending -> False)
  (restEmitted_ne_accepted : restEmitted = restAccepted -> False)
  (restEmitted_ne_consumed : restEmitted = restConsumed -> False)
  (restFailure_ne_receiptPending : restFailure = restReceiptPending -> False)
  (restFailure_ne_accepted : restFailure = restAccepted -> False)
  (restFailure_ne_consumed : restFailure = restConsumed -> False)
  (restReceiptPending_ne_accepted : restReceiptPending = restAccepted -> False)
  (restReceiptPending_ne_consumed : restReceiptPending = restConsumed -> False)
  (restAccepted_ne_consumed : restAccepted = restConsumed -> False)

variable
  (prePending : Pre -> PreBinding -> Prop)
  (restPending : Rest -> RestBinding -> Prop)
  (prePendingDoesNotShare :
    forall {q q' : Pre} {b : PreBinding},
      prePending q b -> prePending q' b -> q = q')
  (restPendingDoesNotShare :
    forall {q q' : Rest} {b : RestBinding},
      restPending q b -> restPending q' b -> q = q')
  (preEmittedBindingFact : prePending preEmitted preEmittedBinding)
  (preFailureBindingFact : prePending preFailure preFailureBinding)
  (preReceiptPendingBindingFact : prePending preReceiptPending preReceiptPendingBinding)
  (preAcceptedBindingFact : prePending preAccepted preAcceptedBinding)
  (preConsumedBindingFact : prePending preConsumed preConsumedBinding)
  (restEmittedBindingFact : restPending restEmitted restEmittedBinding)
  (restFailureBindingFact : restPending restFailure restFailureBinding)
  (restReceiptPendingBindingFact : restPending restReceiptPending restReceiptPendingBinding)
  (restAcceptedBindingFact : restPending restAccepted restAcceptedBinding)
  (restConsumedBindingFact : restPending restConsumed restConsumedBinding)
  (preEmittedBindingUnique :
    forall b : PreBinding, prePending preEmitted b -> b = preEmittedBinding)
  (preFailureBindingUnique :
    forall b : PreBinding, prePending preFailure b -> b = preFailureBinding)
  (preReceiptPendingBindingUnique :
    forall b : PreBinding,
      prePending preReceiptPending b -> b = preReceiptPendingBinding)
  (preAcceptedBindingUnique :
    forall b : PreBinding, prePending preAccepted b -> b = preAcceptedBinding)
  (preConsumedBindingUnique :
    forall b : PreBinding, prePending preConsumed b -> b = preConsumedBinding)
  (restEmittedBindingUnique :
    forall b : RestBinding, restPending restEmitted b -> b = restEmittedBinding)
  (restFailureBindingUnique :
    forall b : RestBinding, restPending restFailure b -> b = restFailureBinding)
  (restReceiptPendingBindingUnique :
    forall b : RestBinding,
      restPending restReceiptPending b -> b = restReceiptPendingBinding)
  (restAcceptedBindingUnique :
    forall b : RestBinding, restPending restAccepted b -> b = restAcceptedBinding)
  (restConsumedBindingUnique :
    forall b : RestBinding, restPending restConsumed b -> b = restConsumedBinding)

variable
  (emittedPre servicePendingPre : Pre -> Prop)
  (failurePre : Pre -> PreFailure -> Prop)
  (successPre resultAvailablePre : Pre -> PreResult -> Prop)
  (receiptPendingPre receiptAcceptedPre : Pre -> Prop)
  (consumedPre : Pre -> PreUse -> Prop)
  (emittedRest servicePendingRest : Rest -> Prop)
  (failureRest : Rest -> RestFailure -> Prop)
  (successRest resultAvailableRest : Rest -> RestResult -> Prop)
  (receiptPendingRest receiptAcceptedRest : Rest -> Prop)
  (consumedRest : Rest -> RestUse -> Prop)
  (m1ClaimPre : Pre -> M1Claim -> Prop)
  (provenancePre : Pre -> Provenance -> Prop)
  (m1ClaimRest : Rest -> M1Claim -> Prop)
  (provenanceRest : Rest -> Provenance -> Prop)
  (adverseAuthorityPre noOwnerMutationPre : Pre -> Prop)
  (adverseAuthorityRest noOwnerMutationRest : Rest -> Prop)

variable
  (preEmittedFact : emittedPre preEmitted)
  (preServicePendingFact : servicePendingPre preEmitted)
  (preFailureFact : failurePre preFailure preFailureValue)
  (preFailureClaimFact : m1ClaimPre preFailure preClaim)
  (preFailureProvenanceFact : provenancePre preFailure preProvenance)
  (preAdverseAuthorityFact : adverseAuthorityPre preFailure)
  (preNoOwnerMutationFact : noOwnerMutationPre preFailure)
  (preReceiptPendingSuccessFact : successPre preReceiptPending preReceiptPendingResult)
  (preReceiptPendingResultFact : resultAvailablePre preReceiptPending preReceiptPendingResult)
  (preReceiptPendingFact : receiptPendingPre preReceiptPending)
  (preReceiptPendingNotAccepted : receiptAcceptedPre preReceiptPending -> False)
  (preAcceptedSuccessFact : successPre preAccepted preAcceptedResult)
  (preAcceptedResultFact : resultAvailablePre preAccepted preAcceptedResult)
  (preAcceptedFact : receiptAcceptedPre preAccepted)
  (preAcceptedHasNoConsumption :
    forall u : PreUse, consumedPre preAccepted u -> False)
  (preConsumedSuccessFact : successPre preConsumed preConsumedResult)
  (preConsumedResultFact : resultAvailablePre preConsumed preConsumedResult)
  (preConsumedAcceptedFact : receiptAcceptedPre preConsumed)
  (preConsumedUseFact : consumedPre preConsumed preConsumedUse)
  (restEmittedFact : emittedRest restEmitted)
  (restServicePendingFact : servicePendingRest restEmitted)
  (restFailureFact : failureRest restFailure restFailureValue)
  (restFailureClaimFact : m1ClaimRest restFailure restClaim)
  (restFailureProvenanceFact : provenanceRest restFailure restProvenance)
  (restAdverseAuthorityFact : adverseAuthorityRest restFailure)
  (restNoOwnerMutationFact : noOwnerMutationRest restFailure)
  (restReceiptPendingSuccessFact : successRest restReceiptPending restReceiptPendingResult)
  (restReceiptPendingResultFact : resultAvailableRest restReceiptPending restReceiptPendingResult)
  (restReceiptPendingFact : receiptPendingRest restReceiptPending)
  (restReceiptPendingNotAccepted : receiptAcceptedRest restReceiptPending -> False)
  (restAcceptedSuccessFact : successRest restAccepted restAcceptedResult)
  (restAcceptedResultFact : resultAvailableRest restAccepted restAcceptedResult)
  (restAcceptedFact : receiptAcceptedRest restAccepted)
  (restAcceptedHasNoConsumption :
    forall u : RestUse, consumedRest restAccepted u -> False)
  (restConsumedSuccessFact : successRest restConsumed restConsumedResult)
  (restConsumedResultFact : resultAvailableRest restConsumed restConsumedResult)
  (restConsumedAcceptedFact : receiptAcceptedRest restConsumed)
  (restConsumedUseFact : consumedRest restConsumed restConsumedUse)

variable
  (preTerminalExclusive :
    forall {q : Pre} {f : PreFailure} {r : PreResult},
      failurePre q f -> successPre q r -> False)
  (restTerminalExclusive :
    forall {q : Rest} {f : RestFailure} {r : RestResult},
      failureRest q f -> successRest q r -> False)
  (preConsumptionUnique :
    forall {q : Pre} {u u' : PreUse},
      consumedPre q u -> consumedPre q u' -> u = u')
  (restConsumptionUnique :
    forall {q : Rest} {u u' : RestUse},
      consumedRest q u -> consumedRest q u' -> u = u')

variable
  (restores : Pre -> PreBinding -> Rest -> RestBinding -> Prop)
  (restoreEmitted :
    restores preEmitted preEmittedBinding restEmitted restEmittedBinding)
  (restoreFailure :
    restores preFailure preFailureBinding restFailure restFailureBinding)
  (restoreReceiptPending :
    restores preReceiptPending preReceiptPendingBinding
      restReceiptPending restReceiptPendingBinding)
  (restoreAccepted :
    restores preAccepted preAcceptedBinding restAccepted restAcceptedBinding)
  (restoreConsumed :
    restores preConsumed preConsumedBinding restConsumed restConsumedBinding)

theorem emitted_binding_exists_at_named_witness
    (namedBinding : prePending preEmitted preEmittedBinding) :
    prePending preEmitted preEmittedBinding :=
  namedBinding

theorem emitted_binding_is_unique
    (unique :
      forall b : PreBinding, prePending preEmitted b -> b = preEmittedBinding) :
    forall b : PreBinding, prePending preEmitted b -> b = preEmittedBinding :=
  unique

theorem named_pre_bindings_do_not_share :
    (forall {q q' : Pre} {b : PreBinding},
      prePending q b -> prePending q' b -> q = q') ->
    forall {q q' : Pre} {b : PreBinding},
      prePending q b -> prePending q' b -> q = q' := by
  intro doesNotShare
  exact doesNotShare

theorem emitted_and_failure_bindings_are_separate :
    prePending preEmitted preEmittedBinding ->
    prePending preFailure preFailureBinding ->
    (forall {q q' : Pre} {b : PreBinding},
      prePending q b -> prePending q' b -> q = q') ->
    (preEmitted = preFailure -> False) ->
    preEmittedBinding = preFailureBinding -> False := by
  intro emittedBinding failureBinding doesNotShare anchorSeparate bindingEqual
  apply anchorSeparate
  apply doesNotShare emittedBinding
  cases bindingEqual
  exact failureBinding

theorem failure_is_not_owner_success :
    failurePre preFailure preFailureValue ->
    (forall {q : Pre} {f : PreFailure} {r : PreResult},
      failurePre q f -> successPre q r -> False) ->
    successPre preFailure preReceiptPendingResult -> False := by
  intro failed terminalExclusive
  intro failed_success
  exact terminalExclusive failed failed_success

theorem accepted_witness_has_no_consumption :
    (forall u : PreUse, consumedPre preAccepted u -> False) ->
    forall u : PreUse, consumedPre preAccepted u -> False := by
  intro noConsumption
  exact noConsumption

theorem consumed_witness_has_at_most_one_named_use :
    consumedPre preConsumed preConsumedUse ->
    (forall {q : Pre} {u u' : PreUse},
      consumedPre q u -> consumedPre q u' -> u = u') ->
    forall u : PreUse, consumedPre preConsumed u -> u = preConsumedUse := by
  intro namedUse unique
  intro u consumed
  exact unique consumed namedUse

theorem five_displayed_restore_pairs :
    restores preEmitted preEmittedBinding restEmitted restEmittedBinding ->
    restores preFailure preFailureBinding restFailure restFailureBinding ->
    restores preReceiptPending preReceiptPendingBinding
      restReceiptPending restReceiptPendingBinding ->
    restores preAccepted preAcceptedBinding restAccepted restAcceptedBinding ->
    restores preConsumed preConsumedBinding restConsumed restConsumedBinding ->
    restores preEmitted preEmittedBinding restEmitted restEmittedBinding /\
    restores preFailure preFailureBinding restFailure restFailureBinding /\
    restores preReceiptPending preReceiptPendingBinding
      restReceiptPending restReceiptPendingBinding /\
    restores preAccepted preAcceptedBinding restAccepted restAcceptedBinding /\
    restores preConsumed preConsumedBinding restConsumed restConsumedBinding :=
  fun emitted failure receiptPending accepted consumed =>
    And.intro emitted
      (And.intro failure
        (And.intro receiptPending
          (And.intro accepted consumed)))

theorem emitted_and_failure_witnesses_are_separate :
    (preEmitted = preFailure -> False) ->
    preEmitted = preFailure -> False := by
  intro separate
  exact separate

theorem restored_accepted_and_consumed_witnesses_are_separate :
    (restAccepted = restConsumed -> False) ->
    restAccepted = restConsumed -> False := by
  intro separate
  exact separate

theorem restored_consumed_witness_has_at_most_one_named_use :
    consumedRest restConsumed restConsumedUse ->
    (forall {q : Rest} {u u' : RestUse},
      consumedRest q u -> consumedRest q u' -> u = u') ->
    forall u : RestUse, consumedRest restConsumed u -> u = restConsumedUse := by
  intro namedUse unique
  intro u consumed
  exact unique consumed namedUse

end P017X1MinimumRelationEnvelopeLab
```

## Bound of the result

The block proves only consequences of the explicit candidate-local premises.
It establishes neither a complete request domain nor a five-state lifecycle,
frontier reachability, semantic receipt transition, real validation,
fail-closed behavior, storage placement, `SaveObject` closure, load
admissibility, a global restore property, one-shot behavior over executions or
continuations, causal acyclicity, an observation policy, a theorem/OBL result,
implementation readiness, or public behavior.
