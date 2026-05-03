# Report 1161 — P-A1-07 First Practical Local Save/Load Floor

- Date: 2026-05-03
- Author / agent: Codex
- Scope: `P-A1-07` first practical local save/load floor over the practical alpha-1 package line
- Decision levels touched: `specs/18` practical local save/load boundary, `plan/44` roadmap memory, distinct save-load-plan / saved-frontier / save-load-report carrier split, snapshot/dashboard wording
- 日本語要約: `P-A1-07` では distributed durable save/load や full `CUT-*` completion へ広げず、checked practical package と one exact practical local-runtime frontier を前提にする first practical local save/load floor だけを closeout した。actualize したのは `SL-A1-01` local-only roundtrip resume と `SL-A1-02` stale-membership non-resurrection に限り、saved carrier は `runtime_snapshot` だけでなく `current_owner`、`visible_history`、`pending_envelopes` を保持する distinct saved local frontier とした。`CHK-CUT-01` reuse は orphan-receive checker guard に留まり、full consistent-cut / `Z-cycle` / distributed durable save/load、stale witness / stale lease non-resurrection completion、queue/channel/transport persistence、final public save-load ABI は still later に残した。

## Objective

Close `P-A1-07` honestly after the first practical devtools floor. Add the narrowest practical local save/load command that can reuse the already-actualized practical front-door/checker/runtime line, keep the save-load carrier split explicit, synchronize roadmap/snapshot/sample dashboards, and promote `P-A1-08` as the next practical package.

## Scope and assumptions

- Scope is limited to the practical alpha-1 save/load lane in `crates/mir-ast`, `crates/mir-runtime`, `scripts/`, `samples/practical-alpha1/packages/`, `samples/practical-alpha1/expected/`, and synchronized docs.
- `P-A1-07` does not widen parser/front-door semantics, transport semantics, hot-plug lifecycle persistence, or the practical sample root into an active runnable root.
- The honest closeout is a first floor only:
  `checked package -> runtime plan`, plus `one exact practical local-runtime frontier + distinct save-load plan -> saved local frontier -> non-final save-load report`.
- Actualized rows are exactly:
  - `SL-A1-01` local-only roundtrip resume
  - `SL-A1-02` stale-membership non-resurrection after restore
- `CHK-CUT-01` is reused only as an orphan-receive checker guard. It is not accepted here as full consistent-cut, `Z-cycle`, stale witness, or stale lease completion.
- The package must not claim distributed durable save/load, queue/channel/transport persistence, Docker transport save/load, product prototype completion, active runnable-root promotion, or final public save-load CLI/API/ABI.

## Start state / dirty state

- Work resumed on `main` after `P-A1-06` had already been closed and pushed.
- The package resumed from a dirty worktree inside package scope:
  practical save/load AST/runtime files, example, script, tests, package fixtures, expected reports, and synchronized docs were already in progress.
- `git status --short` at report write showed only package-scope edits plus new package files; no unrelated user-authored edits were detected in the touched files.
- Long-running research guardrail checks were rerun before closeout:
  - root disk: `99G` total, `65G` used, `30G` available
  - memory: `960Mi` total, `719Mi` used, `88Mi` free, `307Mi` buff/cache, `240Mi` available, `19Gi` swap / `1.9Gi` used

## Documents consulted

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
- `sub-agent-pro/alpha-1/04-stage-roadmap.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `sub-agent-pro/alpha-1/13-autonomous-package-sequence.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `docs/reports/1160-p-a1-06-first-devtools-export-floor.md`

## Actions taken

1. Extended `crates/mir-ast::practical_alpha1` so practical packages can declare a narrow `alpha_local_save_load_input` surface without widening the front-door into a final public save/load grammar.
2. Added `crates/mir-ast::practical_alpha1_save_load_plan` as a distinct plan carrier with explicit save/load scope, scenario kind, base-frontier requirements, resumed envelope, post-restore membership advances, checker guard refs, retained later refs, stop lines, and limitations.
3. Added front-door and plan tests so save/load lowering is admitted only for world packages with a well-formed save/load section and rejects malformed stale-membership scenarios.
4. Added `crates/mir-runtime::practical_alpha1_save_load` plus example `mir_practical_alpha1_save_load` so the runtime path reuses one exact practical local-runtime frontier rather than serializing the local-runtime report directly.
5. Added a distinct `PracticalAlpha1SavedLocalFrontier` carrier with `runtime_snapshot`, `current_owner`, `visible_history`, `pending_envelopes`, and notes, and a distinct non-final `PracticalAlpha1SaveLoadReport`.
6. Fixed save/load execution so it:
   - checks the package and runtime plan,
   - validates the base local-runtime frontier against explicit plan requirements,
   - serializes/deserializes the saved local frontier,
   - restores a runtime shell,
   - optionally advances membership after restore,
   - reevaluates the resumed envelope,
   - emits exact event DAG and save/load report evidence.
7. Added `scripts/practical_alpha1_save_load.py` as the practical command/helper surface for `SL-A1-01/02`, with closeout metadata and explicit non-claims.
8. Added exact practical fixtures and expected save/load reports for `SL-A1-01` and `SL-A1-02`.
9. Synchronized `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `specs/18`, `plan/01`, `plan/44`, `samples/README.md`, practical sample READMEs, and `scripts/README.md`.
10. Ran validation, fixed `cargo fmt --check` drift via `cargo fmt`, reran the closeout floor, and prepared the next promoted package as `P-A1-08`.

## Files changed

- AST / front-door / plan:
  - `crates/mir-ast/src/lib.rs`
  - `crates/mir-ast/src/practical_alpha1.rs`
  - `crates/mir-ast/src/practical_alpha1_save_load_plan.rs`
  - `crates/mir-ast/tests/practical_alpha1_front_door.rs`
  - `crates/mir-ast/tests/practical_alpha1_save_load_plan.rs`
- Runtime / example:
  - `crates/mir-runtime/src/lib.rs`
  - `crates/mir-runtime/src/practical_alpha1_save_load.rs`
  - `crates/mir-runtime/examples/mir_practical_alpha1_save_load.rs`
  - `crates/mir-runtime/tests/practical_alpha1_save_load.rs`
- Script / tests:
  - `scripts/practical_alpha1_save_load.py`
  - `scripts/tests/test_practical_alpha1_save_load.py`
- Practical package fixtures:
  - `samples/practical-alpha1/packages/sl-a1-01-local-save-load-resume/package.mir.json`
  - `samples/practical-alpha1/packages/sl-a1-02-local-load-stale-membership-rejected/package.mir.json`
- Practical expected reports:
  - `samples/practical-alpha1/expected/sl-a1-01-local-save-load-resume.expected.json`
  - `samples/practical-alpha1/expected/sl-a1-02-local-load-stale-membership-rejected.expected.json`
- Docs / dashboards / repository memory:
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
  - `docs/reports/1161-p-a1-07-local-save-load-command.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M JST'
df -h .
free -h
git status --short
cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture
cargo test -p mir-ast practical_alpha1_checker -- --nocapture
cargo test -p mir-ast practical_alpha1_runtime_plan -- --nocapture
cargo test -p mir-ast --test practical_alpha1_save_load_plan -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_save_load -- --nocapture
python3 scripts/practical_alpha1_check.py run CHK-CUT-01 --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha1_save_load.py run SL-A1-01 --format json
python3 scripts/practical_alpha1_save_load.py run SL-A1-02 --format json
python3 scripts/practical_alpha1_save_load.py check-all --format json
python3 scripts/practical_alpha1_save_load.py closeout --format json
python3 -m unittest scripts.tests.test_practical_alpha1_save_load scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
cargo fmt
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- Resource / environment checks passed:
  - `date`: `2026-05-03 23:14 JST` and closeout synchronization at `2026-05-03 23:18 JST`
  - root disk: `99G` total, `65G` used, `30G` available
  - memory: `960Mi` total, `719Mi` used, `88Mi` free, `307Mi` buff/cache, `240Mi` available, `19Gi` swap / `1.9Gi` used
- AST / checker / runtime prerequisites passed:
  - `cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture`
    - passed practical front-door tests after adding the save/load surface key
  - `cargo test -p mir-ast practical_alpha1_checker -- --nocapture`
    - passed checker floor regression; existing warnings in `practical_alpha1_checker_support.rs` remained non-fatal and unchanged
  - `cargo test -p mir-ast practical_alpha1_runtime_plan -- --nocapture`
    - passed runtime-plan regression; the same existing warnings remained non-fatal and unchanged
  - `cargo test -p mir-ast --test practical_alpha1_save_load_plan -- --nocapture`
    - `4` tests passed for accepted plan lowering and malformed stale-membership rejection
  - `cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture`
    - `4` tests passed for the prerequisite exact local-runtime floor
  - `cargo test -p mir-runtime --test practical_alpha1_save_load -- --nocapture`
    - `4` tests passed for exact expected save/load reports and boundary checks
- Save/load command and helper validation passed:
  - `python3 scripts/practical_alpha1_check.py run CHK-CUT-01 --format json`
    - returned the expected orphan-receive rejection report used as a guard only
  - `python3 scripts/practical_alpha1_run_local.py check-all --format json`
    - `sample_count: 2`
    - `local_runtime_first_floor_complete: true`
    - `runtime_plan_boundary_present: true`
    - `save_load_claimed: false`
  - `python3 scripts/practical_alpha1_save_load.py run SL-A1-01 --format json`
    - emitted `surface_kind: practical_alpha1_nonfinal_save_load_report`
    - `save_load_scope: practical-alpha1-save-load-floor`
    - `save_load_plan_scope: practical-alpha1-save-load-plan-floor`
    - `runtime_plan_scope: practical-alpha1-runtime-plan-floor`
    - `saved_owner: Bob`
    - `resumed_owner: Alice`
    - `terminal_outcome: accepted`
    - `state_roundtrip_equal: true`
    - `serialized_state_bytes: 1165`
    - `checker_guard_refs: ["CHK-CUT-01"]`
    - `save_load_claimed: true`
  - `python3 scripts/practical_alpha1_save_load.py run SL-A1-02 --format json`
    - emitted the expected rejected save/load report with:
      - `terminal_outcome: rejected`
      - `reason_family: membership_freshness`
      - `saved_owner: Bob`
      - `resumed_owner: Bob`
      - `state_roundtrip_equal: true`
      - `serialized_state_bytes: 1165`
      - `checker_guard_refs: ["CHK-CUT-01"]`
  - `python3 scripts/practical_alpha1_save_load.py check-all --format json`
    - `sample_count: 2`
    - `passed: ["SL-A1-01", "SL-A1-02"]`
    - `local_save_load_first_floor_complete: true`
    - `save_load_plan_boundary_present: true`
    - `invalid_distributed_cut_guard_present: true`
    - `distributed_save_load_claimed: false`
    - `stale_witness_claimed: false`
    - `stale_lease_claimed: false`
  - `python3 scripts/practical_alpha1_save_load.py closeout --format json`
    - `implemented_rows: ["SL-A1-01", "SL-A1-02"]`
    - `checker_guard_refs: ["CHK-CUT-01"]`
    - stop lines and limitations matched the intended first-floor boundary
- Python unit / docs tests passed:
  - `python3 -m unittest scripts.tests.test_practical_alpha1_save_load scripts.tests.test_validate_docs`
    - `Ran 14 tests in 0.525s`
    - `OK`
- Docs / scaffold / formatting floor passed:
  - `python3 scripts/check_source_hierarchy.py`
    - `required: 73`
    - `present: 73`
    - `missing: 0`
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - `Found 1162 numbered report(s).`
  - first `cargo fmt --check`
    - failed on Rust formatting drift in new save/load files and tests
  - `cargo fmt`
    - normalized the Rust formatting
  - second `cargo fmt --check`
    - passed
  - `git diff --check`
    - passed

## What changed in understanding

- The honest `P-A1-07` solution is not a distributed save/load bridge and not a reused alpha-0 `CUT-*` closeout. It is a first practical local save/load floor over the practical package line.
- `SL-A1-01/02` are safe because they depend on one exact already-validated practical local-runtime frontier, then add a distinct save-load plan plus distinct saved-frontier carrier instead of collapsing into the runtime report.
- `runtime_snapshot` alone is insufficient saved evidence for this floor; `current_owner` and `visible_history` are also required to keep the resumed authority/history reading explicit.
- `CHK-CUT-01` can be reused safely only as a narrow orphan-receive checker guard. It does not justify broader consistent-cut or stale-witness/lease claims.

## Open questions

- What is the narrowest honest shape for `P-A1-08` practical product prototype so it composes the already-closed practical front-door/checker/runtime/hot-plug/transport/devtools/save-load floors without faking E2E or overclaiming a public product surface?
- When save/load is widened later, should stale witness / stale lease non-resurrection and distributed durable save/load live under the same practical save/load lane or require another distinct carrier split?

## Suggested next prompt

Continue to `P-A1-08` by adding the narrowest honest practical product prototype that composes the existing practical alpha-1 floors without promoting `samples/practical-alpha1/` into an active runnable root or claiming a final public product surface.

## Plan update status

`plan/` 更新済み: `plan/01-status-at-a-glance.md` と `plan/44-practical-alpha1-roadmap.md` に `P-A1-07` first practical local save/load floor と `P-A1-08` promoted next package を反映した。

## Documentation.md update status

`Documentation.md` 更新済み: practical alpha-1 line の current snapshot に distinct save-load-plan / saved-frontier / non-final save-load report split と non-claim boundary を反映した。

## progress.md update status

`progress.md` 更新済み: large stage、current status、validation freshness、current practical row、recent log を `P-A1-07` first floor に同期した。

## tasks.md update status

`tasks.md` 更新済み: `P-A1-07` closeout を current task-level status と package map に反映し、`P-A1-08` を promoted next package として固定した。

## samples_progress.md update status

`samples_progress.md` 更新済み: practical alpha-1 stage summary、`SL-A1-01/02` save/load row、recent validation row、snapshot freshness timestamp を反映した。

## Reviewer findings and follow-up

- `Herschel`:
  savepoint には `runtime_snapshot` だけでなく `current_owner` と `visible_history` が必要であり、`CHK-CUT-01` は orphan-receive guard reuse に留めるべき、という finding を採用した。
- `Noether`:
  `PracticalAlpha1LocalRuntimeReport` の直列化は避け、distinct saved carrier type と front-door/runtime-plan/save-load-plan/tests の分離を保つべき、という finding を採用した。
- `Euler`:
  `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`samples_progress.md`、`specs/18`、`plan/44`、practical sample READMEs、`scripts/README.md` の non-claim wording synchronization を要求し、その follow-up を反映した。

## Skipped validations and reasons

- No Docker / transport runtime command was added in `P-A1-07`, so practical transport runtime execution beyond the already-actualized `P-A1-05` floor was not widened here.
- No hot-plug or product prototype validations were added in this package, because `P-A1-07` is intentionally restricted to the first practical local save/load floor.
- Existing warnings in `crates/mir-ast/tests/support/practical_alpha1_checker_support.rs` were not addressed here because they predated the package, did not fail the validation floor, and were unrelated to the save/load boundary.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- `Herschel` completed the semantics/runtime review and is pending closure
- `Noether` completed the carrier/test review and is pending closure
- `Euler` completed the docs/snapshot review and is pending closure
