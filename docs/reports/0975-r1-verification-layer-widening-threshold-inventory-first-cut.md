# Report 0975 — `R1` `VerificationLayer` widening threshold inventory first cut

- Date: 2026-04-28
- Author / agent: Codex
- Scope: `R1` active docs-first inventory の first cut。helper/runtime verification lane の current emitted floor と widening threshold を repository memory / reader-facing docs に actualize する
- Decision levels touched: L2 / L3 documentation boundary only。final public verifier contract や final layer law schema は固定しない

## 1. Objective

helper `verification_handoff_witness` と runtime `verification_model_check` を
current emitted floor として保ったまま、
theorem bridge / runtime policy / visualization-telemetry をどこで widen するかの
threshold を docs-first に整理する。

## 2. Inputs consulted

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/10-open-questions.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- helper evidence:
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers --format json`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
- runtime evidence:
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
- explorer read-only inventory:
  `Lovelace`

## 3. Actions taken

1. `plan/29-verification-layer-widening-threshold.md` を追加し、
   current emitted floor、widening threshold matrix、widening rules、stop line を repo memory に固定した。
2. `verification_layer_widening_threshold_01.md` の summary / landing page を追加し、
   helper/runtime verification lane の current reading を reader-facing に分けた。
3. `plan/09`、`plan/14`、`plan/27`、`specs/10` を更新し、
   theorem bridge / runtime policy / visualization-telemetry は
   active emitted `LayerSignature` row ではなく、
   current evidence carrier または downstream consumer に留める line を mirror した。
4. `README.md`、`Documentation.md`、`docs/research_abstract/README.md`、
   `docs/hands_on/README.md`、`docs/research_abstract/mirrorea_future_axis_01.md`、
   `docs/hands_on/current_phase_closeout_01.md` に `R1` current reading の入口を追加した。

## 4. Files changed

- new:
  - `plan/29-verification-layer-widening-threshold.md`
  - `docs/research_abstract/verification_layer_widening_threshold_01.md`
  - `docs/hands_on/verification_layer_widening_threshold_01.md`
  - `docs/reports/0975-r1-verification-layer-widening-threshold-inventory-first-cut.md`
- updated:
  - `README.md`
  - `Documentation.md`
  - `docs/research_abstract/README.md`
  - `docs/research_abstract/mirrorea_future_axis_01.md`
  - `docs/hands_on/README.md`
  - `docs/hands_on/current_phase_closeout_01.md`
  - `plan/00-index.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/14-glossary-and-boundary-rules.md`
  - `plan/27-public-api-parser-gate-roadmap.md`
  - `specs/10-open-questions.md`

## 5. Commands run and exact outputs

```bash
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers --format json
```

Relevant exact output:

```text
"name": "verification_handoff_witness"
"requires": ["publication_order", "witness(draw_pub#1)"]
"emits": ["term_signatures", "verification"]
"obligations": ["handoff_witness_schema_remains_sample_defined"]
"laws": ["evidence_preservation", "no_hidden_handoff_without_witness", "residual_obligations_are_explicit"]
```

```bash
python3 scripts/sugoroku_world_samples.py closeout --format json
```

Relevant exact output:

```text
"layer_signature_scope": "representative_slice"
"layer_signature_names": [
  "hotplug_activation_boundary",
  "hotplug_detach_boundary",
  "membership_late_join_boundary",
  "membership_leave_invalidation",
  "membership_owner_reassignment",
  "runtime_turn_trace",
  "verification_handoff_witness"
]
"reserved_layer_signature_names": [
  "auth_authority_witness",
  "transport_provider_boundary",
  "verification_model_check",
  "visualization_redacted_debug_view",
  "typed_telemetry_emitter"
]
```

```bash
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
```

Relevant exact output:

```text
"layer_signature_scope": "clean_near_end_canonical_inventory"
"name": "verification_model_check"
"requires": [
  "property:mutual_exclusion",
  "runtime_service:model_check",
  "mode_assumption:second_line_verification"
]
"provides": ["evidence:model_check_result", "view:counterexample_shape"]
"obligations": ["peterson_sc_mutual_exclusion"]
```

Checkpoint validation rerun と snapshot / queue sync は
`docs/reports/0976-r1-verification-threshold-closeout-and-r2-promotion.md`
に記録する。

## 6. Evidence / findings

- current emitted floor は 2 本だけである
  - helper representative:
    `verification_handoff_witness`
  - runtime canonical:
    `verification_model_check`
- `verification_summary` / `model_check_summary` は
  visualization / telemetry downstream consumer であり、
  `VerificationLayer` emitted row と同一視しない方が current repo evidence と整合する
- theorem bridge と runtime policy preview は
  current evidence carrier であり、
  active emitted `LayerSignature` row ではない

## 7. Changes in understanding

- `VerificationLayer` は「verification に関するもの全部」の別名ではなく、
  current emitted floor と widening candidate を分ける typed composition の current reading である
- helper `representative_slice` と runtime `clean_near_end_canonical_inventory` の scope split は
  widening threshold の本体であり、単なる出力差分ではない
- downstream visualization / telemetry は verification evidence の consumer として読む方が
  final public verifier contract への overclaim を防ぎやすい

## 8. Open questions

- theorem bridge をどの widening package で emitted `LayerSignature` row に上げるか
- runtime policy / emitted verifier handoff artifact をどの gate で named carrier にするか
- visualization / telemetry 側で verification downstream surface をどこまで public-facing にするか
- `R1` close後の next self-driven package を `AttachPoint` minimal contract に寄せるか、
  別の repo-side narrowing に寄せるか

## 9. Suggested next prompt

`R1` の snapshot / dashboard / report sync を仕上げ、`python3 scripts/check_source_hierarchy.py`、`python3 scripts/validate_docs.py`、`python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers --format json`、`python3 scripts/sugoroku_world_samples.py closeout --format json`、`cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`、`git diff --check` を rerun した上で commit/push してください。
