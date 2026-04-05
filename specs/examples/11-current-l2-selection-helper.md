# examples/11 — current L2 selection helper

この文書は、current L2 の bundle loader / batch runner を前提に、**fixture bundle を選別して回す selection helper** の最小境界を整理する補助文書である。ここで扱うのは production packaging ではなく、`crates/mir-semantics` で current L2 fixture directory をより細かく回すための verification helper である。

## この文書の位置づけ

- bundle discovery と directory 単位 batch 実行の間に、薄い selection step を追加する。
- current L2 semantics 自体は変えず、既存の sidecar discovery rule と runtime/static-only classification を再利用する。
- `must_explain` は引き続き human-facing explanation obligation に残し、selection helper でも machine-check に上げない。
- 複合指定や profile 名付き summary は、この文書の責務に含めず、必要なら上位の profile helper に送る。

## current L2 で固定すること

- selection helper は少なくとも次の 3 mode を持ってよい。
  - `runtime-only`
  - `static-only`
  - `single-fixture`
- `runtime-only` は `expected_runtime.enters_evaluation = true` の bundle だけを対象にしてよい。
- `static-only` は `expected_runtime.enters_evaluation = false` の bundle だけを対象にしてよい。
- `single-fixture` は fixture stem または fixture path 指定を受け、ちょうど 1 件に絞ってよい。
- runtime bundle の sidecar requirement は既存の bundle loader と同じであり、selection helper 自体が別 discovery rule を持ってはならない。

## selection helper の最小責務

current L2 の selection helper は、少なくとも次の 4 つだけを責務に持てばよい。

1. `runtime-only` selection
   - runtime bundle だけを選ぶ。
2. `static-only` selection
   - static-only bundle だけを選ぶ。
3. `single-fixture` selection
   - stem または path 指定で 1 件だけ選ぶ。
4. selected bundle の batch 実行
   - 選別後の bundle 群に対して既存 batch helper を走らせる。

複合指定は current L2 selection helper の責務ではない。`runtime-only + single-fixture` のような profile request は、selection helper の primitive mode を組み合わせる上位 helper で扱ってよい。

## public behavior と thin delegation の境界

### selection helper が public behavior として持つもの

- primitive な `runtime-only` / `static-only` / `single-fixture`
- discovery 結果に対する selected bundle / selected discovery failure の切り分け
- selected summary の返却

### selection helper が thin delegation に留めるもの

- sidecar discovery rule と runtime/static-only classification 自体
- bundle ごとの static / runtime / trace-audit expectation compare 自体
- 複合 request の組み立て
- profile 名や named alias の包装

## sidecar discovery と selection の整合

- runtime bundle は、既存どおり adjacent `.host-plan.json` sidecar を必須とする。
- static-only bundle は、既存どおり sidecar 無しで成立してよい。
- selection helper は sidecar discovery を再実装せず、bundle discovery の結果を filter するだけに留める。
- `single-fixture` で runtime fixture を選んだとき sidecar が欠けていれば、それは selected discovery failure として summary に残してよい。
- runtime/static-only selection では、pre-classification な discovery failure を hidden に落とさないため、runtime/static のどちらにも未分類 failure を可視のまま残してよい。
- `single-fixture` で selector に一致する bundle も discovery failure も無い場合は、selection error として fail してよい。

## summary report の最小読み

selection helper 付き batch 実行では、少なくとも次が読めればよい。

- total selected bundles
- runtime selected bundles
- static-only selected bundles
- passed
- failed
- discovery failures
- host-plan coverage failures

ここで `failed` は、selected discovery failure と selected runtime / expectation failure を合算してよい。

## machine-check と human-facing explanation の境界

### selection helper が exact compare してよいもの

selection helper は、selected batch 実行を下位 helper に委譲した結果として、少なくとも次を exact compare 済みで扱ってよい。

- static verdict
- final runtime outcome
- `event_kinds`
- formal な `non_admissible_metadata`
- short `narrative_explanations`
- host plan coverage

### selection helper が exact compare しないもの

- `must_explain`
- long-form audit explanation
- static verdict reason の prose
- detached trace / audit serialization

これにより、current L2 の event / metadata / human-facing explanation の三層分離を selection 層でも維持する。

## representative fixture での読み

- `runtime-only` では E1 / E2 / E21 / E22 / E3 variant / E6 を選び、runtime 実行と trace / audit expectation compare をまとめて走らせてよい。
- `static-only` では E4 / E5 だけを選び、static gate だけを確認してよい。
- `single-fixture` では、fixture stem か path で 1 件に絞り、その bundle だけを run してよい。

## current L2 でまだ決めないこと

- bundle manifest を導入するかどうか
- selector grammar を長期固定するかどうか
- path canonicalization policy
- directory discovery rule を長期固定するかどうか
- detached trace / audit serialization
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate`

これらは **未決定** とする。current L2 で固定するのは、batch runner の上に薄く載る selection helper だけである。
