---
id: working/WRK-0010
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, arch/02-boundary-contracts, theory/11-metatheory-ledger]
summary: existing current-L2 static gate の decision payload が formal-hook artifact に literal または明示的 lossless reference として残るかを監査する可逆な L3 record。診断意味、defect、carrier、helper/schema は選ばない。
open_items: []
---

# WRK-0010 - current-L2 static formal-hook decision attribution

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@c072aa9c7585ed456ff438e61e330839df12020f:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, arch/02-boundary-contracts@c072aa9c7585ed456ff438e61e330839df12020f:b9ae8932c10e25d4506f8395a1bbd30aaed8d22f6fd2a293f1a0bed022e39cd3, theory/11-metatheory-ledger@c072aa9c7585ed456ff438e61e330839df12020f:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1
LAB inputs: LAB:plan/158-standing-bounded-autonomy.md@c072aa9c7585ed456ff438e61e330839df12020f:df6e0a6be32f955d003a073803c635dd461e2d857dd5a743c18f040f96bb2ced, LAB:plan/168-wrk0009-e5-skeleton-identity-selection.md@c072aa9c7585ed456ff438e61e330839df12020f:72d90c554d2a71bf360095a35b0d72e68128c9e30a24c50abe5c558fb94c9682, LAB:plan/wrk-0009-e5-skeleton-identity.md@c072aa9c7585ed456ff438e61e330839df12020f:a11429333ae20ee5e8bd920ea616d310c672b92ecf8cd92b2e8d023502017fa6, LAB:samples/current-l2/README.md@c072aa9c7585ed456ff438e61e330839df12020f:c5301214b29162a1c2e60224f36b325a8e9fd289ed9121bd6df60dd4d48a5eb8, LAB:samples/current-l2/e4-malformed-lineage.txt@c072aa9c7585ed456ff438e61e330839df12020f:d38fef3daecb44b3e0fb175b4aae57ae9caee5cb9c8758d72cda62755b741af5, LAB:samples/current-l2/e5-underdeclared-lineage.txt@c072aa9c7585ed456ff438e61e330839df12020f:fb98721840001465c29e51ceb3bfd27e98c86f5543eaffa1d892c8e16c617037, LAB:samples/current-l2/e12-underdeclared-target-missing.txt@c072aa9c7585ed456ff438e61e330839df12020f:8df31f1d2415eb2f04aa7377c022ad0720249b2199671ba2cd2b95aafc43ae4a, LAB:samples/current-l2/e14-malformed-duplicate-option-declaration.txt@c072aa9c7585ed456ff438e61e330839df12020f:17bfc4cd965cd72d8ac04434f8070d52fc1d70e53bd6ed1bb5567bbf061bff9a
Permitted LAB locations: plan, samples/current-l2
Reserved surfaces: excluded

## Pre-registered working question

Question: For the existing static e4/e5/e12/e14 routes, does each emitted `fixture_static_cluster` formal-hook artifact preserve every predeclared static-gate decision field (`checker_core.static_verdict`, `checker_core.reasons`, and presence/value of `detached_noncore.reason_codes`) literally, or carry an explicit lossless reference to the exact static-gate artifact; or does it retain only obligation/identity rows? No spelling convention or inferred lookup counts as a reference.
Status quo: The documented source corpus contains malformed e4/e14 and underdeclared e5/e12 static stops. Existing formal-hook construction first requires the exact static-gate schema version and artifact kind, then a non-valid verdict, and emits fixed `canonical_normalization_law` and `no_re_promotion` rows keyed by fixture identity. The documented artifact shapes do not establish full decision-payload attribution.
Alternative: The unchanged formal-hook JSON may copy every predeclared field, provide an explicit lossless static-gate reference, preserve only a proper subset, or reject/static-classify a selected fixture differently. These are distinct outcomes; no outcome is a correctness judgment.
Expected falsifier: A fresh existing-lane run shows literal preservation of every predeclared field or an explicit lossless reference to the exact static-gate artifact. A command failure, missing documented artifact, need for a new field/helper/schema/fixture/test/runner, or any need to interpret diagnostic/proof semantics also falsifies this bounded experiment and stops it.
Rollback / reopen trigger: If the expected or operational falsifier occurs, set `Reliance status` to `frozen`, retain only reproducible evidence, and reopen through a narrower successor. Escalate rather than repair if resolution requires a diagnostic meaning, defect judgment, new field/schema/helper, carrier, theory/11 movement, OBL/Gate/Phase action, or public/conformance claim.

## Method and evidence plan

Result class: existing-lane-experiment
Commands: workdir="$(mktemp -d /tmp/mirrorea-wrk0010-static-attribution.XXXXXX)" && cargo test -p mir-semantics --test current_l2_formal_hook_support && python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e4-malformed-lineage --artifact-root "$workdir" --run-label wrk0010-e4 && python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e5-underdeclared-lineage --artifact-root "$workdir" --run-label wrk0010-e5 && python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e12-underdeclared-target-missing --artifact-root "$workdir" --run-label wrk0010-e12 && python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e14-malformed-duplicate-option-declaration --artifact-root "$workdir" --run-label wrk0010-e14 && jq -S -s '[.[] | {fixture: .fixture_context.fixture_id, verdict: .checker_core.static_verdict, reasons: .checker_core.reasons, detached_noncore_present: has("detached_noncore"), reason_codes: (if has("detached_noncore") then .detached_noncore.reason_codes else null end)}]' "$workdir/static-gates/wrk0010-e4/e4-malformed-lineage.static-gate.json" "$workdir/static-gates/wrk0010-e5/e5-underdeclared-lineage.static-gate.json" "$workdir/static-gates/wrk0010-e12/e12-underdeclared-target-missing.static-gate.json" "$workdir/static-gates/wrk0010-e14/e14-malformed-duplicate-option-declaration.static-gate.json" && jq -S -s '[.[] | {subject_kind, subject_ref, contract_rows}]' "$workdir/formal-hooks/wrk0010-e4/e4-malformed-lineage.formal-hook.json" "$workdir/formal-hooks/wrk0010-e5/e5-underdeclared-lineage.formal-hook.json" "$workdir/formal-hooks/wrk0010-e12/e12-underdeclared-target-missing.formal-hook.json" "$workdir/formal-hooks/wrk0010-e14/e14-malformed-duplicate-option-declaration.formal-hook.json" && python3 scripts/current_l2_source_sample_regression.py regression --artifact-root "$workdir/regression" --run-label wrk0010
Non-claims: This does not treat a static verdict, reason, reason code, formal-hook row, or their absence as Canon diagnostics, theorem/proof status, OBL evidence, a defect, or a required schema field. It does not select a carrier or mapping; modify any helper, schema, fixture, test, runner, CI/Make surface, runtime, parser, transport, public API, product behavior, SCN, Gate, Phase, conformance classification, theory/11, or lifecycle; or claim formal verification.

## Results and review

Reliance status: not-promoted
Positive evidence: The registered run passed 5 formal-hook support tests, the four e4/e5/e12/e14 static smokes, and the complete 23-command current-L2 regression. Static gates distinguish malformed/underdeclared verdicts, raw reasons, and `detached_noncore` presence/reason-code values, while all four hooks emit the same two obligation kinds and fixture-level references only.
Negative evidence: The full-attribution falsifier did not occur. No hook literally carries the selected decision payload or an explicit reference to the exact static-gate artifact; `static_gate_artifact` references name only the fixture ID. This does not infer a defect, diagnostic meaning, required field, mapping, or semantic consequence.
Evidence artifacts: LAB:plan/wrk-0010-static-formal-hook-decision-attribution.md@15fa586a8733d6c59f9fe23809b902d311fa9861:b59243925e9ffbdd47dcbf86e7b67fc08ac0a341b3468371b696884db888d3d0
Evidence commits: 15fa586a8733d6c59f9fe23809b902d311fa9861
Impact / non-effects: This record is limited to declared `plan` and `samples/current-l2` LAB locations plus disposable `/tmp` outputs. It changes no Canon theory, OBL state, carrier, helper, schema, source fixture, test, runner, conformance, Gate/Phase, runtime, diagnostic contract, or public behavior.
Independent review: not-required-for-L3

### Evidence addendum — 2026-07-22

The evidence commit is `15fa586a8733d6c59f9fe23809b902d311fa9861` and owns
the retained `plan/` artifact append-only. The result is a static artifact
attribution classification only. It neither requires a payload extension nor
changes Canon diagnostics, proof status, OBLs, carriers, helpers, schema,
runtime, lifecycle, or public behavior.

## Supersession

Supersession: none
