---
id: working/WRK-0041
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, meta/proposal-017]
summary: P017 X1 の V1/R1 cross-locus read に限り、supplied fixture の owner-side terminal positive / negative fact が同時に成立する collapse を predicate-only 有限 countermodel で検出できるかを既存 Lean LAB lane で検査する。failure row、transition、carrier、runtime は選ばない。
open_items: []
---

# WRK-0041 - P017 X1 owner-terminal exclusivity countermodel

## Classification and authority cut

Standing eligibility: pass

This record reads the pinned Canon cut without changing it; pre-registers one
finite `countermodel` in the existing `plan/` Lean evidence lane; names an
alternative, falsifiers, non-effects, and rollback; and introduces no helper
family, schema, CI/Make surface, or public interface.

Author: codex

Author fingerprint: not-required-for-L3

Canon anchors: adr/ADR-0014@187c3eacf0f45a194072f004443728e9b94f672b:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, meta/proposal-017@187c3eacf0f45a194072f004443728e9b94f672b:65f847f3d57cbbc5dd1f86540964fd5d9a7b6e3fcf13387c2776a08edf8254e3, theory/01-mircore-v0@187c3eacf0f45a194072f004443728e9b94f672b:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12

LAB inputs: LAB:plan/220-c2b-c3-relation-state-proof-obligation-audit.md@187c3eacf0f45a194072f004443728e9b94f672b:f7b96812358e57df95eacb683acc6f7c5a5028f1121bb11d970cf47f753daa6e, LAB:plan/221-c2b-c3-canon-proposal-preparation.md@187c3eacf0f45a194072f004443728e9b94f672b:841991097c698ad368c500e197f7c5cdf798f8976b02a0323bd39106f1cc1692, LAB:plan/wrk-0040-p017-x1-coupled-anti-collapse-countermodel.md@187c3eacf0f45a194072f004443728e9b94f672b:857480c7f4f26c58bb607d988eeb0fa568aecfebc6ae098ac229f94b9ae04475, LAB:docs/reports/2522-wrk0040-p017-x1-countermodel-execution.md@187c3eacf0f45a194072f004443728e9b94f672b:0b95659d9956caf0aa0aed85e5772db79d28eae263bb7866800a6f043ce2019e

Permitted LAB locations: plan, docs/reports

Reserved surfaces: excluded

This record does not define or amend a Mir request identity or equality, Core,
Config, SaveObject, relation schema, terminal branch algebra, dynamic failure
member or row, transition/causal generator, owner-mutation rule, receipt or
rejection treatment, consumption representation, authority algorithm,
redaction projection, source grammar/elaboration, runtime, transport/wire/API
contract, theory/11, OBL, scenario, Gate, Phase, implementation, or public
behavior.

## Pre-registered working question

Question: Given one supplied fixture anchor `q` and two supplied opaque
fixture facts `P(q)` and `N(q)`, respectively labelled owner-terminal-positive
and owner-terminal-negative, can one predicate-only finite negative oracle
distinguish the no-terminal-fact control, positive-only, and negative-only
fixtures from the forbidden simultaneous `P(q)` and `N(q)` fixture without
defining what generates, types, stores, or transitions either fact?

Status quo: P017 X1 requires an outstanding disposition that need not
terminate and at most one terminal owner success or typed owner-service
failure. Plan 220's `X-BRANCH` names simultaneous owner success and failure
for one `q` as a decisive falsifier. WRK-0040's `PHASE` detector instead
separates owner service from requester receipt/use; it does not test the
exclusive owner-terminal alternatives or the allowed no-terminal case.

Alternative: Even this fixture-only negative test needs an outcome enum,
branch algebra, selected dynamic failure row, transition, storage field,
receipt classification, or another reserved representation. In that case no
source is materialized and the candidate is frozen rather than repaired.

Expected falsifier: The no-terminal control or either singleton fixture
triggers the overlap detector; the simultaneous fixture does not trigger it;
the result requires an outcome constructor, failure row, transition, receipt
classifier, liveness rule, owner mutation, storage field, helper, schema,
CI/Make surface, or public interface; or the retained source leaves the
declared finite predicate-only lane.

Rollback / reopen trigger: On any reproducible expected falsifier, immediately
set `Reliance status: frozen`, retain the exact artifact and command evidence
only in the declared LAB locations, and return an escalation bundle. Do not
repair the result by adding an outcome enum, failure member/row, branch state,
transition, owner-mutation rule, receipt classifier, persistence field,
relation schema, helper, CI/Make surface, or runtime implementation. A later
positive model requires a forward successor or the ordinary Canon process.

## Method and evidence plan

Result class: countermodel

Commands: Registration checks, run before outcome evidence: confirm this
record is committed and pushed; check the exact pinned Canon and LAB input
digests; and verify that the evidence commit changes only the declared LAB
source, a direct numbered report, and permitted operational metadata. Outcome
commands, run only after registration: extract the sole fenced Lean block from
`plan/wrk-0041-p017-x1-owner-terminal-exclusivity-countermodel.md` to a
disposable temporary file; compile it with `lean --trust=0`; print the axioms
of every retained theorem; scan it for `sorry`, `admit`, `unsafe`, `partial`,
`implemented_by`, `Classical`, `Choice`, `Quotient`, `Quot.sound`,
`native_decide`, and `axiom`; assert the four-row control/singleton/overlap
matrix; scan the source for selected carrier, row, transition, persistence,
runtime, transport, and API vocabulary; enforce the evidence-commit allowlist;
and run `git diff --check`.

Execution cut: `187c3eacf0f45a194072f004443728e9b94f672b` is the authority and
input snapshot. No outcome command may run until this registration is committed
and pushed. The evidence commit may add only
`plan/wrk-0041-p017-x1-owner-terminal-exclusivity-countermodel.md`, a direct
numbered report, and permitted operational metadata. The source is extracted
only to disposable temporary files. It is not a stable module, data model,
schema, validator input, runtime implementation, or public interface. A later
metadata-only commit may append the exact evidence commit and artifact digest
without rewriting this pre-registration.

Non-claims: A passing finite detector establishes neither a positive X1 model
nor satisfiability, reachability, delivery, fairness, termination, typed
failure semantics, owner mutation behavior, a semantic receipt transition,
consumption representation, restore behavior, authority enforcement,
observability policy, a Core/Config/SaveObject rule, an OBL/THM result,
scenario conformance, implementation readiness, or a public claim. Fixture
facts are supplied labels, not Mir identities, values, failure members,
transport/session keys, source syntax, storage fields, or runtime events.

## Results and review

Reliance status: not-promoted

Positive evidence: not-run

Negative evidence: no registered outcome command has run.

Evidence artifacts: none

Evidence commits: none

Impact / non-effects: This record is normative only about a reversible L3
research boundary and procedure. It changes no settled theory or implementation
surface and does not make any future finite result an adoption, proof, or
readiness claim.

Independent review: not-required-for-L3

An advisory temporary Oracle review identified this as the only distinct
fixture-only candidate at the current cut. It is not normative evidence and is
summarized only in the LAB report.

## Supersession

Supersession: none
