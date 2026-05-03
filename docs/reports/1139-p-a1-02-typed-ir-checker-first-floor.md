# Report 1139 — P-A1-02 Typed IR / Checker First Floor

- Date: 2026-05-03
- Author / agent: Codex
- Scope: practical alpha-1 first checker floor over real practical package fixtures, distinct lowered checker IR, script `check` surface, snapshot/validator sync
- Decision levels touched: L1, L2
- 日本語要約: `P-A1-02` では、`samples/practical-alpha1/` の real package fixtures を入力に、`crates/mir-ast::practical_alpha1_checker` と `scripts/practical_alpha1_check.py` を使う first practical checker floor を追加した。`CHK-LIF-01..04`、`CHK-VAR-01..03`、`CHK-CUT-01`、`CHK-PKG-01/02` の explicit accepted/rejected evidence は通るが、これは non-final alpha-local checker floor に留まり、full `specs/18` typed-checking completion、runtime plan execution、run-local / run-docker、save/load、devtools export はまだ claim しない。

## Objective

`P-A1-02` を閉じ、practical alpha-1 line が front-door package input の直後に distinct lowered IR と explicit accepted/rejected evidence を持つ first checker floor を持つ状態にする。

## Scope and assumptions

- `P-A1-02` は first checker floor だけを actualize し、full `specs/18` typed-checking completion は主張しない。
- checker floor は `mir-runtime` に進めず、`mir-ast` crate 内の checker-only surface に留める。
- current practical `check` command は non-final alpha-local script surface であり、public CLI/API freeze ではない。
- runtime plan emission、run-local、run-docker、save/load、devtools export、product prototype は後続 package に残す。
- `samples/alpha/` の helper-local / sidecar / runtime-mirror carriers を practical checker report へそのまま昇格しない。

## Start state / dirty state

- start state: `main` branch, worktree clean after `P-A1-01`
- resource preflight:
  - `df -h .`: `/dev/vda2` 99G total, 65G used, 30G available
  - `free -h`: 960Mi total, 395Mi available, swap 19Gi with 1.4Gi used
- in-flight dirty state before closeout:
  - preexisting review artifacts were present and kept:
    - `docs/reports/review-2026-05-03-pa1-02-checker-surface-design-review.md`
    - `docs/reports/1138-p-a1-02-sample-validation-design-review.md`
    - `docs/reports/review-1139-p-a1-02-boundary.md`
  - `P-A1-02` implementation files under `crates/mir-ast/`, `samples/practical-alpha1/`, and `scripts/` were uncommitted

## Documents consulted

- `sub-agent-pro/alpha-1/03-decisions.md`
- `sub-agent-pro/alpha-1/04-stage-roadmap.md`
- `sub-agent-pro/alpha-1/05-theory-freeze.md`
- `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `sub-agent-pro/alpha-1/13-autonomous-package-sequence.md`
- `README.md`
- `Documentation.md`
- `AGENTS.md`
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
- `plan/39-type-system-freeze-roadmap.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `plan/41-save-load-checkpoint-roadmap.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `docs/reports/1137-p-a1-01-alpha-source-front-door.md`
- `docs/reports/review-2026-05-03-pa1-02-checker-surface-design-review.md`
- `docs/reports/1138-p-a1-02-sample-validation-design-review.md`
- `docs/reports/review-1139-p-a1-02-boundary.md`
- `docs/reports/TEMPLATE.md`

## Actions taken

1. Added `crates/mir-ast/src/practical_alpha1_checker.rs` with:
   - distinct lowered checker IR
   - checker-only report surface
   - explicit accepted/rejected evidence rows
   - first practical checker floor constants and stop lines
2. Extended `crates/mir-ast/src/practical_alpha1.rs` so practical package fixtures can carry:
   - `native`
   - namespaced `alpha_local_checker_input`
   - explicit checker family/case declarations for the first floor
3. Added Rust example `crates/mir-ast/examples/mir_practical_alpha1_check.rs` as the checker-only JSON front-door.
4. Added focused checker tests:
   - `crates/mir-ast/tests/practical_alpha1_checker.rs`
   - `crates/mir-ast/tests/support/practical_alpha1_checker_support.rs`
5. Added practical checker fixtures and expected artifacts:
   - `CHK-LIF-01..04`
   - `CHK-VAR-01..03`
   - `CHK-CUT-01`
   - `CHK-PKG-01/02`
6. Added `scripts/practical_alpha1_check.py` and unit tests as the alpha-local script `check` surface.
7. Performed RED→GREEN for the CLI normalization bug:
   - RED: `python3 scripts/practical_alpha1_check.py check-all --format json` failed because `--format` only worked as a root option before the subcommand.
   - GREEN: normalized argv now hoists root options so command-first and direct-path forms both parse.
8. Fixed expected checker artifact drift by removing stale `public_api_frozen` / `runtime_plan_generated` fields from `chk-*.expected.json` so the script compares exact current checker reports.
9. Updated `README.md`, `Documentation.md`, `specs/18`, `plan/01`, `plan/44`, `samples/README.md`, `samples/practical-alpha1/*/README.md`, `scripts/README.md`, `progress.md`, `tasks.md`, and `samples_progress.md` so the repo records `P-A1-02` as the first checker floor rather than full typed-checking completion.
10. Incorporated the four review lines already gathered for `P-A1-02` and kept their findings explicit in the closeout wording instead of treating silence as approval.

## Files changed

- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/src/practical_alpha1.rs`
- `crates/mir-ast/src/practical_alpha1_checker.rs`
- `crates/mir-ast/examples/mir_practical_alpha1_check.rs`
- `crates/mir-ast/tests/practical_alpha1_checker.rs`
- `crates/mir-ast/tests/support/practical_alpha1_checker_support.rs`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `samples/practical-alpha1/packages/chk-lif-01-raw-dangling/package.mir.json`
- `samples/practical-alpha1/packages/chk-lif-02-fallback-access-valid/package.mir.json`
- `samples/practical-alpha1/packages/chk-lif-03-inherited-chain-valid/package.mir.json`
- `samples/practical-alpha1/packages/chk-lif-04-snapshot-selected-distinction/package.mir.json`
- `samples/practical-alpha1/packages/chk-var-01-logging-layer-valid/package.mir.json`
- `samples/practical-alpha1/packages/chk-var-02-precondition-strengthening-rejected/package.mir.json`
- `samples/practical-alpha1/packages/chk-var-03-mutable-covariance-rejected/package.mir.json`
- `samples/practical-alpha1/packages/chk-cut-01-invalid-distributed-cut-rejected/package.mir.json`
- `samples/practical-alpha1/packages/chk-pkg-01-unsigned-native-rejected/package.mir.json`
- `samples/practical-alpha1/packages/chk-pkg-02-over-capability-rejected/package.mir.json`
- `samples/practical-alpha1/expected/chk-lif-01-raw-dangling.expected.json`
- `samples/practical-alpha1/expected/chk-lif-02-fallback-access-valid.expected.json`
- `samples/practical-alpha1/expected/chk-lif-03-inherited-chain-valid.expected.json`
- `samples/practical-alpha1/expected/chk-lif-04-snapshot-selected-distinction.expected.json`
- `samples/practical-alpha1/expected/chk-var-01-logging-layer-valid.expected.json`
- `samples/practical-alpha1/expected/chk-var-02-precondition-strengthening-rejected.expected.json`
- `samples/practical-alpha1/expected/chk-var-03-mutable-covariance-rejected.expected.json`
- `samples/practical-alpha1/expected/chk-cut-01-invalid-distributed-cut-rejected.expected.json`
- `samples/practical-alpha1/expected/chk-pkg-01-unsigned-native-rejected.expected.json`
- `samples/practical-alpha1/expected/chk-pkg-02-over-capability-rejected.expected.json`
- `scripts/practical_alpha1_check.py`
- `scripts/tests/test_practical_alpha1_check.py`
- `README.md`
- `Documentation.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/1139-p-a1-02-typed-ir-checker-first-floor.md`
- `docs/reports/review-2026-05-03-pa1-02-checker-surface-design-review.md`
- `docs/reports/1138-p-a1-02-sample-validation-design-review.md`
- `docs/reports/review-1139-p-a1-02-boundary.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
df -h .
free -h
git status --short
python3 -m unittest scripts.tests.test_practical_alpha1_check
cargo test -p mir-ast practical_alpha1_checker -- --nocapture
cargo test -p mir-ast
python3 scripts/practical_alpha1_check.py --format json check samples/practical-alpha1/packages/chk-lif-02-fallback-access-valid
python3 scripts/practical_alpha1_check.py --format json check-all
python3 scripts/practical_alpha1_check.py check samples/practical-alpha1/packages/chk-lif-02-fallback-access-valid --format json
python3 scripts/practical_alpha1_check.py check-all --format json
python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt
cargo fmt --check
git diff --check
mv docs/reports/2026-05-03-pa1-02-checker-surface-design-review.md docs/reports/review-2026-05-03-pa1-02-checker-surface-design-review.md
date '+%Y-%m-%d %H:%M:%S %Z'
```

## Evidence / outputs / test results

- RED step 1:
  - `python3 scripts/practical_alpha1_check.py check-all --format json` failed with `error: unrecognized arguments: --format json`.
- GREEN step 1:
  - after adding failing tests and hoisting root options in `normalize_argv()`, `python3 -m unittest scripts.tests.test_practical_alpha1_check` passed `7` tests.
- RED step 2:
  - `python3 scripts/practical_alpha1_check.py --format json check-all` reported drift for all `CHK-*` rows because expected reports still contained stale `public_api_frozen` / `runtime_plan_generated` fields.
- GREEN step 2:
  - after removing those stale fields, `python3 scripts/practical_alpha1_check.py check-all --format json` passed `10/10` rows.
- Rust checker floor:
  - `cargo test -p mir-ast practical_alpha1_checker -- --nocapture` passed `3` tests.
  - `cargo test -p mir-ast` passed all `78` crate tests, including front-door and checker suites.
- docs / validator floor:
  - `python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_validate_docs` passed `18` tests.
  - `python3 scripts/check_source_hierarchy.py` reported required/present/missing = `73/73/0`.
  - `python3 scripts/validate_docs.py` reported `Documentation scaffold looks complete.` and `Found 1141 numbered report(s).`
  - `cargo fmt --check` passed after formatting `practical_alpha1_checker.rs` and related test/support files.
  - `git diff --check` was clean.
- focused command evidence:
  - `python3 scripts/practical_alpha1_check.py check samples/practical-alpha1/packages/chk-lif-02-fallback-access-valid --format json` emitted the expected non-final checker report with explicit `accepted_obligations`.

## What changed in understanding

- The main honesty boundary for `P-A1-02` is not “checker exists” but “checker report stays distinct from runtime plan and from alpha-0 helper-local carriers.”
- A first practical checker floor can be useful before full `specs/18` typed-checking completion, as long as the report says exactly what is checked and what remains later.
- Practical script ergonomics matter even on a non-final surface; accepting the same `--format` ordering as other repo helpers removes accidental surface drift without widening semantics.

## Open questions

- Whether remaining typed-checking bullets from `specs/18` should be widened as follow-up checker packages before or alongside `P-A1-03` remains open.
- `P-A1-03` still needs the exact runtime-plan boundary: whether the checked package report is consumed directly or through one more explicit plan carrier.

## Suggested next prompt

Continue with `P-A1-03 local runtime from runtime plan` by consuming checked practical package output in a reusable local runtime path, emitting event DAG evidence, and keeping the runtime plan distinct from both the checker report and the older sample-ID keyed alpha-0 bridges.

## Plan update status

`plan/` 更新済み: `plan/01-status-at-a-glance.md` と `plan/44-practical-alpha1-roadmap.md` に `P-A1-02` closeout、first checker-floor boundary、next promoted package `P-A1-03` を反映した。

## Documentation.md update status

`Documentation.md` 更新済み: practical alpha-1 line が limited front-door に加えて distinct lowered IR を通る first checker floor を current repo state で持つこと、ただし full typed-checking completion / runtime execution ではないことを追記した。

## progress.md update status

`progress.md` 更新済み: `PA1-2` を close、`PA1-3` を promoted next へ更新し、practical alpha-1 overall reading と recent log を first checker-floor actualization に同期した。

## tasks.md update status

`tasks.md` 更新済み: `P-A1-02` を closed、`P-A1-03` を current top work item に更新し、practical package map と autonomous queue を runtime-plan/local-runtime boundary へ進めた。

## samples_progress.md update status

`samples_progress.md` 更新済み: `PA1-2` を closed に更新し、`CHK-LIF/VAR/CUT/PKG-01/02` practical checker row、`SRC-01..05` revalidated row、timestamp refresh を追加した。

## Reviewer findings and follow-up

- theory / type-system review:
  - distinct lowered checker IR は required であり、parsed package carrier と checker IR を分ける必要がある
  - follow-up: `crates/mir-ast::practical_alpha1_checker` を separate module として actualize し、report は explicit accepted/rejected evidence を持つ non-final checker-only surface にした
- runtime / transport boundary review:
  - checker surfaceは runtime plan / local run / Docker run / public CLI freeze を pre-claim してはいけない
  - follow-up: checker report に runtime fields を入れず、`public_cli_frozen: false`、`runtime_plan_emitted: false`、`run_local_claimed: false`、`run_docker_claimed: false` を kept-later marker として残した
- docs / progress consistency review:
  - `P-A1-02` は full typed-checking completion ではなく first checker floor と書くべき
  - follow-up: `README.md`、`Documentation.md`、`specs/18`、`plan/44`、dashboard docs をその wording へ同期した
- sample / validation review:
  - current `CHK-*` set は useful first floor だが full `specs/18` minimum ではなく、positive proofは empty diagnostics では足りない
  - follow-up: `accepted_obligations` / `rejected_rows` を exact expected reports に固定し、closeout wordingを first floor へ狭めた

## Skipped validations and reasons

- broad runtime / Docker / save-load tests are skipped because `P-A1-02` does not modify `mir-runtime` or transport/save-load execution surfaces.
- no `mir-runtime` crate tests were required for this package because the implementation boundary stayed inside `mir-ast`, practical fixture JSONs, script wrapper, and snapshot docs.

## Commit / push status

Closeout commit `1de9b44` (`mirrorea: close p-a1-02 practical checker floor`) was created and pushed to `origin/main`.
This report section was then updated in a docs-only follow-up commit so the report records the actual pushed closeout hash.

## Sub-agent session close status

- `Bacon` review findings were incorporated; no open reviewer task remains
- `Feynman` review findings were incorporated; no open reviewer task remains
- `Hypatia` review findings were incorporated; no open reviewer task remains
- `Popper` review findings were incorporated; no open reviewer task remains
