# Phase 6 要約 — compile-ready minimal actualization

## この phase の役割

Phase 6 は、Phase 1〜5 で固定した semantics / parser-free substrate / proof boundary を、**`mir-ast` / `mir-semantics` / `mir-runtime` の narrow code path と runnable sample lane に actualize する phase** である。

## 固まった current reading

- current code path は non-production minimal cut に留める。
- source-backed current-L2 path は
  - parser carrier
  - checker/runtime skeleton
  - formal hook
  - source sample lowering / runner / ladder
  - helper-local `debug_outputs` / `verification_preview` / `artifact_preview`
  - corrected prototype explicit-path execution lane
  - theorem/model-check first pilot
  を narrow に揃えている。
- authored current-L2 sample sixteen は fixed である。
- corrected runnable prototype nonet は `samples/prototype/`、exact rough stimulus は `samples/not_implemented/` に分ける bucket policy を fixed 済みである。
- sample 冒頭の日本語 intent comment は、leading contiguous `#` block だけを helper-local convenience として許す。
- operational CLI は explicit sample path と adjacent sidecar を使って current sample debug を見やすくする thin shell concern に留める。
- helper-local debug output preview として、`*_output` / `*_pipe` / `debug_*` target の record を `debug_outputs` として表示してよい。
  ただしこれは final stdio / final host-I/O semantics ではない。
- theorem/model-check bridge の current floor では、CLI が `verification_preview` と helper-local `artifact_preview` を出してよい。
  ただしこれは final public verifier contract や final emitted artifact schema を意味しない。
- narrow mixed-gate pre-floor として、sample-local preview-aligned typed artifact route を test/support helper に置き、prototype bucket を含む compare floor を固定してよい。

## source-backed evidence

- `mir-ast` current L2 parser carrier
- `mir-semantics` minimal interpreter / formal hook / model-check carrier
- `mir-runtime` source sample runner / operational CLI
- authored current-L2 sixteen-row corpus
- corrected prototype nonet `p01 / p02 / p03 / p04 / p05 / p06 / p07 / p08 / p09`
- exact rough stimulus preservation bucket A-D under `samples/not_implemented/order-handoff/`

## まだここで決めていないこと

- final parser grammar
- final public parser / checker / runtime API
- installed binary / packaging success criteria
- concrete theorem / model-check tool binding
- shared-space final operational catalog
- backend / external codegen

## 次へ渡したもの

Phase 6 はここで終わりではなく、
- execution lane
- theory-lab lane
- reserve integration lane
を並走させる継続 phase として読む。
current first-line integration queue 自体は close 済みだが、representative trace-alignment / reserve surface package を current reopen queue に戻している。corrected runnable version の current floorは reached である。
- current principal self-driven reopen package は none である。
- corrected prototype tranche close を theory-lab solved と読まない
- verifier-boundary / typed-theorem-model-check line は `specs/examples/459` で current first-line integration close と読む
- order / handoff line は `specs/examples/460` で current first-line integration close と読む
- syntax / modality line は `specs/examples/461` で current first-line integration close と読む
- near-end closeout / mixed-gate-only reading は `specs/examples/462` で同期済みと読む。
- verifier preview alignment pre-floor は `specs/examples/463` で同期済みと読む。
- model-check projection pre-floor は `specs/examples/464` で同期済みと読む。
- theorem discharge pre-floor は `specs/examples/465` で同期済みと読む。
- theorem discharge actual-format probe は `specs/examples/479` で同期済みと読む。
- model-check property/tool-seam probe は `specs/examples/480` で同期済みと読む。
- theorem discharge / public-theorem-contract threshold default は `specs/examples/481` で同期済みと読む。
- theorem contract shape threshold default は `specs/examples/485` で同期済みと読む。
- theorem transport/public-contract coupled later gate は `specs/examples/486` で同期済みと読む。
- theorem review-unit transport / notebook-contract actual adoption は `specs/examples/487` で同期済みと読む。
- theorem result-object preview / proof-object-schema reserve actualization は `specs/examples/491` で同期済みと読む。
- theorem result-object route actual adoption は `specs/examples/500` で同期済みと読む。
- theorem final public-contract reopen threshold は `specs/examples/506` で同期済みと読む。
- theorem Lean-first non-production stub pilot actualization、repo-local artifact-conformance bridge、representative trace-alignment bridge は `specs/examples/508`、`509`、`510` で同期済みと読む。
- theorem proof-object schema / prover-brand coupled later gate は `specs/examples/494` で同期済みと読む。
- model-check final public-contract reopen threshold は `specs/examples/507` で同期済みと読む。
- model-check row-local property / checker-boundary actual adoption は `specs/examples/488` で同期済みと読む。
- model-check public-checker artifact preview / verifier-handoff reserve actualization は `specs/examples/492` で同期済みと読む。
- model-check tool-brand / verifier-handoff coupled later gate は `specs/examples/495` で同期済みと読む。
- model-check public checker artifact / migration coupled later gate は `specs/examples/498` で同期済みと読む。
- model-check checker-artifact route actual adoption は `specs/examples/501` で同期済みと読む。
- witness/provider public-schema coupled later gate は `specs/examples/499` で同期済みと読む。
- witness/provider route actual adoption は `specs/examples/502` で同期済みと読む。
- witness/provider schema route actual adoption は `specs/examples/504` で同期済みと読む。
- witness/provider final public-contract reopen threshold は `specs/examples/505` で同期済みと読む。
- order-handoff source wording / emitted-artifact coupled later gate は `specs/examples/496` で同期済みと読む。
- order-handoff source wording route actual adoption は `specs/examples/503` で同期済みと読む。
- theorem result object / payload public-contract coupled later gate は `specs/examples/497` で同期済みと読む。
- witness/provider/artifact public-shape actual adoption は `specs/examples/489` で同期済みと読む。
- witness/provider public-contract / emitted-handoff coupled later gate は `specs/examples/493` で同期済みと読む。
- order-handoff surface actual adoption は `specs/examples/490` で同期済みと読む。
- model-check property/tool-brand threshold default は `specs/examples/482` で同期済みと読む。
- witness/provider/artifact public-shape threshold default は `specs/examples/483` で同期済みと読む。
- order-handoff surface / artifact threshold default は `specs/examples/484` で同期済みと読む。
一方で、actual typed-surface adoption、final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema、first settled property language / concrete tool brand / final public checker artifact / actual emitted verifier handoff artifact、shared-space final fairness/replay profile、installed-binary promotion は mixed gate に残る。
