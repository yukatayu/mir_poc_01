# Report 2196 — practical alpha-0.5 session path portability

- Date: 2026-07-04 15:27 JST
- Author / agent: Codex
- Scope: Practical alpha-0.5 session helper repo-relative nested package argv maintenance
- Decision levels touched: none; implementation / documentation maintenance only

## Objective

Close the remaining practical alpha-0.5 nested session helper portability gap by
passing repo-owned package roots to `mir_practical_alpha05_session` as
repo-relative `samples/...` CLI arguments, while keeping temporary session files
as absolute temp paths.

## Scope and assumptions

- Scope is limited to `scripts/practical_alpha05_session.py`, its unit tests,
  and snapshot/report documents.
- The change is path-portability maintenance only.
- It does not change bounded alpha-0.5 workflow status, semantics, ABI, canon
  status, sample row count, or product/public readiness.
- Repo-owned paths should become repo-relative CLI arguments.
- External paths and temporary session file paths should remain absolute.

## Start state / dirty state

- Start branch: `main`
- Start HEAD: `40e24ef5fb8dec6db6fbff79a9a5d1b144f519fe`
- Start state: clean and matched `origin/main`.

## Documents consulted

- `AGENTS.md` instructions supplied in the task context
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `docs/reports/TEMPLATE.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- Related helper/test files under `scripts/`

## Actions taken

- Added `repo_cli_arg()` to `scripts/practical_alpha05_session.py`.
- Updated `_run_session_start()` and `_run_session_host_io()` so repo-owned
  package paths are passed to nested Cargo example commands as repo-relative
  `samples/...` paths.
- Left `_run_session_save()`, `_run_session_load()`, `_run_session_observe()`,
  and session output paths unchanged because they operate on temporary session
  files.
- Added regression tests for repo-owned package relativization, external path
  preservation, start argv, and host-I/O argv.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` to mirror the
  alpha-0.5 path-portability close and the broader remaining candidate list.
- Used a read-only sub-agent audit for broader path-portability candidates and
  closed the sub-agent after receiving findings.

## Files changed

- `scripts/practical_alpha05_session.py`
- `scripts/tests/test_practical_alpha05_session.py`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2196-practical-alpha05-session-path-portability.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short --branch && git rev-parse HEAD && git rev-parse origin/main
rg -l "subprocess\.run\(" scripts --glob '*.py' | sort
rg -n "str\(REPO_ROOT /" scripts --glob '*.py'
rg -n "failed.*str\(error\)|guard_error = str\(error\)|\"error\": str\(error\)" scripts --glob '*.py'
python3 -m unittest scripts.tests.test_practical_alpha05_session
python3 scripts/practical_alpha05_session.py check-all --format json
python3 scripts/practical_alpha05_session.py closeout --format json
python3 -m unittest scripts.tests.test_practical_alpha05_session scripts.tests.test_practical_alpha08_session_hotplug scripts.tests.test_practical_alpha09_devtools scripts.tests.test_practical_alpha1_integrated_workflow
python3 scripts/practical_alpha08_session_hotplug.py check-all --format json
python3 scripts/practical_alpha09_devtools.py check-all --format json
python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json
cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture
cargo test -p mir-runtime --test practical_alpha05_host_io -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py
git diff --check
```

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_practical_alpha05_session`: 7 tests
  passed.
- `python3 scripts/practical_alpha05_session.py check-all --format json`: 7/7
  passed, failed `[]`, repo-root absolute match `0`.
- `python3 scripts/practical_alpha05_session.py closeout --format json`:
  `operational_alpha05_ready=True`, failed `[]`, repo-root absolute match `0`.
- Combined Python unit suite for alpha05/alpha08/alpha09/integrated workflow:
  25 tests passed.
- `python3 scripts/practical_alpha08_session_hotplug.py check-all --format json`:
  10/10 passed, repo-root absolute match `0`.
- `python3 scripts/practical_alpha09_devtools.py check-all --format json`:
  9/9 passed, repo-root absolute match `0`.
- `python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json`:
  8/8 passed, repo-root absolute match `0`.
- `cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture`:
  3 tests passed.
- `cargo test -p mir-runtime --test practical_alpha05_host_io -- --nocapture`:
  2 tests passed.
- `cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture`:
  6 tests passed.
- `python3 scripts/validate_docs.py`: passed.
- `python3 scripts/check_source_hierarchy.py`: required 659, present 659,
  missing 0.
- `git diff --check`: passed.

## What changed in understanding

The focused practical helper list was not fully closed until alpha-0.5 itself
was checked. Alpha-0.8 and alpha-0.9 had already been hardened, but both depend
on the alpha-0.5 session example, so alpha-0.5 needed the same repo-relative
package argv treatment.

The broader audit also identified remaining candidates outside this focused
practical-helper line:

- Surface release-check output path serialization.
- Product alpha installed-binary generated path serialization / argv.
- Full System V1 nested source argv.
- Alpha network Docker success/failure path serialization.
- Current-L2 model-check pipeline and detached-loop repo-owned helper argv.
- Source-hierarchy status JSON.
- Shared practical `failed[].error` redaction on failure paths.

## Open questions

- Which broader path-portability candidate should be promoted next is a task
  ordering question, not a semantic blocker.
- No user decision is required for the next narrow maintenance package.

## Suggested next prompt

Continue the path-portability broader sweep from the highest-confidence
remaining candidate, likely Surface release-check output path serialization or
Product Alpha installed-binary generated path serialization / argv.

## Plan update status

`plan/` 更新不要: no long-term repository memory or normative interpretation
changed.

## Documentation.md update status

`Documentation.md` 更新不要: the top-level reader snapshot is unchanged at this
granularity.

## progress.md update status

`progress.md` 更新済み: added the 2026-07-04 15:27 JST alpha-0.5 path
portability log and broader remaining-candidate note.

## tasks.md update status

`tasks.md` 更新済み: added alpha-0.5 path-portability close and the broader
path-portability sweep candidate.

## samples_progress.md update status

`samples_progress.md` 更新済み: updated the alpha-0.5 workflow row and recent
validation log.

## Reviewer findings and follow-up

Sub-agent `Turing` completed a read-only broader audit and reported no edits.
Its findings were used to record the remaining broader path-portability
candidates. Follow-up should start with one narrow candidate at a time.

## Skipped validations and reasons

- Full Docker / release-check suites were not rerun because this package only
  changes alpha-0.5 session helper package argv and its upper practical workflow
  dependents were validated directly.
- Broader candidates from the read-only audit were not fixed in this package to
  keep the commit scoped.

## Commit / push status

Pending at report write.

## Sub-agent session close status

Sub-agent `019f2bc9-4b5b-77e1-aa80-061e542a907a` completed its read-only audit
and was closed.
