---
id: working/WRK-0009
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/06-existence-fallback, theory/11-metatheory-ledger]
summary: existing current-L2 static e5 route と Lean proof-skeleton の review-unit / emitted-stub identity tuple が literal に整合するかを監査する可逆な L3 record。theorem 意味、OBL、carrier、helper/schema は選ばない。
open_items: []
---

# WRK-0009 - current-L2 e5 proof-skeleton identity fidelity

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@e36c804b9149e048c6e92bec2b55d21956354f2f:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/06-existence-fallback@e36c804b9149e048c6e92bec2b55d21956354f2f:3da20d43a0a87ec8417a4519700777adea141f499e2627f433927ce975a086c8, theory/11-metatheory-ledger@e36c804b9149e048c6e92bec2b55d21956354f2f:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1
LAB inputs: LAB:plan/158-standing-bounded-autonomy.md@e36c804b9149e048c6e92bec2b55d21956354f2f:df6e0a6be32f955d003a073803c635dd461e2d857dd5a743c18f040f96bb2ced, LAB:plan/73-g1-obl001-lean-statement-inventory.md@e36c804b9149e048c6e92bec2b55d21956354f2f:08f808c8adff91641a0ca2d5b4036b2f08339bd8363abf3d4e33d5a7bdec806c, LAB:samples/current-l2/README.md@e36c804b9149e048c6e92bec2b55d21956354f2f:c5301214b29162a1c2e60224f36b325a8e9fd289ed9121bd6df60dd4d48a5eb8, LAB:samples/current-l2/e5-underdeclared-lineage.txt@e36c804b9149e048c6e92bec2b55d21956354f2f:fb98721840001465c29e51ceb3bfd27e98c86f5543eaffa1d892c8e16c617037, LAB:samples/lean/foundations/CurrentL2ProofSkeleton.lean@e36c804b9149e048c6e92bec2b55d21956354f2f:18e85288a37dafe3d0f2814144f8de6e64a765c36d13bd0ecf0ebc28574e8b0b, LAB:samples/lean/foundations/CurrentL2ProofSkeleton.md@e36c804b9149e048c6e92bec2b55d21956354f2f:d3c8a9cef082a507fa8b6646ee00138eff7608d6c6ec0523a34a5b9996796bb6
Permitted LAB locations: plan, samples/current-l2, samples/lean
Reserved surfaces: excluded

## Pre-registered working question

Question: Does `CurrentL2ProofSkeleton.lean`'s concrete `e5ReviewUnits` list preserve the active current-L2 static e5 route's exact ordered `(subject_ref, obligation_kind, theorem_name)` identity tuple, or is the foundation internally coherent while representing a distinct, explicitly unrelated synthetic tuple? Here, "preserve" means literal equality unless a pinned registered source explicitly declares a lossless mapping; spelling, casing, punctuation, or suggestive theorem names imply no equivalence.
Status quo: The foundation describes a review-unit to emitted-stub identity pattern and defines e5 as `e5-underdeclared-lineage` with `rollbackCutNonInterference` then `noRePromotion`. The active static e5 route emits `e5_underdeclared_lineage` with `canonical_normalization_law` then `no_repromotion`, and the existing theorem-stub test names its first theorem `e5_underdeclared_lineage__canonical_normalization_law`. The registered inventory explicitly limits the foundation to an identity pattern and excludes rollback/cut and no-repromotion semantics. The listed sources do not declare a lossless route-identity mapping or an explicit statement that this e5 list is unrelated to the active route.
Alternative: The existing lane may emit the foundation's exact ordered tuple; a registered source may explicitly define a lossless mapping; or it may explicitly state that the foundation e5 list is intentionally synthetic and makes no active-route correspondence claim. In any of those outcomes, this record retains no transcription-divergence conclusion.
Expected falsifier: A fresh existing-lane run produces the foundation's exact ordered active e5 tuple without any undeclared normalization, or a source audit in the registered inputs finds an explicit lossless mapping or explicit synthetic/non-correspondence statement. A command failure or need for a new generator, helper, schema, fixture, test, runner, CI/Make surface, carrier, or source change also falsifies this bounded experiment and stops it.
Rollback / reopen trigger: If the expected falsifier occurs, set `Reliance status` to `frozen`, retain only reproducible failure/match evidence, and reopen only through a narrower successor. Escalate rather than repair in place if resolving the question requires treating either tuple as Canon semantics, choosing a mapping/carrier, changing a helper/schema/runner/sample, changing theory/06 or theory/11, moving an OBL/Gate/Phase, or making a public/conformance claim.

## Method and evidence plan

Result class: literal-transcription
Commands: workdir="$(mktemp -d /tmp/mirrorea-wrk0009-e5-skeleton.XXXXXX)" && lean samples/lean/foundations/CurrentL2ProofSkeleton.lean && sed -n '17,40p;56,64p' samples/lean/foundations/CurrentL2ProofSkeleton.lean && cargo test -p mir-semantics --test current_l2_lean_theorem_stub_support && python3 scripts/current_l2_theorem_lean_stub_pipeline.py e5-underdeclared-lineage --artifact-root "$workdir" --run-label wrk0009 && jq -S '[.[] | {subject_ref, obligation_kind: .row.obligation_kind}]' "$workdir/proof-notebook-review-units/wrk0009-e5-underdeclared-lineage/e5-underdeclared-lineage.proof-notebook-review-unit.json" && jq -S '[.[] | {subject_ref, obligation_kind, theorem_name}]' "$workdir/lean-theorem-stubs/wrk0009-e5-underdeclared-lineage/e5-underdeclared-lineage.lean-theorem-stub.json" && python3 scripts/current_l2_source_sample_regression.py regression --artifact-root "$workdir/regression" --run-label wrk0009
Non-claims: This does not claim that `rollbackCutNonInterference` corresponds to `canonical_normalization_law`, that `noRePromotion` corresponds to `no_repromotion`, that hyphens and underscores are interchangeable, or that either LAB artifact is Canon-correct. It does not prove, state, or discharge any theorem/OBL; interpret a tuple as Canon fallback, rollback, cut, or normalization semantics; amend theory/06 or theory/11; select a carrier, mapping, equality, or final theorem interface; modify any helper, schema, fixture, test, runner, CI/Make surface, runtime, parser, transport, public API, product behavior, SCN, Gate, Phase, or conformance classification; or claim a formal verification route.

## Results and review

Reliance status: not-promoted
Positive evidence: Pending pre-registered existing-lane execution.
Negative evidence: Pending pre-registered existing-lane execution.
Evidence artifacts: pending
Evidence commits: none
Impact / non-effects: This record is limited to the declared `plan`, `samples/current-l2`, and `samples/lean` LAB locations plus disposable `/tmp` outputs. It changes no Canon theory, OBL state, carrier, helper, schema, source fixture, test, runner, conformance, Gate/Phase, runtime, or public behavior.
Independent review: not-required-for-L3

## Supersession

Supersession: none
