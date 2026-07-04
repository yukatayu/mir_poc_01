# 2163 - G1 OBL-024 association guard hardening

## Title and identifier

- Identifier: `2163-g1-obl024-association-guard-hardening`
- Package: G1 OBL-024 association guard hardening
- Timestamp: 2026-07-04 09:26 JST

## Objective

Harden the LAB-only OBL-024 Lean sync guard so the report-local association key
cannot silently drift into semantic key-equality association, branch-local
association-key vocabulary, final-looking ABI names, or key comparability /
uniqueness assumptions.

## Scope and assumptions

- Scope is Lean sync tests, OBL-024 Lean explanation, repository memory, and
  snapshot docs.
- `mirrorea_canon/` remains normative and was not edited.
- Runtime code, expected JSON fixtures, repair output, and executable
  diagnostic behavior are out of scope.
- OBL-024 remains compile-check-only. This package does not prove or complete
  OBL-024.

## Start state / dirty state

- Started from clean pushed `main` at `d57a0d0b`.
- Discord task baseline was recorded with
  `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`.
- The prior package had separated scoped semantic association, report-local key
  compatibility, and future proof-level association relation vocabulary.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `specs/00-overview.md`
- `specs/01-core-model.md`
- `specs/02-effects-and-typing.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `CANON.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/81-g1-obl024-statement-shape-inventory.md`
- `plan/109-g1-obl024-lean-statement-draft.md`
- `plan/112-g1-obl024-replay-vocabulary-preflight.md`
- `plan/113-g1-obl024-lean-replay-vocabulary-refinement.md`
- `plan/114-g1-obl024-lean-association-vocabulary-refinement.md`
- `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
- `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- Volta reviewer findings from package 2162

## Actions taken

- Added `plan/115-g1-obl024-association-guard-hardening.md`.
- Strengthened `test_obl024_draft_names_association_vocabulary_boundary` with
  negative guards for final-looking request / branch / ABI names,
  association-key comparability / uniqueness pressure, and direct
  `DiagnosticBranch` / `ReportLocalAssociationKey` carrier shapes.
- Added positive structural guard anchors for `CurrentEvidenceBoundary`,
  `CoveredDiagnosticSoundnessCase`, and `Rejects`.
- Added explanation text saying the report-local association key is not
  semantic association by key equality and not a branch-local association key.
- Updated `plan/00`, `plan/90`, `plan/109`, `plan/114`, README,
  Documentation, progress, tasks, and samples progress.

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/109-g1-obl024-lean-statement-draft.md`
- `plan/114-g1-obl024-lean-association-vocabulary-refinement.md`
- `plan/115-g1-obl024-association-guard-hardening.md`
- `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `docs/reports/2163-g1-obl024-association-guard-hardening.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl024_draft_names_association_vocabulary_boundary
lean samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
python3 scripts/current_l2_lean_sample_sync.py
git status --short -- samples/lean/manifest.json samples/lean/clean-near-end
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
git diff --check
bash -lc 'ENDPOINT_SEGMENT=webhooks; ENDPOINT_PATTERN="discord[.]com/api|api/${ENDPOINT_SEGMENT}"; mapfile -t files < <(git ls-files --modified --others --exclude-standard); if ((${#files[@]} == 0)); then printf "no changed files\n"; exit 0; fi; if rg -n --pcre2 "$ENDPOINT_PATTERN" "${files[@]}"; then exit 2; else rc=$?; if [ "$rc" -eq 1 ]; then printf "no endpoint matches in changed files\n"; exit 0; fi; exit "$rc"; fi'
```

Endpoint scan and post-report validation were run again before commit.

## Evidence / outputs / test results

- RED guard was confirmed before explanation update:
  `test_obl024_draft_names_association_vocabulary_boundary` failed because
  `not semantic association by key equality` was absent.
- After adding the guard prose, the same targeted test passed.
- `lean samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
  passed.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed:
  12 tests OK.
- `python3 scripts/current_l2_lean_sample_sync.py` printed
  `/home/codex/dev/mir_poc_01/samples/lean/manifest.json`; no generated Lean
  manifest / clean-near-end diffs remained.
- `python3 scripts/validate_docs.py` passed before this report was added:
  `Documentation scaffold looks complete. Found 1314 numbered report(s).`
- `python3 scripts/validate_docs.py` passed after this report was added:
  `Documentation scaffold looks complete. Found 1315 numbered report(s).`
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 20 tests OK.
- `python3 scripts/check_source_hierarchy.py` passed: 602 required, 602
  present, 0 missing.
- `git diff --check` passed.
- Leak scan over changed files found no Discord endpoint matches.

## What changed in understanding

The previous split was correct but still needed a regression guard for how
future readers could reintroduce key equality or branch-local association
through names rather than Lean logic. The new guard makes those regressions
visible before they become vocabulary defaults.

## Open questions

- What final proof-level association witness or relation should replace the
  LAB-only vocabulary.
- Whether branch classification will ever participate in proof-level
  association. Current LAB says it is not a report-local association key.
- How final diagnostic identity / ordering interacts with OBL-021 determinism.

## Suggested next prompt

Continue autonomously with the next G1 proof-boundary package, likely either an
OBL-024 theorem-shape boundary inventory or an OBL-025 completeness guard,
while keeping canon proof status and runtime output unchanged unless a package
explicitly calls for it.

## Plan update status

Updated. Added `plan/115`, updated `plan/00-index.md`,
`plan/90-source-traceability.md`, `plan/109`, and `plan/114`.

## Documentation.md update status

Updated with the OBL-024 association guard hardening.

## progress.md update status

Updated with package status and recent log entry for OBL-024 association guard
hardening.

## tasks.md update status

Updated so the G1 task map names `plan/115` and keeps OBL-024 as
compile-check-only.

## samples_progress.md update status

Updated the Lean LAB current focus and recent validation log.

## Reviewer findings and follow-up

- Package 2162 reviewer finding drove this package: stale key-sharing wording
  could reintroduce the key-equality reading.
- Cicero final read-only reviewer:
  - Finding: the first guard hardening only rejected a few exact substrings and
    did not catch Lean-valid forms such as parenthesized `DecidableEq`, `Ord`,
    `LT`, `LE`, or wrapped branch-carrier arrows.
  - Follow-up: replaced the exact substring checks with targeted regex guards
    for parenthesized typeclass spellings, `Ord` / `LT` / `LE`, wrapped
    `DiagnosticBranch` / `ReportLocalAssociationKey` arrows, and common
    uniqueness / collision / stability predicate-name drift.

## Skipped validations and reasons

- Rust tests skipped because no Rust production code or Rust fixtures changed.
- Surface sample JSON regeneration skipped because no expected JSON fixtures,
  runtime output path, or repair output changed.
- Canon proof validation skipped because this package does not edit canon or
  provide an OBL-024 proof.
- Oracle consult skipped because package 2162 already obtained advisory review
  for the association split and this package only hardens static guards around
  that accepted direction.

## Commit / push status

Pending.

## Sub-agent session close status

- Cicero reviewer sub-agent completed and was closed.
