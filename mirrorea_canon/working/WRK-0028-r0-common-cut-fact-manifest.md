---
id: working/WRK-0028
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, spec/01-lexical-and-modules, spec/02-surface-grammar, spec/03-static-semantics, theory/01-mircore-v0, theory/03-elaboration, theory/05-authority, theory/10-diagnostics, meta/proposal-004, meta/proposal-008, meta/proposal-012, meta/proposal-013, meta/proposal-015]
summary: Plan 200 の C0/C2 に必要な pre-enumerated source span を一つの current Canon cut で literal に転記し、operative Canon fact と bounded proposal direction を source 自身の authority language だけから区別できるかを検査する。意味論の合成・選択はしない。
open_items: []
---

# WRK-0028 - R0 common-cut fact manifest

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@4ee275507000b905e46c6b5389865f7c0985ab79:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, spec/01-lexical-and-modules@4ee275507000b905e46c6b5389865f7c0985ab79:558337a52a04e94441bdda161d890d0faf3fa4afb2492e4dd3b090415d1bf2ed, spec/02-surface-grammar@4ee275507000b905e46c6b5389865f7c0985ab79:7d97fb4e77f493c3e0be4dbffdee64f206936c1aca1d5c535c19195f81f592b1, spec/03-static-semantics@4ee275507000b905e46c6b5389865f7c0985ab79:1f708b65993bd3f3b9ae96cb3752f3bfc269b746514a35e459ae034fb124b634, theory/01-mircore-v0@4ee275507000b905e46c6b5389865f7c0985ab79:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/03-elaboration@4ee275507000b905e46c6b5389865f7c0985ab79:2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641, theory/05-authority@4ee275507000b905e46c6b5389865f7c0985ab79:e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4, theory/10-diagnostics@4ee275507000b905e46c6b5389865f7c0985ab79:3aa700a8cb1737028006e11f7574bddcfa60d9f145218ab775976857f184f5da, meta/proposal-004@4ee275507000b905e46c6b5389865f7c0985ab79:9770de1a2657640a08688207b31f8bffaef63fe11f4019e5a058f5f2ac5cf1f7, meta/proposal-008@4ee275507000b905e46c6b5389865f7c0985ab79:777a6b2e043ae0313c402c836341bdedf9e12758f480c44fef8391715d34f3dc, meta/proposal-012@4ee275507000b905e46c6b5389865f7c0985ab79:09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5, meta/proposal-013@4ee275507000b905e46c6b5389865f7c0985ab79:4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213, meta/proposal-015@4ee275507000b905e46c6b5389865f7c0985ab79:e8b016be00bf4dd9bc8204451b7d72a871fc4fd29a88d7f4cdbb5090619f7745
LAB inputs: LAB:plan/199-selected-semantic-composition-and-inference-boundary.md@4ee275507000b905e46c6b5389865f7c0985ab79:df114fcd17b82c482ce234b35635c64509f15abe184ad52870f7017ba9fb31cf, LAB:plan/200-reanchored-semantic-composition-research-plan.md@4ee275507000b905e46c6b5389865f7c0985ab79:fa069a161f567287545ebfd139523b62c4d8e9e36e4ec39cf1a51f2ad4319284
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: At the pinned current Canon cut, can the pre-enumerated C0/C2 source
spans be copied into one LAB Markdown manifest and classified solely from each
source's own status and explicit authority/non-effect wording as either (a) an
operative current Canon fact or (b) a Canon-recorded bounded proposal direction,
while retaining source identity, section, cut, digest, qualifiers, and an
explicit non-claim? The manifest must not resolve a source conflict, normalize
terminology, assert cross-source equivalence, or supply omitted semantics.
Status quo: Plans 199/200 separate C0 and C2 because the current grammar,
elaboration, authority, diagnostic, and proposal texts expose different
authority classes and unresolved interfaces. They are LAB navigation only;
there is no current-cut, source-local fact manifest for the pre-enumerated C0/C2
spans. Historical WRK results are intentionally outside this record and must be
pinned separately by any later package that needs them.
Alternative: Do not consolidate a manifest. Retain disjoint source-local
inventories, require each later WRK to pin its original sources independently,
and treat any attempted consolidation as an authority-normalization risk.
Expected falsifier: A pre-enumerated row cannot be classified from its literal
source status and authority/non-effect clauses; preserving its qualifier needs
semantic interpretation or a synthesized proposition; an anchor/input digest or
source span differs; a Plan summary is the only support for an alleged Canon
fact; or completion needs a precedence/classification rule, new schema/helper/
validator/CI/evidence lane, Core/grammar/diagnostic/identity decision, OBL,
SCN, Gate, Phase, runtime, contract, or public claim.
Rollback / reopen trigger: On any falsifier set `Reliance status: frozen`,
retain only reproducible procedure evidence, and do not repair or rerun this
record. A later Canon cut makes a retained manifest historical; a new claim of
currentness requires a forward successor. Escalate rather than repair if work
needs a semantic reconciliation, source precedence, stable artifact contract,
or any reserved surface.

## Method and evidence plan

Result class: literal-transcription
Commands: test ! -e plan/wrk-0028-r0-common-cut-fact-manifest.md; test -s mirrorea_canon/adr/ADR-0014.md && test -s mirrorea_canon/spec/01-lexical-and-modules.md && test -s mirrorea_canon/spec/02-surface-grammar.md && test -s mirrorea_canon/spec/03-static-semantics.md && test -s mirrorea_canon/theory/01-mircore-v0.md && test -s mirrorea_canon/theory/03-elaboration.md && test -s mirrorea_canon/theory/05-authority.md && test -s mirrorea_canon/theory/10-diagnostics.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-004-surface-v0-grammar-closure.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-008-elaboration-outcome-totality-boundary.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-013-post-admission-request-validation-context.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-015-surface-fallback-and-return-closure.md; sha256sum mirrorea_canon/adr/ADR-0014.md mirrorea_canon/spec/01-lexical-and-modules.md mirrorea_canon/spec/02-surface-grammar.md mirrorea_canon/spec/03-static-semantics.md mirrorea_canon/theory/01-mircore-v0.md mirrorea_canon/theory/03-elaboration.md mirrorea_canon/theory/05-authority.md mirrorea_canon/theory/10-diagnostics.md mirrorea_canon/meta/proposals/PROPOSAL-004-surface-v0-grammar-closure.md mirrorea_canon/meta/proposals/PROPOSAL-008-elaboration-outcome-totality-boundary.md mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md mirrorea_canon/meta/proposals/PROPOSAL-013-post-admission-request-validation-context.md mirrorea_canon/meta/proposals/PROPOSAL-015-surface-fallback-and-return-closure.md; git diff --check
Execution cut: `4ee275507000b905e46c6b5389865f7c0985ab79` is the authority/input
snapshot. Run every outcome command only after this registration is committed
and pushed. The evidence commit may add only
`plan/wrk-0028-r0-common-cut-fact-manifest.md`, its `plan/00-index.md` entry,
a direct numbered report, allowed working-record metadata/control files, and
no helper, schema, validator, CI/Make surface, parser, checker, theory,
contract, runtime, or public artifact. The manifest is ordinary Markdown, not
a stable schema or downstream interface. A later metadata-only commit may append
the exact evidence commit and artifact digest without rewriting this
pre-registration.
Non-claims: This does not determine, choose, alter, reconcile, or close a
lexical/grammar/static domain, Core form/judgment, Diagnostic family, outcome
or equality relation, request/pending/receipt/occurrence identity, replay or
persistence policy, scalar/terminal representation, SCN expectation, OBL/theory
status, Gate/Phase, lifecycle, runtime, wire, serialization, API, or public
contract. It does not treat proposal dispositions as integrated rules, current
WRK records as historical semantic evidence, common terminology as equivalence,
or this manifest as proof, conformance, implementation readiness, or a
machine-consumed artifact.

## Results and review

Reliance status: not-promoted
Positive evidence: After registration
`b1ef315040bc37f499526b70d18de7b7bcb60983` was pushed, the registered
pre-source marker and all 13 source-existence checks passed. Every registered
SHA-256 matched the pinned authority anchors. `make docs` passed with Canon
index 115/115, source hierarchy 750/750, and 1608 numbered reports. The
retained manifest classifies each pre-enumerated C0/C2 span source-locally and
preserves its stated qualifier/non-effect.
Negative evidence: No registered falsifier occurred. No row required a
precedence rule, semantic reconciliation, synthesized proposition, new schema,
helper, validator, CI/Make surface, or reserved decision. This is only a fact
about the listed spans at the pinned cut; it does not show that C0/C2 compose.
Evidence artifacts: LAB:plan/wrk-0028-r0-common-cut-fact-manifest.md@2b4a89801b3d30442426926d6aff96b1d709874a:23c7668615d35f8ee82c85db8f5e73f779badeb7db57f7b94990c63a3bc8e478
Evidence commits: 2b4a89801b3d30442426926d6aff96b1d709874a
Impact / non-effects: The retained manifest is a provenance aid only. It
distinguishes current source wording from bounded proposal directions so later
records can pin their own source-local questions. It selects no C0 domain,
Diagnostic abstraction, equality/identity/replay carrier, scalar form, shared
model, theory/11 status, Gate/Phase, implementation, or public behavior.
Independent review: not-required-for-L3

## Supersession

Supersession: none
