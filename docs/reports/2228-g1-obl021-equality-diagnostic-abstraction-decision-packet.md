# 2228 - G1 OBL-021 equality / diagnostic abstraction decision packet

## Objective

Create a narrow LAB-only decision packet for the OBL-021 equality /
diagnostic abstraction blocker before any OBL-021 requested-status draft.

## Scope and assumptions

- Scope is repository-memory, documentation, and validation discipline only.
- `mirrorea_canon/` remains normative. `plan/` remains LAB repository memory.
- The package may ask which equality / diagnostic / projection / input
  identity boundaries a later OBL-021 status packet is allowed to use.
- The package must not choose requested status, submit a status proposal, edit
  canon, move the metatheory ledger, complete OBL-021, prove OBL-021, create a
  Lean wrapper, refine the Lean predicate, claim conformance, change runtime
  readiness, or claim G1 exit.
- The Oracle and sub-agent results are advisory. Useful findings are mirrored
  into normal repo files instead of becoming external normative state.

## Start state / dirty state

- Start branch: `main`.
- Start tracking state: `main...origin/main`.
- Start dirty state before P90 edits: clean worktree after
  `93babc332710927a2dc2abbb8970c8b285cf2e79`.
- Discord task baseline was recorded before P90 file edits with
  `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`.
- Resource snapshot before docs edits:
  `df -h .` reported `/dev/sda2` size 188G, used 149G, available 30G, use 84%;
  `free -h` reported 15Gi memory total / 10Gi available and 15Gi swap total /
  14Gi free; `du -sk .` reported `7337176`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `progress.md`
- `tasks.md`
- `.docs/progress-task-axes.md`
- `CANON.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/theory/01-mircore-v0.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/77-g1-obl021-lean-statement-draft.md`
- `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`
- `plan/130-g1-obl-statement-status-completion-criteria-inventory.md`
- `plan/133-g1-requested-status-options-matrix.md`
- `plan/139-g1-obl021-artifact-identity-wrapper-preflight.md`
- `plan/140-g1-obl021-artifact-annex-template.md`
- `plan/141-g1-status-packet-shell-unresolved-slots.md`
- `plan/142-g1-status-packet-shell-evidence-dry-run.md`
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- `.agents/skills/discord-report/SKILL.md`
- `.docs/oracle-chatgpt-pro-operations.md`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `/home/codex/.codex/superpowers/skills/using-superpowers/SKILL.md`
- `/home/codex/.codex/superpowers/skills/subagent-driven-development/SKILL.md`
- `/home/codex/.codex/superpowers/skills/requesting-code-review/SKILL.md`
- `/home/codex/.codex/superpowers/skills/verification-before-completion/SKILL.md`

## Actions taken

- Asked a read-only sub-agent to inspect the OBL-021 packet boundary for
  missing sections, unresolved choices, overclaims, and stale wording.
- Asked ChatGPT 5.5 Pro Extended Oracle for an advisory review of the same
  boundary. The result was treated as non-normative and checked against repo
  evidence before being mirrored.
- Added `plan/143-g1-obl021-equality-diagnostic-abstraction-decision-packet.md`.
- Added a later-reading note to `plan/77` clarifying that replacing the
  abstract predicates is now an option, not an already accepted requirement.
- Registered `plan/143` in source-hierarchy and docs validation scaffolds.
- Updated `README.md`, `Documentation.md`, `progress.md`, `tasks.md`,
  `scripts/README.md`, `plan/00-index.md`, and
  `plan/90-source-traceability.md`.
- Closed the completed read-only sub-agent after incorporating its findings.

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `plan/00-index.md`
- `plan/77-g1-obl021-lean-statement-draft.md`
- `plan/90-source-traceability.md`
- `plan/143-g1-obl021-equality-diagnostic-abstraction-decision-packet.md`
- `docs/reports/2228-g1-obl021-equality-diagnostic-abstraction-decision-packet.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short --branch`
- `df -h .`
- `free -h`
- `du -sk .`
- `sed -n '1,260p' README.md`
- `sed -n '1,260p' Documentation.md`
- `sed -n '1,260p' specs/00-document-map.md`
- `sed -n '1,260p' specs/01-charter-and-decision-levels.md`
- `sed -n '1,260p' specs/02-system-overview.md`
- `sed -n '1,260p' specs/03-layer-model.md`
- `sed -n '1,260p' specs/09-invariants-and-constraints.md`
- `sed -n '1,260p' progress.md`
- `sed -n '1,280p' tasks.md`
- `sed -n '1,220p' .docs/progress-task-axes.md`
- `sed -n '1,220p' /home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `sed -n '1,220p' .docs/oracle-chatgpt-pro-operations.md`
- `ask-chatgpt-pro --prompt-file /tmp/mirrorea-obl021-oracle-prompt.md --name mirrorea-repo-advisory-review-request --attach ...`
- `sed -n '1,260p' plan/77-g1-obl021-lean-statement-draft.md`
- `sed -n '1,280p' plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`
- `sed -n '1,280p' plan/130-g1-obl-statement-status-completion-criteria-inventory.md`
- `sed -n '1,260p' plan/133-g1-requested-status-options-matrix.md`
- `sed -n '1,280p' plan/139-g1-obl021-artifact-identity-wrapper-preflight.md`
- `sed -n '1,320p' plan/140-g1-obl021-artifact-annex-template.md`
- `sed -n '1,320p' plan/141-g1-status-packet-shell-unresolved-slots.md`
- `sed -n '1,260p' plan/142-g1-status-packet-shell-evidence-dry-run.md`
- `sed -n '1,240p' samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- `rg -n "plan/00\\.\\.142|plan/39\\.\\.142|plan/70\\.\\.142|plan/118\\.\\.142|three axes|fixed-input" ...`
- `sed -n '1,260p' plan/143-g1-obl021-equality-diagnostic-abstraction-decision-packet.md`
- `sed -n '240,360p' plan/143-g1-obl021-equality-diagnostic-abstraction-decision-packet.md`
- `git diff --stat`
- `git rev-parse HEAD`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 scripts/check_source_hierarchy.py --format json | jq '{status, required_count, present_count, missing_count}'`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `git diff --check`
- `lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- Initial over-broad admitted-stub / placeholder `rg` scan over the OBL-021
  Lean artifact and sync-guard test file.
- Corrected OBL-021 Lean artifact admitted-stub / placeholder `rg` scan.
- Tracked Discord webhook full URL / token-prefix scan excluding
  `.codex-discord`.
- Post-edit `du -sk .`.

## Evidence / outputs / test results

- Sub-agent advisory review identified the needed packet sections, non-claims,
  and stale `plan/77` wording. The completed agent was closed after its result
  was mirrored into repo memory.
- Oracle advisory review recommended keeping the packet narrower than
  `plan/140` and `plan/141`, and adding the fixed-input identity /
  non-vacuity boundary before any status drafting.
- Focused stale-range scan found no remaining `plan/...142` numbered-current
  range in the touched reader-facing files. `plan/143` fixed-input wording was
  present in the expected files.
- Post-edit `python3 scripts/check_source_hierarchy.py --format json | jq ...`:
  status `ok`, required 683, present 683, missing 0.
- Post-edit `python3 -m unittest scripts.tests.test_validate_docs`: 37 tests
  passed.
- Post-edit `python3 scripts/validate_docs.py`: documentation scaffold
  complete, 1380 numbered reports.
- Post-edit `git diff --check`: passed.
- Post-edit `lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`:
  passed.
- Post-edit `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`:
  21 tests passed.
- Initial admitted-stub / placeholder scan was intentionally too broad and
  matched test fixture strings in `scripts/tests/test_current_l2_lean_sample_sync.py`.
  Root cause was scan scope, not an OBL-021 artifact stub.
- Corrected OBL-021 Lean artifact admitted-stub / placeholder scan: passed.
- Post-edit tracked Discord webhook secret scan: passed.
- Repository size after docs edits: `du -sk .` reported `7337292`; this is
  approximately 116 KiB more than the pre-edit `7337176` reading. No heavy
  artifact was created.

## What changed in understanding

The OBL-021 blocker is not only a two-way equality / diagnostic choice. The
status-drafting blocker now has four explicit axes: result equality,
diagnostic equivalence, artifact identity / wrapper, and fixed-input identity /
non-vacuity. This lets later work ask for a precise human/canon decision
without laundering abstract LAB predicates into accepted canon status.

## Open questions

- Is abstract component result equivalence acceptable as OBL-021
  statement-status vocabulary at this checkpoint?
- Is abstract diagnostic equivalence acceptable as OBL-021 statement-status
  vocabulary at this checkpoint?
- Does OBL-021 status drafting require direct LAB artifact acceptance,
  canon-facing wrapper work, or deferral?
- Is the current fixed-input abstraction non-vacuous enough, or must the
  project first define canonical input equality / snapshot and projection
  totality / uniqueness constraints?

## Suggested next prompt

Prepare an OBL-020 full-row vs G1-supporting scope decision packet without
choosing requested status or editing canon.

## Plan update status

Updated: added `plan/143`, updated `plan/00-index.md`, added a source
traceability row in `plan/90-source-traceability.md`, and added a later-reading
note in `plan/77`.

## Documentation.md update status

Updated: `Documentation.md` now mentions `plan/143` as an OBL-021
abstraction-boundary decision packet with no status / proposal / ledger /
proof / conformance / runtime / G1-exit claim.

## progress.md update status

Updated: `progress.md` now records `plan/143`, updates Macro 5 and the LAB
Lean statement row, and adds a timestamped recent-log entry.

## tasks.md update status

Updated: `tasks.md` now records `plan/143`, removes the completed OBL-021
decision packet from candidate next strategy packages, and promotes the next
safe package as OBL-020 scope decision preparation.

## samples_progress.md update status

samples_progress.md 更新不要: no runnable sample path, validation command,
debug surface, sample blocker, or sample readiness status changed.

## Reviewer findings and follow-up

- Read-only code-mapper sub-agent: recommended adding explicit packet sections,
  preserving requested-status / ledger / proof / conformance / G1-exit
  non-claims, and correcting stale `plan/77` wording. Follow-up was applied.
- Oracle advisory review: recommended adding fixed-input identity /
  non-vacuity as a separate blocker. Follow-up was applied.
- Final read-only reviewer `019f2d9d-379a-7b41-b5d1-d2024db7fb04`: one medium
  report-closeout finding that validation, commit/push, and reviewer/sub-agent
  statuses needed concrete values before package close. Follow-up was applied
  in this report update. The reviewer found no semantic overclaim in
  `plan/143`, no accidental status / ledger / proof / conformance / runtime /
  G1-exit claim, and no stale touched-file `plan/...142` range.

## Skipped validations and reasons

No validation is intentionally skipped.

The first admitted-stub / placeholder scan included the sync-guard test file
and therefore found deliberate negative-test literals such as `:= True` and
`sorry`. That command was not used as evidence of OBL-021 artifact failure; it
was replaced with a correctly scoped scan of the OBL-021 Lean artifact itself,
which passed.

## Commit / push status

Commit and push are performed after validation and final review. The first
package commit will include this report and the substantive files; a follow-up
report-closeout commit will record the exact pushed commit hash in this
section.

## Sub-agent session close status

- Code-mapper sub-agent `019f2d8d-fd13-7793-b454-88ca5929bbd0` completed and
  was closed after its advisory result was incorporated.
- Final reviewer sub-agent `019f2d9d-379a-7b41-b5d1-d2024db7fb04` completed
  and reported only the report-closeout finding listed above. It was closed
  after this follow-up was incorporated.
