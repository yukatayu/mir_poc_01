# post P21 distributed activation ordering family 01

## この文書の役割

この文書は、`P21` close 後の
second recommendation closeout memory が
`distributed activation ordering` family であることを、
最短コマンドで読み返す landing page です。

- exact package label は intentionally unfixed のままです
- `rollback / durable migration` family は first recommendation close 済みです
- final public hot-plug ABI はこの文書の scope 外です
- helper-local attach cut を distributed ordering completion と誤読しないことが主眼です

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
- `activation_cut` が still helper-local attach boundary に留まること
- distributed activation ordering family が second recommendation として docs-first に切り出されたこと
- third recommendation family の current repository memory が final public hot-plug ABI であること

## これではまだ確認できないこと

- actual distributed activation ordering implementation
- actual consensus / durable activation commit completion
- actual rollback / durable migration completion
- final public hot-plug ABI

## 関連文書

- `../research_abstract/post_p21_distributed_activation_ordering_family_01.md`
- `../../plan/37-post-p21-distributed-activation-ordering-family.md`
- `post_p21_rollback_durable_migration_family_01.md`
- `hotplug_real_migration_rollback_boundary_01.md`
