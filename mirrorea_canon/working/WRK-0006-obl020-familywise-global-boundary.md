---
id: working/WRK-0006
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/01-mircore-v0, theory/11-metatheory-ledger]
summary: OBL-020 LAB statement draft の global preservation と familywise preservation の論理的接続を、coverage を Canon 要件化せず既存 Lean lane で検査する L3 research record。OBL status、step taxonomy、proof interface は変更しない。
open_items: []
---

# WRK-0006 - OBL-020 familywise/global preservation boundary

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@5f59979ede2079f4f7fe0bb6d3ec9ed70f16ed60:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/01-mircore-v0@5f59979ede2079f4f7fe0bb6d3ec9ed70f16ed60:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/11-metatheory-ledger@5f59979ede2079f4f7fe0bb6d3ec9ed70f16ed60:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1, meta/proposal-003@5f59979ede2079f4f7fe0bb6d3ec9ed70f16ed60:960917141b9e36d089a93f463dd477a0c6efe2e8bbbeb286be7b01a0c325500e
LAB inputs: LAB:samples/lean/lab-statements/obl020/StepWFStatementDraft.lean@5f59979ede2079f4f7fe0bb6d3ec9ed70f16ed60:7fb4e0312f07d950b23249c78537bde9c0b858cd6477ac8ddf3be3f806cfc0cb, LAB:plan/156-t0-t2-research-autonomy-envelope.md@5f59979ede2079f4f7fe0bb6d3ec9ed70f16ed60:dec0742d9ef984441b6b0b35036dcd69e0a597a7b772988359c608e80f21bab0, LAB:plan/158-standing-bounded-autonomy.md@5f59979ede2079f4f7fe0bb6d3ec9ed70f16ed60:df6e0a6be32f955d003a073803c635dd461e2d857dd5a743c18f040f96bb2ced, LAB:plan/161-post-checkpoint-candidate-triage-and-runnable-baseline.md@5f59979ede2079f4f7fe0bb6d3ec9ed70f16ed60:84b2aa6608f82b57a67a24862238fd6a3aec16d047caeafc6a45002591e43c6e
Permitted LAB locations: plan, samples/lean
Reserved surfaces: excluded

## Pre-registered working question

Question: In the existing LAB OBL-020 vocabulary, does the global preservation draft imply the family-qualified preservation wrapper; can the family-qualified wrapper hold while the global draft fails because one actual step is unclassified; and does an explicit experiment-local coverage premise make the family-qualified wrapper sufficient for the global draft? The coverage premise is a conditional proof-composition assumption only, not a Canon requirement, final theorem argument, or step-family design.
Status quo: `OBL020StatementDraft` quantifies only `PreservesWF`, while `CanonStepFamily` and `StepHasFamily` occur only in `FamilyStepPreservesWF`. T-RESEARCH-001 records that unconstrained predicates do not supply preservation and names case exhaustiveness as missing force. T-RESEARCH-006 records that full rule-level OBL-020 needs unselected transition/frame/history/authority/chain premises. The current LAB draft and its documentation explicitly do not claim family completion or per-step proof decomposition.
Alternative: Lean may show that the proposed global-to-family implication, coverage-conditioned converse, or non-vacuous separation model is ill-typed or requires an unstated semantic choice. In that outcome, no familywise/global conclusion is retained; this record is frozen and a narrower successor or escalation is required.
Expected falsifier: the evidence source cannot compile; its model cannot simultaneously contain a canonical family, an actual classified preserving step, family-qualified preservation, and an unclassified non-preserving step; source inspection finds it adds a reusable helper, chooses a canonical family/taxonomy/coverage rule, or changes the existing statement draft; or the claimed conditional converse needs additional premises beyond the explicitly written coverage premise.
Rollback / reopen trigger: On any falsifier, set Reliance status to frozen, retain the failure evidence, and supersede only with a narrower L3 question. Escalate rather than repair in place if resolving the failure needs a Canon transition relation, step taxonomy, coverage/exhaustiveness policy, theorem interface, OBL status, Gate/Phase action, or public/runtime contract.

## Method and evidence plan

Result class: existing-lane-experiment
Commands: lean --version; lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean; test ! -e samples/lean/lab-statements/obl020/FamilywiseGlobalBoundary.lean; lean samples/lean/lab-statements/obl020/FamilywiseGlobalBoundary.lean; for name in global_implies_familywise coverage_and_familywise_imply_global familywise_without_coverage_can_hold_while_global_fails model_has_nonvacuous_canonical_family_and_classified_step; do rg -q "$name" samples/lean/lab-statements/obl020/FamilywiseGlobalBoundary.lean; done; ! rg -n 'sorry|admit|axiom|unsafe|partial|implemented_by|opaque' samples/lean/lab-statements/obl020/FamilywiseGlobalBoundary.lean; python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
Non-claims: This does not identify V.Config, P.WellFormed, P.Step, V.StepLabel, or V.StepFamily with Canon MirCore carriers; choose a canonical step taxonomy; require step-family coverage, uniqueness, disjointness, or partitioning; require a familywise proof decomposition; change the accepted abstract statement draft; answer PROPOSAL-003; improve the 0/65 source-adequacy result; prove or discharge OBL-020; move theory/11; select a scheduler, runtime, transport, public API, contract, conformance, Gate/Phase, or L2 promotion.

## Results and review

Reliance status: not-promoted
Positive evidence: Not run. The evidence source is intentionally absent at registration.
Negative evidence: Not run. The pre-source absence check is reserved for the post-registration evidence package.
Evidence artifacts: none
Evidence commits: none
Impact / non-effects: This record uses only the existing `samples/lean` and `plan` LAB lanes. It examines proof-composition logic inside the existing abstract statement vocabulary. Any conditional coverage premise remains experiment-local and cannot be read as a Canon requirement or final OBL-020 interface.
Independent review: not-required-for-L3

### Advisory review note — 2026-07-22

Temporary Oracle reviews `mirrorea-theory-reopen-20260722` and
`obl020-theory-audit-20260722`, plus a read-only reviewer, agree that the
current draft has no circularity and no full Canon binding. They disagree only
on selection priority: one recommends deferral because the result is low-impact;
the other identifies this exact relation as standing-eligible under ADR-0014.
The record follows the latter only for a bounded L3 experiment and retains the
former's no-overclaim boundary.

## Supersession

Supersession: none
