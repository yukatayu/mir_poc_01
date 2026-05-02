# 1106 — P-A0-10 package review

## Objective

Review package `P-A0-10` with focus on semantic correctness, overclaim risk, missing tests, taxonomy drift, and whether the new avatar/package admission floor stays within the stop lines of `specs/16` and `specs/17`.

## Scope and assumptions

- In scope:
  `crates/mir-runtime/src/alpha_avatar_runtime.rs`,
  example `mirrorea_alpha_avatar_runtime`,
  `crates/mir-runtime/tests/alpha_avatar_runtime.rs`,
  `scripts/alpha_avatar_runtime_samples.py`,
  `scripts/tests/test_alpha_avatar_runtime_samples.py`,
  `samples/alpha/avatar-runtime/*expected.json`,
  `samples/alpha/hotplug-runtime/HP-11/12/15` sidecars,
  `Documentation.md`,
  `progress.md`,
  `tasks.md`,
  `samples_progress.md`,
  `samples/README.md`,
  `scripts/README.md`,
  `samples/alpha/README.md`,
  `samples/alpha/avatar-runtime/README.md`,
  `samples/alpha/hotplug-runtime/README.md`,
  `plan/01-status-at-a-glance.md`,
  `plan/42-runtime-package-avatar-roadmap.md`,
  `plan/43-alpha-e2e-roadmap.md`.
- Review baseline:
  repository instructions from `README.md`, `Documentation.md`, `progress.md`, `.docs/progress-task-axes.md`, `plan/00-index.md`, `specs/00..03`, `specs/09`, `specs/16`, `specs/17`, and relevant `plan/01`, `plan/42`, `plan/43`.
- Prior scope constraints treated as fixed:
  avatar formats stay non-core, signatures are not safety proof, native execution is not claimed, detach lifecycle is not complete, and `samples/alpha/` is not silently promoted to active roots.

## Start state / dirty state

- Start time: `2026-05-02 09:34:39 JST`
- Worktree was dirty at review start.
- Modified or untracked package files already existed in the workspace before this review report was written.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `samples/README.md`
- `samples/alpha/README.md`
- `samples/alpha/avatar-runtime/README.md`
- `samples/alpha/hotplug-runtime/README.md`
- `scripts/README.md`

## Actions taken

- Read the required repo front-door and normative documents in repository order.
- Inspected the package implementation, example, tests, runner, sidecars, and snapshot docs.
- Compared package claims against the stop lines in `specs/16` and `specs/17`.
- Ran the targeted Rust and Python validation commands for the package.
- Checked report inventory under `docs/reports/` to verify closeout evidence claims.

## Files changed

- Added this review report only:
  `docs/reports/1106-p-a0-10-package-review.md`

## Commands run

```bash
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md
sed -n '1,220p' .agents/skills/discord-report/SKILL.md
sed -n '1,260p' /home/yukatayu/.codex/skills/superpowers/skills/receiving-code-review/SKILL.md
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
git branch --show-current
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
ls specs
sed -n '1,260p' progress.md
sed -n '1,260p' .docs/progress-task-axes.md
sed -n '1,260p' plan/00-index.md
sed -n '1,260p' specs/00-document-map.md
sed -n '1,260p' specs/01-charter-and-decision-levels.md
sed -n '1,260p' specs/02-system-overview.md
sed -n '1,260p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,260p' specs/16-runtime-package-adapter-hotplug.md
sed -n '1,260p' specs/17-mirrorea-spaces-alpha-scope.md
sed -n '1,260p' plan/01-status-at-a-glance.md
sed -n '1,260p' plan/42-runtime-package-avatar-roadmap.md
sed -n '1,260p' plan/43-alpha-e2e-roadmap.md
git diff --stat -- crates/mir-runtime/src/lib.rs crates/mir-runtime/src/alpha_avatar_runtime.rs crates/mir-runtime/examples/mirrorea_alpha_avatar_runtime.rs crates/mir-runtime/tests/alpha_avatar_runtime.rs scripts/alpha_avatar_runtime_samples.py scripts/tests/test_alpha_avatar_runtime_samples.py samples/alpha/avatar-runtime samples/alpha/hotplug-runtime Documentation.md progress.md tasks.md samples_progress.md samples/README.md scripts/README.md samples/alpha/README.md samples/alpha/avatar-runtime/README.md samples/alpha/hotplug-runtime/README.md plan/01-status-at-a-glance.md plan/42-runtime-package-avatar-roadmap.md plan/43-alpha-e2e-roadmap.md
sed -n '1,520p' crates/mir-runtime/src/alpha_avatar_runtime.rs
sed -n '1,260p' crates/mir-runtime/examples/mirrorea_alpha_avatar_runtime.rs
sed -n '1,260p' crates/mir-runtime/tests/alpha_avatar_runtime.rs
sed -n '1,520p' scripts/alpha_avatar_runtime_samples.py
sed -n '1,260p' scripts/tests/test_alpha_avatar_runtime_samples.py
rg -n "alpha_avatar_runtime_samples|expected.json|check-all" scripts samples/alpha crates/mir-runtime/tests -g '!target'
sed -n '1,240p' samples/alpha/README.md
sed -n '1,240p' samples/alpha/avatar-runtime/README.md
sed -n '1,260p' samples/alpha/hotplug-runtime/README.md
sed -n '1,260p' samples/README.md
cargo test -p mir-runtime --test alpha_avatar_runtime
python3 -m unittest scripts.tests.test_alpha_avatar_runtime_samples
cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- closeout
python3 scripts/alpha_avatar_runtime_samples.py check-all --format json
ls docs/reports | tail -n 10
find docs/reports -maxdepth 1 -type f | sort | tail -n 20
date '+%Y-%m-%d %H:%M:%S %Z'
```

## Evidence / outputs / test results

- `cargo test -p mir-runtime --test alpha_avatar_runtime` passed: 9 tests.
- `python3 -m unittest scripts.tests.test_alpha_avatar_runtime_samples` passed: 5 tests.
- `cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- closeout` produced JSON closeout output for 8 implemented rows.
- `python3 scripts/alpha_avatar_runtime_samples.py check-all --format json` passed with `sample_count = 8`, `failed = []`.
- `docs/reports/` inventory at review time had no `1105-*` report file even though snapshot docs referenced report `1105`.

## What changed in understanding

- The package is not merely a docs-first scaffold update. It does provide a real runtime-private report generator, example entrypoint, and runner for `AV-01/02/06/08/09` and `HP-11/12/15`.
- The implementation remains inside the non-core / non-native-execution / non-active-root stop lines at a high level.
- The main review problems are narrower:
  semantic mismatch in the fallback-admission path,
  missing sidecar parity enforcement,
  closeout/report overclaim,
  and hotplug planned-row taxonomy drift.

## Open questions

- Should `AV-08` model fallback as:
  requested package rejected/deferred plus placeholder attached separately,
  or requested package accepted-with-degradation as a first-class verdict kind?
- Should the package closeout contract require exact parity between runtime output and committed `*.expected.json` sidecars, as done by the other alpha runtime floors?
- Should `HP-01/07/08/13/14` remain explicitly listed in runner closeout output while still marked planned or mirrored elsewhere?

## Suggested next prompt

Review and fix the `P-A0-10` findings from `docs/reports/1106-p-a0-10-package-review.md`: separate fallback from accepted admission in `AV-08`, add sidecar-parity checks for `alpha_avatar_runtime`, restore the missing hotplug planned-row inventory in the runner/closeout output, and then sync the snapshot docs only after a real package closeout report exists.

## Plan update status

- `plan/` update not performed in this review task.
- Review-only finding: current `plan/42-runtime-package-avatar-roadmap.md` is internally consistent with the package intent, but snapshot docs claim closeout evidence that does not yet exist as a numbered report.

## Documentation.md update status

- Not updated in this review task.
- Review-only finding: `Documentation.md` keeps the correct high-level non-claim boundaries, but its package actualization claim depends on validation/report evidence that is not fully backed by sidecar-parity tests or a committed closeout report.

## progress.md update status

- Not updated in this review task.
- Review-only finding: `progress.md` currently overclaims package closure by treating `P-A0-10` as closed and by recording closeout evidence that points at missing report `1105`.

## tasks.md update status

- Not updated in this review task.
- Review-only finding: `tasks.md` currently marks `P-A0-10` closed with report `1105`, but that report file is absent.

## samples_progress.md update status

- Not updated in this review task.
- Review-only finding: `samples_progress.md` currently cites report `1105` and package-closeout-complete status without corresponding report evidence.

## Reviewer findings and follow-up

- Finding 1:
  `AV-08` conflates fallback with accepted admission. The request targets `RuntimePackage[avatar.custom.runtime_missing@1.0.0]`, `host_support_satisfied` fails, and the selected package becomes the placeholder, yet the verdict is still `accepted`. This overstates the manifest-admission floor and blurs the explicit failure/fallback boundary required by `specs/09` and `specs/16`.
- Finding 2:
  The new runtime tests and runner do not compare runtime output against the committed `*.expected.json` sidecars. Unlike `alpha_local_runtime` and `alpha_layer_insertion_runtime`, the package can drift in sidecar content without any failing test.
- Finding 3:
  Snapshot docs and dashboards reference report `1105` and package closure, but `docs/reports/` contains no such file. This is a concrete overclaim and violates the repository reporting discipline.
- Finding 4:
  `alpha_avatar_runtime_samples.py` underreports planned hotplug rows. The closeout payload and stop line only mention `HP-09/10`, while package docs and `plan/42` still classify `HP-01/07/08/09/10/13/14` as planned or mirrored elsewhere.

## Skipped validations and reasons

- Did not run full repository-wide validation floor because this task was a targeted package review.
- Did not run Docker/network alpha validation because it belongs to `P-A0-09`, not the avatar/package package under review.
- Did not run source-hierarchy/docs validator commands in this review-only task because the goal was package correctness review, not snapshot closeout; the package already claimed those checks in its own snapshot docs.

## Commit / push status

- No commit created.
- No push performed.

## Sub-agent session close status

- No sub-agent session was opened for this review task.
