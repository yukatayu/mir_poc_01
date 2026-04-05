# plan/13 — 重い将来 workstream

## 目的

この文書は、今すぐ着手しないが計画には明示的に入れておくべき重い workstream を整理する。
current L2 parser-free PoC の延長だけでは扱いきれない論点を、将来の独立した workstream として切り出す。

## なぜ今 plan に含めるのか

- 後で急に現れる論点ではないため
- 現在の semantics / helper / notation の設計が、将来の型・解析・証明可能性にどの程度影響するかを意識しておく必要があるため
- current L2 でまだ扱わない理由を明文化し、scope creep を防ぐため

## workstream 1. 型システムの強さ

### 主題

- ownership / lifetime / contract / effect / capability をどこまで型レベルへ持ち上げるか
- inference と annotation のバランス
- linearity / monotonicity をどこまで型規則で強制するか

### entry criteria

- current L2 semantics の failure space と fallback 読みが十分安定していること
- parser 境界の最小 shape が見えていること

### current inventory note

- `place` / `try-fallback` / `perform on` / `perform via` / statement-local `require` / `ensure` / option declaration core / option-local `admit` / explicit edge-row family までは first parser cut 候補として inventory 化してよい。
- exact lexical polish と richer predicate grammar はまだ companion / OPEN に残す。
- same-lineage static evidence floor、malformed / underdeclared rejection、minimal capability strengthening prohibition、request-local / option-local clause attachment、minimal predicate fragment、`try` / rollback locality の structural floor までは first checker cut 候補として inventory 化してよい。

### 今すぐやらない理由

- 現在は syntax と semantics の companion 整理段階であり、型規則を先に固定すると全体が早期凍結しやすい

### 将来 deliverable 候補

- typing judgment の最小スケッチ
- representative programs に対する typing walk-through
- annotation burden / inference burden の比較表

## workstream 2. 静的解析可能性

### 主題

- lineage、fallback admissibility、capability mismatch、underdeclared case を静的にどこまで検出できるか
- parser-free fixture の expectation を、どこまで static analysis に移せるか

### entry criteria

- AST / syntax boundary が一定以上安定していること
- current helper stack の public behavior と thin delegation が整理されていること

### 今すぐやらない理由

- 解析対象の surface と IR がまだ companion notation 段階である

### 将来 deliverable 候補

- static checker の prototype
- representative fixture に対する static-only 判定の formalization
- false positive / false negative trade-off の整理

## workstream 3. 定理証明可能性

### 主題

- current L2 invariants を theorem prover に送れる形へ落とせるか
- canonical normalization、no re-promotion、rollback-cut non-interference などの性質をどう証明するか

### entry criteria

- semantics が current L2 で十分安定していること
- syntax ではなく semantic core を対象にできること

### 今すぐやらない理由

- まだ proof object を支える core formalization が薄い
- 先に parser-free PoC と docs mirror の drift を抑える方が費用対効果が高い

### 将来 deliverable 候補

- proof sketch
- theorem prover 向け core relation の encoding 案
- どの invariant が machine-checked proof に向くかの棚卸し

## workstream 4. 決定可能性

### 主題

- predicate sublanguage、fallback chain validation、contract check、effect wiring のどこが decidable か
- undecidable / semi-decidable な論点をどこで切るか

### entry criteria

- current L2 の範囲が一定程度閉じていること
- static analysis の土台が見えていること

### 今すぐやらない理由

- 現時点では semantics の companion 整理が優先であり、decision procedure 設計はまだ早い

### 将来 deliverable 候補

- decidability matrix
- complexity note
- language core と external verifier 境界の提案

## workstream 5. 実装可能性 / complexity

### 主題

- parser、checker、runtime、host interface、scheduler を実装したときの複雑さ
- current semantics が implementation complexity をどこまで押し上げるか

### entry criteria

- parser 前提の syntax / AST / runtime boundary がもう少し安定していること

### 今すぐやらない理由

- まだ production runtime を設計する段階ではない

### 将来 deliverable 候補

- subsystem ごとの complexity inventory
- prototype 実装比較
- benchmark の最小設計

## language core と external verifier の境界

これは重い workstream の横断論点である。

### current で残す working question

- どこまでを言語 core に入れるか
- どこからを external verifier / theorem prover / out-of-band analyzer に送るか
- machine-check と human-facing explanation の境界をどこまで formalize するか

### current 方針

- いまは language core を過剰に肥大化させない
- current L2 helper stack に proof / analysis obligations を押し込まない
- ただし将来の移送先として external verifier / theorem prover を明示的に計画へ残す
- first parser cut と first checker cut は、heavy workstream の前に切る narrow gate として扱う

## この workstream に最低限必要なもの

本格着手の前に、少なくとも次が必要である。

- stable enough semantic core
- representative examples と fixture による regression baseline
- parser / AST / runtime の boundary inventory
- proof / analysis / complexity を議論するための最小 IR か relation 定義

## まとめ

これらの heavy workstream は、今すぐ実装しない。
しかし current L2 の decisions が将来それらに接続できるよう、**計画には明示的に残す**。
