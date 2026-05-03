# Report 1155 — P-A1-04c Practical Detach Minimal Contract

- Date: 2026-05-03
- Author / agent: Codex
- Scope: `P-A1-04c` practical detach minimal contract over the practical alpha-1 hot-plug floor
- Decision levels touched: `specs/18` practical alpha-1 scope boundary, `plan/44` roadmap memory, practical hotplug-plan/report carrier shape, snapshot/dashboard wording
- 日本語要約: practical alpha-1 の hot-plug floor に `HP-A1-07` を追加し、`operation_kind = detach` と `detach_boundary_ref` を持つ explicit deferred detach minimal contract を actualize した。`PA1-4` は 100% へ進んだが、これは detach runtime lifecycle や final object package attach ではなく、current practical package/hot-plug API closeout に限る。

## Objective

Close `P-A1-04c` without overclaim. Add the narrowest honest detach-time contract to the practical alpha-1 hot-plug floor, keep `checked package -> hotplug plan -> non-final hot-plug report` explicit, synchronize roadmap/snapshot/sample dashboards, and promote `P-A1-05` as the next practical package.

## Scope and assumptions

- Scope is limited to the practical alpha-1 hot-plug lane in `samples/practical-alpha1/`, `crates/mir-ast`, `crates/mir-runtime`, `scripts/`, and the synchronized docs.
- `HP-A1-07` actualizes exactly one detach minimal-contract row.
- The chosen branch is explicit `deferred`, not `accepted`, and not an attach-success alias.
- The practical carrier now admits `operation_kind` and detach-only `detach_boundary_ref`.
- `HP-A1-07` does not prove detach runtime lifecycle, rollback, durable migration, distributed activation ordering, final object package attach, Docker/local TCP, save/load, devtools, or public ABI completion.
- `PA1-4` can reach `100%` with this row because `specs/18` requires at least one explicit reject/defer/fallback detach outcome, not full detach execution.

## Start state / dirty state

- Package work started from the post-`P-A1-04b` practical hot-plug floor on `main`.
- During parallel review, reviewer-generated docs-only report commits advanced `main` to `a85e690` and then `bed8365`. Those commits were kept intact and treated as part of the live repo state.
- The working tree then became dirty only inside the `P-A1-04c` package scope:
  `crates/mir-ast`, `crates/mir-runtime`, `scripts/`, `samples/practical-alpha1/`, and synchronized docs.
- No unrelated user-authored edits were detected in the touched files.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `plan/30-attachpoint-detach-minimal-contract.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `docs/reports/1146-p-a1-04b-practical-hotplug-freshness-object-seam.md`
- `docs/reports/1147-review-pa1-04c-docs-progress-consistency.md`
- `docs/reports/1147-p-a1-04c-sample-validation-detach-review.md`
- `docs/reports/1148-review-p-a1-04c-detach-contract-theory-boundary.md`
- `docs/reports/1154-review-p-a1-04c-runtime-transport-boundary.md`

## Actions taken

1. Added explicit detach carrier lanes to the practical hot-plug input/plan/report surface:
   `operation_kind` plus detach-only `detach_boundary_ref`.
2. Added one new practical fixture and exact expected report for `HP-A1-07`.
3. Chose the `deferred` branch for the minimal contract after reviewer feedback, instead of the earlier in-progress reject-only sketch.
4. Kept the detach row at the request/verdict boundary:
   no attach success outputs are reused, no object detach is admitted, and no runtime detach lifecycle is claimed.
5. Added plan/runtime/Python tests for:
   detach carrier lowering,
   missing `detach_boundary_ref` rejection,
   exact runtime report equality,
   content-driven behavior,
   and sample-ID independence for semantics.
6. Regenerated the practical hot-plug expected reports so the widened stop lines / retained-later wording / detach row stay exact.
7. Synchronized `README.md`, `Documentation.md`, `specs/18`, `plan/44`, `progress.md`, `tasks.md`, `samples_progress.md`, `samples/README.md`, practical sample READMEs, and `scripts/README.md`.

## Files changed

- Code / tests:
  - `crates/mir-ast/src/practical_alpha1.rs`
  - `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
  - `crates/mir-ast/tests/practical_alpha1_hotplug_plan.rs`
  - `crates/mir-runtime/src/practical_alpha1_hotplug.rs`
  - `crates/mir-runtime/tests/practical_alpha1_hotplug.rs`
  - `scripts/practical_alpha1_attach.py`
  - `scripts/tests/test_practical_alpha1_attach.py`
- Practical fixtures / expected reports:
  - `samples/practical-alpha1/packages/hp-a1-07-detach-minimal-contract/package.mir.json`
  - `samples/practical-alpha1/expected/hp-a1-07-detach-minimal-contract.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-01-debug-layer-attach.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-02-non-admin-debug-rejected.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-03-auth-layer-contract-update.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-04-ratelimit-declared-failure.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-05-incompatible-patch-rejected.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-04b1-stale-membership-attach-rejected.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-04b2-missing-witness-attach-rejected.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-06-object-package-attach.expected.json`
- Docs / dashboards / repo memory:
  - `README.md`
  - `Documentation.md`
  - `specs/18-practical-alpha1-scope.md`
  - `plan/44-practical-alpha1-roadmap.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `samples/README.md`
  - `samples/practical-alpha1/README.md`
  - `samples/practical-alpha1/packages/README.md`
  - `samples/practical-alpha1/expected/README.md`
  - `scripts/README.md`
  - `docs/reports/1155-p-a1-04c-practical-detach-minimal-contract.md`

## Commands run

```bash
git status --short
git log --oneline --decorate -5
git branch --show-current
git rev-parse HEAD
git rev-parse origin/main
date '+%Y-%m-%d %H:%M JST'
cargo fmt
cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture
cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture
cargo test -p mir-runtime --test hotplug_runtime_skeleton -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture
cargo test -p mir-runtime --test alpha_layer_insertion_runtime
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_attach.py closeout --format json
python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_attach scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- Rust tests passed:
  - `mir-ast` practical front door: 11 passed
  - `mir-ast` practical hotplug plan: 8 passed
  - `mir-runtime` hotplug runtime skeleton: 8 passed
  - `mir-runtime` practical hotplug: 15 passed
  - `mir-runtime` alpha layer insertion runtime: 6 passed
- `python3 scripts/practical_alpha1_check.py check-all --format json` passed 10/10 with `first_checker_floor_complete: true`.
- `python3 scripts/practical_alpha1_run_local.py check-all --format json` passed 2/2 with `local_runtime_first_floor_complete: true`.
- `python3 scripts/practical_alpha1_attach.py check-all --format json` passed 9/9 with:
  - `package_hotplug_first_floor_complete: true`
  - `hotplug_plan_boundary_present: true`
  - `object_attach_seam_present: true`
  - `object_attach_claimed: false`
  - `freshness_negative_complete: true`
  - `detach_minimal_contract_complete: true`
  - `stage_pa1_4_complete: true`
- `python3 scripts/practical_alpha1_attach.py closeout --format json` reported:
  - implemented rows `HP-A1-01..05`, `HP-A1-04B1`, `HP-A1-04B2`, `HP-A1-06`, `HP-A1-07`
  - `limitations` keep detach runtime lifecycle, Docker/local TCP, save/load, and final public ABI out of scope
- `python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_attach scripts.tests.test_validate_docs` passed 34 tests.
- `python3 scripts/check_source_hierarchy.py` reported `73/73/0`.
- `python3 scripts/validate_docs.py` passed after the closeout report was added as the latest numbered report.
- `cargo fmt --check` passed after `cargo fmt`.
- `git diff --check` passed.

## What changed in understanding

- The narrow honest detach contract is not `accepted` and should not be expressed as attach-success with a renamed operation. It is better modeled as an explicit deferred boundary.
- `detach_boundary_ref` is necessary to keep detach semantics distinct from `activation_cut_ref`.
- The generic hot-plug skeleton already provides the right narrow runtime state language for `detach/deferred`; the practical package floor can reuse that boundary without claiming lifecycle ownership.
- A silent choice between reject/defer/fallback would be wrong. The repo memory and normative wording need to mirror that `P-A1-04c` chose the `defer` branch.

## Open questions

- How much of `P-A1-05` can reuse `PracticalAlpha1RuntimePlan` and `PracticalAlpha1HotPlugPlan` directly before a separate transport plan/report carrier is needed?
- Should `P-A1-05` start from local TCP only, or is the repo already narrow enough to introduce local TCP + Docker in the same package without overclaim?
- What is the smallest practical route-trace surface needed so `P-A1-05` proves separated transport/auth/membership/capability/witness lanes rather than a thick E2E wrapper?

## Suggested next prompt

Close `P-A1-05` by carrying the current practical package line through product-like local TCP / Docker transport, with separated transport/auth/membership/capability/witness lanes and without claiming production WAN, save/load, or public API freeze.

## Plan update status

`plan/` 更新済み:
`plan/44-practical-alpha1-roadmap.md` を更新し、`P-A1-04c` closeout、`HP-A1-07` deferred detach row、`PA1-4` closeout、`P-A1-05` promoted next を反映した。

## Documentation.md update status

`Documentation.md` 更新済み:
practical hot-plug floor を `HP-A1-07` まで同期し、detach path の `operation_kind` / `detach_boundary_ref` lane と non-claim wording を追加した。

## progress.md update status

`progress.md` 更新済み:
`PA1-4` を 100% に更新し、last closed package を `P-A1-04c` に進め、next autonomous package を `P-A1-05` に変更し、recent log を追記した。

## tasks.md update status

`tasks.md` 更新済み:
current practical status、ordered current work、package map を `P-A1-04c` close / `P-A1-05` promoted next に同期した。

## samples_progress.md update status

`samples_progress.md` 更新済み:
practical hot-plug family を `HP-A1-07` まで反映し、`PA1-4` closeout、`P-A1-05` next、validation row と dashboard percentages を更新した。

## Reviewer findings and follow-up

- Docs/progress consistency review:
  `1147-review-pa1-04c-docs-progress-consistency.md`
  - Follow-up: `progress.md` / `tasks.md` / `samples_progress.md` / practical READMEs were updated together so `PA1-4` does not appear both closed and open.
- Sample/validation review:
  `1147-p-a1-04c-sample-validation-detach-review.md`
  - Follow-up: added one new `HP-A1-07` fixture/expected report, content-driven mutation tests, and sample-ID independence coverage.
- Theory/spec review:
  `1148-review-p-a1-04c-detach-contract-theory-boundary.md`
  - Follow-up: switched the in-progress reject-only sketch to an explicit deferred boundary and mirrored that choice into `specs/18` / `plan/44`.
- Runtime/transport review:
  `1154-review-p-a1-04c-runtime-transport-boundary.md`
  - Follow-up: avoided attach-success output reuse for detach, added detach-only `detach_boundary_ref`, kept object detach outside the current floor, and left detach runtime lifecycle as later work.

## Skipped validations and reasons

- Did not run `cargo test -p mir-ast --test practical_alpha1_checker -- --nocapture` because `P-A1-04c` does not change checker logic; checker surface freshness was still covered by `python3 scripts/practical_alpha1_check.py check-all --format json` and the unified Python unittest suite.
- Did not run `cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture` because `P-A1-04c` does not change local-runtime execution; runtime-plan/local-runtime freshness was still covered by `python3 scripts/practical_alpha1_run_local.py check-all --format json` and the unified Python unittest suite.
- Did not run transport/devtools/save-load/product-prototype floors because `P-A1-04c` does not modify or claim those lanes.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- Reviewer agents were used for theory/spec, runtime/transport, docs/progress consistency, and sample/validation review.
- Four reviewer memo reports were produced and incorporated.
- Sessions remain open at report write and will be closed after package commit/push.
