# examples/09 — current L2 bundle loader

この文書は、current L2 の AST fixture schema、host plan sidecar schema、parser-free minimal interpreter / host harness を前提に、**fixture bundle loader / bundle-level helper** の最小 shape を整理する補助文書である。
ここで扱うのは production packaging ではなく、`crates/mir-semantics` で current L2 fixture と `.host-plan.json` sidecar を 1 組として扱い、static gate / runtime / trace-audit expectation をまとめて検証するための companion 境界である。

## この文書の位置づけ

- AST fixture JSON と `.host-plan.json` sidecar を bundle 単位で扱い、PoC の実験ループを「fixture + sidecar の追加」で回しやすくする。
- parser-free minimal interpreter の semantics 自体は変えず、asset 解決と expectation check の入口だけを小さく揃える。
- host plan schema 自体は `specs/examples/08-current-l2-host-plan-schema.md` を参照する。

## current L2 で固定すること

- bundle の最小単位は、fixture path から解決される次の組である。
  - AST fixture 本体
  - `expected_static`
  - `expected_runtime`
  - `expected_trace_audit`
  - optional な host plan sidecar
- runtime fixture では、host plan sidecar を bundle の一部として要求してよい。
- static-only fixture では、host plan sidecar が無くても bundle として成立してよい。
- bundle-level helper は次を一括で走らせてよい。
  - static gate
  - runtime execution
  - trace / audit expectation compare
  - host plan coverage check
- `must_explain` は bundle helper の machine-check に上げず、human-facing explanation obligation に残す。

## bundle の最小 shape

current L2 で最低限必要なのは次である。

1. `fixture_path`
   - bundle の anchor になる AST fixture の path
2. `fixture`
   - machine-readable AST fixture 本体
3. `host_plan_path`
   - host plan sidecar がある場合だけ持つ
4. `host_plan`
   - host plan sidecar を load した結果。static-only fixture では absent を許す
5. `runtime_requirement`
   - `StaticOnly`
   - `RuntimeWithHostPlan`

ここで `runtime_requirement` は final wire field ではなく、bundle loader が runtime に入る fixture と static gate で止まる fixture を区別するための companion carrier である。

## public behavior と thin delegation の境界

### bundle loader が public behavior として持つもの

- fixture 本体と adjacent sidecar を 1 bundle として解決すること
- `expected_runtime.enters_evaluation` から runtime/static-only requirement を判定すること
- 1 bundle 単位で static / runtime / trace-audit expectation を照合すること

### bundle loader が thin delegation に留めるもの

- directory 単位の discovery
- 複数 bundle の集約 summary
- `runtime-only` / `static-only` / `single-fixture` の選別
- profile 名や named alias の包装

## sidecar discovery の最小方針

- discovery の入口は fixture JSON path である。
- host plan sidecar の path は、fixture path の `.json` 拡張子を `.host-plan.json` に置き換えた隣接 path として解決してよい。
- `expected_runtime.enters_evaluation = true` の fixture は `RuntimeWithHostPlan` と読み、この sidecar が無ければ bundle load failure にしてよい。
- `expected_runtime.enters_evaluation = false` の fixture は `StaticOnly` と読み、sidecar が無くてもよい。
- sidecar manifest、directory-level index、bundle manifest file は current L2 では要求しない。

## bundle-level helper が machine-check するもの

bundle helper は、自身の public behavior として少なくとも次を exact compare してよい。

- static verdict
- `enters_evaluation`
- final runtime outcome
- `event_kinds`
- formal な `non_admissible_metadata`
- short `narrative_explanations`

ここで `non_admissible_metadata` と `narrative_explanations` は、host plan の override があればそれを優先し、無ければ fixture 本体の `expected_trace_audit` を使えばよい。

## machine-check に上げないもの

bundle helper は次を exact compare しない。

- `must_explain`
- static verdict reason の prose
- long-form audit explanation
- detached trace / audit serialization

これは current L2 が維持している event / metadata / human-facing explanation の三層分離を bundle 層でも崩さないためである。

## representative fixtures での読み

### runtime fixture

- E1 / E2 / E3 variant / E6 は、fixture 本体と `.host-plan.json` sidecar を合わせて 1 つの runtime bundle として扱う。
- bundle helper は host plan coverage を含めて `run_to_completion` まで走らせ、その結果を `expected_runtime` と `expected_trace_audit` に照合してよい。

### static-only fixture

- E4 / E5 は、fixture 本体だけで bundle として成立してよい。
- bundle helper は static gate のみを通し、runtime evaluation に入らないことを machine-check してよい。

## current L2 でまだ決めないこと

- bundle manifest の導入要否
- fixture directory 全体を batch 実行する API shape
- field 名の長期固定
- detached trace / audit serialization
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate`

これらは **未決定** とする。current L2 で固定するのは、fixture と sidecar を 1 組として扱う最小 helper だけである。
