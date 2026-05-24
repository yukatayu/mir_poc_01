# 2106 P-SURF-06 Source Patch Hot-Plug

- Date: 2026-05-24
- Author / agent: Codex
- Scope: Surface Mir P-SURF-06 source patch hot-plug
- Decision levels touched: L2 alpha implementation evidence, L3 final hot-plug ABI / migration planner non-claim

## Objective

- Identifier: `P-SURF-06 source patch hot-plug`
- Package: Surface Mir brace complete autonomous implementation
- Report path: `docs/reports/2106-p-surf-06-source-patch-hotplug.md`

Implement the narrow Surface Mir source patch pipeline after P-SURF-05: `.mir` source enters parse / typecheck / elaborate / compatibility / admission, accepted `patch-source` emits `HotPlugRequest`, `HotPlugVerdict`, and `activation_cut`, rejected patches do not mutate runtime state, and inspection commands do not activate patches.

## Scope and assumptions

- `.mir` files remain semantic source authority; `package.mir.json` remains an alpha artifact.
- Canonical Surface Mir place-scope syntax remains `S { ... }`; `S[ ... ]` remains rejected and is not sugar.
- Scope is alpha pipeline evidence. This package does not claim final hot-plug ABI, distributed durable source migration, production patch registry/signing, runtime MessageEnvelope dispatch, arbitrary native/WASM execution, final SDK, or production WAN/federation.
- Capability admission is report-level alpha evidence: source patches must carry the declared required capability refs before admission can accept.
- `check-source` and `elaborate-source` are inspection-only; `patch-source` is the command that may produce an activation cut on accepted input.

## Start state / dirty state

Started from pushed P-SURF-05 closeout on branch `main` with untracked `sub-agent-pro/surface-mir-brace-completion-001/` present. That handoff directory was intentionally left untracked and unstaged. P-SURF-06 began with clean committed P-SURF-05 sources plus local P-SURF-06 edits made in this task.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/39-surface-mir-placement-elaboration.md`
- `specs/40-indexed-state-semantics.md`
- `specs/41-role-admission-and-capability-grant.md`
- `specs/42-source-patch-hotplug-semantics.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/64-surface-mir-placement-roadmap.md`
- `plan/65-indexed-state-roadmap.md`
- `plan/66-role-admission-roadmap.md`
- `plan/67-source-patch-hotplug-roadmap.md`
- `plan/68-surface-full-system-v1-roadmap.md`
- `sub-agent-pro/surface-mir-brace-completion-001/*.md`
- `sub-agent-pro/surface-mir-brace-completion-001/sample-blueprints/*.md`
- Reviewer findings from sub-agent `019e5979-e8c9-78f1-8167-e4c620d38253`

## Actions taken

- Added `crates/mir-runtime::surface_source_patch_hotplug` with `SurfaceSourcePatchReport`, stage summaries, compatibility rows, Core IR diff, `HotPlugRequest`, `HotPlugVerdict`, and `SurfacePatchActivationCut`.
- Added CLI commands: `mirrorea-alpha check-source`, `parse-source`, `elaborate-source`, `patch-source`, and `export-core-ir`.
- Kept `check-source` / `elaborate-source` inspection-only and made `patch-source` the activation command.
- Added explicit `source_path_io_error` payloads for path-based source patch commands instead of silently converting missing files into parse failures.
- Added capability admission checks so declared required capabilities such as `PatchSource`, `AddState(World)`, and `PublishVisible(World)` appear in the request before admission accepts.
- Added `samples/full-system-v1-surface/source-patch/` with `PATCH-01..04`.
- Extended `scripts/surface_mir_samples.py`, `scripts/surface_mir_release_check.py`, `scripts/surface_mir_authoring_check.py` integration, and focused script tests for the 32-row Surface matrix.
- Updated documentation snapshots, specs, and plans to mark P-SURF-06 closed and `P-SURF-07 source operational suite` next.

## Files changed

- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/surface_source_patch_hotplug.rs`
- `crates/mir-runtime/tests/source_patch_hotplug.rs`
- `crates/mirrorea-cli/src/main.rs`
- `crates/mirrorea-cli/tests/surface_mir_cli.rs`
- `scripts/surface_mir_samples.py`
- `scripts/surface_mir_release_check.py`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/tests/test_surface_mir_release_check.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `samples/full-system-v1-surface/source-patch/**`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/full-system-v1-surface/README.md`
- `scripts/README.md`
- `docs/hands_on/README.md`
- `docs/hands_on/source_patch_hotplug_01.md`
- `docs/hands_on/surface_mir_alpha_01.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/surface_mir_alpha_01.md`
- `specs/00-document-map.md`
- `specs/42-source-patch-hotplug-semantics.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/00-index.md`
- `plan/66-role-admission-roadmap.md`
- `plan/67-source-patch-hotplug-roadmap.md`
- `plan/68-surface-full-system-v1-roadmap.md`

## Commands run

- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `cargo fmt`
- `cargo fmt --check`
- `git diff --check`
- `cargo test -p mir-ast --test surface_mir_parser -- --nocapture`
- `cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture`
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`
- `cargo test -p mir-semantics --test role_admission_capability_grant -- --nocapture`
- `cargo test -p mir-runtime --test source_patch_hotplug -- --nocapture`
- `cargo test -p mirrorea-cli --test surface_mir_cli -- --nocapture`
- `python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check`
- `python3 scripts/surface_mir_samples.py check-all --format json`
- `python3 scripts/surface_mir_authoring_check.py check-all --format json`
- `python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release-p-surf-06`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-surf-06`
- `python3 scripts/operational_product_samples.py check-all --format json`
- `python3 scripts/minimal_alpha1_patterns.py check-all --format json`

## Evidence / outputs / test results

- `scripts.tests.test_validate_docs`: 18 tests passed.
- `scripts/check_source_hierarchy.py`: 494 required paths present, 0 missing.
- `scripts/validate_docs.py`: documentation scaffold complete; 1258 numbered reports found after this report was added.
- `cargo fmt --check`: passed after applying `cargo fmt`.
- `git diff --check`: passed.
- `mir-ast surface_mir_parser`: 13 tests passed.
- `mir-semantics indexed_state_semantics`: 7 tests passed.
- `mir-semantics surface_to_core_elaboration`: 13 tests passed.
- `mir-semantics role_admission_capability_grant`: 4 tests passed.
- `mir-runtime source_patch_hotplug`: 3 tests passed.
- `mirrorea-cli surface_mir_cli`: 6 tests passed, including inspection-only `check-source` / `elaborate-source` and missing-file `source_path_io_error`.
- Surface helper unit tests: 34 tests passed.
- `scripts/surface_mir_samples.py check-all`: 32 rows passed, failed `[]`, workflow-ready `false`.
- `scripts/surface_mir_authoring_check.py check-all`: 32 sources accepted as `.mir` authority.
- `scripts/surface_mir_release_check.py`: ready `true`, failed commands `[]`.
- `scripts/product_alpha1_release_check.py`: status `accepted`, failed commands `[]`.
- `scripts/operational_product_samples.py`: status `accepted`, failed commands `[]`.
- `scripts/minimal_alpha1_patterns.py`: status `accepted`, failed `[]`.

## What changed in understanding

The source patch floor needs an explicit command split: inspecting a patch can validate and elaborate it, but activation belongs only to `patch-source`. The compatibility report also cannot claim admitted patch capabilities unless the `HotPlugRequest` carries the capability refs required by the same compatibility row.

## Open questions

- Final source patch hot-plug ABI is not frozen.
- Distributed durable migration planning, rollback/replay across durable save-load R3/R4, and production patch registry/signing remain later.
- Runtime MessageEnvelope dispatch integration remains later than this report-level patch pipeline.
- General TypeMismatch typechecker discharge in the Surface alpha floor remains later.
- Source operational roots are pending for P-SURF-07.

## Suggested next prompt

`P-SURF-07 source operational suite`

## Plan update status

Updated `plan/00-index.md`, `plan/66-role-admission-roadmap.md`, `plan/67-source-patch-hotplug-roadmap.md`, and `plan/68-surface-full-system-v1-roadmap.md` to mark P-SURF-06 as closed alpha source patch evidence and P-SURF-07 as the next promoted package.

## Documentation.md update status

Updated. `Documentation.md` now records P-SURF-06 as the source patch hot-plug evidence floor with CLI command anchors and keeps final hot-plug ABI / distributed migration / final SDK as non-claims.

## progress.md update status

Updated. `progress.md` records P-SURF-06 closure, current runnable commands, `PATCH-01..04`, inspection-only command behavior, current non-claims, and next gap `P-SURF-07`.

## tasks.md update status

Updated. `tasks.md` now makes `P-SURF-07 source operational suite` the current promoted autonomous package while preserving P-SURF-06 as alpha source patch pipeline evidence only.

## samples_progress.md update status

Updated. `samples_progress.md` records `samples/full-system-v1-surface/source-patch/`, the 32-row Surface matrix, activation-cut evidence for accepted `patch-source`, inspection-only `check-source` / `elaborate-source`, and rejection-without-mutation rows.

## Reviewer findings and follow-up

Reviewer sub-agent `019e5979-e8c9-78f1-8167-e4c620d38253` reported:

- `check-source` and `elaborate-source` were activating patches like `patch-source`.
- Admission accepted requests that lacked the capabilities declared as required by compatibility rows.
- Path-based commands converted missing files into parse failures instead of explicit source path I/O errors.
- CLI tests did not cover command split or missing-file behavior.

Follow-up implemented:

- Added inspection-only source patch report paths and used them for `check-source`, `elaborate-source`, and `export-core-ir`.
- Kept activation cuts behind `patch-source` only.
- Emitted required capability refs from compatibility rows and required them before admission accepts.
- Returned `source_path_io_error` for missing path-based source patch input.
- Added Rust CLI regression tests for inspection-only commands and missing-file behavior.

## Skipped validations and reasons

No requested P-SURF-06 validations were skipped. The mandatory documentation validation set was rerun after this report was added.

## Commit / push status

Pending at report creation. The intended commit message is `p-surf-06: add source patch hotplug evidence`; final commit hash and push status are reported in the package close response.

## Sub-agent session close status

- `019e5979-e8c9-78f1-8167-e4c620d38253`: completed reviewer pass, findings were addressed, and the session was closed.
- `019e5923-3446-7922-b226-c6535673e880`: prior code-mapper session was no longer needed and was closed.
