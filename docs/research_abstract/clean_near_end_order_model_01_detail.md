# clean near-end order / model 01 detail

## この文書の役割

この文書は、clean near-end の order-handoff family と model-check family の **実際の入力と actual output** を確認するための detail です。
summary は `clean_near_end_order_model_01.md` を参照してください。

## 実行した command

2026-04-23 に次を実行しました。

```bash
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-family order-handoff --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-family model-check --format json
```

## built-in と user-defined の境界

current closeout が built-in vocabulary として列挙している語のうち、この family で主に使うのは次です。

- `transition`
- `stage`
- `perform`
- `publish`
- `handoff`
- `witness`
- `atomic_cut`
- `model`
- `property`

この family で user-defined として出てくる主な語は次です。

- `FingerprintAuthority`
- `dice_owner`
- `draw_pub`
- `DelegatedRngReceipt`
- `AuthorityDrawWitness`
- `PetersonSC`
- `PetersonRelaxedNoPublication`
- `BrokenMutex`

## 共有前提

order-handoff sample は `samples/clean-near-end/00_index_theories.mir` を `use CleanNearEnd.IndexTheories` で読みます。
共有 index theory の全文は `clean_near_end_typing_01_detail.md` に掲載しています。

## order-handoff family

## 01_authorized_roll_publish_handoff

### 入力

```mir
module CleanNearEnd.AuthorizedRollPublishHandoff

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Admin
principal Bob   : FingerprintAuthority.Holder

resource dice_owner : Player
  authority_scope room
  lifetime Session

transition handoff_turn(owner = Alice) {
  stage roll:
    draw <- perform roll_dice via authority_rng

  stage publish:
    publish draw
      scope room
      produces witness draw_pub

  stage handoff:
    handoff dice_owner Alice -> Bob
      after publish(draw)
      requires witness(draw_pub)
}
```

### actual output

```json
{
  "sample": "01_authorized_roll_publish_handoff",
  "family": "order-handoff",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/order-handoff/01_authorized_roll_publish_handoff.mir",
  "static_verdict": "valid",
  "entered_evaluation": true,
  "terminal_outcome": "success",
  "reason_family": null,
  "constraints_solved": [
    "requires witness(draw_pub)",
    "publish(draw) must precede handoff"
  ],
  "constraints_failed": [],
  "residual_obligations": [],
  "relations": [
    [
      "program_order",
      "roll",
      "publish"
    ],
    [
      "publication_order",
      "roll",
      "publish"
    ],
    [
      "witness_order",
      "publish",
      "handoff"
    ],
    [
      "scoped_happens_before",
      "roll",
      "handoff"
    ]
  ],
  "mode_constraints": [],
  "model_check_result": null,
  "property": null,
  "checked_under": null,
  "counterexample_shape": [],
  "explanation": null,
  "built_in_terms": [
    "module",
    "index",
    "policy",
    "principal",
    "resource",
    "effect",
    "place",
    "option",
    "chain",
    "fallback",
    "lineage",
    "perform",
    "via",
    "require",
    "ensure",
    "atomic_cut",
    "transition",
    "stage",
    "publish",
    "observe",
    "handoff",
    "witness",
    "model",
    "property"
  ],
  "user_defined_terms": [
    "dice_owner",
    "draw_pub"
  ],
  "theorem_obligations": [
    "handoff_requires_publication_witness"
  ],
  "witness_core_fields": [
    "draw_pub"
  ],
  "current_owner": "Bob",
  "visible_history": [
    "draw",
    "publish(draw)",
    "handoff(Alice -> Bob)"
  ]
}
```

## 02_missing_witness_rejected

### 入力

```mir
module CleanNearEnd.MissingWitnessRejected

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Admin
principal Bob   : FingerprintAuthority.Holder

resource dice_owner : Player
  authority_scope room
  lifetime Session

transition bad_handoff_missing_witness(owner = Alice) {
  stage roll:
    draw <- perform roll_dice via authority_rng

  stage publish:
    publish draw
      scope room
      after publish(draw)

  stage handoff:
    handoff dice_owner Alice -> Bob
      after publish(draw)
}
```

### actual output

```json
{
  "sample": "02_missing_witness_rejected",
  "family": "order-handoff",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/order-handoff/02_missing_witness_rejected.mir",
  "static_verdict": "malformed",
  "entered_evaluation": false,
  "terminal_outcome": null,
  "reason_family": "missing_handoff_witness",
  "constraints_solved": [],
  "constraints_failed": [
    "requires witness(draw_pub)"
  ],
  "residual_obligations": [],
  "relations": [],
  "mode_constraints": [],
  "model_check_result": null,
  "property": null,
  "checked_under": null,
  "counterexample_shape": [],
  "explanation": null,
  "built_in_terms": [
    "module",
    "index",
    "policy",
    "principal",
    "resource",
    "effect",
    "place",
    "option",
    "chain",
    "fallback",
    "lineage",
    "perform",
    "via",
    "require",
    "ensure",
    "atomic_cut",
    "transition",
    "stage",
    "publish",
    "observe",
    "handoff",
    "witness",
    "model",
    "property"
  ],
  "user_defined_terms": [
    "dice_owner"
  ],
  "theorem_obligations": [
    "handoff_without_witness_is_blocked"
  ],
  "witness_core_fields": [],
  "current_owner": null,
  "visible_history": []
}
```

## 03_handoff_before_publication_rejected

### 入力

```mir
module CleanNearEnd.HandoffBeforePublicationRejected

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Admin
principal Bob   : FingerprintAuthority.Holder

resource dice_owner : Player
  authority_scope room
  lifetime Session

transition bad_handoff_before_publish(owner = Alice) {
  stage roll:
    draw <- perform roll_dice via authority_rng

  stage handoff:
    handoff dice_owner Alice -> Bob
      requires witness(draw_pub)

  stage publish:
    publish draw
      scope room
      produces witness draw_pub
}
```

### actual output

```json
{
  "sample": "03_handoff_before_publication_rejected",
  "family": "order-handoff",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/order-handoff/03_handoff_before_publication_rejected.mir",
  "static_verdict": "malformed",
  "entered_evaluation": false,
  "terminal_outcome": null,
  "reason_family": "handoff_before_publication",
  "constraints_solved": [],
  "constraints_failed": [
    "publish(draw) must precede handoff that requires witness(draw_pub)"
  ],
  "residual_obligations": [],
  "relations": [],
  "mode_constraints": [],
  "model_check_result": null,
  "property": null,
  "checked_under": null,
  "counterexample_shape": [],
  "explanation": null,
  "built_in_terms": [
    "module",
    "index",
    "policy",
    "principal",
    "resource",
    "effect",
    "place",
    "option",
    "chain",
    "fallback",
    "lineage",
    "perform",
    "via",
    "require",
    "ensure",
    "atomic_cut",
    "transition",
    "stage",
    "publish",
    "observe",
    "handoff",
    "witness",
    "model",
    "property"
  ],
  "user_defined_terms": [
    "dice_owner",
    "draw_pub"
  ],
  "theorem_obligations": [
    "publication_precedes_handoff"
  ],
  "witness_core_fields": [],
  "current_owner": null,
  "visible_history": []
}
```

## 04_stage_block_authorized_handoff

### 入力

```mir
module CleanNearEnd.StageBlockAuthorizedHandoff

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Admin
principal Bob   : FingerprintAuthority.Holder

resource dice_owner : Player
  authority_scope room
  lifetime Session

transition stage_block_handoff(owner = Alice) {
  stage roll:
    draw <- perform roll_dice via authority_rng

  stage publish:
    publish draw
      scope room
      produces witness draw_pub

  # stage block
  stage stage_block:
    atomic_cut

  stage handoff:
    handoff dice_owner Alice -> Bob
      after publish(draw)
      requires witness(draw_pub)
}
```

### actual output

```json
{
  "sample": "04_stage_block_authorized_handoff",
  "family": "order-handoff",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/order-handoff/04_stage_block_authorized_handoff.mir",
  "static_verdict": "valid",
  "entered_evaluation": true,
  "terminal_outcome": "success",
  "reason_family": null,
  "constraints_solved": [
    "requires witness(draw_pub)",
    "publish(draw) must precede handoff"
  ],
  "constraints_failed": [],
  "residual_obligations": [],
  "relations": [
    [
      "program_order",
      "publish",
      "stage_block"
    ],
    [
      "finalization_order",
      "stage_block",
      "handoff"
    ],
    [
      "scoped_happens_before",
      "roll",
      "handoff"
    ]
  ],
  "mode_constraints": [],
  "model_check_result": null,
  "property": null,
  "checked_under": null,
  "counterexample_shape": [],
  "explanation": null,
  "built_in_terms": [
    "module",
    "index",
    "policy",
    "principal",
    "resource",
    "effect",
    "place",
    "option",
    "chain",
    "fallback",
    "lineage",
    "perform",
    "via",
    "require",
    "ensure",
    "atomic_cut",
    "transition",
    "stage",
    "publish",
    "observe",
    "handoff",
    "witness",
    "model",
    "property"
  ],
  "user_defined_terms": [
    "draw_pub",
    "stage_block"
  ],
  "theorem_obligations": [
    "atomic_cut_is_local_finalization_only"
  ],
  "witness_core_fields": [
    "draw_pub"
  ],
  "current_owner": "Bob",
  "visible_history": [
    "draw",
    "publish(draw)",
    "atomic_cut(stage_block)",
    "handoff(Alice -> Bob)"
  ]
}
```

## 05_delegated_rng_service

### 入力

```mir
module CleanNearEnd.DelegatedRngService

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Admin
principal Bob   : FingerprintAuthority.Holder

effect delegated_rng_roll
  output draw : DiceDraw
  output provider_receipt : DelegatedRngReceipt
  effect rng

transition delegated_rng_turn(owner = Alice) {
  stage provider:
    draw <- perform delegated_rng_roll()
      produces witness provider_receipt

  stage publish:
    publish draw
      scope room

  stage handoff:
    handoff dice_owner Alice -> Bob
      requires witness(provider_receipt)
}
```

### actual output

```json
{
  "sample": "05_delegated_rng_service",
  "family": "order-handoff",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/order-handoff/05_delegated_rng_service.mir",
  "static_verdict": "valid",
  "entered_evaluation": true,
  "terminal_outcome": "success",
  "reason_family": null,
  "constraints_solved": [
    "requires witness(provider_receipt)",
    "effect row { rng, witness } <= { rng, witness, publish }"
  ],
  "constraints_failed": [],
  "residual_obligations": [],
  "relations": [
    [
      "dependency_order",
      "provider",
      "publish"
    ],
    [
      "witness_order",
      "provider",
      "handoff"
    ],
    [
      "publication_order",
      "publish",
      "handoff"
    ]
  ],
  "mode_constraints": [],
  "model_check_result": null,
  "property": null,
  "checked_under": null,
  "counterexample_shape": [],
  "explanation": "delegated_rng_service stays on a provider boundary and leaves room mutation to the authority holder.",
  "built_in_terms": [
    "module",
    "index",
    "policy",
    "principal",
    "resource",
    "effect",
    "place",
    "option",
    "chain",
    "fallback",
    "lineage",
    "perform",
    "via",
    "require",
    "ensure",
    "atomic_cut",
    "transition",
    "stage",
    "publish",
    "observe",
    "handoff",
    "witness",
    "model",
    "property"
  ],
  "user_defined_terms": [
    "provider_receipt",
    "DelegatedRngReceipt"
  ],
  "theorem_obligations": [
    "provider_returns_draw_not_room_commit"
  ],
  "witness_core_fields": [
    "witness_kind",
    "action_ref",
    "draw_slot",
    "draw_result"
  ],
  "current_owner": "Bob",
  "visible_history": [
    "provider_roll(receipt)",
    "publish(draw)",
    "handoff(Alice -> Bob)"
  ]
}
```

## 06_auditable_authority_witness

### 入力

```mir
module CleanNearEnd.AuditableAuthorityWitness

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Admin
principal Bob   : FingerprintAuthority.Holder

record AuthorityDrawWitness {
  witness_kind : Text
  action_ref : Text
  draw_slot : Text
  draw_result : DiceDraw
}

transition audited_handoff(owner = Alice) {
  stage roll:
    draw <- perform roll_dice via authority_rng

  stage publish:
    publish draw
      scope room
      produces witness draw_pub

  stage handoff:
    handoff dice_owner Alice -> Bob
      requires witness(draw_pub)

  stage audit:
    note AuthorityDrawWitness {
      witness_kind = "authority_draw"
      action_ref = "handoff_dice_owner"
      draw_slot = "primary"
      draw_result = draw
    }
      requires witness(draw_pub)
}
```

### actual output

```json
{
  "sample": "06_auditable_authority_witness",
  "family": "order-handoff",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/order-handoff/06_auditable_authority_witness.mir",
  "static_verdict": "valid",
  "entered_evaluation": true,
  "terminal_outcome": "success",
  "reason_family": null,
  "constraints_solved": [
    "requires witness(draw_pub)"
  ],
  "constraints_failed": [],
  "residual_obligations": [],
  "relations": [
    [
      "witness_order",
      "publish",
      "audit"
    ],
    [
      "scoped_happens_before",
      "roll",
      "audit"
    ]
  ],
  "mode_constraints": [],
  "model_check_result": null,
  "property": null,
  "checked_under": null,
  "counterexample_shape": [],
  "explanation": "The witness core fields are declared by the sample package, not by the language runtime.",
  "built_in_terms": [
    "module",
    "index",
    "policy",
    "principal",
    "resource",
    "effect",
    "place",
    "option",
    "chain",
    "fallback",
    "lineage",
    "perform",
    "via",
    "require",
    "ensure",
    "atomic_cut",
    "transition",
    "stage",
    "publish",
    "observe",
    "handoff",
    "witness",
    "model",
    "property"
  ],
  "user_defined_terms": [
    "AuthorityDrawWitness",
    "draw_pub"
  ],
  "theorem_obligations": [
    "authority_witness_preserves_subject_identity"
  ],
  "witness_core_fields": [
    "witness_kind",
    "action_ref",
    "draw_slot",
    "draw_result"
  ],
  "current_owner": "Bob",
  "visible_history": [
    "draw",
    "publish(draw)",
    "handoff(Alice -> Bob)",
    "audit(draw_pub)"
  ]
}
```

## model-check family

## 01_peterson_sc_pass

### 入力

```mir
module CleanNearEnd.PetersonScPass

model PetersonSC {
  actor A
  actor B

  shared flag[A] : Bool = false
  shared flag[B] : Bool = false
  shared turn : Actor = A

  process A {
    flag[A] <- true
    turn <- B
    wait while flag[B] == true and turn == B
    critical A
    flag[A] <- false
  }

  process B {
    flag[B] <- true
    turn <- A
    wait while flag[A] == true and turn == A
    critical B
    flag[B] <- false
  }

  memory_model sequential_consistency

  property mutual_exclusion:
    never (in_critical(A) and in_critical(B))
}
```

### actual output

```json
{
  "sample": "01_peterson_sc_pass",
  "family": "model-check",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/model-check/01_peterson_sc_pass.mir",
  "static_verdict": "valid",
  "entered_evaluation": false,
  "terminal_outcome": null,
  "reason_family": null,
  "constraints_solved": [],
  "constraints_failed": [],
  "residual_obligations": [],
  "relations": [],
  "mode_constraints": [],
  "model_check_result": "pass",
  "property": "mutual_exclusion",
  "checked_under": "sequential_consistency",
  "counterexample_shape": [],
  "explanation": null,
  "built_in_terms": [
    "module",
    "index",
    "policy",
    "principal",
    "resource",
    "effect",
    "place",
    "option",
    "chain",
    "fallback",
    "lineage",
    "perform",
    "via",
    "require",
    "ensure",
    "atomic_cut",
    "transition",
    "stage",
    "publish",
    "observe",
    "handoff",
    "witness",
    "model",
    "property"
  ],
  "user_defined_terms": [
    "PetersonSC",
    "mutual_exclusion"
  ],
  "theorem_obligations": [
    "peterson_sc_mutual_exclusion"
  ],
  "witness_core_fields": [],
  "current_owner": null,
  "visible_history": []
}
```

## 02_peterson_relaxed_counterexample

### 入力

```mir
module CleanNearEnd.PetersonRelaxedCounterexample

model PetersonRelaxedNoPublication {
  actor A
  actor B

  shared flag[A] : Bool = false
  shared flag[B] : Bool = false
  shared turn : Actor = A

  process A {
    flag[A] <- true
    turn <- B
    wait while flag[B] == true and turn == B
    critical A
    flag[A] <- false
  }

  process B {
    flag[B] <- true
    turn <- A
    wait while flag[A] == true and turn == A
    critical B
    flag[B] <- false
  }

  memory_model relaxed_without_publication_observation_edges

  property mutual_exclusion:
    never (in_critical(A) and in_critical(B))
}
```

### actual output

```json
{
  "sample": "02_peterson_relaxed_counterexample",
  "family": "model-check",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/model-check/02_peterson_relaxed_counterexample.mir",
  "static_verdict": "valid",
  "entered_evaluation": false,
  "terminal_outcome": null,
  "reason_family": null,
  "constraints_solved": [],
  "constraints_failed": [],
  "residual_obligations": [],
  "relations": [],
  "mode_constraints": [],
  "model_check_result": "counterexample",
  "property": "mutual_exclusion",
  "checked_under": "relaxed_without_publication_observation_edges",
  "counterexample_shape": [
    "A writes flag[A] but B has not observed it",
    "A writes turn <- B",
    "A reads flag[B] as false",
    "B writes flag[B] but A has not observed it",
    "B writes turn <- A",
    "B reads flag[A] as false"
  ],
  "explanation": null,
  "built_in_terms": [
    "module",
    "index",
    "policy",
    "principal",
    "resource",
    "effect",
    "place",
    "option",
    "chain",
    "fallback",
    "lineage",
    "perform",
    "via",
    "require",
    "ensure",
    "atomic_cut",
    "transition",
    "stage",
    "publish",
    "observe",
    "handoff",
    "witness",
    "model",
    "property"
  ],
  "user_defined_terms": [
    "PetersonRelaxedNoPublication",
    "mutual_exclusion"
  ],
  "theorem_obligations": [
    "publication_observation_edges_are_required"
  ],
  "witness_core_fields": [],
  "current_owner": null,
  "visible_history": []
}
```

## 03_broken_mutex_counterexample

### 入力

```mir
module CleanNearEnd.BrokenMutexCounterexample

model BrokenMutex {
  actor A
  actor B

  shared flag[A] : Bool = false
  shared flag[B] : Bool = false

  process A {
    if flag[B] == false
    flag[A] <- true
    critical A
    flag[A] <- false
  }

  process B {
    if flag[A] == false
    flag[B] <- true
    critical B
    flag[B] <- false
  }

  property mutual_exclusion:
    never (in_critical(A) and in_critical(B))
}
```

### actual output

```json
{
  "sample": "03_broken_mutex_counterexample",
  "family": "model-check",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/model-check/03_broken_mutex_counterexample.mir",
  "static_verdict": "valid",
  "entered_evaluation": false,
  "terminal_outcome": null,
  "reason_family": null,
  "constraints_solved": [],
  "constraints_failed": [],
  "residual_obligations": [],
  "relations": [],
  "mode_constraints": [],
  "model_check_result": "counterexample",
  "property": "mutual_exclusion",
  "checked_under": "interleaving_sc",
  "counterexample_shape": [
    "A checks flag[B] and sees false",
    "B checks flag[A] and sees false",
    "A sets flag[A]",
    "A enters critical",
    "B sets flag[B]",
    "B enters critical"
  ],
  "explanation": "interleaving or visibility permits both actors to enter critical section",
  "built_in_terms": [
    "module",
    "index",
    "policy",
    "principal",
    "resource",
    "effect",
    "place",
    "option",
    "chain",
    "fallback",
    "lineage",
    "perform",
    "via",
    "require",
    "ensure",
    "atomic_cut",
    "transition",
    "stage",
    "publish",
    "observe",
    "handoff",
    "witness",
    "model",
    "property"
  ],
  "user_defined_terms": [
    "BrokenMutex",
    "mutual_exclusion"
  ],
  "theorem_obligations": [
    "broken_mutex_requires_model_check"
  ],
  "witness_core_fields": [],
  "current_owner": null,
  "visible_history": []
}
```
