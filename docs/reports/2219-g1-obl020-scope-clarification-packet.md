# 2219 - G1 OBL-020 scope clarification packet

## Objective

Clarify the safe OBL-020 scope posture before any future G1 status proposal.

The package decides only a LAB advisory position: full-row OBL-020 status
movement remains deferred; the next safe review target is a G1-supporting
statement-scope candidate; proof-package deferral is fallback only if scoped
statement identity is rejected.

## Scope and assumptions

Scope:

- Create LAB repository memory for the OBL-020 full-vs-G1-supporting scope
  boundary.
- Synchronize root snapshot docs, `plan/` index / traceability, validator
  registration, `progress.md`, `tasks.md`, and this report.
- Use read-only sub-agent review and Oracle advisory review as critique input.

Assumptions:

- `mirrorea_canon/` remains the normative source.
- `plan/` and legacy `specs/` remain LAB evidence / repository memory.
- The canon metatheory ledger remains the only OBL status authority.
- No canon edit or ledger movement is in scope.

## Start state / dirty state

Start state:

- Branch: `main`.
- `HEAD == origin/main` before P81 edits.
- Working tree was clean before P81 edits.

During work:

- P80 substantive commit and report-status follow-up had already been pushed.
- P81 started with a Discord `begin` baseline.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `.docs/progress-task-axes.md`
- `.docs/oracle-chatgpt-pro-operations.md`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/theory/01-mircore-v0.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/00-index.md`
- `plan/76-g1-obl020-021-dependency-inventory.md`
- `plan/90-source-traceability.md`
- `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`
- `plan/130-g1-obl-statement-status-completion-criteria-inventory.md`
- `plan/131-g1-status-proposal-packet-outline.md`
- `plan/132-g1-status-evidence-readiness-dry-run.md`
- `plan/133-g1-requested-status-options-matrix.md`
- `progress.md`
- `tasks.md`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`

## Actions taken

- Added `plan/134-g1-obl020-scope-clarification-packet.md`.
- Recorded the P81 advisory posture:
  - B: G1-supporting statement-scope clarification is the recommended posture.
  - A: full-row OBL-020 status movement is deferred.
  - C: proof-package deferral is fallback only if scoped statement identity is
    rejected.
- Updated `plan/00-index.md` and `plan/90-source-traceability.md`.
- Registered `plan/134` in docs / source-hierarchy validators.
- Updated `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, and
  `scripts/README.md`.
- Ran OBL-020 Lean compile-check, Lean sync guards, docs unit tests, and source
  hierarchy structural check.
- Consulted a read-only sub-agent reviewer and Oracle.

## Files changed

- `README.md`
- `Documentation.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/134-g1-obl020-scope-clarification-packet.md`
- `progress.md`
- `tasks.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `docs/reports/2219-g1-obl020-scope-clarification-packet.md`

## Commands run

Setup / status:

```text
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short --branch
date '+%Y-%m-%d %H:%M %Z'
```

Reading / search:

```text
sed -n ... README.md Documentation.md specs/... mirrorea_canon/... plan/... progress.md tasks.md
rg -n ...
```

Advisory review:

```text
ask-chatgpt-pro --file ... -p "We need an advisory review ..."
oracle status --hours 2 --limit 10
oracle session we-need-an-advisory-review-2
```

Validation:

```text
lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py --format json | jq '{status, required_count, present_count, missing_count}'
python3 scripts/validate_docs.py
git diff --check
tracked Discord webhook full URL / token-prefix secret scan
```

## Evidence / outputs / test results

OBL-020 Lean compile-check:

- `lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- Result: pass, no output.

Lean sync guards:

- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- Result: 21 tests OK.

Docs validator unit tests:

- `python3 -m unittest scripts.tests.test_validate_docs`
- Result: 37 tests OK.

Source hierarchy structural check:

```json
{
  "status": "ok",
  "required_count": 674,
  "present_count": 674,
  "missing_count": 0
}
```

Report-inclusive docs validation:

- `python3 scripts/validate_docs.py`
- Result: pass, "Documentation scaffold looks complete. Found 1371 numbered
  report(s)."

Diff whitespace check:

- `git diff --check`
- Result: pass, no output.

Secret scan:

- Full Discord webhook URL scan and webhook token-prefix scan over tracked
  files, excluding `.codex-discord`.
- Result: pass.

## What changed in understanding

The safest OBL-020 posture is now explicit:

- Current LAB evidence is strong enough for a G1-supporting statement-scope
  review candidate.
- It is not strong enough for full-row OBL-020 movement because the current
  Lean draft leaves `Config`, `StepLabel`, `StepFamily`, `WellFormed`, `Step`,
  concrete WF clauses, scheduler semantics, and per-step preservation lemmas
  abstract.
- Deferring all OBL-020 statement-scope work until proof would overconstrain
  G1 and collapse T1 statement preparation into T2 proof work.

## Open questions

- Whether human/canon review accepts the abstract OBL-020 vocabulary as a
  G1-supporting statement-scope artifact.
- Whether a canon-facing wrapper or artifact-identity annex is required before
  any later `lean-stated` request.
- Whether future full-row OBL-020 work requires concrete definitions before
  any status proposal.

## Suggested next prompt

Proceed with an OBL-020 artifact-identity / wrapper preflight, using
`plan/134` to decide whether the current LAB `OBL020StatementDraft` can be
cited directly as scope evidence or needs a canon-facing wrapper before any
status proposal draft.

## Plan update status

Updated.

- Added `plan/134-g1-obl020-scope-clarification-packet.md`.
- Updated `plan/00-index.md`.
- Updated `plan/90-source-traceability.md`.

## Documentation.md update status

Updated.

- Added a concise `plan/134` scope-clarification note.

## progress.md update status

Updated.

- Updated timestamp to `2026-07-04 21:31 JST`.
- Added current G1 OBL-020 scope clarification note.
- Added Macro 5 latest addendum.
- Added recent log entry.

## tasks.md update status

Updated.

- Updated timestamp to `2026-07-04 21:31 JST`.
- Added `plan/134` holding-state and detailed task-map notes.
- Moved the candidate next package from OBL-020 scope clarification to
  OBL-020 artifact-identity / wrapper preflight.
- Updated plan scaffold ranges to `00..134`, `39..134`, and `118..134`.

## samples_progress.md update status

samples_progress.md 更新不要.

No runnable sample status, validation command, debug surface, or sample blocker
changed. P81 is docs-only plus OBL-020 Lean compile-check confirmation.

## Reviewer findings and follow-up

Read-only sub-agent reviewer
`019f2d1a-2a06-7180-8750-17a85efb7620` completed and was closed.

Sub-agent finding:

- Recommend B, not A or C: use narrower G1-supporting OBL-020 statement-scope
  clarification; defer full-row OBL-020 status movement.
- A overclaims current evidence because the LAB Lean draft is abstract while
  canon full-row OBL-020 ranges over concrete runtime configuration, WF
  clauses, step sketches, scheduler/conformance boundaries, and per-step
  preservation.
- C underclaims and stalls the wrong gate because proof skeletons belong later
  to T2.
- Main laundering risk is treating `MirCore.Lab...` as accepted
  `MirCore.Step.WF`.

Oracle consult `we-need-an-advisory-review-2` completed.

Oracle finding:

- P81 should clarify scope, not propose status movement.
- The current LAB OBL-020 Lean draft supports G1-facing statement-scope
  clarification only.
- Full-row movement remains unsafe without concrete runtime configuration,
  step taxonomy, scheduler model, and per-rule preservation lemmas.
- Blanket deferral until proof is too strong because proof discharge is later
  than statement-scope clarification.

Follow-up applied:

- `plan/134` records B as advisory recommendation, A as deferred, and C as
  fallback only if scoped statement identity is rejected.
- `plan/134` keeps full-row movement, proof, conformance, runtime, and G1 exit
  non-claims explicit.

## Skipped validations and reasons

Skipped:

- Cargo tests and runtime sample helpers.
- Full Surface / Full System V1 sample suites.
- OBL-001 / OBL-021 direct Lean compile-checks.

Reason:

- P81 did not change Rust source, runtime helper logic, executable samples,
  generated sample artifacts, or OBL-001 / OBL-021 Lean files.
- P81 directly concerns OBL-020 scope, so the OBL-020 Lean draft and
  OBL-001/020/021 sync guard suite were rerun.

## Commit / push status

Not yet committed at initial report creation.

## Sub-agent session close status

Sub-agent reviewer `019f2d1a-2a06-7180-8750-17a85efb7620` completed and was
closed.
