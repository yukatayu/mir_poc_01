---
id: working/WRK-0002
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/03-elaboration, theory/10-diagnostics]
summary: OBL-021 の LAB statement draft が projection の存在・一意性なしに異なる成功 Result を許すかを、既存 Lean lane の小さな countermodel で検査する L3 research record。最終 equality、Diagnostic ABI、OBL 状態は変更しない。
open_items: []
---

# WRK-0002 - OBL-021 projection vacuity

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: theory/03-elaboration@f8fb02a8bc77dea82faba20a6760d5b1b525b0db:2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641, theory/10-diagnostics@f8fb02a8bc77dea82faba20a6760d5b1b525b0db:3aa700a8cb1737028006e11f7574bddcfa60d9f145218ab775976857f184f5da, theory/11-metatheory-ledger@f8fb02a8bc77dea82faba20a6760d5b1b525b0db:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1
LAB inputs: LAB:samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean@f8fb02a8bc77dea82faba20a6760d5b1b525b0db:7aa5e01caedc393326c070ffaf033a314c7849db2e734d7b03b34b6d92b6cf0a, LAB:samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.md@f8fb02a8bc77dea82faba20a6760d5b1b525b0db:afaab1b252ef9c8adf452c31a9449beceedf79b0f94a9d05090216576742844f, LAB:samples/lean/manifest.json@f8fb02a8bc77dea82faba20a6760d5b1b525b0db:30cb502777060e23092fd159a9ea1671b192b93c726e9bc19bc78a96233b09ae, LAB:plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md@f8fb02a8bc77dea82faba20a6760d5b1b525b0db:bb52f36f846e80cfdaffd8d6b24ef8d033fb5b878306f96a4a14a2f84e195561, LAB:plan/140-g1-obl021-artifact-annex-template.md@f8fb02a8bc77dea82faba20a6760d5b1b525b0db:1e921164176fac1857981d3f20f953ab122b537c3f94511c2bcf53c1094ff9de, LAB:plan/143-g1-obl021-equality-diagnostic-abstraction-decision-packet.md@f8fb02a8bc77dea82faba20a6760d5b1b525b0db:f4f113dcfdd648f9343746205bfdfae1c2e63960ec1f8cdef0b2c9c817e7ffe9
Permitted LAB locations: plan, samples/lean
Reserved surfaces: excluded

## Pre-registered working question

Question: Does the existing LAB `OBL021StatementDraft` permit two distinct successful `Result` values for one fixed well-scoped input when every result-projection predicate is empty, so that `SameElabResult` holds only vacuously? If so, the draft is insufficient by itself to justify result-identity determinism or projection non-vacuity.
Status quo: theory/03 requires elaboration to be a function of its inputs, while the existing LAB statement draft compares only projected components through abstract predicates. The accompanying LAB boundary records leave final equality, diagnostic equivalence, and projection totality/uniqueness unresolved.
Alternative: the current definitions already force sufficient non-vacuity or equivalence, making a model with `OBL021StatementDraft` and two extensionally distinct successful results impossible; the statement draft would then reject this specific vacuity concern.
Expected falsifier: Lean cannot check the countermodel, the registered source audit finds an existing totality/uniqueness/result-identity condition that invalidates the construction, or the concrete model cannot simultaneously establish the draft and distinct successful results.
Rollback / reopen trigger: On any falsifier, set Reliance status to frozen, retain the failed evidence, and create a narrower successor or escalation bundle if resolving the result requires final equality, diagnostic-equivalence, projection-totality, or statement-status selection.

## Method and evidence plan

Result class: countermodel
Commands: lean --version; lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean; test ! -e samples/lean/lab-statements/obl021/ElabDeterminismProjectionVacuityCountermodel.lean; lean samples/lean/lab-statements/obl021/ElabDeterminismProjectionVacuityCountermodel.lean; python3 -c "from pathlib import Path; text = Path('samples/lean/lab-statements/obl021/ElabDeterminismProjectionVacuityCountermodel.lean').read_text(); required = ('projection_predicates_are_empty', 'statement_draft_holds', 'distinct_results_can_elaborate'); forbidden = ('sorry', 'admit', 'axiom', 'unsafe', 'partial', 'implemented_by'); assert all(name in text for name in required); assert not any(token in text for token in forbidden)"; python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
Non-claims: This does not select final Result equality, diagnostic equivalence, Diagnostic ABI, projection-totality law, input identity, Core IR, grammar, runtime scheduling semantics, parser/checker correctness, an OBL-021 requested status, OBL-021 proof or discharge, theory/11 movement, conformance, Gate/Phase action, L2 promotion, or a public implementation claim.

## Results and review

Reliance status: not-promoted
Positive evidence: pending
Negative evidence: pending
Evidence artifacts: pending
Evidence commits: none
Impact / non-effects: The record uses only existing `plan` and `samples/lean` lanes. It may add a LAB-only countermodel and explanation in the existing OBL-021 statement-draft directory, plus manifest/plan/report metadata. It introduces no primitive, helper family, schema, CI/Make target, source/runtime API, contract, Gate/Phase action, proof-status change, or public claim.
Independent review: not-required-for-L3

## Supersession

Supersession: none
