# Report 1159 — P-A1-05 Practical Transport E2E Closeout

- Date: 2026-05-03
- Author / agent: Codex
- Scope: `P-A1-05` practical transport E2E over the practical alpha-1 package line
- Decision levels touched: `specs/18` practical transport boundary, `plan/44` roadmap memory, practical transport plan/report carrier shape, snapshot/dashboard wording
- 日本語要約: `P-A1-05` では blocker で止まっていた practical transport line を、distinct transport-plan/report carrier と `TR-A1-01..07` exact expected reports で closeout した。local TCP / Docker Compose TCP accepted path に加え、stale-membership / missing-capability / missing-witness reject、observer-safe route trace、auth-lane separation を同じ practical package root 上で actualize したが、これは WAN/federation、save/load、devtools、product prototype、final public transport ABI ではない。

## Objective

Close `P-A1-05` honestly after the earlier scope-blocker review. Actualize a narrow but practical transport floor over the existing practical package line, keep `checked package -> transport plan -> non-final transport report` explicit, synchronize roadmap/snapshot/sample dashboards, and promote `P-A1-06` as the next practical package.

## Scope and assumptions

- Scope is limited to the practical alpha-1 transport lane in `samples/practical-alpha1/`, `crates/mir-ast`, `crates/mir-runtime`, `scripts/`, and synchronized docs.
- `P-A1-05` adopts the widened row set rather than reusing `RUN-*`, `HP-*`, or alpha-0 `NET-*` evidence.
- Actualized rows are exactly:
  - `TR-A1-01` local TCP accepted
  - `TR-A1-02` Docker Compose TCP accepted
  - `TR-A1-03` stale-membership rejected
  - `TR-A1-04` missing-capability rejected
  - `TR-A1-05` missing-witness rejected
  - `TR-A1-06` observer-safe route trace
  - `TR-A1-07` auth-lane preserved
- The package must keep transport/auth/membership/capability/witness lanes distinct.
- The package must not claim WAN/federation, local save/load, devtools export, product prototype, active runnable-root promotion, or final public transport ABI.

## Start state / dirty state

- Work resumed on `main` after blocker report `1158-p-a1-05-transport-scope-blocker.md` had already been pushed.
- The package resumed from a dirty worktree inside package scope:
  `crates/mir-ast`, `crates/mir-runtime`, `samples/practical-alpha1/`, `scripts/`, and synchronized docs were already modified by the in-progress transport implementation.
- `git status --short` at report write showed only package-scope edits plus new package files; no unrelated user-authored edits were detected in the touched files.
- Long-running research guardrail checks were run before/while resuming implementation:
  - root disk: `99G` total, `65G` used, `30G` available
  - memory: `960Mi` total, `658Mi` used, `101Mi` free, `356Mi` buff/cache, `1.7Gi` swap used
  - `/mnt/mirrorea-work` mount was present and usable

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
- `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `sub-agent-pro/alpha-1/13-autonomous-package-sequence.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `samples/practical-alpha1/docker/README.md`
- `scripts/README.md`
- `docs/reports/1155-p-a1-04c-practical-detach-minimal-contract.md`
- `docs/reports/1158-p-a1-05-transport-scope-blocker.md`
- `docs/reports/review-2026-05-03-pa1-05-transport-scope-review.md`

## Actions taken

1. Extended the practical front-door package carrier with `alpha_local_transport_input` and admitted it explicitly in the non-final practical manifest surface.
2. Added a distinct transport-plan carrier in `mir-ast` that requires:
   checked package input, concrete transport surface, concrete request envelope, destination validation, exact capability alignment, and non-empty witness requirements.
3. Added a distinct non-final practical transport runtime/report lane in `mir-runtime` for:
   local TCP loopback execution and Docker Compose TCP execution.
4. Kept the wire request narrow:
   required capabilities / required witness refs / auth binding requirements remain authoritative in the shared transport plan rather than being duplicated as a second public contract.
5. Added practical fixtures and exact expected reports for `TR-A1-01..07`.
6. Added a dedicated Python script surface `scripts/practical_alpha1_transport.py` with `check-all` and `closeout`.
7. Added tests for front-door manifest admission, transport-plan lowering, runtime/report equality, script-level closeout, scope non-claims, and Docker/local transport parity.
8. Synchronized `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `specs/18`, `plan/01`, `plan/44`, `samples/README.md`, practical sample READMEs, and `scripts/README.md`.

## Files changed

- Code / tests:
  - `crates/mir-ast/src/lib.rs`
  - `crates/mir-ast/src/practical_alpha1.rs`
  - `crates/mir-ast/src/practical_alpha1_transport_plan.rs`
  - `crates/mir-ast/tests/practical_alpha1_front_door.rs`
  - `crates/mir-ast/tests/practical_alpha1_transport_plan.rs`
  - `crates/mir-runtime/src/lib.rs`
  - `crates/mir-runtime/src/practical_alpha1_transport.rs`
  - `crates/mir-runtime/examples/mir_practical_alpha1_transport.rs`
  - `crates/mir-runtime/tests/practical_alpha1_transport.rs`
  - `scripts/practical_alpha1_transport.py`
  - `scripts/tests/test_practical_alpha1_transport.py`
- Practical fixtures / expected reports:
  - `samples/practical-alpha1/packages/tr-a1-01-local-tcp-accepted/package.mir.json`
  - `samples/practical-alpha1/packages/tr-a1-02-docker-two-node-accepted/package.mir.json`
  - `samples/practical-alpha1/packages/tr-a1-03-stale-membership-rejected/package.mir.json`
  - `samples/practical-alpha1/packages/tr-a1-04-missing-capability-rejected/package.mir.json`
  - `samples/practical-alpha1/packages/tr-a1-05-missing-witness-rejected/package.mir.json`
  - `samples/practical-alpha1/packages/tr-a1-06-observer-safe-route-trace/package.mir.json`
  - `samples/practical-alpha1/packages/tr-a1-07-auth-lane-preserved/package.mir.json`
  - `samples/practical-alpha1/expected/tr-a1-01-local-tcp-accepted.expected.json`
  - `samples/practical-alpha1/expected/tr-a1-02-docker-two-node-accepted.expected.json`
  - `samples/practical-alpha1/expected/tr-a1-03-stale-membership-rejected.expected.json`
  - `samples/practical-alpha1/expected/tr-a1-04-missing-capability-rejected.expected.json`
  - `samples/practical-alpha1/expected/tr-a1-05-missing-witness-rejected.expected.json`
  - `samples/practical-alpha1/expected/tr-a1-06-observer-safe-route-trace.expected.json`
  - `samples/practical-alpha1/expected/tr-a1-07-auth-lane-preserved.expected.json`
  - `samples/practical-alpha1/docker/docker-compose.practical-alpha1.yml`
- Docs / dashboards / repository memory:
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
  - `samples/practical-alpha1/docker/README.md`
  - `scripts/README.md`
  - `docs/reports/1159-p-a1-05-practical-transport-e2e-closeout.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
df -h .
free -h
lsblk -f
findmnt
du -sh . target .git .cargo .lake 2>/dev/null
git status --short
docker --version
docker compose version
cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture
cargo test -p mir-ast practical_alpha1_checker -- --nocapture
cargo test -p mir-ast --test practical_alpha1_runtime_plan -- --nocapture
cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture
cargo test -p mir-ast --test practical_alpha1_transport_plan -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture
cargo test -p mir-runtime --test hotplug_runtime_skeleton -- --nocapture
cargo test -p mir-runtime --test alpha_layer_insertion_runtime
cargo test -p mir-runtime --test practical_alpha1_transport -- --nocapture
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_transport.py check-all --format json
python3 scripts/practical_alpha1_transport.py closeout --format json
python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_attach scripts.tests.test_practical_alpha1_transport scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt
cargo fmt --check
git diff --check
date '+%Y-%m-%d %H:%M JST'
```

## Evidence / outputs / test results

- Resource / environment checks passed:
  - `docker --version`: `Docker version 29.3.0`
  - `docker compose version`: `v5.1.0`
  - mount / capacity checks confirmed `/mnt/mirrorea-work` available
- Rust validation passed:
  - `practical_alpha1_front_door`: 11 passed
  - `practical_alpha1_checker`: 3 passed
  - `practical_alpha1_runtime_plan`: 5 passed
  - `practical_alpha1_hotplug_plan`: 8 passed
  - `practical_alpha1_transport_plan`: 5 passed
  - `practical_alpha1_local_runtime`: 4 passed
  - `practical_alpha1_hotplug`: 15 passed
  - `hotplug_runtime_skeleton`: 8 passed
  - `alpha_layer_insertion_runtime`: 6 passed
  - `practical_alpha1_transport`: 8 passed
- Python helper validation passed:
  - `python3 scripts/practical_alpha1_check.py check-all --format json`
    - `sample_count: 10`
    - `first_checker_floor_complete: true`
  - `python3 scripts/practical_alpha1_run_local.py check-all --format json`
    - `sample_count: 2`
    - `local_runtime_first_floor_complete: true`
    - `runtime_plan_boundary_present: true`
  - `python3 scripts/practical_alpha1_attach.py check-all --format json`
    - `sample_count: 9`
    - `package_hotplug_first_floor_complete: true`
    - `object_attach_seam_present: true`
    - `detach_minimal_contract_complete: true`
  - `python3 scripts/practical_alpha1_transport.py check-all --format json`
    - `sample_count: 7`
    - `transport_first_floor_complete: true`
    - `transport_plan_boundary_present: true`
    - `docker_row_complete: true`
    - `stale_membership_negative_complete: true`
    - `missing_capability_negative_complete: true`
    - `missing_witness_negative_complete: true`
    - `route_trace_complete: true`
    - `auth_lane_complete: true`
    - `stage_pa1_5_complete: true`
    - `wan_federation_claimed: false`
    - `save_load_claimed: false`
    - `final_public_transport_abi_claimed: false`
  - `python3 scripts/practical_alpha1_transport.py closeout --format json`
    - `implemented_rows: TR-A1-01..07`
    - `transport_plan_boundary_present: true`
    - `stage_pa1_5_complete: true`
- Python unit tests passed:
  - `Ran 40 tests in 0.057s`
- Docs / scaffold floor passed:
  - `python3 scripts/check_source_hierarchy.py`: `required: 73`, `present: 73`, `missing: 0`
  - `python3 scripts/validate_docs.py`: `Documentation scaffold looks complete. Found 1160 numbered report(s).`
  - `cargo fmt --check`: passed after one `cargo fmt` alignment pass on the new transport files
  - `git diff --check`
- Known warnings:
  - `cargo test -p mir-ast practical_alpha1_checker -- --nocapture` and `cargo test -p mir-ast --test practical_alpha1_runtime_plan -- --nocapture` emitted existing `dead_code` warnings in `crates/mir-ast/tests/support/practical_alpha1_checker_support.rs`; no new failures or semantic regressions were observed.

## What changed in understanding

- The honest `P-A1-05` solution was not to invent a parser/runtime bridge or borrow old alpha/network evidence. It was to add one more distinct carrier:
  `checked package -> transport plan -> non-final transport report`.
- `Option A` is now concretely justified: missing-capability and missing-witness negatives can be practical transport rows without collapsing transport semantics into hot-plug/runtime-plan semantics.
- Docker/local parity is viable at this floor if the Compose surface is kept narrow and exact-report-based rather than presented as a general product runtime.
- Route trace and auth-lane proof belong in the exact transport report, not in helper-only side channels.

## Open questions

- How narrow can `P-A1-06` keep its export schema while still serving event DAG, route trace, membership timeline, hot-plug lifecycle, and fallback degradation without inventing a fake integrated viewer?
- Should `P-A1-06` consume the existing checker/runtime/hot-plug/transport reports directly, or introduce one explicit practical devtools export bundle carrier?
- How much of `P-A1-07` local save/load can reuse existing alpha-0 `CUT-04/17` evidence versus requiring a new practical save/load report surface?

## Suggested next prompt

Close `P-A1-06` by defining the narrowest honest practical devtools/viewer export surface over the existing practical checker/runtime/hot-plug/transport reports, without claiming final public viewer API or product prototype completion.

## Plan update status

`plan/` 更新済み:
`plan/01-status-at-a-glance.md` と `plan/44-practical-alpha1-roadmap.md` を更新し、`P-A1-05` closeout、`TR-A1-01..07` actualization、distinct transport carrier split、`P-A1-06` promoted next を反映した。

## Documentation.md update status

`Documentation.md` 更新済み:
practical alpha-1 section を transport floor まで同期し、`TR-A1-01..07` と `checked package -> transport plan -> non-final transport report` carrier split、WAN/save-load/devtools/product/public-ABI non-claims を追記した。

## progress.md update status

`progress.md` 更新済み:
`PA1-5` を 100% に更新し、last closed package を `P-A1-05` に進め、3-axis row、large-stage map、recent log、next autonomous package を `P-A1-06` に同期した。

## tasks.md update status

`tasks.md` 更新済み:
current practical status、ordered current work、package map、user decision blocker numberingを `P-A1-05` close / `P-A1-06` promoted next に同期した。

## samples_progress.md update status

`samples_progress.md` 更新済み:
practical transport family `TR-A1-01..07` を dashboard に追加し、`PA1-5` closeout、`P-A1-06` next、validation row、overall practical toolchain row を更新した。

## Reviewer findings and follow-up

- `Zeno` (spec / semantics review)
  - Finding: `P-A1-05` is safe only if the widened row set is carried as distinct practical transport-carrier evidence rather than reused `RUN-*`, `HP-*`, or `NET-*`.
  - Follow-up: actualized `TR-A1-01..07` as distinct practical transport rows and mirrored the non-claims into `specs/18`, `plan/44`, and snapshot docs.
- `Peirce` (runtime / helper review)
  - Finding: practical transport must not drift into “alpha network runtime reused under a new label”; it needs its own package input -> transport plan -> transport report lane, narrow wire request, and destination validation.
  - Follow-up: added `PracticalAlpha1TransportPlan`, `PracticalAlpha1TransportReport`, narrow `PracticalAlpha1TransportWireRequest`, and explicit destination-place validation.
- `Anscombe` (docs / snapshot / dashboard review)
  - Finding: all snapshots were stale because they still described `P-A1-05` as a blocker; wording needed one consistent widened `TR-A1-01..07` reading and `scripts/practical_alpha1_transport.py` as the runner.
  - Follow-up: synchronized `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `samples/README.md`, practical sample READMEs, and `scripts/README.md` to the same carrier/non-claim reading.
- Existing blocker review `review-2026-05-03-pa1-05-transport-scope-review.md`
  - Follow-up: kept its contradiction analysis as historical evidence, but superseded the blocker state by implementing the widened practical transport floor rather than silently narrowing the stage.

## Skipped validations and reasons

- Did not run `cargo test -p mir-ast` or `cargo test -p mir-runtime` whole-crate floors because `P-A1-05` is a focused practical transport package and the package closeout floor already reran the affected front-door, checker, runtime-plan, hotplug-plan, local-runtime, hot-plug, transport-plan, and transport-report lanes explicitly.
- Did not run alpha-0 Stage-C `alpha_network_runtime` / `alpha_network_docker_e2e.py` because `P-A1-05` must stand on its own practical carrier rather than borrow alpha-0 transport evidence.
- Did not run `P-A1-06` / `P-A1-07` / `P-A1-08` validations because this package does not modify devtools, save/load, or product prototype lanes.

## Commit / push status

- Main package commit: `fe7bb1b` (`mirrorea: close p-a1-05 practical transport e2e`)
- Push status: pushed to `origin/main`
- Follow-up docs-only metadata commit may update this report if later package sequencing needs an explicit report-status sync.

## Sub-agent session close status

- Reviewer agents used in this package:
  - `Zeno` for theory/spec boundary review
  - `Peirce` for runtime/helper/test review
  - `Anscombe` for docs/progress/sample-dashboard review
- All three reviewer findings were incorporated into the final carrier shape and snapshot wording.
- All three reviewer sessions are already closed.
