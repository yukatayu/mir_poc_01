# tasks

最終更新: 2026-07-17 19:35 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project direction, theory, ADRs, conformance, and process. Everything outside `mirrorea_canon/` is LAB: evidence, history, implementation, and operational notes. If LAB text conflicts with canon, canon wins.

## document role

This is the current LAB task map, not an append-only history and not a canon decision record. `plan/` holds detailed memory; `docs/reports/` holds completed evidence; `docs/project-status.md` is the concise reader view.

## current promoted package

No canon package is promoted by this snapshot. **T-RESEARCH-001** (statement-shape countermodels), **T-RESEARCH-002** (one OBL-020 `[E-WRITE]` store-key clause), **T-RESEARCH-003** (one OBL-020 `[E-OBS]` conditional append kernel), **T-RESEARCH-005** (one `[E-DEGRADE]/[E-REACQ]` restricted lineage kernel), **T-RESEARCH-006** (13 selected transition x five named WF-clause source-adequacy audit), **T-RESEARCH-008** (OBL-021 postcondition source audit), **T-RESEARCH-009** (OBL-005 structural-flattening kernel), **T-RESEARCH-010** (OBL-006 relation boundary), **T-RESEARCH-011** (THM-002 / OBL-007 trace-formalization boundary), **T-RESEARCH-012** (THM-004 / OBL-015 mutation-origin boundary), **T-RESEARCH-013** (THM-005 / OBL-017 observer-safe export boundary), **T-RESEARCH-014** (THM-003 / OBL-009 successful-load restoration boundary), **T-RESEARCH-015** (OBL-026 transparent-overlay composition boundary), **T-RESEARCH-016** (OBL-028 revocation-monotonicity boundary), **T-RESEARCH-017** (OBL-022 stream read-side boundary), **T-RESEARCH-018** (OBL-027 atomic-cut rollback boundary), **T-RESEARCH-019** (OBL-023 temporal-coherence boundary), **T-RESEARCH-020** (OBL-010 consistent-cut checker kernel), **T-RESEARCH-021** (OBL-004 no-undeclared-communication kernel), **T-RESEARCH-022** (OBL-003 Line-1 decidability kernel), **T-RESEARCH-023** (OBL-018 explicit-flow observer-safe kernel), and **T-RESEARCH-024** (OBL-014 Z-cycle equivalence boundary) are `research-complete`. T-RESEARCH-006 found `0 direct / 65 missing`; T-RESEARCH-008 and T-RESEARCH-010 through T-RESEARCH-019 plus T-RESEARCH-024 each found a distinct `0 direct / 0 delegated / 1 missing` boundary, while T-RESEARCH-020 through T-RESEARCH-023 prove one direct conditional kernel each and isolate their remaining checker/corollary boundaries. Their results are recorded in `plan/156-t0-t2-research-autonomy-envelope.md`; none changes `mirrorea_canon/plan/01-phases.md` or `mirrorea_canon/theory/11-metatheory-ledger.md`. The next work unit needs a fresh source cut and falsifier.

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
| 7 | T-RESEARCH-007 (decision-ready) | Prepared PROPOSAL-003 for the formalization-organization A/B/C decision. Its five headings are LAB review categories, not canon predicates or a fixed Lean interface. | Macro 1 middle; owner record pending, unrelated research may continue |
| 8 | T-RESEARCH-008 (complete) | Audited the three OBL-021 determinism conjuncts against BND-001: 0 direct / 0 delegated / 3 missing; finite models isolate the missing contracts. | Macro 1/5 middle; closed as LAB source-adequacy evidence |
| 9 | T-RESEARCH-009 (complete) | Proved only experiment-local structural-output reassociation and hole-context identity for OBL-005; reverse order and source-empty mutations delimit what this does not establish. | Macro 1/5 middle; closed as bounded LAB algebraic evidence |
| 10 | T-RESEARCH-010 (complete) | OBL-006 has 0 direct / 0 delegated / 1 missing formalization boundary; a word-preserving local fork proves that output preservation does not determine confluence. | Macro 1/5 middle; closed as LAB source-adequacy evidence |
| 11 | T-RESEARCH-011 (complete) | THM-002 / OBL-007 has 0 direct / 0 delegated / 1 missing trace-formalization boundary; local countermodels isolate missing trace and lineage-origin/reacquire bindings without weakening canon policy. | Macro 1/5 middle; closed as LAB source-adequacy evidence |
| 12 | T-RESEARCH-012 (complete) | THM-004 / OBL-015 has 0 direct / 0 delegated / 1 missing coupled mutation-origin/authorization boundary; a favorable-order delegated twin isolates the association gap without weakening grant-lineage or owner-local policy. | Macro 1/5 middle; closed as LAB source-adequacy evidence |
| 13 | T-RESEARCH-013 (complete) | THM-005 / OBL-017 has 0 direct / 0 delegated / 1 missing coupled formalization boundary; a two-configuration equality twin separates constructor identity from visible-position equality without selecting an export ABI or lattice. | Macro 1/5 middle; closed as LAB source-adequacy evidence |
| 14 | T-RESEARCH-014 (complete) | THM-003 / OBL-009 has 0 direct / 0 delegated / 1 missing coupled successful-load restoration boundary; a one-save/two-result twin preserves the listed load conditions but does not select a Load API, Config interface, or persistence semantics. | Macro 1/5 middle; closed as LAB source-adequacy evidence |
| 15 | T-RESEARCH-015 (complete) | OBL-026 has 0 direct / 0 delegated / 1 missing coupled transparent-overlay composition boundary; a ten-component preorder kernel and opaque-label model select no canonical variance, composition law, Contract ABI, or equality. | Macro 1/5 middle; closed as LAB source-adequacy evidence |
| 16 | T-RESEARCH-016 (complete) | OBL-028 has 0 direct / 0 delegated / 1 missing coupled revocation-monotonicity boundary; an action kernel and unstructured-label model select no revocation, reissue, identity, trace, or authority ABI. | Macro 1/5 middle; closed as LAB source-adequacy evidence |
| 17 | T-RESEARCH-017 (complete) | OBL-022 has 0 direct / 0 delegated / 1 missing coupled stream read-side boundary; an action kernel and unstructured-label model select no stream carrier, adapter declaration, transition, or frame interface. | Macro 1/5 middle; closed as LAB source-adequacy evidence |
| 18 | T-RESEARCH-018 (complete) | OBL-027 has 0 direct / 0 delegated / 1 missing coupled atomic-cut rollback boundary; a frontier kernel and unstructured-label model select no occurrence, causality, locus, cut-projection, or rollback interface. | Macro 1/5 middle; closed as LAB source-adequacy evidence |
| 19 | T-RESEARCH-019 (complete) | OBL-023 has 0 direct / 0 delegated / 1 missing coupled temporal-coherence boundary; a shared-frontier kernel and unstructured-label model select no consumer, atomic-group, frontier, interpretation, coherence, or clock/latency interface. | Macro 1/5 middle; closed as LAB source-adequacy evidence |
| 20 | T-RESEARCH-020 (complete) | OBL-010 has one direct conditional predecessor-closure kernel; a two-edge partial checker countermodel selects no event/cut carrier, enumeration, decider, result, or diagnostic interface. | Macro 1/5 middle; closed as bounded LAB mathematical evidence |
| 21 | T-RESEARCH-021 (complete) | OBL-004 has one direct conditional generated-edge containment kernel; a two-edge partial checker countermodel selects no program/elaboration, `G_e`, declaration, runtime, or transport interface. | Macro 1/5 middle; closed as bounded LAB mathematical evidence |
| 22 | T-RESEARCH-022 (complete) | OBL-003 has one direct conditional finite failure-row checker kernel; an effect-omission countermodel selects no complete rule set, AST/parser, declaration, carrier, residual, result, or diagnostic interface. | Macro 1/5 middle; closed as bounded LAB mathematical evidence |
| 23 | T-RESEARCH-023 (complete) | OBL-018 has one direct conditional explicit-flow kernel: low-position-only redaction is invariant under modeled high state and raw witness/auth variation; a high projection delimits the required redaction law without choosing a configuration/export ABI or lattice. | Macro 1/5 middle; closed as bounded LAB mathematical evidence |
| 24 | T-RESEARCH-024 (complete) | OBL-014 has `0 direct / 0 delegated / 1 missing` checkpoint-graph / Netzer-Xu equivalence boundary; an experiment-local Boolean twin preserves Z-cycle inadmissibility while breaking a stipulated checker/useless-checkpoint equivalence. | Macro 1/5 middle; closed as LAB source-adequacy evidence |
| 25 | next independent theory source cut (unselected) | Select an existing-lane question without choosing an OBL-021 contract, OBL-006 theorem interface, THM-002 trace interface, THM-004 mutation-origin interface, THM-005/OBL-017 configuration/export interface, OBL-018 full proof/lattice/declassification interface, THM-003 load-restoration interface, OBL-014 checkpoint-graph/Netzer-Xu interface, OBL-026 contract-order/composition interface, OBL-028 revocation/reissue interface, OBL-022 stream/adapter interface, OBL-027 rollback interface, OBL-023 consumer/frontier/coherence interface, OBL-010 finite-checker interface, OBL-004 program/elaboration interface, or OBL-003 complete-checker interface. | Macro 1/5 reserve; research-selected |

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
| OBL-020 formalization organization | common review organization is not derivable from current canon prose | owner chooses PROPOSAL-003 A/B/C; its LAB headings are non-exhaustive and the proposal does not block unrelated theory research |
| OBL-021 | deterministic elaboration relation and equality/equivalence premises | OPEN-014 materialization boundary; stop if a canon choice is needed |
| OBL-005 / OBL-006 | a source-level unit interpretation and any rewrite/equivalence relation for uniqueness or confluence | T-RESEARCH-009 is output algebra only; do not turn its hole context or list carrier into source syntax |
| OBL-007 / OBL-008 | a proof-facing THM-002 trace, selection, lineage-origin/reacquire, freshness, and transition/frame boundary | canon policy is direct, but T-RESEARCH-011 found no complete derivable Lean statement; stop for a formalization act before selecting that interface |
| OBL-015 / OBL-016 | a proof-facing THM-004 mutation-origin/authorization boundary | delegated grant-lineage policy and owner-local alternative are direct, but T-RESEARCH-012 found no complete derivable Lean statement; stop before selecting the trace, owner-local, or mutation-association interface |
| OBL-017 / OBL-018 | a proof-facing THM-005 configuration low-equivalence and observer-safe export/equality boundary | observer-safe policy and selected SCN-07 constraints are direct. T-RESEARCH-023 proves only the finite low-position projection kernel; T-RESEARCH-013 still leaves the full statement boundary open. Stop before selecting a configuration relation, label/declassification treatment, export ABI, occurrence provenance, or equality/renaming/order/multiplicity semantics. |
| OBL-009 / THM-003 | a proof-facing successful-load restoration boundary | SaveObject schema and eight necessary load conditions are direct, while Config/WellFormed vocabulary is chapter-local; T-RESEARCH-014 found no complete successful-load-to-restored-Config/prefix interpretation, so stop before selecting load, liveness, prefix, or persistence semantics. OBL-010 through OBL-013 and OBL-027 remain separately open and unproved. |
| OBL-014 | a proof-facing Z-cycle reject / Netzer-Xu useless-checkpoint equivalence | theory/04 fixes only Z-cycle inadmissibility. T-RESEARCH-024 found no checkpoint graph, zigzag, recoverability, structural recognizer, or cited-characterization definition that derives the equivalence; CUT-11 is reason-code evidence only. Stop before selecting any of those interfaces. |
| OBL-026 | a proof-facing transparent-overlay composition boundary | theory/02 fixes ten transparent-policy directions but not their formal orders, a layer-stack composition operator, or equality/extensionality; T-RESEARCH-015 found no complete derivable OBL-026 statement, so stop before selecting variance, `Contract` ABI, `all_of`/`any_of` algebra, or a cost algebra. |
| OBL-028 | a proof-facing revocation-monotonicity boundary | theory/05 fixes lifecycle and no-stale-revival policy but not a revocation predicate, new epoch/evidence occurrence, identity across state changes, or trace/transition relation; T-RESEARCH-016 found no complete derivable OBL-028 statement, so stop before selecting an authority or persistence ABI. |
| OBL-022 | a proof-facing stream read-side boundary | theory/09 fixes samples outside `H`, read-side use, and typed adapter direction but not sample/discrete-state carriers, effect declaration/application, transition, or a frame relation; T-RESEARCH-017 found no complete derivable OBL-022 statement, so stop before selecting a stream, adapter, or View/Provider ABI. |
| OBL-027 | a proof-facing atomic-cut rollback boundary | theory/04 fixes local-cut policy but not occurrence identity, causality, locus membership, cut projection, rollback operation, or result relation; T-RESEARCH-018 found no complete derivable OBL-027 statement, so stop before selecting rollback or persistence semantics. |
| OBL-023 | a proof-facing temporal-coherence boundary | theory/09 fixes consumer-frontier admissibility and no-split-frame policy but not consumer, atomic-group, frontier, interpretation, coherence, or clock/latency relations; T-RESEARCH-019 found no complete derivable OBL-023 statement, so stop before selecting provider, transport, or clock semantics. |
| OBL-010 | a consistent-cut checker boundary | theory/04 directly supplies the direct-edge/transitive-prefix kernel, but not a finite event/cut carrier, complete generator coverage, decider, result, or diagnostic relation; T-RESEARCH-020 found no complete checker statement, so stop before selecting a checker or persistence interface. |
| OBL-004 | a no-undeclared-communication corollary boundary | theory/03 directly supplies the itemwise containment kernel, but not program/elaboration composition, `G_e` carrier, declared-edge mapping, or runtime communication relation; T-RESEARCH-021 found no complete corollary statement, so stop before selecting a program, runtime, or transport interface. |
| OBL-003 | a Line-1 decidability boundary | theory/01/02 directly supply finite row-containment direction, but not a complete rule set, AST/parser, declarations, carrier/equality algorithms, residual split, or checker/diagnostic relation; T-RESEARCH-022 found no complete checker statement, so stop before selecting a language or checker interface. |
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
