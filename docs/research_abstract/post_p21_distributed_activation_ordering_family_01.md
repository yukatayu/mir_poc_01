# post P21 distributed activation ordering family 01

## この文書の役割

この文書は、`P21` close 後の later family のうち
**`distributed activation ordering` を second recommendation として読む current summary**
です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/37-post-p21-distributed-activation-ordering-family.md`
- 実行証跡は `docs/reports/` にあります

## current reading

- `rollback / durable migration` family hardening は
  first recommendation close 済みです
- その次に second recommendation として harden するのは
  exact label 未固定のままの `distributed activation ordering` family です
- ここでいう family は
  multi-place / multi-server attach activation ordering、
  durable activation commit、
  visible active state frontier の widening criteria を指します
- current remaining third recommendation は
  `final public hot-plug ABI` です

## ここでまだ fixed しないこと

- actual rollback protocol completion
- actual durable migration engine completion
- actual distributed activation ordering implementation
- actual consensus / network commit protocol
- final public hot-plug ABI / package catalog naming
- exact package label / numbering

## stop line

- `activation_cut` を distributed activation ordering completion と書かない
- helper-local `local_queue` attach sample を durable commit proof と書かない
- transport / auth / membership / capability / witness を collapse しない
- runtime-private naming を public ABI と書かない

## どこを見るか

- repository memory:
  `../../plan/37-post-p21-distributed-activation-ordering-family.md`
- first recommendation memory:
  `../../plan/36-post-p21-rollback-durable-migration-family.md`
- boundary preconditions:
  `../../plan/32-hotplug-real-migration-rollback-boundary.md`
  `../../plan/35-post-p20-hotplug-next-package-inventory.md`
- landing page:
  `../hands_on/post_p21_distributed_activation_ordering_family_01.md`
