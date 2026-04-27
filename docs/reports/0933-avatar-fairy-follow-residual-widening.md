# 0933 Avatar Fairy Follow Residual Widening

## Objective

phase 8 `avatar fairy follow / fallback anchor` の residual widening を最小限で進め、
`FAIRY-02` visibility-loss fallback を active helper canary に昇格させつつ、
`FAIRY-05` reacquire-after-return を residual planned family に残す current snapshot を固定する。

## Scope and assumptions

- current scope は repo-local helper-local widening に限る。
- `FAIRY-02` を active representative slice に昇格させるが、final public avatar runtime API、final visualization protocol、real transport / auth / session semantics は scope 外に残す。
- `FAIRY-05` は state-timeline / anchor-switch carrier がまだ無いため planned family に残す。
- worktree には unrelated current-L2 Rust diff が残っているため、本 package の stage / commit には含めない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/clean-near-end/avatar-follow/README.md`
- `samples/not_implemented/README.md`
- `samples/not_implemented/avatar-fairy-follow/README.md`
- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/24-avatar-follow-representative-slice-roadmap.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0930-avatar-fairy-follow-representative-slice.md`
- `docs/reports/0932-network-transport-helper-local-canaries.md`

read-only sub-agent findings:

- `019dcebb-4885-7102-a49b-50edaa590484` `Nietzsche`
- `019dcebb-4961-7400-9b4d-01b29966290a` `Curie`
- reviewer follow-up requested on current diff:
  `019dcecb-6134-7e50-841c-1a568a824352` `Tesla`

## Actions taken

1. `scripts/tests/test_avatar_follow_samples.py` で phase 8 widening の expected shape を先に固定した。
   - active sample list に `02_remote_head_not_visible_falls_back_to_local`
   - residual planned family は `FAIRY-05` のみ
   - `FAIRY-06` detached-anchor safety の regression を強化
2. `scripts/avatar_follow_samples.py` を widen し、`FAIRY-02` を active helper canary に昇格させた。
   - `terminal_outcome = fallback_on_visibility_loss`
   - `fallback_reason = VisibilityLost`
   - `transport_recovery_claimed = false`
   - visibility-loss witness が anchor switch 前に publish される verification log
3. sample taxonomy を更新した。
   - active source:
     `samples/clean-near-end/avatar-follow/02_remote_head_not_visible_falls_back_to_local.mir`
   - planned source から同 file を削除し、`samples/not_implemented/avatar-fairy-follow/` には `FAIRY-05` だけを残した
4. reader-facing docs / repository memory / snapshot docs を同期した。
   - active representative slice は `FAIRY-01` / `02` / `03` / `04` / `06`
   - residual planned family は `FAIRY-05`
   - next promoted package は `cross-package sweep`
5. stale reference cleanup を行い、`FAIRY-02` をまだ planned と書いていた root docs / plan / specs / dashboard を更新した。

## Files changed

- `scripts/avatar_follow_samples.py`
- `scripts/tests/test_avatar_follow_samples.py`
- `samples/clean-near-end/avatar-follow/02_remote_head_not_visible_falls_back_to_local.mir`
- `samples/clean-near-end/avatar-follow/README.md`
- `samples/not_implemented/avatar-fairy-follow/02_remote_head_not_visible_falls_back_to_local.mir` (deleted)
- `samples/not_implemented/avatar-fairy-follow/README.md`
- `samples/not_implemented/README.md`
- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/24-avatar-follow-representative-slice-roadmap.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0933-avatar-fairy-follow-residual-widening.md`

## Evidence / outputs / test results

red step:

- `python3 -m unittest scripts.tests.test_avatar_follow_samples`
  - fail before widening (`FAIRY-02` missing from active list / residual family still too wide)

green / closeout step:

- `python3 -m unittest scripts.tests.test_avatar_follow_samples`
  - pass (`Ran 10 tests`)
- `python3 scripts/avatar_follow_samples.py run 02_remote_head_not_visible_falls_back_to_local --debug anchors --format json`
  - pass (`fallback_on_visibility_loss`, `transport_recovery_claimed = false`)
- `python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json`
  - pass (`no_detached_anchor_observed`)
- `python3 scripts/avatar_follow_samples.py check-all --format json`
  - pass (`FAIRY-01` / `02` / `03` / `04` / `06` all green)
- `python3 scripts/avatar_follow_samples.py closeout --format json`
  - pass (`planned_sample_ids = ["05_follow_target_reacquired_after_return"]`)

- `python3 scripts/check_source_hierarchy.py`
  - pass (`required 23, missing 0`)
- `python3 scripts/validate_docs.py`
  - pass after final report sync (`Found 931 numbered report(s).`)
- `git diff --check`
  - pass

## What changed in understanding

- `FAIRY-02` は representative slice の設計を壊さずに active helper canary へ昇格できた。visibility loss は transport recovery ではなく observation / witness lane の問題として切り出す方が current phase に合う。
- `FAIRY-05` は同じ widening と一緒に上げるべきではなかった。reacquire-after-return には state-timeline / anchor-switch evidence が必要で、current helper surfaceの `anchors` / `membership` / `verification` だけでは説明不足になる。
- phase 8 では active / planned / historical taxonomy の drift が起きやすい。sample path だけでなく root docs / `progress.md` / `tasks.md` / `samples_progress.md` / `specs` まで同 package で揃える必要がある。

## Open questions

- `FAIRY-05` reacquire-after-return を reopen するなら、state-timeline / anchor-switch / visibility-return witness のどれを最小 debug carrier にするか。
- phase 8 helper-local `anchors` / `membership` surface を future visualization / projection / hot-plug lane とどこで接続するか。
- cross-package sweep 後の next promoted line を `FAIRY-05` residual reopen、typed external boundary executable widening、projection executable widening のどれに置くか。

## Suggested next prompt

`cross-package sweep` package を進めてください。transport / avatar / hot-plug / dashboard / closeout landing page / next queue を current snapshot に揃え、remaining mixed gate と next promoted line を report-backed に recut してください。
