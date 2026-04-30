# post P20 hot-plug next package inventory 01

## この文書の役割

この文書は、`R7` historical closeout memory と `P21` runtime-private engine-state floor を最短コマンドで確認する landing page です。

- final public completion ではありません
- `P20` thin assembly、`P21` runtime-private narrow floor、post-`P21` family pointers の boundary だけを確認します
- rollback / migration / ordering / public ABI は completion claim せず、post-`P21` family docs から読む対象として残します

## まず実行するコマンド

```bash
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json
python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
cargo test -p mir-runtime --test hotplug_runtime_skeleton
cargo test -p mir-runtime
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

## これで確認できること

- `P20` closeout が
  admitted carrier + existing substrate の thin runtime/report assembly に留まること
- helper-local `hotplug_lifecycle` / sample-grounded IDs が
  依然 preview ownership に残ること
- `P21` runtime-crate hot-plug engine-state narrow floor が
  repo-local current layer で close 済みであること
- `HotPlugRuntimeEngineState` / `HotPlugRuntimeEngineReport`、
  `assemble_hotplug_runtime_engine_report()`、
  `build_hotplug_runtime_engine_report()`
  により runtime-private engine-state report が narrow に actualize されたこと
- post-`P21` family docs が
  `rollback / durable migration`
  `distributed activation ordering`
  `final public hot-plug ABI`
  の 3 pointer に分けて読まれること

## これではまだ確認できないこと

- rollback protocol completion
- durable migration / reattach semantics completion
- distributed activation ordering completion
- final public hot-plug ABI

## 関連文書

- `../research_abstract/post_p20_hotplug_next_package_inventory_01.md`
- `../../plan/35-post-p20-hotplug-next-package-inventory.md`
- `runtime_crate_hotplug_carrier_admission_cut_01.md`
- `hotplug_real_migration_rollback_boundary_01.md`
- `runtime_crate_hotplug_engine_ownership_cut_01.md`
