# plan/09 — helper stack と責務マップ

## 目的

current L2 parser-free PoC では、helper layer が増えるほど docs / tests / code の mirror drift が起きやすい。
ここでは helper ごとの public behavior、thin delegation、tests の literal expectation、docs mirror の境界を揃える。

## helper stack の中心関数

特に重要なのは次の stack である。

```text
run_directory_named_profile
  -> run_directory_profiled
    -> select_bundles_from_request
      -> batch_summary_from_discovery
        -> run_bundle
```

この前段に `discover_bundles_in_directory` があり、後段に `FixtureHostStub::run_fixture` と `run_to_completion` がある。

## layer ごとの public behavior / thin delegation

| helper | public behavior | thin delegation |
|---|---|---|
| `run_bundle` | 1 bundle の static / runtime / trace / host-plan coverage を照合する | oracle 実行そのものは `FixtureHostStub::run_fixture` へ委譲 |
| `batch_summary_from_discovery` | bundle 群を集計し passed / failed / coverage failure を返す | bundle 単位の実行・比較は `run_bundle` に委譲 |
| `select_bundles_from_request` | scope と single-fixture selector を逐次合成して selected discovery を返す | runtime/static classification 自体は bundle / batch 側に委譲 |
| `run_directory_profiled` | `profile_name` と selected batch summary を返す | discovery、selection、execution 自体は下位 helper を再利用 |
| `run_directory_named_profile` | alias 名を受けて named profile 実行を行い、unknown alias failure と `resolved_request` 付き summary を返す | alias 一覧列挙と alias→request 解決の source of truth は `ProfileCatalog` に委譲し、selection-shape coverage は `run_directory_profiled` へ委譲 |

## named profile catalog の責務

### public behavior

- `ProfileCatalog::aliases()`
- `ProfileCatalog::resolve()`
- unknown alias failure
- `run_directory_named_profile()` による alias→profiled execution の thin wrapper

### 持たない責務

- bundle discovery
- runtime/static-only classification
- selected bundle counts の一次計算
- fixture suffix の直接判定

## tests の責務分担

### internal tests (`crates/mir-semantics/src/harness.rs`)

- private preset table の single source of truth を確認する
- `aliases()` と `resolve()` が同じ internal table から導かれることを見る
- public integration behavior の oracle にはしない

### integration tests (`crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`)

- bundle / batch / selection / profile / named profile の public behavior を literal expectation で確認する
- `resolved_request` は literal expected request を使い、`ProfileCatalog::resolve()` 自体を test oracle に再利用しない
- unknown alias failure を literal expectation で維持する
- selected counts / concrete fixture suffix は profile-layer tests が主責務を持つ
- named-profile tests は alias / `resolved_request` / unknown alias / thin delegation を主責務にする

## docs / tests / code の mirror 境界

| 層 | 何を正本とするか |
|---|---|
| code | hard-coded preset table、helper 実装、call chain |
| tests | public behavior coverage、literal expectation、thin delegation coverage |
| docs | helper boundary、責務分担、なぜその boundary を採るかの説明 |

### alias mirror の current 方針

- concrete alias prose は `specs/examples/13-current-l2-profile-catalog.md` に寄せる
- code 側 single source of truth は `harness.rs` の hard-coded preset table
- tests 側は alias list / `resolved_request` / unknown alias failure を machine-check として持つ
- selected counts / fixture suffix は helper 化してよいが、catalog 実装を oracle に再利用しない

## drift が起きやすい点

| drift point | なぜ起きやすいか | current 対策 |
|---|---|---|
| alias list と resolve table | alias 追加時に二重定義しやすい | internal preset table の single source of truth |
| docs の alias prose | 複数 docs に同じ alias 一覧を書きたくなる | `specs/examples/13` に寄せる |
| named profile tests | selected counts / suffix / request を全部ここで持ちたくなる | profile-layer tests と named-profile tests を分離 |
| helper stack docs | 各 layer が下位 helper を再説明しがち | public behavior / thin delegation で揃える |
| sidecar / bundle / batch boundary | discovery / classification / selection を重複実装しやすい | lower-layer responsibility を plan に固定する |

## current named profile catalog の status

- hard-coded table を維持
- aliases:
  - `smoke-runtime`
  - `smoke-static`
  - `runtime-e3`
  - `static-e4`
- machine-readable catalog asset / manifest は comparison 止まり
- externalization は future option であり current L2 採用ではない

## `must_explain` の位置

helper stack のどの layer でも `must_explain` は machine-check に上げない。
これは次を守るためである。

- formal event / metadata / narrative explanation の三層分離
- report / prose 側の human-facing obligation を helper に押し込まないこと

## detached exporter entry の責務境界

detached artifact exporter を narrow に始める comparison では、次の切り分けを current understanding とする。

- payload core
  - `RunReport` に最も近い。
- first exporter entry
  - `run_bundle` / `BundleRunReport`
- later aggregate export
  - `BatchRunSummary`

この切り分けを採る理由は、`run_bundle` が current helper stack で 1 bundle の static / runtime / trace / coverage をまとめる public behavior boundaryだからである。
interpreter / `TraceAuditSink` に直結する exporter から始めると、payload core には近くても bundle context を失いやすい。
逆に batch から始めると、selection / profile / coverage aggregation まで一度に exporter へ持ち込みやすい。

## bundle-first detached artifact の split

bundle-first exporter を採る場合、`run_bundle` 周辺で見えている field は次のように分けるのが current helper boundary と最も整合する。

- payload core
  - `RunReport` 由来の field
- `bundle_context`
  - `fixture_id`
  - `fixture_path`
  - `host_plan_path`
  - `runtime_requirement`
- detached non-core
  - `steps_executed`
  - coverage explanation
  - host-plan explanation
- aggregate-only
  - `host_plan_coverage_failure`

ここで `host_plan_coverage_failure` を bundle-first artifact 側へ入れない理由は、current code でそれが `run_bundle` 成功 payload から得られる field ではなく、`batch_summary_from_discovery` の failure classification として materialize されるためである。

## `host_plan_coverage_failure` の placement 境界

current helper stack では `host_plan_coverage_failure` を次のように読むのが最も自然である。

- current detached artifact
  - aggregate-only
- 将来 typed carrier に昇格させる最小 layer
  - bundle failure artifact
- 置かない layer
  - `RunReport` payload core
  - `bundle_context`
  - detached non-core

この切り分けを採る理由は、current code ですでに per-bundle failure bit が `BatchBundleOutcome::Failed` に現れており、成功 payload や bundle identity と混ぜるより failure artifact として独立させる方が責務境界を保ちやすいためである。

将来 bundle failure artifact 側へ typed carrier を足すとしても、最小 shape は `failure_kind` discriminator だけに留める。

- `bundle_context` は別 section のまま保つ
- detached non-core の short coverage note は typed core に混ぜない
- success artifact には同名 field を持ち込まない

## この先の update 指針

helper layer が変わったら、少なくとも次のどれを更新すべきかを見る。

- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `specs/examples/09..13`
- `Documentation.md`

更新不要なら、その task の report に `plan/ 更新不要` を明記する。
