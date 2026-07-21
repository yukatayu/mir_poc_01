---
id: working/WRK-0008
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/04-ordering-and-cuts, theory/11-metatheory-ledger, arch/02-boundary-contracts]
summary: existing current-L2 runtime try/cut formal hook が、same-Place atomic_cut frontier の no-cross 境界を区別して根拠化するかを監査する可逆な L3 record。OBL-027、carrier、helper/schema、適合性は選ばない。
open_items: []
---

# WRK-0008 - OBL-027 formal-hook attribution boundary

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@057cff585d42c7974e865c19d33ea0555aa917d5:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/04-ordering-and-cuts@057cff585d42c7974e865c19d33ea0555aa917d5:70bde483330d3745a8694b15cd75f447b6610513ae66cb1ad4ec1faed274a264, theory/11-metatheory-ledger@057cff585d42c7974e865c19d33ea0555aa917d5:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1, arch/02-boundary-contracts@057cff585d42c7974e865c19d33ea0555aa917d5:b9ae8932c10e25d4506f8395a1bbd30aaed8d22f6fd2a293f1a0bed022e39cd3
LAB inputs: LAB:plan/158-standing-bounded-autonomy.md@057cff585d42c7974e865c19d33ea0555aa917d5:df6e0a6be32f955d003a073803c635dd461e2d857dd5a743c18f040f96bb2ced, LAB:samples/current-l2/README.md@057cff585d42c7974e865c19d33ea0555aa917d5:c5301214b29162a1c2e60224f36b325a8e9fd289ed9121bd6df60dd4d48a5eb8, LAB:samples/current-l2/e1-place-atomic-cut.txt@057cff585d42c7974e865c19d33ea0555aa917d5:69a24964ad6d15aee3427aaefb6b0096ab14e7b0f75a4135ab758fb00fce0dd8, LAB:samples/current-l2/e2-try-fallback.txt@057cff585d42c7974e865c19d33ea0555aa917d5:c60617c7f59aa0d2ce1da2b8d1ee0d105a3b3068ce224d3328133f119b7c32af, LAB:samples/current-l2/e21-try-atomic-cut-frontier.txt@057cff585d42c7974e865c19d33ea0555aa917d5:b8087d5d9c910f03178e1a478b8cb5070b1a5ab28d435c47fc7c5476154ef6a0, LAB:samples/current-l2/e22-try-atomic-cut-place-mismatch.txt@057cff585d42c7974e865c19d33ea0555aa917d5:a40e646902ddc0d51473bf56c81b2b0c912f25f02f8316e52451d45eb990e453
Permitted LAB locations: plan, samples/current-l2
Reserved surfaces: excluded

## Pre-registered working question

Question: Does the unchanged LAB `runtime_try_cut_cluster` formal-hook route distinguish evidence for Canon theory/04's same-Place `atomic_cut` rollback frontier, or does it attach `rollback_cut_non_interference` when an input merely has either `rollback` or `atomic-cut` among its event kinds? The question is about attribution adequacy of the existing LAB hook only, not whether OBL-027 is true, proved, discharged, or ready for a carrier.
Status quo: Canon OBL-027 says rollback cannot cross `atomic_cut`, with the frontier fixed per Place. The existing LAB sample dashboard labels the four selected cases as a runtime try/cut cluster and names a `rollback_cut_non_interference` formal-hook row. This record tests whether the existing smoke artifacts independently retain the same-Place, pre/post-cut, or rollback-crossing relation needed for that label.
Alternative: The pinned existing lane may reject rollback-only, cut-only, or Place-mismatch inputs; its emitted artifacts may carry a semantic relation outside the initially inspected condition; or the source-sample runner may establish an already documented discriminating witness. In any of those outcomes, this record retains no inadequacy conclusion.
Expected falsifier: A fresh existing-lane run shows that the formal-hook route requires a co-located rollback and atomic-cut relation, rejects a rollback-only, cut-only, or Place-mismatch fixture, or emits an independently derived same-Place frontier / rollback-crossing witness rather than only a symbolic fixture and runtime-cluster reference. A command failure or a need for any new helper, schema, source fixture, carrier, or test also falsifies this bounded experiment and stops it.
Rollback / reopen trigger: If the expected falsifier occurs, set `Reliance status` to `frozen`, retain only the reproducible failure evidence, and reopen only through a narrower successor. Escalate rather than repair in place if resolving the question requires a Canon OBL-027 or theory/04 change, a BND-003 carrier choice, a helper/schema/runner change, a new evidence lane, a conformance/Gate/Phase action, or a public/runtime contract.

## Method and evidence plan

Result class: existing-lane-experiment
Commands: workdir="$(mktemp -d /tmp/mirrorea-wrk0008-formal-hook.XXXXXX)" && cargo test -p mir-semantics --test current_l2_formal_hook_support && cargo test -p mir-runtime --test current_l2_source_sample_runner && python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e1-place-atomic-cut --artifact-root "$workdir" --run-label e1 --overwrite && python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e2-try-fallback --artifact-root "$workdir" --run-label e2 --overwrite && python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e21-try-atomic-cut-frontier --artifact-root "$workdir" --run-label e21 --overwrite && python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e22-try-atomic-cut-place-mismatch --artifact-root "$workdir" --run-label e22 --overwrite && jq -S '{subject_kind, subject_ref, contract_rows}' "$workdir/formal-hooks/e1/e1-place-atomic-cut.formal-hook.json" "$workdir/formal-hooks/e2/e2-try-fallback.formal-hook.json" "$workdir/formal-hooks/e21/e21-try-atomic-cut-frontier.formal-hook.json" "$workdir/formal-hooks/e22/e22-try-atomic-cut-place-mismatch.formal-hook.json" && python3 scripts/current_l2_source_sample_regression.py regression --artifact-root "$workdir/regression" --run-label wrk0008
Non-claims: This does not identify an LAB `runtime_try_cut_cluster` or its event kinds with Canon history, locus, causal relation, rollback frontier, or OBL-027; prove or discharge OBL-027; amend theory/04, theory/11, BND-003, any SCN/Gate/Phase, or a conformance classification; select a proof/model-check carrier or a final diagnostic; modify any current-L2 helper, schema, fixture, test, CI/Make surface, runtime, parser, transport, public API, or product behavior; or claim a final formal verification route.

## Results and review

Reliance status: not-promoted
Positive evidence: Pending pre-registered existing-lane execution.
Negative evidence: Pending pre-registered existing-lane execution.
Evidence artifacts: pending
Evidence commits: none
Impact / non-effects: This record is limited to the declared existing LAB locations and disposable `/tmp` outputs. It changes no Canon theory, OBL state, carrier, helper, schema, source fixture, test, runner, conformance, Gate/Phase, runtime, or public behavior.
Independent review: not-required-for-L3

## Supersession

Supersession: none
