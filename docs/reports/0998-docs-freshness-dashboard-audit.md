# 0998 docs freshness dashboard audit

## Objective

Current documentation, dashboard, and task snapshots were audited after the post-`P21` final-public-hotplug ABI closeout and after the Rust formatting cleanup commit. The goal was to remove stale current-reading drift, keep `research_abstract`, `tasks.md`, `progress.md`, and `samples_progress.md` aligned with the true current gate, run fresh validation, and prepare the remaining docs audit diff for commit/push.

This report records a docs / validation / repository-memory audit. It is not a Mirrorea system completion claim.

## Scope and assumptions

- Normative sources remain `specs/`.
- Long-term repository memory remains `plan/`.
- Reports under `docs/reports/` are evidence, not normative source.
- `progress.md`, `tasks.md`, and `samples_progress.md` are current snapshots and dashboards, not append-only histories.
- The current actual open gate is `U1` actual commitment. post-`P21` docs-first trilogy is closed as repository memory, but public hot-plug ABI remains unfrozen.
- Heavy build output used `/mnt/mirrorea-work` through `scripts/env/mirrorea_storage_env.sh`; no destructive cleanup was run.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `.docs/progress-task-axes.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/11-roadmap-near-term.md`
- `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- `plan/35-post-p20-hotplug-next-package-inventory.md`
- `plan/38-post-p21-final-public-hotplug-abi-family.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/current_phase_closeout_01.md`

## Actions taken

- Committed and pushed the existing Rust formatting cleanup first:
  `b213721 Apply Rust formatting cleanup`.
- Rewrote `progress.md` as a compact current snapshot:
  current executable floor, strict non-claims, 3-axis progress, macro phase map, feature-family snapshot, closed package memory / active gate, validation anchors, and recent log.
- Rewrote `tasks.md` as a compact current task map:
  executable floor, package map, ordered current work, self-driven maintenance, user decision blockers, research-discovery items, and validation floor.
- Rewrote `samples_progress.md` as a runnable sample dashboard:
  active matrix, E2E samples, blockers, validation evidence, and historical/archive boundaries.
- Updated `docs/research_abstract/README.md` to include the missing post-`P21` distributed activation ordering summary.
- Rewrote the front matter and current-state section of `docs/research_abstract/mirrorea_future_axis_01.md` so current executable floor, closeout memory, and next actual `U1` gate are separated.
- Updated `docs/hands_on/current_phase_closeout_01.md` to include `validate_docs.py`, `cargo fmt --check`, `cargo test -p mir-runtime --test hotplug_runtime_skeleton`, and `git diff --check` in the primary command block.
- Updated `plan/11-roadmap-near-term.md` so the `0995` historical line does not read as if the third recommendation is still current-open.
- Reviewer cooperation:
  initial reviewer findings were used to fix stale `future_axis` queue wording, missing hands-on validation commands, missing `0996` / `0997` dashboard evidence, and the missing research abstract index entry.

Files changed by this docs audit:

- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `plan/11-roadmap-near-term.md`
- `docs/reports/0998-docs-freshness-dashboard-audit.md`

Git status at report authoring:

- Formatting cleanup was committed and pushed as `b213721 Apply Rust formatting cleanup`.
- The docs audit diff, including this report, was still uncommitted while the report was being written and was intended to be committed after final reviewer follow-up and validation.

## Evidence / outputs / test results

Fresh commands run in this audit:

| Command | Result | Evidence |
|---|---|---|
| `python3 scripts/check_source_hierarchy.py` | pass | `required: 23`, `present: 23`, `missing: 0` |
| `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete. Found 995 numbered report(s).` before this report was created |
| `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete. Found 996 numbered report(s).` after this report was created |
| `python3 scripts/current_l2_guided_samples.py closeout --format json` | pass | emitted clean near-end canonical inventory with signature / envelope / visualization / telemetry lanes |
| `python3 scripts/clean_near_end_samples.py closeout` | pass | emitted clean near-end suite inventory and current vocabulary split |
| `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | `sample_count: 10`; emitted place model, membership model, `MessageEnvelope`, hot-plug, visualization, telemetry, and `LayerSignature` inventories |
| `python3 scripts/avatar_follow_samples.py closeout --format json` | pass | `sample_count: 5`; `FAIRY-05` remains planned-only with `carrier_choice: UNRESOLVED` |
| `python3 scripts/typed_external_boundary_samples.py closeout --format json` | pass | `preview_sample_ids: EXT-03, EXT-04`; residual `EXT-01/02/05` remain planned |
| `python3 scripts/network_transport_samples.py closeout --format json` | pass | `NET-02..05` process-boundary canaries; real socket / session / durable replay deferred |
| `python3 scripts/projection_codegen_samples.py closeout --format json` | pass | `artifact_count: 4`; committed manifest bridge only |
| `python3 scripts/visual_debugger_viewer_samples.py closeout --format json` | pass | `bundle_count: 5`; typed viewer prototype inventory, not final viewer API |
| `bash scripts/storage/detach_prepare.sh` | pass with warning | `/dev/vdb1` mounted at `/mnt/mirrorea-work`; repo usage `109M`; external workdir usage `6.0G`; `/mnt/mirrorea-work/llvm` parent is root-owned and not writable |
| `bash scripts/storage/cleanup_disposable_artifacts.sh --list` | pass with warning | listed disposable candidates only; no files deleted; `llvm/src` intentionally excluded |
| `source scripts/env/mirrorea_storage_env.sh >/dev/null; cargo fmt --check` | pass | storage env warning only |
| `source scripts/env/mirrorea_storage_env.sh >/dev/null; cargo test -p mir-ast` | pass with warnings | all tests passed; existing dead-code warnings in current-L2 predicate-fragment test support |
| `source scripts/env/mirrorea_storage_env.sh >/dev/null; cargo test -p mirrorea-core` | pass | carrier tests `12/12`; runtime substrate tests `12/12` |
| `source scripts/env/mirrorea_storage_env.sh >/dev/null; cargo test -p mir-runtime` | pass | clean near-end `27/27`; hot-plug runtime skeleton `8/8`; current-L2 runtime / source tests passed |
| `source scripts/env/mirrorea_storage_env.sh >/dev/null; cargo test -p mir-semantics` | pass with warnings | Lean toolchain was available; Lean probe passed; existing dead-code warnings in test support |
| `source scripts/env/mirrorea_storage_env.sh >/dev/null; cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | emitted clean near-end closeout JSON |
| `git diff --check` | pass | whitespace-clean before and after report creation |

Skipped validations and reasons:

- No destructive storage cleanup was run. The cleanup script was run in `--list` mode only, because deletion requires explicit confirmation and was not needed for this docs audit.
- No actual LLVM build was run. LLVM build/install is outside this docs freshness scope, and `/mnt/mirrorea-work/llvm` is currently root-owned / non-writable for the current user.
- No production real-network transport or durable replay test was run. The current validated floor is helper-local `NET-01..05`, not production transport.
- No final public parser / adapter / viewer / hot-plug ABI validation was run, because those public surfaces remain deferred mixed gates.

Storage evidence:

- Root disk: `/dev/vda2` had about `32G` available during `detach_prepare`.
- External work disk: `/dev/vdb1` mounted at `/mnt/mirrorea-work`, about `180G` available.
- `target` in repo root was `0`; cargo target artifacts were under `/mnt/mirrorea-work/cargo-target`.
- `/mnt/mirrorea-work/llvm` root is `root:root` and not writable by the current user, so LLVM build/install cleanup remains guarded until ownership is repaired or explicit setup reruns.

## What changed in understanding

- The docs were not primarily missing a new theory decision; the main risk was current-reading drift caused by old queue history being too close to the active snapshot.
- `progress.md` and `tasks.md` are clearer when they stop re-listing every historical closeout in detail and instead point to `plan/` and `docs/reports/` for the trail.
- `samples_progress.md` should show evidence-backed runnable status and blockers, not act as a report log.
- The latest honest hot-plug statement is still `freeze prerequisite fixed; public ABI still unfrozen`.
- The next meaningful product-shaping step is actual `U1` commitment, not another self-driven post-`P21` docs-first package.

## Open questions

- Which packaging / installed binary target should become the first public adoption target?
- Which host integration target should drive the first real adapter ABI: native process, browser, engine adapter, or mixed?
- Which public surface should ship first: narrow core library surface, integration surface, or a staged split?
- How broad should the first shared-space operational catalog be?
- When actual `U1` is chosen, which hot-plug public ABI names should be candidates, and which runtime-private names must remain internal?

## Suggested next prompt

Proceed with actual `U1` commitment: choose the first packaging target, host integration target, first shipped public surface scope, and final shared-space operational catalog breadth; then create the first post-`U1` implementation tranche without freezing parser / adapter / viewer / hot-plug APIs beyond the chosen scope.
