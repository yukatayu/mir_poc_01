# Report 1177 — Layered Repro Guide 001 Read-Only Audit

- Date: 2026-05-04
- Author / agent: Codex
- Scope: Mir / Mirrorea layered purpose clarification and repo-local reproduction audit for first-time developers
- Decision levels touched: none (read-only audit; no normative edits)

## Objective

実装を進めずに、Mir / Mirrorea / practical alpha-1 の各 layer が何を目指し、何を保証し、どこまで repo-local に再現・検証できるかを、現行 repo state に即して説明できる状態を作る。

## Scope and assumptions

- user 指示に従い、new feature 実装、仕様変更、public API / ABI freeze、sample 昇格、commit / push は行わない。
- repo policy に従い、新規 report は追加する。
- snapshot docs (`Documentation.md` / `progress.md` / `tasks.md` / `samples_progress.md`) と `plan/` / `specs/` は read-only とする。
- final answer では、実行したコマンドと未実行コマンドを区別して説明する。

## Start state / dirty state

- Start branch: `main`
- Start dirty state: clean worktree
- Start sync actions: `git fetch origin`, `git checkout main`, `git pull --ff-only`
- Working branch for this task: `docs/layered-repro-guide-001`

## Documents consulted

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/09-invariants-and-constraints.md`
- `specs/13-type-system-lifetime-fallback.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `specs/15-cut-save-load-checkpoint.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/43-alpha-e2e-roadmap.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/alpha/README.md`
- `samples/practical-alpha1/README.md`
- `scripts/README.md`
- `docs/reports/TEMPLATE.md`
- `docs/reports/1176-practical-alpha1-final-docs-roadmap-audit.md`
- `docs/reports/1175-p-a1-17-practical-save-load-preview-carrier-alignment.md`
- practical alpha-1 code/script anchors:
  `crates/mir-ast/src/practical_alpha1_checker.rs`,
  `crates/mir-ast/src/practical_alpha1_runtime_plan.rs`,
  `crates/mir-runtime/src/practical_alpha1_local_runtime.rs`,
  `crates/mir-runtime/src/practical_alpha1_hotplug.rs`,
  `crates/mir-runtime/src/practical_alpha1_transport.rs`,
  `crates/mir-runtime/src/practical_alpha1_save_load.rs`,
  `crates/mir-runtime/src/practical_alpha1_avatar.rs`,
  `scripts/practical_alpha1_check.py`,
  `scripts/practical_alpha1_run_local.py`,
  `scripts/practical_alpha1_attach.py`,
  `scripts/practical_alpha1_transport.py`,
  `scripts/practical_alpha1_export_devtools.py`,
  `scripts/practical_alpha1_save_load.py`,
  `scripts/practical_alpha1_avatar.py`,
  `scripts/practical_alpha1_product_preview.py`

## Actions taken

1. Repo policy and skill instructions (`using-superpowers`, `discord-report`, `verification-before-completion`) were read and applied.
2. Git state was synchronized on `main`, then a local explanation branch `docs/layered-repro-guide-001` was created.
3. Required docs were read in repository order, with roadmap/snapshot/docs-first separation preserved.
4. A focused code-anchor scan was performed for practical alpha-1 carrier boundaries and CLI entrypoints.
5. Resource status was checked with `df -h .` and `free -h` before running broader validation commands.
6. Repository health, current-L2 baseline, and practical alpha-1 reproduction commands were executed to gather fresh evidence for the final explanation.
7. A read-only report was added; no snapshot or normative documents were edited.

## Files changed

- `docs/reports/1177-layered-repro-guide-001-readonly-repro-audit.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
git fetch origin
git branch --list 'docs/layered-repro-guide-*'
git rev-parse --abbrev-ref HEAD
git checkout main
git pull --ff-only
git switch -c docs/layered-repro-guide-001
ls -1t docs/reports | head -n 10
df -h .
free -h
date '+%Y-%m-%d %H:%M:%S %Z'
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/current_l2_source_sample_regression.py inventory
python3 scripts/current_l2_guided_samples.py closeout --format json
python3 scripts/clean_near_end_samples.py closeout
python3 scripts/current_l2_lean_sample_sync.py
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/avatar_follow_samples.py closeout --format json
cargo test -p mir-ast practical_alpha1_front_door -- --nocapture
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_transport.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py check-all --format json
python3 scripts/practical_alpha1_save_load.py check-all --format json
python3 scripts/practical_alpha1_avatar.py check-all --format json
python3 scripts/practical_alpha1_product_preview.py check-all --format json
python3 scripts/practical_alpha1_product_preview.py closeout --format json
rg -n "PracticalAlpha1(CheckReport|RuntimePlan|LocalRuntimeReport|HotPlugReport|TransportReport|SaveLoadReport|AvatarPreviewReport|ProductPreviewBundle|PackageLoader|LoadedPackage|CheckedPackage)" crates/mir-ast/src crates/mir-runtime/src
rg -n "def (cmd_|run_|check_all|closeout|render_html)|argparse|subparsers|SAMPLE_IDS|PE2E|AV-A1|SL-A1|VIS-A1|HP-A1|TR-A1|RUN-01|CHK-LIF" scripts/practical_alpha1_*.py
git status --short
```

## Evidence / outputs / test results

- `df -h .`: root disk `99G` total, `65G` used, `30G` free, `69%` used.
- `free -h`: `960Mi` RAM total, `418Mi` available, swap `18Gi` free.
- `check_source_hierarchy.py`: required `73`, present `73`, missing `0`.
- `validate_docs.py`: scaffold complete, `1177` numbered reports found before this report was added.
- `cargo fmt --check`: passed.
- `git diff --check`: passed before edits.
- `current_l2_source_sample_regression.py inventory`: authored current-L2 subset inventory printed successfully.
- `current_l2_guided_samples.py closeout --format json`: clean near-end closeout JSON emitted successfully.
- `clean_near_end_samples.py closeout`: clean near-end closeout emitted successfully.
- `current_l2_lean_sample_sync.py`: `samples/lean/manifest.json` refreshed and worktree stayed clean.
- `sugoroku_world_samples.py closeout --format json`: `sample_count = 10`, logical multi-place emulator evidence present, detach remains explicit TODO boundary.
- `avatar_follow_samples.py closeout --format json`: active sample count `5`; `FAIRY-05` remains planned-only.
- `cargo test -p mir-ast practical_alpha1_front_door -- --nocapture`: `11` front-door tests passed; limited `package.mir.json` cut is working.
- `practical_alpha1_check.py check-all --format json`: `10/10` passed; `first_checker_floor_complete = true`; `spec18_typed_checking_complete = false`.
- `practical_alpha1_run_local.py check-all --format json`: `2/2` passed; `local_runtime_first_floor_complete = true`; `run_docker_claimed = false`.
- `practical_alpha1_attach.py check-all --format json`: `9/9` passed; `object_attach_claimed = false`; `detach_minimal_contract_complete = true`.
- `practical_alpha1_transport.py check-all --format json`: `7/7` passed; `docker_row_complete = true`; `wan_federation_claimed = false`.
- `practical_alpha1_export_devtools.py check-all --format json`: `7/7` passed; `VIS-A1-01..07` actualized.
- `practical_alpha1_save_load.py check-all --format json`: `3/3` passed; `distributed_save_load_claimed = false`; `stale_witness_claimed = false`; `stale_lease_claimed = false`.
- `practical_alpha1_avatar.py check-all --format json`: `3/3` passed; `native_execution_claimed = false`.
- `practical_alpha1_product_preview.py check-all --format json`: `9/9` passed; `product_preview_first_floor_complete = true`; `stage_pa1_8_complete = false`.
- `practical_alpha1_product_preview.py closeout --format json`: validation floor and stop lines were printed; viewer HTML remains available as a non-final preview surface.
- Final `git status --short`: clean after all read-only execution work, before adding this report.

## What changed in understanding

- current repo state is unusually explicit about non-claims: many practical alpha-1 floors are complete for current scope, but the repo itself keeps `stage_pa1_8_complete = false` and repeatedly rejects product-ready interpretations.
- Layer 4 through Layer 11 are tied together by distinct carrier boundaries, not by a single monolithic runtime:
  checker report, runtime plan, local runtime report, hot-plug report, transport report, save-load report, avatar preview report, devtools bundle, and product-preview bundle remain separate.
- Alpha-0 Stage A..F evidence closeout is still important for orientation, but it is not the promoted readiness metric anymore.

## Open questions

- Which layer the user wants to inspect next in detail: Mir semantics, Mirrorea fabric, practical checker, practical runtime, hot-plug, transport, save/load, devtools, avatar preview, or product preview.
- Whether a later explanation task should also trace one concrete row end-to-end, for example `RUN-01 -> VIS-A1-01 -> PE2E-01` or `CHK-CUT-01 -> SL-A1-03 -> PE2E-06`.

## Suggested next prompt

`Layer 4 の practical checker floor を、sample ID・carrier・非目標込みでもっと詳しく説明して。` のように、任意の layer を 1 つ指定して深掘りする。

## Plan update status

`plan/` 更新不要:
read-only audit only; no roadmap change was made.

## Documentation.md update status

`Documentation.md` 更新不要:
current wording already matched the validated repo state for this explanation task.

## progress.md update status

`progress.md` 更新不要:
no repo status change was introduced by this read-only audit.

## tasks.md update status

`tasks.md` 更新不要:
no task-map change was introduced by this read-only audit.

## samples_progress.md update status

`samples_progress.md` 更新不要:
no runnable sample status changed in this task.

## Reviewer findings and follow-up

- Local focused review only.
- No sub-agent reviewer was spawned because current operating instructions allow sub-agents only when explicitly requested by the user.
- Main follow-up finding:
  the final explanation must keep three levels separate:
  `current-scope evidence closeout`,
  `practical alpha-1 readiness`,
  and `public/product readiness`.

## Skipped validations and reasons

- `python3 scripts/current_l2_source_sample_regression.py regression --run-label ... --artifact-root ...` was not executed.
  Reason: this task needed a reproducibility explanation baseline, not a full regression artifact run.
- `python3 scripts/current_l2_guided_samples.py smoke-all --format json` was not executed.
  Reason: `closeout` plus the other baseline commands already gave current active-floor evidence for this explanation task.
- Focused Cargo tests for every practical lane (`practical_alpha1_checker`, `practical_alpha1_runtime_plan`, `practical_alpha1_hotplug`, `practical_alpha1_transport`, `practical_alpha1_save_load`) were not individually rerun.
  Reason: the script-level `check-all` commands for those lanes were executed and were sufficient for explanation-oriented verification.
- No browser opening of generated HTML viewers/previews was performed.
  Reason: this task was CLI/document explanation only.

## Commit / push status

Not performed by explicit user instruction.
Current task is a local explanation branch only.

## Sub-agent session close status

No sub-agent sessions were opened in this task.
