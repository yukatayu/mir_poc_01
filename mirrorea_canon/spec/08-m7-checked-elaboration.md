---
id: spec/08-m7-checked-elaboration
status: L1-fixed
maturity: draft
depends_on: [spec/03-static-semantics, spec/04-core-ir, theory/16-m7-checked-elaboration, adr/ADR-0022, adr/ADR-0025, adr/ADR-0029, adr/ADR-0031]
summary: M6 sourceを唯一の入力とするM7 finite check/elaboration API、designated consume Core、provisional relation anchor locus、生成義務、residual、非実行境界。
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
designated decision, designated consume, auth, and verify spans, with stable
ordinal/core references. It
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
StaticRetryContractKind
```

They are implementation/finite-evidence names, not final public surface,
JSON, ABI, wire, or diagnostic-catalog names.

| M7 condition | Typed result | Required non-effect |
|---|---|---|
| M6 parse/classification diagnostic | corresponding forwarded `M7DiagnosticKind` | no executable Core |
| unknown/duplicate `visible observer_safe fields (...)` name | `UnknownObserverSafeField` / `DuplicateObserverSafeField` at declaration-token span | no executable Core or profile-only visibility fallback |
| duplicate declaration | `DuplicateDeclaration` at duplicate declaration span | no executable Core |
| unresolved target field | `UnknownStateField` at field span | no executable Core |
| generated owner failure absent from `fails(...)` | `GeneratedFailureNotDeclared` at failure-row span | no executable Core |
| M6-accepted expression outside finite ordered-tree profile | `UnsupportedExpression` at full expression span | no executable Core |
| `+` or `-` with a non-`Int` operand | `ArithmeticRequiresInt` at operator span | no executable Core |
| finite expression/type/locus consistency gap | `TypeMismatch`, `UndefinedStateIndexType`, `UndefinedStateFieldType`, `UndefinedRelationSubjectType`, `UndefinedOwnerLocus`, `UndefinedConsumerLocus`, `UndefinedSelfPrincipal`, or `UndefinedRoleEvaluationLocus` | no executable Core |
| explicit relation anchor names an undeclared locus | `UndefinedRelationAnchorLocus` at the anchor-locus span | no executable Core, inferred replacement locus, membership, or route |
| duplicate field/event/relation/designated/deferred | `DuplicateStateField` or corresponding `Duplicate…` at the second occurrence | no executable Core |
| relation publication | `PublishRelation`, optional `ConsumerLocalProjection`, `Visibility` + `RelationLifetime` + `FallbackValidity` residuals | no value publication or consumer mutation |
| designated result | `DesignatedPublishValue` + `ValueVisibilityRedaction` residual | no binding-frontier conversion |
| explicit `designated consume E.result at C` | distinct `DesignatedResultConsume` checked Core/effects/obligations bound to the declared producer and named consumer | no evaluator-expression copy, topology-derived consumer, authority grant, delivery, or runtime consumption |
| undeclared designated consumer / second consumer for the same result | `UndefinedDesignatedResultConsumerLocus` / `CompetingDesignatedResultConsumer` at the offending source span | no partial consume Core or inferred replacement consumer |
| `with auth` / `verify` | `AuthDeferred` / `VerifyDeferred` residual | no grant, proof verdict, effect, mutation, or execution admission |
| explicit execution demand on unresolved residual artifact | `ResidualCannotExecute` | no runtime success |
| write to explicitly observer-safe declared field | source-bound `ObserverSafePublish` effect plus `VisibilityDenied` failure entry | no provider proof, hidden observer policy, or implicit public release |
| `Role[self] at L_actor { ... at L_owner { ... } }`, including `L_actor != L_owner` | owner RMW with authority origin `L_actor` and evaluation/request site `L_owner` | locus difference alone is not a diagnostic or authority grant; target owner must equal `L_owner`, same-owner RHS reads resolve at `L_owner`, and `RouteUnavailable` remains generated |

For a valid owner RMW without an observer-safe write, the generated failure
row is exactly the finite `StaleMembership`, `MissingCapability`,
`MissingWitness`, and `RouteUnavailable` row, and it must be a subset of the
declared row. The M10 observer-safe write seam retains that base row and adds
`VisibilityDenied`; its full row must likewise be declared. The artifact
separately retains capability, witness, and `OwnerRmw` evaluation obligations.
Relation publication retains `Authority` plus
`PublishRelation` evaluation, and designated value publication retains
`AdmittedEvaluatorAuthority` plus `DesignatedPublishValue` evaluation. An
obligation is evidence required by a later consumer, not the associated
authority/capability/effect itself.

The bounded designated-consume evaluation is distinct from designated value
publication. It carries `DesignatedResultDelivery` and
`DesignatedResultConsume` effect rows, a
`DesignatedResultConsumerAuthority` requirement, and an
`Evaluation(DesignatedResultConsume)` obligation. Its typed Core preserves the
explicit evaluator/result reference, consumer locus, result version,
input/result frontiers, observation policy, policy stamp, and the finite retry
contract `ReturnExistingNoNewConsumption`. That retry returns the already
decided result to the same named consumer and emits no second semantic consume;
a competing consumer is the typed static conflict above. None of these rows is
authority-success, delivery, receipt, or runtime-execution evidence. In
particular, this new static contract does not describe current M8 behavior:
legacy M8 returns `AlreadyConsumed` for the same delivery id and can consume a
different id. SYS-4 must add the source/Core-bound carrier-side idempotent
return/wrapper and actual endpoint tests; M10 duplicate-delivery behavior stays
an unchanged regression baseline.

The bounded M10 direct consumer reads a `Role[self] at L_actor` header as an
authority origin and the nested `at L_owner` as the owner evaluation/request
site. It accepts different loci without treating the request as a grant. A
checked owner RMW instead requires the target state owner to equal `L_owner`;
its same-owner RHS reads remain local to `L_owner`, never to `L_actor`.

Every `CheckedEvaluation` has orthogonal M3 `SemanticForm`, `EvaluationSite`,
`TriggerClock`, `AuthorityOrigin`, and `Materialization` axes, plus a typed
effect row and a checked Core projection. `EffectRow.entries()` and
`GeneratedObligations.entries()` are enumerable rows; every entry retains its
canonical span and `SourceRef`. The finite owner projection preserves field
target, an M8-consumable ordered typed expression tree (including operand and
operator spans and `Int` arithmetic), bounded summaries, and same-owner reads
with request-to-owner/local-read/owner-write effects. The
relation projection preserves owner/subject/type/binding frontier/primary and
fallback transforms, optional explicit primary/fallback anchor loci and exact
anchor source refs, and consumer locus with a relation-publication effect. An
omitted anchor locus stays absent; M7 does not infer it from relation owner,
consumer, topology, or deployment. The
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

The separate designated-consume projection owns no typed evaluator expression
or raw remote input. It retains its own source/Core identity and requires the
referenced designated producer to exist. Its consumer must be an explicitly
declared locus, and exactly one consumer is admitted for a designated result in
this finite profile. M7 never chooses a consumer from logical topology,
schedule, relation membership, or deployment mapping.

The bounded M10 direct consumer may additionally retain the source declaration
profile for `visible observer_safe fields (...)`: unlisted state fields are
private. Only a write to a listed field has `ObserverSafePublish` and
`VisibilityDenied`; a private-field write has neither. This does not grant an
observer release or make visibility profile metadata outside the checked
identity.

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

The prior OBL-049 `lean-proved` source matrix contains the canonical owner/
relation/designated/deferred positive forms; M6 cross-owner/relation-
publication/relation-mutation diagnostics; M7 failure-row/duplicate/unknown-
field negatives; a residual-only static artifact; a residual-free owner
artifact; and the additional finite type/locus/declaration consistency
negatives. Its deterministic behavior, typed Core/effect/obligation rows, and
stable span/source-map preservation are OBL-049 finite Lean evidence only.

The bounded designated-consume extension is separate evidence. Its AST/M6/M7/
projection positives and falsifiers are accepted at source/evidence cut
`3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9` only as OBL-060
`runtime-monitored` static finite compiler/projector evidence. It does not
extend OBL-049's Lean theorem, prove runtime admission, or claim M8/runtime
consume behavior.

This specification does not claim arbitrary checker completeness/decidability,
runtime admission behavior, transport/receipt semantics, M9 auth/verify
semantics, C-static conformance, final diagnostics wording/IDs, or a public
interface.
