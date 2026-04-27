# 0924 Projection Placement Plan

## Objective

phase 12 `Projection / placement` を docs-first で repo に固定し、
system-wide source と place-specific program の区別、place split、validity checklist、stop line を整理する。

## Scope and assumptions

- current scope は docs-first roadmap と reader-facing summary の追加である。
- final projection IR、generator、optimizer、equivalence checker は固定しない。
- worktree には unrelated current-L2 CLI formatting diff が残っているため、この package では stage / commit に含めない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/05-mirrorea-fabric.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/typed_external_boundary_adapter_plan_01.md`

## Actions taken

1. `plan/20-projection-and-placement-roadmap.md` を新規追加した。
2. system-wide source / place-specific program distinction、server / participant / adapter / visualizer split、projection validity checklist、stop line を文書化した。
3. `docs/research_abstract/projection_placement_plan_01.md` を追加し、reader-facing summary を置いた。
4. README / Documentation / progress / tasks / samples_progress / specs / plan / report を、next promoted package が hot-plug patch / `AttachPoint` である current snapshot に同期した。
5. unrelated current-L2 CLI formatting diff は読み取りのみとし、この package には含めないと明示した。

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/20-projection-and-placement-roadmap.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/projection_placement_plan_01.md`
- `docs/reports/0924-projection-placement-plan.md`

## Evidence / outputs / test results

- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `git diff --check`
  - pass

## What changed in understanding

- projection / placement は executable helper を急いで作らなくても、
  validity checklist と place split を先に固定するだけで hot-plug / transport package の前提を揃えられる。
- `provider_boundary` と Sugoroku place split は、phase 12 の current evidence anchor として十分に機能する。

## Open questions

- final projection IR をどこまで public object にするか。
- generated place-specific emitter / optimizer / equivalence checker をどの段で actualize するか。
- projection viewer と visualization authority model をどう接続するか。

## Suggested next prompt

`HotPlug Patch / AttachPoint` package を進めてください。`Patch Req Prov Δ`、compatibility / activation / migration stop line、`SUG-09` detach TODO boundary との接続、report / progress / tasks / samples_progress 同期まで同じ task で閉じてください。
