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
| detached trace / audit serialization | runtime / tooling | OPEN / comparison 上の先行候補 | trace / audit を helper 内表現に閉じ込めたままでは repo 外保存・再比較・後解析が重い | exact compare core を維持したまま detached artifact の最小 shape を先に切る |
| richer host interface | runtime boundary | OPEN / comparison 上の後続候補 | current host harness を production host に誤昇格しやすく、coverage analysis を先に肥大化させやすい | helper と production host を分離して記述し、detached artifact 境界の後で narrow に切る |
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

- 現在は helper と tests の都合で in-memory compare が中心である
- `TraceAuditSink` / `RunReport` / bundle summary から detached artifact をどう切り出すかは未決である
- ただし detached trace / audit と richer host interface の 2 項目を比べると、前者の方が current parser-free PoC を「大量に回して比較する」段階へ進めやすい
- 先に固定すべき最小 core は、`event_kinds`、formal `non_admissible_metadata`、short `narrative_explanations`、terminal outcome、bundle / profile summary 程度であり、`must_explain` は引き続き human-facing obligation に残す

### richer host interface

- current host harness は current L2 verification basis
- production host interface として扱うと責務が膨らみすぎる
- uncovered call detection、coverage explanation、preflight の必要性は見えているが、先に host API を肥大化させるより detached artifact 境界を切った方が PoC 前進量が大きい
- current host coverage failure が batch summary で文字列検出に依存している点は drift source だが、これは richer host interface を直ちに入れる理由ではなく、後段で typed coverage field を検討する入口とみなす

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
