# post P20 hot-plug next package inventory 01

## この文書の役割

この文書は、`R7` の **historical closeout bridge** を reader-facing に短く読む summary です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/35-post-p20-hotplug-next-package-inventory.md`
- 実行証跡は `docs/reports/` にあります

## `R7` で fixed したこと

- `P20` close 後も deferred concerns を vague な `later` に戻さず、
  smallest plausible package cuts に分けて読む
- `P21` runtime-crate hot-plug engine-state narrow floor は
  repo-local current layer で close 済みである
- `P21` では
  admitted request / verdict carrier と existing runtime substrate の上に
  `HotPlugRuntimeEngineState` / `HotPlugRuntimeEngineReport`、
  consumer-side `assemble_hotplug_runtime_engine_report()`、
  example `build_hotplug_runtime_engine_report()` を置き、
  runtime-private engine-state report を narrow に actualize した
- `R7` close-time grouped family memory は historical であり、
  live status / next reopen point は `../../progress.md` と `../../tasks.md` を参照する
- post-`P21` family docs は
  - `rollback / durable migration`
  - `distributed activation ordering`
  - `final public hot-plug ABI`
  の 3 pointer として読む

## まだ fixed していないこと

- rollback protocol completion
- durable migration / reattach semantics completion
- distributed activation ordering completion
- final public hot-plug ABI
- exact later package labels / point-in-time package ownership at `R7` close time

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

- `P21` close を runtime crate hot-plug broad completion と読まない
- `rollback / durable migration` family を `P21` に混ぜない
- helper-local anchor naming を public ABI と読まない
