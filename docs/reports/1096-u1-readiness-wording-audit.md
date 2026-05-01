# Report 1096 — U1 readiness wording audit

- Date: 2026-05-01 14:33 JST
- Author / agent: Codex
- Scope: docs-first `U1` readiness wording audit
- Decision levels touched: none; no normative `specs/` decision changed

## Objective

Audit `U1` actual commitment readiness wording and repair narrow stale references so active docs consistently describe the open four-axis user decision gate without committing to a product shape or freezing any public surface.

## Scope and assumptions

- This package is docs-first readiness maintenance only.
- It does not decide `U1`, choose packaging, choose host integration, freeze public API/ABI, adopt installed binary packaging, select backend, or change sample/runtime behavior.
- `U1` remains open and user-facing.
- The current four `U1` axes are packaging / installed binary target, host integration target, first shipped public surface scope, and final shared-space operational catalog breadth.

## Start state / dirty state

- Package start state was clean.
- Branch was `main`.
- Prior commit was `1ea59b6 Record post-guardrail full validation`.
- Work started at `2026-05-01 14:29 JST`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/06-prismcascade-positioning.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- `plan/38-post-p21-final-public-hotplug-abi-family.md`
- relevant `plan/01..11` and `plan/18..38`
- `samples/README.md`
- `scripts/README.md`
- `docs/hands_on/post_p18_true_user_spec_hold_01.md`
- `docs/hands_on/public_api_parser_gate_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/public_api_parser_gate_plan_01.md`
- `docs/research_abstract/post_p18_true_user_spec_hold_option_matrix_01.md`
- `docs/research_abstract/README.md`
- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`

## Actions taken

- Spawned a docs researcher sub-agent to audit `U1` readiness wording and non-claim boundaries.
- Replaced stale `broader application target` / `exhaustive shared-space operational catalog` wording in active entry docs and `plan/27` with the current `final shared-space operational catalog breadth` axis.
- Added the missing `final shared-space operational catalog breadth` item to `README.md` and `docs/research_abstract/README.md` deferred lists.
- Updated `plan/27-public-api-parser-gate-roadmap.md` so the public-gate validation floor separates executable anchors from optional inventory / runtime corroboration lanes.
- Updated `docs/hands_on/post_p18_true_user_spec_hold_01.md` so the command sequence prints both user decision blockers and research-discovery items from `tasks.md`.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` with this readiness audit.
- Added this report.

## Files changed

- `README.md`
- `docs/research_abstract/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/post_p18_true_user_spec_hold_01.md`
- `docs/research_abstract/public_api_parser_gate_plan_01.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/1096-u1-readiness-wording-audit.md`

## Commands run

Start / state:

```bash
git status --short
git branch --show-current
git log -1 --oneline
date '+%Y-%m-%d %H:%M %Z'
```

Focused audit:

```bash
sed -n '1,240p' docs/hands_on/post_p18_true_user_spec_hold_01.md
sed -n '1,260p' docs/research_abstract/post_p18_true_user_spec_hold_option_matrix_01.md
sed -n '1,260p' plan/28-post-p18-true-user-spec-hold-option-matrix.md
sed -n '1,240p' plan/27-public-api-parser-gate-roadmap.md
sed -n '1,220p' docs/research_abstract/public_api_parser_gate_plan_01.md
sed -n '1,260p' plan/38-post-p21-final-public-hotplug-abi-family.md
rg -n "broader application target|final shared-space operational catalog|shared-space operational catalog|packaging / installed|host integration|first shipped public surface|network_transport_samples.py closeout|mir-clean-near-end -- closeout" README.md Documentation.md progress.md tasks.md docs/hands_on docs/research_abstract plan/27-public-api-parser-gate-roadmap.md plan/28-post-p18-true-user-spec-hold-option-matrix.md plan/38-post-p21-final-public-hotplug-abi-family.md -S
```

Validation:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- docs researcher found one actionable issue: the `post_p18_true_user_spec_hold_01.md` command stopped at the `## research-discovery items` heading and did not show the research-discovery body.
- docs researcher reported no accidental actual `U1` commitment, public API/ABI freeze overclaim, installed binary/backend/host target overclaim, or stale non-claim boundary in the requested active docs.
- Local audit found stale `U1` axis wording in active docs / `plan/27`: old `broader application target` or `exhaustive shared-space operational catalog` wording was still present where the current gate should be the four-axis `U1` decision.
- `python3 -m unittest scripts.tests.test_validate_docs` passed: `Ran 10 tests ... OK`.
- `python3 scripts/check_source_hierarchy.py` passed: required `35`, present `35`, missing `0`.
- `python3 scripts/validate_docs.py` passed: documentation scaffold complete, `1094` numbered reports.
- `git diff --check` passed.

## What changed in understanding

The open `U1` boundary remains correctly unresolved. The repair needed here was wording alignment: active docs should consistently present `U1` as a four-axis user decision gate and should let readers compare user-decision blockers with research-discovery items directly.

## Open questions

- `U1` actual commitment remains open: packaging / installed binary target, host integration target, first shipped public surface scope, and final shared-space operational catalog breadth.
- No actual product target, public API/ABI freeze, packaging adoption, backend choice, host schema, or shared-space final catalog was selected.

## Suggested next prompt

Continue autonomous maintenance with another narrow docs/readiness audit, or make the actual `U1` choice if the user wants to move from option inventory to product-shape implementation.

## Plan update status

`plan/` 更新済み: `plan/27-public-api-parser-gate-roadmap.md` now mirrors the current four-axis `U1` gate and separates optional inventory/runtime corroboration commands from the public-gate validation floor.

## Documentation.md update status

`Documentation.md` 更新不要: it already stated the current `U1` open gate and non-claim boundaries accurately.

## progress.md update status

`progress.md` 更新済み: recorded the 2026-05-01 14:33 JST `U1` readiness wording audit.

## tasks.md update status

`tasks.md` 更新済み: added the `U1` readiness wording checkpoint while keeping actual `U1` commitment open.

## samples_progress.md update status

`samples_progress.md` 更新済み: added the `U1` readiness wording audit to repository-memory validation.

## Reviewer findings and follow-up

- docs researcher finding: `post_p18_true_user_spec_hold_01.md` did not surface the research-discovery body.
- Follow-up: added a second `sed` command that prints from `## research-discovery items` through the validation floor heading.
- Local finding: `plan/27` and active entry docs retained older `broader application target` / `exhaustive shared-space operational catalog` wording.
- Follow-up: aligned those docs to the current four `U1` axes.
- Local finding: `plan/27` kept inventory / runtime corroboration commands in the main validation floor.
- Follow-up: moved `network_transport_samples.py closeout` and `mir-clean-near-end -- closeout` into an explicit optional corroboration / inventory lane.

## Skipped validations and reasons

- Full sample and Cargo floor was not rerun because this package changed docs wording only. The previous full validation checkpoint remains `1095` / `1ea59b6`.
- No public parser/API/ABI, production transport/replay, production theorem/model-check binding, actual LLVM build/install, storage cleanup, mount, format, or ownership repair validation was attempted.

## Commit / push status

Pending at report write.

## Sub-agent session close status

docs researcher sub-agent `019de203-da16-7b80-bdf5-e6557b3dbec2` completed and was closed.
