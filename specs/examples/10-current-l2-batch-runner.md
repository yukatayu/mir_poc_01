# examples/10 — current L2 batch runner

この文書は、current L2 の fixture bundle loader / bundle-level helper を前提に、**fixture directory を束ねて一括実行する batch runner** の最小境界を整理する補助文書である。
ここで扱うのは production runner ではなく、`crates/mir-semantics` で current L2 fixture directory を実験ループ単位で回しやすくするための companion helper である。

## この文書の位置づけ

- fixture JSON と `.host-plan.json` sidecar を 1 組として扱う bundle helper の上に、directory discovery と summary report を追加する。
- current L2 semantics 自体は変えず、PoC の実験ループを「fixture / sidecar を置いて directory ごと実行する」形へ寄せる。
- `must_explain` は引き続き human-facing explanation obligation に残し、batch 層でも machine-check に上げない。

## current L2 で固定すること

- batch runner は fixture directory から bundle candidate を自動 discovery してよい。
- discovery 対象は、少なくとも top-level の `*.json` であり、`.host-plan.json` は fixture 本体として数えない。
- runtime bundle と static-only bundle は、fixture 本体の `expected_runtime.enters_evaluation` から判定してよい。
- runtime bundle は adjacent `.host-plan.json` sidecar を必須とし、欠落時は discovery failure にしてよい。
- static-only bundle は sidecar 無しでも成立してよい。
- batch runner は directory 単位で次を一括実行してよい。
  - bundle discovery
  - static gate
  - runtime execution
  - trace / audit expectation compare
  - host plan coverage check

## batch runner の最小責務

current L2 の batch runner は、少なくとも次の 4 つだけを責務に持てばよい。

1. `discover_bundles`
   - fixture directory から bundle candidate を見つける。
2. runtime / static-only classification
   - `expected_runtime.enters_evaluation` に従って、sidecar 必須かどうかを決める。
3. bundle execution
   - 既存の `run_bundle` を bundle ごとに呼ぶ。
4. summary report
   - directory 単位で passed / failed / discovery failure / host-plan coverage failure を集約する。

## bundle discovery の最小方針

- discovery の anchor は directory path である。
- current L2 では、manifest file を要求しない。
- candidate fixture path は、directory 直下の `*.json` から `.host-plan.json` を除外して得てよい。
- directory traversal の深さ、glob pattern の長期固定、manifest 導入の要否は current L2 では固定しない。

## runtime bundle と static-only bundle の分岐

### runtime bundle

- `expected_runtime.enters_evaluation = true`
- adjacent `.host-plan.json` sidecar が必須
- sidecar 欠落や sidecar load failure は discovery failure にしてよい

### static-only bundle

- `expected_runtime.enters_evaluation = false`
- sidecar 無しでも bundle として成立してよい
- batch runner は static gate だけを確認し、runtime へ入れない

## summary report の最小 shape

current L2 で最低限必要なのは次である。

- `total_bundles`
- `runtime_bundles`
- `static_only_bundles`
- `passed`
- `failed`
- `discovery_failures`
- `host_plan_coverage_failures`
- optional な per-bundle report

ここで `failed` は、discovery failure と runtime / expectation failure の両方を含めてよい。

## machine-check と human-facing explanation の境界

### batch runner が exact compare してよいもの

- static verdict
- `enters_evaluation`
- final runtime outcome
- `event_kinds`
- formal な `non_admissible_metadata`
- short `narrative_explanations`
- host plan coverage

### batch runner が exact compare しないもの

- `must_explain`
- long-form audit explanation
- static verdict reason の prose
- detached trace / audit serialization

これにより、current L2 の event / metadata / narrative の三層分離を batch 層でも維持する。

## representative fixtures での読み

- E1 / E2 / E3 variant / E6 は runtime bundle として discovery され、sidecar 付きで batch 実行される。
- E4 / E5 は static-only bundle として discovery され、static gate で止まる。
- runtime bundle の uncovered oracle call は host plan coverage failure として summary に残してよい。
- sidecar 欠落や sidecar parse failure は discovery failure として summary に残してよい。

## current L2 でまだ決めないこと

- bundle manifest を導入するかどうか
- directory discovery rule を長期固定するかどうか
- detached trace / audit serialization
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate`
- parser 実装後に batch runner をどの asset format へ接続するか

これらは **未決定** とする。current L2 で固定するのは、bundle 群をまとめて回す最小 verification helper だけである。
