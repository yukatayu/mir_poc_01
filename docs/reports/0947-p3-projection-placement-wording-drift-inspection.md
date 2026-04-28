# 0947 — P3 projection/placement wording drift inspection

## Objective

Inspect the current snapshot and roadmap docs for wording drift around `P3` projection / placement, with emphasis on:

- queue / current-status wording
- the minimum file set that a `P3` close report would need to update
- overclaim risk around emitted programs, generated artifacts, and equivalence checking

Inspection timestamp: 2026-04-28 09:39 JST

## Scope and assumptions

- Scope is limited to wording audit and traceability across the current front-door docs, roadmap memory, and relevant specs.
- This task does **not** change the normative judgment for projection / placement.
- Working assumption: `P3` is still open, `P4` is still the next reopen point, and current executable evidence remains helper-local / report-local preview only.
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
- `specs/10-open-questions.md`
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

## Actions taken

1. Read the required repository sequence before interpreting snapshot wording:
   `README.md` -> `Documentation.md` -> `progress.md` -> `specs/00..03` -> `specs/09` -> `plan/00-index.md` -> targeted roadmap/spec/docs files.
2. Extracted queue/current-status wording for `P3` / `P4` across the listed snapshot files.
3. Compared projection/placement stop-line wording against roadmap and spec anchors.
4. Checked whether any listed file currently overclaims emitted programs, generated artifacts, or equivalence checking.
5. Wrote this inspection report without changing the live snapshot documents.

## Evidence / outputs / test results

- Queue/current-status wording is broadly aligned on:
  - `P3` = current promoted next package
  - `P4` = next reopen point
- The main wording drift is **precision**, not direct contradiction:
  - `progress.md`, `tasks.md`, `plan/01`, `plan/11`, `plan/17`, `specs/11` distinguish `P3` vs `P4`.
  - `README.md`, `docs/hands_on/current_phase_closeout_01.md`, and `docs/research_abstract/mirrorea_future_axis_01.md` compress that state into a broader "next queue" or "stabilized queue" reading.
  - `samples_progress.md` is stricter and says the current active package is only `P3`.
- Projection/placement stop-line wording is mostly aligned:
  - preview floor only
  - no final emitted place program yet
  - no final projection IR / optimizer / equivalence checker yet
- Mild term drift remains between:
  - `generated artifact family`
  - `generated place-specific program emitter`
  - `emitted place program`
  - `equivalence checker` vs `cross-place equivalence checker`
- Commands run:
  - `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  - `rg --files README.md Documentation.md progress.md tasks.md samples_progress.md specs plan docs | sort`
  - `wc -l ...`
  - `rg -n "...projection|placement|queue|P3|P4|emitted|generated artifact|equivalence..." ...`
  - `sed -n ...`
  - `nl -ba ... | sed -n ...`

## What changed in understanding

- There is no major stale "wrong next package" statement in the inspected file set.
- The real drift risk is that some reader-facing summaries flatten:
  - `P3` as the active package
  - `P4` as the next reopen point
  into one queue list.
- The roadmap anchor remains clear that current projection evidence is preview-only and that emitted programs / optimizer / equivalence remain later-gate items.
- A `P3` close report would need to update not only `tasks.md` / `progress.md` / `samples_progress.md`, but also the front-door and reader-facing queue summaries that still narrate `P3` as current.

## Open questions

- When `P3` closes, should the repo standardize on one canonical phrasing:
  - `current promoted next package`
  - `next reopen point`
  - `stabilized queue`
  to reduce future drift?
- Should projection wording consistently say `cross-place equivalence checker` rather than the shorter `equivalence checker` to avoid implying a simpler local checker?
- Should `generated artifact family` be narrowed to `generated place-specific program family` so it does not blur with other artifact reserves in the repo?

## Suggested next prompt

Close `P3` docs-first and update the full snapshot set: `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `docs/hands_on/current_phase_closeout_01.md`, `docs/hands_on/projection_placement_views_01.md`, `docs/research_abstract/mirrorea_future_axis_01.md`, `docs/research_abstract/projection_placement_plan_01.md`, `plan/01-status-at-a-glance.md`, `plan/11-roadmap-near-term.md`, `plan/17-research-phases-and-autonomy-gates.md`, `plan/20-projection-and-placement-roadmap.md`, `plan/90-source-traceability.md`, and `specs/11-roadmap-and-workstreams.md`, while preserving the explicit stop line that emitted place programs, generated program artifacts, and cross-place equivalence checking remain later than the current preview floor unless new evidence is added.
