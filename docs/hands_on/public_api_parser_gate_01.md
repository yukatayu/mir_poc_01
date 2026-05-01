# public API / parser grammar gate 01

## この文書の役割

この文書は、`P18` public API / parser grammar gate の
**current first-cut closeout** を短い command sequence で追う
landing page です。

- final public freeze の確認手順ではありません
- repo-side freeze checklist / boundary inventory / hold-line separation の確認手順です

## まず実行するコマンド

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 scripts/network_transport_samples.py check-all --format json
python3 scripts/projection_codegen_samples.py check-all --format json
python3 scripts/projection_codegen_samples.py closeout --format json
python3 scripts/visual_debugger_viewer_samples.py closeout --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
bash scripts/env/mirrorea_storage_env.sh
bash scripts/env/mirrorea_storage_env.sh --ensure-dirs
bash scripts/storage/detach_prepare.sh
bash scripts/storage/cleanup_disposable_artifacts.sh --list
CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run
git diff --check
```

## これで確認できること

- docs / plan / snapshot / dashboard が `P18` repo-side first cut に同期していること
- parser / checker / runtime / verifier / viewer / adapter / projection / hot-plug / transport の
  current preview / prototype / inventory に qualifier が残っていること
- storage commands が external workdir routing、non-destructive detach audit、
  explicit-confirmation cleanup policy、external cargo cache usability の
  guardrail evidence に留まること。
  これは actual LLVM build、backend choice、packaging adoption の evidence ではありません。
- post-`P18` true user-spec hold line が installed binary / packaging adoption target、host target、first shipped public surface、final catalog に分離されていること

## これではまだ確認できないこと

- final parser grammar
- final public parser / checker / runtime / verifier API
- final public adapter / viewer / projection / hot-plug / transport ABI
- installed binary / FFI / engine adapter / deployment contract

## 関連文書

- `current_phase_closeout_01.md`
- `../research_abstract/public_api_parser_gate_plan_01.md`
- `../../plan/27-public-api-parser-gate-roadmap.md`
- `../../progress.md`
- `../../tasks.md`
