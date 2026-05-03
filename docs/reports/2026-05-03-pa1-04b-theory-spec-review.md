# 2026-05-03 P-A1-04b Theory/Spec Review

## 1. Title and identifier

- `2026-05-03-pa1-04b-theory-spec-review`

## Objective

- Review package `P-A1-04b` only for scope semantics and overclaim risk.
- Focus:
  - exact semantics required to safely actualize stale-membership reject and missing-witness reject in the practical hot-plug floor
  - narrowest safe reading for an object-attach seam without claiming final avatar/package/runtime completion
  - what must remain non-claimed in docs/report

## Scope and assumptions

- Review-only task.
- No implementation change requested.
- Normative basis taken from `specs/18-practical-alpha1-scope.md`, with alpha-0 boundary checks from `specs/16-runtime-package-adapter-hotplug.md` and `specs/17-mirrorea-spaces-alpha-scope.md`.
- Repository memory basis taken from `plan/44-practical-alpha1-roadmap.md`.
- Current code anchor reviewed: `crates/mir-runtime/src/practical_alpha1_hotplug.rs`, plus delegated carriers in `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`, `crates/mir-runtime/src/alpha_layer_insertion_runtime.rs`, and `crates/mirrorea-core/src/fabric.rs`.

## Start state / dirty state

- Working tree was clean at review start.
- No pre-existing dirty changes were observed in `git status --short`.

## Documents consulted

- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `crates/mir-ast/src/practical_alpha1.rs`
- `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
- `crates/mir-runtime/src/practical_alpha1_hotplug.rs`
- `crates/mir-runtime/src/alpha_layer_insertion_runtime.rs`
- `crates/mir-runtime/src/hotplug_runtime.rs`
- `crates/mir-runtime/src/alpha_local_runtime.rs`
- `crates/mir-runtime/src/alpha_network_runtime.rs`
- `crates/mir-runtime/src/alpha_avatar_runtime.rs`
- `crates/mirrorea-core/src/fabric.rs`
- `crates/mir-runtime/tests/practical_alpha1_hotplug.rs`

## Actions taken

- Read the user-specified handoff first, then the repository-required reading stack.
- Traced the current practical hot-plug path:
  - front door -> practical hotplug plan -> practical hotplug report
  - delegated layer insertion runtime
  - delegated hotplug request/verdict carrier
- Compared `specs/18` required package/hot-plug completion semantics against current `P-A1-04a` stop lines and `P-A1-04b` roadmap wording.
- Checked analogous stale-membership and missing-witness semantics in current alpha local/network runtime code to determine the minimum safe actualization shape.
- Checked sample inventory and tests to confirm that `HP-A1-06` and freshness/witness negative rows are not yet actualized in the practical sample root.

## Files changed

- New report only:
  - `docs/reports/2026-05-03-pa1-04b-theory-spec-review.md`

## Commands run

```bash
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md
sed -n '1,220p' .agents/skills/discord-report/SKILL.md
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
rg --files specs plan sub-agent-pro | sort | sed -n '1,220p'
nl -ba sub-agent-pro/alpha-1/08-sample-matrix.md | sed -n '1,260p'
nl -ba README.md | sed -n '1,260p'
nl -ba Documentation.md | sed -n '1,260p'
nl -ba specs/00-document-map.md | sed -n '1,260p'
nl -ba specs/01-charter-and-decision-levels.md | sed -n '1,260p'
nl -ba specs/02-system-overview.md | sed -n '1,260p'
nl -ba specs/03-layer-model.md | sed -n '1,260p'
nl -ba specs/09-invariants-and-constraints.md | sed -n '1,260p'
nl -ba plan/00-index.md | sed -n '1,320p'
nl -ba specs/16-runtime-package-adapter-hotplug.md | sed -n '1,320p'
nl -ba specs/17-mirrorea-spaces-alpha-scope.md | sed -n '110,220p'
nl -ba specs/18-practical-alpha1-scope.md | sed -n '1,320p'
nl -ba plan/44-practical-alpha1-roadmap.md | sed -n '1,320p'
nl -ba crates/mir-runtime/src/practical_alpha1_hotplug.rs | sed -n '1,520p'
rg -n "stale_membership|missing_witness|witness|membership_epoch|member_incarnation|object package|object_attach|package_kind|attach_profile" crates/mir-runtime crates/mir-ast specs/18-practical-alpha1-scope.md plan/44-practical-alpha1-roadmap.md specs/16-runtime-package-adapter-hotplug.md specs/17-mirrorea-spaces-alpha-scope.md
nl -ba crates/mir-ast/src/practical_alpha1_hotplug_plan.rs | sed -n '1,260p'
nl -ba crates/mir-runtime/src/alpha_layer_insertion_runtime.rs | sed -n '180,620p'
nl -ba crates/mir-runtime/src/hotplug_runtime.rs | sed -n '1,260p'
nl -ba crates/mir-runtime/tests/practical_alpha1_hotplug.rs | sed -n '1,260p'
nl -ba crates/mir-ast/src/practical_alpha1.rs | sed -n '1,470p'
nl -ba crates/mir-runtime/src/alpha_network_runtime.rs | sed -n '720,880p'
nl -ba crates/mir-runtime/src/alpha_local_runtime.rs | sed -n '600,680p'
nl -ba crates/mir-runtime/src/alpha_avatar_runtime.rs | sed -n '420,560p'
find samples/practical-alpha1/packages -maxdepth 1 -mindepth 1 -type d | sort
find samples/practical-alpha1/expected -maxdepth 1 -type f | sort
git status --short
date '+%Y-%m-%d %H:%M:%S %Z'
```

## Evidence / outputs / test results

- `samples/practical-alpha1/packages/` currently contains `HP-A1-01..05` only. No practical `HP-A1-06` object attach package and no freshness/witness negative packages were present.
- `samples/practical-alpha1/expected/` likewise contains expected reports only for `HP-A1-01..05`.
- `crates/mir-runtime/tests/practical_alpha1_hotplug.rs` exercises only `HP-A1-01..05` plus two malformed-plan negatives; it does not test stale-membership reject, missing-witness reject, or object attach.
- Current practical hot-plug request construction hard-codes `membership_epoch: 0` and `member_incarnation: 0` in [`crates/mir-runtime/src/practical_alpha1_hotplug.rs`](/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/src/practical_alpha1_hotplug.rs:222).
- Current delegated layer runtime always emits `membership_frontier_verified` and `membership_epoch_current` in verdict freshness refs, without comparing offered vs current epoch/incarnation, in [`crates/mir-runtime/src/alpha_layer_insertion_runtime.rs`](/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/src/alpha_layer_insertion_runtime.rs:508).
- Current practical hot-plug plan admits only `layer` packages in [`crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/src/practical_alpha1_hotplug_plan.rs:106), while the front door already admits `object`, `runtime`, `avatar`, and `adapter` package kinds in [`crates/mir-ast/src/practical_alpha1.rs`](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/src/practical_alpha1.rs:440).

## What changed in understanding

- The main `P-A1-04b` risk is not merely missing fixtures; it is that the current hot-plug carrier shape cannot faithfully express the required stale-membership and missing-witness semantics yet.
- The current code is honest about non-claiming through retained-later refs and booleans, but the delegated runtime still overstates freshness by emitting verified/current reason refs without performing the corresponding comparison.
- The narrowest safe object seam is front-door and planning-level separability only. Actual object attach semantics would be a new hot-plug lane, not a small widening of the existing layer-only lane.

## Open questions

- Should practical hot-plug freshness use the same explicit pair as local/network runtime: offered `membership_epoch` plus `member_incarnation`, compared against current `MembershipRegistry`?
- Should practical hot-plug witness semantics distinguish:
  - witness lane non-emptiness
  - required witness set declared by attach profile/package kind
  - exact missing witness refs in reject reason family
- Should object attach in `P-A1-04b` stop at a seam carrier/report shape, or should it also introduce one actual practical `HP-A1-06` runtime package/object sample in the same package?

## Suggested next prompt

- `Implement only the carrier/report widening needed for P-A1-04b stale-membership and missing-witness rejects, keep object attach as seam-only, add focused tests and expected reports, and keep docs/report non-claims explicit.`

## Plan update status

- `plan/` 更新不要

## Documentation.md update status

- `Documentation.md` 更新不要

## progress.md update status

- `progress.md` 更新不要

## tasks.md update status

- `tasks.md` 更新不要

## samples_progress.md update status

- `samples_progress.md` 更新不要

## Reviewer findings and follow-up

- Finding 1:
  Current practical hot-plug code cannot safely claim stale-membership reject semantics. `specs/18` requires missing-witness and stale-membership typed rejects as part of package/hot-plug completion, but `P-A1-04a` explicitly does not claim them yet ([`specs/18-practical-alpha1-scope.md`](/home/yukatayu/dev/mir_poc_01/specs/18-practical-alpha1-scope.md:161), [`plan/44-practical-alpha1-roadmap.md`](/home/yukatayu/dev/mir_poc_01/plan/44-practical-alpha1-roadmap.md:116)). The current practical attach path fixes `membership_epoch` and `member_incarnation` to `0` and never derives them from package input or compares them to runtime state ([`practical_alpha1_hotplug.rs`](/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/src/practical_alpha1_hotplug.rs:222), [`alpha_layer_insertion_runtime.rs`](/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/src/alpha_layer_insertion_runtime.rs:508)). Worse, the delegated runtime verdict always reports freshness refs as verified/current even on any future rejection path ([`alpha_layer_insertion_runtime.rs`](/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/src/alpha_layer_insertion_runtime.rs:548)). Safe actualization therefore requires:
  - offered freshness fields in the practical hot-plug input/carrier
  - runtime-side comparison against active membership epoch and member incarnation
  - reject-before-activation on epoch drift, incarnation drift, inactive participant, or participant-place drift
  - freshness-specific reason family and reason refs in the practical report
  Stop line: do not claim `stale_membership_attach_reject` before the carrier contains offered freshness state and the report stops emitting unconditional freshness success refs.

- Finding 2:
  Current practical hot-plug code cannot safely claim missing-witness reject semantics. The plan requires witness negatives in `P-A1-04b`, but the current practical path only checks that `witness_refs` is non-empty at plan/request validation time ([`practical_alpha1_hotplug_plan.rs`](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/src/practical_alpha1_hotplug_plan.rs:141), [`alpha_layer_insertion_runtime.rs`](/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/src/alpha_layer_insertion_runtime.rs:221)). There is no required-vs-actual witness relation, no missing-witness diff, and no witness `reason_family`; `practical_alpha1_hotplug.rs` currently classifies rejects only as `authorization` or `compatibility` ([`practical_alpha1_hotplug.rs`](/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/src/practical_alpha1_hotplug.rs:243)). Safe actualization therefore requires:
  - a declared required witness set for the attach attempt
  - subset comparison between required witnesses and offered witness lane
  - reject-before-activation with exact missing witness refs
  - report/verdict surface that can classify `witness` distinctly from authorization/compatibility
  Stop line: do not claim `missing_witness_attach_reject` until witness sufficiency is modeled as a real semantic check, not only non-empty lane presence.

- Finding 3:
  The narrowest safe reading for object attach is seam-only, not actualized object/package/runtime/avatar completion. The practical front door already admits `object` package kinds ([`practical_alpha1.rs`](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/src/practical_alpha1.rs:449)), but the practical hot-plug plan rejects every non-`layer` package ([`practical_alpha1_hotplug_plan.rs`](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/src/practical_alpha1_hotplug_plan.rs:106)). The runtime report itself keeps `object_attach_claimed`, `freshness_negative_complete`, and `stage_pa1_4_complete` all `false` ([`practical_alpha1_hotplug.rs`](/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/src/practical_alpha1_hotplug.rs:297)). Safe wording for `P-A1-04b` is therefore:
  - front-door/package-kind seam exists
  - practical hot-plug still has a layer-only actualized path
  - object attach semantics remain later unless a distinct object/package carrier plus sample/report is introduced
  Stop line: do not describe a seam-only widening as object package attach completion, avatar/runtime package completion, or stage `PA1-4` completion.

## Skipped validations and reasons

- No cargo/python validation was run.
- Reason:
  - review-only task
  - no implementation change was made
  - the requested deliverable was semantic/spec review, not behavioral regression verification

## Commit / push status

- No commit created.
- No push performed.

## Sub-agent session close status

- No sub-agent session used.
