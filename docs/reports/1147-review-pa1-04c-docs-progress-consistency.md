# Report 1147 — Review P-A1-04c Docs/Progress Consistency

- Date: 2026-05-03
- Author / agent: Codex
- Scope: review wording changes required if the detach minimal contract closes inside `PA1-4` and the package/hot-plug stage is marked complete
- Decision levels touched: none

## Objective

Review the current snapshot, roadmap-memory, and normative practical-alpha-1 wording and identify what must change, and what must remain explicitly non-claiming, if `P-A1-04c` closes the detach minimal contract and `PA1-4` becomes complete.

## Scope and assumptions

- Review-only task; no file edits besides this report.
- Assumed close condition: `P-A1-04c` closes the practical detach minimal contract required by `specs/18` for the package/hot-plug stage.
- The user prompt says `HP-A1-07`, but the current repository has no such sample ID. The existing package ID is `P-A1-04c`, while `P-A1-07` already names the later local save/load package.
- Focused on `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `specs/18`, `plan/44`, and the detach-boundary memory in `plan/30`.

## Start state / dirty state

- start state: `main` at current workspace HEAD
- worktree status: clean before this report

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/30-attachpoint-detach-minimal-contract.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`

## Actions taken

1. Read the mandatory repository front-door and normative documents in the order required by `AGENTS.md`.
2. Read the current status snapshots and the progress/task-axis policy.
3. Compared the practical alpha-1 stage wording in snapshot docs against the normative completion condition in `specs/18`.
4. Checked roadmap-memory and detach-memory documents to separate what should move with `PA1-4` from what must stay helper-local or later-family.
5. Collected the concrete wording sites that would need synchronized updates if `P-A1-04c` closes.

## Files changed

- `docs/reports/1147-review-pa1-04c-docs-progress-consistency.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md
sed -n '1,220p' .agents/skills/discord-report/SKILL.md
rg --files README.md Documentation.md progress.md tasks.md samples_progress.md .docs specs plan | sort
rg -n "HP-A1-07|PA1-4|P-A1-04c|detach|minimal contract" README.md Documentation.md progress.md tasks.md samples_progress.md specs plan .docs docs
sed -n '1,260p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,320p' progress.md
sed -n '1,260p' .docs/progress-task-axes.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,220p' specs/02-system-overview.md
sed -n '1,240p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,260p' specs/18-practical-alpha1-scope.md
sed -n '1,260p' plan/00-index.md
sed -n '1,320p' plan/44-practical-alpha1-roadmap.md
sed -n '1,260p' plan/30-attachpoint-detach-minimal-contract.md
sed -n '1,260p' tasks.md
sed -n '1,320p' samples_progress.md
rg -n "HP-A1-07|P-A1-04c|P-A1-07|PA1-4|detach minimal contract|final object package attach|save/load" progress.md tasks.md samples_progress.md README.md Documentation.md specs/18-practical-alpha1-scope.md plan/44-practical-alpha1-roadmap.md plan/30-attachpoint-detach-minimal-contract.md
rg -n "HP-A1-07|P-A1-04c|P-A1-07|PA1-4|detach minimal contract|object_attach_claimed|PlaceholderAvatarObjectPackage|detach" samples/practical-alpha1 crates/mir-ast crates/mir-runtime scripts
nl -ba progress.md | sed -n '20,210p'
nl -ba tasks.md | sed -n '1,270p'
nl -ba samples_progress.md | sed -n '1,240p'
nl -ba README.md | sed -n '46,66p'
nl -ba Documentation.md | sed -n '56,80p'
nl -ba specs/18-practical-alpha1-scope.md | sed -n '145,210p'
nl -ba plan/44-practical-alpha1-roadmap.md | sed -n '96,210p'
nl -ba samples/practical-alpha1/README.md | sed -n '48,72p'
nl -ba samples/practical-alpha1/packages/README.md | sed -n '1,40p'
nl -ba samples/practical-alpha1/expected/README.md | sed -n '1,40p'
nl -ba plan/30-attachpoint-detach-minimal-contract.md | sed -n '1,120p'
git status --short
date '+%Y-%m-%d %H:%M:%S %Z'
python3 scripts/validate_docs.py
git diff --check
git status --short
```

## Evidence / outputs / test results

- `progress.md`, `tasks.md`, and `samples_progress.md` all currently encode the same state: `PA1-4` is `65%` and still blocked on `P-A1-04c` (`progress.md:25-38`, `tasks.md:75-84`, `tasks.md:125-133`, `samples_progress.md:29-40`, `samples_progress.md:88-96`).
- `specs/18` makes detach minimal contract part of the package/hot-plug stage requirement, but keeps transport, save/load, and devtools as later stages (`specs/18-practical-alpha1-scope.md:185-204`).
- `README.md` and `Documentation.md` currently say the practical hot-plug floor stops at `HP-A1-06` and still excludes detach minimal contract (`README.md:52-55`, `Documentation.md:61-75`).
- `plan/44` currently places detach minimal contract inside `PA1-4` and keeps `PA1-7` reserved for local save/load (`plan/44-practical-alpha1-roadmap.md:108-117`, `plan/44-practical-alpha1-roadmap.md:131-160`, `plan/44-practical-alpha1-roadmap.md:164-173`).
- `plan/30` is intentionally helper-local detach-boundary memory, not practical-alpha-1 closeout evidence (`plan/30-attachpoint-detach-minimal-contract.md:10-12`, `plan/30-attachpoint-detach-minimal-contract.md:23-29`, `plan/30-attachpoint-detach-minimal-contract.md:67-72`, `plan/30-attachpoint-detach-minimal-contract.md:103-116`).
- No repository file currently defines an `HP-A1-07` row; the `HP-A1-*` practical sample set ends at `HP-A1-06`, while `P-A1-07` already means the later local save/load package (`tasks.md:129-133`, `tasks.md:238-241`, `plan/44-practical-alpha1-roadmap.md:131-160`).
- `python3 scripts/validate_docs.py` passed after adding this report and printed `Documentation scaffold looks complete.` with `Found 1151 numbered report(s).`
- End-of-task `git status --short` showed this new report plus unrelated live edits in `crates/mir-ast/src/practical_alpha1.rs`, `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`, and `crates/mir-runtime/src/practical_alpha1_hotplug.rs`. Those code changes were not modified by this review task.

## What changed in understanding

- The main risk is not only stale status percentages. The bigger consistency hazard is naming and stage-boundary collapse: if a detach closeout is described as `HP-A1-07` without being explicitly introduced, readers can confuse a hot-plug sample row with the already-reserved `P-A1-07` save/load package.
- `PA1-4` can become complete without promoting transport, save/load, devtools, or public ABI claims, because `specs/18` already separates those into later stages. The snapshots therefore need selective widening, not blanket “hot-plug complete” language.
- `plan/30` should remain a boundary-memory reference. The promoted repository-memory source for a practical `PA1-4` closeout is `plan/44`, not a broadened reinterpretation of the helper-local `AttachPoint` note.

## Open questions

- If the detach closeout introduces a new practical sample row, what exact sample ID should it use? The current tree does not define `HP-A1-07`.
- Should `PA1-4` move directly to `100%`, or does the repository want an intermediate “closeout complete, commit/push pending” wording before the stage percentage is updated?
- Will the practical detach closeout keep the current object-preview boundary (`object_attach_claimed = false`), or is a separate object-path change planned later?

## Suggested next prompt

- `If P-A1-04c closes, update progress.md, tasks.md, samples_progress.md, README.md, Documentation.md, specs/18-practical-alpha1-scope.md, plan/44-practical-alpha1-roadmap.md, and the practical-alpha1 sample READMEs so PA1-4 is shown as complete, P-A1-05 becomes the next package, and detach closeout remains explicitly non-claiming for transport/save-load/devtools/final object attach/public ABI.`

## Plan update status

`plan/` 更新不要: review-only taskであり repository memory は変更していない。

## Documentation.md update status

`Documentation.md` 更新不要: review-only taskであり snapshot wording は変更していない。

## progress.md update status

`progress.md` 更新不要: review-only taskであり current status snapshot は変更していない。

## tasks.md update status

`tasks.md` 更新不要: review-only taskであり task map は変更していない。

## samples_progress.md update status

`samples_progress.md` 更新不要: review-only taskであり sample dashboard snapshot は変更していない。

## Reviewer findings and follow-up

- Finding 1:
  The requested label `HP-A1-07` is currently undefined and is dangerously close to the already-defined package name `P-A1-07` for local save/load. `tasks.md:129-133` and `tasks.md:238-241` reserve `P-A1-07` for save/load, and `plan/44-practical-alpha1-roadmap.md:131-160` does the same. If the detach closeout is written as “`HP-A1-07` closes detach minimal contract” without adding that sample ID consistently to `README.md`, `Documentation.md`, `specs/18`, `plan/44`, and the practical sample READMEs, the repo will read as though `PA1-7` moved rather than `PA1-4` closing. Required follow-up: either keep the closeout wording anchored on `P-A1-04c` alone, or explicitly introduce a new `HP-A1-07` row everywhere the current practical hot-plug rows are enumerated.

- Finding 2:
  The three snapshot authorities must move in lockstep if `PA1-4` becomes complete. Right now all three say `PA1-4` is `65%` and still blocked on detach (`progress.md:25-38`, `tasks.md:75-84`, `tasks.md:125-133`, `samples_progress.md:29-40`, `samples_progress.md:88-96`). If only one or two of them are updated, the repo will simultaneously claim that `PA1-4` is complete and still in progress. Required follow-up: update all three together so `PA1-4` becomes closed, `P-A1-04c` becomes the last closed package, and `P-A1-05` becomes the next autonomous package.

- Finding 3:
  The root summaries and practical-alpha-1 memory/spec docs need selective widening, but must remain explicitly non-claiming. `README.md:52-55` and `Documentation.md:61-75` currently exclude detach minimal contract from the actualized floor. After closeout, they should say that detach minimal contract is now explicit inside the practical hot-plug stage, but they must still not claim final object package attach, Docker/local TCP transport, local save/load command, devtools completion, product prototype, or final public runtime/devtools/package ABI. The same boundary must remain in `specs/18-practical-alpha1-scope.md:168-204` and `plan/44-practical-alpha1-roadmap.md:102-106`, `plan/44-practical-alpha1-roadmap.md:188-190`.

- Finding 4:
  `plan/30` must not be repurposed as the proof that practical `PA1-4` is complete. Its own scope explicitly says it fixes helper-local evidence reading rather than final public ABI, runtime-crate engine behavior, rollback protocol, or durable migration (`plan/30-attachpoint-detach-minimal-contract.md:10-12`, `plan/30-attachpoint-detach-minimal-contract.md:23-29`, `plan/30-attachpoint-detach-minimal-contract.md:103-116`). Required follow-up: if detach closeout wants historical or conceptual grounding, cross-reference `plan/30`; do not rewrite it as the practical alpha-1 stage-close authority. The practical stage-close memory belongs in `plan/44`.

- Finding 5:
  The practical sample READMEs will also need synchronized but still non-claiming updates if a new detach row is added. `samples/practical-alpha1/README.md:55-60`, `samples/practical-alpha1/packages/README.md:10-17`, and `samples/practical-alpha1/expected/README.md:8-13` currently stop at `HP-A1-06` and explicitly say the floor does not imply final object attach, detach lifecycle completion, transport, save/load, or public ABI. If a detach sample is added, these files must enumerate it, but they must continue to say that the sample family is a non-final practical floor rather than a final detach runtime or public hot-plug ABI completion.

## Skipped validations and reasons

- No implementation or sample validation commands were rerun because this was a wording-consistency review, not a closeout or freshness-verification task.
- No snapshot or normative docs were edited beyond this report, so cargo/python/runtime reruns would not materially change the review evidence.

## Commit / push status

- No commit created.
- No push performed.

## Sub-agent session close status

- No sub-agents used.
