# post P20 hot-plug next package inventory 01

## この文書の役割

この文書は、`R7` の **docs-first closeout memory** を reader-facing に短く読む summary です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/35-post-p20-hotplug-next-package-inventory.md`
- 実行証跡は `docs/reports/` にあります

## `R7` で fixed したこと

- `P20` close 後も remaining lane を vague な `later` に戻さず、
  smallest plausible package cuts に分けて読む
- current promoted-next package は
  **`P21` runtime-crate hot-plug completed-engine narrow cut**
  とする
- `P21` は
  admitted request / verdict carrier と existing runtime substrate の上に
  canonical runtime-side engine state progression を narrow に置く package である
- package-level reopen next の exact label はまだ fixed しないが、
  later family は
  - `rollback / durable migration`
  - `distributed activation ordering`
  - `final public hot-plug ABI`
  に grouped して読む

## まだ fixed していないこと

- actual `P21` implementation
- rollback protocol completion
- durable migration / reattach semantics completion
- distributed activation ordering completion
- final public hot-plug ABI
- exact later package labels

## どこを見るか

- repository memory:
  `plan/35-post-p20-hotplug-next-package-inventory.md`
- precondition memory:
  `plan/32-hotplug-real-migration-rollback-boundary.md`
  `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
  `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
- landing page:
  `../hands_on/post_p20_hotplug_next_package_inventory_01.md`

## stop line

- `P21` promoted-next を completed engine と読まない
- `rollback / durable migration` family を `P21` に混ぜない
- helper-local anchor naming を public ABI と読まない
