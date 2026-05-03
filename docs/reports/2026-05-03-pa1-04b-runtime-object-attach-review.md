# 2026-05-03 P-A1-04b runtime/object-attach review

## 1. Title and identifier

- Title: P-A1-04b runtime/object-attach review
- Identifier: `2026-05-03-pa1-04b-runtime-object-attach-review`

## Objective

- Review `P-A1-04b` implementation seams for:
  - practical stale-membership typed reject
  - practical missing-witness typed reject
  - a narrow reusable slice for `HP-A1-06` object package attach
- Keep the reading aligned with `specs/18-practical-alpha1-scope.md` and current repo memory.

## Scope and assumptions

- Scope is review-only. No implementation changes were made.
- Normative source is `specs/18-practical-alpha1-scope.md`.
- `sub-agent-pro/alpha-1/08-sample-matrix.md` is treated as handoff memory, not normative source.
- Working assumption:
  `HP-A1-06` means a narrow front-door practical object/runtime-package attach floor, not full avatar/native/runtime lifecycle completion.

## Start state / dirty state

- Start state: repository worktree appeared clean in `git status --short`.
- No pre-existing dirty tracked change needed accommodation during this review.

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/44-practical-alpha1-roadmap.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `samples/practical-alpha1/README.md`
- `crates/mir-runtime/src/practical_alpha1_hotplug.rs`
- `crates/mir-runtime/src/alpha_avatar_runtime.rs`
- `crates/mirrorea-core/src/fabric.rs`
- Supporting code for seam inspection:
  - `crates/mir-ast/src/practical_alpha1.rs`
  - `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
  - `crates/mir-runtime/src/alpha_layer_insertion_runtime.rs`
  - `crates/mir-runtime/src/hotplug_runtime.rs`
  - `crates/mir-runtime/src/practical_alpha1_local_runtime.rs`
  - `crates/mir-runtime/src/alpha_network_runtime.rs`
  - `crates/mirrorea-core/src/runtime.rs`
  - `crates/mir-runtime/tests/practical_alpha1_hotplug.rs`
  - `crates/mir-runtime/tests/hotplug_runtime_skeleton.rs`

## Actions taken

- Read required repository front-door documents in repo-specified order.
- Read the practical alpha-1 spec, roadmap memory, and named handoff.
- Inspected practical hot-plug runtime/report code, avatar/package report code, and hot-plug carrier definitions.
- Compared hot-plug carrier lanes against local-runtime and network typed negative implementations.
- Reviewed current practical hot-plug tests and sample-root boundaries.
- Derived a minimal recommended implementation path and explicit non-reuse boundaries.

## Files changed

- Added this report:
  - `docs/reports/2026-05-03-pa1-04b-runtime-object-attach-review.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `sed -n '1,220p' ...` / `sed -n '1,260p' ...` over required docs and sample/package files
- `nl -ba ... | sed -n '...'` over target Rust files and tests
- `rg -n ...` over practical alpha-1, runtime, and core files
- `find samples/practical-alpha1/packages -maxdepth 2 -name 'package.mir.json' | sort`
- `git status --short`
- `date '+%Y-%m-%d %H:%M:%S %z'`

## Evidence / outputs / test results

- `git status --short`: no visible tracked worktree changes at review start.
- `date`: `2026-05-03 18:36:45 +0900`
- No validation command or test suite was executed in this review-only task.
- Key evidence anchors:
  - `crates/mir-runtime/src/practical_alpha1_hotplug.rs` hard-codes `membership_epoch: 0` and `member_incarnation: 0` into the layer attach request while never deriving them from front-door input.
  - `crates/mir-runtime/src/alpha_layer_insertion_runtime.rs` emits `membership_frontier_verified` / `membership_epoch_current` without comparing offered freshness against runtime snapshot.
  - `crates/mirrorea-core/src/fabric.rs` has no witness-specific reject lane on `HotPlugVerdict`.
  - `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs` rejects all non-`layer` package kinds in the current hot-plug plan floor.
  - `crates/mir-runtime/src/alpha_avatar_runtime.rs` is scenario-driven and hard-coded, not front-door package driven.

## What changed in understanding

- The current `P-A1-04a` hot-plug floor is narrower than the surface names alone suggest:
  it carries membership/witness references but does not semantically evaluate them.
- Stale-membership negative is not blocked by missing runtime substrate. The substrate already exists in `LogicalPlaceRuntimeShell`; the block is carrier shape plus unused request fields.
- Missing-witness negative is blocked more fundamentally than stale-membership:
  the current hot-plug verdict carrier has no dedicated witness reject lane, so any implementation would otherwise collapse witness failure into compatibility or authorization.
- `HP-A1-06` does have a narrow potential reuse path, but not by directly calling `alpha_avatar_runtime` as-is. The reusable slice is only the report-local package-admission pattern, not the scenario-based avatar bridge.

## Open questions

- OPEN QUESTION:
  For `HP-A1-06`, should the first practical object package attach row target a generic `object` package kind, or should it intentionally mirror the existing avatar/runtime-package bridge as an alpha-local representative object family?
- OPEN QUESTION:
  Should missing-witness reject be modeled with a new `witness_reason_refs` lane on `HotPlugVerdict`, or via a more general typed-lane refactor that also separates capability from authorization?
- OPEN QUESTION:
  Should failing practical manifest checks reject at hot-plug plan build time, or stay runtime-visible as a typed attach reject?

## Suggested next prompt

- `Implement the minimal P-A1-04b carrier changes you recommended: add explicit hot-plug freshness input and witness reject lane, keep layer-only scope, add failing tests first, and stop before HP-A1-06 object attach.`

## Plan update status

- `plan/` 更新不要
- Reason: this task produced review findings only; no roadmap or long-lived repository memory changed yet.

## Documentation.md update status

- `Documentation.md` 更新不要
- Reason: no repo snapshot claim changed in this review-only task.

## progress.md update status

- `progress.md` 更新不要
- Reason: no package was advanced or closed.

## tasks.md update status

- `tasks.md` 更新不要
- Reason: the current promoted line already names `P-A1-04b` as next work.

## samples_progress.md update status

- `samples_progress.md` 更新不要
- Reason: no sample row changed and no validation was run.

## Reviewer findings and follow-up

- Finding 1:
  stale-membership typed reject is not implementable on the current practical hot-plug path without carrier changes, because the front-door input does not carry offered freshness and the runtime path does not compare request freshness against the runtime snapshot.
- Follow-up:
  add `membership_epoch` and `member_incarnation` to `PracticalAlpha1AlphaLocalHotPlugInput` and `PracticalAlpha1HotPlugPlan`, then evaluate them in the layer-attach runtime before computing the verdict.
- Finding 2:
  missing-witness typed reject is not implementable as a typed lane on the current hot-plug verdict carrier, because `HotPlugVerdict` has compatibility / authorization / membership freshness lanes only.
- Follow-up:
  add a witness-specific reject lane or equivalent typed carrier field before implementing `HP-A1-04b`.
- Finding 3:
  current manifest checks are informational only and do not gate attach acceptance, so object package attach must not reuse this path unchanged.
- Follow-up:
  convert failed manifest admission checks into explicit reject outcome before widening package kinds.
- Finding 4:
  `alpha_avatar_runtime` is not directly reusable for practical object attach because it is scenario-ID keyed and report-local, but its package-admission shape is a plausible extraction target.
- Follow-up:
  extract a pure `runtime_package_admission` helper instead of calling avatar scenario builders from practical hot-plug.

## Skipped validations and reasons

- Skipped `cargo test` and helper validation commands.
- Reason: user requested a maintainer-style implementation seam review, not an implementation or verification pass.
- Therefore no success claim is made for any runtime or sample path.

## Commit / push status

- No commit created.
- No push performed.

## Sub-agent session close status

- No sub-agent session was opened for this task.
- Review session closed at report creation.
