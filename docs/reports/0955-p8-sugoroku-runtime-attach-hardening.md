# 0955 — P8 Sugoroku runtime attach hardening

## Objective

`P8` Sugoroku runtime attach hardening として、Sugoroku helper closeout の attach / membership / handoff / late join / detach TODO boundary を、world sugar 境界、`MembershipRegistry` source-of-truth、late-join/handoff boundary、hot-plug stop line まで含めて明示化し、current snapshot / queue / dashboard / reader-facing docs を `P9` promoted line に同期する。

## Scope and assumptions

- current scope は helper/test/docs hardening であり、real network、consensus、durable distributed commit、rollback protocol、durable migration engine、final public runtime / hot-plug ABI はこの task では固定しない。
- `world` は current companion surface の host/server-side sugar として扱い、Mir core primitive へ昇格させない。
- attach / late join / detach の current evidence は representative slice の runtime canary と closeout carrier に留まり、actual multi-place runtime migration completion を意味しない。

## Documents consulted

- `sub-agent-pro/mirrorea_next_stage_full_plan_handoff_2026-04-27.md`
- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hands_on_sugoroku_00_overview.md`
- `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`

## Actions taken

1. Audited the current `P8` wording against the handoff and widened the package gate from a detach-only note to attach / handoff / late-join / stop-line evidence.
2. Kept the TDD path already in progress and added regression coverage for `world_surface`, `membership_model.source_of_truth`, `membership_model.late_join_handoff_boundary`, and `hotplug_stop_line`.
3. Updated Sugoroku helper closeout so the representative slice now records:
   - `world_surface = host_server_side_sugar`
   - `membership_model.source_of_truth = MembershipRegistry`
   - `membership_model.late_join_handoff_boundary`
   - `hotplug_stop_line`
4. Reworded the detach limitation from a generic TODO note to an explicit stop line that does not overclaim completed migration.
5. Advanced front-door docs, snapshot docs, plan memory, and the sample dashboard from `P8` promoted / `P9` reopen to `P9` promoted / `P10` reopen.
6. Added this report as the evidence anchor for the helper/test/docs package close.

## Files changed

- `docs/reports/0955-p8-sugoroku-runtime-attach-hardening.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hands_on_sugoroku_00_overview.md`
- `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`

## Evidence / outputs / test results

- `df -h .`
  - `/dev/vda2` 99G total, 32G free
- `free -h`
  - `960Mi` memory, `239Mi` available, `19Gi` swap with `18Gi` free at probe time
- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  - pass; attachpoint compatibility / activation cut / membership epoch import are visible in `hotplug_lifecycle`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json`
  - pass; handoff target activity, membership freshness, witness ordering, and transport envelope lanes remain visible
- `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership --format json`
  - pass; `MembershipRegistry` timeline, published-history visibility, and deferred turn-order insertion remain visible
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  - pass; detach remains an explicit TODO boundary with post-detach rejection and deferred migration / rollback
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass; helper closeout returns `world_surface`, `membership_model.source_of_truth`, `membership_model.late_join_handoff_boundary`, and `hotplug_stop_line`
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples -v`
  - pass; `39/39`, includes dedicated regression coverage for the new closeout carriers
- `python3 scripts/check_source_hierarchy.py`
  - pass; source hierarchy remains intact after the queue promotion and report addition
- `python3 scripts/validate_docs.py`
  - pass; front-door and snapshot docs remain aligned after the `P9` / `P10` queue shift
- `git diff --check`
  - pass; whitespace-clean after final doc/report sync

## What changed in understanding

- The meaningful `P8` hardening line is not “attach exists”, but “the representative closeout says where attach semantics stop”.
- `world` must stay explicitly on the host/server-side sugar side of the boundary; otherwise later docs drift back toward an implicit core primitive.
- Late join and handoff need a shared boundary statement because the operational question is not just history visibility but also when a target is active enough to receive authority-sensitive transitions.
- The detach sample is valuable as a stop-line proof, not as migration completion evidence.

## Open questions

- When `P14` revisits real hot-plug migration, how should rollback evidence and durable migration state be represented without collapsing them into the current helper closeout carrier?
- When `P10-P12` move toward real runtime / adapter work, should `MembershipRegistry` remain purely logical in the representative slice or gain a more explicit runtime inventory surface?
- What carrier should later expose visibility-return witness / anchor reacquire detail for the avatar slice without leaking it into a premature public contract?

## Suggested next prompt

`P9` avatar fairy follow hardening を進め、active representative slice `FAIRY-01/02/03/04/06` と residual planned family `FAIRY-05` の boundary、follow anchor / visibility guard / fallback lineage / stale-anchor rejection / reacquire stop line の wording と validation floor を helper/docs/report で同期してください。public avatar / visualization API や actual transport recovery completion は claim しないでください。
