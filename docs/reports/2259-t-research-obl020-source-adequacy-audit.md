# Report 2259 - T research OBL-020 source-adequacy audit

- Date: 2026-07-17
- Author / agent: Codex
- Scope: bounded LAB theory research under `plan/156`
- Decision levels touched: no canon decision level changed

## Objective

Test whether the existing canon source cut supplies derivation-complete premises
for any selected OBL-020 transition x named well-formedness-clause preservation
case, without silently introducing transition, frame, graph, freshness, or
record semantics.

## Scope and assumptions

The canon remains `T0/G0 rebaseline`, and
`mirrorea_canon/theory/11-metatheory-ledger.md` remains the only proof-status
source. The audit covers exactly 13 selected transition cases and five named
well-formedness clauses, for 65 cells. It is a frozen LAB source-adequacy result
only. Scratch evidence remains untracked under `/tmp/mirrorea-t-research-006/`.

A `direct` cell would require a derivation from the attached canon prose alone;
an unproved theorem statement, including THM-006 / OBL-019, cannot be used as a
premise. The audit does not define a canonical `Config`, history operation,
frame convention, record representation, or transition relation.

## Start state / dirty state

The worktree was clean at `4bda1288 Record T-RESEARCH-005 lineage kernel`.
T-RESEARCH-001 through T-RESEARCH-003 and T-RESEARCH-005 were LAB
`research-complete`; T-RESEARCH-004 was not selected; the OBL-001 bridge was
owner-record pending. The source-adequacy audit began in disposable `/tmp`
files only.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, `CANON.md`, and the
  applicable canon operating / phase / gate documents
- `mirrorea_canon/theory/01-mircore-v0.md` through
  `08-patch-hotplug.md`, plus `11-metatheory-ledger.md`
- `mirrorea_canon/adr/ADR-0003.md`, `ADR-0005.md`, and `ADR-0007.md`
- `plan/76-g1-obl020-021-dependency-inventory.md` and `plan/156`
- Reports 2253, 2254, and 2258; `Documentation.md`,
  `docs/project-status.md`, `progress.md`, `tasks.md`, `samples_progress.md`,
  and `.docs/progress-task-axes.md`

## Actions taken

- Fixed the audit universe to the 13 selected operational cases in `theory/01`
  and the five exact well-formedness clauses in `theory/01:69-73`.
- Built a disposable 65-cell CSV and validator. It rejects missing cases/cells,
  duplicate cells, and collapsing the `[E-SERVE]` failure branch into a pass
  branch.
- Read one-hop normative material for ordering, authority, fallback,
  observation, and patch behavior rather than treating old LAB kernels as
  normative premises.
- Constructed a disposable two-state Lean challenge showing that a store frame
  alone cannot preserve an Active-key predicate whose live epoch comes from
  membership.
- Corrected the initial mistaken direct classification of
  `[E-SERVE]/fail x WF-ACTIVE-KEY` and normalized the gap taxonomy after
  independent review recovered from its saved transcript.
- Synced the bounded result into `plan/156`, `docs/project-status.md`,
  `progress.md`, and `tasks.md` without touching canon.

## Files changed

- `docs/project-status.md`
- `docs/reports/2259-t-research-obl020-source-adequacy-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `progress.md`
- `tasks.md`

## Commands run

- `git status --short`, `git log -1 --oneline`, and canon / LAB source searches
- `df -h .`, `free -h`, `du -sh .`, and `du -sh target .git .cargo .lake`
- `python3 validate_matrix.py obl020_source_adequacy.csv`
- `python3 mutation_validation.py`
- `lean --version` and `lean --trust=0 ServeFailureFrame.lean`
- placeholder / unsafe scans and `sha256sum` for all scratch evidence
- two exact-file `ask-chatgpt-pro` review attempts, followed by saved-transcript
  inspection after their browser sessions ended

## Evidence / outputs / test results

- The matrix contains exactly 65 cells: `0 direct`, `0 delegated`, and
  `65 missing`. The validator reported `coverage and uniqueness: OK`.
- Mutation checks rejected missing `E-REACQ`, a missing `E-OBS` cell, a duplicate
  cell, and merging `E-SERVE/fail` into `E-SERVE/pass-write`.
- The normalized gap groups are `H_EXTENSION` (33 cells), `COMPONENT_FRAME`
  (18), `STATE_MEMBERSHIP_COHERENCE` (4), `AUTHORITY_RECORD_BINDING` (6), and
  `CHAIN_TRANSITION_DEFINITION` (4).
- Lean 4.29.1 compiled `ServeFailureFrame.lean` with `--trust=0`.
  `#print axioms activeKey_preserved_with_store_and_epoch_frame` reported no
  axioms. The scan found no `sorry`, `admit`, declared `axiom`, `opaque`,
  `unsafe`, `partial`, or `implemented_by`.
- SHA-256: `10798a70359c8b15b581a8094569413430b7d8e274715136d1e5819a8eb65866`
  (matrix), `dbd2849f086a0c98524c3a20097ba92c0a541c2c2a380c60a60d428bcbe22b56`
  (validator), `920804586970f47baad9a812621bfce8d6566e08173beb0c4fdd2f0704c5f339`
  (mutation checker), and
  `d3d05dc444f53582ed14ab1f4419e34347b67ff34572475f8f49787b74af7050`
  (Lean challenge).
- Before subsequent repository validation, storage was 188G total / 166G used /
  13G available (93% used); memory had about 10G available; the repository was
  7.1G, including a 7.0G `target/`. Scratch evidence is small and remains in
  `/tmp`; no new heavy artifact was created.
- After synchronizing the progress timestamp and the report's required
  project-status declaration, `make check` passed: source hierarchy 704/704,
  documentation validation with 1413 numbered reports, and `cargo check`.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed
  all 21 tests. `python3 scripts/surface_mir_samples.py check-all --format json`
  accepted all 53 samples with no failures and `workflow_ready: false`.

## What changed in understanding

The current canon has a coherent high-level operational direction but is not a
derivation-ready OBL-020 calculus. The audit makes that boundary explicit:
rule sketches and theorem statements should not be read as hidden frame or
history definitions. In particular, `[E-PUB]` and `[E-OBS]` provide useful
local facts about new events but not an old-history preservation definition;
THM-006 is an open obligation and cannot discharge the deferred patch cases.

## Open questions

- A future canonical process must determine the formal transition / frame /
  history / record premises only when a proposal can be made without changing
  the project axis or collapsing its layers.
- The next LAB research unit may align the existing OBL-020 statement draft
  with the five gap groups, but must stop before proposing a canonical equation
  or carrier choice.
- OBL-021 and the dormant OBL-001 concrete-evidence bridge remain separate
  boundaries.

## Suggested next prompt

Select a bounded OBL-020 statement/premise source cut under `plan/156`; compare
the existing abstract statement draft with the five source-adequacy groups and
prepare a decision bundle before any canonical transition or frame definition.

## Plan update status

`plan/` updated: `plan/156` records the exact 65-cell scope, zero-direct result,
normalized gap groups, adversarial evidence, review recovery, and next selection
guard.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points and source hierarchy
did not change.

## docs/project-status.md update status

更新済み: the concise control view now states that the whole selected OBL-020
matrix lacks derivation-complete canon premises, without presenting that result
as a proof or status movement.

## progress.md update status

`progress.md` updated: the logical-specification snapshot and dated recent log
now distinguish the source-adequacy audit from the earlier conditional kernels.

## tasks.md update status

`tasks.md` updated: T-RESEARCH-006 replaces the stale unselected-next-unit
description and the next research target is limited to statement/premise
alignment, not canonical semantic invention.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no active sample, validation command,
debug surface, or runnable workflow classification changed.

## Reviewer findings and follow-up

The first Oracle exact-file review waited for a response and then lost its
browser connection. The one permitted retry also ended with a browser failure,
but its saved transcript contained the review answer. It confirmed the
zero-direct result, required excluding unproved THM-006 / OBL-019 as a premise,
and replaced `GLOBAL_FRAME` with five root gap groups. The advice was checked
against the local canon before use. The wrapper did not verify model selection;
no model-selection claim is made. No local sub-agent capability was available.

## Skipped validations and reasons

No canon theorem, runtime, conformance profile, transport, product, or public
API changed. A runtime or distributed validation would not validate this frozen
source reading. Existing repository documentation and front-door checks were
reproduced, but they cannot prove OBL-020 either.

## Commit / push status

Pending after focused diff review and repository validation. The completed LAB
record will be committed with `--no-gpg-sign` and pushed before the next package.

## Sub-agent session close status

Both Oracle browser sessions are closed after connection failures; the retry's
saved transcript was inspected and its advisory findings were distilled. No
local sub-agent session was available or remains active.
