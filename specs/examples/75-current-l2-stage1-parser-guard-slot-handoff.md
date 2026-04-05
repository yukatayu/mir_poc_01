# 75 — current L2 stage 1 parser guard slot handoff

## 目的

この文書は、`specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md` で
stage 1 の declaration-side guard slot を opaque attached slot に留める current judgment を前提に、
**その opaque slot を actual parser / checker handoff へ送るなら、
parser-side carrier と current parser-free AST fixture schema をどのように最小接続するのが自然か**
を narrow に比較する。

ここで固定するのは final parser API ではない。
固定するのは、

- stage 1 の opaque slot を current parser-free AST fixture schema とどう接続するか
- current fixture corpus を unnecessary churn させない最小 handoff は何か
- stage 1 の docs-only cut を壊さない actualization boundary は何か

という current next narrow step だけである。

## 前提

- current L2 の core semantics は変更しない。
- `specs/examples/02-current-l2-ast-fixture-schema.md` の parser-free fixture schema は維持する。
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md` の判断
  - stage 1 では declaration-side guard slot を opaque attached slot に留める
  - predicate fragment parse / well-formedness は later stage に残す
  を維持する。
- final parser grammar、final parser API、final AST schema の長期固定は引き続き OPEN である。

## current source anchor

### parser-free AST fixture carrier

current `OptionDecl` は少なくとも次を持つ。

- `name`
- `target`
- `capability`
- `lease`
- `admit`

ここで `lease` は current fixture corpus では、たとえば

- `live`
- `expired`

のような current L2 companion carrier を持つ。

### representative fixture

- `e4-malformed-lineage`
  - `lease = "live"`
  - `admit = null`
- `e7-write-fallback-after-expiry`
  - `lease = "expired"` と `lease = "live"` が mixed に並ぶ
- `e3-option-admit-chain`
  - `admit` は predicate node を持つ

この組合せにより、current source-backed な事実は次である。

1. stage 1 の declaration structural floor を見るだけなら `lease` slot は present だが、`admit` はまだ不要な fixture が多い
2. `admit` predicate node を stage 1 handoff に混ぜると、request / admissibility cluster を先食いしやすい
3. parser-free fixture schema を stage 1 parser actualization の都合で広く変更する圧はまだ弱い

## 比較する 3 案

### 案 1. parser output を current fixture schema へ直接合わせる

stage 1 parser spike の output を、ほぼそのまま current parser-free AST fixture schema の `OptionDecl` へ合わせる。

つまり declaration-side guard slot は parser 側でも `lease` field として持ち、
current stage 1 では string-like carrier を直接出す。

#### 利点

- 実装上は最も短い。
- stage 1 smoke を current fixture corpus へすぐ流しやすい。
- extra lowering layer を増やさない。

#### 欠点

- stage 1 の opaque slot と current fixture の companion carrier が同一物に見えやすい。
- parser-side carrier と parser-free fixture schema が早く結びつきすぎる。
- later stage で predicate fragment parse を入れるとき、stage 1 の direct string carrier が drift source になりやすい。

### 案 2. parser-side opaque slot carrier を持ち、fixture schema へ narrow lowering する

stage 1 parser spike は parser-side に **opaque slot carrier** を持つ。
ただし current actual smoke / checker handoff では、
その parser-side carrier を thin adapter で current parser-free AST fixture schema の `OptionDecl.lease` へ narrow lowering する。

この案で stage 1 が約束するのは、

- parser-side では opaque slot として declaration attachment を保持する
- fixture-side / checker-side へ渡す current bridge では、current corpus に必要な minimal carrier だけへ落とす

という 2 段切り分けである。

#### 利点

- stage 1 の「opaque attached slot」という docs-only judgmentを保ちやすい。
- current parser-free AST fixture schema をすぐには変更しなくてよい。
- later stage で predicate fragment parse を導入しても、parser-side carrier と fixture-side carrier を切り分けやすい。

#### 欠点

- thin adapter を 1 枚置く必要がある。
- parser-side carrier 名と lowering 契約を narrow に決める必要がある。

### 案 3. current parser-free AST fixture schema 自体を opaque slot carrier へ widen する

stage 1 parser spike に合わせて current fixture schema の `OptionDecl.lease` を
opaque object / union carrier に変更する。

#### 利点

- parser-side carrier と fixture-side carrier が同型になりやすい。
- long term では schema 統一の見通しがある。

#### 欠点

- current parser-free fixture corpus と detached validation loop に unnecessary churn を入れる。
- stage 1 の narrow task に対して変更範囲が大きい。
- current L2 では final AST schema も未固定なので、widening を先に行う pressure が弱い。

## 比較

### stage 1 の docs-only cut を保てるか

- 案 1:
  直接 `lease` string carrier へ寄るため、opaque slot と final-like AST field の区別が薄れやすい。
- 案 2:
  parser-side opaque slot と fixture-side bridge を分けるので最も保ちやすい。
- 案 3:
  一見保てるが、fixture schema 側まで widening するぶん stage 1 の narrowness が弱まる。

### current parser-free fixture corpus を unnecessary churn させないか

- 案 1:
  変更加工は少ない。
- 案 2:
  fixture corpus は据え置きのまま bridge だけで済む。
- 案 3:
  fixture corpus と helper stack を広く触りやすい。

### later stage で predicate fragment parse を入れやすいか

- 案 1:
  stage 1 carrier が direct string だと later widening の説明が必要になる。
- 案 2:
  parser-side opaque slot を later predicate fragment carrier へ発展させやすい。
- 案 3:
  schema widening 自体はしやすいが、今やる必要は薄い。

### hidden elaboration を避けられるか

- 案 1:
  parser output が fixture schema へ直結しすぎるため、opaque slot の意味が見えにくい。
- 案 2:
  parser-side carrierと lowering bridge を分けるので、どこで何を潰したかが最も明示しやすい。
- 案 3:
  schema 側 widening により later semantics を先取りしやすい。

## current judgment

current repo の next narrow step としては、
**案 2. parser-side opaque slot carrier を持ち、fixture schema へ narrow lowering する**
のが最も自然である。

つまり、

1. stage 1 parser spike の parser-side carrier は current parser-free AST fixture schema と同一視しない
2. ただし current actual smoke / checker handoff では thin adapter を介して `OptionDecl.lease` へ narrow lowering してよい
3. `OptionDecl.lease` の current carrier は current fixture corpus compatibility のために維持し、opaque slot の long-term schema 決定は later stage に残す

という cut を current docs-only judgment とする。

## 何を current stage でまだ決めないか

- parser-side opaque slot の exact carrier 名
- parser-side opaque slot が raw text / token slice / opaque leaf のどれを持つか
- thin lowering adapter の actual API surface
- later stage で `OptionDecl.lease` を widening するかどうか
- stage 3 で predicate fragment parse を parser-side opaque slot からどう受けるか

## current meaning

- stage 1 の declaration-side guard slot は opaque attached slot として読む
- その actual handoff では parser-side carrier と fixture-side carrier をすぐ同一化しない
- current parser-free AST fixture schema は current smoke / checker bridge の compatibility anchor として維持する
- final parser API / final AST schema を early freeze しないことが current repo の phase に合う
