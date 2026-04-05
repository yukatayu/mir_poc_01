# 33 — current L2 checked static reasons carrier

## 目的

この文書は、current L2 parser-free PoC で `expected_static.reasons` の dual-use を分離するための
**future checker API の最小 dedicated carrier**
を整理する。

ここで固定するのは final type system や final reason taxonomy ではない。
固定するのは、

- `expected_static.reasons` を human-facing explanation に残し
- actual static gate compare を additive optional carrier へ寄せ
- detached static gate artifact loop と矛盾しない narrow migration cut

だけである。

## current 問題

current fixture corpus では `expected_static.reasons` が次の二重用途になっている。

1. valid fixture での補助説明
2. malformed / underdeclared fixture での user-facing static explanation

一方、actual static gate compare の source は `static_gate_detailed().reasons` であり、
detached static gate artifact の `checker_core.reasons` に保存される。

したがって `expected_static.reasons` をそのまま `run_bundle()` の machine-check へ上げると、
valid fixture 群と underdeclared wording の両方で current corpus と衝突する。

## 比較対象

### 案 1. `checked_reasons`

`expected_static` に additive optional `checked_reasons: Vec<String>` を足す。

- `reasons` は explanation のまま残す
- `checked_reasons` が present のときだけ harness が fail-closed compare する
- absent のときは current behavior のまま `verdict` だけを見る

#### 利点

- current fixture corpus を壊さない
- detached static gate artifact の actual `reasons` と直接対応づけやすい
- typed reason code をまだ固定しなくてよい
- additive migration なので new fixture から段階的に採用できる

#### 欠点

- string wording 依存は残る
- 将来 typed code へ進むなら二段 migration になる

### 案 2. `reason_codes`

`expected_static` に typed `reason_codes` を足し、machine-check は code list だけを見る。

#### 利点

- wording drift に強い
- theorem prover / checker / detached compare の contract を将来揃えやすい

#### 欠点

- current L2 では reason taxonomy を先取りしすぎる
- malformed / underdeclared 以外の future static reason を premature に設計しやすい
- detached static gate artifact の current string list と直結しない

### 案 3. detached-only 維持

fixture schema には何も足さず、actual static reason compare は detached static gate artifact 側に留める。

#### 利点

- schema を増やさない
- current detached loop と完全に整合する

#### 欠点

- harness / bundle machine-check と first checker cut の接続が進まない
- fixture authoring で static compare を bundle run と同じ場所に寄せられない

## current judgment

current L2 の next narrow step として最も自然なのは **案 1. `checked_reasons`** である。

理由は次の通り。

1. `expected_static.reasons` の explanatory role を壊さない
2. detached static gate artifact の actual `checker_core.reasons` と素直につながる
3. typed reason code の最終 taxonomy をまだ固定しないで済む
4. absent 時は current `verdict` only machine-check を維持できる

## minimal carrier shape

current docs-only judgment では、`expected_static` は次の 3 field を持ってよい。

- `verdict`
- `reasons`
- optional `checked_reasons`

意味は次の通り。

- `verdict`
  - current harness core の fail-closed machine-check
- `reasons`
  - human-facing explanation
- `checked_reasons`
  - actual static gate compare の narrow machine-check carrier

## machine-check policy

current narrow policy は次である。

1. harness は常に `verdict` を比較する
2. `checked_reasons` が present のときだけ actual static gate `reasons` と exact compare する
3. `reasons` 自体は exact compare しない

この cut により、current harness / detached static gate artifact / fixture authoring の 3 者を
無理なく接続できる。

## detached artifact loop との関係

detached static gate artifact は引き続き actual source of truth を持つ。

- actual source
  - `checker_core.static_verdict`
  - `checker_core.reasons`
- fixture-side narrow expectation
  - `expected_static.verdict`
  - optional `expected_static.checked_reasons`

したがって `checked_reasons` は detached artifact を置き換えない。
むしろ detached artifact compare で確認した static gate wording を、bundle-level machine-check に narrow transfer するときの bridge として使う。

## OPEN に残すもの

- `checked_reasons` を long-term に維持するか、typed `reason_codes` に移行するか
- detached static gate artifact に `reason_codes` mirror を足すか
- malformed / underdeclared を超える richer static reason taxonomy
- theorem prover / first checker cut / final type system との final relation

## current judgment

- `expected_static.reasons` は explanation に残す
- actual static gate compare を bundle machine-check に narrow transfer したいなら additive optional `checked_reasons` が最小
- typed `reason_codes` は有力な後段候補だが、current L2 ではまだ早い
- detached-only 維持は保守的だが、next narrow step としては進捗量が小さい
