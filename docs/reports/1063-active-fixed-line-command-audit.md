# 1063 - Active fixed-line command audit

## Objective

Audit active reader-facing docs for fixed line-number commands that would drift after snapshot compaction, especially reads into `progress.md` and `tasks.md`.

## Scope and assumptions

- Scope is audit-only plus this report and `progress.md` recent-log evidence.
- Active reader-facing docs include `README.md`, `Documentation.md`, `docs/hands_on/`, `docs/research_abstract/`, `samples/README.md`, and `scripts/README.md`, excluding `docs/research_abstract/old/`.
- Historical `docs/reports/` commands are report evidence and are not changed.
- Stop line: this audit does not change sample status, command behavior, implementation scope, public API, public ABI, or actual `U1`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/06-prismcascade-positioning.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `samples/README.md`
- `scripts/README.md`
- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`

## Actions taken

- Searched active reader-facing docs for fixed `sed -n '<line-range>p'` commands.
- Searched active reader-facing docs for `nl -ba ... | sed -n '<line-range>p'` commands.
- Confirmed no fixed line-number reads into `progress.md` or `tasks.md` remain in active reader-facing docs.
- Classified remaining fixed ranges as acceptable:
  - `docs/research_abstract/hands_on_typing.md` reads a sample source preview range.
  - `docs/hands_on/post_p18_true_user_spec_hold_01.md` reads the front of `plan/28`, which is long-lived repository memory rather than a compacted snapshot.
- Updated `progress.md` recent log with the audit result.

## Files changed

- `progress.md`
- `docs/reports/1063-active-fixed-line-command-audit.md`

## Evidence / outputs / test results

- Package start:
  - `git status --short` -> clean.
  - `git branch --show-current` -> `main`.
  - `git log -1 --oneline` -> `c24adf9 Clean active front-door snapshot drift`.
  - `date '+%Y-%m-%d %H:%M %Z'` -> `2026-05-01 10:09 JST`.
- Fixed-range command audit:
  - `rg -n "sed -n ['\"][0-9][^'\"]*['\"]" docs/hands_on docs/research_abstract README.md Documentation.md samples/README.md scripts/README.md -g '*.md' -g '!docs/research_abstract/old/**'` -> two acceptable hits:
    - `docs/research_abstract/hands_on_typing.md:32`
    - `docs/hands_on/post_p18_true_user_spec_hold_01.md:17`
  - `rg -n "nl -ba .*sed -n ['\"][0-9][^'\"]*['\"]" docs/hands_on docs/research_abstract README.md Documentation.md samples/README.md scripts/README.md -g '*.md' -g '!docs/research_abstract/old/**'` -> no hits.
  - `rg -n "sed -n '[0-9].*tasks.md|sed -n \"[0-9].*tasks.md|sed -n '[0-9].*progress.md|sed -n \"[0-9].*progress.md" README.md Documentation.md docs/hands_on docs/research_abstract -g '*.md' -g '!docs/research_abstract/old/**'` -> no hits.
- Post-report validation:
  - `python3 scripts/check_source_hierarchy.py` -> pass; required 35 / present 35 / missing 0.
  - `python3 scripts/validate_docs.py` -> pass; documentation scaffold complete, 1061 numbered reports found.
  - `git diff --check` -> pass.
  - `git diff --cached --check` -> pass.

## What changed in understanding

- The risky class is fixed line-number reads into compacted snapshot docs, not all fixed range previews.
- Active front-door docs now use section anchors for `progress.md` / `tasks.md` reads after compaction.

## Open questions

- Actual `U1` commitment remains user-facing and open.
- No further fixed-line command cleanup is required in active reader-facing docs at this point.

## Suggested next prompt

Continue autonomous maintenance by checking for another narrow active-docs drift class, such as stale report-count or validation-count wording outside reports.

## Plan update status

`plan/` 更新不要。No roadmap, repository memory, boundary, or sequencing interpretation changed.

## progress.md update status

`progress.md` 更新済み。Recent log records this fixed-line command audit.

## tasks.md update status

`tasks.md` 更新不要。No task status, blocker, or current package map changed.

## samples_progress.md update status

`samples_progress.md` 更新不要。No runnable sample path, validation command, debug surface, blocker, or progress percentage changed.

## Skipped validations and reasons

- Full validation floor was not rerun because this package changed only snapshot docs and report evidence.
- Cargo tests and sample closeouts were not rerun because no Rust code, sample source, runner behavior, or validation command changed.

## Commit / push status

Committed with `git commit --no-gpg-sign` and pushed to `origin/main`.

## Sub-agent session close status

No sub-agent was spawned for this narrow audit-only package. All previously used sub-agent sessions in earlier packages were closed before this package.
