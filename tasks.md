# tasks

最終更新: 2026-07-21 21:06 JST

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
| 3 | Pilot experiment | Closed: WRK-0001's positive/negative Lean reproduction is retained and append-only manifested as L3 evidence. It remains `not-promoted`; no theory or implementation claim moved. | Macro 1/5 middle; closed |
| 4 | Pilot checkpoint | Closed: authoritative clean-worktree validation, full local validation, independent review, dashboard synchronization, and a next candidate class were completed without L3 promotion. | Macro 0/1 closeout; closed |
| 5 | OBL-021 countermodel pre-registration | Closed: WRK-0002 pre-registers a projection-vacuity countermodel in the existing `samples/lean` lane, with a concrete alternative, falsifier, rollback, and no outcome evidence. | Macro 1/5 reserve; closed |
| 6 | OBL-021 countermodel evidence | Closed: the registered red/green Lean workflow was retained as WRK-0002 L3 evidence. It demonstrates only that the current LAB draft does not itself force result identity or projection non-vacuity; it remains `not-promoted`. | Macro 1/5 reserve; closed |
| 7 | OBL-021 premise-gap triage | Closed: the countermodel, temporary Oracle review, and independent Canon audit establish that Canon fixes the intended tuple/function contract but does not furnish the LAB draft's projection witness or extensionality bridge. | Macro 1/5 reserve; closed |
| 8 | OBL-021 projection-extensionality evidence | Closed: WRK-0003 retained a total/unique-projection plus equality countermodel. It isolates the absence of a joint extensionality/direct-Result bridge without selecting either; it remains `not-promoted`. | Macro 1/5 reserve; closed |
| 9 | OBL-021 outcome-totality pre-registration | Closed: WRK-0004 registers a no-outcome countermodel in the existing Lean lane without assigning totality to OBL-021 or Canon. | Macro 1/5 reserve; closed |
| 10 | OBL-021 outcome-totality evidence | Closed: WRK-0004 retained a well-scoped/no-outcome countermodel. It demonstrates only that the LAB draft does not itself entail outcome existence; it does not assign a Canon home for totality and remains `not-promoted`. | Macro 1/5 reserve; closed |
| 11 | OBL-021 conditional outcome relation pre-registration | Closed: WRK-0005 registers an explicit-totality conditional lemma in the existing Lean lane without selecting equality, relation laws, or Canon placement. | Macro 1/5 reserve; closed |
| 12 | OBL-021 conditional outcome relation evidence | Closed: WRK-0005 retained and corrected an L3 conditional lemma. The draft and well-scopedness give guarded all-pairs coherence on a fixed input's actual-outcome fiber; explicit totality makes that fiber nonempty. This does not establish equality, global laws, quotient semantics, diagnostics adequacy, or Canon placement. | Macro 1/5 reserve; closed |
| 13 | OBL-021 statement-shape checkpoint | Closed: WRK-0002 through WRK-0005 now distinguish projection vacuity, absent joint Result adequacy, absent outcome existence, and positive fiberwise coherence. No fifth local theorem has enough decision value before an actual candidate is selected. | Macro 1/5 reserve; closed |
| 14 | Post-checkpoint candidate selection | Current: identify only a standing-eligible L3 question whose alternative outcomes change a live decision branch. Do not reopen OBL-021 for a relation-law restatement; status/refinement requests use the updated `plan/143` decision surface. | Macro 1/5 reserve; research selection |

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
| textual runnable research core | whether the existing Full System V1 source-first lane can be used as L3 retained evidence without changing governance | current WRK validator permits only `plan`, `samples/clean-near-end`, `samples/current-l2`, and `samples/lean`; the Full System V1 lane is outside that boundary, so retain this as an escalation candidate rather than silently widening the permitted roots |
| existence, authority, observation, cut, diagnostics | a smallest formal boundary without turning an experiment-local carrier into canon | T-RESEARCH-009..027 are historical bounds; choose a new candidate with a falsifier |
| proof ledger integrity | that no LAB evidence is presented as a ledger/status change | `mirrorea_canon/theory/11-metatheory-ledger.md`; every ledger edit is owner-reserved |
| literature / comparison | that a comparison clarifies an existing canon difference without importing a new primitive | `mirrorea_canon/theory/12-literature.md`; existing literature route |

## maintenance tasks

- Keep `docs/project-status.md`, `progress.md`, `tasks.md`, and `samples_progress.md` synchronized when their owned dimensions change.
- Re-run documentation/source-hierarchy validation after plan or snapshot changes. Rebase/freeze a proposed L2 working-record update before review and re-review when its cited canon blob or diff changes.
- Keep research source in existing documented LAB lanes. It is retained as WRK evidence only through an append-only manifested full commit; it is never production implementation or a conformance surface.
- Before heavy build / generated-artifact work, recheck root capacity and prefer the configured external workdir, which remains unmounted. The approved cleanup removed local `target/` and Mirrorea `/tmp` artifacts; see `docs/reports/2295-approved-artifact-cleanup.md`.
- Commit with `--no-gpg-sign` and push each completed task package.

## non-promoted references

- Canon lifecycle, Gates, and phases: `mirrorea_canon/plan/00-gates.md` and `mirrorea_canon/plan/01-phases.md`.
- Delegated research authority: `mirrorea_canon/adr/ADR-0014.md`, `mirrorea_canon/working/README.md`, and `mirrorea_canon/plan/02-operating-model.md`.
- Current lifecycle and finite autonomous horizon: `plan/158-standing-bounded-autonomy.md`.
- WRK evidence-commit integrity and its limits: `plan/159-wrk-evidence-commit-integrity-recut.md`.
- Historical research evidence: `plan/156-t0-t2-research-autonomy-envelope.md` and `plan/157-delegated-theory-research-governance.md`.
- Proof status: `mirrorea_canon/theory/11-metatheory-ledger.md`.
- Runnable LAB dashboard: `samples_progress.md`.
