# 1049 - Non-hot-plug research abstract wording cooling

## Objective

Active non-hot-plug `docs/research_abstract/` pages should read as repository-memory summaries, not as live task queues or public-freeze claims.

## Scope and assumptions

- Scope is reader-facing wording maintenance in active `docs/research_abstract/` pages.
- `docs/research_abstract/old/` is historical archive and was intentionally not edited.
- Normative specs, long-lived `plan/` semantics, sample taxonomy, scripts, and Rust implementation are unchanged.
- Live status / reopen authority remains in `progress.md` and `tasks.md`.
- This package does not claim final public parser/API/ABI, final viewer/adapter/projection/transport surface, final auth, packaging, installed binary, or engine adapter completion.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
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
- `samples/README.md`
- `scripts/README.md`
- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`
- active files under `docs/research_abstract/`, excluding `docs/research_abstract/old/`

## Actions taken

- Replaced active summary headings such as `current reading`, `current recommendation`, `current status`, and `current rule` with repository-memory, boundary, guardrail, or sample-split wording where they implied live status authority.
- Cooled `docs/research_abstract/avatar_fairy_follow_plan_01.md` to separate active representative slice, residual planned family, repository-memory provisional recommendation, and helper-local planning surface.
- Cooled `docs/research_abstract/typed_external_boundary_adapter_plan_01.md` to use evidence anchors, executable cut, boundary rule, planned-state table wording, and `progress.md` / `tasks.md` live reopen authority.
- Cooled public-boundary, `U1`, verification, repository structure, viewer, backend, attachpoint, FAIRY-05, projection, network, and phase summaries where `current` wording could be read as live queue ownership.
- Updated `docs/research_abstract/README.md` and `docs/research_abstract/mirrorea_future_axis_01.md` to use repository-memory / snapshot wording while preserving current-layer non-claims.
- Updated `progress.md` last updated and recent log for this package.

## Files changed

- `docs/research_abstract/README.md`
- `docs/research_abstract/attachpoint_detach_minimal_contract_01.md`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
- `docs/research_abstract/clean_near_end_modal_01.md`
- `docs/research_abstract/clean_near_end_order_model_01_detail.md`
- `docs/research_abstract/compiler_backend_llvm_preparation_01.md`
- `docs/research_abstract/fairy05_visibility_return_carrier_bundling_01.md`
- `docs/research_abstract/hotplug_attachpoint_plan_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/network_transport_plan_01.md`
- `docs/research_abstract/phase3-parser-boundary-and-first-checker-cut.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `docs/research_abstract/post_p18_true_user_spec_hold_option_matrix_01.md`
- `docs/research_abstract/projection_placement_plan_01.md`
- `docs/research_abstract/public_api_parser_gate_plan_01.md`
- `docs/research_abstract/repository_layer_structure_01.md`
- `docs/research_abstract/runtime_crate_hotplug_engine_ownership_cut_01.md`
- `docs/research_abstract/typed_external_boundary_adapter_plan_01.md`
- `docs/research_abstract/verification_layer_widening_threshold_01.md`
- `docs/research_abstract/visual_debugger_viewer_plan_01.md`
- `progress.md`
- `docs/reports/1049-non-hotplug-research-abstract-wording-cooling.md`

## Evidence / outputs / test results

- `git status --short` at package start was clean after commit `3d528ce`.
- `git branch --show-current` -> `main`
- `git log -1 --oneline` -> `3d528ce Cool research abstract hotplug trilogy wording`
- `python3 scripts/check_source_hierarchy.py` -> pass; required 23 / present 23 / missing 0.
- `python3 scripts/validate_docs.py` -> pass after report creation; documentation scaffold complete, 1047 numbered reports found.
- `git diff --check` -> pass.
- Targeted active research-abstract wording check returned no matches:
  `rg -n "## current reading|## current recommendation|current recommendation|current summary|current queue|remaining open gate|next open work|promoted next package|current closeout|current repository memory|next narrow implementation line|first recommendation|second recommendation|third recommendation|current status|Current status|Current indirect anchor|promoted queue|current role|current executable cut|current rule|current evidence anchors" docs/research_abstract -g '!**/old/**'`

## What changed in understanding

- Active research abstracts benefit from the same authority split as hands-on pages: repository-memory summaries can mention current-layer evidence, but live queue authority must stay in `progress.md` / `tasks.md`.
- `docs/research_abstract/old/` should not be swept by broad wording maintenance because its historical wording is part of archive evidence.
- `current line` can remain only where it denotes the repo-local alpha-ready current layer rather than a live task queue; this package removed the queue-shaped forms.

## Open questions

- Actual `U1` commitment remains open and user-facing.
- A later package can audit `plan/` wording for similar reader-facing temperature, but this package intentionally avoided changing long-lived repository memory semantics.

## Suggested next prompt

Continue autonomous maintenance with a `plan/` temperature audit focused on active reader-facing summaries, preserving `plan/` as repository memory and avoiding semantic rewrites.

## Plan update status

`plan/` 更新不要。No long-lived repository-memory interpretation changed.

## progress.md update status

`progress.md` 更新済み。Last updated and recent log now record the non-hot-plug research abstract wording cooling.

## tasks.md update status

`tasks.md` 更新不要。Current task map remains maintenance lane plus actual `U1` commitment gate.

## samples_progress.md update status

`samples_progress.md` 更新不要。No runnable sample, validation command, debug surface, blocker, or progress percentage changed.

## Skipped validations and reasons

- Full validation floor was not rerun because this package changed only reader-facing docs wording and `progress.md`.
- Cargo tests and sample closeouts were not rerun because no Rust code, sample source, or script behavior changed.

## Commit / push status

Pending at report write. This report is intended to be committed with the package using `git commit --no-gpg-sign` and pushed immediately after staged diff validation.

## Sub-agent session close status

Docs researcher sub-agent `019de0d3-0504-7c03-8233-39e7787b8b2a` audited non-hot-plug research abstract wording, returned residual candidates, and was closed after incorporation.
