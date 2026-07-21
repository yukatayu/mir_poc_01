---
id: working/WRK-0004
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/03-elaboration, theory/10-diagnostics]
summary: OBL-021 の LAB statement draft が well-scoped input に対する成功 Result または Diagnostic の存在を単独で要求するかを、no-outcome countermodel で検査する L3 research record。totality の最終帰属、Diagnostic ABI、OBL 状態は変更しない。
open_items: []
---

# WRK-0004 - OBL-021 outcome totality

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: theory/03-elaboration@c5397083ef63268e9fa26031048372ced97d9c90:2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641, theory/10-diagnostics@c5397083ef63268e9fa26031048372ced97d9c90:3aa700a8cb1737028006e11f7574bddcfa60d9f145218ab775976857f184f5da, theory/11-metatheory-ledger@c5397083ef63268e9fa26031048372ced97d9c90:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1
LAB inputs: LAB:samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean@c5397083ef63268e9fa26031048372ced97d9c90:7aa5e01caedc393326c070ffaf033a314c7849db2e734d7b03b34b6d92b6cf0a, LAB:plan/wrk-0003-projection-extensionality-countermodel.md@c5397083ef63268e9fa26031048372ced97d9c90:c1f2794f8551660f960446348833d4b1fc0f5de3e468b1f518f931deb37b8230, LAB:plan/143-g1-obl021-equality-diagnostic-abstraction-decision-packet.md@c5397083ef63268e9fa26031048372ced97d9c90:f4f113dcfdd648f9343746205bfdfae1c2e63960ec1f8cdef0b2c9c817e7ffe9
Permitted LAB locations: plan, samples/lean
Reserved surfaces: excluded

## Pre-registered working question

Question: Does the existing LAB `OBL021StatementDraft` permit a fixed well-scoped input for which no `Result` elaborates and no `Diagnostic` rejects? If so, the draft alone does not express the existence half of Canon BND-001's either-success-or-Diagnostic contract.
Status quo: Canon theory/03 says a well-scoped Surface item either produces the elaboration output tuple or a Diagnostic, and calls elaboration a function of its inputs. The LAB draft contains only conditional success-success, reject-reject, and success-reject clauses; it contains no explicit outcome-existence predicate or totality law. Planner review places this question before choosing a Result-relation bridge.
Alternative: the current draft already entails at least one success or rejection for every well-scoped input, making a no-outcome model impossible; then this particular totality concern is unsupported.
Expected falsifier: Lean cannot check the countermodel; source audit finds an existing outcome-existence/totality condition in the registered statement interface; or the concrete model cannot simultaneously establish a well-scoped input, the draft, and absence of both outcomes.
Rollback / reopen trigger: On any falsifier, set Reliance status to frozen, retain failure evidence, and either narrow the candidate or escalate only if resolving the result requires assigning totality to a Canon obligation, selecting final equality, a Diagnostic ABI, a Canon theory change, or an OBL status decision.

## Method and evidence plan

Result class: countermodel
Commands: lean --version; lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean; test ! -e samples/lean/lab-statements/obl021/ElabDeterminismOutcomeTotalityCountermodel.lean; lean samples/lean/lab-statements/obl021/ElabDeterminismOutcomeTotalityCountermodel.lean; python3 -c "from pathlib import Path; text = Path('samples/lean/lab-statements/obl021/ElabDeterminismOutcomeTotalityCountermodel.lean').read_text(); required = ('well_scoped_input_exists', 'no_successful_result_exists', 'no_diagnostic_exists', 'statement_draft_holds'); forbidden = ('sorry', 'admit', 'axiom', 'unsafe', 'partial', 'implemented_by'); assert all(name in text for name in required); assert not any(token in text for token in forbidden)"; python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
Non-claims: This does not assign outcome totality to OBL-021, OBL-003, an elaborator definition, or any other Canon obligation; select final Result equality, diagnostic equivalence, Diagnostic ABI, input identity, Core IR, grammar, runtime scheduling semantics, parser/checker correctness, an OBL-021 requested status, OBL-021 proof or discharge, theory/11 movement, conformance, Gate/Phase action, L2 promotion, or a public implementation claim.

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
