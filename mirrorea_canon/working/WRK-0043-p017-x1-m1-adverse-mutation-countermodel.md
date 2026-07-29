---
id: working/WRK-0043
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, meta/proposal-013, meta/proposal-017]
summary: P017 X1 の V1/R1 cross-locus read に限り、明示済み M1 adverse condition の supplied fixture tag と owner-mutation fact が同時に成立する collapse を predicate-only 有限 countermodel で検査できるかを既存 Lean LAB lane で検査する。validation/failure semantics、mutation rule、transition、carrier、runtime は選ばない。
open_items: []
---

# WRK-0043 - P017 X1 M1-adverse/mutation countermodel

## Classification and authority cut

Standing eligibility: pass

This record reads the pinned Canon cut without changing it; pre-registers one
finite `countermodel` in the existing `plan/` Lean evidence lane; names an
alternative, falsifiers, non-effects, and rollback; and introduces no helper
family, schema, CI/Make surface, or public interface.

Author: codex

Author fingerprint: not-required-for-L3

Canon anchors: adr/ADR-0014@b07ea81d8d1a2117e1e5c861d99f51508764ecf7:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, working/readme@b07ea81d8d1a2117e1e5c861d99f51508764ecf7:5a741218ca6d3a571db6686293401c417693338c379f5ac7aa5708532e599ebf, meta/proposal-013@b07ea81d8d1a2117e1e5c861d99f51508764ecf7:4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213, meta/proposal-017@b07ea81d8d1a2117e1e5c861d99f51508764ecf7:65f847f3d57cbbc5dd1f86540964fd5d9a7b6e3fcf13387c2776a08edf8254e3, theory/01-mircore-v0@b07ea81d8d1a2117e1e5c861d99f51508764ecf7:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12

LAB inputs: LAB:plan/220-c2b-c3-relation-state-proof-obligation-audit.md@b07ea81d8d1a2117e1e5c861d99f51508764ecf7:f7b96812358e57df95eacb683acc6f7c5a5028f1121bb11d970cf47f753daa6e, LAB:plan/221-c2b-c3-canon-proposal-preparation.md@b07ea81d8d1a2117e1e5c861d99f51508764ecf7:2c496830b9b914e5bcca6bc2a33c7dd529dc00400daa2023b4149700b59f3479, LAB:plan/223-p017-x1-owner-negative-mutation-candidate-selection.md@b07ea81d8d1a2117e1e5c861d99f51508764ecf7:adf001c2950762bad0f49a43f505eba9dc764c135e6855e723ba7575d7da5b75, LAB:plan/wrk-0040-p017-x1-coupled-anti-collapse-countermodel.md@b07ea81d8d1a2117e1e5c861d99f51508764ecf7:857480c7f4f26c58bb607d988eeb0fa568aecfebc6ae098ac229f94b9ae04475, LAB:plan/wrk-0041-p017-x1-owner-terminal-exclusivity-countermodel.md@b07ea81d8d1a2117e1e5c861d99f51508764ecf7:c86cf27ac586dc322d2cd991add42949fa7e3108f7a81ec9714c7beb1e70c675, LAB:plan/wrk-0042-p017-x1-owner-negative-mutation-countermodel.md@b07ea81d8d1a2117e1e5c861d99f51508764ecf7:87d181913310cf69f49a659d5d232367719267f101200a21fa4b50c18d4c4aea, LAB:docs/reports/2533-wrk0042-evidence-reader-snapshot.md@b07ea81d8d1a2117e1e5c861d99f51508764ecf7:c6ea217f7cea0ec10dbfcf0243369b62374942aed4fb19663cd80fe7ea308570

Permitted LAB locations: plan, docs/reports

Reserved surfaces: excluded

This record does not define or amend a Mir request identity or equality, Core,
Config, SaveObject, relation schema, validation algorithm, validation
acceptance/rejection or fail-closed representation, terminal branch algebra,
dynamic failure member or row, owner-mutation rule or attribution criterion,
transition/causal generator, receipt or rejection treatment, consumption
representation, authority algorithm, redaction projection, source
grammar/elaboration, runtime, transport/wire/API contract, theory/11, OBL,
scenario, Gate, Phase, implementation, or public behavior.

## Pre-registered working question

Question: Given one supplied fixture anchor `q`, one supplied finite source-name
tag `A(q, k)` for an M1 adverse condition `k`, and one supplied opaque
owner-mutation fixture fact `U(q)`, can one predicate-only finite negative
oracle distinguish the neutral, `A`-only, and `U`-only fixtures from a forbidden
simultaneous `A(q, k)` and `U(q)` fixture without deciding how an actual request
is classified, validated, rejected, terminally represented, or mutated?

Status quo: PROPOSAL-013 requires copied/replayed, stale, wrong-target,
missing-witness, and severed-lineage requests to be rejected without store
mutation. P017's `M1 and authority` row explicitly adds stale epoch/incarnation,
wrong principal/role/target, missing capability/witness, grant-policy mismatch,
severed provenance, visibility denial, and two active principals at one source
locus, all fail closed with no owner mutation. Plan 220's `X-M1` retains this
as a validation-input obligation. WRK-0042 instead detects a supplied
owner-terminal-negative / owner-mutation pair; the current Canon cut selects no
rule that identifies an M1 adverse input with an owner-terminal-negative fact.

Alternative: Even this fixture-only negative test needs an algorithm that
derives a tag from principal, epoch, incarnation, capability, witness, admission,
visibility, or history facts; a validation-acceptance/rejection or fail-closed
predicate; a mapping to owner-terminal-negative; an owner-mutation rule or
attribution criterion; a transition; storage; or another reserved
representation. In that case no source is materialized and the candidate is
frozen rather than repaired.

Expected falsifier: The neutral, `A`-only, or `U`-only fixture triggers the
overlap detector; the simultaneous fixture does not trigger it; the result
requires deriving an adverse tag, a validation result, a failure representation,
a terminal-negative mapping, a mutation rule or attribution criterion, identity,
transition, storage field, helper, schema, CI/Make surface, or public interface;
or the retained source leaves the declared finite predicate-only lane.

Rollback / reopen trigger: On any reproducible expected falsifier, immediately
set `Reliance status: frozen`, retain the exact artifact and command evidence
only in the declared LAB locations, and return an escalation bundle. Do not
repair the result by adding an M1 classifier, validation result, failure
member/row, terminal branch, mutation rule, attribution criterion, identity,
transition, persistence field, relation schema, helper, CI/Make surface, or
runtime implementation. A later positive model requires a forward successor or
the ordinary Canon process.

## Method and evidence plan

Result class: countermodel

Commands: Registration checks, run before outcome evidence: confirm this record
is committed and pushed; check the exact pinned Canon and LAB input digests; and
verify that the evidence commit changes only the declared LAB source, a direct
numbered report, and permitted operational metadata. Outcome commands, run only
after registration: extract the sole fenced Lean block from
`plan/wrk-0043-p017-x1-m1-adverse-mutation-countermodel.md` to a disposable
temporary file; compile it with `lean --trust=0`; print the axioms of every
retained theorem; scan it for `sorry`, `admit`, `unsafe`, `partial`,
`implemented_by`, `Classical`, `Choice`, `Quotient`, `Quot.sound`,
`native_decide`, and `axiom`; assert the four-row control/singleton/overlap
matrix for one finite `AdverseTag` family; scan the source for selected carrier,
validation-result, failure, transition, persistence, runtime, transport, and
API vocabulary; enforce the evidence-commit allowlist; and run
`git diff --check`.

Execution cut: `b07ea81d8d1a2117e1e5c861d99f51508764ecf7` is the authority and
input snapshot. No outcome command may run until this registration is committed
and pushed. The evidence commit may add only
`plan/wrk-0043-p017-x1-m1-adverse-mutation-countermodel.md`, a direct numbered
report, and permitted operational metadata. The source is extracted only to
disposable temporary files. It is not a stable module, data model, schema,
validator input, runtime implementation, or public interface. A later
metadata-only commit may append the exact evidence commit and artifact digest
without rewriting this pre-registration.

Non-claims: A passing finite detector establishes neither a positive X1 model
nor satisfiability, reachability, delivery, fairness, termination, validation
semantics, rejection or fail-closed behavior, typed failure semantics,
owner-mutation behavior, mutation attribution, a semantic receipt transition,
consumption representation, restore behavior, authority enforcement,
observability policy, a Core/Config/SaveObject rule, an OBL/THM result,
scenario conformance, implementation readiness, or a public claim. Fixture tags
are supplied test vocabulary only, not an exhaustive validation-failure
classification, actual request facts, Mir identities, values, failure members,
transport/session keys, source syntax, storage fields, or runtime events.

## Results and review

Reliance status: not-promoted

Positive evidence: not-run

Negative evidence: not-run

Evidence artifacts: none

Evidence commits: none

Impact / non-effects: This record is normative only about a reversible L3
research boundary and procedure. It changes no settled theory or implementation
surface and does not make any future finite result an adoption, proof, or
readiness claim.

Independent review: not-required-for-L3

## Supersession

Supersession: none
