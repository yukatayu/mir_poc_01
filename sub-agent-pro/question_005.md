# Question 005 — 定理証明ラインの first serious formalization

## 前提

- この質問の前に `plan/` と `specs/` を読んでいる前提です。
- この質問は単独で答えてください。
- current repo では、tool-neutral formal hook、`proof_notebook_review_unit` first pilot、plain bridge sketch first actualization まではあります。
- ただし concrete theorem prover binding はまだ選んでいません。
- semantics 側では、fallback / no re-promotion / rollback-cut non-interference / ownership monotonicity / same-lineage floor など、証明したい候補が既にあります。

## 相談したいこと

theorem-side を本格化する場合、どの relation / invariant / proof object から formalization を始めるのが自然でしょうか。

特に迷っているのは次です。

1. まず machine-checked proof に乗せるべき性質は何ですか。
2. proof object は syntax ではなく semantic core relation を対象にすべきだと思っていますが、その切り方は妥当ですか。
3. first concrete prover として、Lean / Coq / Isabelle / F* など、どの family がこの repo の性質に合いそうですか。
4. current `proof_notebook_review_unit` pilot から、どこまで narrow に concrete tool へ橋を架けるべきでしょうか。

## 期待する回答

- first theorem line の対象 invariant を優先順で示してください。
- prover family の比較を、repo の current semantics との相性で整理してください。
- 「いま concrete binding を入れてよい範囲」と「まだ docs-only に留める範囲」を分けてください。
