---
id: working/WRK-0032
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/01-mircore-v0, theory/04-ordering-and-cuts, theory/05-authority, spec/05-runtime-semantics, meta/proposal-012, meta/proposal-013]
summary: Plan 201 C5-PRE の ordinary-admission source span に、verdict から独立した membership/grant/witness issuance phase を示す literal wording があるかだけを source-local に記録する。A2 atomicity/facet/compatibility、occurrence identity、Core/history/runtime は選ばない。
open_items: []
---

# WRK-0032 - C5-PRE ordinary-admission issuance guard

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@4eb2634841184f7306d22e6df3cc8e7002873878:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/01-mircore-v0@4eb2634841184f7306d22e6df3cc8e7002873878:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/04-ordering-and-cuts@4eb2634841184f7306d22e6df3cc8e7002873878:70bde483330d3745a8694b15cd75f447b6610513ae66cb1ad4ec1faed274a264, theory/05-authority@4eb2634841184f7306d22e6df3cc8e7002873878:e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4, spec/05-runtime-semantics@4eb2634841184f7306d22e6df3cc8e7002873878:25749e3b171659fa59e3de6ff49126e15331ef52cf3ba5337ece4c46e72ca06c, meta/proposal-012@4eb2634841184f7306d22e6df3cc8e7002873878:09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5, meta/proposal-013@4eb2634841184f7306d22e6df3cc8e7002873878:4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213
LAB inputs: LAB:plan/199-selected-semantic-composition-and-inference-boundary.md@4eb2634841184f7306d22e6df3cc8e7002873878:ff3af0ee3da29602325bb01e8ce17c77003441b64f0b695468607b58b31fa6af, LAB:plan/200-reanchored-semantic-composition-research-plan.md@4eb2634841184f7306d22e6df3cc8e7002873878:623ea30a3c9985983b40146e23dab755ff948c3626d192a74a5175fecc29b963, LAB:plan/201-c5-a2-issuance-guard-candidate-selection.md@4eb2634841184f7306d22e6df3cc8e7002873878:2c062df09ebc58228a90c18ca22a20acdf2df7fa85174b5f0260203401d2b22d, LAB:docs/reports/2473-c3-c5-c4-portfolio-disposition.md@4eb2634841184f7306d22e6df3cc8e7002873878:96475736ea6599aae2ac44d42d6d0ab4b783f6930699331d0d832bac9ddfc8a2
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: At the pinned cut, can the five pre-enumerated ordinary-admission
source spans be recorded source-locally as naming or not naming a
membership/grant/witness issuance phase distinct from verdict/`[E-ADMIT]`
through a distinct rule, transition, state, issuance-specific failure,
queue/scheduling point, or independent observation point? The sole permitted
result is a source-tagged literal matrix. A match or non-match in one named
span must not answer atomicity, occurrence identity, facet cardinality,
compatibility, or any global absence question.
Status quo: P012 records conditional A2 only when one verdict atomically
establishes membership and a finite named grant/witness set, and requires an A1
successor assessment if a later design exposes separately failing, observable,
or schedulable issuance. theory/01 currently names ordinary `admitreq`,
`verdict`, Config `M/G/W`, and one `[E-ADMIT]` sketch; theory/04, theory/05,
and spec/05 state adjacent causal, authority, and observable behavior. Plan 186
and WRK-0028 retain broader admission/occurrence context, but no current-cut
ordinary-admission span-by-span audit of P012's issuance guard is retained.
Alternative: One enumerated span explicitly names a distinct ordinary-admission
issuance transition/state, separate failure, scheduling point, or independent
observation. Retain only that source fact and require a future ordinary
Canon/A1-successor assessment before any design relies on conditional A2.
Expected falsifier: An identical current-cut ordinary-admission issuance-guard
audit is discovered; no source-local observation remains after excluding P012's
already-retained direction; or a row needs same/different occurrence identity,
atomicity from a shared rule label or zero-or-one-step wording, singular/plural
grant or witness cardinality, an operational trace, a patch-admission fact,
or an absence claim beyond the named spans. Freeze also if the source cut or
digest changes, or if any Core, grammar, theory/11, OBL, scenario, Gate/Phase,
helper/schema/validator/CI/Make surface, runtime, wire, API, or public surface
is needed.
Rollback / reopen trigger: On any falsifier set `Reliance status: frozen`,
retain only reproducible procedure evidence, and do not repair or rerun this
record. A later Canon cut needs a forward successor for a current-source claim.
Escalate rather than repair if work needs an occurrence/history mapping,
admission transaction, A1/A2 selection, source reconciliation, or any reserved
surface.

## Method and evidence plan

Result class: literal-transcription
Commands: test ! -e plan/wrk-0032-c5pre-ordinary-admission-issuance-guard.md; test -s mirrorea_canon/adr/ADR-0014.md && test -s mirrorea_canon/theory/01-mircore-v0.md && test -s mirrorea_canon/theory/04-ordering-and-cuts.md && test -s mirrorea_canon/theory/05-authority.md && test -s mirrorea_canon/spec/05-runtime-semantics.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-013-post-admission-request-validation-context.md; sha256sum mirrorea_canon/adr/ADR-0014.md mirrorea_canon/theory/01-mircore-v0.md mirrorea_canon/theory/04-ordering-and-cuts.md mirrorea_canon/theory/05-authority.md mirrorea_canon/spec/05-runtime-semantics.md mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md mirrorea_canon/meta/proposals/PROPOSAL-013-post-admission-request-validation-context.md; rg -n -C 3 'admitreq|Admission(Request|Verdict)|\\[E-ADMIT\\]|issues grants|witness|membership|issuance|issue|failure|queue|schedul|observ' mirrorea_canon/theory/01-mircore-v0.md mirrorea_canon/theory/04-ordering-and-cuts.md mirrorea_canon/theory/05-authority.md mirrorea_canon/spec/05-runtime-semantics.md mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md; git diff --check
Execution cut: `4eb2634841184f7306d22e6df3cc8e7002873878` is the authority/input
snapshot. Run every outcome command only after this registration is committed
and pushed. The evidence commit may add only
`plan/wrk-0032-c5pre-ordinary-admission-issuance-guard.md`, its
`plan/00-index.md` entry, a direct numbered report, allowed working-record
metadata/control files, and no helper, schema, validator, CI/Make surface,
parser, checker, theory, contract, runtime, or public artifact. The result is
ordinary Markdown, not a stable schema, data model, validator input, or
downstream interface. A later metadata-only commit may append the exact
evidence commit and artifact digest without rewriting this pre-registration.
Non-claims: This does not determine, choose, define, alter, reconcile, or close
an admission protocol, verdict/membership/grant/witness occurrence mapping,
atomicity, transaction, request/attempt/occurrence identity, projection/facet
cardinality, grant/witness identifier, rejection residue, causal edge, queue,
scheduler, failure behavior, persistence, save/load/rollback behavior, Core
form/judgment, generated edge, history schema, Diagnostic, SCN, OBL/theory
status, Gate/Phase/lifecycle, parser/checker/runtime behavior, wire,
serialization, API, or public contract. It does not treat a shared rule label,
source adjacency, or a missing queried marker as a Canon conclusion, or P012's
direction as current Core semantics. It is not proof, conformance,
implementation readiness, or a machine-consumed artifact.

## Results and review

Reliance status: not-promoted
Positive evidence: pending execution after the pushed registration.
Negative evidence: pending execution after the pushed registration.
Evidence artifacts: none
Evidence commits: none
Impact / non-effects: Pending execution can retain only a source-local literal
matrix over the named ordinary-admission spans. It cannot supply an ergonomic
inference, choose A1/A2, establish atomic issuance, or alter a Canon rule.
Independent review: not-required-for-L3

## Supersession

Supersession: none
