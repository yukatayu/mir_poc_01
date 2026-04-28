# post P21 final public hot-plug ABI family 01

## この文書の役割

この文書は、`P21` close 後の later family のうち
**`final public hot-plug ABI` を third recommendation として読む current summary**
です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/38-post-p21-final-public-hotplug-abi-family.md`
- 実行証跡は `docs/reports/` にあります

## current reading

- `rollback / durable migration` family と
  `distributed activation ordering` family は
  docs-first close 済みです
- current self-driven third recommendation も
  docs-first close 済みであり、
  ここで fixed したのは
  `freeze prerequisite fixed; public ABI still unfrozen`
  という line です
- helper-local `hotplug_lifecycle`、
  sample anchor IDs、
  view IDs、
  telemetry row IDs は preview-only に残します
- runtime-private `HotPlugRequest` / `HotPlugVerdict`、
  `HotPlugRuntimeEngineState` / `HotPlugRuntimeEngineReport`、
  `reason_refs`、
  `active_membership_epoch`
  は non-public anchor に残します
- actual public request/response/event naming、
  `AttachPoint` / `Patch` package catalog naming、
  actual shipped public ABI surface は
  post-`P18` mixed gate / `U1` hold line に残します

## ここでまだ fixed しないこと

- actual final public request/response/event names
- actual final public `AttachPoint` / `Patch` package catalog names
- actual shipped public hot-plug ABI surface
- actual packaging / host integration / engine adapter commitment
- transport / auth / membership / capability / witness の final public schema

## stop line

- helper-local preview naming を final public ABI naming と書かない
- runtime-private naming を final public ABI naming と書かない
- third recommendation docs-first close を actual final public ABI freeze と書かない

## どこを見るか

- repository memory:
  `../../plan/38-post-p21-final-public-hotplug-abi-family.md`
- mixed gate bridge:
  `../../plan/27-public-api-parser-gate-roadmap.md`
  `../../plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- landing page:
  `../hands_on/post_p21_final_public_hotplug_abi_family_01.md`
