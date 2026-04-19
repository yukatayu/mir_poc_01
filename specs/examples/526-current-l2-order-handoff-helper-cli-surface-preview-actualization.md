# 526 — current L2 order-handoff helper CLI surface-preview actualization

## 目的

Package 58 の helper / CLI hardening として、

- `p07 / p08 / p09` の current order-handoff surface recommendation を
  CLI から inspectable に見せられるか
- final grammar や final public parser / checker / runtime API に上げずに、
  helper-local preview として止められるか

を整理する。

ここで actualize するのは
**`mir-current-l2 run-source-sample --format json|pretty` が `surface_preview` を出し、
principal companion / stage-block secondary / serial-scope reserve を sample-local に見せる current cut**
である。

したがって、ここでは次を fixed しない。

- final parser grammar
- final public parser / checker / runtime API
- final source-surface handoff wording
- final emitted-artifact / emitted-handoff schema
- final public witness/provider/artifact contract

## current question

helper / CLI hardening の current package で、
すでに compare floor にある order-handoff surface family

- explicit edge-row companion
- stage-block secondary candidate
- authoritative-room `serial on ...` reserve

を CLI preview として actualize してよいか。

## current first line

current recommendation は次である。

1. CLI `run-source-sample` summary に helper-local `surface_preview` を追加する
2. `surface_preview` は
   - `minimal_companion`
   - `stage_block_secondary`
   - `serial_scope_reserve`
   の 3 section に分ける
3. `p07 / p08` では principal companion と stage-block secondary と serial reserve を reached として見せる
4. `p09` では delegated provider practical cut に合わせて `serial_scope_reserve` だけを reached とし、`minimal_companion` と `stage_block_secondary` は guarded に保つ
5. compare / guard / kept-later refs を同時に載せるが、helper-local preview に留める

## actualized CLI preview

| sample | `minimal_companion` | `stage_block_secondary` | `serial_scope_reserve` | current reading |
|---|---|---|---|---|
| `p07-dice-late-join-visible-history` | `Reached` | `Reached` | `Reached` | authoritative-room default profile の publication → handoff → observe lineを sample-local preview に出す |
| `p08-dice-stale-reconnect-refresh` | `Reached` | `Reached` | `Reached` | fail-then-refresh / replay invalidate lineを sample-local preview に出す |
| `p09-dice-delegated-rng-provider-placement` | `Guarded` | `Guarded` | `Reached` | delegated provider practical cut に合わせて serial reserve だけを sample-local preview に出す |

## なぜこの cut でよいか

- order-handoff surface family 自体は `specs/examples/472`、`473`、`511` で compare / reserve / narrowing floor を already 持つ。
- helper/CLI hardening の段階では、new syntax を parser に入れるより、既存 recommendation を inspectable preview に actualize する方が adoption debt を減らしやすい。
- `p09` を `serial_scope_reserve` だけ reached にすることで、
  provider placement practical cut と source principal edge-row family を collapse せずに保てる。

## evidence

- CLI runtime summary
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/examples/mir_current_l2.rs`
- regression
  - `crates/mir-runtime/tests/current_l2_order_handoff_surface_preview_cli.rs`
- prior sample-visible floors
  - `specs/examples/472-current-l2-minimal-companion-experimental-order-handoff-surface.md`
  - `specs/examples/473-current-l2-order-handoff-surface-narrowing-and-stage-block-secondary-candidate.md`
  - `specs/examples/511-current-l2-order-handoff-serial-scope-reserve-surface.md`

## retained later

- final parser grammar
- final public parser / checker / runtime API
- final source-surface handoff wording
- final emitted-artifact schema
- final emitted-handoff contract
- final public witness/provider/artifact contract

## stop line

この package の stop line は、

- CLI `run-source-sample --format json|pretty` が `surface_preview` を返す
- `p07 / p08 / p09` の reached / guarded reading が current sample role と一致する
- docs / roadmap / snapshot が helper/CLI hardening の actual package として追える

の 3 点である。
