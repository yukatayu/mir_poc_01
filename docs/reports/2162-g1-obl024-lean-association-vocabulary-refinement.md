# 2162 - G1 OBL-024 Lean association vocabulary refinement

## Title and identifier

- Identifier: `2162-g1-obl024-lean-association-vocabulary-refinement`
- Package: G1 OBL-024 Lean association vocabulary refinement
- Timestamp: 2026-07-04 09:17 JST

## Objective

Refine the LAB-only OBL-024 Lean statement draft so diagnostic association
vocabulary no longer suggests that the current serialized
`lab_association_key` is final semantic association, final Diagnostic ABI, or
final proof-level evidence.

## Scope and assumptions

- Scope is LAB-only Lean statement shape, explanation, repository memory, and
  static vocabulary guards.
- `mirrorea_canon/` remains normative and was not edited.
- Runtime code, expected JSON fixtures, repair payloads, and executable
  diagnostic behavior are out of scope for this package.
- OBL-024 remains compile-check-only. This package does not prove or complete
  OBL-024.
- User requested autonomous continuation; the package records design judgment
  in repository memory instead of pausing for approval.

## Start state / dirty state

- Started from clean pushed `main` at `13a6967e`.
- Discord task baseline was recorded with
  `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`.
- Initial package direction came from the prior OBL-024 replay vocabulary
  refinement and the follow-up association vocabulary review.

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
- `plan/110-g1-obl024-executable-projection-carrier.md`
- `plan/112-g1-obl024-replay-vocabulary-preflight.md`
- `plan/113-g1-obl024-lean-replay-vocabulary-refinement.md`
- `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
- `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md`
- `samples/lean/lab-statements/obl024/README.md`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- Sub-agent mapping findings from Pauli
- Oracle consult `we-are-working-in-a-2` completed / advisory

## Actions taken

- Added `plan/114-g1-obl024-lean-association-vocabulary-refinement.md`.
- Split OBL-024 association vocabulary into three roles:
  - scoped semantic `DiagnosticAssociatedToRejection`;
  - report-local `ReportLocalAssociationKey` compatibility;
  - future proof-level `ProofLevelAssociationWitness` /
    `ProofLevelAssociationRelation`.
- Replaced the lower-scope `AssociatedEmittedDiagnostic` predicate with scoped
  `DiagnosticAssociatedToRejection`.
- Replaced final-looking helper semantics with
  `DiagnosticAssociationCompatible`.
- Kept the report-local key out of replay semantics and out of branch-local
  association vocabulary.
- Added static unit guards for positive vocabulary, stale-name rejection, and
  no obvious final ABI names such as `RequestId`, `BranchAssociationKey`, and
  `FinalAssociationKey`.
- Updated README, Documentation, progress, tasks, samples progress, plan index,
  source traceability, OBL-024 Lean explanation, and OBL-024 directory note.
- Updated `plan/90-source-traceability.md` to mark Oracle consult
  `we-are-working-in-a-2` completed / advisory.

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
- `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
- `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md`
- `samples/lean/lab-statements/obl024/README.md`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `docs/reports/2162-g1-obl024-lean-association-vocabulary-refinement.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
ask-chatgpt-pro -p "<OBL-024 association vocabulary consult>" --file plan/81-g1-obl024-statement-shape-inventory.md --file plan/109-g1-obl024-lean-statement-draft.md --file plan/112-g1-obl024-replay-vocabulary-preflight.md --file plan/113-g1-obl024-lean-replay-vocabulary-refinement.md --file samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean --file samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md
oracle status --hours 2 --limit 10
oracle session --hide-prompt we-are-working-in-a-2
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

These commands were re-run after reviewer follow-up edits before commit.

## Evidence / outputs / test results

- RED guard was confirmed before implementation:
  `test_obl024_draft_names_association_vocabulary_boundary` failed because
  `ReportLocalAssociationKey : Type u` was absent.
- `oracle status --hours 2 --limit 10` reported
  `we-are-working-in-a-2` as completed.
- Oracle recommendation: keep the package LAB-only, split report-local key,
  semantic diagnostic-to-rejection association, and future proof-level
  association evidence; avoid final key ABI, uniqueness, branch-local
  association, replay conflation, runtime changes, and OBL-024 completion
  claims.
- `lean samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
  passed.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed:
  12 tests OK.
- `python3 scripts/current_l2_lean_sample_sync.py` printed
  `/home/codex/dev/mir_poc_01/samples/lean/manifest.json`; no generated Lean
  manifest / clean-near-end diffs remained.
- `python3 scripts/validate_docs.py` passed:
  `Documentation scaffold looks complete. Found 1314 numbered report(s).`
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 20 tests OK.
- `python3 scripts/check_source_hierarchy.py` passed: 602 required, 602
  present, 0 missing.
- `git diff --check` passed.
- Leak scan over changed files found no Discord endpoint matches.

## What changed in understanding

The unsafe reading was not just the name `AssociationKey`. The more important
risk was letting key sharing become the definition of association. The revised
shape therefore gives the semantic relation its own scoped predicate and treats
the current key only as report-local compatibility evidence.

## Open questions

- What final proof object, if any, should witness diagnostic association?
- Whether final association is tied to judgment attempt, emitted diagnostic
  event, rule instance, branch classification, or a combination.
- How final diagnostic equality / ordering should interact with OBL-021
  determinism.
- Whether the LAB names used here should survive into final proof vocabulary.

## Suggested next prompt

Continue autonomously with the next G1 OBL-024/OBL-025 package that either
hardens theorem-shape guards or inventories the next proof-level boundary,
without moving canon proof status or widening runtime output unless a package
explicitly calls for it.

## Plan update status

Updated. Added `plan/114`, updated `plan/00-index.md`,
`plan/90-source-traceability.md`, and `plan/109`.

## Documentation.md update status

Updated with the OBL-024 Lean association vocabulary refinement.

## progress.md update status

Updated with package status and recent log entry for the OBL-024 association
vocabulary refinement.

## tasks.md update status

Updated so the G1 task map names `plan/114` and keeps OBL-024 as
compile-check-only.

## samples_progress.md update status

Updated the Lean LAB statement row and recent validation log.

## Reviewer findings and follow-up

- Pauli sub-agent mapping: confirmed smallest safe scope is Lean/docs/test only
  and recommended separating report-local key vocabulary from future proof
  association vocabulary.
- Oracle `we-are-working-in-a-2`: completed / advisory. Followed by replacing
  lower-scope association with scoped `DiagnosticAssociatedToRejection` and
  adding `DiagnosticAssociationCompatible`.
- Volta final read-only reviewer:
  - Finding: stale "share a report-local association key" wording remained in
    the Lean explanation and `plan/109`. Follow-up: replaced it with
    compatibility wording that does not define semantic association as key
    equality.
  - Finding: report closeout fields were still pending during draft review.
    Follow-up: final validation and sub-agent close status were recorded here.
  - Finding: report consulted list omitted canon entry files. Follow-up:
    added `CANON.md`, `mirrorea_canon/README.md`, and `mirrorea_canon/MAP.md`.

## Skipped validations and reasons

- Rust tests skipped because no Rust production code or Rust fixtures changed.
- Surface sample JSON regeneration skipped because no expected JSON fixtures,
  runtime output path, or repair output changed.
- Canon proof validation skipped because this package does not edit canon or
  provide an OBL-024 proof.

## Commit / push status

- Body commit `d5ef5070` (`Refine OBL-024 Lean association vocabulary`) was
  pushed to `origin/main`.
- This report status update is committed separately so the pushed body commit
  can be named accurately.

## Sub-agent session close status

- Pauli mapping sub-agent completed and was closed.
- Volta reviewer sub-agent completed and was closed.
