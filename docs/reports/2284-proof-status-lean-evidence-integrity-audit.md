# Report 2284 - Proof-status and Lean-evidence integrity audit

## Objective

Verify that the current theory ledger, active Lean artifacts, and LAB status
views do not skip a proof, hide an active proof placeholder, or describe a
bounded artifact as an OBL discharge.

## Scope and assumptions

Canon remains normative. `mirrorea_canon/theory/11-metatheory-ledger.md` alone
states proof status. This audit reads existing Lean files and existing LAB
research records; it selects no semantic interface, adds no evidence lane, and
does not make a proof-status claim outside the ledger.

## Start state / dirty state

The worktree was clean and synchronized at `1bd41782` before the audit. The
task recorded its Discord baseline before the substantive inspection. No
tracked source, sample, generated artifact, or canon status had changed.

## Documents consulted

- Canon `README`, `MAP`, theory/00, theory/03, theory/04, theory/08,
  theory/11, plan/00--03, and meta/agent-instructions/style-guide
- LAB `README`, `Documentation.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, `plan/00`, `plan/156`, and Reports 2278, 2282, 2283
- `samples/lean/README.md`, its manifest, the four foundations, and the five
  active LAB statement-draft families

## Actions taken

- Partitioned all 28 OBL IDs against the dependency classes recorded by
  T-RESEARCH-025 and checked for duplicate, missing, or extra classifications.
- Inspected the active Lean manifest and all active Lean source for proof
  placeholders and for the distinction between foundations, statement drafts,
  and generated stubs.
- Compiled the five statement drafts and the 20 foundation/clean-stub files in
  non-overlapping sequential runs, then ran the canonical Lean sync command.
- Investigated a transient direct-compile error rather than treating it as a
  source defect.

## Files changed

- `docs/reports/2284-proof-status-lean-evidence-integrity-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

## Commands run

- resource audit: `df -h .`, `free -h`, and repository/build-size checks
- focused `rg`, `sed`, `find`, and read-only Python ledger/Lean inventories
- direct `lake env lean` checks for the five statement drafts and the 20
  foundation/clean files, in separate sequential runs
- `python3 scripts/current_l2_lean_sample_sync.py`
- final documentation/source-hierarchy, Surface, Lean-sync, Rust parser, and
  Git checks listed after the documentation edits

## Evidence / outputs / test results

- The 28 ledger OBLs partitioned with zero duplicates, missing IDs, or extra
  IDs: 15 parent-statement boundaries, six proof-or-lemma dependents, four
  bounded direct fragments, one explicit OBL-019 overlap, and two diagnostic
  boundaries. OBL-011--013 remain dependents of the missing OBL-009
  Load/restored-state/live-after-load relation; their lack of standalone
  research is therefore deliberate rather than an omission.
- theory/11 is the sole proof-status authority and records all entries `open`.
  No active reader/status view claimed `lean-proved`, proof discharge, or an
  OBL completion.
- The active manifest has 25 existing files: four foundations, five statement
  drafts, and 16 clean-suite stubs. Every manifest verification succeeded and
  every listed path exists. Active source has zero `sorry`, `admit`, `axiom`,
  `unsafe`, or `partial` occurrences.
- The five statement drafts define abstract proposition shapes; none declares a
  Lean theorem, axiom, or constant. The 16 generated stubs intentionally prove
  only `True` by `trivial`. The four foundations contain actual proofs only for
  their own finite user-defined label, lifetime/capture, or stub-identity
  models. None is a MirCore OBL proof.
- The historical `samples/lean/old/` archive has 15 `sorry` occurrences in 12
  files and labels itself as historical/non-discharge evidence. It is outside
  the active corpus.
- An initial parallel direct compile observed transient missing-file errors.
  Root-cause inspection showed that `current_l2_lean_sample_sync.py` removes
  and recreates `samples/lean/clean-near-end/` before writing stubs. Re-running
  the same artifact sets without that overlapping regeneration compiled all
  25 files successfully. This is an audit-command race, not invalid Lean.

## What changed in understanding

The repository distinguishes three Lean evidence levels more clearly than a
single successful compile result suggests: helper-local proofs, abstract
statement expressibility, and generated source-identity stubs. The current
theory state is honest about all three. The next formal advance still requires
an owner/canon-selected relation; this audit only verifies that missing proof
work has not been mislabelled as complete.

## Open questions

- Which owner/canon record will open the first exact proof-facing relation after
  the existing G0-D3 and PROPOSAL-003 boundaries?
- Does the owner accept PROPOSAL-004 A, B, or C for the Surface v0 grammar
  closure?
- When a parent statement is selected, which dependent proof/lemma should be
  reopened first with its exact formal carrier?

## Suggested next prompt

Record the applicable owner disposition for G0-D3 / PROPOSAL-003 and, separately,
the PROPOSAL-004 A/B/C grammar choice. Then open one scoped formalization
package that names only the relation needed by its first proof target.

## Plan update status

Updated: plan/156 records T-RESEARCH-031, the complete ledger classification,
the active Lean evidence levels, and the regeneration-race interpretation.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points and user commands
did not change.

## docs/project-status.md update status

更新済み: the human status view now separates the 28-entry open ledger from
the 25 active Lean artifacts and states why neither is an OBL discharge.

## progress.md update status

Updated: the logical-specification snapshot and dated recent log now include
the proof-status integrity audit and its non-claim.

## tasks.md update status

Updated: the current task map records T-RESEARCH-031 as completed integrity
maintenance and keeps the next semantic move owner-controlled.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no sample path, runner command, or
workflow classification changed.

## Reviewer findings and follow-up

The local audit found no hidden active proof placeholder or overclaim. A new
temporary Oracle review was not retried because the immediately preceding
temporary request failed before prompt submission due to the unchanged browser
profile authentication state. No local sub-agent service was available.

## Skipped validations and reasons

No final proof, theorem semantics, runtime, distributed, product, or
conformance validation applies to this evidence-integrity audit. Parallel
direct Lean compilation is intentionally not used as a validation mode because
the canonical sync command regenerates the same clean-stub directory.

## Commit / push status

The package was committed with `--no-gpg-sign` as `53ad3676` (`Audit theory
proof-status evidence`) and pushed to `origin/main` after the final
validation. A post-push worktree/upstream check is recorded with this status
sync.

## Sub-agent session close status

No local sub-agent service was available; no session was opened or requires
closure.
