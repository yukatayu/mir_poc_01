# Report 0731 — runnable validation status and queue refresh

- Date: 2026-04-17T11:47:27.449546Z
- Author / agent: Codex
- Scope: review `plan/` and `docs/` for stale lane/sample-count readings, rewrite `progress.md` and `tasks.md` as fresh snapshots, and re-check the current runnable-comparison floor
- Decision levels touched: none; snapshot / repository-memory / condensed-summary refresh only

## 1. Objective

Refresh the repo snapshot so it answers, without stale wording:

- how far implementation- and execution-based comparison has actually progressed,
- whether corrected runnable versions of the current mapped feature families already exist,
- what remains before the next practical milestone,
- and whether that milestone is still self-driven.

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `samples/prototype/README.md`
- `docs/reports/0716-sample-stimuli-status-map.md`
- `docs/reports/0718-prototype-sample-actualization-first-tranche.md`
- `docs/reports/0725-typed-theorem-model-check-corrected-prototype-tranche.md`
- `docs/reports/0726-order-handoff-corrected-prototype-third-tranche.md`
- `docs/reports/0730-faq006-theory-line-reintegration-and-queue-reconstruction.md`

## 3. Actions taken

1. Re-read the repo entry docs, current snapshot docs, and progress/task axis guide to re-establish the required reading order and snapshot-writing rules.
2. Checked resource status with `df -h .` and `free -h` before continuing the longer review / verification pass.
3. Re-audited `plan/` and `docs/research_abstract/` for stale phrases such as old `Macro 0〜4 closeout fixed`, `Macro 5 boundary / pilot / framing closeout fixed`, `Macro 6/7 mixed-gate boundary fixed`, and old sample-count wording.
4. Re-checked sample-bucket and prototype placement docs against the recent corrected-prototype reports (`p06` / `p07` / `p08`) and the current fixture / sample plan.
5. Ran fresh executable validation on the current source-sample runner, operational CLI, and authored-sample regression helper.
6. Rewrote `progress.md` and `tasks.md` so they now explicitly distinguish:
   - authored current-l2 runnable floor,
   - corrected prototype runnable floor,
   - rough-stimulus preservation bucket,
   - and the remaining self-driven queue.
7. Updated relevant plan / condensed-summary docs and traceability so the stale lane/sample-count readings no longer conflict with the current queue.

## 4. Files changed

- `docs/reports/0731-runnable-validation-status-and-queue-refresh.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/10-roadmap-overall.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`

## 5. Commands run and exact outputs

- `date '+%Y-%m-%d %H:%M:%S %Z'`
  - `2026-04-17 20:47:27 JST`
- `df -h .`
  - `/dev/vda2 99G size / 78G used / 17G avail / 83%`
- `free -h`
  - `Mem: 960Mi total / 666Mi used / 86Mi free / 293Mi available`
- `cargo test -p mir-runtime --test current_l2_source_sample_runner`
  - `running 22 tests`
  - `test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `cargo test -p mir-runtime --test current_l2_operational_cli`
  - `running 12 tests`
  - `test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `python3 scripts/current_l2_source_sample_regression.py inventory`
  - reported all 16 authored current-l2 rows as `present`
- `python3 scripts/current_l2_source_sample_regression.py regression`
  - `[1/20]` through `[20/20]` all succeeded
  - final line: `all regression commands passed`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete. Found 730 numbered report(s).`
- `git diff --check`
  - no output

Additional read-only inspection commands were used to locate stale phrases and review current file contents:

- `rg -n ... plan docs Documentation.md progress.md tasks.md`
- `sed -n ...` over `plan/`, `docs/research_abstract/`, `progress.md`, `tasks.md`, and recent reports
- `rg --files samples/current-l2 samples/prototype samples/not_implemented | sort`

## 6. Evidence / findings

- The repo’s runnable-comparison floor is materially further along than the stale `0 package` snapshot reading suggested.
- Fresh execution evidence confirms:
  - authored current-l2 corpus: 16 source-authored rows are present and green through inventory / regression,
  - source-sample runner: green for named authored rows and explicit prototype paths,
  - operational CLI: green for runtime, static-stop, typed-bridge, and order/handoff prototype previews.
- The practical status split is now:
  - `samples/current-l2/` = source-backed authored current subset,
  - `samples/prototype/` = corrected runnable prototypes for currently mapped comparison families,
  - `samples/not_implemented/` = exact rough stimuli preserved on purpose outside the runnable path.
- For currently mapped families, the stage “corrected version runs, even if via interpreter / helper-local runner” is largely already reached.
- The near-term remaining work is therefore mainly theory-lab integration:
  - `Package B` typed / theorem / model-check current first-line hardening,
  - `Package C` order / handoff current first-line hardening,
  - `Package D` syntax / modality comparison hardening,
  - `Package E` snapshot reintegration.
- The main stale items found in this pass were:
  - `plan/00-index.md`
  - `plan/10-roadmap-overall.md`
  - `docs/research_abstract/README.md`
  - `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
  - `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
  These still used old lane readings or old sample counts.

## 7. Changes in understanding

- The corrected-prototype tranche close should be read as “runnable comparison floor exists”, not as “theory queue is empty”.
- The next practical milestone is no longer “make the examples runnable from scratch” for the mapped families; it is “integrate the current first line / retained alternatives / stop line / user-spec gate around the already runnable evidence”.
- The repo remains self-driven up to that integration milestone. Mixed gate begins later, at actual adoption / concrete binding / final public contract / final shared-space policy profile.

## 8. Open questions

- If the user wants a one-to-one schedule for every previously suggested feature/example, the next useful artifact would be a dedicated mapping table from each prior stimulus to:
  - authored current-l2 row,
  - corrected prototype,
  - or still-unmapped family.
- If there are prior examples outside the currently mapped families, those may still need a narrow prototype package of their own.
- The mixed-gate items remain unchanged:
  - stronger typed-surface actual adoption,
  - theorem discharge/public-contract concretization,
  - settled property language/concrete model-check seam,
  - shared-space final fairness/replay operational profile,
  - packaging / installed-binary success criteria.

## 9. Suggested next prompt

Using the refreshed snapshot, estimate the remaining self-driven work in two tiers:

1. “corrected runnable analogue exists for each currently mapped feature family”
2. “current first line / retained alternatives / stop line / user-spec gate are fully integrated around that runnable evidence”

Then list any prior user-proposed examples that still lack a corrected runnable analogue.
