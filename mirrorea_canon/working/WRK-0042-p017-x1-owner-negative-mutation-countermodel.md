---
id: working/WRK-0042
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, meta/proposal-017]
summary: P017 X1 の V1/R1 cross-locus read に限り、supplied fixture の owner-terminal-negative fact と owner-mutation fact が同時に成立する collapse を predicate-only 有限 countermodel で検出できるかを既存 Lean LAB lane で検査する。failure semantics、mutation rule、transition、carrier、runtime は選ばない。
open_items: []
---

# WRK-0042 - P017 X1 owner-negative/mutation countermodel

## Classification and authority cut

Standing eligibility: pass

This record reads the pinned Canon cut without changing it; pre-registers one
finite `countermodel` in the existing `plan/` Lean evidence lane; names an
alternative, falsifiers, non-effects, and rollback; and introduces no helper
family, schema, CI/Make surface, or public interface.

Author: codex

Author fingerprint: not-required-for-L3

Canon anchors: adr/ADR-0014@5384e46094e58c8dbe2c66fa759f9f96dd6ee9fb:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, working/readme@5384e46094e58c8dbe2c66fa759f9f96dd6ee9fb:5a741218ca6d3a571db6686293401c417693338c379f5ac7aa5708532e599ebf, meta/proposal-017@5384e46094e58c8dbe2c66fa759f9f96dd6ee9fb:65f847f3d57cbbc5dd1f86540964fd5d9a7b6e3fcf13387c2776a08edf8254e3, theory/01-mircore-v0@5384e46094e58c8dbe2c66fa759f9f96dd6ee9fb:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12

LAB inputs: LAB:plan/220-c2b-c3-relation-state-proof-obligation-audit.md@5384e46094e58c8dbe2c66fa759f9f96dd6ee9fb:f7b96812358e57df95eacb683acc6f7c5a5028f1121bb11d970cf47f753daa6e, LAB:plan/221-c2b-c3-canon-proposal-preparation.md@5384e46094e58c8dbe2c66fa759f9f96dd6ee9fb:103b18d166fe0c0d1b0b8cb0c2c52aa0e8a3d4309c1f55d7dccb9fe06ec42c93, LAB:plan/222-p017-x1-owner-terminal-exclusivity-candidate-selection.md@5384e46094e58c8dbe2c66fa759f9f96dd6ee9fb:c0963bdd99a951b5709b10deb99e1f3b80084ce5d72e380f82d3be9f85bb2068, LAB:plan/wrk-0040-p017-x1-coupled-anti-collapse-countermodel.md@5384e46094e58c8dbe2c66fa759f9f96dd6ee9fb:857480c7f4f26c58bb607d988eeb0fa568aecfebc6ae098ac229f94b9ae04475, LAB:plan/wrk-0041-p017-x1-owner-terminal-exclusivity-countermodel.md@5384e46094e58c8dbe2c66fa759f9f96dd6ee9fb:c86cf27ac586dc322d2cd991add42949fa7e3108f7a81ec9714c7beb1e70c675, LAB:docs/reports/2528-wrk0041-evidence-reader-snapshot.md@5384e46094e58c8dbe2c66fa759f9f96dd6ee9fb:0f399bfd60dcc6623895b2aa8d4bec440e09202dd949b5c4c0dfd14863759f9e

Permitted LAB locations: plan, docs/reports

Reserved surfaces: excluded

This record does not define or amend a Mir request identity or equality, Core,
Config, SaveObject, relation schema, terminal branch algebra, typed owner
failure, owner-mutation rule, transition/causal generator, receipt or rejection
treatment, consumption representation, authority algorithm, redaction
projection, source grammar/elaboration, runtime, transport/wire/API contract,
theory/11, OBL, scenario, Gate, Phase, implementation, or public behavior.

## Pre-registered working question

Question: Given one supplied fixture anchor `q` and two supplied opaque
fixture facts `N(q)` and `U(q)`, respectively labelled owner-terminal-negative
and owner-mutation, can one predicate-only finite negative oracle distinguish
the neutral, `N`-only, and `U`-only fixtures from a forbidden simultaneous
`N(q)` and `U(q)` fixture without defining what produces, attributes, types,
stores, or changes either fact?

Status quo: P017 X1 requires at most one terminal owner success or typed
owner-service failure, and says that owner failure has no owner mutation. Plan
220's `X-BRANCH` repeats that no-mutation condition. WRK-0041 detects a
simultaneous owner-terminal-positive/negative pair; it intentionally has no
owner-mutation column. WRK-0040's `PHASE` detector separates owner service from
requester receipt/use; it has neither an owner-terminal-negative/mutation pair
nor this conjunction.

Alternative: Even this fixture-only negative test needs an actual failure
member, mutation attribution or identity rule, branch algebra, transition,
storage field, receipt classification, or another reserved representation. In
that case no source is materialized and the candidate is frozen rather than
repaired.

Expected falsifier: The neutral, `N`-only, or `U`-only fixture triggers the
overlap detector; the simultaneous fixture does not trigger it; the result
requires a failure member/row, mutation rule, attribution criterion, identity,
transition, storage field, receipt classifier, liveness rule, helper, schema,
CI/Make surface, or public interface; or the retained source leaves the
declared finite predicate-only lane.

Rollback / reopen trigger: On any reproducible expected falsifier, immediately
set `Reliance status: frozen`, retain the exact artifact and command evidence
only in the declared LAB locations, and return an escalation bundle. Do not
repair the result by adding a failure member/row, mutation rule, attribution
criterion, branch state, identity, transition, receipt classifier, persistence
field, relation schema, helper, CI/Make surface, or runtime implementation. A
later positive model requires a forward successor or the ordinary Canon
process.

## Method and evidence plan

Result class: countermodel

Commands: Registration checks, run before outcome evidence: confirm this
record is committed and pushed; check the exact pinned Canon and LAB input
digests; and verify that the evidence commit changes only the declared LAB
source, a direct numbered report, and permitted operational metadata. Outcome
commands, run only after registration: extract the sole fenced Lean block from
`plan/wrk-0042-p017-x1-owner-negative-mutation-countermodel.md` to a disposable
temporary file; compile it with `lean --trust=0`; print the axioms of every
retained theorem; scan it for `sorry`, `admit`, `unsafe`, `partial`,
`implemented_by`, `Classical`, `Choice`, `Quotient`, `Quot.sound`,
`native_decide`, and `axiom`; assert the four-row control/singleton/overlap
matrix; scan the source for selected carrier, row, transition, persistence,
runtime, transport, and API vocabulary; enforce the evidence-commit allowlist;
and run `git diff --check`.

Execution cut: `5384e46094e58c8dbe2c66fa759f9f96dd6ee9fb` is the authority and
input snapshot. No outcome command may run until this registration is committed
and pushed. The evidence commit may add only
`plan/wrk-0042-p017-x1-owner-negative-mutation-countermodel.md`, a direct
numbered report, and permitted operational metadata. The source is extracted
only to disposable temporary files. It is not a stable module, data model,
schema, validator input, runtime implementation, or public interface. A later
metadata-only commit may append the exact evidence commit and artifact digest
without rewriting this pre-registration.

Non-claims: A passing finite detector establishes neither a positive X1 model
nor satisfiability, reachability, delivery, fairness, termination, typed
failure semantics, owner mutation behavior, mutation attribution, a semantic
receipt transition, consumption representation, restore behavior, authority
enforcement, observability policy, a Core/Config/SaveObject rule, an OBL/THM
result, scenario conformance, implementation readiness, or a public claim.
Fixture facts are supplied labels, not Mir identities, values, failure members,
transport/session keys, source syntax, storage fields, or runtime events.

## Results and review

Reliance status: not-promoted

Positive evidence: none; this is a pre-registration.

Negative evidence: none; outcome commands have not run.

Evidence artifacts: none

Evidence commits: none

Impact / non-effects: This record is normative only about a reversible L3
research boundary and procedure. It changes no settled theory or implementation
surface and does not make any future finite result an adoption, proof, or
readiness claim.

Independent review: not-required-for-L3

An advisory temporary Oracle review identified this as the only independently
falsifiable fixture-only candidate at the post-WRK-0041 cut. It is not
normative evidence and is summarized only in the LAB report.

## Supersession

Supersession: none
