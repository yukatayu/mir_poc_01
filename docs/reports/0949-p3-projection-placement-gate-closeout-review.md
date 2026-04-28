# 0949 — P3 projection / placement gate closeout review

## Objective

Review the current working tree for the `P3` Projection / placement residual emitted-program gate closeout, with emphasis on scope overclaim, queue drift, validation-evidence mismatch, generated reserve path usage, and `P3 closed / P4 next / P5 reopen next` consistency.

Inspection timestamp: 2026-04-28 09:55 JST

## Scope and assumptions

- Scope is limited to the current working tree and the user-listed files.
- This task is a maintainer-style review, not a docs closeout or normative rewrite.
- `progress.md` 更新不要
- `tasks.md` 更新不要
- `samples_progress.md` 更新不要
- `plan/` 更新不要

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/20-projection-and-placement-roadmap.md`
- `plan/90-source-traceability.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/projection_placement_views_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/projection_placement_plan_01.md`
- `samples/README.md`
- `samples/generated/README.md`
- `docs/reports/0947-p3-projection-placement-wording-drift-inspection.md`
- `docs/reports/0948-p3-projection-emitted-program-gate-closeout.md`

## Actions taken

1. Read the required repository sequence before reviewing the scoped files.
2. Diffed the user-listed files against `HEAD`.
3. Extracted `P3` / `P4` / `P5` / `P15` wording and compared it across roadmap, snapshot, hands-on, and report layers.
4. Checked whether the new generated-reserve wording is backed by the listed command surfaces.
5. Ran focused verification commands for hierarchy, docs scaffold, whitespace hygiene, and the two projection preview anchors.
6. Wrote this review report without changing the reviewed snapshot documents.

## Evidence / outputs / test results

- Commands run:
  - `git diff -- README.md Documentation.md progress.md tasks.md samples_progress.md plan/20-projection-and-placement-roadmap.md plan/01-status-at-a-glance.md plan/11-roadmap-near-term.md plan/17-research-phases-and-autonomy-gates.md plan/90-source-traceability.md specs/11-roadmap-and-workstreams.md docs/hands_on/current_phase_closeout_01.md docs/hands_on/projection_placement_views_01.md docs/research_abstract/mirrorea_future_axis_01.md docs/research_abstract/projection_placement_plan_01.md samples/README.md samples/generated/README.md docs/reports/0947-p3-projection-placement-wording-drift-inspection.md docs/reports/0948-p3-projection-emitted-program-gate-closeout.md`
  - `rg -n "P3|P4|P5|P15|generated artifact|generated place-specific|emitted place-specific|server/client|projection/codegen|preview floor" ...`
  - `find samples/generated -maxdepth 3 -type f | sort`
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`
  - `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`
  - `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
- Command results:
  - `check_source_hierarchy.py`: pass
  - `validate_docs.py`: pass
  - `git diff --check`: pass
  - projection preview helper anchor: pass
  - `cross_place_projection` runtime anchor: pass
  - `find samples/generated ...`: current output is `samples/generated/README.md` only

## What changed in understanding

- The `P3 -> P4 -> P5` queue promotion is mostly synchronized across the scoped snapshot files.
- The remaining defects are not broad queue drift but two narrower issues:
  - `P15` still carries a server/client framing that is weaker than the newly fixed place-split rule.
  - some hands-on / task-map validation wording now promises more than the listed commands can prove.

## Open questions

- Should `P15` be renamed everywhere from `server/client programs` to `place-specific programs` to match the `P3` boundary that just closed?
- What exact evidence should be required before `P15` can claim generated-program actualization instead of reserve-path policy only?
- Should the top-level closeout guide include the generated-reserve inventory command whenever it claims the `P3` reserve policy is part of the verified closeout?

## Suggested next prompt

Fix the `P15` wording/evidence drift by updating `specs/11-roadmap-and-workstreams.md`, `tasks.md`, and `docs/hands_on/current_phase_closeout_01.md` so that:

- `P15` uses `place-specific program` terminology instead of `server/client programs`
- `P15` validation no longer reuses preview-only commands as if they validated emitted-program actualization
- the top-level closeout hands-on page includes the generated-reserve inventory command if it claims that reserve policy is part of the verified `P3` closeout
