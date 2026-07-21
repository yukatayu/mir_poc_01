# tasks

最終更新: 2026-07-21 10:22 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project direction, theory, ADRs, conformance, and process. Everything outside `mirrorea_canon/` is LAB: evidence, history, implementation, and operational notes. If LAB text conflicts with canon, canon wins.

## document role

This is the current LAB task map, not an append-only history and not a canon
decision record. `plan/` holds detailed memory; `docs/reports/` holds completed
evidence; `docs/project-status.md` is the concise reader view.

## current promoted package

The governance adoption package is integrated:
`mirrorea_canon/adr/ADR-0014.md` and
`plan/157-delegated-theory-research-governance.md` now govern L2/L3 working
theory research. It does not promote a Gate, Phase, implementation, or proof
package. T-RESEARCH-001..033 remain historical pre-delegation evidence in
`plan/156-t0-t2-research-autonomy-envelope.md`.

The current promoted research line is to select and investigate one
non-reserved LAB candidate through authority-cut, comparison, negative evidence,
and rollback planning. Canon integration is not currently available because the
ADR-0014 editable-target table has no active row. `PROPOSAL-003` and
`PROPOSAL-004` are excluded because they are owner-reserved L1 questions.

## ordered self-driven packages

| Order | Work unit | Aim and completion signal | Macro / rough estimate |
| --- | --- | --- | --- |
| 1 | Candidate triage | Map one target ID to existing canon text, LAB lane, dependent IDs, alternative/falsifier, and reserved-boundary check. | Macro 1/5 early; 1-2 research sessions |
| 2 | Candidate experiment | Reproduce anchors and run the smallest countermodel, literal transcription, conditional lemma, or existing-lane experiment that can falsify the candidate. | Macro 1/5 middle; 1-4 research sessions |
| 3 | Owner target row, review, and integration | Owner adds an exact row; steward freezes final diff; reviewer approves it; then integrate or record falsification/escalation. | Macro 0/1 closeout; owner decision plus 1 session |
| 4 | Cross-cut recut | Re-read affected theory, SCN links, proof ledger, and dashboards; remove duplicated or stale candidate wording. | Macro 0/1 reserve; periodic |

## self-driven macro phase reading

- **Closeable by research:** scoped candidate comparison, countermodel, literal
  transcription, conditional Lean lemma, existing-lane experiment, and
  falsifier-driven LAB reliance stop.
- **Research discovers:** the smallest canonical premise, carrier, relation, or
  proof organization that needs escalation, rather than assuming it from LAB.
- **Owner decides:** L0/L1, primitives, external contracts, SCN/Gate/Phase,
  final proof / OBL discharge, and public completion.

## user decision gates

| Item | Impact | Options | Current recommendation |
| --- | --- | --- | --- |
| G0-D3 | G0 exit and official T1 entry | continue defer; owner reopens and records canonical exit | dormant; do not solicit absent owner reopen |
| OBL-001 concrete-evidence bridge | whether static elaborator output can instantiate authority/capability/witness evidence | defer; authorize artifact-free design comparison | defer until a selected proof-facing package needs it |
| PROPOSAL-003 | OBL-020 formalization organization | A shared checklist; B package-local organization; C defer | owner chooses; do not use as a candidate pilot |
| PROPOSAL-004 | Surface v0 grammar closure | A Participant-only closure; B custom keyspaces; C defer | A is LAB recommendation; owner chooses |
| ADR-0014 first editable target | whether a named bounded canon claim may receive delegated L2/L3 maintenance | add one exact row; leave LAB-only | add no broad row; choose only after the LAB candidate identifies a safely bounded claim |

## research discovery items

| Item | What research must establish | Current route |
| --- | --- | --- |
| OBL-020 / OBL-021 / OBL-001 | whether a scoped premise or relation is both canon-compatible and non-reserved | historical source-adequacy evidence in `plan/156`; investigate in LAB through `plan/157`; no `theory/11` movement |
| existence, authority, observation, cut, diagnostics | a smallest formal boundary without turning an experiment-local carrier into canon | T-RESEARCH-009..027 are historical bounds; choose an independent candidate and falsifier |
| proof ledger integrity | that no LAB evidence is presented as a ledger/status change | `mirrorea_canon/theory/11-metatheory-ledger.md`; every ledger edit is owner-reserved |
| literature / comparison | that a comparison clarifies an existing canon difference without importing a new primitive | `mirrorea_canon/theory/12-literature.md`; existing literature route |

## maintenance tasks

- Keep `docs/project-status.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md` synchronized when their owned dimensions change.
- Re-run documentation/source-hierarchy validation after plan or snapshot
  changes; rebase/freeze a proposed canon change before review, and re-review
  when its cited canon blob or diff changes.
- Keep disposable spikes out of tracked mainline sources. A tracked experiment
  needs an existing permitted lane and the ADR-0014 authority cut.
- Commit with `--no-gpg-sign` and push each completed task package.

## non-promoted references

- Canon lifecycle, Gates, and phases: `mirrorea_canon/plan/00-gates.md` and
  `mirrorea_canon/plan/01-phases.md`.
- Delegated research authority: `mirrorea_canon/adr/ADR-0014.md` and
  `mirrorea_canon/plan/02-operating-model.md`.
- Current lifecycle and candidate record: `plan/157-delegated-theory-research-governance.md`.
- Historical research evidence: `plan/156-t0-t2-research-autonomy-envelope.md`.
- Proof status: `mirrorea_canon/theory/11-metatheory-ledger.md`.
- Runnable LAB dashboard: `samples_progress.md`.
