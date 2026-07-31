# WRK-0045 P017 X1 A-Sigma H_K-rs conditional trace

## Evidence role

This is the sole Markdown-held Lean source declared by `working/WRK-0045`.
It is a candidate-local, conditional examination of the pre-registered
`H_K-rs` ledger. It does not define a Mir relation schema, occurrence kind,
transition, receipt carrier, matching identity, persistence format, restore
function, validator, runtime, transport, or public interface.

Every carrier below is an opaque parameter and every presentation is a
Prop-valued predicate. In particular, A-Sigma residence is q/branch-indexed
relation membership rather than a fiber object; the current/restored
correspondences are relations rather than equality or a restoring function.
The source has no imports, constructors, finite carrier, enumeration, key,
lookup, choice, global coherence predicate, or data-returning definition.

The conclusions are conditional. They establish neither a joint Canon model,
an operationally reachable `r`, a semantic receipt transition, an authority
rule, a failure semantics, nor a general restore property. The disposable
execution harnesses, not this source, check that the named premises can have a
finite metalogical interpretation and that removing a premise defeats its
named consumer.

## Lean source

```lean
set_option autoImplicit false

namespace P017X1ASigmaConditionalTraceLab

variable
  {Occurrence Branch Binding Value Use Locus Gamma Delta Claim Ground Failure : Type}
  {Raw Current Restored Cut : Type}

variable
  (prec directSendReceive : Occurrence -> Occurrence -> Prop)
  (requestSend serviceReceive replySend requesterReceive : Occurrence -> Prop)
  (sigmaPending sigmaOutcome sigmaReceipt sigmaAcceptance sigmaUse :
    Occurrence -> Branch -> Prop)
  (pending : Occurrence -> Branch -> Binding -> Locus -> Gamma -> Delta -> Prop)
  (ownerOutstanding : Occurrence -> Branch -> Prop)
  (ownerSuccess : Occurrence -> Branch -> Value -> Ground -> Prop)
  (ownerFailure ownerFailureRow ownerNoMutation : Occurrence -> Branch -> Failure -> Prop)
  (replyResult : Occurrence -> Value -> Ground -> Prop)
  (receipt : Occurrence -> Branch -> Occurrence -> Value -> Ground -> Prop)
  (valueHasExistingType : Value -> Prop)
  (resultGround : Occurrence -> Branch -> Ground -> Prop)
  (m1Claim : Occurrence -> Claim -> Prop)
  (consulted : Claim -> Ground -> Prop)
  (liveAuthority : Occurrence -> Branch -> Prop)
  (capabilityGrant witnessCreate authEvidenceCreate membershipUpdate : Occurrence -> Prop)
  (serviceVisible : Occurrence -> Branch -> Prop)
  (receiptRequest : Occurrence -> Occurrence -> Prop)
  (receiptBranch : Occurrence -> Branch -> Prop)
  (receiptValue : Occurrence -> Value -> Prop)
  (accepted : Occurrence -> Branch -> Occurrence -> Value -> Ground -> Prop)
  (enabled : Occurrence -> Branch -> Occurrence -> Value -> Ground -> Use -> Prop)
  (used : Occurrence -> Branch -> Occurrence -> Value -> Ground -> Use -> Prop)
  (authorityOrder : Occurrence -> Occurrence -> Prop)
  (rawRejected rawNoOccurrence rawNoFailure rawNoReceipt rawNoUse rawNoRestore :
    Raw -> Prop)
  (inCut : Cut -> Occurrence -> Prop)
  (inFlight : Cut -> Occurrence -> Occurrence -> Prop)
  (sigmaCorrespondence : Current -> Restored -> Occurrence -> Branch -> Prop)
  (pendingCorrespondence : Current -> Restored -> Occurrence -> Branch -> Binding -> Prop)
  (failureCorrespondence : Current -> Restored -> Occurrence -> Branch -> Failure -> Prop)
  (resultReceiptCorrespondence :
    Current -> Restored -> Occurrence -> Branch -> Occurrence -> Value -> Ground -> Prop)
  (acceptUseGroundChannelCorrespondence :
    Current -> Restored -> Occurrence -> Branch -> Occurrence -> Value -> Ground -> Use -> Prop)
  (restoredSigmaPending restoredSigmaOutcome restoredSigmaReceipt
    restoredSigmaAcceptance restoredSigmaUse :
    Restored -> Occurrence -> Branch -> Prop)
  (restoredPending : Restored -> Occurrence -> Branch -> Binding -> Prop)
  (restoredFailure : Restored -> Occurrence -> Branch -> Failure -> Prop)
  (restoredResultReceipt :
    Restored -> Occurrence -> Branch -> Occurrence -> Value -> Ground -> Prop)
  (restoredAcceptUseGroundChannel :
    Restored -> Occurrence -> Branch -> Occurrence -> Value -> Ground -> Use -> Prop)
  (emitted : Occurrence -> Prop)

theorem pending_has_one_named_binding_and_no_shared_requester
    {q q' : Occurrence} {b : Branch} {binding : Binding}
    {locus : Locus} {gamma : Gamma} {delta : Delta}
    (h_pending : pending q b binding locus gamma delta)
    (h_binding_unique :
      forall {otherBinding : Binding} {otherLocus : Locus}
        {otherGamma : Gamma} {otherDelta : Delta},
        pending q b otherBinding otherLocus otherGamma otherDelta ->
        otherBinding = binding)
    (h_binding_nonshared :
      forall {otherRequest : Occurrence} {otherBranch : Branch}
        {otherLocus : Locus} {otherGamma : Gamma} {otherDelta : Delta},
        pending q b binding locus gamma delta ->
        pending otherRequest otherBranch binding otherLocus otherGamma otherDelta ->
        otherRequest = q) :
    (exists namedBinding : Binding,
      pending q b namedBinding locus gamma delta /\
      forall {otherBinding : Binding} {otherLocus : Locus}
        {otherGamma : Gamma} {otherDelta : Delta},
        pending q b otherBinding otherLocus otherGamma otherDelta ->
        otherBinding = namedBinding) /\
    (forall {otherBranch : Branch} {otherLocus : Locus}
      {otherGamma : Gamma} {otherDelta : Delta},
      pending q' otherBranch binding otherLocus otherGamma otherDelta -> q' = q) := by
  constructor
  · exact Exists.intro binding (And.intro h_pending h_binding_unique)
  · intro otherBranch otherLocus otherGamma otherDelta h_other
    exact h_binding_nonshared h_pending h_other

theorem sigma_whole_slice_correspondence_keeps_named_q_branch_residence
    {before : Current} {after : Restored} {q : Occurrence} {b : Branch}
    (h_pending_residence : sigmaPending q b)
    (h_outcome_residence : sigmaOutcome q b)
    (h_receipt_residence : sigmaReceipt q b)
    (h_acceptance_residence : sigmaAcceptance q b)
    (h_use_residence : sigmaUse q b)
    (h_correspondence : sigmaCorrespondence before after q b)
    (h_pending_preserved :
      sigmaCorrespondence before after q b -> sigmaPending q b ->
      restoredSigmaPending after q b)
    (h_outcome_preserved :
      sigmaCorrespondence before after q b -> sigmaOutcome q b ->
      restoredSigmaOutcome after q b)
    (h_receipt_preserved :
      sigmaCorrespondence before after q b -> sigmaReceipt q b ->
      restoredSigmaReceipt after q b)
    (h_acceptance_preserved :
      sigmaCorrespondence before after q b -> sigmaAcceptance q b ->
      restoredSigmaAcceptance after q b)
    (h_use_preserved :
      sigmaCorrespondence before after q b -> sigmaUse q b ->
      restoredSigmaUse after q b) :
    restoredSigmaPending after q b /\ restoredSigmaOutcome after q b /\
    restoredSigmaReceipt after q b /\ restoredSigmaAcceptance after q b /\
    restoredSigmaUse after q b := by
  exact And.intro (h_pending_preserved h_correspondence h_pending_residence)
    (And.intro (h_outcome_preserved h_correspondence h_outcome_residence)
      (And.intro (h_receipt_preserved h_correspondence h_receipt_residence)
        (And.intro (h_acceptance_preserved h_correspondence h_acceptance_residence)
          (h_use_preserved h_correspondence h_use_residence))))

theorem direct_legs_form_a_strict_q_s_r_path
    {q s r : Occurrence}
    (h_request_send : requestSend q)
    (h_service_receive : serviceReceive s)
    (h_reply_send : replySend s)
    (h_requester_receive : requesterReceive r)
    (h_sr1 : directSendReceive q s)
    (h_sr2 : directSendReceive s r)
    (h_direct_in_prec :
      forall {left right : Occurrence},
      directSendReceive left right -> prec left right)
    (h_trans : forall {left middle right : Occurrence},
      prec left middle -> prec middle right -> prec left right)
    (h_irrefl : forall point : Occurrence, prec point point -> False) :
    requestSend q /\ serviceReceive s /\ replySend s /\ requesterReceive r /\
    prec q s /\ prec s r /\ prec q r /\
    (q = s -> False) /\ (s = r -> False) /\ (q = r -> False) := by
  have h_qs : prec q s := h_direct_in_prec h_sr1
  have h_sr : prec s r := h_direct_in_prec h_sr2
  have h_qr : prec q r := h_trans h_qs h_sr
  refine And.intro h_request_send (And.intro h_service_receive ?_)
  refine And.intro h_reply_send (And.intro h_requester_receive ?_)
  refine And.intro h_qs (And.intro h_sr (And.intro h_qr ?_))
  constructor
  · intro h_equal
    cases h_equal
    exact h_irrefl q h_qs
  · constructor
    · intro h_equal
      cases h_equal
      exact h_irrefl s h_sr
    · intro h_equal
      cases h_equal
      exact h_irrefl q h_qr

theorem exact_typed_receipt_uses_only_named_matching_laws
    {q s r : Occurrence} {b : Branch} {v : Value} {g : Ground}
    (h_owner_success : ownerSuccess q b v g)
    (h_result_ground : resultGround q b g)
    (h_reply_result : replyResult s v g)
    (h_receipt : receipt q b r v g)
    (h_existing_type : valueHasExistingType v)
    (h_receipt_request : receipt q b r v g -> receiptRequest r q)
    (h_receipt_branch : receipt q b r v g -> receiptBranch r b)
    (h_receipt_value : receipt q b r v g -> receiptValue r v)
    (h_request_functional :
      forall {otherRequest : Occurrence}, receiptRequest r q ->
      receiptRequest r otherRequest -> otherRequest = q)
    (h_branch_functional :
      forall {otherBranch : Branch}, receiptBranch r b ->
      receiptBranch r otherBranch -> otherBranch = b)
    (h_value_functional :
      forall {otherValue : Value}, receiptValue r v ->
      receiptValue r otherValue -> otherValue = v) :
    ownerSuccess q b v g /\ resultGround q b g /\ replyResult s v g /\
    receipt q b r v g /\ valueHasExistingType v /\
    (forall {otherRequest : Occurrence}, receiptRequest r otherRequest ->
      otherRequest = q) /\
    (forall {otherBranch : Branch}, receiptBranch r otherBranch ->
      otherBranch = b) /\
    (forall {otherValue : Value}, receiptValue r otherValue ->
      otherValue = v) := by
  have h_request_match : receiptRequest r q := h_receipt_request h_receipt
  have h_branch_match : receiptBranch r b := h_receipt_branch h_receipt
  have h_value_match : receiptValue r v := h_receipt_value h_receipt
  have h_request_exact :
      forall {otherRequest : Occurrence}, receiptRequest r otherRequest ->
      otherRequest = q := by
    intro otherRequest h_other
    exact h_request_functional h_request_match h_other
  have h_branch_exact :
      forall {otherBranch : Branch}, receiptBranch r otherBranch ->
      otherBranch = b := by
    intro otherBranch h_other
    exact h_branch_functional h_branch_match h_other
  have h_value_exact :
      forall {otherValue : Value}, receiptValue r otherValue ->
      otherValue = v := by
    intro otherValue h_other
    exact h_value_functional h_value_match h_other
  exact And.intro h_owner_success
    (And.intro h_result_ground
      (And.intro h_reply_result
        (And.intro h_receipt
          (And.intro h_existing_type
            (And.intro h_request_exact
              (And.intro h_branch_exact h_value_exact))))))

theorem consulted_m1_ground_is_traceability_not_authority
    {q : Occurrence} {b : Branch} {c : Claim} {g : Ground}
    (h_m1_claim : m1Claim q c)
    (h_consulted : consulted c g)
    (h_result_ground : resultGround q b g)
    (h_not_live_authority : liveAuthority q b -> False) :
    m1Claim q c /\ consulted c g /\ resultGround q b g /\
    (liveAuthority q b -> False) :=
  And.intro h_m1_claim
    (And.intro h_consulted (And.intro h_result_ground h_not_live_authority))

theorem failed_owner_branch_has_no_success_receipt_acceptance_or_use
    {q : Occurrence} {b : Branch} {f : Failure}
    (h_outstanding : ownerOutstanding q b)
    (h_failure : ownerFailure q b f)
    (h_failure_row : ownerFailureRow q b f)
    (h_no_mutation : ownerNoMutation q b f)
    (h_exclusive : forall {v : Value} {g : Ground},
      ownerFailure q b f -> ownerSuccess q b v g -> False)
    (h_receipt_needs_success : forall {r : Occurrence} {v : Value} {g : Ground},
      receipt q b r v g -> ownerSuccess q b v g)
    (h_acceptance_needs_receipt : forall {r : Occurrence} {v : Value} {g : Ground},
      accepted q b r v g -> receipt q b r v g)
    (h_use_needs_acceptance :
      forall {r : Occurrence} {v : Value} {g : Ground} {u : Use},
      used q b r v g u -> accepted q b r v g) :
    ownerOutstanding q b /\ ownerFailure q b f /\ ownerFailureRow q b f /\
    ownerNoMutation q b f /\
    (forall {v : Value} {g : Ground}, ownerSuccess q b v g -> False) /\
    (forall {r : Occurrence} {v : Value} {g : Ground}, receipt q b r v g -> False) /\
    (forall {r : Occurrence} {v : Value} {g : Ground}, accepted q b r v g -> False) /\
    (forall {r : Occurrence} {v : Value} {g : Ground} {u : Use},
      used q b r v g u -> False) := by
  have h_no_success :
      forall {v : Value} {g : Ground}, ownerSuccess q b v g -> False := by
    intro v g h_success
    exact h_exclusive h_failure h_success
  have h_no_receipt :
      forall {r : Occurrence} {v : Value} {g : Ground}, receipt q b r v g -> False := by
    intro r v g h_receipt
    exact h_no_success (h_receipt_needs_success h_receipt)
  have h_no_acceptance :
      forall {r : Occurrence} {v : Value} {g : Ground}, accepted q b r v g -> False := by
    intro r v g h_accepted
    exact h_no_receipt (h_acceptance_needs_receipt h_accepted)
  have h_no_use :
      forall {r : Occurrence} {v : Value} {g : Ground} {u : Use},
      used q b r v g u -> False := by
    intro r v g u h_used
    exact h_no_acceptance (h_use_needs_acceptance h_used)
  exact And.intro h_outstanding
    (And.intro h_failure
      (And.intro h_failure_row
        (And.intro h_no_mutation
          (And.intro h_no_success
            (And.intro h_no_receipt
              (And.intro h_no_acceptance h_no_use))))))

theorem accepted_receipt_can_remain_unconsumed
    {q r : Occurrence} {b : Branch} {v : Value} {g : Ground}
    (h_accepted : accepted q b r v g)
    (h_no_use : forall u : Use, used q b r v g u -> False) :
    accepted q b r v g /\ forall u : Use, used q b r v g u -> False :=
  And.intro h_accepted h_no_use

theorem accepted_enabled_slot_can_remain_unconsumed
    {q r : Occurrence} {b : Branch} {v : Value} {g : Ground} {u : Use}
    (h_accepted : accepted q b r v g)
    (h_enabled : enabled q b r v g u)
    (h_no_use : forall otherUse : Use, used q b r v g otherUse -> False) :
    accepted q b r v g /\ enabled q b r v g u /\
    forall otherUse : Use, used q b r v g otherUse -> False :=
  And.intro h_accepted (And.intro h_enabled h_no_use)

theorem named_consumption_has_explicit_prerequisites_and_at_most_one_use
    {q r : Occurrence} {b : Branch} {v : Value} {g : Ground} {u : Use}
    (h_used : used q b r v g u)
    (h_use_needs_acceptance :
      forall {otherUse : Use}, used q b r v g otherUse -> accepted q b r v g)
    (h_use_needs_enabling :
      forall {otherUse : Use}, used q b r v g otherUse -> enabled q b r v g otherUse)
    (h_use_functional :
      forall {otherUse : Use}, used q b r v g otherUse -> otherUse = u) :
    used q b r v g u /\
    (forall {otherUse : Use}, used q b r v g otherUse -> accepted q b r v g) /\
    (forall {otherUse : Use}, used q b r v g otherUse -> enabled q b r v g otherUse) /\
    (forall {otherUse : Use}, used q b r v g otherUse -> otherUse = u) := by
  exact And.intro h_used
    (And.intro h_use_needs_acceptance
      (And.intro h_use_needs_enabling (by
        intro otherUse h_other
        exact h_use_functional h_other)))

theorem authority_predecessors_are_separate_and_precede_r
    {capability witness evidence membership s r : Occurrence}
    {b : Branch}
    (h_capability : capabilityGrant capability)
    (h_witness : witnessCreate witness)
    (h_evidence : authEvidenceCreate evidence)
    (h_membership : membershipUpdate membership)
    (h_service_visible : serviceVisible s b)
    (h_capability_order : authorityOrder capability s)
    (h_witness_order : authorityOrder witness s)
    (h_evidence_order : authorityOrder evidence s)
    (h_membership_order : authorityOrder membership s)
    (h_authority_order_in_prec : forall {left right : Occurrence},
      authorityOrder left right -> prec left right)
    (h_sr2 : directSendReceive s r)
    (h_direct_in_prec : forall {left right : Occurrence},
      directSendReceive left right -> prec left right)
    (h_trans : forall {left middle right : Occurrence},
      prec left middle -> prec middle right -> prec left right) :
    capabilityGrant capability /\ witnessCreate witness /\ authEvidenceCreate evidence /\
    membershipUpdate membership /\ serviceVisible s b /\
    prec capability s /\ prec witness s /\ prec evidence s /\ prec membership s /\
    prec capability r /\ prec witness r /\ prec evidence r /\ prec membership r := by
  have h_capability_s : prec capability s := h_authority_order_in_prec h_capability_order
  have h_witness_s : prec witness s := h_authority_order_in_prec h_witness_order
  have h_evidence_s : prec evidence s := h_authority_order_in_prec h_evidence_order
  have h_membership_s : prec membership s := h_authority_order_in_prec h_membership_order
  have h_sr : prec s r := h_direct_in_prec h_sr2
  exact And.intro h_capability
    (And.intro h_witness
      (And.intro h_evidence
        (And.intro h_membership
          (And.intro h_service_visible
            (And.intro h_capability_s
              (And.intro h_witness_s
                (And.intro h_evidence_s
                  (And.intro h_membership_s
                    (And.intro (h_trans h_capability_s h_sr)
                      (And.intro (h_trans h_witness_s h_sr)
                        (And.intro (h_trans h_evidence_s h_sr)
                          (h_trans h_membership_s h_sr))))))))))))

theorem raw_rejection_stays_outside_the_semantic_exchange
    {raw : Raw}
    (h_raw_rejected : rawRejected raw)
    (h_no_occurrence : rawNoOccurrence raw)
    (h_no_failure : rawNoFailure raw)
    (h_no_receipt : rawNoReceipt raw)
    (h_no_use : rawNoUse raw)
    (h_no_restore : rawNoRestore raw) :
    rawRejected raw /\ rawNoOccurrence raw /\ rawNoFailure raw /\
    rawNoReceipt raw /\ rawNoUse raw /\ rawNoRestore raw :=
  And.intro h_raw_rejected
    (And.intro h_no_occurrence
      (And.intro h_no_failure
        (And.intro h_no_receipt (And.intro h_no_use h_no_restore))))

theorem middle_cut_has_only_the_declared_in_flight_closure
    {middle : Cut} {q s r : Occurrence}
    (h_q_in_middle : inCut middle q)
    (h_in_flight : inFlight middle s r)
    (h_in_flight_closure : forall {left right : Occurrence},
      inFlight middle left right -> inCut middle left /\ (inCut middle right -> False)) :
    inCut middle q /\ inCut middle s /\ (inCut middle r -> False) /\
    inFlight middle s r := by
  have h_channel : inCut middle s /\ (inCut middle r -> False) :=
    h_in_flight_closure h_in_flight
  exact And.intro h_q_in_middle
    (And.intro h_channel.left
      (And.intro h_channel.right h_in_flight))

theorem post_r_cut_contains_the_explicit_q_and_s_predecessors
    {post : Cut} {q s r : Occurrence}
    (h_post_r : inCut post r)
    (h_sr1 : directSendReceive q s)
    (h_sr2 : directSendReceive s r)
    (h_direct_in_prec : forall {left right : Occurrence},
      directSendReceive left right -> prec left right)
    (h_prefix : forall {left right : Occurrence},
      inCut post right -> prec left right -> inCut post left) :
    inCut post q /\ inCut post s := by
  have h_qs : prec q s := h_direct_in_prec h_sr1
  have h_sr : prec s r := h_direct_in_prec h_sr2
  have h_post_s : inCut post s := h_prefix h_post_r h_sr
  have h_post_q : inCut post q := h_prefix h_post_s h_qs
  exact And.intro h_post_q h_post_s

theorem pending_correspondence_preserves_the_named_binding
    {before : Current} {after : Restored} {q : Occurrence} {b : Branch}
    {binding : Binding}
    (h_pending_correspondence : pendingCorrespondence before after q b binding)
    (h_pending_preserved :
      pendingCorrespondence before after q b binding -> restoredPending after q b binding) :
    restoredPending after q b binding :=
  h_pending_preserved h_pending_correspondence

theorem failure_correspondence_preserves_the_named_failure
    {before : Current} {after : Restored} {q : Occurrence} {b : Branch}
    {f : Failure}
    (h_failure_correspondence : failureCorrespondence before after q b f)
    (h_failure_preserved :
      failureCorrespondence before after q b f -> restoredFailure after q b f) :
    restoredFailure after q b f :=
  h_failure_preserved h_failure_correspondence

theorem result_receipt_correspondence_preserves_the_named_association
    {before : Current} {after : Restored} {q r : Occurrence} {b : Branch}
    {v : Value} {g : Ground}
    (h_result_receipt_correspondence :
      resultReceiptCorrespondence before after q b r v g)
    (h_result_receipt_preserved :
      resultReceiptCorrespondence before after q b r v g ->
      restoredResultReceipt after q b r v g) :
    restoredResultReceipt after q b r v g :=
  h_result_receipt_preserved h_result_receipt_correspondence

theorem accept_use_ground_channel_correspondence_preserves_the_named_fact
    {before : Current} {after : Restored} {q r : Occurrence} {b : Branch}
    {v : Value} {g : Ground} {u : Use}
    (h_correspondence :
      acceptUseGroundChannelCorrespondence before after q b r v g u)
    (h_preserved :
      acceptUseGroundChannelCorrespondence before after q b r v g u ->
      restoredAcceptUseGroundChannel after q b r v g u) :
    restoredAcceptUseGroundChannel after q b r v g u :=
  h_preserved h_correspondence

theorem receipt_to_cut_and_restore_uses_the_registered_r_delta
    {q s r : Occurrence} {b : Branch} {v : Value} {g : Ground}
    {post : Cut} {before : Current} {after : Restored}
    (h_r : requesterReceive r)
    (h_sr1 : directSendReceive q s)
    (h_sr2 : directSendReceive s r)
    (h_result_send : replySend s)
    (h_receipt : receipt q b r v g)
    (h_receipt_request : receipt q b r v g -> receiptRequest r q)
    (h_request_functional : forall {otherRequest : Occurrence},
      receiptRequest r q -> receiptRequest r otherRequest -> otherRequest = q)
    (h_post_r : inCut post r)
    (h_direct_in_prec : forall {left right : Occurrence},
      directSendReceive left right -> prec left right)
    (h_prefix : forall {left right : Occurrence},
      inCut post right -> prec left right -> inCut post left)
    (h_restore : resultReceiptCorrespondence before after q b r v g)
    (h_restore_preserved :
      resultReceiptCorrespondence before after q b r v g ->
      restoredResultReceipt after q b r v g) :
    requesterReceive r /\ replySend s /\ receipt q b r v g /\
    (forall {otherRequest : Occurrence}, receiptRequest r otherRequest ->
      otherRequest = q) /\
    inCut post q /\ inCut post s /\ restoredResultReceipt after q b r v g := by
  have h_qs : prec q s := h_direct_in_prec h_sr1
  have h_sr : prec s r := h_direct_in_prec h_sr2
  have h_post_s : inCut post s := h_prefix h_post_r h_sr
  have h_post_q : inCut post q := h_prefix h_post_s h_qs
  have h_request_match : receiptRequest r q := h_receipt_request h_receipt
  have h_request_exact :
      forall {otherRequest : Occurrence}, receiptRequest r otherRequest ->
      otherRequest = q := by
    intro otherRequest h_other
    exact h_request_functional h_request_match h_other
  have h_restored : restoredResultReceipt after q b r v g :=
    h_restore_preserved h_restore
  exact And.intro h_r
    (And.intro h_result_send
      (And.intro h_receipt
        (And.intro h_request_exact
          (And.intro h_post_q (And.intro h_post_s h_restored)))))

theorem named_nonvacuity_cases_remain_separate
    {q qFailure qReceipt rReceipt qAccepted rAccepted qConsumed rConsumed : Occurrence}
    {bFailure bReceipt bAccepted bConsumed : Branch}
    {f : Failure} {v : Value} {g : Ground} {u : Use}
    {raw : Raw} {before : Current} {after : Restored}
    (h_emitted : emitted q)
    (h_failure : ownerFailure qFailure bFailure f)
    (h_receipt_pending : receipt qReceipt bReceipt rReceipt v g)
    (h_accepted_unconsumed : accepted qAccepted bAccepted rAccepted v g)
    (h_consumed : used qConsumed bConsumed rConsumed v g u)
    (h_raw_rejected : rawRejected raw)
    (h_restore : sigmaCorrespondence before after qReceipt bReceipt) :
    emitted q /\ ownerFailure qFailure bFailure f /\
    receipt qReceipt bReceipt rReceipt v g /\
    accepted qAccepted bAccepted rAccepted v g /\
    used qConsumed bConsumed rConsumed v g u /\ rawRejected raw /\
    sigmaCorrespondence before after qReceipt bReceipt :=
  And.intro h_emitted
    (And.intro h_failure
      (And.intro h_receipt_pending
        (And.intro h_accepted_unconsumed
          (And.intro h_consumed (And.intro h_raw_rejected h_restore)))))

end P017X1ASigmaConditionalTraceLab
```

## Bound of the result

The code exposes every result, receipt, matching, order, authority, raw-K0,
cut, restore, and non-vacuity fact it relies on as a premise. It derives strict
order only from named direct-generator inclusions, derives exactness only from
named matching functionality, and derives downstream failure exclusion only
from named prerequisites. It does not turn any of these hypotheses into a
Core rule or a claim about actual execution.
