# Report 1143 — P-A1-03 Local Runtime From Runtime Plan

- Date: 2026-05-03 17:39 JST
- Author / agent: Codex
- Scope: close `P-A1-03` by actualizing the first practical alpha-1 local-runtime floor over checked practical packages, with a distinct runtime-plan carrier, exact expected runtime reports for `RUN-01/02`, synchronized snapshot docs, review follow-up fixes, and package-closeout validation
- Decision levels touched: L1, L2
- 日本語要約: `P-A1-03` では、checked practical package をそのまま runtime report にせず、`PracticalAlpha1RuntimePlan` を中間 carrier として追加し、その plan を `PracticalAlpha1LocalRuntimeReport` に流す first local-runtime floor を閉じた。actualized row は `RUN-01` accepted local dispatch と `RUN-02` stale-membership rejection のみであり、package/hot-plug practical API、Docker/local TCP transport、save/load、devtools export、final public ABI はまだ claim していない。closeout 直前の review finding で見つかった `runtime_plan_emitted` overclaim、hardcoded closeout success、negative coverage 不足もこの package 内で解消した。

## Objective

Close `P-A1-03` as the first practical alpha-1 local-runtime floor:

- consume checked practical packages through a distinct runtime-plan carrier
- execute a narrow local queue/runtime floor over real practical package fixtures
- emit exact expected local-runtime reports for one positive and one negative runtime row
- keep package/hot-plug, transport, save/load, devtools, and public-ABI claims explicitly later

## Scope and assumptions

- Normative authority remains `specs/18-practical-alpha1-scope.md`.
- `samples/practical-alpha1/` remains a practical alpha-1 evidence/sample root, not an active repo-wide runnable root.
- The package closes only the first local-runtime floor:
  - `RUN-01` accepted local dispatch with event-DAG export
  - `RUN-02` stale-membership rejection before state mutation
- This package does not close:
  - practical package/hot-plug API
  - Docker/local TCP transport execution
  - local save/load command
  - devtools/public export schema
  - final public runtime API or product prototype

## Start state / dirty state

- Start state: `main` branch with `P-A1-03` implementation and docs edits already present in the working tree, not yet committed.
- Dirty state at closeout start:
  - modified docs/spec/plan/dashboard files for the practical alpha-1 line
  - modified Rust library files
  - new runtime-plan/local-runtime implementation, tests, sample fixtures, expected reports, and helper script files
- No unrelated user changes were reverted.

## Documents consulted

- `sub-agent-pro/alpha-1/00-index.md`
- `sub-agent-pro/alpha-1/02-practical-alpha1-definition.md`
- `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `sub-agent-pro/alpha-1/13-autonomous-package-sequence.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `docs/reports/1139-p-a1-02-typed-ir-checker-first-floor.md`
- `docs/reports/1140-p-a1-03-runtime-plan-local-runtime-boundary-review.md`
- `docs/reports/1140-p-a1-03-runtime-sample-validation-floor-review.md`
- `docs/reports/1140-pa1-03-docs-snapshot-review.md`
- `docs/reports/1142-review-p-a1-03-runtime-transport-boundary.md`

## Actions taken

1. Added a distinct runtime-plan carrier in `mir-ast`:
   - `crates/mir-ast/src/practical_alpha1_runtime_plan.rs`
   - validates that practical runtime execution starts only from a checked package with explicit `alpha_local_runtime_input`
   - rejects front-door-only and checker-rejected packages before runtime execution
2. Extended practical package input types in `crates/mir-ast/src/practical_alpha1.rs` so runtime-plan lowering can carry:
   - runtime place seeds
   - bootstrap participants
   - membership frontier advances
   - envelope/principal/auth lanes
   - one narrow local dispatch program
3. Added a distinct practical local-runtime report in `mir-runtime`:
   - `crates/mir-runtime/src/practical_alpha1_local_runtime.rs`
   - runs a runtime plan through `LogicalPlaceRuntimeShell` and local queue dispatch
   - keeps `runtime_plan_scope` and `runtime_scope` separate
   - preserves explicit later refs for Docker, package/hot-plug, save/load, devtools, and public ABI
4. Added a Rust example command:
   - `crates/mir-runtime/examples/mir_practical_alpha1_run_local.rs`
5. Added practical runtime fixtures and exact expected outputs:
   - `samples/practical-alpha1/packages/run-01-local-sugoroku/package.mir.json`
   - `samples/practical-alpha1/packages/run-02-stale-membership-rejected/package.mir.json`
   - `samples/practical-alpha1/expected/run-01-local-sugoroku.expected.json`
   - `samples/practical-alpha1/expected/run-02-stale-membership-rejected.expected.json`
6. Added the first practical local-runtime script surface:
   - `scripts/practical_alpha1_run_local.py`
   - row-keyed `RUN-01/02` plus aggregate `check-all` and `closeout`
7. Added focused tests:
   - `crates/mir-ast/tests/practical_alpha1_runtime_plan.rs`
   - `crates/mir-runtime/tests/practical_alpha1_local_runtime.rs`
   - `scripts/tests/test_practical_alpha1_run_local.py`
8. Addressed closeout review findings before final package close:
   - changed `runtime_plan_emitted` in the local-runtime report/expected JSON from `true` to `false` because the `run-local` surface does not emit a separate runtime-plan artifact
   - changed `scripts/practical_alpha1_run_local.py closeout` to derive completion/boundary booleans from `check_all()` instead of hardcoded success
   - widened negative coverage with:
     - malformed runtime-plan queue-kind rejection
     - checker-rejected runtime package rejection at the `run_local` entrypoint
9. Synchronized `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `specs/18`, `plan/01`, `plan/44`, `samples/README.md`, `samples/practical-alpha1/*/README.md`, and `scripts/README.md` to describe `P-A1-03` as a first local-runtime floor only.
10. Applied `cargo fmt` after `cargo fmt --check` found formatting drift and reran the focused Rust validation floor.

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/src/practical_alpha1.rs`
- `crates/mir-ast/src/practical_alpha1_runtime_plan.rs`
- `crates/mir-ast/tests/practical_alpha1_runtime_plan.rs`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/alpha_local_runtime.rs`
- `crates/mir-runtime/src/practical_alpha1_local_runtime.rs`
- `crates/mir-runtime/tests/practical_alpha1_local_runtime.rs`
- `crates/mir-runtime/examples/mir_practical_alpha1_run_local.rs`
- `samples/practical-alpha1/packages/run-01-local-sugoroku/package.mir.json`
- `samples/practical-alpha1/packages/run-02-stale-membership-rejected/package.mir.json`
- `samples/practical-alpha1/expected/run-01-local-sugoroku.expected.json`
- `samples/practical-alpha1/expected/run-02-stale-membership-rejected.expected.json`
- `scripts/practical_alpha1_run_local.py`
- `scripts/tests/test_practical_alpha1_run_local.py`
- `docs/reports/1143-p-a1-03-local-runtime-from-runtime-plan.md`

## Commands run

```bash
git status --short
date '+%Y-%m-%d %H:%M JST'
ls docs/reports | tail -n 15
rg -n "P-A1-03|1143|practical alpha-1|run-local|local runtime" progress.md tasks.md samples_progress.md Documentation.md plan/44-practical-alpha1-roadmap.md samples/practical-alpha1/README.md scripts/README.md
sed -n '1,240p' docs/reports/TEMPLATE.md
sed -n '1,220p' docs/reports/1140-p-a1-03-runtime-plan-local-runtime-boundary-review.md
sed -n '1,220p' docs/reports/1140-p-a1-03-runtime-sample-validation-floor-review.md
sed -n '1,220p' docs/reports/1140-pa1-03-docs-snapshot-review.md
sed -n '1,220p' docs/reports/1142-review-p-a1-03-runtime-transport-boundary.md
cargo test -p mir-ast
cargo test -p mir-runtime --test alpha_local_runtime
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_run_local.py check samples/practical-alpha1/packages/run-01-local-sugoroku --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_practical_alpha1_run_local scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
cargo fmt
cargo test -p mir-ast practical_alpha1_runtime_plan -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_local_runtime
python3 -m unittest scripts.tests.test_practical_alpha1_run_local
python3 scripts/practical_alpha1_run_local.py closeout --format json
date '+%Y-%m-%d %H:%M JST'
```

## Evidence / outputs / test results

- `cargo test -p mir-ast` passed, including:
  - `practical_alpha1_front_door` `11` tests
  - `practical_alpha1_checker` `3` tests
  - `practical_alpha1_runtime_plan` `4` tests before review follow-up; focused rerun passed `5` tests after adding malformed-input coverage
- `cargo test -p mir-ast practical_alpha1_runtime_plan -- --nocapture` passed `5` focused runtime-plan tests after review follow-up.
- `cargo test -p mir-runtime --test alpha_local_runtime` passed `3` regression tests, keeping the alpha-0 Stage-B floor green.
- `cargo test -p mir-runtime --test practical_alpha1_local_runtime` passed `4` focused tests after review follow-up.
- `python3 scripts/practical_alpha1_check.py check-all --format json` returned:
  - `sample_count: 10`
  - all `CHK-*` rows passed
  - `runtime_plan_emitted: false`
  - `run_local_claimed: false`
  - `run_docker_claimed: false`
- `python3 scripts/practical_alpha1_run_local.py check samples/practical-alpha1/packages/run-01-local-sugoroku --format json` returned the exact expected `RUN-01` accepted local-runtime report with:
  - `surface_kind = practical_alpha1_nonfinal_local_runtime_report`
  - `runtime_scope = practical-alpha1-local-runtime-floor`
  - `runtime_plan_scope = practical-alpha1-runtime-plan-floor`
  - `runtime_plan_emitted = false`
  - one accepted dispatch record
  - event DAG edges `publication_order`, `witness_order`, `handoff_order`
  - `run_docker_claimed = false`
  - `package_hotplug_claimed = false`
  - `save_load_claimed = false`
- `python3 scripts/practical_alpha1_run_local.py check-all --format json` returned:
  - `sample_count: 2`
  - `passed = ["RUN-01", "RUN-02"]`
  - `local_runtime_first_floor_complete: true`
  - `runtime_plan_boundary_present: true`
  - `run_docker_claimed: false`
  - `package_hotplug_claimed: false`
  - `save_load_claimed: false`
- `python3 scripts/practical_alpha1_run_local.py closeout --format json` returned a summary whose completion/boundary booleans are derived from `check_all()` rather than hardcoded constants.
- `python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_practical_alpha1_run_local scripts.tests.test_validate_docs` passed `26` tests after the additional `run-local` boundary test.
- `python3 scripts/check_source_hierarchy.py` passed with `required/present/missing = 73/73/0`.
- `python3 scripts/validate_docs.py` passed after report creation with `Documentation scaffold looks complete.` and `Found 1146 numbered report(s).`.
- `cargo fmt --check` initially failed on Rust formatting only; after `cargo fmt`, `cargo fmt --check` passed.
- `git diff --check` passed after the final report/timestamp edits.

## What changed in understanding

- `P-A1-03` can be closed honestly without widening into package/hot-plug or transport, as long as the carrier split stays explicit:
  - checked practical package
  - distinct runtime plan
  - non-final local-runtime report
- The existing alpha-0 Stage-B runtime substrate was sufficient as a lower execution layer, but not sufficient reason to reuse its sample-keyed report surface directly.
- The closeout review materially improved the honesty of the runtime surface:
  - the presence of an internal runtime-plan carrier is not the same thing as emitting a runtime-plan artifact on the command surface
  - package-closeout summaries must derive from executed checks rather than constants
  - runtime-boundary packages need at least one malformed-input and one checker-rejected negative anchored at the real entrypoints

## Open questions

- `P-A1-04` still needs to decide the narrow manifest-driven practical attach path for debug/rate-limit/auth/object packages without collapsing package admission into runtime execution.
- Capability/auth/witness lanes are carried through the runtime-plan/local-runtime surfaces, but full runtime enforcement remains later and must not be overclaimed in `P-A1-04`.
- A docs/dashboard closeout subagent could not be started because the agent thread limit was reached during closeout; I performed that focused review locally and recorded the result below.

## Suggested next prompt

Close `P-A1-04` by introducing a narrow manifest-driven practical package/hot-plug admission surface over the practical alpha-1 package root, reusing the `P-A1-03` runtime-plan/local-runtime floor without claiming Docker/local TCP, save/load, or public ABI completion.

## Plan update status

`plan/` 更新済み: `plan/01-status-at-a-glance.md` と `plan/44-practical-alpha1-roadmap.md` は `P-A1-03` closed / `P-A1-04` next を mirror する practical line memory に同期した。

## Documentation.md update status

`Documentation.md` 更新済み: practical alpha-1 line の current runtime floor を checked package -> runtime plan -> local runtime report の 3-carrier split として明記し、package/hot-plug、transport、save/load、devtools の non-claim を維持した。

## progress.md update status

`progress.md` 更新済み: practical alpha-1 stage、3-axis row、readiness map、recent log、large stage map の current package を `P-A1-03` closed / `P-A1-04` next に同期した。

## tasks.md update status

`tasks.md` 更新済み: `P-A1-03` freshness、practical package map、ordered current work、next promoted package を `P-A1-04` に揃えた。

## samples_progress.md update status

`samples_progress.md` 更新済み: `RUN-01/02` row、practical package map、dashboard timestamps、`PH0` repository-memory row を `P-A1-03` closeout stateへ同期した。

## Reviewer findings and follow-up

- Pre-close review evidence consulted:
  - `1140-p-a1-03-runtime-plan-local-runtime-boundary-review.md`
  - `1140-p-a1-03-runtime-sample-validation-floor-review.md`
  - `1140-pa1-03-docs-snapshot-review.md`
  - `1142-review-p-a1-03-runtime-transport-boundary.md`
- Additional closeout reviewers:
  - `Aristotle` reported no blocking findings. Residual risk: future widening could blur the checker/runtime-plan/runtime seam unless the explicit stop lines in `Documentation.md`, `specs/18`, and `plan/44` stay synchronized.
  - `Averroes` reported three concrete findings:
    1. `runtime_plan_emitted` overclaimed what the `run-local` surface actually exposed.
    2. `closeout` hardcoded success instead of deriving it from executed checks.
    3. Negative coverage was too narrow for malformed runtime-plan input and checker-rejected runtime packages.
- Follow-up implemented in this package:
  - changed `runtime_plan_emitted` to `false` in the local-runtime report and expected reports
  - changed `scripts/practical_alpha1_run_local.py closeout` to derive its booleans from `check_all()`
  - added malformed-input and checker-rejected boundary tests
- Docs/progress/dashboard closeout review was completed locally because a third reviewer could not be started under the current thread limit.

## Skipped validations and reasons

- No Docker/local TCP runtime validation was run for `P-A1-03` because this package closes only the first local-runtime floor; transport belongs to later `P-A1-05`.
- No practical save/load command validation was run because `P-A1-03` does not introduce a practical save/load surface; that belongs to later `P-A1-07`.
- No package/hot-plug practical API validation was run because `P-A1-03` explicitly stops before that boundary; that belongs to `P-A1-04`.

## Commit / push status

- Commit created and pushed: `d960987` (`mirrorea: close p-a1-03 local runtime floor`)
- Report metadata follow-up pending at the moment of this edit.

## Sub-agent session close status

- Closeout reviewer sessions completed:
  - `Aristotle` — theory/spec closeout review complete
  - `Averroes` — runtime/sample closeout review complete
- No dedicated docs reviewer session was opened because the thread limit was reached; that review was completed locally.
