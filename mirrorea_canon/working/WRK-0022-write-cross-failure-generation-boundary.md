---
id: working/WRK-0022
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/01-mircore-v0, theory/03-elaboration, theory/11-metatheory-ledger]
summary: WRITE-CROSS の表示済み failure-row 包含節だけが experiment-local な phi_gen を一意に定めるかを、既存 OBL-021 Lean lane の有限 countermodel で検査する L3 record。意図した生成関数や Canon derivation は選ばない。
open_items: []
---

# WRK-0022 - WRITE-CROSS failure-generation boundary

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@4be022b3a0d21b533c557e5745e3b57dc2b423f6:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/01-mircore-v0@4be022b3a0d21b533c557e5745e3b57dc2b423f6:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/03-elaboration@4be022b3a0d21b533c557e5745e3b57dc2b423f6:2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641, theory/11-metatheory-ledger@4be022b3a0d21b533c557e5745e3b57dc2b423f6:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1
LAB inputs: LAB:plan/76-g1-obl020-021-dependency-inventory.md@4be022b3a0d21b533c557e5745e3b57dc2b423f6:4188230f71e39b3bbeedecf5ca1a878d8196d68804bd7ba5e2cc07ed0f2e1cf1, LAB:plan/179-independent-source-locus-audit.md@4be022b3a0d21b533c557e5745e3b57dc2b423f6:1f88fcf9e8e83ba330dcb0aa0ded7768c170b3ca7219276787ecdd24a2ad39c1, LAB:plan/180-t1-t2-statement-identity-dependency-closure-audit.md@4be022b3a0d21b533c557e5745e3b57dc2b423f6:80725fbf9fbdaa45f49406a5d1a7c03e2a1e25f586ad5c0dff9997a963a4a2d1, LAB:plan/189-autonomous-theory-frontier-revalidation.md@4be022b3a0d21b533c557e5745e3b57dc2b423f6:5f5d1992045495d72ec8bbc633fdacf48b2836885968ff29051accc9ebfd9722, LAB:samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean@4be022b3a0d21b533c557e5745e3b57dc2b423f6:7aa5e01caedc393326c070ffaf033a314c7849db2e734d7b03b34b6d92b6cf0a, LAB:samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.md@4be022b3a0d21b533c557e5745e3b57dc2b423f6:afaab1b252ef9c8adf452c31a9449beceedf79b0f94a9d05090216576742844f
Permitted LAB locations: plan, samples/lean
Reserved surfaces: excluded

## Pre-registered working question

Question: For the literal finite experiment `Allowed = {StaleMembership,
MissingCapability, MissingWitness, RouteUnavailable, VisibilityDenied,
TypeMismatch}` and `Declared = {StaleMembership}`, do the two displayed
`[WRITE-CROSS]` containment clauses alone, `phi_gen subset Allowed` and
`phi_gen subset Declared`, force a unique experiment-local generated-failure
row? The two fixed candidate rows are the empty row and `{StaleMembership}`.
Status quo: theory/01 displays those two containment clauses for
`[WRITE-CROSS]`, while theory/03 requires elaboration to be function-like.
LAB plan/76 lists a fixed failure-generation function as an OBL-021 dependency.
The current OBL-021 draft compares a supplied failure-row projection, but it
does not define this generator. This record does not assert that the rule
sketch admits two Canon derivations.
Alternative: a pinned source already makes the intended generator explicit; a
matching retained countermodel already exists; or the finite construction
cannot prove that both rows satisfy the displayed clauses and differ. Any of
these rejects this premise-sufficiency result.
Expected falsifier: Any pinned digest differs; the pre-source marker is already
present; the source explicitly forces a unique generator in this scope; Lean
cannot prove both containment instances and row difference; the result requires
a generator choice, final failure-row equivalence, Core/result correspondence,
Diagnostic behavior, or a new helper, schema, CI/Make surface.
Rollback / reopen trigger: On any falsifier, set `Reliance status` to `frozen`,
retain only reproducible failure evidence, and do not repair or rerun this
record. Escalate rather than repair if a next step would choose a failure
generator, failure-row carrier/equivalence, Core or outcome representation,
OBL statement/proof/status, diagnostic ABI, authority semantics, Gate/Phase,
or public behavior.

## Method and evidence plan

Result class: countermodel
Commands: lean --version; test ! -e samples/lean/lab-statements/obl021/WriteCrossFailureGenerationBoundary.lean; lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean; lean samples/lean/lab-statements/obl021/WriteCrossFailureGenerationBoundary.lean; python3 -c "from pathlib import Path; text = Path('samples/lean/lab-statements/obl021/WriteCrossFailureGenerationBoundary.lean').read_text(); required = ('empty_row_satisfies_displayed_premises', 'singleton_row_satisfies_displayed_premises', 'candidate_rows_are_distinct', 'displayed_premises_do_not_determine_row'); forbidden = ('sorry', 'admit', 'axiom', 'unsafe', 'partial', 'implemented_by'); assert all(name in text for name in required); assert not any(token in text for token in forbidden)"; python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
Execution cut: `4be022b3a0d21b533c557e5745e3b57dc2b423f6` is the authority/input snapshot. Execute no outcome command before this registration is committed and pushed. After the pushed marker check, the evidence commit may add only the declared standalone Lean source and explanation in the existing OBL-021 lane, `plan/wrk-0022-write-cross-failure-generation-boundary.md` with its `plan/00-index.md` entry, a direct numbered report, and allowed working-record operational metadata. It may not modify the statement draft, manifest, helper, schema, CI/Make, runtime, parser, theory, or public surface. A later metadata-only commit may append exact evidence commits and artifact digests without rewriting this pre-registration.
Non-claims: This does not assert that either finite row is the intended Canon
output or semantically admissible; that Canon elaboration has two derivations
or is nondeterministic; a required failure-generation function; final
failure-row carrier/equality; E-ROW-001 behavior or a diagnostic defect;
Core/result correspondence; BND-001 totality; P008/P012 dispositions; an OBL
statement, proof, status, Gate, Phase, implementation, conformance, or public
claim.

## Results and review

Reliance status: frozen
Positive evidence: none. The registered transient source did not compile, so
it establishes no finite premise countermodel or failure-generation conclusion.
Negative evidence: After registration commit `cc8652f9` was pushed, the
pre-source marker check passed and the existing OBL-021 statement draft
compiled with Lean 4.29.1. The declared standalone source was then added only
in the existing OBL-021 lane and the exact registered bare `lean` command
failed at line 1 with `unknown module prefix 'samples'`. This is the registered
`Lean cannot prove both containment instances and row difference` falsifier.
The source-audit and synchronization commands ran independently after source
creation, but they do not establish a countermodel after the failed import. The
transient source and explanation were removed. Do not repair or rerun this
record.
Evidence artifacts: LAB:plan/wrk-0022-write-cross-failure-generation-boundary-falsifier.md@81781b4e6fbf6feb8f82676e7e142fe59dc45070:bda556db8668ce1d2501cc6706176ae5bbb33db5f1f7ce76da628b347d4fc86c
Evidence commits: 81781b4e6fbf6feb8f82676e7e142fe59dc45070
Impact / non-effects: This route is frozen before it establishes any
displayed-premise result. It neither defines nor changes the intended
failure-generation function, Canon rule, OBL-021 statement/status,
implementation, or public behavior. A future inquiry requires a distinct
registration; it must not repair or reuse this record as a premise result.
Independent review: not-required-for-L3

## Supersession

Supersession: none
