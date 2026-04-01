# Report 0059 — current L2 host plan sidecar loader

- Date: 2026-04-01
- Author / agent: Codex
- Scope: current L2 parser-free minimal interpreter / host harness 向け machine-readable host plan schema と JSON sidecar loader の導入
- Decision levels touched: L2

## 1. Objective

Rust 側に ad hoc に書いていた fixture host plan / plan builder を machine-readable asset に外出しし、fixture ごとに predicate verdict、effect outcome、success-side carrier、trace expectation、narrative explanation を sidecar JSON で差し替えられる current L2 host plan schema と loader を追加する。

## 2. Scope and assumptions

- Mir-0 / Mir-1 / Mirrorea の境界は変更しない。
- canonical normalization law、rejection phase、static evidence floor、underdeclared handling、lineage annotation surface form、`perform` / `try` / `fallback` / `contract` policy、predicate sublanguage、option-local `admit` runtime reading、trace-audit reading、capability mismatch taxonomy、AST fixture schema、evaluation state schema、step semantics、oracle API、parser-free minimal interpreter skeleton、current host harness semantics は既存判断を維持する。
- host plan schema / loader は current L2 semantics の検証基盤に留め、production host interface にはしない。
- `must_explain` は machine-check に上げず、human-facing explanation obligation に残す。
- task 開始時点の既存 dirty state は無く、`git status --short --branch` は `## main...origin/main [ahead 2]` のみだった。
- 冒頭で `0056` の short re-review を行い、permissive default と hidden rule precedence に関する finding を取り込んだ。

## 3. Documents consulted

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
- `docs/reports/0052-review-0051-short-rereview.md`
- `docs/reports/0053-final-review-current-l2-interpreter-skeleton.md`
- `docs/reports/0054-current-l2-parser-free-interpreter-skeleton.md`
- `docs/reports/0055-review-0054-short-rereview.md`
- `docs/reports/0056-current-l2-host-stub-harness.md`
- `docs/reports/0057-final-review-current-l2-host-harness.md`
- `crates/mir-ast/tests/fixtures/current-l2/`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 4. Actions taken

1. `0056` に対する short re-review を行い、permissive default と hidden rule precedence の 2 finding を確認した。
2. fixture JSON への埋め込み案と sidecar JSON 案を比較し、AST fixture schema と host-side verification plan の責務を分離しやすい sidecar JSON を採用した。
3. `crates/mir-semantics/src/harness.rs` に machine-readable host plan schema を受ける serde loader を追加した。
   - `CURRENT_L2_HOST_PLAN_SCHEMA_VERSION`
   - `host_plan_sidecar_path_for_fixture_path`
   - `load_host_plan_from_path`
   - `load_host_plan_sidecar_for_fixture_path`
4. `FixtureStoreMutation`、`FixtureCommitPlan`、`PredicatePlanRule`、`EffectPlanRule`、`EffectPlanVerdict`、`TraceExpectationOverride` を JSON asset から deserialize できるようにした。
5. runtime fixture 用に `.host-plan.json` sidecar を追加した。
   - `e1-place-atomic-cut.host-plan.json`
   - `e2-try-fallback.host-plan.json`
   - `e3-option-admit-chain.host-plan.json`
   - `e6-write-after-expiry.host-plan.json`
6. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` から ad hoc な Rust-side plan builder を撤去し、runtime fixture は JSON sidecar から host plan を読むようにした。
7. loader / harness tests を補強し、次を確認するようにした。
   - sidecar path discovery
   - schema version mismatch rejection
   - overlapping predicate rule rejection
   - overlapping effect rule rejection
   - uncovered oracle call rejection
8. short re-review finding を踏まえ、harness を stricter に補正した。
   - runtime に入る fixture で実際に発生した oracle call を host plan が明示的に被覆することを要求
   - uncovered predicate call は fail-closed placeholder `Unsatisfied`
   - uncovered effect call は fail-closed placeholder `ExplicitFailure`
   - synthetic success-side commit の default 補完を削除
   - overlap する rule は loader / harness が reject
9. `specs/examples/07-current-l2-host-stub-harness.md` を tightening し、coverage 方針と overlap rejection を明文化した。
10. 新規補助文書 `specs/examples/08-current-l2-host-plan-schema.md` を追加し、host plan schema、sidecar JSON 方針、machine-check と narrative explanation の境界を整理した。
11. `specs/10-open-questions.md` と `specs/12-decision-register.md` に current L2 host plan schema / sidecar loader 方針を最小限反映した。
12. `Documentation.md` と `specs/00-document-map.md` に examples 08 への導線を追加した。
13. final reviewer に recheck を依頼し、`no findings` を確認した。

## 5. Evidence / outputs / test results

### Files changed

仕様本文 / 実装本文 commit `4a832bd` に含めた変更:

- 更新: `Documentation.md`
- 更新: `crates/mir-semantics/src/harness.rs`
- 更新: `crates/mir-semantics/src/lib.rs`
- 更新: `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- 更新: `specs/00-document-map.md`
- 更新: `specs/10-open-questions.md`
- 更新: `specs/12-decision-register.md`
- 更新: `specs/examples/07-current-l2-host-stub-harness.md`
- 新規: `specs/examples/08-current-l2-host-plan-schema.md`
- 新規: `crates/mir-ast/tests/fixtures/current-l2/e1-place-atomic-cut.host-plan.json`
- 新規: `crates/mir-ast/tests/fixtures/current-l2/e2-try-fallback.host-plan.json`
- 新規: `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.host-plan.json`
- 新規: `crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.host-plan.json`

short re-review finding を受けた follow-up fix commit `5d5d172` に含めた変更:

- 更新: `crates/mir-semantics/src/harness.rs`
- 更新: `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- 更新: `specs/10-open-questions.md`
- 更新: `specs/12-decision-register.md`
- 更新: `specs/examples/07-current-l2-host-stub-harness.md`
- 更新: `specs/examples/08-current-l2-host-plan-schema.md`

report commit に含める変更:

- 新規: `docs/reports/0058-review-0056-short-rereview.md`
- 新規: `docs/reports/0059-current-l2-host-plan-sidecar-loader.md`

### Commands run and exact outputs

task 開始時点:

```bash
git status --short --branch
```

```text
## main...origin/main [ahead 2]
```

short re-review finding:

```text
1. `FixtureHostStub` is not just a neutral stub boundary right now; it silently supplies permissive host semantics when the plan is incomplete.
2. The rule matcher has silent shadowing semantics of its own: fields are wildcardable, and the first matching rule wins, but overlapping rules are neither specified nor rejected.
```

verification:

```bash
cargo test -p mir-semantics
```

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 3.18s
running 9 tests
test harness_can_override_trace_expectation_without_changing_runtime_plan ... ok
test harness_rejects_uncovered_oracle_calls_without_synthetic_success_commit ... ok
test host_plan_loader_reads_sidecar_for_runtime_fixture ... ok
test host_plan_loader_rejects_overlapping_effect_rules ... ok
test host_plan_loader_rejects_overlapping_predicate_rules ... ok
test host_plan_loader_rejects_unknown_schema_version ... ok
test static_gate_rejects_malformed_and_underdeclared_fixtures ... ok
test runtime_fixtures_reach_expected_outcomes_via_declarative_host_plan ... ok
test trace_and_audit_expectations_follow_fixture_or_harness_override ... ok
test result: ok. 9 passed; 0 failed
```

```bash
cargo check -p mir-semantics
```

```text
Checking mir-semantics v0.1.0 (/home/yukatayu/dev/mir_poc_01/crates/mir-semantics)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.52s
```

```bash
python3 scripts/validate_docs.py
```

```text
Documentation scaffold looks complete.
Found 58 numbered report(s).
```

```bash
git diff --check
```

```text

```

```bash
python3 - <<'PY'
import json
from pathlib import Path
paths = sorted(Path('crates/mir-ast/tests/fixtures/current-l2').glob('*.host-plan.json'))
for path in paths:
    with path.open() as f:
        json.load(f)
print(f'Loaded {len(paths)} host plan sidecar(s).')
PY
```

```text
Loaded 4 host plan sidecar(s).
```

final reviewer recheck:

```text
no findings
```

### Findings

- sidecar JSON を fixture 本体から分離したことで、AST fixture schema と host-side verification plan の責務を current L2 で明確に分けられた。
- runtime fixture の host plan を JSON sidecar に出したことで、predicate verdict、effect outcome、success-side carrier、trace expectation override、narrative explanation override を Rust の ad hoc builder なしで差し替えられるようになった。
- short re-review の finding により、host harness は「未被覆 call を permissive success default で通さない」「overlap rule を reject する」という stricter な verification boundary に揃った。
- `must_explain` は引き続き machine-check に上げておらず、event / metadata / narrative の current L2 分離を維持した。
- 仕様本文 / 実装本文の commit hash は `4a832bd` と `5d5d172` である。report 自身の commit hash は self-reference の都合で本文に固定しない。

## 6. What changed in understanding

- host harness を machine-readable asset に外出しするときは、plan shape だけでなく「未被覆 call をどう invalid とみなすか」「rule overlap をどう reject するか」まで current L2 として最小限定めないと、hidden host semantics が入り込みやすいことが明確になった。
- sidecar JSON は parser-free PoC の実験ループに十分小さく、AST fixture schema を汚さずに host-side verification plan を差し替えられるため、current L2 の境界に非常に相性が良かった。

## 7. Open questions

- richer host interface は **未決定**。
- detached trace / audit serialization は **未決定**。
- multi-request scheduler は **未決定**。
- `Approximate` / `Compensate` を host plan carrier に入れるかは **未決定**。
- uncovered oracle call を runtime 前の richer preflight coverage analysis で完全検出するかは **未決定**。
- parser 実装前に host harness 側でさらに詰めるべき点として、sidecar discovery rule の長期固定と host plan schema の versioning policy は **未決定**。

## 8. Suggested next prompt

`crates/mir-semantics` の current L2 host plan sidecar loader を前提に、fixture expectation と host plan coverage をまとめて検証する小さな bundle loader を追加してください。特に fixture JSON と `.host-plan.json` sidecar を 1 つの test bundle として扱い、static gate / runtime outcome / trace-audit expectation / host plan coverage を一括で走らせる helper を入れると、次の PoC 実験ループがかなり回しやすくなります。
