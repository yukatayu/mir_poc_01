# tasks

最終更新: 2026-07-17 13:17 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project direction, theory, ADRs, conformance, and process. Everything outside `mirrorea_canon/` is LAB: evidence, history, implementation, and operational notes. If LAB text conflicts with canon, canon wins.

## document role

This is the current LAB task map, not an append-only history and not a canon decision record. `plan/` holds detailed memory; `docs/reports/` holds completed evidence; `docs/project-status.md` is the concise reader view.

## current promoted package

No canon package is promoted by this snapshot. **T-RESEARCH-001** (statement-shape countermodels), **T-RESEARCH-002** (one OBL-020 `[E-WRITE]` store-key clause), **T-RESEARCH-003** (one OBL-020 `[E-OBS]` conditional append kernel), **T-RESEARCH-005** (one `[E-DEGRADE]/[E-REACQ]` restricted lineage kernel), and **T-RESEARCH-006** (13 selected transition x five named WF-clause source-adequacy audit) are `research-complete`. T-RESEARCH-006 found `0 direct / 65 missing`: complete operational proof premises remain absent from the canon source cut. Their results are recorded in `plan/156-t0-t2-research-autonomy-envelope.md`; none changes `mirrorea_canon/plan/01-phases.md` or `mirrorea_canon/theory/11-metatheory-ledger.md`. The next work unit needs a fresh source cut and falsifier.

The 2026-07-17 runnable-front-door audit reproduced the existing Surface, Full
System V1, Product Alpha, installed-binary, operational, and current-L2
evidence paths without a failure. It does not promote a successor research unit
or alter any readiness classification.

The subsequent `T-RESEARCH-004` candidate preflight is **not selected**. Its
literal-RHS source pair reproduces request/failure/span evidence, but the
current elaborator does not expose the authority/capability/witness carrier
needed to evaluate the stated OBL-001 subcase. The next research unit remains
unselected; a bridge-specific owner disposition is required before any scoped
design comparison, and it does not by itself permit a committed bridge.
The owner-facing decision bundle is complete and independently re-reviewed.
The later direct theory objective permits unrelated existing-lane research
selection around the dormant bridge, but is not recorded as a bridge
disposition and does not authorize bridge design.

## ordered self-driven packages

| Order | Work unit | Aim and completion signal | Macro / rough estimate |
| --- | --- | --- | --- |
| 1 | T-RESEARCH-001 (complete) | Reproduced Surface/Lean anchors and constructed finite countermodels for OBL-020, OBL-021, and OBL-001. | Macro 5 early; closed as LAB evidence |
| 2 | T-RESEARCH-002 (complete) | Proved one `[E-WRITE]` store-key preservation case under value-only update; showed epoch-changing update breaks that clause. | Macro 1/5 middle; closed as LAB evidence |
| 3 | T-RESEARCH-003 (complete) | Proved a fresh incoming-only `[E-OBS]` graph kernel for acyclicity and kind-level publication ancestry; a weak outgoing-edge model forms a cycle. | Macro 1/5 middle; closed as conditional LAB evidence |
| 4 | T-RESEARCH-004 preflight (not selected) | Literal-RHS foreign-locus source pair passed request/failure/span checks, but the existing lane cannot evaluate the required authority carrier. | Macro 1 early; bounded falsifier recorded |
| 5 | T-RESEARCH-005 (complete) | In a two-rule experiment only, proved initial defined-entry persistence/nondecrease and separated local support bookkeeping from canon well-formedness; negative models delimit the assumptions. | Macro 1/5 middle; closed as conditional LAB evidence |
| 6 | T-RESEARCH-006 (complete) | Audited all 65 selected transition x named-WF cells. None has derivation-complete canon premises; five missing-premise groups replace a generic global-frame label. | Macro 1/5 middle; closed as LAB source-adequacy evidence |
| 7 | OBL-020 statement/premise source cut (unselected) | Compare the existing abstract statement draft with the five missing-premise groups; stop before proposing any canonical transition/frame/carrier definition. | Macro 1/5 reserve; research-selected |

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
| OBL-001 concrete-evidence bridge | whether static elaborator output can instantiate authority/capability/witness OBL-001 evidence | explicitly defer until proof-facing need; authorize an artifact-free design comparison with existing route and permitted persistence | defer is recommended; direct theory work leaves the bridge dormant; a committed bridge is a separate escalation |
| semantic premise not derivable from canon | scope and truth of a proposed theorem | owner/canon decision; explicitly defer; change research target | stop with a decision bundle, not an inferred choice |
| SCN/canon/ADR/theory ledger action | normative semantics or proof status | apply through canon process; defer | never apply through LAB task close |

G0-D3's current defer is recorded in `plan/155-t0-g0-governance-profile-proposal.md`.

## research discovery items

| Item | What research must establish | Current route |
| --- | --- | --- |
| OBL-020 | source-adequacy audit found no derivation-complete case across all 13 selected rules x five named clauses; next research must distinguish statement premises from any canonical semantic choice | compare the existing abstract statement draft with the five missing-premise groups; do not treat a LAB taxonomy or experiment-local update as canon-defined |
| OBL-021 | deterministic elaboration relation and equality/equivalence premises | OPEN-014 materialization boundary; stop if a canon choice is needed |
| OBL-001 / OBL-002 | assignment elaboration premises and later proof skeleton | source preflight reproduced a narrow pair but falsified the existing concrete authority-carrier lane; the complete owner-facing bundle awaits an explicit disposition |
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
