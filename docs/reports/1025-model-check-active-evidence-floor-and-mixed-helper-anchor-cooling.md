# 1025 — model-check active evidence floor and mixed-helper anchor cooling

## Objective

model-check late-package docs and theorem/model-check mixed helper mirrors を、current clean-near-end live floor と historical prototype anchor の split に同期し、stale current-reading / stale evidence command / stale queue wording を除去する。

## Scope and assumptions

- scope は docs / repository-memory / snapshot sync に限定する。
- `specs/` の stop line は維持し、final public theorem/model-check contract adoption は claim しない。
- theorem side current live floor は live subject `e5`、clean-near-end `05_delegated_rng_service` runtime-adjacent compare floor、committed bridge floor `samples/lean/foundations` + `samples/lean/clean-near-end` を採る。
- model-check side current live floor は clean-near-end model-check family `01_peterson_sc_pass / 02_peterson_relaxed_counterexample / 03_broken_mutex_counterexample` を採る。
- historical `p08/p09` prototype は mixed-helper asymmetry anchor としてのみ扱い、current accepted sample set へ戻さない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/06-prismcascade-positioning.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `samples/README.md`
- `scripts/README.md`
- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`
- relevant example docs:
  - `specs/examples/501-current-l2-model-check-checker-artifact-route-actual-adoption.md`
  - `specs/examples/507-current-l2-model-check-final-public-contract-reopen-threshold.md`
  - `specs/examples/517-current-l2-model-check-public-seam-compression-after-threshold-and-probe.md`
  - `specs/examples/530-current-l2-theorem-and-model-check-helper-preview-widening.md`
  - `specs/examples/532-current-l2-theorem-model-check-reopen-threshold-helper-mirror.md`

## Actions taken

1. scanned model-check late-package docs, mixed helper mirrors, `specs/11`, `specs/12`, `plan/09`, `samples_progress.md` for stale current/historical shorthand.
2. rewrote `specs/examples/501/507/517` to use current clean-near-end model-check live floor and exact current command surface, while cooling `e5/p..` wording to historical package-reading anchors.
3. rewrote `specs/examples/530/532` so theorem/model-check mixed helper mirrors explicitly distinguish:
   - theorem live floor
   - model-check live floor
   - historical `p08/p09` mixed-helper asymmetry anchors
   - historical queue memory vs current queue authority
4. synced `specs/11` and `specs/12` to the same live-floor / historical-anchor split.
5. cooled residual theorem bridge wording in `specs/11`, `specs/12`, `plan/09`, and `samples_progress.md` from `e2/e5 live bridge` to `e5` live subject + committed bridge floor, with `e2` retained only as foundation/contrast anchor.
6. investigated failed `run-source-sample p08/p09` invocations, confirmed they are historical prototype anchors outside the current accepted sample set, and converted docs to historical report evidence rather than current runnable evidence.
7. synced `progress.md`, `tasks.md`, and `samples_progress.md` to the package close state.

## Files changed

- `specs/examples/501-current-l2-model-check-checker-artifact-route-actual-adoption.md`
- `specs/examples/507-current-l2-model-check-final-public-contract-reopen-threshold.md`
- `specs/examples/517-current-l2-model-check-public-seam-compression-after-threshold-and-probe.md`
- `specs/examples/530-current-l2-theorem-and-model-check-helper-preview-widening.md`
- `specs/examples/532-current-l2-theorem-model-check-reopen-threshold-helper-mirror.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/1025-model-check-active-evidence-floor-and-mixed-helper-anchor-cooling.md`

## Evidence / outputs / test results

Passed:

- `python3 scripts/clean_near_end_samples.py run model-check --format json`
- `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
- `python3 scripts/current_l2_lean_sample_sync.py`
- `cargo test -q -p mir-runtime --test current_l2_operational_cli`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/model-check/01_peterson_sc_pass.mir --format json`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/order-handoff/05_delegated_rng_service.mir --format json`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

Investigation-only failures, not counted as passing validation:

- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample p08-dice-stale-reconnect-refresh --format json`
  - failed: missing `--host-plan` and no adjacent host plan
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample p09-dice-delegated-rng-provider-placement --format json`
  - failed: missing `--host-plan` and no adjacent host plan
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt --host-plan samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.host-plan.json --format json`
  - failed: source sample path is outside the current accepted sample set

## What changed in understanding

- mixed helper docs were not merely missing a theorem-side sync; they were also overclaiming current runnable evidence for historical `p08/p09` prototype anchors.
- the honest split is:
  - current live theorem/model-check evidence = clean-near-end + current CLI / test surface
  - historical `p08/p09` asymmetry = report-local repository memory, not current accepted-sample validation
- residual `e2/e5 live bridge` wording still survived in theorem bridge memory and sample dashboard, so the theorem-side sync was not fully closed until `specs/11`, `specs/12`, `plan/09`, and `samples_progress.md` were brought into the same pattern.

## Open questions

- `specs/12` `D-052` and adjacent current-first-line memory may still deserve a separate cooling pass if we want all theorem/model-check current-first-line wording to use the stricter live-floor split.
- reserve/index docs such as `specs/examples/612` may still need explicit labeling when they mention theorem/model-check adjacency via historical bridge memory.

## Suggested next prompt

Continue autonomous maintenance on theorem/model-check current-first-line and reserve-summary memory, especially `D-052` and `specs/examples/612`, while keeping current clean-near-end live floors and historical prototype anchors explicitly separated.

## plan/ update

updated: `plan/09-helper-stack-and-responsibility-map.md`

## progress.md update

updated

## tasks.md update

updated

## samples_progress.md update

updated

## skipped validations and reasons

- did not keep direct `run-source-sample` re-execution of historical `p08/p09` prototypes as success criteria because those prototype files are outside the current accepted sample set.
- full validation floor was not run because this package changed docs / repository memory only and did not change runtime or Rust source behavior.

## commit / push status

pending at report creation time; exact commit SHA / push result will be recorded at package close

## sub-agent session close status

- `Ptolemy` (`019dddfb-5c95-7471-87b8-0713d6b8d822`): completed and closed after findings were integrated
- reviewer `Turing` (`019dde08-db91-7771-9c7e-067af62f4be8`): timed out twice without final review payload; session closed and local diff inspection + passing validation were used as fallback evidence
