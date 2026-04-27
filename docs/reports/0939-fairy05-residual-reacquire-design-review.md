# Report 0939 — FAIRY-05 Residual Reacquire Design Review

## 1. Objective

`FAIRY-05` residual planned family について、sample 自体を active helper に昇格させず、
docs-first design review package として reopen gate だけを明文化する。
特に active evidence / next docs-first package / planned sample family を混同せず、
legacy phase label と current macro-phase reading を併記し、
visibility-return witness の carrier bundling を `UNRESOLVED` として残す。

## 2. Scope and assumptions

- scope は phase 8 avatar follow residual `FAIRY-05` の docs / plan / snapshot synchronization に限る
- helper code 実装、new runnable sample actualization、public API fix は行わない
- `FAIRY-05` sample は `samples/not_implemented/` に残し、active representative slice は
  `FAIRY-01` / `FAIRY-02` / `FAIRY-03` / `FAIRY-04` / `FAIRY-06` のままとする
- current reading では `phase 8` は legacy sample-family label、
  macro-phase reading は `Macro 6 reserve` とする

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/24-avatar-follow-representative-slice-roadmap.md`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `samples/not_implemented/avatar-fairy-follow/README.md`
- `samples/not_implemented/avatar-fairy-follow/05_follow_target_reacquired_after_return.mir`
- `docs/reports/0938-review-fairy05-residual-reacquire-wording-risk.md`

## 4. Actions taken

1. `FAIRY-05` 関連 snapshot を横断し、active evidence / planned family / queue wording のズレを再確認した。
2. `README.md` / `Documentation.md` では、`FAIRY-05` を next executable package ではなく
   docs-first design review closeout として書き直し、sample remains planned と
   no public avatar / visualization API implied を同じ文に入れた。
3. `tasks.md` / `progress.md` / `samples_progress.md` の current snapshot を更新し、
   current promoted next line を Typed external boundary / adapter executable widening に戻した。
4. `docs/research_abstract/avatar_fairy_follow_plan_01.md`、
   `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`、
   `plan/24-avatar-follow-representative-slice-roadmap.md` で、
   `phase 8` を legacy sample-family label と明記し、
   current macro-phase reading を `Macro 6 reserve` と追記した。
5. `FAIRY-05` に必要な条件は
   explicit state timeline / anchor switch evidence gate までに留め、
   visibility-return witness の carrier bundling と helper-local debug surface exact naming は
   `UNRESOLVED` として残した。
6. `samples/not_implemented/avatar-fairy-follow/05_follow_target_reacquired_after_return.mir` と
   residual README へ planning-only note を追加し、hidden transport/session recovery を
   claim しないことを補った。

## 5. Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/24-avatar-follow-representative-slice-roadmap.md`
- `plan/90-source-traceability.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `samples/not_implemented/avatar-fairy-follow/README.md`
- `samples/not_implemented/avatar-fairy-follow/05_follow_target_reacquired_after_return.mir`
- `docs/reports/0939-fairy05-residual-reacquire-design-review.md`

## 6. Commands run

- `python3 scripts/avatar_follow_samples.py check-all --format json`
- `python3 scripts/avatar_follow_samples.py closeout --format json`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## 7. Evidence / outputs / test results

- validation commands:
  - `python3 scripts/avatar_follow_samples.py check-all --format json`
  - `python3 scripts/avatar_follow_samples.py closeout --format json`
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`
- validation results:
  - `avatar_follow_samples.py check-all --format json`
    - pass
    - `sample_count = 5`
    - passed sample IDs:
      `01_follow_remote_head_with_local_fallback`
      `02_remote_head_not_visible_falls_back_to_local`
      `03_remote_avatar_leaves_falls_back_to_local`
      `04_invalid_cross_anchor_chain_rejected`
      `06_model_check_no_detached_anchor_observed`
  - `avatar_follow_samples.py closeout --format json`
    - pass
    - active sample IDs remain `FAIRY-01/02/03/04/06`
    - `planned_sample_ids = ["05_follow_target_reacquired_after_return"]`
    - `planned_remaining = "FAIRY-05 reacquire-after-return remains planned"`
  - `check_source_hierarchy.py`
    - pass
    - `required = 23`, `present = 23`, `missing = 0`
  - `validate_docs.py`
    - pass
    - `Documentation scaffold looks complete.`
    - final rerun after review-only report addition: `Found 938 numbered report(s).`
  - `git diff --check`
    - pass
  - review-only rereview:
    - `docs/reports/0940-rereview-fairy05-residual-reacquire-wording-after-local-edits.md`
    - `no findings`
- skipped validation:
  - Rust / cargo tests は未実行
    - docs-only package で runtime/helper code を変更していないため

## 8. What changed in understanding

- `FAIRY-05` は active helper widening package ではなく、
  planned sample family に対する docs-first design review package として扱う方が
  queue / dashboard / sample status の三層分離に整合する。
- reopen gate で必要なのは exact carrier 実装の fix ではなく、
  hidden repair に落とさないための evidence requirement の fix である。
- `visibility-return witness` は重要概念だが、独立 carrier にするか、
  `state_timeline` / `anchor_switch` 候補へ束ねるかは current scope では決めない方が安全である。

## 9. Open questions

- `visibility-return witness` を timeline event / anchor-switch event / witness event /
  typed bundle のどれとして carry するか
- helper-local debug surface / CLI option の exact naming
- `FAIRY-05` runnable widening を typed external boundary / projection widening の前に再度 promote する価値があるか

## 10. Suggested next prompt

`Typed external boundary / adapter executable widening` package を進めてください。
`EXT-01..05` のうち最小の helper-local cut を actualize し、provider boundary / local queue /
typed failure / debug redaction の evidence を runnable helper、docs、snapshot、report まで同一 task で同期してください。
