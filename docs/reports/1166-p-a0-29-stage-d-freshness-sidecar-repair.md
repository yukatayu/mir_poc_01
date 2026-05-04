# Report 1166 — P-A0-29 Stage D Freshness and Sidecar Repair

- Date: 2026-05-04
- Author / agent: Codex
- Scope: alpha-0 evidence line Stage D freshness repair after avatar/package sidecar drift
- Decision levels touched: L2 implementation snapshot / repository memory only

日本語要約:
shared `HotPlugVerdict.witness_reason_refs` lane widening の後、alpha-0 Stage D の avatar/package expected sidecar が stale になり、`cargo test -p mir-runtime --test alpha_avatar_runtime` の exact sidecar comparison が落ちていました。`AV-01/02/06/08/09` と `HP-11/12/15` の committed expected JSON を current runtime output に合わせて narrow refresh し、Stage D current-scope closeout の範囲を広げずに freshness を回復しました。

## Objective

Repair the Stage D alpha-0 evidence floor so `alpha_avatar_runtime` exact-sidecar validation is green again after the shared hot-plug verdict lane widened with `witness_reason_refs`.

## Scope and assumptions

- Scope is limited to the Stage D avatar/package subset:
  `AV-01/02/06/08/09` and `HP-11/12/15`.
- No new runtime semantics, no new admissible rows, no new runnable-root claim.
- `alpha-0.8` remains a current-scope evidence closeout, not a reopened promoted implementation line.
- Practical alpha-1 package line remains the promoted mainline; this package is maintenance on the reserve/reference alpha-0 lane.

## Start state / dirty state

- Working tree was clean at task start.
- Resource check was performed before heavier reruns.
  - `df -h .`: root disk had ~30G available.
  - `free -h`: ~273Mi available RAM plus swap headroom.
- Initial focused rerun found a real failure:
  `cargo test -p mir-runtime --test alpha_avatar_runtime` failed with `sidecar drift for AV-01`.

## Documents consulted

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
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `samples/alpha/README.md`
- `samples/alpha/avatar-runtime/README.md`
- `samples/alpha/hotplug-runtime/README.md`
- `samples/alpha/layer-insertion/README.md`
- `scripts/alpha_hotplug_lifecycle_samples.py`
- `scripts/tests/test_alpha_hotplug_lifecycle_samples.py`
- `crates/mir-runtime/tests/alpha_avatar_runtime.rs`
- `crates/mir-runtime/src/alpha_avatar_runtime.rs`
- `docs/reports/1132-p-a0-25-stage-d-lifecycle-closeout.md`
- `docs/reports/1163-alpha-0-5-stage-b-freshness-and-status-clarification.md`
- relevant current sidecars under `samples/alpha/avatar-runtime/` and `samples/alpha/hotplug-runtime/`

## Actions taken

1. Re-ran the focused Stage D floor and reproduced the failure in `alpha_avatar_runtime`.
2. Compared committed sidecars against current runtime closeout output.
3. Confirmed the drift pattern was uniform across all eight Stage D avatar/package reports:
   only `hotplug_skeleton.verdict.witness_reason_refs` and `"witness_reason_refs"` in `verdict_lanes` were missing from the committed sidecars.
4. Verified the shared lane widening came from the existing hot-plug carrier path and matched other current runtime/practical patterns, rather than being an AV-only regression.
5. Refreshed the eight stale expected sidecars only.
6. Re-ran the Stage D runtime, helper, and docs validation floor.
7. Synchronized `Documentation.md`, `plan/01-status-at-a-glance.md`, `progress.md`, `tasks.md`, and `samples_progress.md` so the repo now records this as `P-A0-29` maintenance on a closed evidence row.

## Files changed

- `samples/alpha/avatar-runtime/av-01-placeholder_avatar_runtime.expected.json`
- `samples/alpha/avatar-runtime/av-02-custom_mir_avatar_runtime.expected.json`
- `samples/alpha/avatar-runtime/av-06-untrusted_native_avatar_rejected.expected.json`
- `samples/alpha/avatar-runtime/av-08-runtime_unavailable_placeholder.expected.json`
- `samples/alpha/avatar-runtime/av-09-adapter_attempts_undeclared_effect.expected.json`
- `samples/alpha/hotplug-runtime/hp-11-unsigned_native_package_rejected.expected.json`
- `samples/alpha/hotplug-runtime/hp-12-signed_overcapability_package_rejected.expected.json`
- `samples/alpha/hotplug-runtime/hp-15-revoked_signed_package_rejected.expected.json`
- `Documentation.md`
- `plan/01-status-at-a-glance.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `df -h .`
- `free -h`
- `git status --short`
- `date '+%Y-%m-%d %H:%M JST'`
- `python3 -m unittest scripts.tests.test_alpha_hotplug_lifecycle_samples scripts.tests.test_alpha_avatar_runtime_samples scripts.tests.test_validate_docs`
- `cargo test -p mir-runtime --test alpha_layer_insertion_runtime`
- `cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- closeout`
- `cargo test -p mir-runtime --test alpha_avatar_runtime`
- `cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- closeout`
- `python3 scripts/alpha_avatar_runtime_samples.py check-all --format json`
- `python3 scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout --format json`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `cargo fmt --check`
- `git diff --check`

## Evidence / outputs / test results

- Initial failure reproduced exactly:
  `cargo test -p mir-runtime --test alpha_avatar_runtime`
  failed at `implemented_reports_match_committed_sidecars` with `sidecar drift for AV-01`.
- Root-cause diff inspection showed a uniform two-field drift across all eight reports:
  committed sidecars were missing
  `hotplug_skeleton.verdict.witness_reason_refs = ["required_witnesses_present"]`
  and `"witness_reason_refs"` in `hotplug_skeleton.verdict_lanes`.
- After the narrow sidecar refresh:
  - `python3 -m unittest scripts.tests.test_alpha_hotplug_lifecycle_samples scripts.tests.test_alpha_avatar_runtime_samples scripts.tests.test_validate_docs`
    passed `23` tests.
  - `cargo test -p mir-runtime --test alpha_layer_insertion_runtime`
    passed `6` tests.
  - `cargo test -p mir-runtime --test alpha_avatar_runtime`
    passed `10` tests.
  - `python3 scripts/alpha_avatar_runtime_samples.py check-all --format json`
    passed `8/8`.
  - `python3 scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout --format json`
    returned `stage_d_complete: true`.
  - `python3 scripts/check_source_hierarchy.py`
    passed.
  - `python3 scripts/validate_docs.py`
    passed.
  - `cargo fmt --check`
    passed.
  - `git diff --check`
    passed.

## What changed in understanding

- The failure was not a Stage D semantic regression.
- The failure was a stale-fixture problem caused by a previously widened shared hot-plug verdict lane.
- Stage D alpha-0.8 remains closed for current scope; what needed repair was only exact sidecar freshness on the avatar/package subset.

## Open questions

- No new semantic open question was created by this package.
- The broader product-lane blocker remains unchanged:
  practical `P-A1-08` wording still overclaims avatar/product semantics unless it is recut or preceded by practical `AV-A1-*` carriers.

## Suggested next prompt

Proceed on the practical alpha-1 line, either by recutting `P-A1-08` into a thin product-preview bundle or by promoting a prior practical `AV-A1-*` carrier package.

## Plan update status

`plan/` 更新済み:
`plan/01-status-at-a-glance.md` now records `P-A0-29` as a Stage D freshness/sidecar repair package and updates the alpha-0 package-reading memory to `P-A0-01..29`.

## Documentation.md update status

`Documentation.md` 更新済み:
added a concise note that `P-A0-29` refreshed stale Stage D avatar/package sidecars after shared `witness_reason_refs` widening without widening semantics or scope.

## progress.md update status

`progress.md` 更新済み:
updated the alpha-0 evidence reference snapshot, marked `P-A0-29` as the latest alpha-0 evidence maintenance package, and appended a new dated recent-log entry.

## tasks.md update status

`tasks.md` 更新済み:
added `P-A0-29` to the closed package/task map and recorded the Stage D freshness repair as maintenance rather than a reopened implementation line.

## samples_progress.md update status

`samples_progress.md` 更新済み:
updated the alpha-0 Stage D rows/timestamps, added report `1166` to the relevant alpha avatar/hot-plug entries, and logged the refreshed Stage D validation checkpoint.

## Reviewer findings and follow-up

- No sub-agent reviewers were opened in this task.
- Local focused review found one uniform root cause:
  the shared hot-plug verdict lane widened, but only the Stage D avatar/package sidecars had not been refreshed.
- Follow-up performed:
  refreshed only the stale sidecars and revalidated the whole Stage D bundle.

## Skipped validations and reasons

- No additional broad practical alpha-1 or full Stage F validation suite was run.
  Reason: this package was a narrow Stage D alpha-0 freshness repair; the focused Stage D floor plus docs/source-hierarchy/fmt/diff was sufficient and directly relevant.
- No extra Rust crates outside the Stage D floor were re-run.
  Reason: no Rust source semantics changed; only exact expected sidecars and snapshot docs changed.

## Commit / push status

- Main package commit `aa78c60` (`mirrorea: close p-a0-29 stage-d freshness repair`) was pushed to `origin/main`.
- This report status block is being finalized in a docs-only follow-up commit after the package push.

## Sub-agent session close status

No sub-agent sessions were opened in this task due the current delegation policy for this turn. Local focused review was completed instead.
