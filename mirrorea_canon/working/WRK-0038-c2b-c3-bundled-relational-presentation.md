---
id: working/WRK-0038
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/01-mircore-v0, theory/04-ordering-and-cuts, theory/05-authority, meta/proposal-012, meta/proposal-013]
summary: C2-B/C3 の Canon carrier を選ばず、WRK-0037 の二原子有限表に限って bundled DirectView と independently stated relation-first presentation が同じ観測・遷移を保持するかを検査する。identity、authority、persistence、source rule、runtime は除外する。
open_items: []
---

# WRK-0038 - C2-B/C3 bundled/relational presentation experiment

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@1041505a5979591414ef29e4f850e9d6ab52f28a:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, meta/proposal-012@fcf5ea613c2153667e1c4a887589fb939692c7a5:09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5, meta/proposal-013@fcf5ea613c2153667e1c4a887589fb939692c7a5:4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213, theory/01-mircore-v0@a51ab57b2df121186029dfae09a8206cee1f6702:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/04-ordering-and-cuts@a51ab57b2df121186029dfae09a8206cee1f6702:70bde483330d3745a8694b15cd75f447b6610513ae66cb1ad4ec1faed274a264, theory/05-authority@a51ab57b2df121186029dfae09a8206cee1f6702:e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4
LAB inputs: LAB:plan/210-c2b-c3-family-a-b-instantiation-audit.md@6303d2ef40dc1ba00a71bca44436b51180b470d1:4cba73fdbb245b16bf9fdd312609401518abaae6c96273923e1bf861e1548ffe, LAB:plan/212-c2b-c3-bundled-relational-presentation-comparison-selection.md@ca53a83f73976ffa613e7bbe35ee51fc62a43435:4485ec34a3c170dd2a249b18748d5fed35eed2852f68d5ba991df9a58ff9c5aa, LAB:plan/wrk-0037-c2b-c3-b-primary-opaque-anchor-experiment.md@99f468d6d5e415ed05f90b77c2b37956102fdc36:839ffda0e4c01fb1dab476598b97f658a8f85e27d8ce2547ab6a8c49e8662739
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: Can one artifact-local relation-first presentation over exactly WRK-0037's two opaque request atoms, listed incidental observations, reachable frontiers, receipt/resume transitions, failure behavior, dependency ground, and local restore be stated independently of `DirectView` and translated to and from the bundled view by total maps whose composites are inverse on reachable states and preserve every registered observation and transition result? The relation-first side must directly carry phase, checked validation/outcome, reply, receipt, failure, held disposition, result/provenance, accepted receipt, resume, dependency ground, mutation, and restore. No function or theorem denotes a Mir carrier, occurrence equality, authority, freshness, persistence, recovery, source rule, runtime behavior, or public contract.
Status quo: WRK-0037 retained one bounded explicit `DirectView` table and showed its listed incidental observation has no total left inverse recovering both opaque atoms. Plans 208--210 leave Family A relations and Family B occurrence anchors unselected. Plan 212 selects only this same-table comparison to test whether finite evidence distinguishes an explicit bundled view from an explicit relation-first view; it cannot choose either as Canon semantics.
Alternative: Retain no comparison result because an independently stated relation-first presentation cannot be written over the pinned table, is only a definitional repackaging of `DirectView`, or needs a new semantic premise. In that event, do not materialize source; return to the ordinary owner/Canon design boundary and retain `no-candidate` for this finite comparison.
Expected falsifier: A translation is partial on a reachable state; a composite is non-identity; a registered phase, validation/outcome, reply, receipt, failure, held disposition, result/provenance, accepted receipt, resume, dependency ground, mutation, restore, or acceptance/rejection result differs; the relational side recovers an endpoint from payload, M1 claims, principal, locus, span, transport/session, queue context, DAG ancestry, or other incidental observation; or it needs selected identity, authority, freshness, persistence, recovery, a nominal attempt, Core/Config/history/SaveObject/queue/wire field, source/elaboration contract, helper/schema/CI/Make surface, THM/OBL, SCN, Gate/Phase, runtime, or public claim.
Rollback / reopen trigger: On any reproducible falsifier, set `Reliance status: frozen`, retain the procedure and falsifier only in the declared LAB locations, and do not repair this record into a positive equivalence result. If the relation-first side is a definitional repackaging, close it as duplicate rather than enlarging the table. A changed authority cut, actual carrier or identity selection, source/elaboration artifact, contract, theorem/OBL, helper/schema/CI surface, runtime, or public behavior requires a forward successor or ordinary Canon escalation.

## Method and evidence plan

Result class: existing-lane-experiment
Commands: Registration check, run before this record is created: `test ! -e mirrorea_canon/working/WRK-0038-c2b-c3-bundled-relational-presentation.md`. Outcome commands, run only after this registration is committed and pushed: `test -s mirrorea_canon/adr/ADR-0014.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-013-post-admission-request-validation-context.md && test -s mirrorea_canon/theory/01-mircore-v0.md && test -s mirrorea_canon/theory/04-ordering-and-cuts.md && test -s mirrorea_canon/theory/05-authority.md && test -s plan/210-c2b-c3-family-a-b-instantiation-audit.md && test -s plan/212-c2b-c3-bundled-relational-presentation-comparison-selection.md && test -s plan/wrk-0037-c2b-c3-b-primary-opaque-anchor-experiment.md`; `sha256sum mirrorea_canon/adr/ADR-0014.md mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md mirrorea_canon/meta/proposals/PROPOSAL-013-post-admission-request-validation-context.md mirrorea_canon/theory/01-mircore-v0.md mirrorea_canon/theory/04-ordering-and-cuts.md mirrorea_canon/theory/05-authority.md plan/210-c2b-c3-family-a-b-instantiation-audit.md plan/212-c2b-c3-bundled-relational-presentation-comparison-selection.md plan/wrk-0037-c2b-c3-b-primary-opaque-anchor-experiment.md`; `rg -n -i -C 3 'bundled|relation-first|DirectView|AB-VIEW-REL|opaque request|incidental observation|inverse translation|presentation comparison' plan docs/reports mirrorea_canon/working mirrorea_canon/theory mirrorea_canon/spec mirrorea_canon/meta`; `awk 'BEGIN { in_block = 0 } /^```lean$/ { in_block = 1; next } in_block && /^```$/ { exit } in_block { print }' plan/wrk-0038-c2b-c3-bundled-relational-presentation-experiment.md > "${TMPDIR:-/tmp}/mir-wrk0038-c2b-c3-bundled-relational-presentation.lean" && lean --trust=0 "${TMPDIR:-/tmp}/mir-wrk0038-c2b-c3-bundled-relational-presentation.lean"`; `rg -n 'sorry|admit|unsafe|partial|implemented_by|Classical|Choice|Quotient|Quot\.sound|^axiom ' "${TMPDIR:-/tmp}/mir-wrk0038-c2b-c3-bundled-relational-presentation.lean" && exit 1 || true`; `git diff --check`
Execution cut: `9a11570ed2b2b55482ea833671dc649a58872084` is the authority/input snapshot. Run every outcome command only after this registration is committed and pushed. The evidence commit may add only `plan/wrk-0038-c2b-c3-bundled-relational-presentation-experiment.md`, a direct numbered report, declared control files, and no helper, schema, validator, CI/Make surface, parser, checker, theory, contract, runtime, sample, or public artifact. The fenced Lean source is materialized only to a disposable temporary file. It is not a stable module, schema, data model, validator input, or downstream interface. A later metadata-only commit may append the exact evidence commit and artifact digest without rewriting this pre-registration.
Non-claims: This does not select Family A, B, or C; define any Mir request/attempt/occurrence identity, equality, correlation, authority, freshness, persistence, or recovery; add a Core constructor/judgment, Config/history/SaveObject/IR/queue/wire field, relation object, source grammar/elaboration/omission rule, delivery/retry/fairness/timeout/cancellation behavior, implementation, API, contract, THM/OBL, SCN, Gate, Phase, conformance, lifecycle, or public claim. It is finite L3 evidence only, not a proof, family adequacy result, source-inference authorization, or implementation readiness.

## Results and review

Reliance status: not-promoted
Positive evidence: not-run
Negative evidence: not-run
Evidence artifacts: none
Evidence commits: none
Impact / non-effects: This record is normative only about a reversible L3 research boundary and procedure. Before registered outcome commands complete, it supplies no equivalence, distinction, carrier, identity, recovery, source inference, proof, or implementation result.
Independent review: not-required-for-L3

## Supersession

Supersession: none
