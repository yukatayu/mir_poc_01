# plan/12 — 未解決問題とリスク

## 目的

この文書は current repo の compact risk register である。

## risk register

| 項目 | 種別 | current 状態 | リスク | current 対応 |
|---|---|---|---|---|
| public surface hidden promotion | helper / public API | 継続中 | repo-local helper を final public surface と誤読する | clean suite と final public API を明示的に分ける |
| old sample drift re-entry | docs / sample policy | 低下したが継続監視 | old `p..` chain が active path に戻る | archive 分離と migration note を維持 |
| low-level memory-order premature import | semantics | OPEN | source principal が C++/ISA vocabulary に引きずられる | high-level relation family を principal に保つ |
| cut-family conflation | semantics | OPEN | `atomic_cut` を global mutex / durable commit と誤読する | local finalization nucleus に固定 |
| typed overclaim | type / proof | OPEN | finite-index first layer を full dependent core と誤読する | first fragment と still-later line を分ける |
| theorem/model-check overclaim | proof workflow | OPEN | generated stubs / model-check carrier を production binding と誤読する | repo-local bridge と public contract を分ける |
| solver / artifact blow-up | tooling | OPEN | constraint / artifact / trace volume が急増する | finite first fragment と helper-local threshold を維持 |
| shared-space profile collapse | shared-space | OPEN | authoritative-room default を exhaustive catalog と誤読する | minimal working subset と final catalog を分ける |
| sugoroku vertical-slice overclaim | Mirrorea / application sample | NEW | single-process logical multi-place emulator を real network / consensus 実装と誤読する | limitations と closeout で no network / no consensus / no durable commit を明記する |

## current open problems

- final parser grammar
- final parser/checker/runtime/verifier API
- final theorem result / checker artifact / verifier contract
- concrete theorem/model-check tool binding
- final witness/provider/emitted-artifact public contract
- exhaustive shared-space catalog
- Sugoroku detach lifecycle implementation
- packaging / installed binary / FFI

## historical note

pre-clean-near-end prototype chain 由来の detailed package risk は
historical memory として `specs/examples/` と archive 側に残す。
current active risk register は clean near-end current layer に対して読む。
