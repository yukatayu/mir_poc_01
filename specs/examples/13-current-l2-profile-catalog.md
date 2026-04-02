# examples/13 — current L2 profile catalog

この文書は、current L2 の selection profile helper を前提に、**small named profile catalog / preset table** の最小境界を整理する補助文書である。ここで扱うのは production packaging ではなく、`crates/mir-semantics` で current L2 fixture directory を少ない引数で回しやすくするための人間向け alias layer である。

## この文書の位置づけ

- batch runner、selection helper、profile helper の上に、さらに薄い named alias layer を追加する。
- current L2 semantics 自体は変えず、既存の `SelectionRequest` / `SelectionProfile` へ解決するだけに留める。
- `must_explain` は引き続き human-facing explanation obligation に残し、catalog layer でも machine-check に上げない。

## current L2 で固定すること

- small named profile catalog は、human-friendly alias を既存 `SelectionProfile` へ解決する preset table として扱ってよい。
- current L2 で最低限持つ alias は次である。
  - `smoke-runtime`
  - `smoke-static`
  - `runtime-e3`
  - `static-e4`
- hard-coded catalog を維持する実装では、alias list と alias→request 解決を 1 箇所の preset table から導いてよい。
- catalog layer は sidecar discovery、runtime/static-only classification、primitive selection 自体を再実装しない。
- alias 解決後は既存 `SelectionRequest` / `SelectionProfile` / `run_directory_profiled` に流し込むだけに留める。
- unknown alias は hidden skip にせず error にしてよい。

## profile catalog の最小責務

current L2 の profile catalog は、少なくとも次の 3 つだけを責務に持てばよい。

1. human-friendly alias の提示
   - 利用可能な preset 名を列挙する。
2. alias から既存 request への解決
   - alias を `SelectionRequest` / `SelectionProfile` に解決する。
3. resolved request 付き summary の返却
   - selected batch 実行の結果に、どの request へ解決されたかを添えて返す。

## selection helper / profile helper との役割分担

### selection helper 側に残すもの

- primitive な `runtime-only`
- primitive な `static-only`
- primitive な `single-fixture`
- discovery 結果に対する単発 filter

### profile helper 側に残すもの

- 複合指定の carrier
- primitive selection の逐次合成
- `profile_name`
- selected summary

### named profile catalog 側に載せるもの

- human-friendly alias
- alias と request の対応表
- `resolved_request` を含む summary wrapper

これにより、selection helper と profile helper を肥大化させずに、PoC 実験ループの操作だけを短くできる。

## current L2 の最小 preset table

current L2 では、少なくとも次の対応があればよい。

- `smoke-runtime`
  - `SelectionRequest { scope = runtime-only, single_fixture = absent }`
- `smoke-static`
  - `SelectionRequest { scope = static-only, single_fixture = absent }`
- `runtime-e3`
  - `SelectionRequest { scope = runtime-only, single_fixture = stem(e3-option-admit-chain) }`
- `static-e4`
  - `SelectionRequest { scope = static-only, single_fixture = stem(e4-malformed-lineage) }`

alias 名の長期固定や、より豊かな alias grammar は current L2 では決めない。

## resolved summary の最小 shape

current L2 で最低限必要なのは次である。

- `profile_name`
- `resolved_request`
- `total_selected_bundles`
- `runtime_selected_bundles`
- `static_selected_bundles`
- `passed`
- `failed`
- `discovery_failures`
- `host_plan_coverage_failures`
- optional な per-bundle report

ここで `resolved_request` は、machine-readable な current L2 request carrier として読めれば足りる。field 名や serialization の長期固定は行わない。

## machine-check と human-facing explanation の境界

### catalog layer が exact compare してよいもの

- alias 解決後の `resolved_request`
- static verdict
- final runtime outcome
- `event_kinds`
- formal な `non_admissible_metadata`
- short `narrative_explanations`
- host plan coverage

### catalog layer が exact compare しないもの

- `must_explain`
- long-form audit explanation
- static verdict reason の prose
- detached trace / audit serialization

これにより、current L2 の event / metadata / human-facing explanation の三層分離を alias 層でも維持する。

## representative fixture での読み

- `smoke-runtime` は既存の `runtime-only` と同じ bundle 群、すなわち E1 / E2 / E3 variant / E6 を回してよい。
- `smoke-static` は既存の `static-only` と同じ bundle 群、すなわち E4 / E5 を回してよい。
- `runtime-e3` は E3 runtime bundle だけを回してよい。
- `static-e4` は E4 static-only bundle だけを回してよい。

## current L2 でまだ決めないこと

- bundle manifest を導入するかどうか
- selector grammar を長期固定するかどうか
- path canonicalization policy
- detached trace / audit serialization
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate`
- alias catalog を sidecar や manifest へ外出しするかどうか

catalog externalization の比較整理は `specs/examples/14-current-l2-profile-catalog-externalization.md` を参照する。

これらは **未決定** とする。current L2 で固定するのは、selection profile helper の上に薄く載る small named profile catalog だけである。
