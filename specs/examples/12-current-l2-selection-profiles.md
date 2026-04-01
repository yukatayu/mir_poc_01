# examples/12 — current L2 selection profiles

この文書は、current L2 の bundle loader / batch runner / selection helper を前提に、**selection mode を組み合わせる小さな profile layer** の最小境界を整理する補助文書である。ここで扱うのは production packaging ではなく、`crates/mir-semantics` で current L2 fixture directory をより細かく、かつ profile 名付きで回すための verification helper である。

## この文書の位置づけ

- batch runner と selection helper の上に、さらに薄い profile/request carrier を追加する。
- current L2 semantics 自体は変えず、既存の sidecar discovery rule、runtime/static-only classification、selection helper の primitive mode を再利用する。
- `must_explain` は引き続き human-facing explanation obligation に残し、profile helper でも machine-check に上げない。
- 人間向け alias や preset table が必要なら、この文書のさらに上の named profile catalog に送る。

## current L2 で固定すること

- profile layer は少なくとも次を 1 つの request として扱ってよい。
  - `runtime-only`
  - `static-only`
  - `single-fixture`
  - 上記の複合
- current L2 では、複合指定は少なくとも次を表せればよい。
  - `runtime-only + single-fixture`
  - `static-only + single-fixture`
  - `runtime-only + path-selector`
- profile layer は sidecar discovery や runtime/static-only classification を再実装せず、既存 bundle discovery と selection helper の結果を合成するだけに留める。
- `single-fixture` の unknown selector は、profile helper でも hidden skip にせず selection error のまま維持してよい。
- runtime/static selection で pre-classification discovery failure を hidden に落とさない current L2 方針は、profile helper でも維持する。

## profile layer の最小責務

current L2 の profile helper は、少なくとも次の 4 つだけを責務に持てばよい。

1. profile 名付き request の受理
   - `profile_name` と、複合指定を表す request carrier を受け取る。
2. primitive selection の合成
   - 既存の `runtime-only` / `static-only` / `single-fixture` を順に適用する。
3. selected batch 実行
   - 選別後の discovery 結果に対して既存 batch helper を走らせる。
4. profile 名付き summary の返却
   - selected bundle 数と run result を profile 名付きで返す。

human-friendly alias は current L2 profile helper の責務ではない。`smoke-runtime` や `runtime-e3` のような preset 名は、profile helper を解決対象にするさらに薄い catalog layer で扱ってよい。

## selection helper との役割分担

### selection helper 側に残すもの

- primitive な `runtime-only`
- primitive な `static-only`
- primitive な `single-fixture`
- discovery 結果に対する単発の filter

### profile layer 側に載せるもの

- 複合指定の carrier
- primitive selection の逐次合成
- `profile_name`
- selected summary の profile 名付き包装

これにより、selection helper を膨らませずに、PoC 実験ループだけを細かくできる。

## 最小 request shape

current L2 で最低限必要なのは次である。

- optional `scope`
  - `runtime-only`
  - `static-only`
- optional `single_fixture`
  - fixture stem
  - fixture path

`scope` も `single_fixture` も absent の request を将来許すかどうかは未決定であるが、current L2 では許しても semantics は増えない。

## profile summary の最小 shape

current L2 で最低限必要なのは次である。

- `profile_name`
- `total_selected_bundles`
- `runtime_selected_bundles`
- `static_selected_bundles`
- `passed`
- `failed`
- `discovery_failures`
- `host_plan_coverage_failures`
- optional な per-bundle report

ここで selected counts は、選別後の bundle / selected discovery failure を基準にしてよい。

## machine-check と human-facing explanation の境界

### profile helper が exact compare してよいもの

- static verdict
- final runtime outcome
- `event_kinds`
- formal な `non_admissible_metadata`
- short `narrative_explanations`
- host plan coverage

### profile helper が exact compare しないもの

- `must_explain`
- long-form audit explanation
- static verdict reason の prose
- detached trace / audit serialization

これにより、current L2 の event / metadata / human-facing explanation の三層分離を profile 層でも維持する。

## representative fixture での読み

- `runtime-only + single-fixture(stem = e2-try-fallback)` は、runtime bundle 1 件だけを選び、その bundle だけを batch 実行してよい。
- `static-only + single-fixture(stem = e4-malformed-lineage)` は、static-only bundle 1 件だけを選び、その static gate だけを確認してよい。
- `runtime-only + single-fixture(path = e6-write-after-expiry.json)` は、path selector を通じて runtime bundle 1 件だけを選んでよい。

## current L2 でまだ決めないこと

- bundle manifest を導入するかどうか
- selector grammar を長期固定するかどうか
- path canonicalization policy
- directory discovery rule を長期固定するかどうか
- detached trace / audit serialization
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate`

これらは **未決定** とする。current L2 で固定するのは、selection helper の上に薄く載る profile helper だけである。
