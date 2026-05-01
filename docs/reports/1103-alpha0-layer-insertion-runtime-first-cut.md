# Report 1103 — Alpha-0 Layer Insertion Runtime First Cut

- Date: 2026-05-02
- Author / agent: Codex
- Scope: `P-A0-08` layer insertion runtime first cut
- Decision levels touched: `L1` / `L2` implementation of already-frozen `specs/14` / `specs/17`; no new normative decision

## Objective

Close `P-A0-08` by turning `samples/alpha/layer-insertion/` from a README-only scaffold into the first non-public Rust attach-time layer-insertion floor, while keeping auth/rate-limit/lifecycle/public-ABI stop lines explicit.

## Scope and assumptions

- Scope is limited to one runtime-private `MessageDispatch` attach point over the already-actualized `P-A0-07` local-runtime floor.
- The current cut proves attach-time contract comparison, authority-gated debug attach, trace visibility only after accepted attach, explicit auth contract-update admission, declared-failure rate-limit preview, and incompatible patch rejection.
- This task does not freeze a final public layer-attachment ABI, does not claim completed hot-plug lifecycle/detach/migration/distributed ordering, and does not claim parser-driven execution of `samples/alpha/`.
- `hotplug_runtime` is reused only as a runtime-private carrier/report substrate. The new floor computes compatibility locally before producing a verdict; it does not import an already-admitted verdict as authority.

## Start state / dirty state

- Start state was clean: `git status --short --branch` showed `## main...origin/main`.
- No unrelated dirty files were introduced before edits.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `scripts/README.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `specs/00-overview-and-principles.md`
- `specs/01-core-calculus.md`
- `specs/02-effects-and-handlers.md`
- `specs/03-temporal-and-ownership-modes.md`
- `specs/09-language-and-toolchain-status.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `sub-agent-pro/alpha-0/01..12`
- `docs/reports/1102-alpha0-local-runtime-first-cut.md`

## Actions taken

1. Re-read the Alpha-0 package, repository hierarchy docs, and current Phase 4 evidence to confirm that `layer-insertion/` was still scaffold-only and that `P-A0-08` needed a runtime-sensitive floor rather than more planned rows.
2. Ran three sub-agents for package-specific review:
   - `Socrates`: smallest honest cut / file ownership / deferred lines
   - `Heisenberg`: contract/layer compatibility soundness review
   - `Chandrasekhar`: sample/validation floor review
3. Implemented `crates/mir-runtime/src/alpha_layer_insertion_runtime.rs` as a runtime-private attach-time compatibility engine over:
   - `LogicalPlaceRuntimeShell`
   - `HotPlugRequest` / `HotPlugVerdict`
   - `assemble_hotplug_runtime_skeleton_report`
   - `LayerSignature`
4. Kept the cut narrow and sample-driven:
   - `LI-01` accepted debug attach with authority and redacted trace only after attach
   - `LI-02` rejected non-admin debug attach with no active layer and no trace
   - `LI-03` auth attach accepted only via explicit contract-update activation cut
   - `LI-04` rate-limit attach accepted only because `RateLimited` is already declared, with preview reject evidence
   - `LI-05` incompatible patch rejected before activation
5. Added dedicated example and integration test coverage for the new floor.
6. Materialized `samples/alpha/layer-insertion/LI-01..05` source-ish anchors and `.expected.json` sidecars.
7. Updated snapshot/taxonomy docs so `layer-insertion/` is now the current runtime-sensitive authority for Phase 4, while `hotplug-runtime/` and `contract-variance/` keep their broader planned/mirror roles.

## Files changed

- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/hotplug_runtime.rs`
- `crates/mir-runtime/src/alpha_layer_insertion_runtime.rs`
- `crates/mir-runtime/examples/mirrorea_alpha_layer_insertion_runtime.rs`
- `crates/mir-runtime/tests/alpha_layer_insertion_runtime.rs`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `scripts/README.md`
- `samples/alpha/README.md`
- `samples/alpha/contract-variance/README.md`
- `samples/alpha/hotplug-runtime/README.md`
- `samples/alpha/layer-insertion/README.md`
- `samples/alpha/layer-insertion/li-01-debug_layer_attach_authorized.mir`
- `samples/alpha/layer-insertion/li-01-debug_layer_attach_authorized.expected.json`
- `samples/alpha/layer-insertion/li-02-debug_layer_non_admin_rejected.mir`
- `samples/alpha/layer-insertion/li-02-debug_layer_non_admin_rejected.expected.json`
- `samples/alpha/layer-insertion/li-03-auth_layer_contract_update_path.mir`
- `samples/alpha/layer-insertion/li-03-auth_layer_contract_update_path.expected.json`
- `samples/alpha/layer-insertion/li-04-ratelimit_declared_failure.mir`
- `samples/alpha/layer-insertion/li-04-ratelimit_declared_failure.expected.json`
- `samples/alpha/layer-insertion/li-05-incompatible_patch_rejected.mir`
- `samples/alpha/layer-insertion/li-05-incompatible_patch_rejected.expected.json`
- `plan/01-status-at-a-glance.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`

## Commands run

```bash
git status --short --branch
sed -n '1,220p' .agents/skills/discord-report/SKILL.md
sed -n '1,220p' crates/mirrorea-core/src/layer.rs
sed -n '1,220p' samples/alpha/layer-insertion/README.md
sed -n '1,220p' specs/14-contract-subtyping-layer-compatibility.md
sed -n '1,260p' crates/mir-runtime/src/clean_near_end.rs
sed -n '1,220p' samples/alpha/contract-variance/var-11-redaction_layer_valid.mir
sed -n '1,220p' samples/alpha/contract-variance/var-11-redaction_layer_valid.expected.json
sed -n '1,220p' samples/alpha/contract-variance/var-12-debug_layer_requires_authority.mir
sed -n '1,220p' samples/alpha/contract-variance/var-12-debug_layer_requires_authority.expected.json
sed -n '1,220p' samples/alpha/hotplug-runtime/hp-02-attach_debug_layer_admin.expected.json
sed -n '1,220p' samples/alpha/hotplug-runtime/hp-05-attach_ratelimit_layer_contract_valid.expected.json
sed -n '1,260p' crates/mir-runtime/src/alpha_local_runtime.rs
sed -n '1,260p' sub-agent-pro/alpha-0/12-codex-task-packages.md
sed -n '1,220p' samples/alpha/local-runtime/README.md
sed -n '1,220p' samples/alpha/README.md
sed -n '1,220p' crates/mir-runtime/src/lib.rs
sed -n '1,240p' crates/mir-runtime/tests/hotplug_runtime_skeleton.rs
sed -n '1,240p' crates/mir-runtime/tests/alpha_local_runtime.rs
sed -n '1,220p' plan/40-layer-compatibility-freeze-roadmap.md
sed -n '1,220p' specs/17-mirrorea-spaces-alpha-scope.md
sed -n '1,220p' specs/16-runtime-package-adapter-hotplug.md
sed -n '1,360p' crates/mirrorea-core/src/runtime.rs
sed -n '1,220p' crates/mir-runtime/examples/mirrorea_alpha_local_runtime.rs
sed -n '1,220p' progress.md
sed -n '1,240p' samples_progress.md
sed -n '1,320p' crates/mir-runtime/src/hotplug_runtime.rs
sed -n '1,260p' samples/alpha/contract-variance/README.md
find samples/alpha/hotplug-runtime -maxdepth 1 -type f | sort
sed -n '1,220p' plan/01-status-at-a-glance.md
sed -n '1,260p' plan/43-alpha-e2e-roadmap.md
sed -n '1,220p' Documentation.md
sed -n '1,260p' samples/README.md
sed -n '1,260p' scripts/README.md
cargo test -p mirrorea-core --test carriers
cargo test -p mir-runtime --test alpha_local_runtime
cargo test -p mir-runtime --test hotplug_runtime_skeleton
cargo test -p mir-runtime --test alpha_layer_insertion_runtime
cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- closeout
find samples/alpha/layer-insertion -maxdepth 1 -type f | sort
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- `cargo test -p mirrorea-core --test carriers`
  - 12 tests passed.
- `cargo test -p mir-runtime --test alpha_local_runtime`
  - 3 tests passed.
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton`
  - 8 tests passed after adding `Serialize` derivations to the runtime-private report structs reused by the new layer floor.
- `cargo test -p mir-runtime --test alpha_layer_insertion_runtime`
  - 6 tests passed.
  - Verified accepted/rejected debug attach, explicit auth contract-update path, declared-failure rate-limit path, incompatible patch reject, and kept-later refs.
- `cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- closeout`
  - emitted JSON closeout evidence for `LI-01..05`
  - `LI-01` showed `terminal_outcome = accepted`, `active_layers_after = ["debug_trace_layer"]`, and two redacted trace rows sourced from `LR-01`
  - `LI-03` showed `accepted_path = explicit_contract_update`
  - `LI-04` showed preview `terminal_outcome = rejected` with `RateLimited`
  - `LI-05` showed compatibility failures including `provided_interface_shrunk`, `observation_labels_widened`, `redaction_weakened`, and `trace_mode_would_overobserve`
- `find samples/alpha/layer-insertion -maxdepth 1 -type f | sort`
  - confirmed README plus `LI-01..05` `.mir` / `.expected.json` pairs
- `python3 scripts/check_source_hierarchy.py`
  - passed with required/present/missing = 60/60/0.
- `python3 scripts/validate_docs.py`
  - passed with `Documentation scaffold looks complete.` and `Found 1104 numbered report(s).`
- `python3 -m unittest scripts.tests.test_validate_docs`
  - 11 tests passed.
- `cargo fmt --check`
  - passed.
- `git diff --check`
  - passed.

## What changed in understanding

- The smallest honest `P-A0-08` close is not a full hot-plug engine. It is an attach-time compatibility floor over the Stage-B local runtime.
- `hotplug_runtime` can be reused as a report/carrier substrate only if verdicts are recomputed locally first; importing admitted verdicts would overclaim safety.
- `layer-insertion/` needed to become the runtime-sensitive authority for Phase 4, while `hotplug-runtime/` and `contract-variance/` remain broader planned/mirror families.
- Auth and rate-limit must stay separated:
  - rate-limit can stay transparent only when `RateLimited` is already declared
  - auth must stay an explicit contract-update / activation-cut path in the current cut

## Open questions

- `P-A0-09` still needs a concrete Docker/local-subprocess evidence line and may hit a real environment/Docker-availability blocker.
- `P-A0-10` still needs a smallest honest package/avatar floor that does not collapse avatar/VRM/VRChat/Unity concepts into Mir core primitives.
- The current contract comparison uses exact type-name equality plus finite-set containment. Widening that to a richer attach-time algebra is still later.

## Suggested next prompt

Continue autonomously with `P-A0-09` and keep Docker unavailability or scope reduction honest if the environment does not support the narrow network floor.

## Plan update status

`plan/` 更新済み:
`plan/01-status-at-a-glance.md`, `plan/40-layer-compatibility-freeze-roadmap.md`, `plan/43-alpha-e2e-roadmap.md`

## Documentation.md update status

`Documentation.md` 更新済み:
Alpha-0 line now mentions the non-public Rust layer-insertion floor and its stop lines.

## progress.md update status

`progress.md` 更新済み:
Stage/phase/package reading, large stage map progress, and alpha runtime-prep status were moved to the `P-A0-08`-closed / `P-A0-09`-next state.

## tasks.md update status

`tasks.md` 更新済み:
Current package order and the autonomous package table now show `P-A0-08` closed and `P-A0-09` next.

## samples_progress.md update status

`samples_progress.md` 更新済み:
`A0-LAYER` moved from scaffold-only to Rust runtime floor, and `A0-HP` / `A0-VAR` now document their runtime-backed mirrors.

## Reviewer findings and follow-up

- `Socrates`
  - recommended keeping the cut in `mir-runtime`, reusing `LogicalPlaceRuntimeShell`, `HotPlugRequest` / `HotPlugVerdict`, and `assemble_hotplug_runtime_skeleton_report`
  - recommended `HP-02` + `HP-03` plus `VAR-08` as the strongest proving floor
  - implemented follow-up:
    the new floor lives in `mir-runtime`, keeps one attach point only, and materializes `LI-01..05` with `HP-02..06` / `VAR-08/12/13` mirrors
- `Heisenberg`
  - warned that the existing runtime path trusted already-admitted verdicts and that observation/redaction/retention needed to be part of attach-time compatibility
  - implemented follow-up:
    `alpha_layer_insertion_runtime` computes compatibility locally and checks pre/post/effect/failure/capability/surface/observation/redaction/retention/trace-mode rows before emitting a verdict
- `Chandrasekhar`
  - warned that `layer-insertion/` could not be honestly closed while it was README-only
  - recommended the smallest five-row closeout set: debug accept, debug reject, auth explicit update, rate-limit declared failure, incompatible patch reject
  - implemented follow-up:
    `layer-insertion/` now contains `LI-01..05` and is the current runtime-sensitive authority for Phase 4

## Skipped validations and reasons

- Docker/network validation was not run in this package because it belongs to `P-A0-09`, not `P-A0-08`.
- Runtime package/avatar validation was not run in this package because it belongs to `P-A0-10`.
- No parser-driven `samples/alpha/` execution was run because the current floor is still non-public sample-ID keyed Rust runtime code, not a parser front door.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- `Socrates` (`019de5bb-0207-76f0-82c6-9ba279e3fb7a`) closed after returning the smallest-cut/runtime-scope review.
- `Heisenberg` (`019de5bb-0473-7010-b9cf-9966b159fc97`) closed after returning the contract/layer compatibility review.
- `Chandrasekhar` (`019de5bb-04f5-7111-a144-5260986133dc`) closed after returning the sample/validation review.
