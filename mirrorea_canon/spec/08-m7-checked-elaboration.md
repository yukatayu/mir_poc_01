---
id: spec/08-m7-checked-elaboration
status: L1-fixed
maturity: draft
depends_on: [spec/03-static-semantics, spec/04-core-ir, theory/16-m7-checked-elaboration, adr/ADR-0022]
summary: M6 sourceを唯一の入力とするM7 finite check/elaboration API、生成義務、residual、非実行境界。
open_items: []
---

# 08 — M7 checked elaboration

## One input path

The only M7 path for this profile is:

```text
FixtureSource(.mir)
→ M6 parse_surface_v0
→ M6 classify_surface_v0
→ retain accepted M6 classification
→ check_and_elaborate_surface_v0
→ CheckedElaboration | SurfaceV0PipelineDiagnostics
```

The checked function does not accept report/JSON/expected-output fixtures as
a semantic shortcut. M6 retains its broad ordered expression-token collector;
the full `M6ExprToken` set is `Ident`, integer, `{ } [ ] ( ) : , . = + -`, and
each retained token has its M6 span. M7 may reject an accepted M6 expression
only through its named finite checker diagnostics. The function must move and
publish the complete accepted `SurfaceV0Classification` via
`CheckedSurfaceV0::consumed_m6_classification() -> &SurfaceV0Classification`,
not a classification summary or an AST reconstruction. It therefore preserves
the M6 root/source `SourceRef`s, templates, canonical source spans, and
source-to-Core entries. Its map is total over checked assignment, relation,
designated, auth, and verify spans, with stable ordinal/core references. It
must forward an M6 diagnostic at its original span rather than change its
meaning.

## Internal finite carrier names

The exact internal M7 names are:

```text
check_and_elaborate_surface_v0
SurfaceV0PipelineDiagnostics
M7DiagnosticKind
ResidualObligationKind
CheckedEvaluationKind
CheckedBinaryOperator
EffectEntry / EffectKind
GeneratedObligation / GeneratedObligationKind
```

They are implementation/finite-evidence names, not final public surface,
JSON, ABI, wire, or diagnostic-catalog names.

| M7 condition | Typed result | Required non-effect |
|---|---|---|
| M6 parse/classification diagnostic | corresponding forwarded `M7DiagnosticKind` | no executable Core |
| duplicate declaration | `DuplicateDeclaration` at duplicate declaration span | no executable Core |
| unresolved target field | `UnknownStateField` at field span | no executable Core |
| generated owner failure absent from `fails(...)` | `GeneratedFailureNotDeclared` at failure-row span | no executable Core |
| M6-accepted expression outside finite ordered-tree profile | `UnsupportedExpression` at full expression span | no executable Core |
| `+` or `-` with a non-`Int` operand | `ArithmeticRequiresInt` at operator span | no executable Core |
| finite expression/type/locus consistency gap | `TypeMismatch`, `UndefinedStateIndexType`, `UndefinedStateFieldType`, `UndefinedRelationSubjectType`, `UndefinedOwnerLocus`, `UndefinedConsumerLocus`, `UndefinedSelfPrincipal`, or `UndefinedRoleEvaluationLocus` | no executable Core |
| duplicate field/event/relation/designated/deferred | `DuplicateStateField` or corresponding `Duplicate…` at the second occurrence | no executable Core |
| relation publication | `PublishRelation`, optional `ConsumerLocalProjection`, `Visibility` + `RelationLifetime` + `FallbackValidity` residuals | no value publication or consumer mutation |
| designated result | `DesignatedPublishValue` + `ValueVisibilityRedaction` residual | no binding-frontier conversion |
| `with auth` / `verify` | `AuthDeferred` / `VerifyDeferred` residual | no grant, proof verdict, effect, mutation, or execution admission |
| explicit execution demand on unresolved residual artifact | `ResidualCannotExecute` | no runtime success |

For a valid owner RMW, the generated failure row is exactly the finite
`StaleMembership`, `MissingCapability`, `MissingWitness`, and
`RouteUnavailable` row, and it must be a subset of the declared row. The
artifact separately retains capability, witness, and `OwnerRmw` evaluation
obligations. Relation publication retains `Authority` plus
`PublishRelation` evaluation, and designated value publication retains
`AdmittedEvaluatorAuthority` plus `DesignatedPublishValue` evaluation. An
obligation is evidence required by a later consumer, not the associated
authority/capability/effect itself.

Every `CheckedEvaluation` has orthogonal M3 `SemanticForm`, `EvaluationSite`,
`TriggerClock`, `AuthorityOrigin`, and `Materialization` axes, plus a typed
effect row and a checked Core projection. `EffectRow.entries()` and
`GeneratedObligations.entries()` are enumerable rows; every entry retains its
canonical span and `SourceRef`. The finite owner projection preserves field
target, an M8-consumable ordered typed expression tree (including operand and
operator spans and `Int` arithmetic), bounded summaries, and same-owner reads
with request-to-owner/local-read/owner-write effects. The
relation projection preserves owner/subject/type/binding frontier/primary and
fallback transforms/consumer locus with a relation-publication effect. The
designated projection preserves evaluator, tick/input/result frontier/version,
an ordered typed expression tree, a deterministic `EvaluationPolicy`,
conservative `ObservationPolicy`, immutable `PolicyStamp`, and an M7-specific generated request-plus-receipt-use
input dependency with a value-publication effect. The dependency retains the
evaluator, requester site, authority origin, source-owner locus, typed state
read, request, and receipt use; it is not an M5 request edge. Those artifacts
are M8 input evidence, not M8 execution or transport/delivery behavior. Its
axes use `SemanticForm::Value`, and `InputFrontier(F)` is distinct from the
result frontier. `Authority` and `AdmittedEvaluatorAuthority` are obligations,
not authority-success evidence.

## Residual admission rule

`CheckedElaboration.execution_is_admissible` is false whenever unresolved
residual evidence remains. It is true only when a nonempty checked evaluation
is present and its residual row is empty. A successful static artifact is
therefore not by itself an execution verdict.
`require_execution_admission` must return `ResidualCannotExecute` for an
unresolved residual-only artifact. In
particular, neither `with auth MembershipAuth` nor `verify finite_refinement`
can satisfy membership or verification at M7.

## Finite evidence boundary

The selected source matrix contains the canonical owner/relation/designated/
deferred positive forms; M6 cross-owner/relation-publication/relation-mutation
diagnostics; M7 failure-row/duplicate/unknown-field negatives; a residual-only
static artifact; a residual-free owner artifact; and the additional finite
type/locus/declaration consistency negatives. Its deterministic behavior,
typed Core/effect/obligation rows, and stable span/source-map preservation are
OBL-049 finite evidence only.

This specification does not claim arbitrary checker completeness/decidability,
runtime admission behavior, transport/receipt semantics, M9 auth/verify
semantics, C-static conformance, final diagnostics wording/IDs, or a public
interface.
