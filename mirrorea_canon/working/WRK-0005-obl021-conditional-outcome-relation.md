---
id: working/WRK-0005
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/03-elaboration, theory/10-diagnostics]
summary: OBL-021 の LAB statement draft に明示的 outcome-totality を仮定したとき、success/reject をまとめる実験用 SameOutcome relation で全 outcome pair の関係が導けるかを検査する L3 conditional lemma record。最終 equality、relation laws、Canon 帰属、OBL 状態は変更しない。
open_items: []
---

# WRK-0005 - OBL-021 conditional outcome relation

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: theory/03-elaboration@2f33576aceb0ffeb32a424a140d78b8e919a0370:2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641, theory/10-diagnostics@2f33576aceb0ffeb32a424a140d78b8e919a0370:3aa700a8cb1737028006e11f7574bddcfa60d9f145218ab775976857f184f5da, theory/11-metatheory-ledger@2f33576aceb0ffeb32a424a140d78b8e919a0370:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1
LAB inputs: LAB:samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean@2f33576aceb0ffeb32a424a140d78b8e919a0370:7aa5e01caedc393326c070ffaf033a314c7849db2e734d7b03b34b6d92b6cf0a, LAB:plan/wrk-0004-outcome-totality-countermodel.md@2f33576aceb0ffeb32a424a140d78b8e919a0370:9b07fbf5b407f6062213f862919e5e8a4848674b017440e0f63e22e82fa96456, LAB:plan/143-g1-obl021-equality-diagnostic-abstraction-decision-packet.md@2f33576aceb0ffeb32a424a140d78b8e919a0370:f4f113dcfdd648f9343746205bfdfae1c2e63960ec1f8cdef0b2c9c817e7ffe9
Permitted LAB locations: plan, samples/lean
Reserved surfaces: excluded

## Pre-registered working question

Question: If an explicit experiment-local `OutcomeTotal` premise says a fixed well-scoped input has at least one outcome, does the existing LAB `OBL021StatementDraft` entail that every two tagged outcomes are related by an experiment-local `SameOutcome` which delegates success-success to `SameElabResult`, reject-reject to `SameDiagnostic`, and rejects mixed variants? If so, this is the strongest conditional relational reading tested here without selecting Result equality or relation laws.
Status quo: WRK-0004 shows the draft alone does not entail outcome existence. The draft's three conditional clauses already cover success-success, reject-reject, and mixed success/reject cases. The Canon says elaboration is function-like but does not choose native equality, observational equivalence, or quotient semantics for the LAB interface.
Alternative: the outcome wrapper needs an additional premise beyond explicit existence and the draft, or its case analysis cannot establish the relation; then the proposed bridge-free conditional reading is unsupported.
Expected falsifier: Lean cannot check the conditional lemma; source audit finds the wrapper silently selects a final equality, relation law, or Canon condition; or the draft plus explicit outcome totality is insufficient to relate all outcome pairs.
Rollback / reopen trigger: On any falsifier, set Reliance status to frozen, retain failure evidence, and either narrow the conditional relation or escalate only if resolving the result requires selecting final equality, relation laws, a Diagnostic ABI, a Canon theory change, or an OBL status decision.

## Method and evidence plan

Result class: conditional-lemma
Commands: lean --version; lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean; test ! -e samples/lean/lab-statements/obl021/ElabDeterminismConditionalOutcomeRelation.lean; lean samples/lean/lab-statements/obl021/ElabDeterminismConditionalOutcomeRelation.lean; python3 -c "from pathlib import Path; text = Path('samples/lean/lab-statements/obl021/ElabDeterminismConditionalOutcomeRelation.lean').read_text(); required = ('Outcome', 'OutcomeOf', 'SameOutcome', 'outcome_totality_and_draft_imply_unique_relation'); forbidden = ('sorry', 'admit', 'axiom', 'unsafe', 'partial', 'implemented_by'); assert all(name in text for name in required); assert not any(token in text for token in forbidden)"; python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
Non-claims: This does not assign outcome totality to OBL-021, OBL-003, or another Canon obligation; select final Result equality, final Diagnostic equality, an equivalence/setoid law, quotient semantics, a direct public Result relation, Diagnostic ABI, input identity, Core IR, grammar, runtime scheduling semantics, parser/checker correctness, an OBL-021 requested status, OBL-021 proof or discharge, theory/11 movement, conformance, Gate/Phase action, L2 promotion, or a public implementation claim.

## Results and review

Reliance status: not-promoted
Positive evidence: Lean 4.29.1 checks the imported existing statement draft and `statement_draft_implies_outcomes_related`: for a fixed well-scoped input, any two values satisfying `OutcomeOf` are `SameOutcome`-related. Its four constructor combinations use the draft's homogeneous clauses or discharge mixed pairs by the success/reject exclusion. The result may be vacuous when the actual-outcome fiber is empty. `outcome_totality_supplies_witness_and_draft_relates_actual_outcomes` uses the explicit experiment-local `OutcomeTotal` only to unpack one witness and package it with the already-derived guarded all-pairs relation.
Negative evidence: The registered pre-source red check confirmed that the target did not exist before the first evidence commit. The first registered source audit found its historical required names and no `sorry`, `admit`, `axiom`, `unsafe`, `partial`, or `implemented_by` token. Correction evidence then used two red source audits to reject the old and intermediate ambiguous names, followed by a green audit of the final name and the same forbidden tokens. The existing Lean synchronization test passed in both source packages. WRK-0004 remains the separate countermodel showing that the draft itself does not entail the explicit totality premise.
Evidence artifacts: LAB:plan/wrk-0005-conditional-outcome-relation.md@208c5f0ba1013ed513273772ef6b05d30d7d585c:5c9d2c74fc9fe633620169f60698ba12a169afbf25810fb98b41ed81f3d3824b, LAB:samples/lean/lab-statements/obl021/ElabDeterminismConditionalOutcomeRelation.lean@208c5f0ba1013ed513273772ef6b05d30d7d585c:c518f36a45a80d4c6e197fac4c22c828ce2d325409885983e68b0e40b0a40cde, LAB:samples/lean/lab-statements/obl021/ElabDeterminismConditionalOutcomeRelation.md@208c5f0ba1013ed513273772ef6b05d30d7d585c:bd64294ba461d39113e181b241237116511bcab4a43c2b43b5dfab2b32929505, LAB:plan/wrk-0005-conditional-outcome-relation.md@7ab4c91205bf69edf702d7841e9f0fe42cccc9b4:08aee3638bd86333c4c0046faced85530b70737d811b00b6ad6463a079525a7b, LAB:samples/lean/lab-statements/obl021/ElabDeterminismConditionalOutcomeRelation.lean@7ab4c91205bf69edf702d7841e9f0fe42cccc9b4:5eb8f2702b099076ddbab874d0b96c3a6a980e190149b83b3627642beb6209ab, LAB:samples/lean/lab-statements/obl021/ElabDeterminismConditionalOutcomeRelation.md@7ab4c91205bf69edf702d7841e9f0fe42cccc9b4:ebb6acb429b83b55c77a84788afb277c3e4b3d9d2a707adb26320b909b9e8ebf
Evidence commits: 208c5f0ba1013ed513273772ef6b05d30d7d585c, 7ab4c91205bf69edf702d7841e9f0fe42cccc9b4
Impact / non-effects: The record uses only existing `plan` and `samples/lean` lanes. Its retained result is limited to partial relational coherence: the existing LAB draft and well-scopedness relate every pair in one fixed input's actual-outcome fiber, while explicit totality makes that fiber nonempty. It does not establish the totality premise, native equality, a global equivalence/setoid law on all `Outcome` values, quotient semantics, observational adequacy, a final Result relation, a Diagnostic-field/explanation bridge, or a Canon home for totality. It introduces no primitive, reusable helper family, schema, CI/Make target, source/runtime API, contract, Gate/Phase action, proof-status change, or public claim.
Independent review: not-required-for-L3

### Correction addendum — 2026-07-21

The pre-registered question and command list remain historical registration
text. The second evidence commit corrects only the later source presentation:
it does not change the proposition, proof body, premises, permitted lanes, or
non-claims. The initial `unique_relation` name did not prove `ExistsUnique`,
payload equality, or uniqueness of a relation. The restricted all-pairs result
does entail reflexivity, symmetry, and transitivity on the actual-outcome fiber,
but no global or Canon relation law is selected.

## Supersession

Supersession: none
