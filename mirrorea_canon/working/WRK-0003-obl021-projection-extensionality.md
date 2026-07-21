---
id: working/WRK-0003
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/03-elaboration, theory/10-diagnostics]
summary: OBL-021 の LAB statement draft に各 projection の total/unique witness と component equality を与えても異なる成功 Result を許すかを、joint extensionality 不在の小さな countermodel で検査する L3 research record。最終 equality、Diagnostic ABI、OBL 状態は変更しない。
open_items: []
---

# WRK-0003 - OBL-021 projection extensionality

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: theory/03-elaboration@45403b681a3897aadb377ec0ff76a5a0447ca55e:2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641, theory/10-diagnostics@45403b681a3897aadb377ec0ff76a5a0447ca55e:3aa700a8cb1737028006e11f7574bddcfa60d9f145218ab775976857f184f5da, theory/11-metatheory-ledger@45403b681a3897aadb377ec0ff76a5a0447ca55e:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1
LAB inputs: LAB:samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean@45403b681a3897aadb377ec0ff76a5a0447ca55e:7aa5e01caedc393326c070ffaf033a314c7849db2e734d7b03b34b6d92b6cf0a, LAB:samples/lean/lab-statements/obl021/ElabDeterminismProjectionVacuityCountermodel.lean@45403b681a3897aadb377ec0ff76a5a0447ca55e:5cb7a60b68bb0bdf23d8fed5f4e5acf963f162ede5d2a6161bae21b90f548098, LAB:plan/wrk-0002-projection-vacuity-countermodel.md@45403b681a3897aadb377ec0ff76a5a0447ca55e:382cfb7589aec6bf901b4922499f96cdeb497be9ec3cf781abb2d260b6c78890, LAB:plan/143-g1-obl021-equality-diagnostic-abstraction-decision-packet.md@45403b681a3897aadb377ec0ff76a5a0447ca55e:f4f113dcfdd648f9343746205bfdfae1c2e63960ec1f8cdef0b2c9c817e7ffe9
Permitted LAB locations: plan, samples/lean
Reserved surfaces: excluded

## Pre-registered working question

Question: Does the existing LAB `OBL021StatementDraft` still permit two distinct successful `Result` values for one fixed well-scoped input when every listed result-projection predicate is total and unique for each result, and every component-equivalence predicate is native equality? If so, per-projection non-vacuity and equality are insufficient by themselves to derive Result identity without a joint extensionality or direct Result-relation bridge.
Status quo: Canon theory/03 states that a well-scoped input yields the elaboration output tuple or a Diagnostic and that elaboration is a function of its inputs. The LAB draft compares projections but contains no direct Result equality/relation or joint extensionality law. WRK-0002 established that empty projections can make the comparison vacuous; Canon does not define the draft's abstract projection predicates or their witness laws.
Alternative: total and unique projections with equality component comparisons already make a model with the draft and two distinct successful Results impossible, or the current interface already contains a direct identity/extensionality bridge. The proposed countermodel would then fail and this specific insufficiency would not be supported.
Expected falsifier: Lean cannot check the countermodel; source audit finds an existing direct Result equality, extensionality, or equivalent law in the registered statement interface; or the concrete model cannot simultaneously establish total/unique projections, equality component comparisons, the draft, and distinct successes.
Rollback / reopen trigger: On any falsifier, set Reliance status to frozen, retain failure evidence, and either narrow the candidate or escalate only if resolving the result requires selecting final equality, a Diagnostic ABI, a Canon theory change, or an OBL status decision.

## Method and evidence plan

Result class: countermodel
Commands: lean --version; lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean; test ! -e samples/lean/lab-statements/obl021/ElabDeterminismProjectionExtensionalityCountermodel.lean; lean samples/lean/lab-statements/obl021/ElabDeterminismProjectionExtensionalityCountermodel.lean; python3 -c "from pathlib import Path; text = Path('samples/lean/lab-statements/obl021/ElabDeterminismProjectionExtensionalityCountermodel.lean').read_text(); required = ('projection_predicates_are_total_and_unique', 'component_equivalences_are_equality', 'statement_draft_holds', 'distinct_results_can_elaborate'); forbidden = ('sorry', 'admit', 'axiom', 'unsafe', 'partial', 'implemented_by'); assert all(name in text for name in required); assert not any(token in text for token in forbidden)"; python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
Non-claims: This does not select final Result equality, a joint extensionality law, a direct Result relation, diagnostic equivalence, Diagnostic ABI, input identity, projection-totality policy, Core IR, grammar, runtime scheduling semantics, parser/checker correctness, an OBL-021 requested status, OBL-021 proof or discharge, theory/11 movement, conformance, Gate/Phase action, L2 promotion, or a public implementation claim.

## Results and review

Reliance status: not-promoted
Positive evidence: Lean 4.29.1 checks the imported existing statement draft and the countermodel, including `projection_predicates_are_total_and_unique`, `component_equivalences_are_equality`, `statement_draft_holds`, `distinct_results_can_elaborate`, and the corrected `total_unique_equality_projections_still_allow_distinct_results` aggregate theorem. In the concrete model, all nine result projections have one witness per Result and component comparisons are native equality, while two distinct successful Result values coexist for one well-scoped input.
Negative evidence: The registered pre-source red check confirmed that the target did not exist before the evidence commit. The registered source audit found the four required theorem names and no `sorry`, `admit`, `axiom`, `unsafe`, `partial`, or `implemented_by` token. The existing Lean synchronization test passed.
Evidence artifacts: LAB:plan/wrk-0003-projection-extensionality-countermodel.md@bf373a3ff46584d805a5da6618fb8e16a83ea6dd:3b541113342034f00561b121bee6f07a108d8024a9f661f09aaa0b3acc2105ef, LAB:samples/lean/lab-statements/obl021/ElabDeterminismProjectionExtensionalityCountermodel.lean@bf373a3ff46584d805a5da6618fb8e16a83ea6dd:32ff605a4aadee56a907267384868f283dc19ffc43dd98ea1d61f782a1a663fa, LAB:plan/wrk-0003-projection-extensionality-countermodel.md@701a001fcbc76fe17ff52c1c569d373f2828bd01:ae9ce3f74d6152dc2ec7e06ccae10a36e6427afcc724854c9616b3cf55358a1b, LAB:samples/lean/lab-statements/obl021/ElabDeterminismProjectionExtensionalityCountermodel.lean@701a001fcbc76fe17ff52c1c569d373f2828bd01:31cd92b3c62b5d8bf6ece698b6431e123c4d915645f8111daacf0f0bda252000
Evidence commits: bf373a3ff46584d805a5da6618fb8e16a83ea6dd, 701a001fcbc76fe17ff52c1c569d373f2828bd01
Impact / non-effects: The record uses only existing `plan` and `samples/lean` lanes. Its retained result is limited to a countermodel of the current LAB statement shape: it shows that per-projection totality/uniqueness and component equality do not by themselves entail Result identity without a joint extensionality or direct Result-relation bridge. It introduces no primitive, helper family, schema, CI/Make target, source/runtime API, contract, Gate/Phase action, proof-status change, selected bridge, final equality, diagnostic ABI, or public claim.
Independent review: not-required-for-L3

## Supersession

Supersession: none
