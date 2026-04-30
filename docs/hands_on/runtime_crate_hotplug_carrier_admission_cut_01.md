# runtime crate hot-plug carrier admission cut 01

## この文書の役割

この文書は、`R6` docs-first closeout を最短コマンドで確認する landing page です。

- final public completion ではありません
- helper-local preview を Rust-side engine completion と混同しません
- request / verdict carrier boundary だけを確認します

## まず実行するコマンド

```bash
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json
python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
cargo test -p mirrorea-core
cargo test -p mir-runtime
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

## これで確認できること

- helper-local `hotplug_lifecycle` と attach/detach anchor IDs が
  helper-local attach/detach evidence carrier であること
- `mirrorea-core` が Rust-side generic carrier / logical runtime substrate を持ち、
  hot-plug engine をまだ持たないこと
- `mir-runtime` が thin runtime/report assembly であり、
  completed hot-plug engine owner ではないこと
- historical package sequencing / recut は `../../plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
  と `../../docs/reports/` を参照し、この page では request / verdict carrier boundary の確認だけを行うこと

## これではまだ確認できないこと

- actual hot-plug engine actualization
- rollback protocol
- durable migration engine
- distributed activation ordering
- final public hot-plug ABI
- rejoin / reattach semantics

## 関連文書

- `../research_abstract/runtime_crate_hotplug_carrier_admission_cut_01.md`
- `../../plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
- `runtime_crate_hotplug_engine_ownership_cut_01.md`
- `hotplug_real_migration_rollback_boundary_01.md`
- `attachpoint_detach_minimal_contract_01.md`
