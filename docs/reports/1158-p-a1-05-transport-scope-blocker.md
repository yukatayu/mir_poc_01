# Report 1158 — P-A1-05 Transport Scope Blocker

- Date: 2026-05-03
- Author / agent: Codex
- Scope: `P-A1-05` transport practical E2E blocker recording, roadmap/snapshot reconciliation, and user-decision handoff
- Decision levels touched: `specs/18` interpretation boundary unchanged, `plan/44` repository-memory blocker status, `Documentation.md` / `progress.md` / `tasks.md` / `samples_progress.md` snapshot wording
- 日本語要約: `P-A1-05` は実装を始める前に scope contradiction が見つかったため、transport carrier や sample 実装を進めずに blocker package として記録した。`plan/44` / stage-roadmap は transport-stage missing-capability / missing-witness negatives を要求しているが、current sample matrix / snapshot wording は accepted local/Docker path と stale-membership / route-trace / auth-lane rows までしか安定していない。current recommendation は silent narrowing ではなく、distinct transport negatives を含む wider row set を user decision で fix すること。

## Objective

Stop `P-A1-05` before dishonest implementation. Record the practical transport scope contradiction in repository memory and current snapshots, preserve the existing review findings, and return the exact decision that must be made before the practical alpha-1 stage sequence can continue.

## Scope and assumptions

- This package is docs-only.
- No practical transport carrier, sample fixture, expected report, Rust runtime path, or Python runner is added in this package.
- `specs/18-practical-alpha1-scope.md` stays unchanged because the package does not make a new normative decision.
- The blocker is specific to `P-A1-05` practical transport scope:
  roadmap memory and stage-roadmap wording require a broader transport negative set than the current practical sample matrix / snapshot wording stabilizes.
- The package must preserve the distinction between:
  current practical hot-plug closeout through `P-A1-04c`,
  existing alpha-0 Stage C evidence,
  and not-yet-actualized practical transport carrier work.

## Start state / dirty state

- Work started on `main` after `P-A1-04c` was already closed and pushed.
- The working tree was already dirty inside the blocker-package scope:
  `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `plan/44-practical-alpha1-roadmap.md`, plus four new review reports under `docs/reports/`.
- No Rust, Python, sample fixture, or normative spec file was left half-modified for this package.
- The dirty state came from scope review and snapshot synchronization work for `P-A1-05`, not from a partially accepted transport implementation.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `sub-agent-pro/alpha-1/04-stage-roadmap.md`
- `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `sub-agent-pro/alpha-1/13-autonomous-package-sequence.md`
- `samples/practical-alpha1/README.md`
- `docs/reports/1155-p-a1-04c-practical-detach-minimal-contract.md`
- `docs/reports/1156-review-p-a1-05-docs-progress-consistency.md`
- `docs/reports/1156-p-a1-05-sample-validation-review.md`
- `docs/reports/1157-review-p-a1-05-runtime-transport-carrier-split.md`
- `docs/reports/review-2026-05-03-pa1-05-transport-scope-review.md`

## Actions taken

1. Re-read the practical alpha-1 roadmap and sample-matrix files that define `P-A1-05`.
2. Compared those files against current snapshot docs and the existing practical package line after `P-A1-04c`.
3. Confirmed that the contradiction is not a simple implementation omission:
   the repo currently lacks one stable admissible row set for `P-A1-05`.
4. Chose not to implement a transport carrier, not to add `TR-A1-*` fixtures, and not to borrow `RUN-*` or `HP-*` evidence.
5. Recorded the blocker in `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, and `plan/44-practical-alpha1-roadmap.md`.
6. Added a dedicated blocker report so the next user decision is explicit and does not depend on reading working-tree diffs.

## Files changed

- Snapshot / repository-memory docs:
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `plan/44-practical-alpha1-roadmap.md`
- Review evidence carried into this package:
  - `docs/reports/1156-review-p-a1-05-docs-progress-consistency.md`
  - `docs/reports/1156-p-a1-05-sample-validation-review.md`
  - `docs/reports/1157-review-p-a1-05-runtime-transport-carrier-split.md`
  - `docs/reports/review-2026-05-03-pa1-05-transport-scope-review.md`
- New closeout report:
  - `docs/reports/1158-p-a1-05-transport-scope-blocker.md`

## Commands run

```bash
git status --short
date '+%Y-%m-%d %H:%M JST'
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- Practical transport scope contradiction confirmed:
  - `plan/44` / stage-roadmap still define `PA1-5` with `stale membership / missing capability / missing witness negatives`.
  - `sub-agent-pro/alpha-1/08-sample-matrix.md` and current snapshot wording only stabilize:
    accepted local TCP path,
    accepted Docker path,
    stale-membership reject,
    route-trace row,
    auth-lane-separate row.
- Review findings aligned on the same blocker from four angles:
  - docs/progress consistency
  - sample/validation completeness
  - runtime/transport carrier split
  - theory/spec scope review
- No practical transport implementation was accepted:
  there is still no practical `TR-A1-*` runner, no transport expected-report family, and no actualized practical transport carrier.
- Docs validation floor passed for this blocker package:
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `python3 -m unittest scripts.tests.test_validate_docs`
  - `cargo fmt --check`
  - `git diff --check`

## What changed in understanding

- The `P-A1-05` problem is not “transport work has not started yet”; it is “the repo currently contains two inconsistent readings of what transport closeout must prove.”
- Silent narrowing would damage the sequential large-stage reading the user explicitly asked to preserve.
- Reusing `RUN-*` or `HP-*` negatives would hide a carrier mismatch instead of solving it.
- Therefore the next honest step is a user decision on the admissible row set, not implementation.

## Open questions

- Should `P-A1-05` adopt `Option A` and widen the practical transport matrix with distinct transport-specific missing-capability / missing-witness negatives?
- Or should `P-A1-05` adopt `Option B` and be explicitly narrowed to the current accepted local/Docker + stale-membership / route-trace / auth-lane set?
- If `Option A` is chosen, should route-trace and auth-lane separation remain one combined security row or be split into two explicit rows?

## Suggested next prompt

Choose the `P-A1-05` practical transport row-set direction:
`Option A` keep the broader roadmap reading and add distinct transport-specific missing-capability / missing-witness negatives,
or `Option B` narrow `P-A1-05` explicitly to the current 5-row set and defer those negatives to a later package.

## Plan update status

`plan/` 更新済み:
`plan/44-practical-alpha1-roadmap.md` に `P-A1-05` promoted-next だが blocked であること、current contradiction、silent narrowing 不可、current recommendation を追加した。

## Documentation.md update status

`Documentation.md` 更新済み:
practical alpha-1 sectionに、transport practical E2E は next gate だが row-set blocker があり、practical transport carrier はまだ actualize していないことを追記した。

## progress.md update status

`progress.md` 更新済み:
`PA1-5` を blocked on row-set reconciliation に変更し、current blocker / next autonomous package / user-decision blocker wording と recent log を同期した。

## tasks.md update status

`tasks.md` 更新済み:
`P-A1-05` を blocked pending user decision に変更し、ordered current work と package map を同期し、専用 user-decision blocker を追加した。

## samples_progress.md update status

`samples_progress.md` 更新済み:
practical alpha-1 summary / package map / blocker table / recent validation rowを `P-A1-05` blocker package に合わせて更新した。

## Reviewer findings and follow-up

- `1156-review-p-a1-05-docs-progress-consistency.md`
  - Follow-up: `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, and `plan/44` were synchronized so `P-A1-05` no longer appears both promoted-next and implementation-ready.
- `1156-p-a1-05-sample-validation-review.md`
  - Follow-up: the blocker wording now states explicitly that current practical transport closeout is under-specified if the roadmap keeps missing-capability / missing-witness negatives inside `PA1-5`.
- `1157-review-p-a1-05-runtime-transport-carrier-split.md`
  - Follow-up: the package does not reuse `practical_alpha1_local_runtime` or alpha-0 Stage C reports as if they were a practical transport carrier.
- `review-2026-05-03-pa1-05-transport-scope-review.md`
  - Follow-up: the blocker report now records the broader matrix drift and keeps `P-A1-05` blocked instead of silently promoting implementation.

## Skipped validations and reasons

- Did not run Rust or Python practical transport tests because no practical transport implementation was accepted in this package.
- Did not run `cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture`, checker, runtime-plan, local-runtime, or hot-plug suites because the package is docs-only and does not modify those lanes.
- Did not run Docker transport floors because this package intentionally stops before adding a practical transport runner or fixture family.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- Existing reviewer agents for theory/spec, runtime/transport, docs/progress consistency, and sample/validation were reused as evidence sources for this blocker package.
- They will be closed after commit/push once this blocker state is recorded on `origin/main`.
