# 0978 — R3 FAIRY-05 visibility-return carrier bundling closeout

## Objective

`R3` `FAIRY-05` visibility-return carrier bundling を docs-first package として close し、
active representative slice を保ったまま residual planned family `FAIRY-05` の
carrier-choice matrix と provisional recommendation を
repo memory / reader-facing docs / snapshot docs に同期する。

## Scope and assumptions

- current scope は helper closeout implementation inventory の整理であり、new helper carrier の実装ではない
- current scope は `FAIRY-05` sample の active promotion ではない
- current scope は final public avatar / visualization API、final transport / session / auth semantics の固定ではない
- helper closeout `carrier_choice = UNRESOLVED` は current implementation inventory として維持する
- `state_timeline` / `anchor_switch` は planning-only candidate labels であり、current debug mode や final public API と混同しない
- user-dirty の `crates/mir-ast/*` は触らない

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `tasks.md`
- `samples_progress.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/24-avatar-follow-representative-slice-roadmap.md`
- `specs/10-open-questions.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`
- `docs/reports/0978-r3-fairy05-visibility-return-carrier-bundling-closeout.md`
- `docs/reports/0939-fairy05-residual-reacquire-design.md`
- `docs/reports/0956-p9-avatar-fairy-follow-hardening.md`
- `docs/reports/0977-r2-attachpoint-detach-minimal-contract-closeout.md`

## Actions taken

1. avatar helper closeout / active canary floor を再確認した
   - `FAIRY-01/02/03/04/06` active representative slice
   - helper closeout `planned_sample_paths`
   - helper closeout `fairy05_reopen_gate`
2. `FAIRY-05` carrier-choice matrix を `plan/31-fairy05-visibility-return-carrier-bundling.md` として追加した
3. reader-facing summary / landing page を追加した
   - `docs/research_abstract/fairy05_visibility_return_carrier_bundling_01.md`
   - `docs/hands_on/fairy05_visibility_return_carrier_bundling_01.md`
4. avatar memory / open-question wording を同期した
   - `plan/24`
   - `specs/10`
   - `docs/research_abstract/avatar_fairy_follow_plan_01.md`
   - `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`
5. front-door / snapshot / queue wording を `R3` close / `R4` promoted-next に同期した
   - `README.md`
   - `Documentation.md`
   - `progress.md`
   - `tasks.md`
   - `samples_progress.md`
   - `plan/01`
   - `plan/11`
   - `docs/research_abstract/mirrorea_future_axis_01.md`
   - `docs/hands_on/current_phase_closeout_01.md`

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/24-avatar-follow-representative-slice-roadmap.md`
- `plan/31-fairy05-visibility-return-carrier-bundling.md`
- `specs/10-open-questions.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
- `docs/research_abstract/fairy05_visibility_return_carrier_bundling_01.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`
- `docs/hands_on/fairy05_visibility_return_carrier_bundling_01.md`
- `docs/reports/0978-r3-fairy05-visibility-return-carrier-bundling-closeout.md`

## Commands run

Resource baseline:

```bash
df -h .
free -h
```

Validation:

```bash
python3 -m unittest scripts.tests.test_avatar_follow_samples
python3 scripts/avatar_follow_samples.py check-all --format json
python3 scripts/avatar_follow_samples.py closeout --format json
python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json
python3 scripts/avatar_follow_samples.py run 02_remote_head_not_visible_falls_back_to_local --debug anchors --format json
python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json
python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- resource baseline
  - root disk: `/dev/vda2` 99G, 32G free, 67% used
  - memory: `960Mi` total, `153Mi` available, swap `17Gi` free
- unit validation
  - `scripts.tests.test_avatar_follow_samples`: `13/13` pass
- active representative slice
  - `check-all`: `5/5` active canaries pass
  - `01_follow_remote_head_with_local_fallback`
    - `anchor_graph = follow -> fallback`
    - `remote_head_seen witness used before follow commit`
  - `02_remote_head_not_visible_falls_back_to_local`
    - `fallback_reason = VisibilityLost`
    - `transport_recovery_claimed = false`
  - `03_remote_avatar_leaves_falls_back_to_local`
    - `membership_epoch = 1`
    - `remote_head_anchor` rejected as `StaleMembershipEpoch`
  - `06_model_check_no_detached_anchor_observed`
    - `model_check.verdict = pass`
    - `states_explored = 4`
- closeout inventory
  - `fairy05_reopen_gate.sample_status = planned_only`
  - `fairy05_reopen_gate.carrier_choice = UNRESOLVED`
  - `fairy05_reopen_gate.planning_only_candidate_labels = [state_timeline, anchor_switch]`
  - `required_evidence` still includes `positive_reacquire_after_return_sample`、`negative_missing_return_witness_or_stale_membership_companion`、`docs_report_snapshot_sync`
- doc / whitespace validation
  - `check_source_hierarchy.py`: pass
  - `validate_docs.py`: pass (`Found 976 numbered report(s).`)
  - `git diff --check`: pass

## What changed in understanding

- `P9` helper closeout `fairy05_reopen_gate` だけでは reopen 条件は読めても、
  carrier-choice matrix と current recommendation 自体は front-door に出ていなかった
- `R3` の適切な narrow line は helper schema を急いで固定することではなく、
  helper implementation inventory を保ったまま repository memory 側で
  typed bundle recommendation と planning-only label boundary を整理することだった
- `FAIRY-05` の reopen boundary を public API へ誤読させないには、
  active canary / planned family / planning-only candidate label / final public stop line を
  同じ package で併記する必要があった

## Open questions

- visibility-return witness を helper/runtime/public surface のどこで final naming するか
- `state_timeline` / `anchor_switch` を helper-local debug mode に actualize するかどうか
- `FAIRY-05` positive reacquire-after-return sample と negative companion をどの package で active 化するか
- final public avatar / visualization API を current representative slice からどう切り離すか
- `R4` で hot-plug real migration / rollback boundary をどこまで current kept-later gate として narrow にするか

## Suggested next prompt

`R4` hot-plug real migration / rollback boundary を進め、`R2` current minimal contract row と helper-local `hotplug_kept_later_gates` を保ったまま、real migration / rollback / runtime-crate hot-plug engine / distributed activation ordering の kept-later boundary を `plan/21` / `plan/30` / front-door docs / `progress.md` / `tasks.md` / `samples_progress.md` に同期してください。helper-local lifecycle canary を completed migration evidence と混同しないでください。
