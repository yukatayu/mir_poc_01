# 0925 HotPlug AttachPoint Plan

## Objective

phase 14 `HotPlug Patch / AttachPoint` を docs-first で repo に固定し、
compatibility / activation / migration stop line と attach / detach current anchor を整理する。

## Scope and assumptions

- current scope は docs-first roadmap と reader-facing summary の追加である。
- final hot-plug ABI、rollback、durable migration engine は固定しない。
- `scripts/storage/*` の detach/cleanup script は runtime hot-plug lifecycle と別 concern として扱う。
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
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hands_on_sugoroku_detail.md`

## Actions taken

1. `plan/21-hotplug-attachpoint-roadmap.md` を新規追加した。
2. attach / detach current anchor、compatibility checklist、activation cut、migration stop line を文書化した。
3. `docs/research_abstract/hotplug_attachpoint_plan_01.md` を追加し、reader-facing summary を置いた。
4. README / Documentation / progress / tasks / samples_progress / specs / plan / report を、next promoted package が network transport である current snapshot に同期した。
5. storage detach script と runtime hot-plug lifecycle を混同しない note を追加した。
6. stale queue を避けるため、`tasks.md` の duplicate future package row と `docs/research_abstract/mirrorea_future_axis_01.md` の old next-package queue を current snapshot に合わせて整理した。

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
- `plan/21-hotplug-attachpoint-roadmap.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hotplug_attachpoint_plan_01.md`
- `docs/reports/0925-hotplug-attachpoint-plan.md`

## Evidence / outputs / test results

- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --format json`
  - pass
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --format json`
  - pass
- `python3 scripts/sugoroku_world_samples.py check-all`
  - pass
- `git diff --check`
  - pass

## What changed in understanding

- hot-plug lifecycle は attach request / active state mutation / detach TODO boundary / migration stop line を分けて読むだけでも、
  network transport package の前提をかなり安定化できる。
- storage detach/cleanup script を runtime detach と切り分けておくことが、repo-local ops と semantics の goal drift を防ぐ。

## Open questions

- final hot-plug ABI をどこまで public object にするか。
- rollback protocol と durable migration engine をどの段で actualize するか。
- attach compatibility と projection validity をどこで machine-check に渡すか。

## Suggested next prompt

`Network transport` package を進めてください。separate-process / loopback / reconnect / failure matrix、auth/adapter split の stop line、report / progress / tasks / samples_progress 同期まで同じ task で閉じてください。
