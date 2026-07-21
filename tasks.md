# tasks

最終更新: 2026-07-21 17:09 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project direction, theory, ADRs, conformance, and process. Everything outside `mirrorea_canon/` is LAB: evidence, history, implementation, and operational notes. If LAB text conflicts with canon, canon wins.

## document role

This is the current LAB task map, not an append-only history and not a canon decision record. `plan/` holds detailed memory; `docs/reports/` holds completed evidence; `docs/project-status.md` is the concise reader view.

## current promoted package

The standing-bounded-autonomy route is validated for committed, reversible L3
pre-registration in `mirrorea_canon/adr/ADR-0014.md`,
`mirrorea_canon/working/README.md`, and
`plan/158-standing-bounded-autonomy.md`. The validator now rejects malformed or
rewritten reachable WRK history, non-metadata registration changes, and
manifested evidence outside its declared lane; its authoritative mode rejects
dirty evidence state. L2 is
intentionally fail-closed until an owner-authenticated trust anchor is added by
a separate canon action. This does not promote a Gate, Phase, implementation,
or proof package. T-RESEARCH-001..033 remain historical pre-delegation evidence
in `plan/156-t0-t2-research-autonomy-envelope.md`.

## ordered self-driven packages

| Order | Work unit | Aim and completion signal | Macro / rough estimate |
| --- | --- | --- | --- |
| 1 | Governance closeout | Closed: standing route, WRK structure/history/evidence/index checks, canon/LAB mirrors, validation, commit/push, and clean detached-worktree authoritative pass. | Macro 0/1 early; closed |
| 2 | Pilot triage and pre-registration | Closed: WRK-0001 pre-registers a theory/02 finite-index reproduction with anchors, alternative/falsifier, non-effects, rollback, and existing Lean commands. | Macro 1/5 early; closed |
| 3 | Pilot experiment | Current package: run WRK-0001's positive/negative Lean reproduction, retain only manifested existing-lane evidence, and keep L2 fail-closed. | Macro 1/5 middle; 1 session |
| 4 | Pilot checkpoint | Independent review, cross-cut impact read, dashboard/report synchronization, and next target class or escalation bundle. This is the current run's planned stop. | Macro 0/1 closeout; 1 session |
| 5 | Subsequent ratchet | Select the next eligible candidate only after the checkpoint; do not pre-commit its detailed semantics. | Macro 1/5 reserve; recurring |

## self-driven macro phase reading

- **Closeable by research:** scoped candidate comparison, countermodel, literal transcription, conditional Lean lemma, existing-lane implementation validation, and falsifier-driven reliance stop.
- **Research discovers:** the smallest premise, carrier, relation, or proof organization that must be escalated, rather than assuming it from LAB.
- **Owner decides:** L0/L1, primitives, external contracts, SCN/Gate/Phase, final proof / OBL discharge, and public completion.

## user decision gates

| Item | Impact | Options | Current recommendation |
| --- | --- | --- | --- |
| G0-D3 | G0 exit and official T1 entry | continue defer; owner reopens and records canonical exit | dormant; do not solicit absent owner reopen |
| OBL-001 concrete-evidence bridge | whether static elaborator output can instantiate authority/capability/witness evidence | defer; authorize artifact-free design comparison | defer until a selected proof-facing package needs it |
| PROPOSAL-003 | OBL-020 formalization organization | A shared checklist; B package-local organization; C defer | owner chooses; exclude from pilot |
| PROPOSAL-004 | Surface v0 grammar closure | A Participant-only closure; B custom keyspaces; C defer | A is LAB recommendation; owner chooses |

Routine non-reserved target selection is **not** a user decision gate. A target that touches the reserved boundary is escalated with its evidence rather than placed in `working/`.

## research discovery items

| Item | What research must establish | Current route |
| --- | --- | --- |
| OBL-020 / OBL-021 / OBL-001 | whether a scoped premise or relation is canon-compatible, sufficiently explicit, and non-reserved | historical source-adequacy evidence in `plan/156`; select a bounded WRK candidate under `plan/158`; no `theory/11` movement |
| existence, authority, observation, cut, diagnostics | a smallest formal boundary without turning an experiment-local carrier into canon | T-RESEARCH-009..027 are historical bounds; choose a new candidate with a falsifier |
| proof ledger integrity | that no LAB evidence is presented as a ledger/status change | `mirrorea_canon/theory/11-metatheory-ledger.md`; every ledger edit is owner-reserved |
| literature / comparison | that a comparison clarifies an existing canon difference without importing a new primitive | `mirrorea_canon/theory/12-literature.md`; existing literature route |

## maintenance tasks

- Keep `docs/project-status.md`, `progress.md`, `tasks.md`, and `samples_progress.md` synchronized when their owned dimensions change.
- Re-run documentation/source-hierarchy validation after plan or snapshot changes. Rebase/freeze a proposed L2 working-record update before review and re-review when its cited canon blob or diff changes.
- Keep research source in existing documented LAB lanes. It is retained as WRK evidence only through an append-only manifested full commit; it is never production implementation or a conformance surface.
- Commit with `--no-gpg-sign` and push each completed task package.

## non-promoted references

- Canon lifecycle, Gates, and phases: `mirrorea_canon/plan/00-gates.md` and `mirrorea_canon/plan/01-phases.md`.
- Delegated research authority: `mirrorea_canon/adr/ADR-0014.md`, `mirrorea_canon/working/README.md`, and `mirrorea_canon/plan/02-operating-model.md`.
- Current lifecycle and finite autonomous horizon: `plan/158-standing-bounded-autonomy.md`.
- WRK evidence-commit integrity and its limits: `plan/159-wrk-evidence-commit-integrity-recut.md`.
- Historical research evidence: `plan/156-t0-t2-research-autonomy-envelope.md` and `plan/157-delegated-theory-research-governance.md`.
- Proof status: `mirrorea_canon/theory/11-metatheory-ledger.md`.
- Runnable LAB dashboard: `samples_progress.md`.
