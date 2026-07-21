---
id: working/WRK-0007
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/01-mircore-v0, theory/03-elaboration, theory/11-metatheory-ledger]
summary: OBL-001 LAB statement draft が実験用 Result 内の write を GeneratedWrite で尽くすことを要求するか、Core 表現を選ばず既存 Lean lane で検査する L3 record。OBL status、Core IR、THM-001 は変更しない。
open_items: []
---

# WRK-0007 - OBL-001 result/write coverage boundary

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@c6ab70f8913a665623a05984c0e4240b78193cb3:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/01-mircore-v0@c6ab70f8913a665623a05984c0e4240b78193cb3:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/03-elaboration@c6ab70f8913a665623a05984c0e4240b78193cb3:2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641, theory/11-metatheory-ledger@c6ab70f8913a665623a05984c0e4240b78193cb3:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1
LAB inputs: LAB:samples/lean/lab-statements/obl001/THM001StatementDraft.lean@c6ab70f8913a665623a05984c0e4240b78193cb3:60b53c6b7e826c2d02278b10fb1e895185c98fc3a54d6d0f27b7e8581513e4b0, LAB:samples/lean/lab-statements/obl001/THM001StatementDraft.md@c6ab70f8913a665623a05984c0e4240b78193cb3:9eb851918a0d5fa7baaef4967a93cc79f5e2ab83490995ff23b10243ff0e282b, LAB:plan/124-g1-obl001-boundary-audit.md@c6ab70f8913a665623a05984c0e4240b78193cb3:1997cac0d468427b7e0a4870350708c06c8f34f6d6be2e806bcb7c3c9229123c, LAB:plan/156-t0-t2-research-autonomy-envelope.md@c6ab70f8913a665623a05984c0e4240b78193cb3:dec0742d9ef984441b6b0b35036dcd69e0a597a7b772988359c608e80f21bab0, LAB:plan/162-post-wrk0006-candidate-selection.md@c6ab70f8913a665623a05984c0e4240b78193cb3:ddaab7d7d90b5aac7bec4c82623098107940a9bad4161a2b9ec2adf6c67e969b
Permitted LAB locations: plan, samples/lean
Reserved surfaces: excluded

## Pre-registered working question

Question: Does the unchanged LAB `THM001StatementDraft` require every write represented in a successful experiment-local Result carrier to satisfy `GeneratedWrite`, or can a successful result have an experiment-local untracked cross-write while the draft still holds? The experiment-local membership relation is an instrument only; it neither defines Canon Core nor selects an eventual Core/result bridge.
Status quo: Canon THM-001 quantifies over every write in `c`, while the LAB draft quantifies soundness only after `GeneratedWrite result write`. The draft's opaque `Result` carrier has no write-membership relation and no bridge from all represented writes to `GeneratedWrite`. T-RESEARCH-001 separately showed that a write already classified by `GeneratedWrite` may lack justification when predicates are unconstrained; it did not test an unclassified write outside that guarded domain. `plan/124` found its hooks sufficient for three fixture mappings and allows reopening if a concrete missing abstraction is found.
Alternative: Existing LAB definitions may already entail a result/write-to-`GeneratedWrite` bridge, an existing OBL-001 record may already contain this same model class, or the planned finite source may fail to preserve the unchanged draft while showing an untracked write. In any such outcome, no result/write coverage conclusion is retained.
Expected falsifier: source search finds a prior successful-result plus untracked-write countermodel with `GeneratedWrite = false` and the unchanged draft true; source inspection finds an existing bridge from every represented write to `GeneratedWrite`; the new source cannot compile; or its countermodel requires a canonical Core AST, equality, result carrier, request semantics, or a new helper/runner/schema.
Rollback / reopen trigger: On any falsifier, set Reliance status to frozen, retain the failure evidence, and supersede only with a narrower L3 question. Escalate rather than repair in place if resolving the result needs a Canon Core/result representation, a Canon enumeration invariant, a THM-001/BND-001/theory/11 change, an OBL status action, SCN/Gate/Phase action, or a public/runtime contract.

## Method and evidence plan

Result class: countermodel
Commands: lean --version; lean samples/lean/lab-statements/obl001/THM001StatementDraft.lean; test ! -e samples/lean/lab-statements/obl001/ResultWriteCoverageCountermodel.lean; mkdir -p /tmp/mirrorea-wrk0007-olean; lake env lean -o /tmp/mirrorea-wrk0007-olean/THM001StatementDraft.olean samples/lean/lab-statements/obl001/THM001StatementDraft.lean; LEAN_PATH=/tmp/mirrorea-wrk0007-olean lean samples/lean/lab-statements/obl001/ResultWriteCoverageCountermodel.lean; rg -q 'statement_draft_holds' samples/lean/lab-statements/obl001/ResultWriteCoverageCountermodel.lean; rg -q 'result_write_coverage_fails' samples/lean/lab-statements/obl001/ResultWriteCoverageCountermodel.lean; ! rg -n 'sorry|admit|axiom|unsafe|partial|implemented_by|opaque' samples/lean/lab-statements/obl001/ResultWriteCoverageCountermodel.lean; python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
Non-claims: This does not identify an experiment-local Result, Write, result/write membership relation, or untracked-cross label with Canon Core `c`; select a Core AST, Core IR exchange field, equality, request relation, final write enumeration, proof interface, or new BND-001 clause; amend THM-001, theory/03, theory/11, OBL-001, any Gate/Phase, SCN, or contract; reopen PROPOSAL-008; prove/discharge any OBL; change runtime, parser, transport, conformance, public API, or L2 status.

## Results and review

Reliance status: not-promoted
Positive evidence: Lean 4.29.1 compiled the unchanged OBL-001 draft and the imported countermodel. The source proves `statement_draft_holds`, a successful `untrackedCross` result, experiment-only write membership, absence of `GeneratedWrite`, and `result_write_coverage_fails`.
Negative evidence: The registered pre-source absence check passed. A source search found no prior OBL-001 countermodel with a successful experiment-local result/write membership relation outside `GeneratedWrite`. The exact flat external output location cannot resolve the imported module prefix; recompiling the unchanged draft beneath its import-relative path makes the same source compile. This is an operational path condition, not an additional semantic premise.
Evidence artifacts: LAB:plan/wrk-0007-obl001-result-write-coverage.md@8d28ed89b63595296a2acb508b884de611b7a6d6:3a94f2e70cd362a271b218cf93909c5180ec2bc9ccf7515a42a63242a4ee79f6, LAB:samples/lean/lab-statements/obl001/ResultWriteCoverageCountermodel.lean@8d28ed89b63595296a2acb508b884de611b7a6d6:b4099aba75273018bd6c9cb355478064e02099f8974fa2bf5aecd9ea3fd3b23b, LAB:samples/lean/lab-statements/obl001/ResultWriteCoverageCountermodel.md@8d28ed89b63595296a2acb508b884de611b7a6d6:c76159178bdef525f00fa598c30f1da2ffc440f4604f7391ba6f49588ccefa0e
Evidence commits: 8d28ed89b63595296a2acb508b884de611b7a6d6
Impact / non-effects: This retained L3 evidence uses only the declared `plan` and `samples/lean` lanes. It establishes a LAB statement-shape gap only; no existing Canon text, Core representation, helper, schema, runner, CI, Make surface, runtime behavior, or product behavior changes.
Independent review: not-required-for-L3

### Method clarification — 2026-07-22

The pre-registered flat external `.olean` command is preserved as historical
registration text. The imported source names the draft by its repository module
path, so reproducible replay writes the unchanged draft to the matching relative
directory under a fresh external root and supplies that root via `LEAN_PATH`.
This changes neither the question, result class, finite countermodel, nor any
Canon boundary.

## Supersession

Supersession: none
