# 2118 — mirrorea_canon consistency and LAB downgrade guardrails

## Objective

Inspect newly added `mirrorea_canon/` as the intended big-picture canon, check
its consistency against the existing LAB repository, and add minimal T0
guardrails so future agents do not continue to treat legacy LAB `specs/` as the
normative source.

## Scope and assumptions

Scope was limited to source-hierarchy adoption guardrails and consistency
inspection:

- Read canon and existing LAB entry points.
- Check canon index / front matter / links / OPEN references at a lightweight
  level.
- Update LAB entry points to point to canon first.
- Preserve legacy `specs/`, `plan/`, samples, and reports as LAB evidence.
- Do not change canon L0/L1 semantic content, ADR status, scenario
  expectations, gate criteria, or theorem status.

Working assumption: the user's instruction that `mirrorea_canon/` should be the
big-picture source is the human decision needed to perform the LAB-side T0
downgrade guardrail. This report does not claim G0 is fully closed; a
LAB-to-canon reconciliation ledger remains open.

## Start state / dirty state

Start state was clean:

- `git status --short --branch`: `## main...origin/main`
- `mirrorea_canon/` was already tracked with 71 files.
- `du -sh mirrorea_canon`: `364K`

Initial finding: canon itself declared `mirrorea_canon/` as normative, while
root LAB docs still said `specs/` was the normative source.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `AGENTS.md`
- `.docs/progress-task-axes.md`
- `.docs/oracle-chatgpt-pro-operations.md`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/NORTH-STAR.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/GLOSSARY.md`
- `mirrorea_canon/CHANGELOG.md`
- `mirrorea_canon/INDEX.json`
- `mirrorea_canon/meta/source-hierarchy.md`
- `mirrorea_canon/meta/agent-instructions.md`
- `mirrorea_canon/meta/style-guide.md`
- `mirrorea_canon/adr/README.md`
- `mirrorea_canon/adr/ADR-0001.md` through `ADR-0012.md`
- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/plan/02-operating-model.md`
- `mirrorea_canon/plan/03-risks.md`
- `mirrorea_canon/spec/README.md`
- `mirrorea_canon/spec/01-surface-syntax.md` through `07-minimal-language.md`
- `mirrorea_canon/theory/README.md`
- `mirrorea_canon/theory/00-overview.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `mirrorea_canon/architecture/01-strata.md`
- `mirrorea_canon/architecture/02-boundary-contracts.md`
- `mirrorea_canon/scenarios/README.md`
- `samples/clean-near-end/README.md`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`

## Actions taken

- Added root `CANON.md` as the repo-level source-hierarchy entry point.
- Added canon notices to `README.md`, `Documentation.md`, `AGENTS.md`,
  `progress.md`, and `tasks.md`.
- Changed AGENTS read order to canon-first, then LAB evidence.
- Reclassified `Documentation.md`, `progress.md`, `tasks.md`, and
  `plan/00-index.md` role summaries so legacy `specs/` / `plan/` are LAB
  evidence rather than the normative source.
- Added T0/G0 canon adoption status and next reconciliation gap to
  `progress.md` / `tasks.md`.
- Added a historical LAB note to `samples/clean-near-end/README.md`.
- Removed `world` from `mir_hilight.html` highlighted keyword /
  declaration-core list and documented it as legacy LAB vocabulary.
- Added canon entry files to `scripts/check_source_hierarchy.py`.
- Added required canon notice checks to `scripts/validate_docs.py`.
- Updated validator unit tests for canon entry files and missing canon notice
  rejection.
- Consulted a read-only sub-agent reviewer and ChatGPT Pro Extended Oracle.

## Files changed

- `CANON.md`
- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `samples/clean-near-end/README.md`
- `mir_hilight.html`
- `specs/00-document-map.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `docs/reports/2118-mirrorea-canon-consistency-and-lab-downgrade.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short --branch`
- `find mirrorea_canon -maxdepth 3 -type f | sort`
- `du -sh mirrorea_canon`
- `git ls-files mirrorea_canon | wc -l`
- `wc -l mirrorea_canon/**/*.md`
- `python3 meta/build-index.py --check` from `mirrorea_canon/`
- `python3 -m json.tool INDEX.json >/tmp/mirrorea_canon_index_pretty.json`
  from `mirrorea_canon/`
- `python3 meta/build-index.py` from `mirrorea_canon/`
- lightweight local front-matter/status check for canon markdown files
- lightweight local markdown link check for canon markdown links
- lightweight local `INDEX.json` status/maturity summary
- lightweight local OPEN-reference check for canon OPEN IDs
- `test -f CANON.md`
- `rg` scans for legacy source-hierarchy wording and `world` vocabulary
- `ask-chatgpt-pro ...` for Oracle review
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 -m unittest discover -s scripts/tests`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `python3 mirrorea_canon/meta/build-index.py --check` from repo root
- `python3 meta/build-index.py --check` from `mirrorea_canon/`
- `git diff --check`

## Evidence / outputs / test results

Canon inspection evidence:

- `python3 meta/build-index.py --check` from `mirrorea_canon/`: `ok: 69 files indexed`
- `python3 meta/build-index.py` from `mirrorea_canon/`: no `INDEX.json` diff
- front-matter/status check: `markdown files 69`, `errors 0`
- markdown link check: `missing markdown links 0`
- `INDEX.json` summary: `canon_version 0.1.0`, `files 69 entries 69`
- canon status counts: `L1-fixed: 39`, `L0-frozen: 22`, `L2-working: 8`
- canon maturity count: `draft: 69`
- max canon file size observed: `9639 bytes`, under the 15KB style-guide limit
- OPEN-reference check: `open refs 30`, `open front matter defs 30`, no missing
  or unreferenced OPEN IDs found

Post-edit validation:

- `python3 -m unittest scripts.tests.test_validate_docs`: ran 20 tests, OK
- `python3 -m unittest discover -s scripts/tests`: ran 642 tests, OK
- `python3 scripts/check_source_hierarchy.py`: required 556, present 556,
  missing 0
- `python3 scripts/validate_docs.py`: documentation scaffold complete, 1270
  numbered reports found
- `python3 meta/build-index.py --check` from `mirrorea_canon/`: `ok: 69 files indexed`
- `git diff --check`: no whitespace errors reported

One command was intentionally recorded as failed and not counted as validation
success:

- `python3 mirrorea_canon/meta/build-index.py --check` from repo root returned
  `canon root not found`. The same check was rerun with cwd `mirrorea_canon/`
  and passed.

## What changed in understanding

`mirrorea_canon/` is internally coherent enough to serve as the big-picture
canon: it has a clear source hierarchy, ADR set, gate/phase plan, scenario
suite, conformance definition, and metatheory ledger.

The main problem was not canon-internal content. It was source-hierarchy
split-brain: LAB entry points still instructed agents to treat `specs/` as
normative. That is now guarded at the root entry level.

The next substantive line is not broad runtime implementation. Canon places the
project at T0/G0 rebaseline. The next management work is LAB-to-canon
reconciliation, followed by G1 ordinary-assignment / elaboration target work.

## Open questions

- How detailed should the LAB-to-canon reconciliation ledger be for old
  `specs/`, `plan/`, and report claims?
- Should `mirrorea_canon/mental-model/README.md` wording be clarified to say it
  is a canonical explanatory source subordinate to theory/spec?
- Which old LAB claims should be rejected explicitly versus left as historical
  evidence with no canon destination?
- Should validators later scan for unqualified old `specs/` normative wording
  beyond root entry docs?

## Suggested next prompt

Execute the T0 LAB-to-canon reconciliation ledger package: map legacy LAB
`specs/`, `plan/`, and snapshot claims to canon IDs, rejected historical claims,
or OPEN entries; do not change canon L0/L1 decisions without proposal, human
decision, and ADR/changelog/index updates where required.

## Plan update status

`plan/00-index.md` updated to point to `mirrorea_canon/` first and classify the
legacy `plan/` tree as LAB evidence / historical repository memory.

## Documentation.md update status

`Documentation.md` updated with a canon notice and source-hierarchy role list:
`mirrorea_canon/` is normative; legacy `specs/` / `plan/` are LAB evidence /
repository memory.

## progress.md update status

`progress.md` updated with canon notice, T0/G0 canon position, migration note,
and a timestamped recent log entry.

## tasks.md update status

`tasks.md` updated with canon notice, T0/G0 holding state, and next candidate
package `LAB-to-canon reconciliation ledger`.

## samples_progress.md update status

`samples_progress.md` 更新不要。Runnable sample status did not change; only the
source-hierarchy reading of existing LAB evidence changed.

## Reviewer findings and follow-up

Read-only sub-agent reviewer found:

- High: source hierarchy split-brain between LAB `specs/` normative wording and
  canon `mirrorea_canon/` normative wording.
- Medium: canon T0 LAB downgrade steps were not applied.
- Medium: validators did not catch the new hierarchy conflict.
- Low: `mental-model/` wording should eventually clarify explanatory status.

Oracle review independently agreed:

- `canon > LAB` is adoptable, but only as an explicit T0 governance rebaseline.
- Main contradiction is source hierarchy, not theory content.
- Current LAB progress language is acceptable only as frozen evidence.
- A reconciliation ledger is the next important management artifact.

Follow-up applied in this task: root `CANON.md`, entry notices, AGENTS read
order, validator guardrails, `world` highlighter cleanup, and clean sample LAB
note. Follow-up still open: reconciliation ledger and optional mental-model
wording cleanup.

## Skipped validations and reasons

- Full Cargo workspace build/test was skipped because this task only changed
  documentation, validator scripts, and a static HTML highlighter keyword list.
- Sample helper execution was skipped because no sample source, expected output,
  or runnable sample dashboard status changed.
- Browser rendering of `mir_hilight.html` was skipped because the change only
  removes `world` from keyword/declaration matching; no layout or asset behavior
  was changed.

## Commit / push status

Pending at report-write time. This task should be committed with
`git commit --no-gpg-sign` and pushed after final validation. The final response
records the exact commit and push status.

## Sub-agent session close status

Sub-agent reviewer completed and was closed. Oracle consult completed and was
folded into this report as advisory input, not as normative state.
