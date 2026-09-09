# Question 007 — language core / static checker / external verifier の境界

## 前提

- この質問の前に `plan/` と `specs/` を読んでいる前提です。
- この質問は単独で答えてください。
- current repo では、language core を過度に肥大化させず、proof / analysis obligation は helper stack や external verifier へ送る前提を強めています。
- 一方で、same-lineage floor、capability strengthening floor、missing-option structure floor など、current static checker 候補もかなり見えてきています。

## 相談したいこと

どこまでを language core / static checker に入れ、どこからを theorem prover / model checker / out-of-band analyzer に送るのが自然でしょうか。

特に知りたいのは次です。

1. decidable core として、今の repo が早めに固定してよい static judgment は何ですか。
2. 逆に、undecidable か heavy なので external verifier 側へ送るべき obligation は何ですか。
3. current fixed-subset sample line と相性が良い boundary cut はどれですか。
4. 将来の full 型システムや richer verifier を見据えたとき、いまの minimal boundary をどう刻むと後戻りが少ないですか。

## 期待する回答

- `language core`
- `static checker`
- `theorem prover`
- `model checker`
- `runtime / audit`

の 5 列くらいで、主要 obligation の置き場所を整理してください。
