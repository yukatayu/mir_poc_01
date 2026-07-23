---
id: working/WRK-0021
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/02-types-effects-failures, meta/proposal-011]
summary: active clean-near-end の三 counter CostBudget に限り、ordinary scalar total の順序が pointwise natural bound を反映するという実験候補を finite Lean countermodel で検査する。Contract の cost rule、final algebra、runtime は選ばない。
open_items: []
---

# WRK-0021 - CostBudget scalar-projection countermodel

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@bfa1e809165226873bc7cfea523450054e09bd11:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/02-types-effects-failures@bfa1e809165226873bc7cfea523450054e09bd11:40c49504e86162fb065d0f5850c4039d88d08af30da7d12dc2e073c43a107257, meta/proposal-011@bfa1e809165226873bc7cfea523450054e09bd11:eeb6c0fd34620969503b93e969955406cde548732174e18bb5a38267cb2e47b1
LAB inputs: LAB:plan/183-transparent-cost-bound-substitutability-decision.md@bfa1e809165226873bc7cfea523450054e09bd11:284534ba6a0192fff18f741c6be8bf1b5183445633f46f20d1199c322c27d6ca, LAB:samples/clean-near-end/00_index_theories.mir@bfa1e809165226873bc7cfea523450054e09bd11:c79f4a8ec98e2d5a0e6ba7171c70ce76e73cdcffa5acdb1bde04fda92be99664, LAB:samples/clean-near-end/typing/05_cost_bound_rejected.mir@bfa1e809165226873bc7cfea523450054e09bd11:f6ff4f098a98e6b7ba92744ebf778b325408f831e46fcc51e7a17137d7d583db, LAB:samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean@bfa1e809165226873bc7cfea523450054e09bd11:dd3eeffcd5dc4ed0b90496f4b048941f6d3aede7e5142f07a8385de1581c1b64
Permitted LAB locations: plan, samples/lean, samples/clean-near-end
Reserved surfaces: excluded

## Pre-registered working question

Question: For the existing LAB CostBudget names `cpu_steps`, `remote_calls`,
and `writes`, does the explicitly experimental ordinary scalar-total relation
`total(candidate) <= total(reference)` imply the model's pointwise natural
bound `candidate <=pointwise reference`? The fixed candidate is `(0, 1, 0)`
and the fixed reference is `(1, 0, 0)` in that named-counter order.
Status quo: The active clean-near-end input declares these three counters with
`pointwise_natural_bound`; its cost-negative sample demonstrates one
remote-call bound rejection. `plan/183` says that no scalar-total policy is
selected and permits a separately pre-registered projection test only.
Alternative: The named source may not state this three-counter pointwise model;
the selected finite pair may fail to satisfy the scalar relation or may satisfy
the pointwise relation; or a prior matching countermodel may already exist. In
each case, this record retains no scalar-reflection countermodel result.
Expected falsifier: Any pinned digest differs; the pre-source search finds the
registered theorem marker already present; the selected source lacks the named
counters or pointwise-law text; Lean cannot establish both the scalar relation
and the failed pointwise relation for the fixed pair; or answering requires an
old/new Contract direction, a final cost algebra, a layer classification, or a
runtime accounting interpretation.
Rollback / reopen trigger: On any falsifier, set Reliance status to frozen,
retain only reproducible failure evidence, and do not repair this record. A
future inquiry must use a distinct registration. Escalate rather than repair if
the work requires choosing a Contract cost relation, transparent-overlay
classification, OBL-026 premise, patch carrier, final cost algebra, runtime
accounting model, checker semantics, helper/schema/CI/Make surface, or public
claim.

## Method and evidence plan

Result class: countermodel
Commands: lean --version; ! rg -q 'scalar_total_does_not_reflect_pointwise' samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean; lean --trust=0 samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean; python3 -c 'from pathlib import Path; text=Path("samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean").read_text(); required=("ThreeCounterBudget", "pointwiseBudgetLeq", "scalarTotalLeq", "scalar_total_does_not_reflect_pointwise"); forbidden=("sorry", "admit", "axiom", "unsafe", "partial", "implemented_by"); assert all(name in text for name in required); assert not any(token in text for token in forbidden)'; python3 scripts/clean_near_end_samples.py run typing --format json; python3 scripts/current_l2_lean_sample_sync.py; git diff --exit-code -- samples/lean/manifest.json
Execution cut: `bfa1e809165226873bc7cfea523450054e09bd11` is the authority/input snapshot. Execute the pre-source marker check only after this registration commit is committed and pushed. After that red check, the evidence commit may add only the declared existing Lean foundation source and explanation update, `plan/wrk-0021-costbudget-scalar-projection-countermodel.md`, its `plan/00-index.md` entry, a direct numbered report, allowed working-record metadata/control files, and no helper, schema, CI/Make, runtime, parser, contract, or public artifact. A later metadata-only commit may append the exact evidence commit and artifact digests without rewriting this pre-registration.
Non-claims: This does not select pointwise order, scalar total, named-vector, weighted, probabilistic, or another cost algebra as Canon; define an old/new Contract direction; classify any layer as transparent/non-transparent; amend theory/02, theory/08, theory/11, OPEN-013, OBL-026, a Gate, a Phase, scenario, or patch carrier; interpret the clean-near-end checker as a Contract/layer evaluator; change runtime accounting, parser, helper, schema, CI/Make, API, transport, conformance, L2 status, proof status, or public behavior.

## Results and review

Reliance status: frozen
Positive evidence: none. The registered source amendment did not compile, so
it establishes no scalar/pointwise countermodel and no CostBudget conclusion.
Negative evidence: After registration commit `4ac08f77` was pushed, the
pre-source marker check passed. The declared source amendment was then added
only in the existing Lean foundation and the exact registered Lean command
failed at lines 152 and 156: Lean could not synthesize `Decidable
(scalarTotalLeq scalarCandidate scalarReference)` or `Decidable
¬pointwiseBudgetLeq scalarCandidate scalarReference`. This is the registered
`Lean cannot establish` falsifier. The outer command used semicolon separators,
so later baseline commands ran after that failure; their output is not retained
as countermodel evidence. The amendment and generated manifest change were
restored. Do not repair or rerun this record.
Evidence artifacts: none
Evidence commits: none
Impact / non-effects: This route is frozen before it establishes the fixed
scalar-reflection implication. It cannot make the pointwise relation, the
witness pair, any scalar representation, a Canon or Contract rule, or decide
PROPOSAL-011. A future inquiry requires a distinct registration; it must not
repair or reuse this record as a CostBudget result.
Independent review: not-required-for-L3

## Supersession

Supersession: none
