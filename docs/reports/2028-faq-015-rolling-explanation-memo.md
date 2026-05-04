# Report 2028 — FAQ 015 Rolling Explanation Memo

- Date: 2026-05-04 22:20 JST
- Author / agent: Codex
- Scope: create `tmp_faq/faq_015.md` as a concise rolling explanation memo for future Q&A findings, seed it with current Mir-bottom-layer points, and commit/push only this task's FAQ/report on the current branch
- Decision levels touched: none normative; helper memo only

## Objective

Create `tmp_faq/faq_015.md` as the cumulative landing place for future explanation-task findings, unresolved implementation boundaries, and short concrete examples, starting with the current Mir bottom-layer discussion.

## Scope and assumptions

- Keep `tmp_faq/faq_015.md` as helper/dossier text, not normative source.
- Follow the user's instruction to keep the file concise and append-oriented rather than heavily structured.
- Leave the work on the current branch `docs/layered-repro-guide-001`; do not merge into `main`.
- Commit/push only the new FAQ and the required report for this task, so the user can later pull only the FAQ-related change into `main`.

## Start state / dirty state

- Current branch: `docs/layered-repro-guide-001`
- Worktree at task start already had unrelated untracked reports:
  - `?? docs/reports/1177-layered-repro-guide-001-readonly-repro-audit.md`
  - `?? docs/reports/2027-mir-bottom-layer-readonly-explanation-001.md`
- These pre-existing untracked files were left untouched and uncommitted in this task.

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `tmp_faq/faq_013.md`
- `tmp_faq/faq_014.md`
- `docs/reports/1125-faq-014-decision-inventory.md`
- `docs/reports/TEMPLATE.md`

## Actions taken

1. Confirmed the current dirty state and the existing FAQ/report naming convention.
2. Re-read the required top-level docs and layer docs to keep the helper memo aligned with current repo wording.
3. Inspected `faq_013` and `faq_014` to match the helper-file role while simplifying the new file into append-oriented notes.
4. Created `tmp_faq/faq_015.md` with a lightweight preamble and seeded it with the Mir bottom-layer findings from the immediately preceding explanation:
   sample roots, runnable commands, host-plan caveat, model-check scope, typing scope, dependent-type / Lean boundary, I/O boundary, computation capability, and native-binary unresolved boundary.
5. Added this task report.
6. Validated docs/diff health.
7. Staged only `tmp_faq/faq_015.md` and `docs/reports/2028-faq-015-rolling-explanation-memo.md`, then committed and pushed the current branch.

## Files changed

- `tmp_faq/faq_015.md`
- `docs/reports/2028-faq-015-rolling-explanation-memo.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
rg --files | rg 'faq_015\.md$|faq'
sed -n '1,200p' README.md
sed -n '1,220p' Documentation.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,220p' specs/02-system-overview.md
sed -n '1,220p' specs/03-layer-model.md
sed -n '1,220p' specs/09-invariants-and-constraints.md
sed -n '1,220p' tmp_faq/faq_014.md
sed -n '1,220p' tmp_faq/faq_013.md
sed -n '1,220p' docs/reports/1125-faq-014-decision-inventory.md
git branch --show-current
date '+%Y-%m-%d %H:%M:%S %Z'
sed -n '1,220p' docs/reports/TEMPLATE.md
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
git add tmp_faq/faq_015.md docs/reports/2028-faq-015-rolling-explanation-memo.md
git commit --no-gpg-sign -m "docs: add faq 015 rolling explanation memo"
git push -u origin docs/layered-repro-guide-001
git add docs/reports/2028-faq-015-rolling-explanation-memo.md
git commit --no-gpg-sign -m "docs: record faq 015 push status"
git push
git rev-parse HEAD
git status --short
python3 .agents/skills/discord-report/scripts/discord_notify.py complete --cwd . --result success --summary 'Added FAQ 015 rolling memo for explanation findings and pushed current branch commit' --include-diff
```

## Evidence / outputs / test results

- `tmp_faq/faq_015.md` was newly created under `tmp_faq/`, following the existing FAQ family location.
- The new FAQ stays intentionally lightweight: short preamble plus appendable bullet notes rather than a large section tree.
- The seeded content includes:
  - Mir core vs. underlying OS/network `L0`
  - active sample root vs. current-L2 corpus split
  - direct runnable commands
  - `e2-try-fallback` concrete runtime note
  - model-check second-line scope
  - finite decidable typing fragment note
  - dependent-type / Lean boundary
  - no-privileged-stdio boundary
  - native binary / binary split unresolved boundary
- `python3 scripts/check_source_hierarchy.py`
  passed.
- `python3 scripts/validate_docs.py`
  passed.
- `git diff --check`
  passed.
- The FAQ/report branch was published at `origin/docs/layered-repro-guide-001`.

## What changed in understanding

- The repo already had a pattern of numbered FAQ helper files, but `faq_015` did not exist yet.
- For the user's intended workflow, the right unit is not a big structured dossier refresh like `faq_014`, but a slimmer rolling memo that can absorb explanation-task findings incrementally.
- Because the user plans to manually bring only this FAQ forward into `main`, this task should avoid bundling unrelated untracked reports into the commit.

## Open questions

- If `faq_015.md` grows too large, whether to keep appending indefinitely or cut a later `faq_016.md` with a fresh rolling window remains open.
- Whether future FAQ entries should also mirror “not yet implemented” items into `tasks.md` / `plan/` will depend on whether the later explanation reveals real snapshot drift rather than just clarification.

## Suggested next prompt

Continue with `Layer 1` and, after the explanation, append the newly clarified points and any newly exposed unimplemented boundaries into `tmp_faq/faq_015.md`.

## Plan update status

`plan/` 更新不要:
this task only creates a helper memo and report; it does not change repository memory.

## Documentation.md update status

`Documentation.md` 更新不要:
the top-level snapshot already points readers to the correct authority chain.

## progress.md update status

`progress.md` 更新不要:
no project status or readiness line changed in this task.

## tasks.md update status

`tasks.md` 更新不要:
no execution package or blocker queue changed in this task.

## samples_progress.md update status

`samples_progress.md` 更新不要:
no sample status or validation scope changed in this helper-memo task.

## Reviewer findings and follow-up

- Local self-review only.
- No sub-agent review was opened; the task is small, docs-only, and the user did not ask for delegation.
- Review focus:
  - keep `faq_015.md` concise
  - preserve source hierarchy wording
  - avoid turning helper notes into normative claims
  - keep future manual main-branch intake easy by staging only the intended files

## Skipped validations and reasons

- No Rust test suite or cargo formatting run was repeated because this task changes no Rust/code sample behavior.
- No broader sample rerun was needed because `faq_015.md` only records already established explanatory findings; it does not change sample assets or helper command surfaces.

## Commit / push status

- Closeout commit `31c4140` (`docs: add faq 015 rolling explanation memo`) was created with `--no-gpg-sign`.
- The current branch `docs/layered-repro-guide-001` was pushed to `origin/docs/layered-repro-guide-001`.
- This report section was refreshed after the first push so the recorded status matches the actual branch state.

## Sub-agent session close status

No sub-agent sessions were opened in this task.
