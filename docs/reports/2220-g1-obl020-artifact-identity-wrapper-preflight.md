# 2220 - G1 OBL-020 artifact identity / wrapper preflight

## Objective

Clarify how the current OBL-020 LAB Lean artifact may be cited before any
future requested-status packet.

The package decides only a LAB advisory posture: direct citation is allowed as
LAB evidence, but requested-status artifact identity still needs an
artifact-identity annex / wrapper decision. Actual wrapper creation is deferred
unless human/canon review requires it.

## Scope and assumptions

Scope:

- Add LAB repository memory for OBL-020 artifact identity / wrapper preflight.
- Synchronize `plan/` index / traceability, root snapshot docs, validators,
  `progress.md`, `tasks.md`, and this report.
- Use a read-only sub-agent review and Oracle advisory review as critique
  input.

Assumptions:

- `mirrorea_canon/` remains the normative source.
- `plan/` and legacy `specs/` remain LAB evidence / repository memory.
- The canon metatheory ledger remains the only OBL status authority.
- No canon edit, ledger movement, wrapper file, or requested-status proposal
  is in scope.

## Start state / dirty state

Start state:

- Branch: `main`.
- `HEAD == origin/main` before P82 edits.
- Working tree was clean before P82 edits.

During work:

- P81 substantive commit and report-status follow-up had already been pushed.
- P82 started with a Discord `begin` baseline.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/theory/01-mircore-v0.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/00-index.md`
- `plan/78-g1-obl020-lean-statement-draft.md`
- `plan/90-source-traceability.md`
- `plan/130-g1-obl-statement-status-completion-criteria-inventory.md`
- `plan/131-g1-status-proposal-packet-outline.md`
- `plan/133-g1-requested-status-options-matrix.md`
- `plan/134-g1-obl020-scope-clarification-packet.md`
- `progress.md`
- `tasks.md`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.md`
- `samples/lean/manifest.json`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`

## Actions taken

- Added `plan/135-g1-obl020-artifact-identity-wrapper-preflight.md`.
- Recorded the P82 advisory posture:
  - direct citation of `MirCore.Lab.OBL020.StatementDraft.OBL020StatementDraft`
    is safe only as LAB evidence;
  - requested-status artifact identity requires a later artifact annex /
    wrapper decision;
  - actual wrapper creation should wait until human/canon review requires one.
- Updated `plan/00-index.md` and `plan/90-source-traceability.md`.
- Registered `plan/135` in docs / source-hierarchy validators.
- Updated `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, and
  `scripts/README.md`.
- Consulted a read-only sub-agent reviewer.
- Started Oracle consult `advisory-review-for-mir-poc`.

## Files changed

- `README.md`
- `Documentation.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/135-g1-obl020-artifact-identity-wrapper-preflight.md`
- `progress.md`
- `tasks.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `docs/reports/2220-g1-obl020-artifact-identity-wrapper-preflight.md`

## Commands run

Setup / status:

```text
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short --branch
date '+%Y-%m-%d %H:%M %Z'
```

Reading / search:

```text
sed -n ... plan/134-g1-obl020-scope-clarification-packet.md
sed -n ... plan/78-g1-obl020-lean-statement-draft.md
sed -n ... samples/lean/lab-statements/obl020/StepWFStatementDraft.lean
sed -n ... samples/lean/lab-statements/obl020/StepWFStatementDraft.md
rg -n ...
```

Advisory review:

```text
ask-chatgpt-pro --file ... -p "Advisory review for mir_poc_01 P82 ..."
oracle status --hours 1 --limit 5
```

Validation so far:

```text
python3 scripts/check_source_hierarchy.py --format json | jq '{status, required_count, present_count, missing_count}'
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/validate_docs.py
git diff --check
tracked Discord webhook full URL / token-prefix secret scan
lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
```

## Evidence / outputs / test results

Source hierarchy structural check:

```json
{
  "status": "ok",
  "required_count": 675,
  "present_count": 675,
  "missing_count": 0
}
```

Docs validator unit tests:

- `python3 -m unittest scripts.tests.test_validate_docs`
- Result: 37 tests OK.

Report-inclusive docs validation:

- `python3 scripts/validate_docs.py`
- Result: pass, "Documentation scaffold looks complete. Found 1372 numbered
  report(s)."

Diff whitespace check:

- `git diff --check`
- Result: pass, no output.

Secret scan:

- Full Discord webhook URL scan and webhook token-prefix scan over tracked
  files, excluding `.codex-discord`.
- Result: pass.

OBL-020 Lean compile-check:

- `lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- Result: pass, no output.

Lean sync guards:

- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- Result: 21 tests OK.

## What changed in understanding

P82 separates four different artifact concepts:

- LAB evidence citation;
- requested-status artifact identity;
- possible canon-facing wrapper;
- canon ledger target.

The current LAB Lean artifact may be cited directly as evidence that the
G1-supporting abstract OBL-020 statement shape exists and compile-checks. It
should not be treated as the requested-status artifact without a later
artifact-identity annex / wrapper decision.

## Open questions

- Whether human/canon review accepts the LAB path / namespace / constant as the
  OBL-020 requested-status artifact.
- Whether a canon-facing wrapper is required.
- Whether artifact identity should be deferred until concrete `Config`,
  `Step`, and `WellFormed` definitions are chosen.

## Suggested next prompt

Proceed with an OBL-020 artifact annex template, mapping canon OBL-020 target,
LAB path / namespace / constant, scope label, validation evidence, and
non-claims for a later requested-status packet without creating a wrapper or
moving the ledger.

## Plan update status

Updated.

- Added `plan/135-g1-obl020-artifact-identity-wrapper-preflight.md`.
- Updated `plan/00-index.md`.
- Updated `plan/90-source-traceability.md`.

## Documentation.md update status

Updated.

- Added a concise `plan/135` artifact-identity / wrapper preflight note.

## progress.md update status

Updated.

- Updated timestamp to `2026-07-04 21:46 JST`.
- Added current G1 OBL-020 artifact identity / wrapper preflight note.
- Updated Macro 5 addendum.
- Added recent log entry.

## tasks.md update status

Updated.

- Updated timestamp to `2026-07-04 21:46 JST`.
- Added `plan/135` holding-state and detailed task-map notes.
- Moved the candidate next package from OBL-020 artifact-identity / wrapper
  preflight to OBL-020 artifact annex template.
- Added OBL-001 canon-facing wrapper preflight as another candidate.
- Updated plan scaffold ranges to `00..135`, `39..135`, and `118..135`.

## samples_progress.md update status

samples_progress.md 更新不要.

No runnable sample status, validation command, debug surface, or sample blocker
changed. P82 is docs-only and creates no wrapper file.

## Reviewer findings and follow-up

Read-only sub-agent reviewer
`019f2d27-2cdd-7790-8b4e-e7e1b1f15736` completed and was closed.

Sub-agent finding:

- Direct citation is safe only as LAB evidence, not as requested-status
  artifact identity.
- Recommended posture is artifact-identity annex / wrapper decision before any
  requested-status packet; prefer annex first.
- Direct-citation risks include silent LAB namespace promotion, abstract
  predicate overread, proof/conformance/G1-exit collapse, and full OBL-020
  completion overclaim.
- Wrapper risks include silent canon promotion, premature namespace/API freeze,
  duplicate statement identity drift, and hidden full-vs-G1 scope ambiguity.

Follow-up applied:

- `plan/135` recommends annex-only preflight now and defers wrapper creation
  until human/canon review requires it.

Oracle consult `advisory-review-for-mir-poc` completed.

Oracle finding:

- Recommended wrapper / artifact-identity annex before any status request.
- Direct citation is permitted only as subordinate LAB statement-shape
  evidence.
- Deferral is fallback only if canon/human review rejects scoped artifact
  identity or requires concrete definitions / proof-package binding first.
- Safe wording must keep the LAB artifact, namespace, and constant separate
  from the canon `MirCore.Step.WF` target and open ledger status.

Note:

- The shell prompt that launched Oracle accidentally command-substituted the
  inline backtick identifiers before sending them, but the attached files
  contained the exact artifact and canon target. The answer correctly used
  those attached-file identifiers, and its recommendation matched local source
  review and sub-agent review.

## Skipped validations and reasons

Skipped:

- Cargo tests and runtime sample helpers.
- Full Surface / Full System V1 sample suites.
- Lean wrapper compile-check, because no wrapper file was created.

Reason:

- P82 did not change Rust source, runtime helper logic, executable samples,
  generated sample artifacts, or Lean source.
- P82 reran the existing OBL-020 Lean artifact compile-check and OBL statement
  sync guard suite as artifact evidence checks.

## Commit / push status

Not yet committed at initial report creation.

## Sub-agent session close status

Sub-agent reviewer `019f2d27-2cdd-7790-8b4e-e7e1b1f15736` completed and was
closed.
