# hot-plug real migration / rollback boundary 01

## 目的

`R4` の current line は、
helper-local hot-plug evidence を widened implementation と誤読しないために、
current minimal contract row の先に残る kept-later boundary を読むことです。

## まず確認するコマンド

```bash
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json
python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
```

## どこを見るか

- `activation_cut`
  requested attach と active state mutation の分離
- `detach_boundary`
  detach 後 action rejection の explicit boundary
- `migration_contract`
  migration / rollback が unresolved のまま visible であること
- `hotplug_scope = helper_local_package_manager_preview`
  current ownership が helper-local preview に留まること

## ここから読んではいけないもの

- distributed activation ordering
- rollback protocol completion
- durable migration engine
- runtime-crate hot-plug engine ownership
- final public hot-plug ABI

## current reading

`R4` は helper output を増やす package ではありません。
current helper floor をそのまま保ちつつ、
何がまだ deferred かを docs-first に固定する package です。
historical next relation at `R4` close time は `R5` runtime-crate hot-plug engine ownership cut でした。
current repo state では `P21` も close 済みであり、
exact next label intentionally unfixed のまま
`rollback / durable migration` family hardening を first recommendation として読みます。

## 関連

- `../research_abstract/hotplug_real_migration_rollback_boundary_01.md`
- `../../plan/32-hotplug-real-migration-rollback-boundary.md`
- `post_p21_rollback_durable_migration_family_01.md`
- `attachpoint_detach_minimal_contract_01.md`
- `current_phase_closeout_01.md`
