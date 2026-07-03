# Report 2161 - G1 OBL-024 Lean replay vocabulary refinement

Date: 2026-07-04 08:44 JST
Author: Codex
Scope: Package 23 / OBL-024 Lean statement replay vocabulary refinement
Decision levels: L2/L3 LAB statement-shape evidence only; no canon status movement
Final validation: 2026-07-04 08:57 JST

## Objective

Mirror `plan/112` inside the OBL-024 LAB Lean statement draft by separating the
current report-local replay anchor from future proof-level replay relation
vocabulary, while keeping OBL-024 compile-check-only and non-final.

## Scope and assumptions

- Scope is the OBL-024 LAB Lean statement draft, its explanation, repository
  memory, snapshots, and a narrow sync-unit vocabulary guard.
- No canon file, runtime JSON, expected sample JSON, production Rust emission,
  repair output, or OBL proof ledger status is changed.
- The current executable `trace_local_replay` object remains report-local LAB
  evidence, not proof-level replay semantics.
- `ProofLevelReplayWitness` / `ProofLevelReplayRelation` are abstract future
  proof vocabulary, not final names or proof discharge.

## Start state / dirty state

- Started from pushed clean `main` at `4e3d104b`.
- `origin/main` also pointed to `4e3d104b`.
- Discord task baseline was recorded with
  `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`.
- Worktree became dirty through Package 23 Lean / docs / status updates only.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/00-index.md`
- `plan/81-g1-obl024-statement-shape-inventory.md`
- `plan/90-source-traceability.md`
- `plan/109-g1-obl024-lean-statement-draft.md`
- `plan/112-g1-obl024-replay-vocabulary-preflight.md`
- `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
- `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md`
- `samples/lean/manifest.json`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- Bacon read-only sub-agent mapping for OBL-024 Lean replay vocabulary.

## Actions taken

1. Used TDD for the vocabulary guard:
   - added a failing sync-unit assertion for `ReportLocalReplayAnchor`,
     `ProofLevelReplayWitness`, `DiagnosticReportsReplayAnchor`,
     `ProofLevelReplayRelation`, `ReportLocalReplayAnchorCompatible`, and the
     matching explanation prose;
   - confirmed the test failed against the pre-refinement Lean draft;
   - refined the Lean draft and explanation until the test passed.
2. Refined `DiagnosticSoundnessStatementDraft.lean`:
   - replaced the single `ReplayWitness` role with `ReportLocalReplayAnchor`
     and `ProofLevelReplayWitness`;
   - added `DiagnosticReportsReplayAnchor`;
   - added `ReportLocalReplayAnchorFor` and
     `ReportLocalReplayAnchorNonFinal`;
   - added `ProofLevelReplayWitnessFor` and `ProofLevelReplayRelation`;
   - added `ReportLocalReplayAnchorCompatible`;
   - threaded the report-local anchor through `ReportedDiagnosticShape`,
     `ReplaySoundAtReportedPremise`, and `DiagnosticSoundForRejection`.
3. Updated `DiagnosticSoundnessStatementDraft.md` to explain the same split.
4. Added `plan/113-g1-obl024-lean-replay-vocabulary-refinement.md`.
5. Updated `plan/00-index.md`, `plan/90-source-traceability.md`, `plan/109`,
   `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, and
   `samples_progress.md`.
6. Ran Lean compile and sync validations. `current_l2_lean_sample_sync.py`
   completed and produced no manifest / generated-stub diff.

## Files changed

- `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
- `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `plan/113-g1-obl024-lean-replay-vocabulary-refinement.md`
- `plan/109-g1-obl024-lean-statement-draft.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2161-g1-obl024-lean-replay-vocabulary-refinement.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
lean samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl024_draft_names_replay_vocabulary_boundary
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
python3 scripts/current_l2_lean_sample_sync.py
git status --short -- samples/lean/manifest.json samples/lean/clean-near-end
date '+%Y-%m-%d %H:%M %Z'
git status --short
git diff --stat
git rev-parse --short HEAD
git rev-parse --short origin/main
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py
git diff --check
bash -lc 'ENDPOINT_SEGMENT=webhooks; ENDPOINT_PATTERN="discord[.]com/api|api/${ENDPOINT_SEGMENT}"; mapfile -t files < <(git ls-files --modified --others --exclude-standard); if ((${#files[@]} == 0)); then printf "no changed files\n"; exit 0; fi; if rg -n --pcre2 "$ENDPOINT_PATTERN" "${files[@]}"; then exit 2; else rc=$?; if [ "$rc" -eq 1 ]; then printf "no endpoint matches in changed files\n"; exit 0; fi; exit "$rc"; fi'
python3 -m unittest scripts.tests.test_validate_docs
```

Final Lean, sync, docs, source-hierarchy, diff, and endpoint validations passed
before commit. During final validation, the strengthened old-name absence
guard initially matched `ProofLevelReplayWitness` as a substring; the guard was
then narrowed to a standalone field-name regex and rerun successfully.

## Evidence / outputs / test results

- RED: the new OBL-024 vocabulary guard failed because the draft did not yet
  contain `ReportLocalReplayAnchor : Type u`.
- GREEN:
  `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl024_draft_names_replay_vocabulary_boundary`
  passed.
- `lean samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
  passed after fixing the `ReportLocalReplayAnchorFor` signature call.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed:
  11 tests.
- `python3 scripts/current_l2_lean_sample_sync.py` completed and printed
  `samples/lean/manifest.json`; no manifest / generated-stub diff remained.
- `python3 scripts/validate_docs.py` passed:
  `Documentation scaffold looks complete. Found 1313 numbered report(s).`
- `python3 scripts/check_source_hierarchy.py` passed: 602 required paths, 602
  present, 0 missing.
- `git diff --check` passed.
- Endpoint scan over changed files passed: no endpoint matches in changed files.
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 20 tests.
- `git status --short -- samples/lean/manifest.json samples/lean/clean-near-end`
  produced no output after rerunning `current_l2_lean_sample_sync.py`.

## What changed in understanding

The previous Lean draft was already compile-checkable, but its single
`ReplayWitness` role allowed the current helper-local JSON anchor and future
proof relation to be read too closely together. The safer LAB statement shape
keeps three roles distinct: diagnostic reports a non-final report-local anchor,
that anchor is compatible with the reported premise, and a separate future
proof-level witness / relation carries trace-local replay exactness.

## Open questions

- Whether final OBL-024 replay is whole-judgment, rule-local, or both.
- Whether `NoEarlierTraceLocalFailure` belongs in OBL-024 or a later ordering
  theorem.
- Whether the current LAB names should survive into final canon / Lean theorem
  vocabulary.
- How final Diagnostic equality / ordering should interact with OBL-021.

## Suggested next prompt

Continue the G1 OBL-024 ratchet by refining diagnostic-to-rejection association
or proof-level replay relation vocabulary, while keeping OBL-024
compile-check-only until the theorem boundary is ready.

## Plan update status

Updated. Added `plan/113-g1-obl024-lean-replay-vocabulary-refinement.md` and
linked it from `plan/00-index.md`, `plan/90-source-traceability.md`, and
`plan/109`.

## Documentation.md update status

Updated. `Documentation.md` now records the Lean replay vocabulary refinement
and keeps OBL-024 proof / final ABI claims out of scope.

## progress.md update status

Updated. `progress.md` now records the Package 23 Lean refinement in the
OBL-024 statement note, LAB Lean feature row, and recent log.

## tasks.md update status

Updated. `tasks.md` now records `plan/113` as the current LAB-only Lean
replay vocabulary refinement.

## samples_progress.md update status

Updated. `samples_progress.md` now records the Lean mechanization evidence row
and recent validation log for this package.

## Reviewer findings and follow-up

Completed. Bacon completed read-only mapping before the edit and its findings
were used to bound the package. Linnaeus final read-only reviewer found:

- Medium: the vocabulary guard only checked new strings and would not catch old
  standalone `ReplayWitness` / `TraceLocalReplayFailsExactlyAt` being
  reintroduced or `ReportLocalReplayAnchor` becoming decorative.
- Low: `tasks.md` still summarized `plan/109` as a "trace-local replay `Prop`"
  shape.

Follow-up: strengthened the test with absence checks for the old standalone
names and anchor-threading checks around `ProofLevelReplayWitnessFor`; updated
`tasks.md` to summarize `plan/109` as report-local replay anchor / future
proof-level replay relation vocabulary.

## Skipped validations and reasons

- Rust Surface elaboration tests and Surface sample helper were not rerun in
  this draft stage because no runtime JSON, expected JSON, production Rust
  emission, repair output, or sample row count changed.
- No canon proof validation was run because no canon file or proof-status ledger
  entry changed.

## Commit / push status

Committed and pushed:

- `e14bc87d` - `Refine OBL-024 Lean replay vocabulary`

Post-push check confirmed `HEAD` and `origin/main` both pointed to `e14bc87d`
before this status-only report update.

## Sub-agent session close status

Bacon read-only mapping sub-agent `019f2a5b-0b8a-7092-9fd6-87800fd1f64a`
completed and was closed. Linnaeus final read-only reviewer
`019f2a64-e0e7-70c2-acbc-01505ef54ae8` completed and was closed.
