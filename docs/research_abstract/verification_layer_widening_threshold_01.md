# verification layer widening threshold 01

## この文書の役割

この文書は `R1` `VerificationLayer` widening threshold inventory を
reader-facing に短く読む summary です。

- final public verifier contract を決める文書ではありません
- helper/runtime inventory を 1 つの public verifier に潰す文書でもありません
- current emitted floor と widening threshold を current reading として束ねます

## current emitted floor

| surface | current status | current reading |
|---|---|---|
| helper `verification_handoff_witness` | emitted floor | witness-backed handoff verification の representative slice |
| runtime `verification_model_check` | emitted floor | model-check second-line canonical inventory |
| `verification_summary` / `model_check_summary` | downstream consumer only | verification evidence を外へ出す typed visualization / telemetry。`VerificationLayer` row ではない |
| theorem bridge / runtime policy preview | evidence carrier only | future widening candidate。public contract ではない |

## widening threshold

- emitted row に上げる前に、
  `requires / provides / transforms / checks / emits / obligations / laws`
  を named carrier で持つ必要がある
- helper `representative_slice` と runtime `clean_near_end_canonical_inventory` は collapse しない
- downstream visualization / telemetry は verification evidence の consumer として扱い、
  verification row そのものにしない

## stop line

- hidden verifier builtin を既成事実化しない
- final public verifier contract を claim しない
- theorem bridge / runtime policy / visualization-telemetry を
  active emitted `LayerSignature` row と呼ばない

## 関連文書

- `mirrorea_future_axis_01.md`
- `public_api_parser_gate_plan_01.md`
- `../hands_on/verification_layer_widening_threshold_01.md`
- `../../plan/29-verification-layer-widening-threshold.md`
