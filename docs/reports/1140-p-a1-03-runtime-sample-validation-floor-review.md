# Report 1140 — P-A1-03 Runtime Sample Validation Floor Review

- Date: 2026-05-03
- Author / agent: Codex
- Scope: review `P-A1-03 local runtime from runtime plan` from a sample/validation perspective, using only the relevant alpha-1 handoff docs, report 1139, current practical alpha-1 sample root, and current alpha local-runtime evidence commands
- Decision levels touched: L1, L2

## Objective

Determine the minimum honest practical sample set and validation floor for `P-A1-03 local runtime from runtime plan` immediately after `P-A1-02`, with concrete findings on sample IDs, expected outputs, and must-have negative cases.

## Scope and assumptions

- This is a review-only task. No normative spec text or implementation behavior is changed here.
- The review is constrained to:
  - relevant `sub-agent-pro/alpha-1/*.md` handoff docs
  - `docs/reports/1139-p-a1-02-typed-ir-checker-first-floor.md`
  - `samples/practical-alpha1/`
  - current alpha local-runtime evidence docs/commands
- The target question is the narrow floor immediately after `P-A1-02`, not full practical alpha-1 runtime completion.
- If a recommendation goes beyond current text, it is presented as review guidance rather than as decided spec text.

## Start state / dirty state

- start state: `main` branch
- initial worktree check during review showed no staged output from this task yet
- final validation before commit showed one unrelated preexisting modified file and it was left untouched:
  - `docs/reports/1142-review-p-a1-03-runtime-transport-boundary.md`
- this task adds one new report file only

## Documents consulted

- `sub-agent-pro/alpha-1/00-index.md`
- `sub-agent-pro/alpha-1/01-current-state-gap.md`
- `sub-agent-pro/alpha-1/02-practical-alpha1-definition.md`
- `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `sub-agent-pro/alpha-1/13-autonomous-package-sequence.md`
- `docs/reports/1139-p-a1-02-typed-ir-checker-first-floor.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `samples/alpha/local-runtime/README.md`
- `samples/alpha/local-runtime/lr-01-local_sugoroku_roll_publish_handoff.expected.json`
- `samples/alpha/local-runtime/lr-02-stale_membership_rejected.expected.json`
- `scripts/practical_alpha1_check.py`
- `scripts/alpha_local_runtime_samples.py`
- `scripts/README.md`

## Actions taken

1. Read the alpha-1 handoff sections that define `P-A1-03`, the runtime sample matrix rows, and the planned validation floor.
2. Read report 1139 to lock the exact `P-A1-02` honesty boundary and the explicit kept-later markers for runtime work.
3. Inspected the current practical alpha-1 sample root to confirm which `SRC-*` and `CHK-*` fixtures exist and which runtime fixtures do not yet exist.
4. Inspected the current alpha local-runtime evidence runner and expected sidecars to identify the smallest currently-proven runtime assertions.
5. Ran the current checker/front-door/runtime evidence commands to confirm the present executable floor and its outputs.
6. Derived the minimum honest `P-A1-03` practical floor by separating:
   - what can be reused as shape or evidence pattern from `LR-01/02`
   - what must become new practical `RUN-*` fixtures under `samples/practical-alpha1/`
   - what must remain explicitly later than `P-A1-03`

## Files changed

- `docs/reports/1140-p-a1-03-runtime-sample-validation-floor-review.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
rg --files sub-agent-pro docs/reports samples scripts plan . | rg 'alpha-1|alpha1|A1|1139|practical-alpha1|local runtime|runtime plan|evidence'
rg -n "P-A1-03|P-A1-02|runtime plan|local runtime|practical-alpha1|alpha local runtime|evidence command" sub-agent-pro docs/reports samples scripts plan .
rg -n "local runtime|run_local|alpha local runtime|lr-01|lr-02|practical_alpha1_run_local|runtime runner" samples/alpha scripts samples/practical-alpha1 docs/reports/1139-p-a1-02-typed-ir-checker-first-floor.md sub-agent-pro/alpha-1
sed -n '1,220p' sub-agent-pro/alpha-1/00-index.md
sed -n '1,220p' sub-agent-pro/alpha-1/01-current-state-gap.md
sed -n '1,240p' sub-agent-pro/alpha-1/02-practical-alpha1-definition.md
sed -n '1,220p' sub-agent-pro/alpha-1/06-toolchain-architecture.md
sed -n '1,220p' sub-agent-pro/alpha-1/08-sample-matrix.md
sed -n '1,220p' sub-agent-pro/alpha-1/09-validation-plan.md
sed -n '1,220p' sub-agent-pro/alpha-1/13-autonomous-package-sequence.md
sed -n '1,280p' docs/reports/1139-p-a1-02-typed-ir-checker-first-floor.md
sed -n '1,220p' samples/practical-alpha1/README.md
sed -n '1,220p' samples/practical-alpha1/packages/README.md
sed -n '1,220p' samples/practical-alpha1/expected/README.md
find samples/practical-alpha1/packages -mindepth 1 -maxdepth 1 -type d | sort
sed -n '1,240p' samples/alpha/local-runtime/README.md
sed -n '1,260p' scripts/alpha_local_runtime_samples.py
sed -n '1,160p' scripts/README.md
python3 scripts/alpha_local_runtime_samples.py --help
sed -n '1,220p' samples/practical-alpha1/packages/src-01-minimal-world/package.mir.json
sed -n '1,220p' samples/practical-alpha1/packages/src-03-layer-debug/package.mir.json
sed -n '1,220p' samples/practical-alpha1/packages/chk-lif-02-fallback-access-valid/package.mir.json
sed -n '1,220p' samples/practical-alpha1/packages/chk-pkg-02-over-capability-rejected/package.mir.json
sed -n '1,220p' samples/practical-alpha1/expected/src-01-minimal-world.expected.json
sed -n '1,260p' samples/practical-alpha1/expected/chk-lif-02-fallback-access-valid.expected.json
sed -n '1,260p' samples/alpha/local-runtime/lr-01-local_sugoroku_roll_publish_handoff.expected.json
sed -n '1,260p' samples/alpha/local-runtime/lr-02-stale_membership_rejected.expected.json
python3 scripts/alpha_local_runtime_samples.py check-all --format json
python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json
python3 scripts/practical_alpha1_check.py check-all --format json
cargo test -p mir-ast practical_alpha1_front_door -- --nocapture
git status --short
date '+%Y-%m-%d %H:%M:%S %Z'
```

## Evidence / outputs / test results

- `python3 scripts/practical_alpha1_check.py check-all --format json` returned:
  - `sample_count: 10`
  - all `CHK-*` rows passed
  - `runtime_plan_emitted: false`
  - `run_local_claimed: false`
  - `run_docker_claimed: false`
- `python3 scripts/alpha_local_runtime_samples.py check-all --format json` returned:
  - `sample_count: 2`
  - `passed: ["LR-01", "LR-02"]`
  - `active_root_promoted: false`
  - `parser_runtime_bridge_claimed: false`
- `python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json` returned:
  - Stage B closeout depends on `LR-01/02` plus `CUT-04/17`
  - `distributed_save_load_claimed: false`
  - `parser_runtime_bridge_claimed: false`
- `cargo test -p mir-ast practical_alpha1_front_door -- --nocapture` passed `11` practical front-door tests.
- The current practical sample root still contains only:
  - `SRC-01..05`
  - `CHK-LIF-01..04`
  - `CHK-VAR-01..03`
  - `CHK-CUT-01`
  - `CHK-PKG-01/02`
  and no `RUN-*` fixtures.

## What changed in understanding

- The narrowest honest `P-A1-03` floor is not “wire the existing alpha runtime runner to practical packages.” That would preserve the old scenario-keyed bridge that `P-A1-03` is explicitly supposed to replace.
- The real review boundary is not just whether something runs locally, but whether the runtime consumes a checked practical carrier distinct from:
  - raw package loading
  - checker-only reports
  - alpha sample-ID/scenario keyed bridges
- The smallest reusable runtime evidence already proven by alpha local-runtime is:
  - one accepted dispatch with event DAG export
  - one stale-membership rejection before state mutation
  This is the right shape for the first practical runtime floor, but not the right sample root or command surface.

## Open questions

- Whether `P-A1-03` should introduce an explicit runtime-plan artifact/schema, or whether `run-local` should internally lower checked package output into a non-public plan carrier while still exposing that boundary in the JSON report, remains open.
- Whether `RUN-03 missing capability reject` and `RUN-04 missing witness reject` must both land in `P-A1-03`, or whether one or both can remain for `P-A1-04`, should be decided explicitly. My current recommendation is below.

## Suggested next prompt

Implement `P-A1-03` with new `samples/practical-alpha1/packages/run-*` fixtures, a `practical_alpha1_run_local` command that consumes checked practical input through an explicit runtime-plan boundary, and expected JSON artifacts for `RUN-01`/`RUN-02` plus at least one runtime-boundary negative.

## Plan update status

`plan/` 更新不要: this task is a review-only recommendation and does not change repository memory or normative package status.

## Documentation.md update status

`Documentation.md` 更新不要: no repo status claim changed in this review-only task.

## progress.md update status

`progress.md` 更新不要: this task did not advance or close a package; it only reviews the next package floor.

## tasks.md update status

`tasks.md` 更新不要: no new blocker or promoted line was introduced beyond the already-current `P-A1-03`.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample implementation or runnable validation state changed.

## Reviewer findings and follow-up

- Finding 1:
  reusing `samples/alpha/local-runtime` or `scripts/alpha_local_runtime_samples.py` as the practical `P-A1-03` floor would be dishonest. The current alpha runner is explicitly scenario-keyed (`local-sugoroku`, `stale-membership`) and explicitly marked as non-parser/runtime front-door evidence, while `P-A1-03` requires runtime consumption of checked plan rather than sample ID. New practical `RUN-*` fixtures under `samples/practical-alpha1/` are required.
- Finding 2:
  the minimum honest practical sample set immediately after `P-A1-02` is at least two runtime rows:
  - `RUN-01`: positive local runtime from practical checked plan with event DAG export
  - `RUN-02`: stale membership rejection before state mutation
  `RUN-01` and `RUN-02` are the smallest set that covers the explicit `P-A1-03` actions (`run local world`, `export event DAG`) plus the one negative row already proven and already called out in the runtime matrix.
- Finding 3:
  `RUN-01` should not reuse `SRC-01` or `SRC-03` as-is. `SRC-01` only proves minimal package loading, and `SRC-03` only proves a requested debug layer is present in the package shape. Neither gives enough runtime semantics to justify accepted dispatch plus event-DAG evidence. `P-A1-03` needs dedicated `run-*` fixtures or a new practical Sugoroku-style world package.
- Finding 4:
  the per-sample expected output floor for `RUN-01` should be the current `LR-01` runtime contract translated to the practical command surface:
  - explicit `terminal_outcome: "accepted"`
  - exactly one dispatch record
  - non-empty visible history
  - event DAG relations including at least `publication_order`, `witness_order`, and `handoff_order`
  - explicit kept-later markers for Docker, save/load, final public ABI, and runtime package/avatar admission
  A plain “command exits 0” smoke check would be below the current honesty line.
- Finding 5:
  the per-sample expected output floor for `RUN-02` should be the current `LR-02` runtime contract translated to the practical command surface:
  - explicit `terminal_outcome: "rejected"`
  - `reason_family: "membership_freshness"`
  - dispatch outcome `rejected_stale_membership`
  - `reason_refs` containing `membership_epoch_drift`
  - empty visible history after rejection
  This negative is already the one runtime rejection row with executable evidence and should not be dropped in the practical promotion.
- Finding 6:
  the minimum command floor should remain row-keyed plus aggregate, not aggregate-only:
  - focused front-door regression: existing `cargo test -p mir-ast practical_alpha1_front_door -- --nocapture`
  - focused checker regression: existing `python3 scripts/practical_alpha1_check.py check-all --format json`
  - new focused runtime rows: `python3 scripts/practical_alpha1_run_local.py run RUN-01 --format json` and `... run RUN-02 --format json` or equivalent path-based invocations
  - aggregate runtime regression: `python3 scripts/practical_alpha1_run_local.py check-all --format json`
  - focused unit/integration tests for the runtime command surface
  One product-like smoke command alone would not localize whether the breakage is in plan lowering, runtime dispatch, or event export.
- Finding 7:
  one more runtime-boundary negative should be present if `P-A1-03` wants to claim “from runtime plan” rather than just “local run exists.” My recommendation is:
  - either make `RUN-03` = missing capability reject
  - or add an explicit boundary negative that refuses unchecked/checker-rejected input before runtime starts
  If neither lands in `P-A1-03`, the package should explicitly keep capability/witness/runtime-admission handling as later and not imply those lanes are already enforced.

## Skipped validations and reasons

- No `mir-runtime` practical runtime command was run because `practical_alpha1_run_local.py` does not exist yet in the current repo state.
- No Docker/runtime transport validations were run because this review targets the immediate post-`P-A1-02` local-runtime floor only.
- No docs/dashboard update validators were rerun because this task only adds a review report and does not change the tracked snapshot docs.

## Commit / push status

Initial report commit: `5d902ed` (`mirrorea: add p-a1-03 runtime floor review`).
This report then received metadata-only follow-up updates and the resulting commits were pushed to `origin/main`.

## Sub-agent session close status

- No external reviewer/sub-agent was invoked for this task.
