# tasks

最終更新: 2026-07-22 22:54 JST

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

The bounded Full System V1 semantic invariant repair is closed. The source-first
checker now shares its exact host-adapter policy with the bounded runtime,
rejects adapter signature/capability/context mismatches before evaluation,
rejects duplicate record construction fields, and admits only the scalar
equality floor. The 21-row checker corpus, direct static-rejection coverage,
and accepted 29-command final release check at clean `4a52dd3e` matching the
upstream tracking ref make these guards runnable LAB evidence only; they do not promote a theory
candidate, trusted runtime authorization, or a public effect ABI. No further
Full System V1 maintenance item is currently selected, so the current
no-candidate research selection remains unchanged.

The foundation integrity audit is also closed as LAB evidence. It found no need
for an additional Core primitive and isolated the owner-reserved BND-001
outcome-totality source-to-ledger question in PROPOSAL-008. A subsequent
review found the distinct OBL-001 Result/write enumeration gap, so WRK-0007 is
now manifested L3 evidence. The prior no-candidate result remains a priority
disposition, not an independent restriction on ADR-0014 eligibility.

The post-WRK-0007 selection is closed with no new WRK. OBL-001 predicate
disconnection repeats existing evidence; OBL-025 scope/metadata variants are
explicit LAB scope or the recorded repair-realization boundary; and OBL-024
projection variants require an unselected field-functionality reading. This is
a new evidence-backed priority disposition, not a Canon ban on future L3 work.

WRK-0008's scoped L3 audit is now closed in the existing current-L2 lane; its
Canon working record remains `L3-open`. Its
four discriminating cases show that the formal-hook row accepts either cut or
rollback event presence and carries only symbolic identity references, not a
same-Place frontier relation. The current interpreter's separate locality
handling remains outside that row. This closes the attribution question only;
it does not select a carrier or change OBL-027, theory, runtime, or lifecycle.

WRK-0009 is manifested scoped L3 evidence: the e5 Lean foundation tuple does
not literally match the existing static current-L2 e5 route. Both positions
mismatch, while the registered Lean/test/regression command passed. This neither
selects a mapping nor determines a synthetic role, defect, theorem, carrier, or
Canon semantics. `plan/wrk-0009-e5-skeleton-identity.md` retains the matrix;
plan/168 records why the upstream projection audit was not selectable and why
the remaining candidates stay reserve paths.

WRK-0010 is manifested scoped L3 evidence: existing static formal hooks retain
neither the selected static decision payload nor an exact artifact reference.
WRK-0011 is separately manifested assertion-provenance evidence: its four
named e21/e22 source-route bodies do not directly compare
`RunReport.final_place_store`, while two named direct fixture/evaluator bodies
compare `evaluator.state.place_store`. Neither record judges diagnostics,
state meaning, correctness, coverage, defects, or repair.

The corrected post-WRK-0011 selection produced `WRK-0012`: one fixed accepted
and one fixed rejected P-COMP-03 direct-carrier row in their existing Product
Alpha directories. Its committed pre-registration preceded the two sidecars
and its exact command sequence. The observed checks/runs met the registered
classifications, but the required numbered result artifact needs a validator /
source-hierarchy registration change that the record explicitly excluded.
`WRK-0012` is therefore `frozen`; its only artifacts are the two committed
sidecars, and R-2347 is historical metadata. It changes no helper, schema,
runtime, CLI, public carrier, Canon state, OBL, Gate, Phase, or sample workflow.
`plan/170-post-wrk0011-candidate-selection.md` remains the pre-selection memory.

The post-WRK-0012 source screen is now closed. Existing unnumbered
`plan/wrk-...` evidence files are an indexed LAB convention, and a disposable
validator probe passed without static-list changes. The selected next question
is a fresh retained reproduction, not another direct-carrier discovery:
WRK-0013 pinned the two existing sidecars only as inputs, ran only after its
pushed registration, and retained fresh output through the declared unnumbered
memo plus its index entry. The old W12 run remains history; W13 is manifested
`not-promoted` provenance/retention evidence, not a general direct-carrier or
workflow result. Details are in `plan/wrk-0013-pcomp03-retained-reproduction.md`.

The executable computational baseline audit is closed as LAB classification
evidence. The 15-row matrix has two direct Product Alpha Rust-runtime
acceptances, ten helper-only `P-COMP-03` rows, and three direct package-check
rejections; direct textual `.mir` input is intentionally unsupported by Product
Alpha `check` / `run-local`. Separate runtime tests execute the closed
`P-COMP-03` positive modules and reject its negative modules through constructed
valid packages. Their five negative registry modules split into four static
typecheck rejections and one evaluation-time bounds rejection; helper
`runtime_rejection` and Product Alpha `MirCompute` do not represent that phase
split. This does not turn checked-in helper fixtures into package-runtime
evidence. A direct
fixture widening remains self-driven only while it stays an existing-lane,
non-production artifact without a new helper/schema/CI/Make surface or runtime
production implementation; otherwise it escalates. See
`plan/166-mir-computational-baseline-directness-audit.md` and
`plan/167-pcomp03-rejection-phase-cross-carrier-audit.md`.

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
| 14 | Post-checkpoint candidate selection | Closed: at that checkpoint, source audit, read-only candidate mapping, planner challenge, and temporary Oracle review found no standing-eligible L3 proposition with distinct live-branch outcomes. OBL-024 soundness-to-comparison would repeat the known abstract bridge gap; no WRK-0006 was opened then. | Macro 1/5 reserve; closed historical triage |
| 15 | OBL-020 familywise/global pre-registration | Closed: WRK-0006 registered an existing-LAB-vocabulary question about the global preservation draft and the familywise wrapper. It leaves coverage experiment-local and does not select a Canon step taxonomy, theorem interface, or binding. | Macro 1/5 reserve; registration closed |
| 16 | OBL-020 familywise/global evidence | Closed: WRK-0006 manifests that global preservation implies the wrapper, while only an explicit experiment-local coverage premise gives the converse; its non-vacuous finite model has an unclassified actual non-preserving step. It remains `not-promoted`, selects no Canon coverage rule or taxonomy, and changes no `theory/11` status. | Macro 1/5 reserve; closed L3 evidence |
| 17 | Post-WRK-0006 candidate selection | Closed historical triage: local source review and temporary Oracle review found no then-known non-duplicative L3 question. `plan/162-post-wrk0006-candidate-selection.md` records reopen conditions; it is not a Canon prohibition. | Macro 1/5 reserve; selection closed |
| 18 | Full System V1 semantic invariant repair | Closed and post-repair attested at clean `4a52dd3e` matching the upstream tracking ref: shared private host-adapter policy checks exact pair signature, operation-specific declared and ambient capability requirements, and rejects host adapter use without a transition context. The checker also rejects duplicate record fields and record/fixed-array equality before runtime. The active checker corpus is 3 positive / 18 negative rows. | Macro 2/3 maintenance; closed with Rust/Python/source-matrix/release evidence; no widened claim |
| 19 | Foundation integrity and outcome audit | Closed: Core direction and proof-status separation were rechecked; all five import-bearing OBL-020/021 L3 Lean sources replayed from external `.olean` inputs; BND-001 outcome-totality placement is PROPOSAL-008. | Macro 1/5 audit; owner P008 is the totality decision surface |
| 20 | OBL-001 result/write coverage pre-registration and evidence | Closed: WRK-0007 manifests an imported finite model where a successful experiment-local Result has a labeled write outside `GeneratedWrite` while the unchanged LAB draft holds. It is a statement-shape gap only, not a THM-001 counterexample or Core/result interface selection. | Macro 1/5 reserve; closed L3 evidence |
| 21 | Post-WRK-0007 candidate selection | Closed: Canon/LAB source audit, independent reviews, and temporary Oracle adjudication found no distinct next L3 record. OBL-001 request/result variants duplicate prior evidence; OBL-025 scope/metadata variants are known LAB boundaries; OBL-024 extra-projection variants need an unselected carrier law. | Macro 1/5 reserve; closed selection, reopen only on a new structural mismatch |
| 22 | Mir computational baseline directness audit | Closed: reproduced the 15-row matrix, direct CLI probes, Rust semantic/schema/runtime tests, source mapping, and Oracle boundary review. It records direct fixtures, helper-only fixtures, and separate closed-registry runtime tests without adding implementation. | Macro 2 parser-free substrate; closed audit |
| 23 | OBL-027 formal-hook attribution | Closed scoped audit: WRK-0008 uses four current-L2 runtime cases and the 23-command regression to show the formal-hook row is a coarse reachability/identity reference, not a same-Place cut-frontier witness. Separate runtime locality remains non-claimed; the Canon working record remains `L3-open`. | Macro 1/5 reserve; closed scoped audit |
| 24 | Next standing-eligible target triage | Closed selection: planner/Oracle/local source review selected WRK-0009 e5 proof-skeleton literal identity fidelity. Upstream projection loss lacked an existing discriminating record; static verdict/reason and source-route state coverage remain reserve candidates. | Macro 1/5 reserve; closed selection |
| 25 | WRK-0009 e5 proof-skeleton identity evidence | Closed scoped result: the registered command passed Lean, 4 theorem-stub support tests, and 23/23 regression commands, but the foundation and emitted tuple mismatch literally at both positions. No mapping, semantic conclusion, or repair was selected. | Macro 1/5 reserve; closed scoped evidence |
| 26 | Next standing-eligible target triage | Closed selection: planner/Oracle/local review selected WRK-0010 static decision attribution; e21/e22 final-store assertion coverage remains reserve. | Macro 1/5 reserve; closed selection |
| 27 | WRK-0010 static decision attribution evidence | Closed scoped result: 5 support tests, four static smokes, and 23/23 regression passed. Static decision payload is not literally attributed by the existing formal hook; no diagnostic/defect/schema conclusion or repair was selected. | Macro 1/5 reserve; closed scoped evidence |
| 28 | WRK-0011 final-store assertion directness evidence | Closed scoped result: in the named e21/e22 source-route bodies no exact `RunReport.final_place_store` equality occurs; two named direct fixture/evaluator bodies directly compare `evaluator.state.place_store`. Six focused tests and 23/23 regression passed in a clean detached worktree. No state meaning, correctness, coverage, defect, or repair conclusion was selected. | Macro 1/5 reserve; closed scoped evidence |
| 29 | Next standing-eligible target triage | Closed after correction: P-COMP-03 direct-carrier evidence is the selected next cut. The selected scope is one fixed accepted and one fixed rejected row; earlier WRK roots are not a global whitelist. | Macro 1/5 reserve; selection closed |
| 30 | P-COMP-03 direct-carrier record | Closed frozen: `WRK-0012` pre-registered, committed two sidecars, and ran the exact command sequence. Its required numbered result artifact would require an excluded validator/source-hierarchy change, so the record is frozen without repair; its observations are not reusable evidence. | Macro 2 parser-free substrate; frozen L3 stop |
| 31 | Post-WRK-0012 retention-boundary triage | Closed selection: an existing unnumbered `plan/wrk-...` artifact path supports a distinct retained-reproduction question without validator change. The old run remains history and WRK-0012 stays frozen. | Macro 0/1 and 2 reserve; selection closed |
| 32 | WRK-0013 retained-reproduction registration | Closed: committed a new L3 preregistration that pins the two sidecars as inputs, declares the exact unnumbered result memo/index path and stop line, and performs no outcome command or plan/index edit. | Macro 2 parser-free substrate; registration closed |
| 33 | WRK-0013 fresh retained reproduction | Closed: the fresh registered two-input command reproduced both classifications, and the exact unnumbered memo/index/report delta passed unchanged validation and was manifested as `not-promoted` L3 evidence. | Macro 2 parser-free substrate; closed scoped evidence |
| 34 | Post-WRK-0013 standing-eligible target triage | Closed as evidence-backed no-candidate after local and whole-portfolio review. Surface source-patch / ELAB lacks an exact shared literal key; source probes, current-L2 variants, OBL-024/025 gaps, and operational reserves either duplicate prior evidence, lack live branches, or require a reserved choice. This historical cut does not forbid a later, distinct source-grounded theory mismatch. | Macro 0/1 and 2 reserve; reopen on exact structural mismatch plus bounded downstream decision |
| 35 | Theory-core correspondence checkpoint | Closed: clean Lean replay and independent reviews confirm that the current LAB OBL-001 draft's Result carrier is not a Core/write enumeration proof, its familywise OBL-020 wrapper cannot replace global preservation without coverage, and its OBL-021 coherence draft does not supply outcome totality. The audit subsequently supports a distinct parameter-only variance question, without selecting a Canon interface. | Macro 1/5 reserve; PROPOSAL-008 is an open owner-decision request with no owner answer |
| 36 | WRK-0014 same-carrier variance evidence | Closed scoped evidence: three standalone Lean lemmas compile with only identical carrier and relation parameters. In their stated forms, intended-to-model inclusion suffices to transfer universal safety/coherence and model-to-intended realization suffices to transfer outcome existence. `f01e5160` passes source-history audit only with reversible ignored-local-state quarantine; the normal audit rejects those local files by design. No actual bridge, concrete Canon relation, coverage/realizability, fairness, outcome representation, OBL, or theorem interface is introduced. | Macro 1/5 reserve; use as proof-hygiene guard, then reopen only for a distinct actual-bridge candidate |
| 37 | Post-WRK-0014 actual-bridge triage | Closed as evidence-backed no-candidate: the OBL-020 LAB source has only one abstract `P.Step` and no second same-carrier relation/literal mapping or variance-lemma importer. OBL-001 requires a Core/write interface and OBL-021 is PROPOSAL-008-bound. | Macro 1/5 reserve; reopen only on an existing second relation/mapping, an already-fixed proof interface, or owner/canon action |
| 38 | Post-WRK-0014 remaining-ledger revalidation | Closed as evidence-backed no-candidate for the screened OBL-024/025 and authority/time/cut families: they repeat recorded T-RESEARCH boundaries unless a reserved association, repair, epoch/evidence, frame, rollback, or clock relation is selected. The smallest useful actual-bridge prerequisite is an OBL-001 direct-`c` versus output/Core-write interface decision. | Macro 1/5 reserve; actual-bridge reopen needs a new relation/mapping or fixed proof interface. Other ADR-0014-eligible non-duplicative L3 research remains independently selectable. |
| 39 | P-SURF-05 stale-fence registration preflight | Closed without a WRK: the current validator rejects the selected checker/test/sample inputs; the exploratory command remains excluded and no fresh command ran. Do not bypass the validator, widen it, or conceal inputs autonomously. Whether this guardrail is the complete Canon lane catalog is not decided here. | Macro 4 / reserve; current fail-close. Reopen only after catalog correspondence is resolved, or with another concrete candidate already admitted by the validator. |
| 40 | Post-WRK-0015 current-root triage | Closed as a bounded no-candidate selection: duplicates, absent concrete source/digest differentiation, and reserved-boundary variants were separately rejected. No WRK or fresh command was created. Non-duplication/exact-command/live-decision are this run's priority filters, not new ADR requirements. | Macro 4 / reserve; reopen on a concrete current-validator-permitted dossier with pre-registrable alternative/falsifier and no reserved choice. |
| 41 | Standing-autonomy lane correspondence checkpoint | Closed as the `plan/158` finite-ratchet checkpoint: validator tuple is deliberate fail-closed behavior, while its exhaustive correspondence to ADR-0014's existing documented lane remains UNRESOLVED. No Canon or validator root-policy change was made. | Macro 0/1 and 4 checkpoint; owner may choose closed catalog, reviewed guardrail correction, or defer. Other concrete admitted L3 candidates remain independently selectable. |
| 42 | Foundational local-predicate candidate selection | Closed selection: OBL-005 repeats Report 2262; OBL-015 has no identified consumer that treats the IFC Boolean helper as grant-lineage evidence. Only all-input `captureSubset` constructivity is selected, as a non-OBL existing-LAB experiment with positive controls. | Macro 1/5 reserve; pre-register a fresh record before any outcome command, then freeze on helper/API, generic-carrier, global-instance, classical-leakage, or OBL/Line-1 scope pressure. |

## self-driven macro phase reading

- **Closeable by research:** scoped candidate comparison, countermodel, literal transcription, conditional Lean lemma, existing-lane implementation validation, and falsifier-driven reliance stop.
- **Research discovers:** the smallest premise, carrier, relation, or proof organization that must be escalated, rather than assuming it from LAB.
- **Owner decides:** L0/L1, primitives, external contracts, SCN/Gate/Phase, final proof / OBL discharge, and public completion.

## user decision gates

| Item | Impact | Options | Current recommendation |
| --- | --- | --- | --- |
| G0-D3 | G0 exit and official T1 entry | continue defer; owner reopens and records canonical exit | dormant; do not solicit absent owner reopen |
| OBL-001 Core/result correspondence | whether a future proof-facing package uses direct Core `c` or a selected Result/write enumeration bridge | defer; authorize a specific formalization boundary | prefer direct-`c` reading; do not identify experiment-only Result with Canon Core |
| PROPOSAL-003 | OBL-020 formalization organization | A shared checklist; B package-local organization; C defer | owner chooses; exclude from pilot |
| PROPOSAL-004 | Surface v0 grammar closure | A Participant-only closure; B custom keyspaces; C defer | A is LAB recommendation; owner chooses |
| PROPOSAL-008 | BND-001 outcome-totality interpretation | separate obligation; OBL-021 scope; outcome-classification reading; defer | owner chooses before a proof-facing package assumes totality |

Routine non-reserved target selection is **not** a user decision gate. A target that touches the reserved boundary is escalated with its evidence rather than placed in `working/`.

## research discovery items

| Item | What research must establish | Current route |
| --- | --- | --- |
| OBL-020 / OBL-021 / OBL-001 / OBL-024 / OBL-025 | whether a scoped premise or relation is canon-compatible, sufficiently explicit, and non-reserved | historical source-adequacy evidence in `plan/156`; plan/171 records the LAB reading that the familywise wrapper cannot replace direct global OBL-020 without coverage, the OBL-021 coherence draft does not give totality, and WRK-0007's Result/write gap needs a future direct-`c` or explicit bridge. Manifested WRK-0014 establishes only two sufficient same-carrier transfer forms: intended-to-model for universal safety/coherence and model-to-intended for outcome existence. Post-W14 actual-bridge and remaining-ledger screens found no distinct record in their screened families: no second relation/mapping exists, while diagnostics and authority/time/cut repeat recorded source boundaries. Those actual-bridge conditions do not narrow independent ADR-0014 L3 eligibility. P008 is an open BND-001 totality decision request. No `theory/11` movement |
| textual runnable research core | whether existing source-first lanes can yield a distinct direct execution or literal parity question without changing governance | WRK-0013 retained fresh execution provenance and the existing unnumbered result-memo path as not-promoted L3 evidence. Local and portfolio triage are closed as no-candidate: Surface artifacts lack an exact shared key, while Full System V1 and Product Alpha source observations remain reserves until an exact structural mismatch and documented downstream branch appear. This does not establish the old carrier result, general direct execution, or workflow readiness. |
| existence, authority, observation, cut, diagnostics | a smallest formal boundary without turning an experiment-local carrier into canon | T-RESEARCH-009..027 are historical bounds; WRK-0008 closes only the current-L2 formal-hook attribution gap. WRK-0009 closes a separate e5 literal-identity mismatch; any mapping/synthetic-role question needs a new registered falsifier, not a helper/schema repair |
| proof ledger integrity | that no LAB evidence is presented as a ledger/status change | `mirrorea_canon/theory/11-metatheory-ledger.md`; every ledger edit is owner-reserved |
| literature / comparison | that a comparison clarifies an existing canon difference without importing a new primitive | `mirrorea_canon/theory/12-literature.md`; existing literature route |

## maintenance tasks

- Keep `docs/project-status.md`, `progress.md`, `tasks.md`, and `samples_progress.md` synchronized when their owned dimensions change. The latest runnable baseline is `plan/161`; it does not widen working-annex evidence roots.
- Re-run documentation/source-hierarchy validation after plan or snapshot changes. Rebase/freeze a proposed L2 working-record update before review and re-review when its cited canon blob or diff changes.
- Keep research source in existing documented LAB lanes. It is retained as WRK evidence only through an append-only manifested full commit; it is never production implementation or a conformance surface.
- Before heavy build / generated-artifact work, recheck root capacity and prefer the configured external workdir, which remains unmounted. The approved cleanup removed local `target/` and Mirrorea `/tmp` artifacts; see `docs/reports/2295-approved-artifact-cleanup.md`.
- Commit with `--no-gpg-sign` and push each completed task package.

## non-promoted references

- Canon lifecycle, Gates, and phases: `mirrorea_canon/plan/00-gates.md` and `mirrorea_canon/plan/01-phases.md`.
- Delegated research authority: `mirrorea_canon/adr/ADR-0014.md`, `mirrorea_canon/working/README.md`, and `mirrorea_canon/plan/02-operating-model.md`.
- Current lifecycle and finite autonomous horizon: `plan/158-standing-bounded-autonomy.md`.
- WRK evidence-commit integrity and its limits: `plan/159-wrk-evidence-commit-integrity-recut.md`.
- Current no-candidate triage and runnable baseline: `plan/161-post-checkpoint-candidate-triage-and-runnable-baseline.md`.
- Post-WRK-0006 candidate selection: `plan/162-post-wrk0006-candidate-selection.md`.
- Foundation integrity and BND-001 outcome audit: `plan/163-foundation-integrity-and-elaboration-outcome-audit.md`.
- OBL-001 Result/write coverage boundary: `plan/164-obl001-result-write-coverage-boundary.md`.
- WRK-0007 evidence: `plan/wrk-0007-obl001-result-write-coverage.md`.
- Post-WRK-0007 candidate selection: `plan/165-post-wrk0007-candidate-selection.md`.
- WRK-0008 formal-hook attribution evidence: `plan/wrk-0008-obl027-formal-hook-attribution.md`.
- WRK-0009 e5 identity selection: `plan/168-wrk0009-e5-skeleton-identity-selection.md`.
- WRK-0009 e5 identity evidence: `plan/wrk-0009-e5-skeleton-identity.md`.
- WRK-0010 static decision attribution evidence: `plan/wrk-0010-static-formal-hook-decision-attribution.md`.
- WRK-0011 final-store assertion directness evidence: `plan/wrk-0011-current-l2-final-store-directness.md`.
- Post-WRK-0011 corrected candidate selection: `plan/170-post-wrk0011-candidate-selection.md`.
- Mir computational directness audit: `plan/166-mir-computational-baseline-directness-audit.md`.
- Historical research evidence: `plan/156-t0-t2-research-autonomy-envelope.md` and `plan/157-delegated-theory-research-governance.md`.
- Proof status: `mirrorea_canon/theory/11-metatheory-ledger.md`.
- Runnable LAB dashboard: `samples_progress.md`.
