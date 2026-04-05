# 52 — current L2 try/rollback fourth checker spike comparison

## 目的

この文書は、current L2 parser-free PoC と existing checker helper family を前提に、
**`TryFallback` / `AtomicCut` の structural floor を fourth checker spike として今 helper-local に actualize してよいか**
を narrow に整理する。

ここで固定するのは final checker API ではない。
固定するのは、

- current helper family がどの carrier に依存しているか
- `try` / rollback locality がその carrier に自然に乗るか
- 今は docs/runtime representative に留めるべきか

という next-step judgment だけである。

## current 前提

current L2 では次が成立している。

- same-lineage / missing-option / capability の 3 spike は、fixture-side `expected_static.checked_reason_codes` と detached static gate artifact `detached_noncore.reason_codes` の row family を helper-local exact compare する cut で actualize 済みである
- shared support helper は row family compare contract だけを共有し、generic checker-side shared entry はまだ切っていない
- `TryFallback` / `AtomicCut` については、`specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md` で structural floor と runtime/proof boundary が docs-only で整理済みである
- current representative evidence は `E2` / `E21` / `E22` という **runtime valid** fixture であり、try/rollback floor を malformed / underdeclared static reason family として actualize した fixture 群はまだ無い
- current `StaticReasonCodeRow` inventory は same-lineage / missing-option / capability の 8 kind に限られており、`try` / rollback locality family を表す kind は未導入である

したがって current 問いは、
**`try` / rollback locality を fourth checker spike に進めるなら、existing row-family helper を再利用してよいか、それともまだ docs/runtime representative に留めるべきか**
である。

## 比較観点

1. existing checker helper family が依存する carrier と整合するか
2. current code anchor と `specs/examples/51` の runtime/proof split を壊さないか
3. hidden acceptance や unsupported typed carrier を増やさないか
4. parser boundary / first checker cut / theorem prover boundary の順序を保てるか

## 比較対象

### 案 1. existing reason-code family helper に fourth spike を追加する

- `scripts/current_l2_try_rollback_checker.py` を増やし、
  `checked_reason_codes` / detached `reason_codes` から `try` / rollback family を filter して compare する

#### 利点

- same-lineage / missing-option / capability と surface pattern はそろう
- detached loop wrapper に `smoke-try-rollback-checker` を追加しやすい

#### 欠点

- current `StaticReasonCodeRow` inventory に `try` / rollback locality family が存在しない
- `TryFallback` / `AtomicCut` floor は current docs では region shape / statement shape / event-vs-gate distinction として整理されており、row-family reason code としてはまだ source-backed に切れていない
- row family を先に導入すると、dynamic gate や restore scope まで typed row へ押し込みやすい
- existing three spikes は static-only malformed / underdeclared cluster を対象にしているが、`E2` / `E21` / `E22` は runtime valid fixture であり、cluster shape も違う

### 案 2. dedicated AST structural helper として fourth spike を切る

- `checked_reason_codes` を使わず、fixture AST と static gate verdict から
  `TryFallback` region shape / `AtomicCut` statement shape だけを読む helper-local checker を別に置く

#### 利点

- row-family helper を無理に広げなくて済む
- structural floor だけを dedicated helper で読むなら、`specs/examples/51` の split に沿いやすい

#### 欠点

- current helper family の compare contract とは別軸になり、`shared_family_checker_support` を再利用しにくい
- malformed / underdeclared fixture corpus がまだ無いため、runtime valid fixture だけで checker spike を名乗ると static regression family と誤読されやすい
- first checker cut の public artifact shape がまだ無い段階で AST structural helper を増やすと、parser boundary / checker API cut を先食いしやすい

### 案 3. current phase では docs/runtime representative に留め、fourth spike actualization はまだ切らない

- `specs/examples/51` と `E2` / `E21` / `E22` を current evidence として維持し、
  helper-local spike actualization は parser boundary と first checker API cut がもう一段見えるまで保留する

#### 利点

- current code anchor と docs judgment の split を最も素直に保てる
- unsupported typed carrier を増やさずに済む
- existing three spikes の reason-row family と、`try` / rollback locality の structural floor を無理に同一視しない
- fourth spike が必要になった時も、row-family helper か dedicated AST helper かを改めて選べる

#### 欠点

- executable checker spike はまだ増えない
- `try` / rollback locality は current phase では runtime representative と prose comparison に依存する

## current judgment

current L2 の next narrow step として最も自然なのは、
**案 3. current phase では docs/runtime representative に留め、fourth spike actualization はまだ切らない**
である。

理由は次の通り。

1. existing three spikes は `checked_reason_codes` / detached `reason_codes` row family compare に依存しているが、`try` / rollback locality はまだその family として切れていない
2. `specs/examples/51` で残すと決めた dynamic gate / restore scope を、premature な typed carrier や helper contract に押し込みたくない
3. current representative evidence は runtime valid fixture `E2` / `E21` / `E22` であり、static malformed / underdeclared cluster と同じ helper pattern へ載せるにはまだ無理がある
4. parser boundary と first checker API cut の inventory がもう一段見えた時点で、dedicated AST helper か row-family extension かを改めて選ぶ方が手戻りが少ない

## future actualization を切るなら何が最小か

current docs-only judgment の次段として actualization を切るなら、先に比較すべきは次である。

1. **dedicated AST structural helper**
   - `TryFallback` region shape
   - `fallback_body` presence
   - `AtomicCut` statement shape
   - event-vs-gate distinction を checker proof obligation に入れない current cut
2. **static malformed fixture family が本当に必要か**
   - current runtime representative だけで十分か
   - malformed / underdeclared static family を新設する価値があるか
3. **first checker API との handoff**
   - helper-local compare で止めるのか
   - parser / checker entry の inventory と一緒に actual API shape を決めるのか

## current cut

この task では次を行わない。

- `StaticReasonCodeRow` への new kind 追加
- `scripts/current_l2_try_rollback_checker.py` の actual helper 追加
- `smoke-try-rollback-checker` wrapper 追加
- malformed / underdeclared な `try` / rollback fixture family の新設
- runtime/proof boundary を狭める theorem-level claim

## OPEN に残すもの

- `TryFallback` / `AtomicCut` structural floor を future checker API に actualize するなら row-family helper と dedicated AST helper のどちらが自然か
- static malformed fixture family を追加する必要があるか
- parser boundary inventory と first checker cut inventory をどの文書で handoff するか
- rollback locality を theorem prover / model checker 側へどう送るか
