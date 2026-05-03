# Report 1163 — Alpha-0.5 Stage-B Freshness and Status Clarification

- Date: 2026-05-04
- Author / agent: Codex
- Scope: focused alpha-0.5 / Stage B freshness rerun plus docs/helper clarification
- Decision levels touched: no new normative decision; snapshot wording, helper validation-floor wording, repository-memory status clarification only
- 日本語要約: `alpha-0.5` は current repo state ではすでに `P-A0-23` で閉じた Stage B current-scope evidence closeout であり、新しい promoted implementation line ではないことを fresh validation つきで再確認した。`LR-01/02` と supporting local-only save/load subset `CUT-04/17`、`runtime_substrate` dependency、`P-A1-07` までで practical line が止まっていることを README / Documentation / dashboards / local-runtime helper metadata に短く反映した。新しい runtime semantics や practical alpha-1 package は追加していない。

## Objective

Confirm whether `alpha-0.5` still represents an unfinished implementation target or an already-closed current-scope evidence bundle in the current repo state. If already closed, rerun the honest Stage B floor, tighten the wording that still invites misreading, and record the status precisely without silently reopening the alpha-0 line.

## Scope and assumptions

- Scope is limited to:
  - focused Stage B validation rerun
  - wording/metadata clarification in front-door docs, snapshot dashboards, local-runtime README, and helper closeout metadata
  - no new runtime/checker semantics
- No new normative statement is introduced in `specs/`.
- `alpha-0.5` is interpreted strictly by `specs/17`:
  `LR-01/02` plus supporting local-only `CUT-04/17` subset.
- The task does not promote `samples/alpha/` to an active runnable root.
- The task does not resolve the practical alpha-1 `P-A1-08` blocker.

## Start state / dirty state

- Work resumed on `main` after `P-A1-08` had already been recorded as recut-required.
- The worktree was clean at task start.
- Resource probe before the long-running task:
  - root disk: `99G` total, `65G` used, `30G` available
  - memory: `960Mi` total, `648Mi` used, `81Mi` free, `386Mi` buff/cache, `311Mi` available, `19Gi` swap / `1.8Gi` used

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/15-cut-save-load-checkpoint.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/43-alpha-e2e-roadmap.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/alpha/README.md`
- `samples/alpha/local-runtime/README.md`
- `docs/reports/1161-p-a1-07-local-save-load-command.md`
- `docs/reports/1162-p-a1-08-recut-blocker.md`

## Actions taken

1. Re-read the current alpha-0 and practical alpha-1 status docs to determine whether `alpha-0.5` still names unfinished implementation work.
2. Ran a focused Stage B floor over:
   - `LR-01/02`
   - supporting local-only save/load subset `CUT-04/17`
   - shared substrate test floor
   - helper wrapper tests
3. Asked three read-only reviewers to check:
   - stage/normative honesty,
   - Stage B validation completeness,
   - dashboard/front-door confusion points.
4. Confirmed that `alpha-0.5` is already complete for current scope and that the remaining issue is wording, not missing Stage B implementation.
5. Tightened `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `plan/01-status-at-a-glance.md`, `samples/alpha/local-runtime/README.md`, and `scripts/alpha_local_runtime_samples.py` so they no longer imply a reopened alpha-0 implementation mainline.
6. Added the substrate dependency to the local-runtime helper closeout metadata and test coverage so the Stage B validation floor is stated more honestly.

## Files changed

- Front-door / snapshot / repository memory:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `plan/01-status-at-a-glance.md`
- Alpha Stage B docs / helper:
  - `samples/alpha/local-runtime/README.md`
  - `scripts/alpha_local_runtime_samples.py`
  - `scripts/tests/test_alpha_local_runtime_samples.py`
- Report:
  - `docs/reports/1163-alpha-0-5-stage-b-freshness-and-status-clarification.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
df -h .
free -h
git status --short
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' progress.md
sed -n '1,260p' .docs/progress-task-axes.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,240p' specs/02-system-overview.md
sed -n '1,240p' specs/03-layer-model.md
sed -n '1,240p' specs/09-invariants-and-constraints.md
sed -n '1,260p' specs/17-mirrorea-spaces-alpha-scope.md
sed -n '1,260p' specs/18-practical-alpha1-scope.md
sed -n '1,260p' plan/43-alpha-e2e-roadmap.md
sed -n '1,260p' plan/44-practical-alpha1-roadmap.md
sed -n '1,240p' samples/alpha/local-runtime/README.md
sed -n '1,240p' docs/reports/1161-p-a1-07-local-save-load-command.md
sed -n '1,240p' docs/reports/1162-p-a1-08-recut-blocker.md
date '+%Y-%m-%d %H:%M JST'
python3 scripts/alpha_local_runtime_samples.py check-all --format json
python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json
python3 scripts/alpha_cut_save_load_samples.py check-all --format json
cargo test -p mirrorea-core --test runtime_substrate
cargo test -p mir-runtime --test alpha_local_runtime
cargo test -p mir-runtime --test alpha_cut_save_load_runtime
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- local-sugoroku
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- stale-membership
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-resume
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-stale-membership
python3 -m unittest scripts.tests.test_alpha_local_runtime_samples scripts.tests.test_alpha_cut_save_load_samples scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- Resource probe passed:
  - root disk: `99G` total, `65G` used, `30G` available
  - memory: `960Mi` total, `648Mi` used, `81Mi` free, `386Mi` buff/cache, `311Mi` available
- Stage B bundle checks passed:
  - `python3 scripts/alpha_local_runtime_samples.py check-all --format json`
    - `sample_count: 2`
    - `passed: ["LR-01", "LR-02"]`
  - `python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json`
    - `stage_b_complete: true`
    - `local_runtime_rows: ["LR-01", "LR-02"]`
    - `supporting_local_save_load_rows: ["CUT-04", "CUT-17"]`
    - `distributed_save_load_claimed: false`
    - `cut_family_complete: false`
  - `python3 scripts/alpha_cut_save_load_samples.py check-all --format json`
    - `sample_count: 2`
    - `passed: ["CUT-04", "CUT-17"]`
    - `distributed_save_load_claimed: false`
    - `durable_cut_claimed: false`
- Substrate/runtime/example floor passed:
  - `cargo test -p mirrorea-core --test runtime_substrate`
    - `16` tests passed
  - `cargo test -p mir-runtime --test alpha_local_runtime`
    - `3` tests passed
  - `cargo test -p mir-runtime --test alpha_cut_save_load_runtime`
    - `2` tests passed
  - `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- local-sugoroku`
    - emitted accepted `LR-01` report with event DAG/handoff evidence
  - `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- stale-membership`
    - emitted rejected `LR-02` report before state mutation
  - `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-resume`
    - emitted accepted `CUT-04` resumed dispatch report
  - `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-stale-membership`
    - emitted rejected `CUT-17` stale-membership-after-restore report
- Wrapper/docs floor passed:
  - `python3 -m unittest scripts.tests.test_alpha_local_runtime_samples scripts.tests.test_alpha_cut_save_load_samples scripts.tests.test_validate_docs`
    - `Ran 22 tests`
    - `OK`
- Final repository floor:
  - `python3 scripts/check_source_hierarchy.py`
    - `required: 73`
    - `present: 73`
    - `missing: 0`
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - `Found 1164 numbered report(s).`
  - `cargo fmt --check`
    - passed
  - `git diff --check`
    - passed

## What changed in understanding

- `alpha-0.5` is not the next implementation milestone anymore. In the current repo state it is already a closed Stage B evidence bundle.
- The honest Stage B reading depends on both:
  - `LR-01/02` local-runtime evidence, and
  - `CUT-04/17` supporting local-only save/load subset.
- The local-runtime family docs were slightly under-reporting that dependency and the shared substrate floor; fixing the wording was enough to remove the ambiguity.
- The remaining practical queue is blocked by `P-A1-08` recut/AV-carrier selection, not by unfinished Stage B work.

## Open questions

- Should `LR-01/02` later gain exact sidecar-parity validation similar to `CUT-04/17`, or is the current selected-field contract sufficient for the evidence line?
- When `P-A1-08` is eventually reopened, should the repo default to the thin product-preview recut over existing carriers, or should it wait for a practical `AV-A1-*` carrier first?

## Suggested next prompt

If you want the main implementation line to move again, choose whether `P-A1-08` should be recut to a thin practical product-preview bundle or whether practical `AV-A1-*` carriers must be actualized first. If the immediate goal is only alpha-0.5, no new implementation package remains; only maintenance/freshness reruns are left under that label.

## plan/ update status

`plan/` 更新済み: `plan/01-status-at-a-glance.md` に、`alpha-0.5` が already-closed evidence line であり、remaining work under that label is freshness/maintenance only, と明記した。`plan/43` / `plan/44` は current reading で足りるため今回は更新不要。

## Documentation.md update status

`Documentation.md` 更新済み: practical line の current stop point と、alpha Stage B が `CUT-04/17` supporting subset を伴う closeout surfaceであることを明記した。

## progress.md update status

`progress.md` 更新済み: `alpha-0.5` が既に達成済みの closed evidence bundle であること、`P-A0-28` が latest alpha-0 evidence refresh package であること、focused rerun log を反映した。

## tasks.md update status

`tasks.md` 更新済み: `alpha-0.5` request は already satisfied であり、新しい promoted implementation package ではないことを current task-level status に追加した。

## samples_progress.md update status

`samples_progress.md` 更新済み: Stage B row が closed evidence bundle であること、focused rerun の validation row、latest alpha-0 evidence refresh wording を反映した。

## reviewer findings and follow-up

- `Einstein`:
  front-door docs は practical line の promoted status と current stop pointの関係が still slightly easy to misread, と指摘した。`README.md` / `Documentation.md` に `P-A1-07` stop point と `P-A1-08` recut blocker を短く追記した。
- `Linnaeus`:
  Stage B closeout は規範的には already complete であり、overclaim risk は wording 側だと指摘した。`Stage B 100%` 表記を `current-scope ... closeout bundle` へ寄せ、`CUT-04/17` supporting subset dependency を明記した。
- `Turing`:
  helper closeout surface が `runtime_substrate` dependency を validation-floor wording 上で隠している、と指摘した。`scripts/alpha_local_runtime_samples.py` と `samples/alpha/local-runtime/README.md` の validation floor に substrate test と wrapper tests を追加した。

## skipped validations and reasons

- No practical alpha-1 implementation lane was touched, so `practical_alpha1_*` Cargo/tests/CLI surfaces were not rerun.
- No network/Docker/runtime Stage C..F code was widened, so Stage C..F alpha runtime floors were not rerun.
- No `specs/` file was edited because the task is a status clarification package, not a normative scope change.

## commit / push status

Pending at report write. Commit and push will be executed after final source-hierarchy/docs/format/diff validation and then reflected in a follow-up update if needed.

## sub-agent session close status

- `Linnaeus` completed the stage/normative honesty review and is closed
- `Turing` completed the Stage B validation-floor review and is closed
- `Einstein` completed the docs/snapshot clarity review and is closed
