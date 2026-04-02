# Report 0090 — current L2 trace audit serialization vs richer host interface boundary

- Date: 2026-04-03
- Author / agent: Codex
- Scope: current L2 parser-free PoC 基盤を前提に、detached trace / audit serialization と richer host interface / coverage analysis の最小 boundary を narrow scope で比較し、PoC 実験ループを「より回せる」段階へ進めるにはどちらを先に固定すべきかを整理する
- Decision levels touched: L2

## 1. Objective

current L2 の core semantics と notation family はいったん十分安定したとみなし、次の operational bottleneck を比較する。

今回比較するのは次の 2 つである。

1. detached trace / audit serialization
2. richer host interface と coverage analysis

目的は、current parser-free fixture loop を「狭く正確に回す」段階から「大量に回して比較しやすい」段階へ一歩進めるために、まずどちらを先に固定すべきかを source-backed に判断することである。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/03-current-l2-evaluation-state-schema.md`
- `specs/examples/04-current-l2-step-semantics.md`
- `specs/examples/05-current-l2-oracle-api.md`
- `specs/examples/06-current-l2-interpreter-skeleton.md`
- `specs/examples/07-current-l2-host-stub-harness.md`
- `specs/examples/08-current-l2-host-plan-schema.md`
- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `specs/examples/11-current-l2-selection-helper.md`
- `specs/examples/12-current-l2-selection-profiles.md`
- `specs/examples/13-current-l2-profile-catalog.md`
- `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
- `plan/01-status-at-a-glance.md`
- `plan/05-fallback-lease-and-chain-semantics.md`
- `plan/06-surface-notation-status.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `docs/reports/0054-current-l2-parser-free-interpreter-skeleton.md`
- `docs/reports/0056-current-l2-host-stub-harness.md`
- `docs/reports/0059-current-l2-host-plan-sidecar-loader.md`
- `docs/reports/0060-current-l2-bundle-loader.md`
- `docs/reports/0062-current-l2-batch-runner.md`
- `docs/reports/0063-current-l2-selection-helper.md`
- `docs/reports/0064-current-l2-selection-profiles.md`
- `docs/reports/0066-current-l2-profile-catalog.md`
- `docs/reports/0069-current-l2-profile-catalog-single-source-of-truth.md`
- `docs/reports/0076-current-l2-profile-catalog-internal-integration-test-boundary.md`
- `docs/reports/0077-current-l2-helper-layer-responsibility-alignment.md`
- `docs/reports/0078-current-l2-fallback-lease-regression-fixtures.md`
- `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md`
- `docs/reports/0085-plan-repository-memory-externalization.md`
- `docs/reports/0086-review-plan-memory-doc-boundary-consistency.md`
- `docs/reports/0087-current-l2-a2-rollout-boundary.md`
- `docs/reports/0088-current-l2-representative-prose-drift-check.md`
- `docs/reports/0089-review-followup-0088-and-poc-blockers.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-ast/tests/fixtures/current-l2/`

## 3. Actions taken

1. current L2 parser-free PoC stack の call chain と helper boundary を再確認した。
2. `ExpectedTraceAudit`、`TraceAuditSink`、`RunReport`、`FixtureHostPlan`、`FixtureHostStub`、`run_bundle`、`batch_summary_from_discovery` の責務を source から読み直した。
3. detached trace / audit serialization と richer host interface / coverage analysis を、次の観点で比較した。
   - parser-free fixture loop に対する前進量
   - 実装コスト
   - helper stack への波及範囲
   - tests / review のしやすさ
   - plan mirror への影響
   - multi-request scheduler への接続しやすさ
4. detached artifact を docs-only で考えるための最小 shape を report 内に schema sketch として記述した。
5. near-term 優先順位と risk register が変わるため、`plan/11`、`plan/12`、traceability の `plan/90` を最小更新した。
6. reviewer を 1 回だけ起動して long wait し、bottleneck ordering を repo-wide settled judgment に見せすぎているという 1 finding を受けた。
7. `fixture authoring / elaboration` の独立 bottleneck を取り消していないことを明示し、trace/audit vs host の **比較に限った provisional judgment** へ wording を弱めた。

## 4. Files changed

- 更新: `plan/11-roadmap-near-term.md`
- 更新: `plan/12-open-problems-and-risks.md`
- 更新: `plan/90-source-traceability.md`
- 新規: `docs/reports/0090-current-l2-trace-audit-vs-host-interface-boundary.md`
- 新規: `docs/reports/0091-review-current-l2-boundary-comparison-narrow-scope.md`

task 開始時点の既存 dirty state:

- branch 状態: `main...origin/main [ahead 3]`
- 未追跡: `diff_investigation_01.md`
- 未追跡ディレクトリ: `旧資料_参考_ChatGPT_01_69c5e3f6/`
- 未追跡ディレクトリ: `旧資料_参考_ChatGPT_02_Mirrorea_2025/`

## 5. Commands run and exact outputs

```bash
git status --short --branch
python3 scripts/validate_docs.py
git diff --check
cargo test -p mir-semantics
git add plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/90-source-traceability.md
git -c commit.gpgsign=false commit -m "trace と host interface の境界比較を整理する"
```

### Exact outputs

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 91 numbered report(s).
```

- `git diff --check`

```text
(no output)
```

- `cargo test -p mir-semantics`

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
running 2 tests
test harness::tests::named_profile_catalog_aliases_are_derived_from_internal_specs ... ok
test harness::tests::named_profile_catalog_resolve_is_derived_from_internal_specs ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 33 tests
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

- `git -c commit.gpgsign=false commit -m "trace と host interface の境界比較を整理する"`

```text
[main 44ec7e0] trace と host interface の境界比較を整理する
 3 files changed, 45 insertions(+), 28 deletions(-)
```

仕様本文 / plan 本文 commit hash は `44ec7e0` である。review report `0091` は同 task 内の review artifact であり、report 自身の commit hash は self-reference の都合で本文に固定しない。

## 6. Evidence / findings

### 6.1 current loop のどこが詰まり始めているか

current parser-free PoC loop は、single bundle / directory batch / selection / profile / named profile までは揃っている。  
しかし、結果の主要 carrier はまだ process 内表現に強く寄っている。

- trace / audit は `TraceAuditSink` に集約され、`run_to_completion()` の `RunReport` に載る
- bundle 層はその場で `event_kinds`、formal `non_admissible_metadata`、short `narrative_explanations` を exact compare する
- batch / profile / catalog 層はその比較結果を summary に束ねる
- host coverage failure は batch 層で文字列検出に依存して集計している

要するに、**いまの loop は「正しく回る」が、「大量に保存して後で横比較する」境界はまだ持っていない**。

### 6.2 detached trace / audit serialization を先にやる利点

#### 前進量

- 1 run ごとの trace / audit を repo 外 artifact として残せる
- batch / profile 実行の結果をあとから diff しやすくなる
- future の multi-request scheduler に進む前でも、single-request baseline の蓄積ができる

#### helper stack への波及範囲

- 主に `TraceAuditSink`、`RunReport`、`BundleRunReport`、batch/profile summary の外側に薄い export boundary を足す話で済む
- `PredicateOracle` / `EffectOracle` の trait や host-plan semantics 自体は大きく変えずに済む

#### tests / review のしやすさ

- current exact compare core をそのまま artifact field へ落としやすい
- `must_explain` は human-facing explanation として detached artifact に残せても、machine-check 昇格は避けられる
- semantics に触れずに verification / tooling boundary を切れる

### 6.3 richer host interface を先にやる弱点

#### 前進量

- preflight
- uncovered call detection の見え方
- coverage explanation
- host-plan coverage failure の typed 化

には効く。

ただし、これだけでは結果保存・再比較・後解析は依然として弱い。

#### helper stack への波及範囲

- `PredicateOracle` / `EffectOracle`
- `FixtureHostPlan`
- `FixtureHostStub`
- `run_bundle`
- `batch_summary_from_discovery`

まで広く触りやすい。

しかも current host harness は verification basis であり production host ではないため、ここを早く肥大化させると **helper boundary と production boundary が混線しやすい**。

#### tests / review のしやすさ

- uncovered call / overlap / preflight / richer carrier を足し始めると review 面積が広がる
- host 側の convenience が semantics と誤読されやすい
- batch summary に残る string-based coverage detection の改善は必要だが、それだけのために host API を先に大きくするのは cost が高い

### 6.4 provisional judgment

`fixture authoring / elaboration` の独立 bottleneck は別に残る。  
そのうえで、**detached trace / audit serialization と richer host interface の 2 項目を比べるなら、まず detached trace / audit serialization を先にやる**のが current L2 では最も前進量が大きい。

理由は次のとおりである。

1. current parser-free loop の強みである exact compare core を壊さずに伸ばせる
2. host interface を早く広げるより波及範囲が狭い
3. repo 外保存・再比較・後解析という「大量に回す」要件に直結する
4. richer host interface / coverage analysis は、そのあとでも入口を narrow に切り直せる

### 6.4.1 reviewer finding と修正

reviewer からの唯一の finding は、detached trace / audit を「repo 全体の次の主 bottleneck」と断定したように読める wording が強すぎる、というものだった。  
既存の `0089` では `fixture authoring / elaboration` も独立 bottleneck として残っているため、今回は次のように弱めた。

- `fixture authoring / elaboration` の独立 bottleneck は残る
- そのうえで、**detached trace / audit serialization と richer host interface の 2 項目を比べるなら** detached を先に切る
- plan 側の表現も `comparison 上の先行候補 / 後続候補` に揃える

この修正後、reviewer が指摘した「未決事項を決まったことのように見せる」問題は局所化できたと判断した。

### 6.5 detached artifact の最小 shape

今回は production schema を固定しない。  
ただし comparison aid として、current L2 で最小に必要そうな detached artifact は次程度で足りる。

```json
{
  "schema_version": "draft-current-l2-trace-audit-v1",
  "fixture_id": "e7-write-fallback-after-expiry",
  "runtime_requirement": "runtime-with-host-plan",
  "static_verdict": "Valid",
  "entered_evaluation": true,
  "terminal_outcome": "Success",
  "steps_executed": 5,
  "event_kinds": ["RequestStarted", "OptionSkipped", "EffectCommitted"],
  "non_admissible_metadata": [
    { "option_ref": "fast-write", "subreason": "lease-expired" }
  ],
  "narrative_explanations": [
    "capability mismatch for cache-write"
  ],
  "bundle_summary": {
    "host_plan_coverage_failure": false
  }
}
```

この sketch では、current machine-check core と human-facing explanation を次のように分ける。

- exact compare に残す core
  - `static_verdict`
  - `entered_evaluation`
  - `terminal_outcome`
  - `steps_executed` のような軽量 operational counter
  - `event_kinds`
  - formal `non_admissible_metadata`
  - short `narrative_explanations`
- human-facing explanation に残すもの
  - `must_explain`
  - 長文 audit
  - why-this-is-good/bad の解説文

つまり detached artifact を作っても、**`must_explain` を machine-check に上げる必要はない**。

### 6.6 richer host interface は何を後段に残すか

detached artifact の次に詰めるべき richer host interface の最小論点は次である。

- uncovered call detection を batch summary の文字列検出から typed field へ寄せるか
- preflight で「この host plan は fixture runtime path を覆いきれていない」と言うか
- coverage explanation を machine-check core に入れず summary / detached artifact に残すか
- `PredicateOracle` / `EffectOracle` の trait は保ったまま carrier だけ少し増やすか

ここでも production host interface へ直進せず、**current host harness を verification basis に留める**のが前提である。

## 7. Changes in understanding

- current L2 の notation や fallback reading は、直近 bottleneck ではなくなっている。
- 「PoC をもっと回す」ための次の詰まりは、結果 carrier をどう外へ持ち出すかである。
- richer host interface は必要だが、coverage analysis の convenience を先に増やすより、trace / audit artifact を先に切る方が helper boundary を壊しにくい。
- `must_explain` を human-facing explanation obligation に残したまま detached artifact を作れるため、current L2 policy と矛盾しない。

## 8. Open questions

- detached trace / audit artifact の schema versioning をどこで固定するか
- `steps_executed` のような operational counter を detached artifact core に入れるか
- host-plan coverage failure を typed field として bundle / batch summary に昇格するか
- preflight coverage analysis を richer host interface に入れるか、detached artifact の後解析側へ寄せるか
- final parser syntax
- machine-readable catalog asset / manifest
- selector grammar / alias grammar の長期固定
- path canonicalization policy
- multi-request scheduler
- `Approximate` / `Compensate`
- static analysis / theorem prover 側との boundary

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、detached trace / audit serialization の最小 artifact schema を docs-only で切り出し、RunReport / BundleRunReport / BatchRunSummary のどの field だけを exact compare core として外へ出すべきかを narrow scope で整理してください。`
