# WRK-0031 C0-C - source-local Diagnostic reference audit

## Role and evidence boundary

This is a **LAB** result artifact for `working/WRK-0031`. It is ordinary
Markdown evidence, pinned to its owning commit after retention. Canon remains
normative. The table is a source-query transcript, not a front-end stage model,
rejection relation, Diagnostic assignment, or coverage proof.

“Reference” means only literal wording or an explicit cross-reference displayed
in the cited source span. It does not establish that the referenced item applies
to a particular input, that every input has an outcome, or that every rejection
has a Diagnostic.

## Pinned source-local observations

| Source-owned span | Literal source-local query observation | Explicit non-inference |
| --- | --- | --- |
| `spec/01-lexical-and-modules` | The Core-facing token note says Surface v0 items are rejected with `E-PARSE-005`; duplicate module paths have `E-NAME-004`, and the module path is called the diagnostic namespace root. | These literal named errors do not define a lexical stage, a rejection relation, a Diagnostic assignment, or any coverage property. |
| `spec/02-surface-grammar` | Brace-disambiguation text names rejected constructs and `E-NAME-003`/`E-PARSE-002`; notes additionally name `E-PARSE-006` and `E-DECL-001`. | The listed examples do not classify all grammar productions, establish an accepted/rejected domain, or map parse cases to a Diagnostic carrier. |
| `spec/03-static-semantics` | The chapter says its surface-visible obligations each have a diagnostic id in `spec/07`, then displays named `E-*` identifiers beside individual obligations. | The explicit `spec/07` reference is not a stage partition, totality claim, per-input assignment, or proof of named-family completeness. |
| `spec/07-diagnostics-format` | The ID-scheme table displays named `E-<FAMILY>-<###>` families and representative examples; its required-field section cites the theory/10 carrier. | A family inventory and required-format text do not prove that every reference above has a particular family member or that the carrier is final. |
| `theory/03-elaboration` | BND-001 literally says a well-scoped item produces the displayed tuple or a `Diagnostic (theory/10)`; row containment names `E-ROW-001`. | This repeats the BND-001/carrier direction already retained in WRK-0028 and supplies no exact `WellScoped` domain, outcome relation, or Diagnostic equality. |
| `theory/10-diagnostics` | The L2-working carrier names `spec/07` error IDs, rule instances, failed premises, spans, and repair fields; its blame principle calls rejection failure of a named premise/rule instance. | This repeats the generic carrier reading retained in WRK-0028. It does not determine a source-to-rule mapping, coverage, family/code allocation, ABI, or totality. |

The registered source query found literal `Diagnostic` or `E-*` material in the
listed files. The two theory rows are retained only to mark the already known
boundary; the independent documentary delta is the source-local explicit error
and cross-reference wording in `spec/01`, `spec/02`, `spec/03`, and `spec/07`.

## Retained result

At the WRK-0031 authority cut, the C0-C audit retains a source-tagged query
record of literal named-error and Diagnostic-format references. The record has
no stage-to-Diagnostic mapping and no coverage conclusion. In particular,
neither a source's named error nor its explicit reference to `spec/07`/theory/10
answers whether a particular source item is well-scoped, rejected, diagnosed,
or covered by totality.

This is non-duplicate of WRK-0028 only in the narrow sense above. WRK-0028's
BND-001 and generic-carrier rows remain the authority for their own literal
facts; this artifact does not reconcile them with specification text.

## Falsifier audit

The registered absence marker passed before this artifact was created. All eight
registered Canon inputs existed, their SHA-256 values matched WRK-0031, the
registered literal query returned source-local `Diagnostic`/`E-*` material, and
`git diff --check` passed. No result row required a stage/member/order,
reject-domain, accepted/rejected classification, Diagnostic assignment/equality/
completeness, totality/coherence, new code/family/carrier/ABI, source
reconciliation, or a new tool/interface.

Any attempt to use a row as such a mapping or coverage result triggers the
registered freeze/escalation route. A later Canon cut requires a forward
successor rather than revision of this artifact.

## Consequences and non-effects

The result helps preserve source provenance for a later ordinary Canon package:
it can see which current spans literally mention an error identifier or an
existing Diagnostic reference. It supplies no semantic solution and grants no
source-level omission or ergonomic inference. Such inference remains permitted
only after settled semantics uniquely determine the omitted fact and an
elaborated artifact retains reconstructible evidence.

No grammar, parser, checker, Core form, judgment, outcome, failure behavior,
Diagnostic carrier/family/code/ABI, OBL/theory status, SCN, Gate, Phase,
runtime, wire, API, public behavior, or implementation readiness changes.
