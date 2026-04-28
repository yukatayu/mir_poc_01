# runtime crate hot-plug engine ownership cut 01

## 何を見る文書か

この文書は、`R5` docs-first package の owner split を
実行コマンド付きで短く確認する hands-on です。

## 実行コマンド

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

## これで確認すること

- helper closeout が
  `hotplug_scope = helper_local_package_manager_preview`
  と
  `not completed migration/rollback/runtime-crate ownership`
  を返すこと
- attach/detach anchor は helper-local `MessageEnvelope` evidence として見えていること
- `mirrorea-core` は generic carrier / logical runtime substrate を test pass で維持していること
- `mir-runtime` は thin runtime/report assembly のまま green であり、
  hot-plug engine を current package で claim していないこと

## 読み方

- helper-local `hotplug_lifecycle` を engine state machine と読まない
- `MessageEnvelope` generic carrier ownershipを、
  hot-plug protocol ownershipへ勝手に拡張しない
- `LogicalPlaceRuntimeShell` を world/game aggregate や attach/detach engine と読まない
- Python/Rust duplication が残っていることを、
  ownership migration incomplete の evidence として読む

## 関連文書

- `../research_abstract/runtime_crate_hotplug_engine_ownership_cut_01.md`
- `../../plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
- `hotplug_real_migration_rollback_boundary_01.md`
