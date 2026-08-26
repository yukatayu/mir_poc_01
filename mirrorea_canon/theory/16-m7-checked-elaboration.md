---
id: theory/16-m7-checked-elaboration
status: L1-fixed
maturity: draft
depends_on: [theory/03-elaboration, theory/10-diagnostics, theory/13-evaluation-materialization, theory/14-maintained-relation-projection, theory/15-shared-formal-model, adr/ADR-0022, adr/ADR-0029]
summary: M6 fixed source classificationを有限検査し、designated consumeを含むdeterministic checked elaboration・義務・residualを返すM7境界。
open_items: []
---

# 16 — M7 checked elaboration

This chapter defines the M7 finite refinement boundary. It consumes and retains
the complete M6 `SurfaceV0Classification` from ADR-0021 after M6's broad
ordered-token collector accepts the source; it neither re-parses a new grammar,
narrows M6 expression acceptance, nor changes an M6 classification result. In
the bounded implementation the exact accepted classification is moved into the
checked artifact and published by `consumed_m6_classification()`: root/source
`SourceRef`s, templates, and source-to-Core entries are not replaced by a
kind/name summary. The M6 collector retains every `M6ExprToken` (`Ident`,
integer, `{ } [ ] ( ) : , . = + -`) with spans; M7 is the first boundary that
may issue `UnsupportedExpression` for a non-finite tree.

## 1. Pipeline and outcomes

For selected source `s`, M6 classification is `C₆(s)`. M7 is the total finite
function:

```text
check_and_elaborate_surface_v0(s) =
  CheckedElaboration | SurfaceV0PipelineDiagnostics
```

An M6 diagnostic is forwarded as an `M7DiagnosticKind` at exactly its M6
source span. Otherwise M7 checks the finite declaration namespace, declared
state fields/index/subject/locus types, finite ordered expression-tree shape
and `Int` arithmetic, bounded expression consistency, and generated
failure-row coverage before constructing a checked artifact. The
finite new diagnostics include:

```text
DuplicateDeclaration
UnknownStateField
GeneratedFailureNotDeclared
TypeMismatch | UndefinedStateIndexType | UndefinedStateFieldType |
UndefinedRelationSubjectType | UndefinedOwnerLocus | UndefinedConsumerLocus
UndefinedSelfPrincipal | UndefinedRoleEvaluationLocus | DuplicateStateField |
DuplicateEvent | DuplicateRelation | DuplicateDesignated | DuplicateDeferred
UndefinedDesignatedResultConsumerLocus | CompetingDesignatedResultConsumer
UnsupportedExpression | ArithmeticRequiresInt
ResidualCannotExecute       only for an explicit admission demand
```

All static diagnostics reject elaboration and have no executable Core. A
duplicate identifies the second/offending declaration span.
`ResidualCannotExecute` does not make the original static artifact erroneous;
it rejects a later request to treat unresolved residual evidence as executable.

## 2. Checked artifact and generated obligations

The checked artifact carries the accepted M6 classification, canonical source
spans, and the M6 source-to-Core map without rewriting them. Its map is total over the checked assignment,
relation, designated decision, designated consume, auth, and verify spans; each entry has a stable ordinal
and Core reference. For every admitted finite evaluation it carries:

```text
EvaluationAxes = ⟨M3 SemanticForm, M3 EvaluationSite, M3 TriggerClock,
                  M3 AuthorityOrigin, M3 Materialization⟩
EffectRow      = enumerable source-spanned EffectEntry rows
                 (owner request/local read/write | relation publish |
                  designated request/receipt-use/value publish |
                  designated result delivery/consume)
CheckedCore    = owner-RMW | maintained relation | designated result projection |
                 designated result consume
```

For every admitted finite template it may also carry:

```text
GeneratedObligation ::= Failure(f) | Capability | Witness | Authority |
                        AdmittedEvaluatorAuthority |
                        DesignatedResultConsumerAuthority |
                        Evaluation(CheckedEvaluationKind)
```

An owner RMW carries an owner site, typed field target, left-associated ordered
typed expression tree (with operand/operator spans and `Int` arithmetic),
bounded summaries, same-owner reads, `store` materialization, and
request-to-owner/local-read/owner-write effects; it emits
its generated failure row plus capability, witness, and `OwnerRmw` evaluation
obligations. A maintained relation carries owner/subject/type,
binding-frontier, consumer-local projection, and primary/fallback bounded
transform options; it has `publish-relation` materialization, a
relation-publication effect, and authority/`PublishRelation` obligations. A
designated result carries evaluator/logical tick/input/result frontier/version,
bounded ordered expression tree, `publish-value` materialization,
request/receipt-use/value-publication effects,
and its finite generated M7 request/receipt-use input dependency. The
dependency retains designated evaluator, requester site, authority origin,
source-owner locus, typed state read, request, and receipt use; it is not an
M5 request edge or transport receipt delivery. It has M3 `SemanticForm::Value`,
`InputFrontier(F)` distinct from its result frontier, a deterministic
`EvaluationPolicy`, a conservative `ObservationPolicy`, and their immutable
`PolicyStamp`. It emits the `AdmittedEvaluatorAuthority` and
`DesignatedPublishValue` obligations. These are obligations/effect plans, not
authority grants, runtime steps, or execution results.

The source clause `designated consume E.result at C` elaborates separately to
one source-bound `DesignatedResultConsume` Core edge from the existing
designated result to the explicitly declared consumer locus. It retains the
producer result identity, input/result frontiers, result version, observation
policy, policy stamp, and `ReturnExistingNoNewConsumption` retry contract. It
does not carry the evaluator expression or raw input. The same named consumer
may retrieve the existing decided result without another semantic-consumption
row; a second consumer for the same `(E, result)` is
`CompetingDesignatedResultConsumer`. The consumer is never inferred from
topology, schedule, a relation, or runtime metadata.

This is a new static refinement requirement for SYS-4, not an existing M8
implementation fact. Legacy M8 rejects the same delivery id with
`AlreadyConsumed` and may consume a distinct id; accepted M10 duplicate-
delivery behavior remains unchanged. SYS-3 records the semantic-consumption
identity and contract only. SYS-4 must supply the carrier-side idempotent return
or compatible wrapper and actual endpoint positive/retry/conflict evidence.

Every `EffectEntry` and `GeneratedObligation` is enumerable and retains the
canonical source span/`SourceRef`. Neither `Authority` nor
`AdmittedEvaluatorAuthority` is authority-success evidence.

The generated failure row must be a subset of the enclosing handler's
declared `fails(...)` row. In the exact finite owner case this is:

```text
StaleMembership, MissingCapability, MissingWitness, RouteUnavailable
```

No M7 rule identifies a designated result frontier/version with a relation
binding frontier. No M7 rule replaces a relation publication with a value
publication or consumer semantic mutation. The designated-consume Core is a
static source-derived delivery/consumption obligation; it does not claim that
delivery or consumption has occurred.

## 3. Residual obligations and execution boundary

```text
ResidualObligationKind ::= Visibility | RelationLifetime | FallbackValidity |
                           ValueVisibilityRedaction | AuthDeferred |
                           VerifyDeferred
```

`Visibility`, `RelationLifetime`, and `FallbackValidity` retain unsupplied
relation projection, release/redaction, lifetime, and fallback-validity
evidence. `ValueVisibilityRedaction` retains designated value label/redaction
evidence. `AuthDeferred` retains only the named required authority; it is not
a membership decision or grant.
`VerifyDeferred` retains an unsatisfied verification boundary; it is not a
proof verdict.

An artifact with residuals is a successful static record but has
`execution_is_admissible = false`; in the selected checker, a checked artifact
is admissible at this static handoff only when it has a nonempty checked
evaluation and an empty residual row. Each residual is non-executable and creates
no authority, capability, effect, semantic mutation, verification verdict, or
runtime admission. The only finite outcome of explicitly demanding execution
admission in this state is `ResidualCannotExecute` at the selected residual
span. M8 consumes this distinction through the separate finite runtime
admission judgment in theory/17; it does not redefine this M7 judgment.

## 4. Determinism and source preservation statements

For the exact finite domain:

```text
check_and_elaborate_surface_v0(s) = check_and_elaborate_surface_v0(s)
```

and every checked source-to-Core entry retains the source span already held by
its M6 template. An M6 diagnostic retains its M6 span when forwarded. These
are statements for OBL-049's finite model only; they are not the general
elaboration determinism or diagnostic theorem in OBL-021/024/025.

## 5. Boundary

This chapter does not define arbitrary source parsing, general namespace/type
checking, a final Core exchange format, runtime execution/admission semantics,
queues, transport/receipt delivery semantics, M9 authorization/verification
semantics, conformance, or a public API/wire. The evidence status and exact
finite coverage are only in theory/11.
