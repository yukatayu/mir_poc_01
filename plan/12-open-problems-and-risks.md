# plan/12 — 未解決問題とリスク

## 目的

この文書は current repo の compact risk register である。

## risk register

| 項目 | 種別 | current 状態 | リスク | current 対応 |
|---|---|---|---|---|
| public surface hidden promotion | helper / public API | 継続中 | repo-local helper を final public surface と誤読する | clean suite / future-axis queue / final public API を明示的に分ける |
| old sample drift re-entry | docs / sample policy | 低下したが継続監視 | old line が active path に戻る | archive 分離と stale-reference scan を維持 |
| low-level memory-order premature import | semantics | OPEN | source principal が C++ / ISA vocabulary に引きずられる | high-level relation family を principal に保つ |
| cut-family conflation | semantics | OPEN | `atomic_cut` を global mutex / durable commit と誤読する | local finalization nucleus に固定 |
| typed overclaim | type / proof | OPEN | finite-index first layer を full dependent core と誤読する | first fragment と still-later line を分ける |
| theorem / model-check overclaim | proof workflow | OPEN | generated stub / model-check carrier を production binding と誤読する | repo-local bridge と public contract を分ける |
| auth-transport collapse | Mirrorea layering | 低下したが継続監視 | envelope / adapter 設計で authentication を transport に潰す | `MessageEnvelope / AuthEvidence` seam の helper-local / report-local first cut を維持し、final public widening を後段に残す |
| visualization / telemetry leak | observability | NEW | debug output が label / authority / redaction を回避する | typed visualization / telemetry line を package 化する |
| place / participant / world conflation | sample / docs | NEW | participant、Place、world sugar が core primitive として固定される | `Place` distinction と world-sugar note を README / Documentation / plan に残す |
| goal drift via local optimization | roadmap | NEW | Sugoroku 現行 floor を Mirrorea full runtime 完了と誤読する | project axis と stop line を progress / tasks / docs に明記する |
| sample-progress inflation | dashboard / reporting | NEW | runnable sample や validation の裏付けなしに `%` を上げる | `samples_progress.md` の `%` と report / validation command を結びつける |
| repository taxonomy drift | docs / structure | NEW | sample/script/crate taxonomy が drift し、active/planned/generated/archive が混ざる | `plan/19`、`samples/README.md`、`scripts/README.md`、no-risk-first staged migration を維持する |
| root-disk exhaustion | storage / VPS | NEW | `target/`、LLVM、generated artifact が root を圧迫する | external workdir policy、storage audit、cleanup script を先に入れる |
| unsafe detachable cleanup | storage / ops | NEW | cleanup script が repo source や report を消す | `--confirm` 必須、known disposable dir 限定、detach_prepare non-destructive |
| shared-space profile collapse | shared-space | OPEN | authoritative-room default を exhaustive catalog と誤読する | minimal working subset と final catalog を分ける |
| sugoroku vertical-slice overclaim | Mirrorea / application sample | 継続監視 | single-process logical multi-place emulator を real network / consensus と誤読する | no network / no consensus / no durable commit を closeout で明記する |

## current open problems

- `TermSignature` first cut の exact granularity、reserved kind の扱い、`LayerSignature` law surface
- `MessageEnvelope` / `AuthEvidence` / transport insertion seam の final public shape
- visualization / telemetry security の label / authority / redaction / retention model
  - helper-local / report-local first cut は `0922` で actualize したが、final public viewer contract と retention は未決のまま残す
- projection / placement validity report と world-sugar boundary
  - docs-first plan は `0924` / `plan/20` で actualize したが、generator / optimizer / equivalence checker は未決のまま残す
- `Patch Req Prov Δ` / `AttachPoint` / activation cut / migration contract
  - docs-first plan は `0925` / `plan/21` で actualize したが、final ABI / rollback / durable migration engine は未決のまま残す
- network transport widening / reconnect / failure matrix
  - docs-first plan は `0926` / `plan/22` で actualize したが、final transport ABI / session protocol / multi-server consensus / durable commit は未決のまま残す
- actual LLVM build / backend choice / packaging success criteria
  - guardrail と non-destructive probe は `0927` / `plan/23` で actualize したが、actual LLVM artifact、final backend choice、installed-binary / FFI / engine adapter success criteria は未決のまま残す
- avatar fairy follow residual `FAIRY-05` を active helper に取り込むか
  - docs-first では explicit state timeline / anchor switch evidence gate だけを固定し、
    visibility-return witness の carrier bundling と runnable widening の価値は未決
- final parser grammar
- final parser / checker / runtime / verifier API
- final theorem result / checker artifact / verifier contract
- concrete theorem / model-check tool binding
- final witness / provider / emitted-artifact public contract
- exhaustive shared-space catalog
- real network transport / durable distributed commit / multi-server consensus / detach lifecycle
- packaging / installed binary / FFI / engine adapter

## historical note

pre-clean-near-end prototype chain 由来の detailed package risk は
historical memory として `specs/examples/` と archive 側に残す。
current active risk register は clean near-end current layer と Mirrorea future-axis queue に対して読む。
