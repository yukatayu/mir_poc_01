---
id: working/WRK-0031
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, spec/01-lexical-and-modules, spec/02-surface-grammar, spec/03-static-semantics, spec/07-diagnostics-format, theory/03-elaboration, theory/10-diagnostics, meta/proposal-008]
summary: Plan 200 C0-C の pre-enumerated source span について、literal terminal/reject/Diagnostic wording と明示 cross-reference の有無だけを source-local に記録する。stage membership、coverage、reject domain、Diagnostic assignment/equality/completeness、totality/coherence は選ばない。
open_items: []
---

# WRK-0031 - C0-C source-local Diagnostic reference audit

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@f2da0b1ede4f437ba022865809411a02aa4a0bf0:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, spec/01-lexical-and-modules@f2da0b1ede4f437ba022865809411a02aa4a0bf0:558337a52a04e94441bdda161d890d0faf3fa4afb2492e4dd3b090415d1bf2ed, spec/02-surface-grammar@f2da0b1ede4f437ba022865809411a02aa4a0bf0:7d97fb4e77f493c3e0be4dbffdee64f206936c1aca1d5c535c19195f81f592b1, spec/03-static-semantics@f2da0b1ede4f437ba022865809411a02aa4a0bf0:1f708b65993bd3f3b9ae96cb3752f3bfc269b746514a35e459ae034fb124b634, spec/07-diagnostics-format@f2da0b1ede4f437ba022865809411a02aa4a0bf0:251a2366e0743459f28681bfc32a3cb91903b0aa0f4a4c0f300b05b62b4ef854, theory/03-elaboration@f2da0b1ede4f437ba022865809411a02aa4a0bf0:2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641, theory/10-diagnostics@f2da0b1ede4f437ba022865809411a02aa4a0bf0:3aa700a8cb1737028006e11f7574bddcfa60d9f145218ab775976857f184f5da, meta/proposal-008@f2da0b1ede4f437ba022865809411a02aa4a0bf0:777a6b2e043ae0313c402c836341bdedf9e12758f480c44fef8391715d34f3dc
LAB inputs: LAB:plan/199-selected-semantic-composition-and-inference-boundary.md@f2da0b1ede4f437ba022865809411a02aa4a0bf0:3d2c31c9bbcfe843857165d123acbb330f9f5b387864c5cb0af5f80306bb8983, LAB:plan/200-reanchored-semantic-composition-research-plan.md@f2da0b1ede4f437ba022865809411a02aa4a0bf0:4a201c96593070e530e2f7be746db867e264fa5b2fc04acc9842e709e31be0ae, LAB:docs/reports/2466-semantic-composition-next-candidate-screen.md@f2da0b1ede4f437ba022865809411a02aa4a0bf0:834c40e9e98e681b5345076f83d85da08d1a84d52b293ce3077e18918d6937e8, LAB:plan/wrk-0028-r0-common-cut-fact-manifest.md@2b4a89801b3d30442426926d6aff96b1d709874a:23c7668615d35f8ee82c85db8f5e73f779badeb7db57f7b94990c63a3bc8e478
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: At the pinned cut, can the pre-enumerated source spans in
`spec/01`, `spec/02`, `spec/03`, `theory/03`, and `theory/10` be recorded
source-locally as containing or not containing literal terminal/reject/
`Diagnostic` wording, and as naming or not naming an explicit cross-reference
to an existing Diagnostic carrier or named error family? The sole permitted
result is a source-tagged query record. A record of a literal reference or of
its absence from a named source span must not answer whether any language stage,
input, rejection, Diagnostic assignment, or coverage relation exists.
Status quo: WRK-0028 already separates BND-001's tuple-or-Diagnostic wording,
the L2-working generic Diagnostic carrier, and P008's separate-totality
direction without reconciling them. `spec/03` and `spec/07` display diagnostic
identifiers, but no retained record yet isolates which pre-enumerated span
literally points to an existing carrier/family and which does not. Plan 200's
earlier phrase “each claimed stage” is not available: stage membership and a
reject domain are unresolved.
Alternative: Retain only WRK-0028's existing source-role manifest and defer all
additional Diagnostic-reference reading until an ordinary Canon package selects
the exact domain, well-scopedness predicate, result/Diagnostic abstraction, and
obligation placement required by P008.
Expected falsifier: After excluding material already retained by WRK-0028, no
independent source-local reference/query observation remains; an entry requires
defining a lexical/parse/static/`WellScoped` stage, its members/order, a reject
relation, accepted/rejected domain, Diagnostic assignment/equality/
completeness, outcome/coherence relation, or source precedence. Freeze also if
the work needs a new error code/family/carrier/ABI, raw-source totality,
implementation exception, Core/judgment, OBL identity/ledger placement, SCN,
Gate/Phase, helper/schema/validator/CI/Make surface, runtime/wire/API/public
claim, or treats P004/P008/P015 directions as current grammar or rule.
Rollback / reopen trigger: On any falsifier set `Reliance status: frozen`, keep
only reproducible procedure evidence, and do not repair or rerun this record.
A later Canon cut needs a forward successor for a current-source claim.
Escalate rather than repair if work needs a semantic rejection/Diagnostic
relation, exact domain, totality/coherence statement, carrier/ABI, source
reconciliation, or any reserved surface.

## Method and evidence plan

Result class: literal-transcription
Commands: test ! -e plan/wrk-0031-c0c-source-local-diagnostic-reference-audit.md; test -s mirrorea_canon/adr/ADR-0014.md && test -s mirrorea_canon/spec/01-lexical-and-modules.md && test -s mirrorea_canon/spec/02-surface-grammar.md && test -s mirrorea_canon/spec/03-static-semantics.md && test -s mirrorea_canon/spec/07-diagnostics-format.md && test -s mirrorea_canon/theory/03-elaboration.md && test -s mirrorea_canon/theory/10-diagnostics.md && test -s mirrorea_canon/meta/proposals/PROPOSAL-008-elaboration-outcome-totality-boundary.md; sha256sum mirrorea_canon/adr/ADR-0014.md mirrorea_canon/spec/01-lexical-and-modules.md mirrorea_canon/spec/02-surface-grammar.md mirrorea_canon/spec/03-static-semantics.md mirrorea_canon/spec/07-diagnostics-format.md mirrorea_canon/theory/03-elaboration.md mirrorea_canon/theory/10-diagnostics.md mirrorea_canon/meta/proposals/PROPOSAL-008-elaboration-outcome-totality-boundary.md; rg -n 'Diagnostic|E-[A-Z]+-[0-9]+' mirrorea_canon/spec/01-lexical-and-modules.md mirrorea_canon/spec/02-surface-grammar.md mirrorea_canon/spec/03-static-semantics.md mirrorea_canon/spec/07-diagnostics-format.md mirrorea_canon/theory/03-elaboration.md mirrorea_canon/theory/10-diagnostics.md; git diff --check
Execution cut: `f2da0b1ede4f437ba022865809411a02aa4a0bf0` is the authority/input
snapshot. Run every outcome command only after this registration is committed
and pushed. The evidence commit may add only
`plan/wrk-0031-c0c-source-local-diagnostic-reference-audit.md`, its
`plan/00-index.md` entry, a direct numbered report, allowed working-record
metadata/control files, and no helper, schema, validator, CI/Make surface,
parser, checker, theory, contract, runtime, or public artifact. The result is
ordinary Markdown, not a stable schema, data model, validator input, or
downstream interface. A later metadata-only commit may append the exact
evidence commit and artifact digest without rewriting this pre-registration.
Non-claims: This does not determine, choose, define, alter, reconcile, or close
lexical/parse/static/`WellScoped` stages or their order/membership; accepted or
rejected source domain; an elaboration outcome or rejection relation; a
Diagnostic assignment, equality, equivalence, coverage, completeness, carrier,
family, code, ABI, repair policy, or user-facing wording; totality, coherence,
determinism, an OBL identity/status/placement, Core form/judgment, generated
edge, SCN expectation, Gate/Phase/lifecycle, parser/checker/runtime behavior,
wire/serialization/API, or public contract. It does not turn a literal source
reference into a coverage claim, a missing queried token into a Canon
prohibition, or a proposal direction into current Canon semantics. It is not
proof, conformance, implementation readiness, or a machine-consumed artifact.

## Results and review

Reliance status: not-promoted
Positive evidence: Pending registered commands and a source-tagged LAB result.
Negative evidence: Pending registered falsifier check.
Evidence artifacts: none
Evidence commits: none
Impact / non-effects: No result exists at registration. Any retained result can
be only a source-local query record of literal references/nonreferences. It
cannot define a stage, rejection, Diagnostic relation, totality, or coverage.
Independent review: not-required-for-L3

## Supersession

Supersession: none
