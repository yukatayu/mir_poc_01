# tasks

最終更新: 2026-07-16 19:15 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project direction, theory, ADRs, conformance, and process. Everything outside `mirrorea_canon/` is LAB: evidence, history, implementation, and operational notes. If LAB text conflicts with canon, canon wins.

## document role

This is the current LAB task map, not an append-only history and not a canon decision record. `plan/` holds detailed memory; `docs/reports/` holds completed evidence; `docs/project-status.md` is the concise reader view.

## current promoted package

No canon package is promoted by this snapshot. The active item is the LAB research work unit **T-RESEARCH-001**, defined by `plan/156-t0-t2-research-autonomy-envelope.md`: reproduce the existing OBL-001/020/021 statement checks and test finite countermodels for their standalone proof boundary. It may end only as `research-complete` or `decision-ready`; neither result changes `mirrorea_canon/plan/01-phases.md` or `mirrorea_canon/theory/11-metatheory-ledger.md`.

## ordered self-driven packages

| Order | Work unit | Aim and completion signal | Macro / rough estimate |
| --- | --- | --- | --- |
| 1 | T-RESEARCH-001 (current) | Reproduce existing Surface/Lean anchors; construct finite countermodels for OBL-020, OBL-021, and OBL-001; identify the missing concrete premise. | Macro 5 early; one focused research session |
| 2 | next eligible statement-premise study | Select the narrowest existing OBL or SCN boundary only after item 1; do not create a new lane. | Macro 1/5 middle; one to two focused sessions |
| 3 | acceptance / proof-preflight preparation | Consolidate only reusable evidence for a later owner/canon packet or T2 proof skeleton. | Macro 5 reserve; depends on item 2 |

The sequence follows `plan/156-t0-t2-research-autonomy-envelope.md`; it is not an official Phase progression. A work unit is skipped rather than invented if the selection rule is not satisfied.

## self-driven macro phase reading

- **Can self-drive:** counterexample search, existing Lean statement/proof exploration, existing-runner reproduction, SCN-to-LAB evidence mapping, reference-scenario candidate design, and decision-bundle preparation.
- **Can close only as LAB research:** a bounded work unit with reproducible evidence and explicit non-claims.
- **Cannot self-drive:** canon package close, Gate/Phase exit, ADR effectivity, L0/L1 choice, SCN expectation change, `theory/11` proof-status update, runtime/product/conformance/final-ABI implementation.

The authority source is `mirrorea_canon/plan/02-operating-model.md`; the current operational boundary is `plan/156-t0-t2-research-autonomy-envelope.md`.

## user decision gates

| Item | Impact | Options | Current recommendation |
| --- | --- | --- | --- |
| G0-D3 | G0 exit and official T1 entry | continue defer; owner reopens and records canonical exit | dormant; do not solicit absent owner reopen |
| semantic premise not derivable from canon | scope and truth of a proposed theorem | owner/canon decision; explicitly defer; change research target | stop with a decision bundle, not an inferred choice |
| SCN/canon/ADR/theory ledger action | normative semantics or proof status | apply through canon process; defer | never apply through LAB task close |

G0-D3's current defer is recorded in `plan/155-t0-g0-governance-profile-proposal.md`.

## research discovery items

| Item | What research must establish | Current route |
| --- | --- | --- |
| OBL-020 | the rule and well-formedness premises needed for preservation | T-RESEARCH-001 then concrete calculus study |
| OBL-021 | deterministic elaboration relation and equality/equivalence premises | T-RESEARCH-001 then existing abstraction decision route |
| OBL-001 / OBL-002 | assignment elaboration premises and later proof skeleton | T-RESEARCH-001 then G1 static bridge evidence |
| G2 / G3 statements | feasibility of chain and authority statement groups | only after current higher-priority boundary is clear |

## maintenance tasks

- Keep `docs/project-status.md`, `progress.md`, `tasks.md`, and `samples_progress.md` synchronized when their stated dimensions change.
- Run documentation/source-hierarchy checks after plan or snapshot changes.
- Keep temporary Lean/spike artifacts outside tracked mainline sources; record reproducible commands and bounded conclusions in the report.
- Commit with `--no-gpg-sign` and push each completed task package.

## non-promoted references

- Canon lifecycle, Gates, and phases: `mirrorea_canon/plan/00-gates.md` and `mirrorea_canon/plan/01-phases.md`.
- Proof status: `mirrorea_canon/theory/11-metatheory-ledger.md`.
- G1 static bridge evidence: `plan/121-g1-minimal-vertical-slice-candidate-map.md` through `plan/128-g1-bridge-handoff-blocker-ledger.md`.
- Runnable LAB dashboard: `samples_progress.md`.
- Detailed autonomy/stop protocol: `plan/156-t0-t2-research-autonomy-envelope.md`.
