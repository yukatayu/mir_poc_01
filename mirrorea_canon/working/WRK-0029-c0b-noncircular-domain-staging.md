---
id: working/WRK-0029
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, spec/01-lexical-and-modules, spec/02-surface-grammar, spec/03-static-semantics, theory/03-elaboration, theory/10-diagnostics, meta/proposal-004, meta/proposal-008, meta/proposal-015]
summary: Plan 200 C0-B の四つの opaque front-end role を条件付きの有限依存グラフとしてだけ置き、elaboration outcome 又は Diagnostic を input role の定義に使わない限り非循環であることを既存 LAB Markdown lane で検査する。語彙、受理域、WellScoped、outcome、Diagnostic、Core は定義・選択しない。
open_items: []
---

# WRK-0029 - C0-B noncircular domain staging

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@29ee19ead0d55a024d922d6e693ebebb07c2ae88:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, spec/01-lexical-and-modules@29ee19ead0d55a024d922d6e693ebebb07c2ae88:558337a52a04e94441bdda161d890d0faf3fa4afb2492e4dd3b090415d1bf2ed, spec/02-surface-grammar@29ee19ead0d55a024d922d6e693ebebb07c2ae88:7d97fb4e77f493c3e0be4dbffdee64f206936c1aca1d5c535c19195f81f592b1, spec/03-static-semantics@29ee19ead0d55a024d922d6e693ebebb07c2ae88:1f708b65993bd3f3b9ae96cb3752f3bfc269b746514a35e459ae034fb124b634, theory/03-elaboration@29ee19ead0d55a024d922d6e693ebebb07c2ae88:2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641, theory/10-diagnostics@29ee19ead0d55a024d922d6e693ebebb07c2ae88:3aa700a8cb1737028006e11f7574bddcfa60d9f145218ab775976857f184f5da, meta/proposal-004@29ee19ead0d55a024d922d6e693ebebb07c2ae88:9770de1a2657640a08688207b31f8bffaef63fe11f4019e5a058f5f2ac5cf1f7, meta/proposal-008@29ee19ead0d55a024d922d6e693ebebb07c2ae88:777a6b2e043ae0313c402c836341bdedf9e12758f480c44fef8391715d34f3dc, meta/proposal-015@29ee19ead0d55a024d922d6e693ebebb07c2ae88:e8b016be00bf4dd9bc8204451b7d72a871fc4fd29a88d7f4cdbb5090619f7745
LAB inputs: LAB:plan/199-selected-semantic-composition-and-inference-boundary.md@29ee19ead0d55a024d922d6e693ebebb07c2ae88:870f26ac112aacc8d4f2c010f8c88b3244b698e85efdd942b9f54a968e12103e, LAB:plan/200-reanchored-semantic-composition-research-plan.md@29ee19ead0d55a024d922d6e693ebebb07c2ae88:2811522516e127fdb63e5e84bc497745bba04751f1583eee698a2af5d7243c37, LAB:plan/wrk-0028-r0-common-cut-fact-manifest.md@2b4a89801b3d30442426926d6aff96b1d709874a:23c7668615d35f8ee82c85db8f5e73f779badeb7db57f7b94990c63a3bc8e478
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: At the pinned cut, can an existing-LAB Markdown conditional lemma use
only four opaque labels -- lexical role, parse role, surface-static role, and
BND-001 `WellScoped` input role -- plus a terminal elaboration branch, and show
that the candidate orientation
`lexical -> parse -> surface-static -> WellScoped-input -> terminal branch` is
acyclic provided no input role is defined by elaboration success, an outcome
tuple, or a Diagnostic? The lemma may state the finite graph fact under that
hypothesis only. It must not claim that Canon currently defines the four roles,
their members, their exact edges, or their implementation order.
Status quo: `spec/01` describes lexical structure, `spec/02` displays Surface
grammar, and `spec/03` names surface-visible static obligations while binding
typing/elaboration authority to theory/01--03. BND-001 takes a well-scoped
Surface item as input and then describes a tuple-or-Diagnostic branch.
WRK-0028 retains those source roles and the separate status of P004/P008/P015
at its own cut, but no Canon or LAB artifact has established an accepted
front-end domain, `WellScoped` predicate, outcome relation, or shared model.
Alternative: Retain no staging graph. Require each later design package to
introduce its full domain and relation through the ordinary Canon process, and
use no C0-B conditional observation before then.
Expected falsifier: Retaining the graph requires a concrete lexical/parse/static
member, accepted/rejected item, grammar production, parser/checker behavior,
`WellScoped` predicate, success/outcome relation, Diagnostic id/family, Core or
judgment, source precedence/reconciliation, or an edge asserted as a current
Canon rule rather than as the lemma's hypothesis. Freeze also if P004/P008/P015
detail must be treated as current rule, the source cut/digest differs, a Plan
summary is the only support for an alleged Canon statement, or a new helper,
schema, validator, CI/Make surface, evidence lane, contract, SCN, OBL, Gate,
Phase, runtime, or public claim is required.
Rollback / reopen trigger: On any falsifier set `Reliance status: frozen`,
retain only reproducible procedure evidence, and do not repair or rerun this
record. A later Canon cut or an inquiry needing a graph edge as a settled rule
requires a forward successor. Escalate rather than repair if work needs a
defined `WellScoped` predicate, outcome/Diagnostic relation, grammar/static
semantics, Core/judgment, source precedence, or any reserved surface.

## Method and evidence plan

Result class: conditional-lemma
Commands: test ! -e plan/wrk-0029-c0b-noncircular-domain-staging.md; test -s mirrorea_canon/adr/ADR-0014.md && test -s mirrorea_canon/spec/01-lexical-and-modules.md && test -s mirrorea_canon/spec/02-surface-grammar.md && test -s mirrorea_canon/spec/03-static-semantics.md && test -s mirrorea_canon/theory/03-elaboration.md && test -s mirrorea_canon/theory/10-diagnostics.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-004-surface-v0-grammar-closure.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-008-elaboration-outcome-totality-boundary.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-015-surface-fallback-and-return-closure.md; sha256sum mirrorea_canon/adr/ADR-0014.md mirrorea_canon/spec/01-lexical-and-modules.md mirrorea_canon/spec/02-surface-grammar.md mirrorea_canon/spec/03-static-semantics.md mirrorea_canon/theory/03-elaboration.md mirrorea_canon/theory/10-diagnostics.md mirrorea_canon/meta/proposals/PROPOSAL-004-surface-v0-grammar-closure.md mirrorea_canon/meta/proposals/PROPOSAL-008-elaboration-outcome-totality-boundary.md mirrorea_canon/meta/proposals/PROPOSAL-015-surface-fallback-and-return-closure.md; git diff --check
Execution cut: `29ee19ead0d55a024d922d6e693ebebb07c2ae88` is the authority/input
snapshot. Run every outcome command only after this registration is committed
and pushed. The evidence commit may add only
`plan/wrk-0029-c0b-noncircular-domain-staging.md`, its `plan/00-index.md`
entry, a direct numbered report, allowed working-record metadata/control files,
and no helper, schema, validator, CI/Make surface, parser, checker, theory,
contract, runtime, or public artifact. The result is ordinary Markdown, not a
stable schema or downstream interface. A later metadata-only commit may append
the exact evidence commit and artifact digest without rewriting this
pre-registration.
Non-claims: This does not determine, choose, define, alter, reconcile, or close
a lexical/grammar/static domain, source acceptance or rejection, `WellScoped`
predicate, Core form/judgment, elaboration outcome or equality relation,
Diagnostic family/id, request/pending/receipt/occurrence identity, replay or
persistence policy, scalar/terminal representation, SCN expectation,
OBL/theory status, Gate/Phase, lifecycle, runtime, wire, serialization, API,
or public contract. It does not treat a conditional graph orientation as a
Canon dependency, proposal detail as an integrated rule, or this result as
proof, conformance, implementation readiness, or a machine-consumed artifact.

## Results and review

Reliance status: not-promoted
Positive evidence: Pending registered commands and a source-local LAB result.
Negative evidence: Pending registered falsifier check.
Evidence artifacts: none
Evidence commits: none
Impact / non-effects: No result exists at registration. Any retained result can
be only a conditional fact about an opaque, finite directed graph. It cannot
turn the four labels into Canon sets, define `WellScoped`, or authorize a
front-end implementation or source inference.
Independent review: not-required-for-L3

## Supersession

Supersession: none
