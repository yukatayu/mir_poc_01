# verification layer widening threshold 01

## この文書の役割

この文書は、`R1` `VerificationLayer` widening threshold inventory を
短い command sequence で追う landing page です。

- final public verifier contract の確認手順ではありません
- hidden verifier builtin を追加する手順でもありません
- helper/runtime verification lane の current emitted floor と widening threshold を確認する手順です

## まず実行するコマンド

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
git diff --check
```

## これで確認できること

- helper `verification_handoff_witness` が representative helper slice の emitted floor であること
- runtime `verification_model_check` が canonical runtime inventory の emitted floor であること
- theorem bridge / runtime policy / visualization-telemetry は widening threshold の外側に残ること

## これではまだ確認できないこと

- final public verifier contract
- final public layer law schema
- concrete theorem / model-check production binding
- public checker migration timing

## 関連文書

- `current_phase_closeout_01.md`
- `../research_abstract/verification_layer_widening_threshold_01.md`
- `../../plan/29-verification-layer-widening-threshold.md`
