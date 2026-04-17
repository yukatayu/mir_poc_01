# Phase 1 要約 — current L2 semantics stabilization

## この phase の役割

Phase 1 は、**parser より先に current L2 の意味論を固める phase** である。

## 固まった current reading

- fallback は guarded option chain で読む。
- degradation は left-to-right monotone で、no re-promotion を守る。
- `require / ensure` は request-local、`admit` は option-local である。
- `try { ... } fallback { ... }` と option-chain fallback は別 layer である。
- `atomic_cut` は local rollback frontier に関わる最小核であり、global sync や durable commit ではない。

## source-backed evidence

- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- current L2 companion notation と invariant wording

## まだここで決めていないこと

- final parser grammar
- full typed surface
- theorem / model-check の concrete contract
- higher-level ordering / fairness family

## 次へ渡したもの

Phase 2 はこの semantics を parser-free に実行・比較する。
Phase 3 はこの semantics を壊さずに parser boundary と first checker cut を narrow に切る。
