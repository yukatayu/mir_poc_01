# MirTheoryV0M7CheckedElaboration.lean

## Summary

This is the finite M7 evidence for the one source-first boundary named
`check_and_elaborate_surface_v0`:

```text
selected `.mir` source
→ M6 broad-token parse and retained M6 classification
→ finite M7 checks
→ deterministic CheckedElaboration | SurfaceV0PipelineDiagnostics
```

It is a fresh, self-contained finite model aligned to the production internal
type names `SurfaceV0PipelineDiagnostics`, `M7DiagnosticKind`,
`ResidualObligationKind`, and `CheckedEvaluationKind`. It refines M6; it does
not parse a second grammar, narrow M6 expression acceptance, or reinterpret
M6 classifications. Its finite `consumedM6Classification` carrier models the
complete accepted production `SurfaceV0Classification` retained by
`CheckedSurfaceV0::consumed_m6_classification()`, including the
classification-owned source-to-Core map; it is not a kind/name summary.

## Exact finite coverage

- The canonical same-owner source generates the fixed failure-row,
  capability, witness, and `OwnerRmw` evaluation obligations. Its retained
  source map remains `ownerRmw` / `ownerLocalRead` / `ownerLocalWrite` at the
  canonical assignment span. Its checked Core retains a typed target,
  left-associated ordered typed expression tree, bounded `-` expression,
  same-owner reads, owner evaluation axes, and enumerable source-spanned owner
  request/local-read/owner-write effect rows.
- A maintained relation remains `publishRelation` with consumer-local
  projection mapping. Its checked Core retains owner/subject/type, binding
  frontier, consumer locus, primary `translate(3,-2)` and fallback `identity`
  options, and a relation-publication effect row. It retains `Visibility`,
  `RelationLifetime`, and `FallbackValidity` residual obligations instead of
  silently settling release, lifetime, or fallback validity.
- A designated result remains `DesignatedPublishValue`, distinct from a
  relation publication. Its checked Core retains evaluator, logical-tick
  frontier, input frontier, result frontier/version, `publish-value`
  materialization, bounded `+ 1` expression, deterministic evaluation policy,
  conservative observation policy, policy stamp, enumerable source-spanned
  request/receipt-use/value-publication effect rows, and the explicit finite
  typed M7 request/receipt-use input dependency (evaluator, requester site,
  authority origin, source-owner locus, typed state read, request, and receipt
  use). It also retains `ValueVisibilityRedaction`; this is not an M5 request
  edge or a transport/delivery claim.
- Each checked evaluation records orthogonal M3 `SemanticForm`,
  `EvaluationSite`, `TriggerClock`, `AuthorityOrigin`, and `Materialization`
  axes. The finite source map
  covers assignment, relation, designated, auth, and verify spans with stable
  ordinal/core references.
- `with auth MembershipAuth` and `verify finite_refinement` are successful
  static residual evidence only. Their residuals grant neither authority nor
  capability; emit no effect, mutation, or verdict; and make execution
  inadmissible. An explicit admission request returns
  `M7DiagnosticKind::ResidualCannotExecute` at the verification span.
- M6 diagnostic input is forwarded with its original span. The exact finite
  M7 negatives are `GeneratedFailureNotDeclared`, `DuplicateDeclaration`,
  `UnknownStateField`, `UnsupportedExpression`, `ArithmeticRequiresInt`,
  `TypeMismatch`, `UndefinedStateIndexType`,
  `UndefinedStateFieldType`, `UndefinedRelationSubjectType`,
  `UndefinedOwnerLocus`, `UndefinedConsumerLocus`, `UndefinedSelfPrincipal`,
  `UndefinedRoleEvaluationLocus`, `DuplicateStateField`, and duplicate
  event/relation/designated/deferred checks; each produces diagnostics with no
  executable Core. Duplicate diagnostics select the second/offending span.
  `Authority` and `AdmittedEvaluatorAuthority` are source-spanned generated
  obligations rather than authority-success evidence.
- The M6 broad collector first accepts the complete canonical token set
  `Ident`, integer, `{ } [ ] ( ) : , . = + -` with exact spans. The finite M7
  check then rejects the selected full-punctuation expression as
  `UnsupportedExpression`; it does not turn it into an M6 parse diagnostic.

The Lean constructor names use conventional lower camel case, while the
production Rust enum spelling is `M7DiagnosticKind::…`,
`ResidualObligationKind::…`, and `CheckedEvaluationKind::…`.

## Evidence boundary

The file proves only its exact finite source cases and the listed
deterministic/static properties. It does not prove general parser coverage,
arbitrary name or type checking, general elaboration determinism, runtime
execution/admission semantics, queue/transport/receipt behavior, M9
authorization or verification semantics, SCN conformance, a public API/wire
contract, or a final grammar.

## Compile

```bash
lean --trust=0 samples/lean/foundations/MirTheoryV0M7CheckedElaboration.lean
```

## Inventory sync

```bash
python3 scripts/current_l2_lean_sample_sync.py
```
