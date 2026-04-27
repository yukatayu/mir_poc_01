# 0930 Avatar Fairy Follow Representative Slice

## Objective

phase 8 `avatar fairy follow / fallback anchor` を、docs-first skeleton だけでなく
repo-local helper-local representative slice として actualize し、
active sample / residual planned sample / historical prototype の境界を明示したまま
follow / fallback / stale-anchor rejection / detached-anchor safety を runnable evidence に上げる。

## Scope and assumptions

- current scope は repo-local helper-local representative slice である。
- active scope は `FAIRY-01` / `FAIRY-03` / `FAIRY-04` / `FAIRY-06` に限定する。
- `FAIRY-02` / `FAIRY-05` は residual planned family として `samples/not_implemented/` に残す。
- final public avatar runtime API、final visualization protocol、real transport / auth / session semantics、hot-plug implementation は scope 外である。
- worktree には unrelated current-L2 CLI diff が残っているため、この package でも stage / commit に含めない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/not_implemented/README.md`
- `samples/not_implemented/avatar-fairy-follow/README.md`
- `samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.txt`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0917-avatar-fairy-follow-sample-plan.md`
- `docs/reports/0929-network-transport-loopback-preview.md`

## Actions taken

1. `scripts/tests/test_avatar_follow_samples.py` を先に追加し、`avatar_follow_samples` module missing の RED を確認した。
2. `scripts/avatar_follow_samples.py` を追加し、dedicated helper surface `list / run / check-all / closeout` と debug modes `summary / anchors / membership / verification` を実装した。
3. active representative sample root `samples/clean-near-end/avatar-follow/` を追加し、`FAIRY-01` / `03` / `04` / `06` を active canary として置いた。
4. residual planned family `samples/not_implemented/avatar-fairy-follow/` から promoted active canary file を外し、`FAIRY-02` / `05` だけが残る形へ再説明して、historical prototype と active sample の境界を固定した。
5. `plan/24-avatar-follow-representative-slice-roadmap.md` と `docs/hands_on/avatar_fairy_follow_representative_slice_01.md` を追加し、repository memory と reader-facing command guide を用意した。
6. root docs / `samples_progress.md` / `progress.md` / `tasks.md` / relevant `plan/` / `specs/` / `docs/` を同期し、phase 8 を `active representative slice + residual planned family` として current snapshot に反映した。

## Files changed

- `scripts/avatar_follow_samples.py`
- `scripts/tests/test_avatar_follow_samples.py`
- `samples/clean-near-end/avatar-follow/01_follow_remote_head_with_local_fallback.mir`
- `samples/clean-near-end/avatar-follow/03_remote_avatar_leaves_falls_back_to_local.mir`
- `samples/clean-near-end/avatar-follow/04_invalid_cross_anchor_chain_rejected.mir`
- `samples/clean-near-end/avatar-follow/06_model_check_no_detached_anchor_observed.mir`
- `samples/clean-near-end/avatar-follow/README.md`
- `samples/not_implemented/avatar-fairy-follow/README.md`
- `samples/not_implemented/avatar-fairy-follow/01_follow_remote_head_with_local_fallback.mir` (deleted)
- `samples/not_implemented/avatar-fairy-follow/03_remote_avatar_leaves_falls_back_to_local.mir` (deleted)
- `samples/not_implemented/avatar-fairy-follow/04_invalid_cross_anchor_chain_rejected.mir` (deleted)
- `samples/not_implemented/avatar-fairy-follow/06_model_check_no_detached_anchor_observed.mir` (deleted)
- `samples/not_implemented/README.md`
- `samples/README.md`
- `scripts/README.md`
- `docs/hands_on/README.md`
- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/README.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/24-avatar-follow-representative-slice-roadmap.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/0930-avatar-fairy-follow-representative-slice.md`

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_avatar_follow_samples`
  - pass
- `python3 scripts/avatar_follow_samples.py check-all --format json`
  - pass
- `python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json`
  - pass
- `python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json`
  - pass
- `python3 scripts/avatar_follow_samples.py run 04_invalid_cross_anchor_chain_rejected --format json`
  - pass
- `python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json`
  - pass
- `python3 scripts/avatar_follow_samples.py closeout --format json`
  - pass
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass (`Found 928 numbered report(s).`)
- `git diff --check`
  - pass

## What changed in understanding

- phase 8 は full avatar stack を待たなくても、dedicated helper と active representative sample root で安全に runnable evidence へ押し上げられる。
- `FAIRY-01` / `03` / `04` / `06` を active にし、`FAIRY-02` / `05` を residual planned family に残す構成なら、active / planned / historical の taxonomy を壊さない。
- avatar follow helper を Sugoroku helper に埋め込まず別 script に切る方が、layer boundary と sample taxonomy を明示的に保ちやすい。

## Open questions

- `FAIRY-02` visibility-loss-only fallback を active helper に取り込むべきか。
- `FAIRY-05` reacquire-after-return を representative slice の外でどう widen するか。
- phase 8 helper-local `anchors` / `membership` / `verification` surface を future visualization / projection / hot-plug lane とどこで接続するか。

## Suggested next prompt

`HotPlug Patch / AttachPoint executable widening` package を進めてください。`SUG-01` attach と `SUG-09` detach TODO boundary を使い、compatibility summary、activation-cut readiness、post-detach rejection evidence を helper-local canary として actualize し、storage detach と runtime hot-plug の非同一性を report / docs / snapshot に同期してください。
