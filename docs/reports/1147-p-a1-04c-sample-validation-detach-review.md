# Report 1147 — P-A1-04c Sample / Validation Detach Review

- Date: 2026-05-03
- Author / agent: Codex
- Scope: review `P-A1-04c` from the practical sample / expected-artifact / validation-test perspective, with focus on the minimum new coverage for `HP-A1-07` and how to prove detach behavior depends on carrier content rather than sample ID
- Decision levels touched: none

## Objective

Return concrete findings and recommended test cases for the narrowest honest `HP-A1-07` detach-minimal package inside `P-A1-04c`, without overclaiming rollback, migration, Docker/local TCP, save/load, or final public hot-plug ABI completion.

## Scope and assumptions

- Review-only task except for this required report.
- The review is limited to current practical alpha-1 hot-plug code/tests/docs plus the minimum normative and repository-memory anchors needed to judge `HP-A1-07`.
- Current practical hot-plug behavior is treated as attach-only unless a detach-specific carrier field, expected report, and validation path are added.
- `plan/30` helper-local detach evidence is a design anchor, not proof that practical alpha-1 detach is already actualized.

## Start state / dirty state

- start state: `main` at `83c9e13c1e7f6c51424aea127d5f173e9842f4ad`
- worktree was already dirty before this review in:
  - `crates/mir-ast/src/practical_alpha1.rs`
  - `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
- this review adds only this new report file and does not modify the in-progress implementation files above.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/30-attachpoint-detach-minimal-contract.md`
- `plan/44-practical-alpha1-roadmap.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `scripts/practical_alpha1_attach.py`
- `scripts/tests/test_practical_alpha1_attach.py`
- `crates/mir-ast/src/practical_alpha1.rs`
- `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
- `crates/mir-ast/tests/practical_alpha1_hotplug_plan.rs`
- `crates/mir-runtime/src/hotplug_runtime.rs`
- `crates/mir-runtime/src/practical_alpha1_hotplug.rs`
- `crates/mir-runtime/tests/hotplug_runtime_skeleton.rs`
- `crates/mir-runtime/tests/practical_alpha1_hotplug.rs`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `docs/reports/review-1145-p-a1-04b-sample-validation-seam.md`
- `docs/reports/1146-p-a1-04b-practical-hotplug-freshness-object-seam.md`

## Actions taken

1. Read the repository front-door and required specs in AGENTS order, then practical alpha-1 scope / roadmap / sample-matrix anchors.
2. Compared current practical hot-plug fixture inventory, expected reports, Python helper, and Rust tests against the `HP-A1-07` row reserved in the sample matrix.
3. Checked whether the current practical carrier already has detach-specific fields or only attach-specific ones.
4. Compared the practical implementation boundary with the generic hot-plug runtime skeleton and the helper-local detach memory in `plan/30`.

## Files changed

- `docs/reports/1147-p-a1-04c-sample-validation-detach-review.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md
sed -n '1,260p' .agents/skills/discord-report/SKILL.md
rg --files -g 'README.md' -g 'Documentation.md' -g 'progress.md' -g 'tasks.md' -g 'samples_progress.md' -g 'specs/**' -g 'plan/**' -g 'sub-agent-pro/**'
git status --short
rg -n "P-A1-04c|HP-A1-07" README.md Documentation.md progress.md tasks.md specs plan docs samples sub-agent-pro
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,240p' specs/01-charter-and-decision-levels.md
sed -n '1,260p' specs/02-system-overview.md
sed -n '1,260p' specs/03-layer-model.md
sed -n '1,240p' specs/09-invariants-and-constraints.md
sed -n '1,280p' specs/16-runtime-package-adapter-hotplug.md
sed -n '1,260p' specs/18-practical-alpha1-scope.md
sed -n '1,320p' plan/00-index.md
sed -n '1,260p' plan/30-attachpoint-detach-minimal-contract.md
sed -n '1,340p' plan/44-practical-alpha1-roadmap.md
sed -n '1,220p' sub-agent-pro/alpha-1/08-sample-matrix.md
sed -n '1,260p' scripts/practical_alpha1_attach.py
sed -n '1,260p' scripts/tests/test_practical_alpha1_attach.py
sed -n '1,320p' crates/mir-ast/tests/practical_alpha1_hotplug_plan.rs
sed -n '1,420p' crates/mir-runtime/tests/practical_alpha1_hotplug.rs
sed -n '1,340p' crates/mir-ast/src/practical_alpha1_hotplug_plan.rs
sed -n '1,760p' crates/mir-runtime/src/practical_alpha1_hotplug.rs
sed -n '1,320p' crates/mir-runtime/src/hotplug_runtime.rs
sed -n '1,220p' crates/mir-runtime/tests/hotplug_runtime_skeleton.rs
sed -n '1,220p' samples/practical-alpha1/packages/README.md
sed -n '1,220p' samples/practical-alpha1/expected/README.md
find samples/practical-alpha1/packages -maxdepth 1 -mindepth 1 -type d | sort
find samples/practical-alpha1/expected -maxdepth 1 -type f | sort
sed -n '1,220p' samples/practical-alpha1/packages/hp-a1-04b1-stale-membership-attach-rejected/package.mir.json
sed -n '1,220p' samples/practical-alpha1/packages/hp-a1-04b2-missing-witness-attach-rejected/package.mir.json
sed -n '1,220p' samples/practical-alpha1/packages/hp-a1-06-object-package-attach/package.mir.json
rg -n "sample_id|detach|operation_kind|pre_attach_membership_advances|required_witness_refs" crates/mir-ast/src/practical_alpha1.rs crates/mir-ast/src/practical_alpha1_hotplug_plan.rs crates/mir-runtime/src/practical_alpha1_hotplug.rs scripts/practical_alpha1_attach.py crates/mir-ast/tests/practical_alpha1_hotplug_plan.rs crates/mir-runtime/tests/practical_alpha1_hotplug.rs scripts/tests/test_practical_alpha1_attach.py
find docs/reports -maxdepth 1 -type f -name '*.md' | sed 's#.*/##' | sort | tail -n 20
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
date '+%Y-%m-%d %H:%M:%S %z %Z'
```

## Evidence / outputs / test results

- `sub-agent-pro/alpha-1/08-sample-matrix.md` reserves only one named practical detach row: `HP-A1-07`.
- `specs/18-practical-alpha1-scope.md` requires the package/hot-plug stage to make detach explicit through at least one of reject / defer / fallback.
- `plan/44-practical-alpha1-roadmap.md` promotes `P-A1-04c` as the remaining in-stage package for detach minimal contract.
- Current practical fixture and expected roots contain `HP-A1-01..05`, `HP-A1-04B1`, `HP-A1-04B2`, and `HP-A1-06`, but no `HP-A1-07`.
- Current practical package schema and hot-plug plan still expose only attach-shaped carrier fields such as `attach_profile`, `pre_attach_membership_advances`, and `activation_cut_ref`; there is no practical detach operation field or detach-boundary carrier in the current actualized surface.
- `crates/mir-runtime/src/hotplug_runtime.rs` already supports generic detach engine states for `accepted`, `rejected`, and `deferred`, so the missing piece is the practical carrier/report surface, not the generic runtime-skeleton enum mapping.
- Existing content-driven proof is partial: current runtime tests mutate fixture contents while keeping the same `sample_id`, but there is no complementary test that changes only `sample_id` while preserving carrier content.
- No Cargo or Python validation commands were executed in this review-only task.

## What changed in understanding

- The narrowest missing item for `HP-A1-07` is not “just add another sample directory”. The current practical alpha-1 hot-plug carrier is attach-only, so a detach row needs a detach-shaped carrier seam first.
- Only one new named row is required by the current sample matrix, but one named row alone is not enough to prove semantics are content-driven.
- The generic runtime skeleton already has enough detach state vocabulary to support a minimal practical row; the real review risk is overfitting the practical layer to fixture/sample naming instead of explicit carrier fields.

## Open questions

- Which single detach outcome is the narrowest honest first cut for `HP-A1-07`: `rejected`, `deferred`, or `accepted` with explicit kept-later migration notes?
- Should practical alpha-1 widen the carrier with detach-specific names (`operation_kind`, `detach_boundary_ref`, `target_patch_ref`) or generalize current attach-only names before adding the row?
- If `P-A1-04c` closes `PA1-4`, should `stage_pa1_4_complete` flip to `true` while `object_attach_claimed` remains `false` as the non-final object-preview stop line?

## Suggested next prompt

Implement `P-A1-04c` with one narrow `HP-A1-07` detach package fixture plus one exact expected report, add detach-specific carrier fields instead of reusing attach-only names, and include paired mutation tests that show the verdict changes when detach carrier content changes and does not change when only `sample_id` changes.

## Plan update status

`plan/` 更新不要: review-only taskであり repository memory の事実関係は変更していない。

## Documentation.md update status

`Documentation.md` 更新不要: review-only taskであり current snapshot の叙述は変更していない。

## progress.md update status

`progress.md` 更新不要: review-only taskであり current progress snapshot は変更していない。

## tasks.md update status

`tasks.md` 更新不要: review-only taskであり current task map は変更していない。

## samples_progress.md update status

`samples_progress.md` 更新不要: review-only taskであり dashboard snapshot は変更していない。

## Reviewer findings and follow-up

- Finding 1:
  `HP-A1-07` cannot be justified by a new expected JSON alone because the current practical carrier is still attach-only. `PracticalAlpha1AlphaLocalHotPlugInput` exposes `pre_attach_membership_advances` and no detach operation selector, while `build_hotplug_runtime_report` hard-codes `operation_kind = "attach"`. This means any detach claim would currently be sample-name theater rather than carrier-backed behavior. Relevant anchors: `crates/mir-ast/src/practical_alpha1.rs`, `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`, `crates/mir-runtime/src/practical_alpha1_hotplug.rs`.

- Finding 2:
  The minimum new named coverage is exactly one practical row, not a family expansion. The sample matrix reserves a single row `HP-A1-07`, and `specs/18` only requires at least one explicit detach outcome among reject / defer / fallback. Therefore the narrowest honest filesystem addition is:
  - one fixture directory `samples/practical-alpha1/packages/hp-a1-07-.../`
  - one exact expected report `samples/practical-alpha1/expected/hp-a1-07-....expected.json`
  - one new entry in `scripts/practical_alpha1_attach.py`
  Anything beyond that should correspond to an actually claimed broadened detach surface, not be added “for completeness”.

- Finding 3:
  The exact expected report for `HP-A1-07` must assert detach semantics directly. At minimum it should pin:
  - `hotplug_runtime_report.request.operation_kind = "detach"`
  - `hotplug_runtime_report.verdict.verdict_kind = <chosen outcome>`
  - `terminal_outcome`
  - `reason_family`
  - detach-specific reason refs or defer refs
  - the retained-later boundary around rollback / migration / final ABI
  Without those keys, the artifact would not prove practical detach semantics; it would only prove another attach-shaped report with a detach-themed filename.

- Finding 4:
  Current content-driven tests are necessary but incomplete for the user’s stated proof goal. `crates/mir-runtime/tests/practical_alpha1_hotplug.rs` already keeps `sample_id` fixed and mutates freshness/witness carrier content, which shows content matters. It does not show that `sample_id` does not matter. `P-A1-04c` should add both:
  - a same-`sample_id`, different-carrier-content mutation test
  - a different-`sample_id`, same-carrier-content mutation test
  The second test is the missing proof against sample-ID keyed semantics.

- Finding 5:
  The strongest proof should stay at the library/path boundary, not only at the Python row registry. `scripts/practical_alpha1_attach.py` is intentionally sample-ID keyed for inventory lookup, so tests that only call `run_sample("HP-A1-07")` cannot prove semantic independence from sample ID. The proof needs direct-path and/or inline-package tests against:
  - `build_practical_alpha1_hotplug_plan_path`
  - `attach_practical_alpha1_package_path`
  and optionally `attach_practical_alpha1_package` over an inline mutated package value.

- Finding 6:
  The generic runtime skeleton already supports detach `accepted` / `rejected` / `deferred` state mapping. Because of that, the minimal new Rust validation surface for `HP-A1-07` is narrower than a new subsystem:
  - one plan test proving detach carrier fields lower into a practical hot-plug plan
  - one runtime equality test against the exact expected `HP-A1-07` report
  - one content mutation test that flips the outcome
  - one sample-ID mutation test that preserves the outcome
  No broader detach lifecycle suite is required unless `P-A1-04c` claims more than a single minimal outcome.

- Finding 7:
  The Python helper/tests need one closeout-level assertion update if `P-A1-04c` closes `PA1-4`. `scripts/practical_alpha1_attach.py` currently hard-codes no `HP-A1-07`, `stage_pa1_4_complete = False`, and a validation floor that does not mention detach. If `HP-A1-07` lands, the helper should:
  - add the new row to `IMPLEMENTED_ROWS`
  - add a direct-path `check` command for the new package
  - update `check-all` / `closeout`
  - test the new completion bit behavior in `scripts/tests/test_practical_alpha1_attach.py`

- Recommended test cases:
  1. `crates/mir-ast/tests/practical_alpha1_hotplug_plan.rs`
     Add `practical_hotplug_plan_lowers_detach_minimal_fixture` asserting the new detach carrier fields survive lowering and remain on the practical hotplug-plan scope.
  2. `crates/mir-runtime/tests/practical_alpha1_hotplug.rs`
     Add `practical_hotplug_matches_detach_minimal_row` asserting exact equality with `hp-a1-07-*.expected.json` and checking `operation_kind = "detach"` plus the chosen outcome.
  3. `crates/mir-runtime/tests/practical_alpha1_hotplug.rs`
     Add `practical_detach_outcome_is_driven_by_detach_carrier_fields_not_sample_id` by mutating the detach-driving carrier field while keeping `sample_id = "HP-A1-07"` unchanged.
  4. `crates/mir-runtime/tests/practical_alpha1_hotplug.rs`
     Add `practical_detach_outcome_is_not_selected_by_sample_id` by cloning the same fixture, changing only `sample_id`, and asserting identical semantic fields apart from explicitly sample-id-derived echo fields.
  5. `scripts/tests/test_practical_alpha1_attach.py`
     Add a direct package-path `check` test for the new fixture and a `closeout` test that the validation floor and completion bits mention `HP-A1-07`.
  6. Optional but high-signal:
     Add an inline-package test against `attach_practical_alpha1_package` to avoid filesystem/sample-registry coupling entirely when proving sample-ID independence.

## Skipped validations and reasons

- Cargo and Python validation commands were not run because the user asked for a review of minimal sample / expected-artifact / test coverage, not an implementation or regression verification pass.
- No detach prototype was executed because current gaps were provable from the schema / fixture / helper / test inventory.

## Commit / push status

Pending at report write. No commit created for this review-only task.

## Sub-agent session close status

- No sub-agents used.
