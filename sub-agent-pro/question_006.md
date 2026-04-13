# Question 006 — モデル検査ラインの first concrete carrier と first tool cut

## 前提

- この質問の前に `plan/` と `specs/` を読んでいる前提です。
- この質問は単独で答えてください。
- current repo では、tool-neutral formal hook から row-local machine-facing sibling artifact として model-check concrete carrier first actualization までは入っています。
- source sample emitted artifact wiring と sample-facing summary / bless-review flow も narrow に actualize 済みです。
- ただし concrete model checker binding と finite-state abstraction の設計はまだ未決です。

## 相談したいこと

model-check 側を本格化するなら、今の carrier / artifact line をどう concrete tool へ接続するのが自然でしょうか。

聞きたいことは次です。

1. first concrete tool family として、TLA+ / Alloy / Ivy / Apalache / explicit-state model checker / 自前 finite abstraction など、どの方向がこの repo に合うでしょうか。
2. current carrier は row-local / machine-facing sibling artifact ですが、これを state machine / transition system / temporal obligation へどう持ち上げるのが良いでしょうか。
3. first model-check line で対象にすべき property は何ですか。
   - rollback-cut non-interference
   - authority transition safety
   - no re-promotion
   - route proof / overlay path
   - fairness / liveness
4. まだ model-check へ送らず theorem-side や static checker 側に残すべき性質は何ですか。

## 期待する回答

- first concrete tool cut の推奨順序を示してください。
- property ごとに、static / theorem / model-check / runtime どこへ置くべきかを分類してください。
