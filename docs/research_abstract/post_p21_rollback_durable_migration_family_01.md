# post P21 rollback / durable migration family 01

## この文書の役割

この文書は、`P21` close 後の later family のうち
**`rollback / durable migration` を first recommendation として読む current summary**
です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/36-post-p21-rollback-durable-migration-family.md`
- 実行証跡は `docs/reports/` にあります

## current reading

- `P21` で runtime-side engine state progression は narrow に actualize 済みです
- その次に first recommendation として harden するのは
  exact label 未固定のままの `rollback / durable migration` family です
- ここでいう family は
  rollback state machine、durable migration、reattach semantics を
  まだ 1 bucket に保つ current recommendation を指します
- `distributed activation ordering` は second recommendation、
  `final public hot-plug ABI` は third recommendation に残します

## ここでまだ fixed しないこと

- actual rollback protocol completion
- actual durable migration engine completion
- distributed activation ordering contract
- final public hot-plug ABI / package catalog naming
- exact package label / numbering

## stop line

- `migration_contract` を protocol と書かない
- detach 後 reject を completed rollback と書かない
- helper-local anchor naming を runtime-canonical or public ABI と書かない

## どこを見るか

- repository memory:
  `../../plan/36-post-p21-rollback-durable-migration-family.md`
- boundary preconditions:
  `../../plan/32-hotplug-real-migration-rollback-boundary.md`
  `../../plan/35-post-p20-hotplug-next-package-inventory.md`
- landing page:
  `../hands_on/post_p21_rollback_durable_migration_family_01.md`
