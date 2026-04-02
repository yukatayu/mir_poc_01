# plan/12 — 未解決問題とリスク

## 目的

この文書は、current repo の未解決問題と risk を 1 箇所で管理する。
未決のものは未決と書き、将来 workstream と current L2 settled judgment を混ぜない。

## risk register

| 項目 | 種別 | current 状態 | リスク | current 対応 |
|---|---|---|---|---|
| fallback intuition drift | semantics / notation | 継続中 | outer-longer-lifetime wrapper と誤読しやすい | prose、fixture、notation 比較で drift を明示 |
| notation の outer/inner 誘発 | notation | 継続中 | nested 直感が chain semantics を上書きする | explicit edge-row form を維持 |
| final parser grammar 未固定 | syntax | OPEN | syntax を早く決めすぎると semantics を拘束する | companion notation に留める |
| review infrastructure の返答遅延 | process | 継続中 | task close が reviewer 依存で滞る | 最後に 1 回長く待ち、必要なら retry 1 回、なお無理なら local evidence を report に残す |
| machine-readable catalog / manifest を今は入れないこと | architecture | current 方針 | hard-coded catalog と externalization 比較が再燃しやすい | current L2 では comparison 止まりと明記 |
| path canonicalization | helper / packaging | OPEN | selector / single-fixture / sidecar discovery の長期互換性に影響 | current L2 では minimal behavior のみ保持 |
| detached trace / audit serialization | runtime / tooling | OPEN | trace / audit を helper 内表現に閉じ込める危険 | current helper は in-memory compare に限定 |
| richer host interface | runtime boundary | FUTURE | current host harness を production host に誤昇格しやすい | helper と production host を分離して記述 |
| multi-request scheduler | runtime | FUTURE | current direct-style interpreter と概念が混ざる | 現時点では未着手を明示 |
| `Approximate` / `Compensate` | semantics / runtime | FUTURE | failure space と rollback を広く再設計する必要がある | 今は plan に残すだけ |

## 各項目の補足

### fallback intuition drift

- current L2 では fallback は guarded option chain である
- しかし見た目や用語から outer wrapper 読みが起きやすい
- これは semantics を変えるより、prose / examples / regression fixture で drift を抑える方針を採っている

### notation の outer/inner 誘発

- line-leading `>` ladder は compact だが、operator / nested wrapper / expression 風の誤読を誘発しやすい
- explicit edge-row form はやや重いが、current semantics を誤読させにくい

### final parser grammar 未固定

- current companion notation は parser-ready final syntax ではない
- final punctuation、keyword、indentation discipline、serialization contract は未決である

### machine-readable catalog / manifest を今は入れない理由

- aliases がまだ 4 個規模であり、hard-coded table が最小
- asset 化すると loader / placement / validation / path policy が先に膨らむ
- current L2 では利点より固定圧の方が大きい

### path canonicalization

- `single-fixture` selector と sidecar discovery は現状の repo layout に依存している
- 長期固定するには canonical path policy が要るが、current L2 ではまだ決めない

### detached trace / audit serialization

- 現在は helper と tests の都合で in-memory compare が中心
- trace を detached asset として運ぶ形式は未決

### richer host interface

- current host harness は current L2 verification basis
- production host interface として扱うと責務が膨らみすぎる

## 何を未決のまま残すか

次は current L2 で無理に決めない。

- final parser syntax
- machine-readable catalog asset / manifest 採用
- selector grammar / alias grammar の長期固定
- path canonicalization policy
- detached trace / audit serialization
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate`

## update の見方

各 task でこれらを新たに決めた場合は、この文書を更新する。
決まっていない場合は、決まっていない理由を report に残し、この文書の status を維持する。
