# plan/29 — `VerificationLayer` widening threshold inventory

## role

この文書は `R1` `VerificationLayer` widening threshold inventory の
repository memory である。

- final public verifier contract をここで固定しない
- hidden verifier builtin をここで既成事実化しない
- helper-local preview、runtime canonical inventory、visualization / telemetry downstream surface を
  1 つの public verifier surface に潰さない

## already fixed before `R1`

- `P5` で `LayerSignature` row schema は
  `name / requires / provides / transforms / checks / emits / obligations / laws`
  に揃えてある
- helper closeout は `layer_signature_scope = representative_slice`
- clean near-end runtime closeout は
  `layer_signature_scope = clean_near_end_canonical_inventory`
- `U1` では first shipped public surface scope を
  `two-step split` provisional recommendation に整理済みであり、
  core-side narrowing と integration-side reopening を分けて読む

## current emitted floor

### helper representative floor

- `verification_handoff_witness`
  - requires:
    `publication_order`, `witness(draw_pub#1)`
  - provides:
    `owner_only_rolls`,
    `roll_is_published_before_handoff`,
    `handoff_target_is_active`,
    `no_double_dice_owner`
  - emits:
    `term_signatures`, `verification`
  - obligations:
    `handoff_witness_schema_remains_sample_defined`
  - laws:
    `evidence_preservation`,
    `no_hidden_handoff_without_witness`,
    `residual_obligations_are_explicit`

### runtime canonical floor

- `verification_model_check`
  - requires:
    `property:mutual_exclusion`,
    `runtime_service:model_check`,
    `mode_assumption:second_line_verification`
  - provides:
    `evidence:model_check_result`,
    `view:counterexample_shape`
  - emits:
    `verification_result:pass`
  - obligations:
    `peterson_sc_mutual_exclusion`
  - laws:
    `residual_obligations_are_explicit`,
    `evidence_preservation`

## current law-family inventory

- shared emitted laws on the current `VerificationLayer` floor:
  - `evidence_preservation`
  - `residual_obligations_are_explicit`
- helper-only emitted law on the current floor:
  - `no_hidden_handoff_without_witness`
- not yet emitted as active `VerificationLayer` laws:
  - theorem bridge law families
  - runtime policy / emitted verifier handoff artifact law families
  - visualization / telemetry downstream law families
- this inventory is current repository memory only and does not claim a final public layer law schema

## current naming-gate inventory

- active emitted row names on the current floor:
  - helper `verification_handoff_witness`
  - runtime `verification_model_check`
- evidence-carrier or downstream-consumer names that are intentionally not active emitted row names:
  - helper-local `verification_preview`
  - helper `verification_summary`
  - helper `model_check_summary`
  - theorem bridge
  - runtime policy preview
- not yet fixed as public/shared `VerificationLayer` names:
  - theorem bridge / runtime policy / visualization lane public naming
  - whether current emitted row names survive unchanged into any future public verifier surface
  - the exact naming relation between emitted row names and downstream consumer names
- this naming inventory is current repository memory only and does not claim a final public verifier contract

## current theorem-bridge / runtime-policy boundary inventory

- theorem bridge currently stays on the evidence-carrier side of the boundary.
  - current anchors:
    - proof notebook review unit
    - Lean bridge docs / repo-local stub evidence
    - theorem reopen-threshold memory
  - current stop line:
    - not an active emitted `LayerSignature` row
    - not a proof-object schema
    - not a theorem result-object public contract
    - not a concrete theorem prover brand / execution binding
    - not a final public verifier contract
- runtime policy / emitted verifier handoff artifact currently stays on the kept-later side of the boundary.
  - current anchors:
    - model-check second-line planning memory
    - phase5 proof/protocol/runtime-policy handoff threshold memory
    - runtime-policy preview language
  - current stop line:
    - not an active emitted `LayerSignature` row
    - not an actual emitted verifier handoff artifact
    - not a production checker / runtime-policy contract
    - not a hidden verifier builtin
    - not a final public verifier contract
- shared unresolved widening questions before any emitted-row promotion:
  - whether theorem bridge and runtime policy widen as separate emitted rows or one composed family, and how emitted verifier handoff artifacts fit that widened surface
  - which law families would become shared emitted laws versus family-local obligations
  - the exact relation among `verification_model_check`, theorem result objects, emitted verifier handoff artifacts, and any future public verifier contract
- this inventory is current repository memory only. It does not choose a widening package and does not freeze public naming.

## current composition-contract boundary inventory

- current `VerificationLayer` surfaces remain explicitly split.
  - active emitted rows:
    - helper `verification_handoff_witness`
    - runtime `verification_model_check`
  - evidence carriers / helper-local previews:
    - `verification_preview`
    - theorem bridge
    - runtime policy preview
  - downstream consumers:
    - `verification_summary`
    - `model_check_summary`
    - viewer / telemetry prototype inventory
  - kept-later contract carriers:
    - actual emitted verifier handoff artifact
    - production checker / runtime-policy contract
    - final public verifier contract
- current fixed relation among these surfaces:
  - emitted rows may feed preview or downstream evidence, but emitted rows alone do not define a public verifier contract
  - evidence carriers may summarize or stage future widening material, but they do not auto-promote into emitted rows
  - downstream consumer lanes may expose verification evidence outward, but they are not `VerificationLayer` rows
  - emitted verifier handoff artifacts and final public verifier contracts remain later gates beyond the current emitted floor
- still unresolved before any public composition contract is fixed:
  - which of these surfaces become part of the first frozen public verifier seam
  - the exact naming and migration relation among emitted rows, theorem/model-check artifacts, and any emitted verifier handoff artifact
  - how helper `representative_slice` and runtime `clean_near_end_canonical_inventory` map into any shared public verifier surface
- this composition inventory is current repository memory only. It does not freeze a public verifier contract.

## widening threshold matrix

| family | current carrier | current status | widen only when | kept-later / stop line |
|---|---|---|---|---|
| helper handoff witness | helper `layer_signatures` row `verification_handoff_witness`、helper `--debug layers`、helper closeout `representative_slice` | current emitted floor | already actualized; keep row + obligations + laws explicit | sample-defined witness schema remains explicit; not a final public verifier contract |
| runtime model-check | runtime canonical `layer_signatures` row `verification_model_check`、runtime closeout `clean_near_end_canonical_inventory` | current emitted floor | already actualized; keep property/tool/mode assumptions explicit | concrete tool brand、public checker artifact、production checker/runtime-policy contract stay later |
| finite-index checker preview | helper-local `verification_preview` / static verdict / property preview | current evidence carrier only | settled property language、checker artifact boundary、subject/evidence carrier naming が揃ったとき | final parser / checker / verifier contract と混同しない |
| theorem bridge | proof notebook review unit、Lean bridge docs、theorem reopen-threshold memory | current evidence carrier only | theorem result object、proof object schema、tool-brand / execution boundary が揃ったとき | final public theorem / verifier contract は later |
| runtime policy / emitted verifier handoff artifact | model-check second-line planning memory、phase5 proof/protocol/runtime-policy handoff threshold memory、runtime-policy preview language | kept-later widening candidate | actual emitted verifier handoff artifact と production checker/runtime-policy contract が named になったとき | runtime policy preview を hidden verifier builtin にしない |
| visualization / telemetry around verification | helper `verification_summary` view、helper `model_check_summary` telemetry row、viewer prototype inventory | downstream consumer only | viewer / telemetry package で public viewer contract を別途 narrow したとき | verification evidence downstream surface であり、`VerificationLayer` emitted row と同一視しない |

## widening rules

1. widening は named carrier が先である
   - `requires / provides / transforms / checks / emits / obligations / laws`
     を満たす row なしに emitted `LayerSignature` へ上げない
2. helper/runtime scope split を残す
   - `representative_slice` と
     `clean_near_end_canonical_inventory`
     を collapse しない
3. `obligations` lane は widening gate の一部である
   - row が emitted floor に入るなら residual obligation を explicit に持つ
4. visualization / telemetry は downstream effect として扱う
   - verification evidence を外へ出すが、verification row そのものではない
5. public contract は別 gate である
   - current emitted floor と final public verifier contract を混同しない

## validation floor

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
git diff --check
```

## stop line

- hidden verifier builtin を既成事実化しない
- final public verifier contract を claim しない
- final public layer law schema を claim しない
- theorem bridge / runtime policy / visualization-telemetry を
  active emitted `LayerSignature` row に premature に上げない
- concrete theorem / model-check production binding を claim しない

## related memory

- `specs/10-open-questions.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
