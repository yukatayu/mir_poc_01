# Question 008 — final parser grammar freeze の進め方

## 前提

- この質問の前に `plan/` と `specs/` を読んでいる前提です。
- この質問は単独で答えてください。
- current repo は explicit edge-row companion notation を維持しつつ、`mir-ast` / source lowering / fixed-subset sample runner で narrow syntax-backed path を持っています。
- stage1 / stage2 / stage3 attached-slot / predicate fragment / shared single attachment frame までは current actual path に入っています。
- final parser grammar はまだ凍結していません。

## 相談したいこと

parser grammar をいつ・どの順で freeze していくのが自然でしょうか。

課題感は次です。

- grammar を早く固定しすぎると semantics を拘束しやすい
- しかし runnable sample が増えるほど、syntax-backed path を完全に companion 扱いにするのも難しくなる
- current explicit edge-row form と future surface polish の切り分けも必要

聞きたいのは次です。

1. final grammar freeze の entry criteria は何でしょうか。
2. grammar freeze 前に、parser carrier / AST / checker-facing subset で先に固定すべきものは何ですか。
3. companion notation を長く残してよい部分と、早めに parser grammar に寄せるべき部分は何ですか。
4. sample corpus と formal bridge の両方に手戻りが少ない freeze strategy は何ですか。

## 期待する回答

- staged grammar freeze の段階案をください。
- 「今の repo では freeze してよい / まだ freeze しない」を分けてください。
