---
id: meta/proposal-025
status: L1-fixed
maturity: draft
depends_on: [meta/proposal-018, adr/ADR-0015, adr/ADR-0021, theory/15-shared-formal-model]
summary: M6 fixed inputを有限検査し、deterministic checked elaborationとresidual obligationを返すM7境界を選ぶ提案。
open_items: []
---

# PROPOSAL-025 — M7 checked elaboration

## Owner disposition

Under the owner-approved ADR-0015 program, select one source-first M7
boundary:

```text
.mir source
→ M6 broad-token parse
→ retained M6 classification
→ finite M7 check
→ check_and_elaborate_surface_v0
→ CheckedElaboration | SurfaceV0PipelineDiagnostics
```

M7 consumes and retains the complete accepted M6 `SurfaceV0Classification`
value before applying any M7 check; the checked artifact publishes that same
value through `consumed_m6_classification()`, rather than a kind/name summary
or a rebuilt classification.  This retains its root `SourceRef`, source refs,
templates, and source-to-Core map.  It does not parse a second grammar, narrow
M6's broad ordered expression-token collector, change a source classification,
infer a hidden receipt, or replace the M6 classification with an M5/Core
success shortcut.  The M6 collector accepts its full `M6ExprToken` set
(`Ident`, integer, `{ } [ ] ( ) : , . = + -`) with spans; only M7 assigns a
finite `UnsupportedExpression` diagnostic to an accepted but non-finite tree.
`M7DiagnosticKind`, `ResidualObligationKind`,
`CheckedEvaluationKind`, and `SurfaceV0PipelineDiagnostics` are selected
internal checker names only, not a public API/wire commitment.

The finite checker forwards an M6 diagnostic with its existing source span.
On M6-accepted input it rejects duplicate declarations, an unknown state
field, a generated failure outside the handler's declared `fails(...)` row,
an M6-accepted expression outside the finite M7 tree subset, non-`Int`
arithmetic, the selected declaration/type/locus consistency gaps, and duplicate event /
relation / designated / deferred declarations. A duplicate identifies the
second/offending occurrence. Rejection returns a typed diagnostic and no
executable Core.

For the selected profile, checked elaboration retains the M6 source-to-Core
map and canonical source spans as a total, stably ordered map over assignment,
relation, designated, auth, and verify source spans. Every M8-consumable
checked evaluation records the orthogonal M3 `SemanticForm`, `EvaluationSite`,
`TriggerClock`, `AuthorityOrigin`, and `Materialization` axes, an ordered typed
expression tree with source spans and explicit `Int` arithmetic, a typed Core
projection, typed effect row, and
generated obligations. The latter include the declared failure row,
capability, witness, authority, and checked evaluation as applicable.

Same-owner RMW remains `OwnerRmw` with an M8-consumable left-associated typed
expression tree, bounded typed expression summaries, and same-owner reads; its
effect row carries the request-to-owner and owner state write. Each
`EffectEntry` and `GeneratedObligation` is enumerable and retains its canonical
source span/`SourceRef`. A maintained
relation remains `PublishRelation` plus a consumer-local projection, owner /
subject/type / binding-frontier / transform options, and a relation-publication
effect row. A designated result remains `DesignatedPublishValue` with a
logical-tick/result-frontier/version / bounded expression / value-publication
effect row and its explicit typed M7 request-plus-receipt-use input dependency.
That dependency records the designated evaluator, requester site, authority
origin, source-owner locus, typed state read, request, and receipt use; it is
not an M5 request edge or transport delivery semantics. Its evaluation uses
M3 `SemanticForm::Value`, an M3 `InputFrontier(F)` distinct from its result
frontier, a deterministic `EvaluationPolicy`, a conservative
`ObservationPolicy`, and their immutable `PolicyStamp`.
Result/input/result-frontier and relation/binding-frontier stay nominally distinct.

Relation projection carries `Visibility`, `RelationLifetime`, and
`FallbackValidity` residual obligations. A designated published value carries
`ValueVisibilityRedaction`. `with auth MembershipAuth` and `verify finite_refinement` are
successful static residual evidence only (`AuthDeferred` / `VerifyDeferred`).
A residual grants no authority or capability; emits no effect, mutation, or
verdict; and does not make execution admissible. `Authority` and
`AdmittedEvaluatorAuthority` are generated obligations, never authority-success
evidence. The checked artifact records
`execution_is_admissible = false` while residuals remain. In the selected
finite checker, admission additionally requires a nonempty checked evaluation;
it is true only when that evaluation condition and an empty residual row both
hold. An explicit admission demand is the typed
`ResidualCannotExecute` diagnostic, not a verification success.

## Selected alternative and falsifier

The smallest alternative is to wrap M6's templates in an opaque M8-ready
success object. It is rejected because it could turn deferred auth/verify or
unsettled relation release/lifetime facts into implicit authority or execution
success without a typed obligation.

The selected route is falsified if it needs a new M6 grammar form, changes an
M6 classification/span/non-effect, creates executable Core for rejected
source, creates authority/capability/effect/mutation/verdict from a residual,
or produces distinct checked output for the same source. Preserve such
counterevidence and reopen the direct preceding boundary rather than adding a
default.

## Scope and non-effects

This authorizes ADR-0022, theory/16, spec/08, OBL-049 exact finite Lean
evidence, and the bounded checker/test implementation. It does not decide
runtime scheduling/admission, transport, receipt implementation, M9
authorization or verifier semantics, general checker decidability/general
elaboration theorem, SCN conformance, public CLI/JSON/API/ABI/wire, final
grammar, deployment, or an M8 success claim.
