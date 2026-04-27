# Report 0917 — Avatar fairy follow sample plan

## 1. Title and identifier

- Identifier: `0917`
- Title: `Avatar fairy follow sample plan`

## 2. Objective

phase 8 `avatar fairy follow / fallback anchor` について、historical prototype と
future active sample family を混同しない planned skeleton family を repo に置き、
sample ID、goal、reader-facing explanation、progress/dashboard row を固定する。

## 3. Scope and assumptions

- この package は docs-first / sample-first planning closeout であり、active helper implementation ではない。
- `%` は `10%` に留める。sample skeleton と explanation があるが、helper / parser / loader / static carrier はまだない。
- historical prototype は planning evidence として残すが、current active sample として扱わない。

## 4. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/not_implemented/README.md`
- `samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.txt`
- `samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.host-plan.json`
- `sub-agent-pro/mirrorea_future_plan_full_handoff_2026-04-24.md`
- `docs/research_abstract/README.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`

## 5. Actions taken

- `samples/not_implemented/avatar-fairy-follow/` を新設し、`FAIRY-01..06` skeleton file と README を追加した。
- `docs/research_abstract/avatar_fairy_follow_plan_01.md` を追加し、phase 8 の目的、sample ID、historical prototype との差分、未決 helper surface を reader-facing に整理した。
- `samples_progress.md` に phase 8 row と current blocker を反映し、`FAIRY-01..06` を `% = 10` の planned family として登録した。
- `tasks.md`、`progress.md`、`README.md`、`Documentation.md`、`plan/01`、`plan/11`、`plan/12`、`plan/16`、`plan/17`、`plan/90`、`specs/10`、`specs/11` を同期し、prototype / planned skeleton / future active helper の 3 段階を区別した。

### files changed

- `samples/not_implemented/README.md`
- `samples/not_implemented/avatar-fairy-follow/README.md`
- `samples/not_implemented/avatar-fairy-follow/01_follow_remote_head_with_local_fallback.mir`
- `samples/not_implemented/avatar-fairy-follow/02_remote_head_not_visible_falls_back_to_local.mir`
- `samples/not_implemented/avatar-fairy-follow/03_remote_avatar_leaves_falls_back_to_local.mir`
- `samples/not_implemented/avatar-fairy-follow/04_invalid_cross_anchor_chain_rejected.mir`
- `samples/not_implemented/avatar-fairy-follow/05_follow_target_reacquired_after_return.mir`
- `samples/not_implemented/avatar-fairy-follow/06_model_check_no_detached_anchor_observed.mir`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
- `samples_progress.md`
- `tasks.md`
- `progress.md`
- `README.md`
- `Documentation.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`

### commands run

- `python3 scripts/current_l2_guided_samples.py run-source-sample samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.txt --host-plan samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.host-plan.json --format json`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## 6. Evidence / outputs / test results

- historical prototype compatibility command:
  - exit code `2`
  - output:
    - `current_l2_guided_samples.py now forwards to the clean near-end active suite`
    - `supported compatibility commands: list, smoke-all, closeout`
  - meaning:
    - prototype path is no longer an active runnable route
    - keeping avatar phase 8 under `samples/not_implemented/` is the correct non-overclaiming status
- `python3 scripts/check_source_hierarchy.py`
  - required `23`, missing `0`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 913 numbered report(s).`
- `git diff --check`
  - clean

## 7. What changed in understanding

- phase 8 で最初に必要なのは active helper そのものではなく、prototype と current active line の混同を止める stable planning anchor だった。
- `FAIRY-01..06` を先に fixed すると、次の `TermSignature` / `LayerSignature` package で必要になる anchor / fallback / rejection / verification surface の話を sample-first に繋ぎやすい。
- historical prototype を消す必要はなく、`prototype reference only` と `planned skeleton only` を分ければ十分に reader-friendly かつ正確に保てる。

## 8. Open questions

- active helper surface を Sugoroku helper extension と専用 helper のどちらに置くか
- `FollowAnchor` / visibility / stale-anchor rejection をどの signature carrier で表すか
- debug mode を `summary` / `membership` / `verification` 風に寄せるか、anchor graph 専用 surface を切るか

## 9. Suggested next prompt

`TermSignature registry / debug output` を進めてください。phase 8 sample skeleton に出てくる `FollowAnchor`、fallback lineage、stale-anchor rejection を、Sugoroku witness / handoff line と並べて読める shared signature vocabulary に切り出し、helper-local `--debug signatures` surface の first cut を追加してください。
