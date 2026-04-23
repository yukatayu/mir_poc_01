# clean near-end typing 01 detail

## この文書の役割

この文書は、clean near-end typing family の **実際に実行可能な入力と actual output** をそのまま確認するための detail です。
summary は `clean_near_end_typing_01.md` を参照してください。

## 実行した command

2026-04-23 に次を実行しました。

```bash
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-family typing --format json
```

## built-in と user-defined の境界

closeout が current built-in vocabulary として列挙しているのは次です。

- `module`
- `index`
- `policy`
- `principal`
- `resource`
- `effect`
- `place`
- `option`
- `chain`
- `fallback`
- `lineage`
- `perform`
- `via`
- `require`
- `ensure`
- `atomic_cut`
- `transition`
- `stage`
- `publish`
- `observe`
- `handoff`
- `witness`
- `model`
- `property`

typing family で user-defined として宣言している主な語は次です。

- `SecurityLabel`
- `FingerprintAuthority`
- `CaptureScope`
- `Region`
- `CostBudget`
- `FingerprintReleasePolicy`
- `Public`
- `UserSecret`
- `KeyMaterial`
- `Observer`
- `Holder`
- `Releaser`
- `Admin`
- `RoomHistory`
- `EphemeralToken`
- `SecretKeyStore`

## 共有前提: `00_index_theories.mir`

```mir
module CleanNearEnd.IndexTheories

index theory SecurityLabel {
  element Public
  element UserSecret
  element KeyMaterial

  order Public <= UserSecret
  order UserSecret <= KeyMaterial

  join
  meet
  law finite_lattice
}

index theory FingerprintAuthority {
  element Observer
  element Holder
  element Releaser
  element Admin

  order Observer <= Holder
  order Holder <= Releaser
  order Releaser <= Admin

  law finite_preorder
}

index theory CaptureScope {
  element RoomHistory
  element EphemeralToken
  element SecretKeyStore

  law finite_powerset
}

index theory Region {
  element Step
  element Turn
  element Session

  order Step <= Turn
  order Turn <= Session

  law finite_preorder
}

index theory CostBudget {
  counter cpu_steps
  counter remote_calls
  counter writes

  law pointwise_natural_bound
}

policy FingerprintReleasePolicy {
  permit declassify SecurityLabel.KeyMaterial -> SecurityLabel.Public
    requires authority >= FingerprintAuthority.Releaser
}
```

## 01_authorized_declassification

### 入力

```mir
module CleanNearEnd.AuthorizedDeclassification

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Releaser

resource secret_key : PrivateKey
  label SecurityLabel.KeyMaterial
  capture SecretKeyStore
  lifetime Session

resource public_board : Board
  label SecurityLabel.Public
  lifetime Session

transition release_fingerprint(actor = Alice) {
  stage derive:
    fp_secret <- perform derive_fingerprint(secret_key)
      ensure label(fp_secret) = SecurityLabel.KeyMaterial

  stage declassify:
    fp_public <- declassify fp_secret
      from SecurityLabel.KeyMaterial
      to SecurityLabel.Public
      using FingerprintReleasePolicy
      requires authority(Alice) >= FingerprintAuthority.Releaser

  stage publish:
    receipt <- perform publish_fingerprint(fp_public)
      to public_board
      after declassify(fp_public)
      requires witness(fp_public)
      ensure label(fp_public) = SecurityLabel.Public
}
```

### actual output

```json
{
  "sample": "01_authorized_declassification",
  "family": "typing",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/typing/01_authorized_declassification.mir",
  "static_verdict": "valid",
  "entered_evaluation": true,
  "terminal_outcome": "success",
  "reason_family": null,
  "constraints_solved": [
    "authority(Alice) >= FingerprintAuthority.Releaser",
    "declassify SecurityLabel.KeyMaterial -> SecurityLabel.Public permitted by FingerprintReleasePolicy",
    "label(fp_public) = SecurityLabel.Public"
  ],
  "constraints_failed": [],
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
    "SecurityLabel",
    "FingerprintAuthority",
    "FingerprintReleasePolicy",
    "Alice"
  ],
  "theorem_obligations": [
    "authorized_release_preserves_public_label"
  ],
  "witness_core_fields": [],
  "current_owner": null,
  "visible_history": []
}
```

## 02_unauthorized_declassification_rejected

### 入力

```mir
module CleanNearEnd.UnauthorizedDeclassificationRejected

use CleanNearEnd.IndexTheories

principal Bob : FingerprintAuthority.Observer

resource secret_key : PrivateKey
  label SecurityLabel.KeyMaterial
  capture SecretKeyStore
  lifetime Session

transition bad_release(actor = Bob) {
  stage derive:
    fp_secret <- perform derive_fingerprint(secret_key)

  stage declassify:
    fp_public <- declassify fp_secret
      from SecurityLabel.KeyMaterial
      to SecurityLabel.Public
      using FingerprintReleasePolicy
      requires authority(Bob) >= FingerprintAuthority.Releaser
}
```

### actual output

```json
{
  "sample": "02_unauthorized_declassification_rejected",
  "family": "typing",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/typing/02_unauthorized_declassification_rejected.mir",
  "static_verdict": "malformed",
  "entered_evaluation": false,
  "terminal_outcome": null,
  "reason_family": "authority_preorder_constraint_failed",
  "constraints_solved": [],
  "constraints_failed": [
    "FingerprintAuthority.Observer >= FingerprintAuthority.Releaser"
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
    "SecurityLabel",
    "FingerprintAuthority",
    "FingerprintReleasePolicy",
    "Bob"
  ],
  "theorem_obligations": [
    "unauthorized_release_is_impossible"
  ],
  "witness_core_fields": [],
  "current_owner": null,
  "visible_history": []
}
```

## 03_label_flow_rejected

### 入力

```mir
module CleanNearEnd.LabelFlowRejected

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Releaser

resource fp_secret : Fingerprint
  label SecurityLabel.KeyMaterial
  lifetime Session

resource public_board : Board
  label SecurityLabel.Public
  lifetime Session

transition bad_publish_without_declassify(actor = Alice) {
  stage derive:
    keep fp_secret
      ensure label(fp_secret) = SecurityLabel.KeyMaterial

  stage publish:
    receipt <- perform publish_fingerprint(fp_secret)
      to public_board
      requires SecurityLabel.KeyMaterial <= SecurityLabel.Public
}
```

### actual output

```json
{
  "sample": "03_label_flow_rejected",
  "family": "typing",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/typing/03_label_flow_rejected.mir",
  "static_verdict": "malformed",
  "entered_evaluation": false,
  "terminal_outcome": null,
  "reason_family": "label_flow_constraint_failed",
  "constraints_solved": [],
  "constraints_failed": [
    "SecurityLabel.KeyMaterial <= SecurityLabel.Public"
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
    "SecurityLabel",
    "KeyMaterial",
    "Public"
  ],
  "theorem_obligations": [
    "no_high_to_low_without_policy"
  ],
  "witness_core_fields": [],
  "current_owner": null,
  "visible_history": []
}
```

## 04_capture_escape_rejected

### 入力

```mir
module CleanNearEnd.CaptureEscapeRejected

use CleanNearEnd.IndexTheories

resource token : SessionToken
  capture EphemeralToken
  lifetime Step

fn make_public_closure(token : SessionToken @ capture(EphemeralToken))
  -> Fn[Unit -> Unit] @ capture(RoomHistory) {
  return fn () {
    use token
  }
}

transition bad_capture_escape {
  stage closure:
    f <- make_public_closure(token)
      requires {EphemeralToken} <= {RoomHistory}
      requires Region.Session <= Region.Step
}
```

### actual output

```json
{
  "sample": "04_capture_escape_rejected",
  "family": "typing",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/typing/04_capture_escape_rejected.mir",
  "static_verdict": "malformed",
  "entered_evaluation": false,
  "terminal_outcome": null,
  "reason_family": "capture_escape",
  "constraints_solved": [],
  "constraints_failed": [
    "{EphemeralToken} <= {RoomHistory}",
    "Region.Session <= Region.Step"
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
    "CaptureScope",
    "EphemeralToken",
    "RoomHistory",
    "Region"
  ],
  "theorem_obligations": [
    "ephemeral_capture_cannot_outlive_step"
  ],
  "witness_core_fields": [],
  "current_owner": null,
  "visible_history": []
}
```

## 05_cost_bound_rejected

### 入力

```mir
module CleanNearEnd.CostBoundRejected

use CleanNearEnd.IndexTheories

effect fetch_remote_bonus
  output bonus : Bonus
  effect remote_call
  cost <= { remote_calls: 1, cpu_steps: 20, writes: 0 }

fn move_without_remote_budget(player : Player)
  -> Board
  cost <= { remote_calls: 0, cpu_steps: 100, writes: 0 } {
  bonus <- perform fetch_remote_bonus()
  require remote_calls: 0
  return apply_bonus(player, bonus)
}
```

### actual output

```json
{
  "sample": "05_cost_bound_rejected",
  "family": "typing",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/typing/05_cost_bound_rejected.mir",
  "static_verdict": "malformed",
  "entered_evaluation": false,
  "terminal_outcome": null,
  "reason_family": "cost_bound_exceeded",
  "constraints_solved": [],
  "constraints_failed": [
    "remote_calls 1 <= 0"
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
    "CostBudget",
    "remote_calls",
    "fetch_remote_bonus"
  ],
  "theorem_obligations": [
    "pointwise_remote_budget_never_increases"
  ],
  "witness_core_fields": [],
  "current_owner": null,
  "visible_history": []
}
```
