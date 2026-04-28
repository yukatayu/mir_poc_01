# 0956 — P9 avatar fairy follow hardening

## Objective

`P9` avatar fairy follow hardening として、active representative slice `FAIRY-01/02/03/04/06` を崩さずに residual planned family `FAIRY-05` の reopen gate を helper closeout / dashboard / snapshot docs に明示化し、queue を `P10` promoted / `P11` reopen next へ進める。

## Scope and assumptions

- current scope は helper/test/docs hardening であり、`FAIRY-05` 自体を runnable widening しない。
- `state_timeline` / `anchor_switch` は planning-only candidate label であり、current debug mode や final public visualization / avatar runtime API ではない。
- real transport / session / auth semantics、Rust runtime 実装、public avatar / visualization API はこの task では固定しない。

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
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/24-avatar-follow-representative-slice-roadmap.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `samples/clean-near-end/avatar-follow/README.md`
- `scripts/avatar_follow_samples.py`
- `scripts/tests/test_avatar_follow_samples.py`
- `docs/reports/0939-fairy05-residual-reacquire-design-review.md`
- `docs/reports/0940-rereview-fairy05-residual-reacquire-wording-after-local-edits.md`

## Actions taken

1. Audited the repo-local P9 scope and kept it as helper/test/docs hardening only, without widening `FAIRY-05` into an active sample.
2. Used test-first edits to add closeout regressions for `fairy05_reopen_gate`, `planned_sample_paths`, and missing planned-path rejection, verified they failed, then updated `scripts/avatar_follow_samples.py` minimally to pass.
3. Recorded the planning-only reopen gate in helper closeout as:
   - `sample_status = planned_only`
   - `required_evidence = positive sample / negative companion / state_timeline / anchor_switch / docs_report_snapshot_sync`
   - `carrier_choice = UNRESOLVED`
   - `planning_only_candidate_labels = state_timeline / anchor_switch`
4. Fixed dashboard and doc drift around the actual `FAIRY-05` `.mir` path, the fuller reopen gate inventory, and the next-package queue.
5. Advanced front-door docs, snapshot docs, repository memory, and the sample dashboard from `P9` promoted / `P10` reopen to `P10` promoted / `P11` reopen.
6. Corrected a repository-memory attribution drift in `docs/research_abstract/mirrorea_future_axis_01.md` so that the broader reopen gate is recorded as part of 2026-04-28 `P9` hardening rather than the earlier 2026-04-27 docs-first fixation, then re-reviewed the narrowed diff with no remaining findings.

## Files changed

- `docs/reports/0956-p9-avatar-fairy-follow-hardening.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/24-avatar-follow-representative-slice-roadmap.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `samples/clean-near-end/avatar-follow/README.md`
- `scripts/avatar_follow_samples.py`
- `scripts/tests/test_avatar_follow_samples.py`

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_avatar_follow_samples.AvatarFollowSamplesTests.test_closeout_records_fairy05_reopen_gate scripts.tests.test_avatar_follow_samples.AvatarFollowSamplesTests.test_closeout_records_planned_sample_path_for_fairy05 -v`
  - RED observed first as `KeyError: 'fairy05_reopen_gate'` and `KeyError: 'planned_sample_paths'`
- `python3 -m unittest scripts.tests.test_avatar_follow_samples.AvatarFollowSamplesTests.test_closeout_rejects_missing_planned_sample_path -v`
  - RED observed first because `closeout()` did not reject a missing planned path
- `python3 -m unittest scripts.tests.test_avatar_follow_samples.AvatarFollowSamplesTests.test_closeout_records_fairy05_reopen_gate scripts.tests.test_avatar_follow_samples.AvatarFollowSamplesTests.test_closeout_records_planned_sample_path_for_fairy05 -v`
  - GREEN after minimal code update
- `python3 -m unittest scripts.tests.test_avatar_follow_samples.AvatarFollowSamplesTests.test_closeout_rejects_missing_planned_sample_path -v`
  - GREEN after adding fail-fast planned-path existence checks
- `python3 -m unittest scripts.tests.test_avatar_follow_samples -v`
  - pass; `13/13`, full helper regression floor
- `python3 scripts/avatar_follow_samples.py check-all --format json`
  - pass; active canaries `5/5` stay green
- `python3 scripts/avatar_follow_samples.py closeout --format json`
  - pass; closeout now returns `planned_sample_paths` and `fairy05_reopen_gate`
- `python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json`
  - pass; explicit follow/fallback lineage remains intact
- `python3 scripts/avatar_follow_samples.py run 02_remote_head_not_visible_falls_back_to_local --debug anchors --format json`
  - pass; visibility-loss fallback stays local without transport recovery claim
- `python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json`
  - pass; stale-membership rejection path remains intact
- `python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json`
  - pass; detached-anchor safety remains a verification canary
- `python3 scripts/check_source_hierarchy.py`
  - pass; source hierarchy intact after doc/report sync
- `python3 scripts/validate_docs.py`
  - pass; front-door and snapshot docs aligned after queue promotion
- `git diff --check`
  - pass; whitespace-clean after final sync

## What changed in understanding

- The right P9 close line is not “make `FAIRY-05` work”, but “make the planned gate inspectable without pretending it is runnable”.
- The helper closeout needed to carry the gate explicitly; leaving it only in prose docs made the active/planned boundary too easy to blur.
- `state_timeline` / `anchor_switch` are useful planning labels, but they must remain clearly on the planning-only side until a later widening package justifies them.

## Open questions

- If `FAIRY-05` is later widened, should visibility-return witness land in a timeline event, anchor-switch event, witness event, or typed bundle?
- What negative companion should be paired with the positive reacquire-after-return case first: missing return witness, stale membership, or both?
- At what later package should avatar slice concerns move from helper-local evidence into crate-side runtime ownership?

## Suggested next prompt

`P10` `mirrorea-core` first real implementation tranche に入り、crate ownership boundary、minimal core carrier、helper script から切り出す責務、最初の validation floor を `plan/19`、`progress.md`、`tasks.md`、`samples_progress.md`、report に整理してください。subsystem collapse や public API freeze はまだ行わないでください。
