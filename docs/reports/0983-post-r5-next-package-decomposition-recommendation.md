# Report 0983 — post-R5 next package decomposition recommendation

- Date: 2026-04-28 23:12:03 +0900
- Author / agent: Codex (GPT-5)
- Scope: read-only inspection of the current repo state to recommend the smallest honest docs-first next package or minimal package sequence after `R5`
- Decision levels touched: `L2` planning / queue decomposition only; no normative spec change

## 1. Objective

Recommend the narrowest source-backed post-`R5` next package decomposition that preserves the current owner split:

- helper-local preview stays helper-local
- `mirrorea-core` stays carrier / substrate only
- `mir-runtime` stays thin assembly unless explicitly widened

without overclaiming runtime-crate hot-plug engine completion.

## 2. Scope and assumptions

- This task is analysis-only; no code or docs wording outside this report was changed.
- The recommendation is repository-memory guidance, not an adopted queue rewrite.
- Current docs saying `implementation-side runtime-crate hot-plug engine actualization remains later; exact package decomposition is OPEN` remain authoritative until separately updated.

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- `plan/32-hotplug-real-migration-rollback-boundary.md`
- `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
- `crates/mirrorea-core/src/lib.rs`
- `crates/mirrorea-core/src/runtime.rs`
- `crates/mirrorea-core/tests/carriers.rs`
- `crates/mirrorea-core/tests/runtime_substrate.rs`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/clean_near_end.rs`
- `crates/mir-runtime/tests/clean_near_end_samples.rs`
- `scripts/sugoroku_world_samples.py`

## 4. Actions taken

1. Re-read the current front-door docs and queue snapshot to confirm the promoted next line after `R5`.
2. Re-read `plan/32` and `plan/33` to confirm the kept-later boundary and owner split.
3. Re-read helper-local hot-plug anchors in `scripts/sugoroku_world_samples.py`.
4. Re-read `mirrorea-core` and `mir-runtime` crate docs / code / tests to confirm actual current Rust-side ownership.
5. Ran helper closeout and crate tests to verify the repo still matches the documented reading.

## 5. Files changed

- Added this report only:
  - `docs/reports/0983-post-r5-next-package-decomposition-recommendation.md`

No other files changed.

- `plan/ 更新不要`
- `progress.md 更新不要`
- `tasks.md 更新不要`
- `samples_progress.md 更新不要`

## 6. Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
rg -n "runtime-crate hot-plug engine|exact package decomposition|R5|owner split|implementation-side" README.md Documentation.md progress.md tasks.md plan/21-hotplug-attachpoint-roadmap.md plan/32-hotplug-real-migration-rollback-boundary.md plan/33-runtime-crate-hotplug-engine-ownership-cut.md
rg -n "hotplug|attach|detach|migration_contract|activation_cut|provider_boundary|audit_trace_boundary|attach_point_boundary|LogicalPlaceRuntimeShell" crates/mirrorea-core crates/mir-runtime scripts/sugoroku_world_samples.py
python3 scripts/sugoroku_world_samples.py closeout --format json
cargo test -p mirrorea-core
cargo test -p mir-runtime
date '+%Y-%m-%d %H:%M:%S %z'
```

## 7. Evidence / outputs / test results

- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  returned:
  - `hotplug_scope = helper_local_package_manager_preview`
  - `hotplug_anchor_samples = [01_runtime_attach_game, 09_detach_todo]`
  - `hotplug_kept_later_gates = [runtime_crate_hotplug_engine, rollback_protocol, durable_migration_engine, final_public_hotplug_abi]`
  - `hotplug_validation_floor = helper-local attach/detach lifecycle evidence only; not completed migration/rollback/runtime-crate ownership`
- `cargo test -p mirrorea-core` passed:
  - `tests/carriers.rs`: 7 passed
  - `tests/runtime_substrate.rs`: 12 passed
- `cargo test -p mir-runtime` passed:
  - `tests/clean_near_end_samples.rs`: 27 passed
  - other runtime/current-L2 manifests and runtime skeleton tests also passed
- Rust-side code/test anchors still show:
  - `mirrorea-core` owns generic carriers plus `MembershipRegistry` / `PlaceCatalog` / `LogicalPlaceRuntimeShell`
  - `mir-runtime` canonical seams are still `provider_boundary` / `audit_trace_boundary`
  - no Rust-side hot-plug-specific engine symbol family is currently actualized

## 8. What changed in understanding

- `R5` is already fully mirrored across front-door docs and `plan/33`; the repo no longer needs another owner-split package.
- The strongest reason not to promote a broad “runtime hot-plug engine first cut” immediately is that `plan/33` explicitly recommends avoiding hot-plug-specific Rust carrier names until the owning layer is explicit.
- The narrowest honest next package is therefore a docs-first admissibility / decomposition cut that decides what the first Rust-side hot-plug-specific slice may be, before actualizing engine-neutral carriers or runtime orchestration.
- If implementation must start immediately, the smallest non-overclaiming sequence is:
  1. docs-first carrier admission cut
  2. `mirrorea-core` engine-neutral hot-plug carrier tranche
  3. `mir-runtime` thin orchestration skeleton tranche

## 9. Open questions

- Should the first Rust-side hot-plug-specific family begin at request / verdict carriers only, or also include attach-visible lifecycle summary rows?
- Should the first implementation tranche land in `mirrorea-core` as reusable engine-neutral carriers before any `mir-runtime` orchestration code, or does the user want a runtime-first spike even if the carrier vocabulary remains provisional?
- Is the next package meant to remain docs-first only, or should it be promoted directly into implementation work after approval?

## 10. Suggested next prompt

Draft `plan/34` as a docs-first package named `runtime-crate hot-plug carrier admission cut`, limited to the first admissible Rust-side hot-plug-specific carrier family, its owner layer, validation floor, and explicit non-goals, while keeping orchestration, rollback, migration, distributed ordering, and final public ABI out of scope.
