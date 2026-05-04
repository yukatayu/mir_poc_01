# Report 1176 — Practical Alpha-1 Final Docs / Roadmap Audit

## Objective

大局的な目標、やり残し、snapshot docs / plan / specs / sample taxonomy の実態一致を最終確認する。

日本語要約:
current repo state は `P-A1-17` 時点の practical alpha-1 first-floor / widened evidence floor として整合している。`PA1-0..7` は current practical floor として閉じ、`PA1-8` は widened product-preview + avatar companion floor として 80% のまま正しく残っている。full practical alpha-1 / public alpha / `U1` / same-session product runtime / distributed durable save-load は未完であり、docs もそれを completion としては claim していない。

## Scope and Assumptions

- 既存 `docs/reports/` 本文は user 指示により監査対象外とした。
- 新規 report は repository reporting policy に従って作成した。
- 監査対象は `README.md`、`Documentation.md`、`AGENTS.md`、`progress.md`、`tasks.md`、`samples_progress.md`、`.docs/progress-task-axes.md`、`specs/18`、relevant `plan/`、`samples/README.md`、`samples/practical-alpha1/README.md`、`scripts/README.md`、および relevant command surfaces。
- この report は semantic row actualization ではなく、docs / roadmap consistency audit である。

## Start State / Dirty State

- Start branch: `main`.
- Start dirty state: clean worktree.
- Latest closeout before this audit: `P-A1-17` practical save-load preview carrier alignment.

## Documents Consulted

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `scripts/README.md`

Existing report bodies were not read.

## Actions Taken

- Audited top-level docs, current snapshots, normative practical alpha-1 scope, roadmap memory, sample taxonomy, and script taxonomy for stale large-goal claims.
- Confirmed current docs separate current-scope alpha-0 evidence closeout from practical alpha-1 readiness.
- Confirmed current docs do not claim final parser/runtime/checker API, public `U1`, product-ready runtime, same-session product runtime, native avatar execution, or distributed durable save/load.
- Corrected stale `P-A0-01..28` references to `P-A0-01..29`.
- Updated `plan/44` implementation order and current reading from `P-A1-13` to `P-A1-17`.
- Added missing practical avatar and product-preview check commands to `samples/README.md` and roadmap validation floors.
- Fixed `tasks.md` blocker numbering and latest alpha-0 maintenance package wording.
- Added concise audit entries to `progress.md` and `samples_progress.md`.

## Files Changed

- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `tasks.md`
- `progress.md`
- `samples_progress.md`
- `docs/reports/1176-practical-alpha1-final-docs-roadmap-audit.md`

## Commands Run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
date '+%Y-%m-%d %H:%M %Z'
rg -n 'P-A0-01\.\.28|latest alpha-0 evidence refresh|current reading after `P-A1-13`|current actualized cut after `P-A1-11`|Blocker 9|Blocker 8\. save/load' README.md Documentation.md progress.md tasks.md samples_progress.md specs plan samples scripts --glob '!docs/reports/**'
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_transport.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py check-all --format json
python3 scripts/practical_alpha1_save_load.py check-all --format json
python3 scripts/practical_alpha1_avatar.py check-all --format json
python3 scripts/practical_alpha1_product_preview.py check-all --format json
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / Outputs / Test Results

- `practical_alpha1_check.py check-all`: passed 10/10; first checker floor complete; runtime/public CLI still not claimed.
- `practical_alpha1_run_local.py check-all`: passed 2/2; local runtime first floor complete; Docker/package/save-load not claimed by that lane.
- `practical_alpha1_attach.py check-all`: passed 9/9; package/hot-plug first floor and deferred detach minimal contract complete; object attach / save-load not overclaimed.
- `practical_alpha1_transport.py check-all`: passed 7/7; local TCP / Docker rows complete; WAN/final ABI not claimed.
- `practical_alpha1_export_devtools.py check-all`: passed 7/7; `VIS-A1-01..07` actualized.
- `practical_alpha1_save_load.py check-all`: passed 3/3; `SL-A1-01/02/03` complete; distributed save/load, stale witness, stale lease not claimed.
- `practical_alpha1_avatar.py check-all`: passed 3/3; avatar preview floor complete; native execution and final avatar ABI not claimed.
- `practical_alpha1_product_preview.py check-all`: passed 9/9; `PE2E-01..09` actualized; `stage_pa1_8_complete = false` remains correct.
- `python3 -m unittest scripts.tests.test_validate_docs`: 11 tests passed.
- `python3 scripts/check_source_hierarchy.py`: required 73, present 73, missing 0.
- `python3 scripts/validate_docs.py`: scaffold complete; final pass found 1177 numbered reports after this report was added.
- `cargo fmt --check`: passed.
- `git diff --check`: passed.

## What Changed in Understanding

- No large-goal contradiction was found in the current docs: the repo is consistent as a practical alpha-1 in-progress toolchain with multiple completed first floors, not as full practical alpha-1 100%.
- `PA1-8` correctly remains 80% because same-session product runtime and full product prototype completion are not implemented.
- The main docs issue was stale roadmap wording, not missing implementation evidence.

## Open Questions

- Whether to promote `P-A1-18` requires a concrete narrow row definition. Current likely reopen points are broader save/load widening, same-session product runtime semantics, or another exact-evidence package that does not collapse existing carrier boundaries.
- Public `U1` still requires user-facing decisions about packaging / installed binary target, host integration target, first shipped public surface scope, and final shared-space operational catalog breadth.

## Suggested Next Prompt

Promote the next package only after selecting one narrow target: broader save/load widening with a concrete carrier, same-session product runtime semantics, or an equally narrow exact-evidence widening. Do not promote full practical alpha-1 or `U1` by wording alone.

## `plan/` Update Status

Updated `plan/44-practical-alpha1-roadmap.md` to reflect `P-A0-01..29`, the `P-A1-17` current reading, and complete practical command coverage. `plan/01-status-at-a-glance.md` required no edit.

## `Documentation.md` Update Status

Reviewed. No edit required; current wording already separates `P-A1-17` actualization from broader save/load, same-session runtime, product-ready runtime, and public `U1`.

## `progress.md` Update Status

Updated timestamp and recent log with this audit and the corrected roadmap/sample command alignment.

## `tasks.md` Update Status

Updated timestamp, alpha-0 closeout range, latest alpha-0 maintenance package wording, blocker numbering, and practical validation floor commands.

## `samples_progress.md` Update Status

Updated timestamp, `PH0` report list, and recent validation row for this docs / roadmap audit. No new sample row was actualized.

## Reviewer Findings and Follow-Up

Local focused review only. No sub-agent reviewer was spawned because this user request asked for final checking rather than delegated review, and the current operating instruction permits sub-agent use only when explicitly requested. Findings:

- Specs / roadmap: stale `P-A0-01..28` and `P-A1-13` wording needed correction.
- Sample taxonomy: practical avatar and product-preview commands were implemented but omitted from `samples/README.md` current command list.
- Snapshot docs: current blocker and non-claim wording was accurate; no large overclaim was found.

## Skipped Validations and Reasons

- Existing `docs/reports/` body review was skipped by explicit user instruction.
- Broad `cargo test` was skipped because this task touched docs only and no Rust source changed. `cargo fmt --check` was run.
- Heavy storage / external workdir audit was skipped because no build artifacts, storage policy, or large generated artifacts were changed.

## Commit / Push Status

Primary audit commit pushed:

- `5cb2887` — `docs: audit practical alpha1 roadmap status`

This follow-up metadata update is docs-only and will be committed separately.

## Sub-Agent Session Close Status

No new sub-agent sessions were opened for this audit. Existing sessions were not used.
