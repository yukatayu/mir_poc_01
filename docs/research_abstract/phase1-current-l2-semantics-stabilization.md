# Phase 1: current-L2 semantics stabilization

## この phase の意味

Phase 1 は、Mir current-L2 の意味論を **まだ final public syntax に固定しないまま** 安定化する層です。
ここでは parser より先に、因果、effect、ownership、lifetime、contract、rollback frontier の意味側を固めます。

## 2026-04-23 時点で固まっていること

- `atomic_cut` は local finalization / rollback frontier
- compatibility-preserving overlay と DAG-safe evolution を core invariant として維持
- order / handoff の principal reading は high-level relation family
- domain predicate を magical builtin にしない

## この phase の current output

current clean near-end suite では、意味論の読みが sample まで接続されています。

- typing:
  authority / label / capture / region / cost を finite theory で読む
- order-handoff:
  publish / witness / handoff の因果関係を relation として読む
- modal:
  `stable` / `later` / `published` / `witnessed` の mode line を raw modal symbol なしで読む

## まだ残ること

- final public parser grammar と punctuation の凍結
- final public source principal と helper-local wording の切り分けの固定
- richer surface syntax をどこまで official にするか

## 関連する summary / detail

- `clean_near_end_typing_01.md`
- `clean_near_end_order_model_01.md`
- `clean_near_end_modal_01.md`
