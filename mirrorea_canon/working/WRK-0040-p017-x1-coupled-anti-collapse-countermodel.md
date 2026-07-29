---
id: working/WRK-0040
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, meta/proposal-017]
summary: P017 X1 の V1/R1 cross-locus read に限り、五つの不適切な collapse を predicate-only 有限 countermodel で検出できるかを既存 Lean LAB lane で検査する。Core、relation schema、request identity、transition、restore、runtime は選ばない。
open_items: []
---

# WRK-0040 - P017 X1 coupled anti-collapse countermodel

## Classification and authority cut

Standing eligibility: pass

This record reads the pinned Canon cut without changing it; pre-registers a
finite `countermodel` in the existing `plan/`
Lean evidence lane; names an alternative, falsifiers, non-effects, and a
rollback; and introduces no helper family, schema, CI/Make surface, or public
interface.

Author: codex

Author fingerprint: not-required-for-L3

Canon anchors: adr/ADR-0014@0da3869b1307409ae7260b360c7b1ce0a1d60c2d:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, working/readme@0da3869b1307409ae7260b360c7b1ce0a1d60c2d:5a741218ca6d3a571db6686293401c417693338c379f5ac7aa5708532e599ebf, meta/proposal-017@0da3869b1307409ae7260b360c7b1ce0a1d60c2d:65f847f3d57cbbc5dd1f86540964fd5d9a7b6e3fcf13387c2776a08edf8254e3, theory/01-mircore-v0@0da3869b1307409ae7260b360c7b1ce0a1d60c2d:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/04-ordering-and-cuts@0da3869b1307409ae7260b360c7b1ce0a1d60c2d:70bde483330d3745a8694b15cd75f447b6610513ae66cb1ad4ec1faed274a264, theory/05-authority@0da3869b1307409ae7260b360c7b1ce0a1d60c2d:e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4, theory/07-observation@0da3869b1307409ae7260b360c7b1ce0a1d60c2d:3b0ed16c0506550e33f25f2d71839cef14e545fb9f51bd7a117e2a9b41f8d239

LAB inputs: LAB:plan/217-c2b-c3-carrier-neutral-conditional-comparison.md@0da3869b1307409ae7260b360c7b1ce0a1d60c2d:e115fa5c24024de7c641b69fde76b690581ac3a310c482dce24f466d6aa80e5e, LAB:plan/220-c2b-c3-relation-state-proof-obligation-audit.md@0da3869b1307409ae7260b360c7b1ce0a1d60c2d:f7b96812358e57df95eacb683acc6f7c5a5028f1121bb11d970cf47f753daa6e, LAB:plan/221-c2b-c3-canon-proposal-preparation.md@0da3869b1307409ae7260b360c7b1ce0a1d60c2d:7cfbc5e92475ddec959b55525d46ab791895830b5eaa78c172dd2d376d36bd7e, LAB:plan/wrk-0039-c2b-c3-fiberwise-relational-presentation-experiment.md@0da3869b1307409ae7260b360c7b1ce0a1d60c2d:bfbc66cf7fea87bdebb42e0412dd9e6c9279fd8dcfd24c672ebac5150c9aa229

Permitted LAB locations: plan, docs/reports

Reserved surfaces: excluded

This record does not define or amend a Mir
request identity or equality, Core, Config, SaveObject, relation schema,
transition/causal generator, receipt-rejection treatment, consumption
representation, `Gamma`/`Delta` disposition, authority algorithm, redaction
projection, source grammar/elaboration, runtime, transport/wire/API contract,
theory/11, OBL, scenario, Gate, Phase, implementation, or public behavior.

## Pre-registered working question

Question: Given two supplied distinct in-history occurrence witnesses `q0` and
`q1` with equal incidental facts, two supplied distinct post-load witnesses
`r0` and `r1`, and supplied restore correspondences `q0 -> r0` and `q1 -> r1`
without cross-load equality, can one predicate-only finite negative oracle
distinguish a neutral control from cumulative mutants that exhibit each of the
following X1-prohibited collapses: `SEP` (actual effective state shared solely
because incidental facts are equal), `PHASE` (terminal owner service outcome
identified with requester receipt/use), `ONE` (a second distinct accepted
consumption after the corresponding load), `AUTH` (owner success/provenance
without live authoritative grounds), and `OBS` (raw result/provenance export
without a complete separately authorized theory/07 projection witness)?

Status quo: P017 X1 requires an eventual selected design to make pending
binding, distinct owner service and requester receipt/use, at-most-one accepted
consumption across an admissible restored continuation, authority/provenance,
and non-observability explicit. It deliberately leaves their carrier,
identity, transition, restore, failure, consumption, and projection
presentations unselected. WRK-0039 is earlier artifact-local presentation
evidence only; its enumerated requests, phase table, receipt roles, state
functions, and restore function are not a baseline for this record.

Alternative: At least one of the five mutants cannot be detected without
introducing a reserved positive decision, such as a new occurrence/key
constructor, transition or restore function, phase/receipt classifier,
consumption field or token, causal edge, schema, authority algorithm, or
observation projection. In that case the neutral-control detector is not an
eligible evidence artifact for this scope.

Expected falsifier: the neutral control triggers any detector; any declared
mutant does not trigger its added detector; the cumulative final mutant fails
to trigger all five detectors; the result requires cross-load equality,
rejection treatment, a field/function/constructor, or another fact that would
settle a reserved Canon surface; or the retained source leaves the declared
finite predicate-only lane.

Rollback / reopen trigger: On any reproducible expected falsifier, immediately
set `Reliance status: frozen`, retain the exact artifact and command evidence
only in the declared LAB locations, and return an escalation bundle. Do not
repair the result by adding a phase enum, request/correlation token, receipt
classifier, consumption flag, restore function, causal edge, Config/SaveObject
field, raw export, failure row, helper, schema, CI/Make surface, or runtime
implementation. A later positive model requires a forward successor or the
ordinary Canon process.

## Method and evidence plan

Result class: countermodel

Commands: Registration checks, run before outcome evidence: confirm this record
is committed and pushed; check the exact pinned Canon and LAB input digests;
and verify that the evidence commit changes only the declared LAB source, a
direct numbered report, and permitted operational metadata. Outcome commands,
run only after registration: extract the sole fenced Lean block from
`plan/wrk-0040-p017-x1-coupled-anti-collapse-countermodel.md` to a disposable
temporary file; compile it with `lean --trust=0`; print the axioms of every
retained theorem; scan it for `sorry`, `admit`, `unsafe`, `partial`,
`implemented_by`, `Classical`, `Choice`, `Quotient`, `Quot.sound`,
`native_decide`, and `axiom`; assert the six-row control/mutant matrix and its
five detector columns; scan the source for prohibited schema, transition,
identity, transport, runtime, and raw-observation vocabulary; enforce the
evidence-commit allowlist; and run `git diff --check`.

Execution cut: `0da3869b1307409ae7260b360c7b1ce0a1d60c2d` is the authority and
input snapshot. No outcome command may run until this registration is committed
and pushed. The evidence commit may add only
`plan/wrk-0040-p017-x1-coupled-anti-collapse-countermodel.md`, a direct
numbered report, and permitted operational metadata. The source is extracted
only to disposable temporary files. It is not a stable module, data model,
schema, validator input, runtime implementation, or public interface. A later
metadata-only commit may append the exact evidence commit and artifact digest
without rewriting this pre-registration.

Non-claims: A passing finite detector establishes neither a positive X1 model
nor satisfiability, reachability, delivery/fairness/exactly-once, a semantic
receipt transition, a consumption representation, correct restore behavior,
authority enforcement, observability policy, a Core/Config/SaveObject rule,
an OBL/THM result, scenario conformance, implementation readiness, or a public
claim. Fixture labels are supplied witnesses, not Mir identities, values,
transport/session keys, source syntax, or storage fields. Receipt rejection is
outside the experiment and remains unselected.

## Results and review

Reliance status: not-promoted

Positive evidence: none; execution has not started.

Negative evidence: none; execution has not started.

Evidence artifacts: none

Evidence commits: none

Impact / non-effects: This record is normative only about a reversible L3
research boundary and procedure. It changes no settled theory or implementation
surface and does not make any future positive finite result an adoption,
proof, or readiness claim.

Independent review: not-required-for-L3

An advisory temporary Oracle review was consulted before registration; it is
not normative evidence and is summarized only in the LAB report.

## Supersession

Supersession: none
