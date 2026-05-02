# Report 1114 — P-A0-14 honest CUT widening closeout

- Date: 2026-05-02 12:11 JST
- Author / agent: Codex
- Scope: close `P-A0-14` by synchronizing the honest `CUT-11` / `CUT-17` subset across runtime/checker/sample/docs/report surfaces
- Decision levels touched: L1, L2

## Objective

Close `P-A0-14` without overclaiming distributed save/load or Z-cycle repair by actualizing only the safe subset:

- `CUT-17` as a local save/load stale-membership rejection bridge
- `CUT-11` as a checker-backed Z-cycle checkpoint inadmissibility row

## Scope and assumptions

- `specs/15` remains the normative source for save/load and consistent-cut semantics.
- Fallback wording remains `guarded logical access path` wording, not lifetime extension.
- Distributed save/load, durable cut completion, `CUT-12` communication-induced checkpoint repair, and `CUT-10/16` lease/witness-store-backed load verdict split remain out of scope.
- `samples/alpha/` remains a mixed scaffold / non-public runner root, not an active parser-front-door root.

## Start state / dirty state

- Start state was dirty by design: the working tree already contained the uncommitted `P-A0-14` runtime/checker/sample changes summarized in the active session.
- `P-A0-13` was already committed and pushed as `50a07f3`.
- Resource check before the heavier reruns:
  - `df -h .`: `/dev/vda2` 99G total, 65G used, 30G available, 69% used
  - `free -h`: 960Mi RAM total, 580Mi used, 71Mi free, 379Mi available, 19Gi swap with 1.9Gi used

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `scripts/README.md`
- `specs/00..03`
- `specs/09-hotplug-and-safe-downstream-addition.md`
- `specs/13-lifetime-fallback-typing.md`
- `specs/14-contract-subtyping-and-layer-compatibility.md`
- `specs/15-cut-save-load-checkpoint.md`
- `specs/16-runtime-package-avatar-adapter.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/41-save-load-checkpoint-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `sub-agent-pro/alpha-0/` package files
- reviewer/explorer findings from `Volta` and `Anscombe`

## Actions taken

- Kept the implementation scope honest after the `Volta` / `Anscombe` review pass.
- Actualized `CUT-17` in `crates/mir-runtime/src/alpha_local_runtime.rs` and `mirrorea_alpha_local_runtime -- save-load-stale-membership`.
- Added `CUT-11` checker-floor support via `zcycle_checkpoint_inadmissible`.
- Widened `scripts/alpha_cut_save_load_samples.py` to treat `CUT-04/17` as runtime-backed rows and `CUT-11` as checker-backed.
- Narrowed the integrated E2E deferred wording away from `CUT-11/17` and onto `CUT-12` plus `CUT-10/16`.
- Synchronized roadmap memory and sample/script taxonomy docs to the new honest subset.
- Prepared snapshot docs and validation/report evidence for `P-A0-14` closeout.

## Files changed

- `crates/mir-runtime/src/alpha_local_runtime.rs`
- `crates/mir-runtime/examples/mirrorea_alpha_local_runtime.rs`
- `crates/mir-runtime/tests/alpha_cut_save_load_runtime.rs`
- `scripts/alpha_cut_save_load_checker.py`
- `scripts/alpha_cut_save_load_samples.py`
- `scripts/alpha_e2e_samples.py`
- `scripts/tests/test_alpha_cut_save_load_checker.py`
- `scripts/tests/test_alpha_cut_save_load_samples.py`
- `samples/alpha/cut-save-load/cut-04-local_save_load_valid.mir`
- `samples/alpha/cut-save-load/cut-11-zcycle_checkpoint_invalid.expected.json`
- `samples/alpha/cut-save-load/cut-17-load_does_not_resurrect_stale_membership.expected.json`
- `samples/alpha/cut-save-load/cut-17-load_does_not_resurrect_stale_membership.mir`
- `samples/alpha/e2e/e2e-06-local_save_load_continue.expected.json`
- `samples/alpha/e2e/e2e-07-distributed_inconsistent_save_negative.expected.json`
- `Documentation.md`
- `plan/01-status-at-a-glance.md`
- `plan/41-save-load-checkpoint-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `samples/README.md`
- `samples/alpha/README.md`
- `samples/alpha/cut-save-load/README.md`
- `samples/alpha/e2e/README.md`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/1114-p-a0-14-cut-safe-widening-closeout.md`

## Commands run

```bash
df -h .
free -h
git status --short
cargo test -p mirrorea-core --test runtime_substrate
cargo test -p mir-runtime --test alpha_local_runtime --test alpha_cut_save_load_runtime
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-resume
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-stale-membership
python3 scripts/alpha_cut_save_load_samples.py check-all --format json
python3 scripts/alpha_cut_save_load_samples.py closeout --format json
python3 -m unittest scripts.tests.test_alpha_cut_save_load_checker scripts.tests.test_alpha_cut_save_load_samples scripts.tests.test_alpha_e2e_samples
python3 scripts/alpha_e2e_samples.py check-all --format json
python3 scripts/alpha_e2e_samples.py closeout --format json
```

## Evidence / outputs / test results

- `cargo test -p mirrorea-core --test runtime_substrate` passed 16/16.
- `cargo test -p mir-runtime --test alpha_local_runtime --test alpha_cut_save_load_runtime` passed 5/5.
- `cargo run ... -- save-load-resume` emitted the existing `CUT-04` accepted local save/load report.
- `cargo run ... -- save-load-stale-membership` emitted the new `CUT-17` rejected stale-membership report with unchanged visible history after rejection.
- `python3 scripts/alpha_cut_save_load_samples.py check-all --format json` passed `CUT-04` and `CUT-17`, with `distributed_save_load_claimed: false` and `durable_cut_claimed: false`.
- `python3 scripts/alpha_cut_save_load_samples.py closeout --format json` reported runtime-backed rows `CUT-04/17`, checker-backed rows `CUT-05/07/08/09/11/13/14/15`, and planned-only rows `CUT-01/02/03/06/10/12/16`.
- `python3 -m unittest scripts.tests.test_alpha_cut_save_load_checker scripts.tests.test_alpha_cut_save_load_samples scripts.tests.test_alpha_e2e_samples` passed 17 tests.
- `python3 scripts/alpha_e2e_samples.py check-all --format json` passed 9 implemented rows and kept planned-only `E2E-08`.
- `python3 scripts/alpha_e2e_samples.py closeout --format json` kept `stage_e_complete: false` and `stage_f_complete: false`.
- `python3 scripts/check_source_hierarchy.py` passed with required/present/missing = `60/60/0`.
- `python3 scripts/validate_docs.py` reported `Documentation scaffold looks complete.` and `Found 1115 numbered report(s).`
- `python3 -m unittest scripts.tests.test_validate_docs` passed 11 tests.
- `cargo fmt --check` initially found Rust formatting drift in the new `CUT-17` edits; after `cargo fmt`, the affected Rust tests plus CUT/E2E runners were rerun and remained green.
- `git diff --check` was clean after the final formatting/doc gate.

## What changed in understanding

- `CUT-17` is honest now only as a narrow local save/load stale-membership rejection bridge, not as a general load-time freshness theorem.
- `CUT-11` is honest now as checker-backed checkpoint inadmissibility, not as runtime-side Z-cycle repair.
- `CUT-10/12/16` remain blocked on missing lease/witness save carriers or communication-induced checkpoint protocol state and should not be smuggled in under the same package label.
- The next promoted line is more likely to be Stage-E remaining-row widening than further CUT widening unless a new save/load substrate package is explicitly opened.

## Open questions

- Which remaining Stage-E rows can be actualized honestly from existing alpha/helper/runtime evidence without inventing a viewer API or telemetry service?
- If the CUT line is reopened later, should it first grow a lease/witness save carrier or a communication-induced checkpoint protocol skeleton?

## Suggested next prompt

Continue from `P-A0-15` by auditing the remaining Stage-E rows `VIS-02/04/05/09/12`, actualizing only the rows that can be backed by existing alpha/helper/runtime evidence, and leave the rest explicitly planned-only.

## Plan update status

`plan/` 更新済み:
`plan/01-status-at-a-glance.md`, `plan/41-save-load-checkpoint-roadmap.md`, `plan/43-alpha-e2e-roadmap.md`

## Documentation.md update status

`Documentation.md` 更新済み:
Alpha-0 summary and cut/save-load validation anchors now mention `CUT-17` runtime evidence, `CUT-11` checker evidence, and the unchanged non-claims.

## progress.md update status

`progress.md` 更新済み:
snapshot refreshed to show `P-A0-14` closed, the current next package, updated large-stage percentages, and fresh validation evidence.

## tasks.md update status

`tasks.md` 更新済み:
task map now moves the queue to `P-A0-15` and records the CUT-local blocker split for `CUT-10/12/16`.

## samples_progress.md update status

`samples_progress.md` 更新済み:
dashboard rows now reflect `A0-CUT` widening to `CUT-04/17` runtime-backed plus `CUT-11` checker-backed evidence and the new next package.

## Reviewer findings and follow-up

- `Volta` reviewer:
  - accepted `CUT-11` only as checker-backed checkpoint inadmissibility
  - accepted `CUT-17` only as narrow stale-membership rejection after local restore
  - rejected any claim that `CUT-10`, `CUT-12`, or `CUT-16` were honest now
- `Anscombe` explorer:
  - identified the minimal honest write surface
  - confirmed `CUT-17` should reuse the existing membership frontier rather than invent lease/witness carriers
  - advised against touching `E2E-06` semantics beyond the deferred-list narrowing
- `Curie` final diff reviewer:
  - did not return findings within the wait window even after one interrupt/retry
- Follow-up:
  all accepted findings were incorporated; the blocked rows remain explicitly deferred in code, sidecars, roadmap memory, and snapshot docs. For the final diff, local evidence (`cargo`/Python reruns, docs guardrails, focused diff inspection) was used because `Curie` timed out.

## Skipped validations and reasons

- No additional visualization runner rerun was needed in this package because `P-A0-14` only changed CUT/E2E wording and did not modify `scripts/alpha_visualization_samples.py` or its sidecars.
- No distributed/Docker/network/avatar cargo rerun was needed in this package because their executable surfaces were unchanged; integrated E2E sidecar parity was checked through `alpha_e2e_samples.py`.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- `Volta` (`019de691-3056-7961-a817-9a3e40cc04de`): closed after findings were incorporated
- `Anscombe` (`019de691-3210-7e52-95cc-3ec79dddf794`): closed after findings were incorporated
- `Curie` (`019de6ad-24b0-7331-bf3e-c76f8c4dd646`): closed after timeout without final findings
