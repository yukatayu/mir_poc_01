# post P21 rollback / durable migration family 01

## この文書の役割

この文書は、`P21` close 後に
current self-driven first recommendation が
`rollback / durable migration` family であることを、
最短コマンドで読み返す landing page です。

- exact package label は intentionally unfixed のままです
- distributed activation ordering と final public hot-plug ABI はこの文書の scope 外です
- helper-local preview を completed rollback / migration と誤読しないことが主眼です

## まず実行するコマンド

```bash
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json
python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
cargo test -p mir-runtime --test hotplug_runtime_skeleton
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

## これで確認できること

- `P21` close 後の runtime-side engine state progression narrow cut
- `migration_contract` が still unresolved-state honesty lane に留まること
- detach 後 reject が explicit boundary evidence であり、completed rollback ではないこと
- current self-driven first recommendation が `rollback / durable migration` family であること

## これではまだ確認できないこと

- actual rollback protocol completion
- actual durable migration engine / reattach semantics completion
- distributed activation ordering completion
- final public hot-plug ABI

## 関連文書

- `../research_abstract/post_p21_rollback_durable_migration_family_01.md`
- `../../plan/36-post-p21-rollback-durable-migration-family.md`
- `post_p20_hotplug_next_package_inventory_01.md`
- `hotplug_real_migration_rollback_boundary_01.md`
