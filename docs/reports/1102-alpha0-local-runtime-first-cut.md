# Report 1102 — alpha0 local runtime first cut

- Date: 2026-05-02T07:45:00+09:00
- Author / agent: Codex
- Scope: `P-A0-07` first non-public Rust local-runtime floor for Mirrorea Spaces Alpha-0, including local-runtime sample rows `LR-01/02`, focused Rust tests/example, and snapshot/roadmap/sample synchronization
- Decision levels touched: `L1` package sequencing and stop-line preservation, `L2` local-runtime floor shape and sample/runtime evidence contract, no new `L0` foundation change

## Objective

Close `P-A0-07` by moving the smallest honest local-runtime behavior out of Python-only helpers into `mir-runtime`, while preserving the repository stop lines around hot-plug lifecycle, runtime package/avatar admission, Docker/network runtime, and final public ABI.

## Scope and assumptions

- Scope is intentionally narrow:
  one positive local queue dispatch path (`LR-01`) and one stale-membership rejection path (`LR-02`) over `LogicalPlaceRuntimeShell` / `MessageEnvelope`, plus a report-local event DAG export hook.
- This package does not add:
  parser integration, active `samples/alpha/` front-door execution, layer insertion runtime, runtime package/avatar admission, Docker/network runtime, save/load completion, or final Spaces alpha completion evidence.
- Working assumption for this cut:
  keep the runtime module alpha-local and sample-id keyed so the stop line remains explicit.

## Start state / dirty state

- Branch at package start: `main...origin/main`
- Start state before `P-A0-07`: clean after `ce17c0b` (`docs: finalize alpha cut report status`)
- Dirty state during this package: one new `mir-runtime` module, one example, one integration test, local-runtime sample anchors/sidecars, snapshot/roadmap docs, and this new report were edited in one in-flight package; no unrelated user changes were reverted.

## Documents consulted

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-language-overview-and-design-principles.md`
- `specs/02-system-model-and-execution-semantics.md`
- `specs/03-type-system-overview.md`
- `specs/09-structured-concurrency-and-effect-runtime.md`
- `specs/10-mir-machine-and-runtime-hooks.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/13-type-system-lifetime-fallback.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `specs/15-cut-save-load-checkpoint.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `scripts/README.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/43-alpha-e2e-roadmap.md`
- `samples/alpha/README.md`
- `samples/alpha/local-runtime/README.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- all files under `sub-agent-pro/alpha-0/`

## Actions taken

- Added `crates/mir-runtime/tests/alpha_local_runtime.rs` first and confirmed RED via unresolved import for `mir_runtime::alpha_local_runtime`.
- Added `crates/mir-runtime/src/alpha_local_runtime.rs` as a non-public in-memory Rust local-runtime floor over `mirrorea-core` substrate.
- Added example `crates/mir-runtime/examples/mirrorea_alpha_local_runtime.rs` to emit JSON evidence for the positive and stale-membership rows.
- Added `samples/alpha/local-runtime/LR-01/02` source-ish anchors plus `.expected.json` sidecars and promoted the family README from directory scaffold-only to first runtime-floor documentation.
- Synced `Documentation.md`, `samples/alpha/README.md`, `samples/README.md`, `scripts/README.md`, `plan/01-status-at-a-glance.md`, `plan/43-alpha-e2e-roadmap.md`, `progress.md`, `tasks.md`, and `samples_progress.md` to the new floor and its retained stop line.
- Kept hot-plug/package/avatar/runtime-network boundaries deferred despite related package pressure from the broader Alpha-0 roadmap.

## Files changed

- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/alpha_local_runtime.rs`
- `crates/mir-runtime/examples/mirrorea_alpha_local_runtime.rs`
- `crates/mir-runtime/tests/alpha_local_runtime.rs`
- `samples/alpha/local-runtime/README.md`
- `samples/alpha/local-runtime/lr-01-local_sugoroku_roll_publish_handoff.expected.json`
- `samples/alpha/local-runtime/lr-01-local_sugoroku_roll_publish_handoff.mir`
- `samples/alpha/local-runtime/lr-02-stale_membership_rejected.expected.json`
- `samples/alpha/local-runtime/lr-02-stale_membership_rejected.mir`
- `samples/alpha/README.md`
- `samples/README.md`
- `scripts/README.md`
- `Documentation.md`
- `plan/01-status-at-a-glance.md`
- `plan/43-alpha-e2e-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`

## Commands run

- `sed -n '200,260p' sub-agent-pro/alpha-0/12-codex-task-packages.md`
- `sed -n '1,220p' samples/alpha/local-runtime/README.md`
- `sed -n '1,180p' samples/clean-near-end/sugoroku-world/README.md`
- `cargo test -p mir-runtime --test alpha_local_runtime`
  initial RED at package start: unresolved import `mir_runtime::alpha_local_runtime`
- `cargo test -p mir-runtime --test alpha_local_runtime`
  GREEN after runtime module/test/sample implementation
- `cargo test -p mirrorea-core --test carriers`
- `cargo test -p mirrorea-core --test runtime_substrate`
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton`
- `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- local-sugoroku`
- `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- stale-membership`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --format json`
- `cargo fmt`
- `date '+%Y-%m-%d %H:%M JST'`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `cargo fmt --check`
- `git diff --check`

## Evidence / outputs / test results

- `cargo test -p mir-runtime --test alpha_local_runtime` is green:
  `3 passed; 0 failed`
- The positive example emits a Rust report with:
  `sample_id = LR-01`, `current_owner = Bob`, one accepted `roll_request#1`, and an event DAG containing `publication_order`, `witness_order`, and `handoff_order`.
- The stale-membership example emits a Rust report with:
  `sample_id = LR-02`, `terminal_outcome = rejected`, `reason_family = membership_freshness`, `dispatch_outcome = rejected_stale_membership`, `reason_refs = ["membership_epoch_drift"]`, and no visible-history mutation.
- The Python parity anchor `03_roll_publish_handoff` still reports `dice_owner = Bob`, witness `draw_pub#1`, and relation family including `publication_order`, `witness_order`, and `scoped_happens_before`.
- Final closeout freshness rerun is green at `2026-05-02 07:45 JST`:
  `mirrorea-core` carriers `12 passed`, `runtime_substrate` `12 passed`, `mir-runtime` hot-plug skeleton `8 passed`, local-runtime `3 passed`, both Rust examples emitted JSON reports, source hierarchy `required: 60 / present: 60 / missing: 0`, `validate_docs.py` reported `Documentation scaffold looks complete.` and `Found 1103 numbered report(s).`, report-schema unit ran `11` tests and passed, `cargo fmt --check` passed, and `git diff --check` was clean.

## What changed in understanding

- The first honest Rust local-runtime cut is smaller than “integrated Sugoroku runtime” in the broad sense.
  The safe floor is queue/message/membership/event export over `mirrorea-core`, not a port of the whole Python helper catalog.
- Sample identity can advance before parser integration if the stop line is explicit.
  `LR-01/02` are source-ish anchors plus sidecars, while the current runnable floor is sample-id keyed Rust logic.
- The runtime cut should stay visibly separate from hot-plug/package/avatar claims.
  Reusing related substrate does not authorize Phase 4/5/7 completion claims.

## Open questions

- Whether the next local-runtime widening should add an explicit second envelope (`handoff_notice#1`) or stay single-envelope until layer insertion is in place remains open.
- Whether the module name should stay alpha-local (`alpha_local_runtime`) or be generalized later is open; for this cut the alpha-local name keeps the stop line explicit.
- A dedicated missing-witness negative row in the Rust local-runtime family is still open; this package only actualizes stale-membership rejection.

## Suggested next prompt

Continue with `P-A0-08`: add the first contract-checked layer insertion runtime cut without promoting hot-plug/package/avatar/network claims.

## Plan update status

`plan/` 更新済み:
`plan/01-status-at-a-glance.md` now records `P-A0-07` closeout and the `P-A0-08` reopen point; `plan/43-alpha-e2e-roadmap.md` now records the first Rust local-runtime floor and its validation anchors.

## Documentation.md update status

`Documentation.md` 更新済み:
the Alpha-0 integration summary now includes the non-public Rust local-runtime floor and its stop line.

## progress.md update status

`progress.md` 更新済み:
`P-A0-08` is now the current package, the large stage map now includes progress percentages, and `P-A0-07` local-runtime closeout evidence is recorded at `2026-05-02 07:45 JST`.

## tasks.md update status

`tasks.md` 更新済み:
Alpha-0 package ordering now marks `P-A0-07` closed, promotes `P-A0-08` to the head of the autonomous queue, and adds `P-A0-10` runtime package/avatar skeleton to the queue snapshot.

## samples_progress.md update status

`samples_progress.md` 更新済み:
`A0-LOCAL` now sits at `75%` with positive/negative Rust runtime evidence and a family-specific cargo/example validation anchor.

## Reviewer findings and follow-up

- `Ampere` (explorer) recommended a `mir-runtime`-owned local-runtime module over `mirrorea-core` substrate, with queue/message/membership/event-DAG scope only. Followed.
- `Nash` (reviewer) warned that `P-A0-07` must stop before hot-plug/package/avatar/runtime attach claims and keep the cut in Stage B / Phase 3 only. Followed.
- `Russell` (reviewer) warned that the pre-existing repo evidence was still split between Python helper and Rust substrate/hot-plug narrow projections. Followed by adding a new Rust runtime floor instead of re-labeling existing evidence as integrated runtime.
- Contradiction resolution:
  - kept the runtime floor in `mir-runtime`, but under an alpha-local module name so the stop line remains explicit
  - did not widen into runtime attach, layer insertion, runtime package/avatar, or Docker/network runtime
  - kept the Python Sugoroku helper as a parity anchor rather than turning it into a hidden dependency of the new Rust floor

## Skipped validations and reasons

- No Docker/network validation was added for this package because `P-A0-09` remains separate.
- No layer insertion / auth / rate-limit runtime validation was added because `P-A0-08` remains separate.
- No runtime package/avatar validation was added because `specs/16` and `samples/alpha/avatar-runtime/` / `hotplug-runtime/` remain later families.
- No additional closeout validations were skipped after the final freshness rerun.

## Commit / push status

Pending commit / push at this report write; local closeout validation is green.

## Sub-agent session close status

- `Ampere` explorer completed and the session is closed.
- `Nash` reviewer completed and the session is closed.
- `Russell` reviewer completed and the session is closed.
