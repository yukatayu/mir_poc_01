---
id: working/WRK-0011
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, arch/02-boundary-contracts, theory/11-metatheory-ledger]
summary: existing current-L2 e21/e22 source route が exact final PlaceStore を直接アサートするか、fixture/direct-evaluator lane に限定されるかを literal に監査する可逆な L3 record。状態意味、同値性、defect、coverage 要求、carrier は選ばない。
open_items: []
---

# WRK-0011 - current-L2 final-store assertion directness

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@6297b9e6d60b8d4f02bd2efa744beb15648d9e53:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, arch/02-boundary-contracts@6297b9e6d60b8d4f02bd2efa744beb15648d9e53:b9ae8932c10e25d4506f8395a1bbd30aaed8d22f6fd2a293f1a0bed022e39cd3, theory/11-metatheory-ledger@6297b9e6d60b8d4f02bd2efa744beb15648d9e53:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1
LAB inputs: LAB:plan/158-standing-bounded-autonomy.md@6297b9e6d60b8d4f02bd2efa744beb15648d9e53:df6e0a6be32f955d003a073803c635dd461e2d857dd5a743c18f040f96bb2ced, LAB:plan/169-wrk0010-static-decision-attribution-selection.md@6297b9e6d60b8d4f02bd2efa744beb15648d9e53:07b52c5d04f4a3b58be959f075d82549f8d4b3da481482c0b87e4a0685a7c348, LAB:samples/current-l2/README.md@6297b9e6d60b8d4f02bd2efa744beb15648d9e53:c5301214b29162a1c2e60224f36b325a8e9fd289ed9121bd6df60dd4d48a5eb8, LAB:samples/current-l2/e21-try-atomic-cut-frontier.txt@6297b9e6d60b8d4f02bd2efa744beb15648d9e53:b8087d5d9c910f03178e1a478b8cb5070b1a5ab28d435c47fc7c5476154ef6a0, LAB:samples/current-l2/e22-try-atomic-cut-place-mismatch.txt@6297b9e6d60b8d4f02bd2efa744beb15648d9e53:a40e646902ddc0d51473bf56c81b2b0c912f25f02f8316e52451d45eb990e453
Permitted LAB locations: plan, samples/current-l2
Reserved surfaces: excluded

## Pre-registered working question

Question: At the pinned base, do existing tests that execute the source-authored current-L2 e21/e22 routes directly assert exact values of the resulting `run_report.final_place_store`; or do those source-route tests assert only static validity, evaluation/terminal outcome, trace, and formal-hook reachability while exact final `PlaceStore` maps are asserted only by the separate fixture/direct-evaluator tests? Here, "direct" means an equality assertion over the source-derived run report's exact final store. Event-sequence inference, fixture identity, prose, a host-plan dependency, and a separate direct-evaluator assertion do not count.
Status quo: `samples/current-l2/README.md` documents e21/e22 as existing source-authored runtime routes, and LAB plan 169 retains this assertion-directness audit as a reserve candidate. The documented source corpus and that selection do not themselves settle which existing test lane directly asserts the final store.
Alternative: One or both existing source-route tests may directly assert an exact final store; source-route tests may differ between e21 and e22; exact store assertions may occur only in the direct fixture/evaluator lane; or a required documented command may fail. These are assertion-provenance outcomes only.
Expected falsifier: A pinned existing source-route test directly compares an exact e21 or e22 `run_report.final_place_store`, or a documented existing source command plus its existing assertion directly binds that exact source-derived final store. Command failure, absent input, or any need for a new helper, test, fixture, schema, runner, CLI field, or other surface also falsifies this bounded audit and stops it.
Rollback / reopen trigger: If the expected or operational falsifier occurs, set `Reliance status` to `frozen`, retain only reproducible evidence, and reopen through a narrower successor. Escalate rather than repair if proceeding would require a claim about Place/history/cut/rollback meaning, source/fixture semantic equivalence, runtime correctness, defect status, required test coverage, OBL/Gate/Phase state, carrier choice, or a changed surface.

## Method and evidence plan

Result class: literal-transcription
Commands: base=6297b9e6d60b8d4f02bd2efa744beb15648d9e53 && git grep -n -E 'final_place_store|place_store|terminal_outcome|trace_audit_sink' "$base" -- crates/mir-runtime/tests/current_l2_source_lowering.rs crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs crates/mir-runtime/tests/current_l2_source_sample_runner.rs crates/mir-semantics/tests/current_l2_minimal_interpreter.rs crates/mir-runtime/src/current_l2.rs crates/mir-runtime/src/current_l2_cli.rs && cargo test -p mir-runtime --test current_l2_source_lowering current_l2_source_lowering_matches_e21_fixture_and_try_atomic_cut_frontier -- --exact && cargo test -p mir-runtime --test current_l2_source_lowering current_l2_source_lowering_matches_e22_fixture_and_nested_place_atomic_cut_mismatch -- --exact && cargo test -p mir-runtime --test current_l2_source_sample_verification_ladder verification_ladder_marks_e21_as_runtime_and_formal_hook_reached -- --exact && cargo test -p mir-runtime --test current_l2_source_sample_verification_ladder verification_ladder_marks_e22_as_runtime_and_formal_hook_reached -- --exact && cargo test -p mir-semantics --test current_l2_minimal_interpreter try_body_atomic_cut_updates_rollback_frontier_without_skipping_fallback -- --exact && cargo test -p mir-semantics --test current_l2_minimal_interpreter nested_place_atomic_cut_does_not_update_rollback_frontier -- --exact && workdir="$(mktemp -d /tmp/mirrorea-wrk0011-final-store-directness.XXXXXX)" && python3 scripts/current_l2_source_sample_regression.py regression --artifact-root "$workdir/regression" --run-label wrk0011
Non-claims: This does not treat either final store, event sequence, terminal outcome, source text, fixture, host plan, or their absence as a Canon state semantics, correct/incorrect runtime behavior, source/fixture equivalence, a defect, a coverage requirement, formal verification, OBL evidence, a diagnostic, or a required public field. It does not modify or select any helper, test, fixture, schema, runner, CLI, parser, runtime, transport, adapter, API, contract, conformance classification, SCN, Gate, Phase, theory/11 entry, lifecycle state, or product behavior.

## Results and review

Reliance status: not-promoted
Positive evidence: The registered command ran in a clean detached worktree at `fa130a499cecca20c625663e4ad20872ef192d67`. Its six named focused tests each passed, and the unchanged current-L2 regression passed all 23 commands. The pinned source-route test-file search found no `final_place_store` reference in the registered three source-route test files.
Negative evidence: The direct-source-route expected falsifier did not occur: neither named e21/e22 source-route body compares an exact `run_report.final_place_store`. The two named direct fixture/evaluator bodies instead compare exact `evaluator.state.place_store` values. This is literal assertion provenance only and not a conclusion about state meaning, correctness, sufficiency, or repair.
Evidence artifacts: LAB:plan/wrk-0011-current-l2-final-store-directness.md@7c16c8abce99f2ff23f8d34c2f849f1ef54c8da1:ae6dfe0e40586cf2b4c85e18e83b8577a7a8da84ddd3e6f8638722977d498130
Evidence commits: 7c16c8abce99f2ff23f8d34c2f849f1ef54c8da1
Impact / non-effects: This record is limited to declared `plan` and `samples/current-l2` LAB locations plus disposable `/tmp` outputs. It changes no Canon theory, Place/cut meaning, OBL state, carrier, helper, test, fixture, schema, source, runner, conformance, Gate/Phase, runtime, diagnostic contract, or public behavior.
Independent review: not-required-for-L3

### Evidence addendum — 2026-07-22

The evidence commit owns only the retained `plan/` matrix, its index entry, and
direct report operational metadata. Existing unmodified source/tests were
executed and inspected at the pinned revision as non-production machinery; they
are neither retained artifacts nor changed evidence. The result does not make a
source/fixture equivalence, state-semantics, defect, coverage, OBL, or repair
claim.

## Supersession

Supersession: none
