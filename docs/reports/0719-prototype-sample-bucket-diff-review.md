# Report 0719 — prototype sample bucket diff review

- Date: 2026-04-17T01:54:14.505369Z
- Author / agent: Codex
- Scope: review of the current uncommitted prototype-sample bucket diff for semantic correctness, regressions, stale docs wording, missing tests, and scope leaks
- Decision levels touched: none; review-only assessment of current L2 helper-local support and snapshot docs

## 1. Objective

Review the current uncommitted diff around the prototype-sample bucket task and report only actionable findings.

### Scope and assumptions

- This task is review-only except for this report.
- The review focuses on the files named by the user plus the adjacent changed files that participate in the same bucket-policy change.
- `plan/` 更新不要。
- `progress.md` 更新不要。
- `tasks.md` 更新不要。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `faq_005.md`
- `specs/examples/451-current-l2-runnable-prototype-and-not-implemented-sample-buckets.md`
- `docs/reports/0716-sample-stimuli-status-map.md`
- `docs/reports/0718-prototype-sample-actualization-first-tranche.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `samples/current-l2/README.md`
- `samples/prototype/README.md`
- `samples/not_implemented/README.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `samples/prototype/current-l2-order-handoff/*`
- `samples/prototype/current-l2-dynamic-attach-detach/*`
- `samples/not_implemented/order-handoff/*`

## 3. Actions taken

1. Read the repo-required entry documents, current snapshot docs, and the new bucket-policy spec/report pair before reviewing code.
2. Inspected the uncommitted diff for runtime, CLI, tests, samples, snapshot docs, and reports.
3. Read the full current implementations around `resolve_current_l2_source_sample_path`, line collection, and CLI host-plan resolution to check whether the new docs claims match actual behavior.
4. Ran the changed `mir-runtime` test targets to confirm the diff’s intended green path before judging regressions.

## 4. Files changed

- `docs/reports/0719-prototype-sample-bucket-diff-review.md`
- `plan/` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要

## 5. Commands run and exact outputs

- `date '+%Y-%m-%d %H:%M:%S %Z'`
  - `2026-04-17 10:54:50 JST`
- `git status --short`
  - modified files included:
    `.docs/current-l2-source-sample-authoring-policy.md`,
    `Documentation.md`,
    `crates/mir-runtime/src/current_l2.rs`,
    `crates/mir-runtime/src/current_l2_cli.rs`,
    `crates/mir-runtime/tests/current_l2_operational_cli.rs`,
    `crates/mir-runtime/tests/current_l2_source_lowering.rs`,
    `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`,
    `faq_005.md`,
    `plan/01`,
    `plan/08`,
    `plan/09`,
    `plan/11`,
    `plan/18`,
    `plan/90`,
    `progress.md`,
    `samples/current-l2/README.md`,
    `specs/00-document-map.md`,
    `tasks.md`
  - untracked paths included:
    `docs/reports/0716...`,
    `docs/reports/0718...`,
    `samples/not_implemented/`,
    `samples/prototype/`,
    `specs/examples/451...`
- `git diff --stat -- .`
  - key summary at review time:
    `18 files changed, 262 insertions(+), 25 deletions(-)` for tracked files
- `cargo test -p mir-runtime --test current_l2_source_lowering`
  - `running 15 tests`
  - `test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `cargo test -p mir-runtime --test current_l2_source_sample_runner`
  - `running 18 tests`
  - `test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `cargo test -p mir-runtime --test current_l2_operational_cli`
  - `running 5 tests`
  - `test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`

## 6. Evidence / findings

### Finding 1

- `crates/mir-runtime/src/current_l2.rs:271-285` implements `#` comments as “ignore any trimmed line whose first character is `#`”.
- The new normative/docs wording is narrower:
  - `specs/examples/451-current-l2-runnable-prototype-and-not-implemented-sample-buckets.md:24-28`
  - `.docs/current-l2-source-sample-authoring-policy.md:21-28`
  - `docs/reports/0718-prototype-sample-actualization-first-tranche.md:45-48, 97-99, 111-113`
  all describe only a sample-top / leading `#` comment convenience.
- This is a real scope leak: mid-file `#` lines are now silently accepted too, so the implementation widened the parser surface beyond the documented narrow cut. The tests only cover the leading-line happy path (`crates/mir-runtime/tests/current_l2_source_lowering.rs:24-42`) and do not lock the intended “leading only” boundary.
- Action: either narrow `collect_current_l2_source_lines` to ignore only the initial contiguous `#` block before the first nonblank program line, or widen the spec/docs and add explicit tests for non-leading comment acceptance.

### Finding 2

- `crates/mir-runtime/src/current_l2_cli.rs:63-66` now advertises `mir-current-l2 run-source-sample <sample-or-path> [--host-plan <path>] --format pretty|json` as if `--host-plan` were generally optional.
- The actual omission rule is conditional on adjacent sidecar discovery (`crates/mir-runtime/src/current_l2_cli.rs:181-190`), and the new bucket-policy wording scopes that convenience to prototype-path usage:
  - `specs/examples/451-current-l2-runnable-prototype-and-not-implemented-sample-buckets.md:14-18`
  - `samples/prototype/README.md:5-8`
  - `docs/reports/0718-prototype-sample-actualization-first-tranche.md:46-49`
- The updated test `crates/mir-runtime/tests/current_l2_operational_cli.rs:73-91` now bakes in the broad usage string even for a named current-L2 sample (`e2-try-fallback`) where omission still fails. That leaves the public shell wording overstated and makes the prototype-only convenience look like a general contract.
- Action: tighten the usage/help/error wording to describe omission as an adjacent-sidecar convenience for explicit-path cases, or intentionally promote the broader contract in spec/docs and add tests that cover non-prototype sidecar auto-resolution.

## 7. Changes in understanding

- The prototype bucket itself is wired conservatively: explicit prototype paths are kept out of the authored-fourteen inventory and the current tests cover the intended success path.
- The actionable drift is at the boundary wording level, not in the prototype sample contents or their host plans.

## 8. Open questions

- Should the `#` comment convenience remain strictly “sample-top intent line only”, or is a broader comment syntax acceptable for the current lowerer support cut?
- Should adjacent host-plan auto-resolution remain prototype-only convenience, or should the CLI contract be widened to any explicit sample path with a neighboring sidecar?

## 9. Suggested next prompt

Tighten the prototype-sample bucket diff by either narrowing `#` comment handling and CLI host-plan wording to the documented cut, or intentionally widening the docs/spec/tests to match the broader implementation behavior.
