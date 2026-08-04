---
id: meta/proposal-023
status: L1-fixed
maturity: reviewed
depends_on: [meta/proposal-018, adr/ADR-0015, adr/ADR-0018, adr/ADR-0019]
summary: owner-approved M5 scopeで、M1--M4 を一つの concrete finite shared model に統合する提案。
open_items: []
---

# PROPOSAL-023 — Shared formal model / metatheory

## Owner disposition

Apply the owner-approved M5 scope in PROPOSAL-018 and ADR-0015.  Adopt one
fresh, concrete, finite proof-facing universe for the accepted M1--M4 rules.
It contains a semantic-tag `SurfaceFragment` with `SourceRef`, deterministic
elaboration to `Core` or `Diagnostic`, `Config`, `Step`, `WellFormed`,
occurrence/history, typed observation, relation/binding projection, and a
cut/save/restore interface.  It is the M6 semantic source, not M6 grammar.

The finite universe has nominally distinct carriers for M3 designated-result
frontier/version, M4 relation activation frontier and epochs, presentation
context, and atomic cut.  It therefore does not identify a result producer
set with a relation binding frontier, or a temporary consumer frame with a
saved semantic carrier.  `Config` has concrete finite request, receipt,
designated-result, authority/witness, relation/binding, optional
published-projection, and inactive-patch components; no predicate is left
opaque for the declared profile.

## Selected finite rule

The owner validates an exact capability and witness lineage for the relation
and binding epoch before its semantic bind/degrade/reacquire/publication
steps.  It publishes only `publish-relation`.  Every required consumer sample
independently validates its release/admission, anchor, epoch, and activation
frontier; the consumer plan is `local-only`.  Projection derives the finite
greatest restriction over the relation and all admitted inputs, and checked
finite transform arithmetic rejects overflow.

The required mixed trace is owner bind → owner `publish-relation` →
consumer-local projection.  Publication places the exact relation/binding
projection carrier in `Config`, and the consumer-local action reads that
carrier with a separate presentation context.  The required mixed falsifier
rejects a consumer attempt to `store`, `publish-value`, or mutate `J`,
preserving semantic owner state.  A cut-backed SaveObject retains all finite
relation/binding/published-projection, authority/witness, receipt/result, and
fallback provenance, but excludes the presentation context.  Restore rejects
the declared stale provenance cases.

## Alternative and falsifier

The sole smaller alternative is an adapter layer that imports M3 and M4 and
translates their existing types.  It is rejected: their `Locus`, frontier,
authority, materialization, and state carriers are incompatible, so such a
layer would prove translation behavior without establishing one shared
`Config`/`Step` source.  A model that permits a result frontier as a relation
frontier, stores a presentation frame, or admits consumer semantic mutation
falsifies the selected route.

## Scope and non-effects

This authorizes ADR-0020, theory/15, exact finite Lean evidence, the ledger
classification for that evidence, and the normal report/review procedure.  It
does not select M6 grammar, parser, final diagnostic IDs, Core exchange/JSON,
public API/ABI/wire, arbitrary relation DAGs or label lattices, general
save/load/cut theorems, patch semantics, transport/distributed execution,
runtime conformance, or product/deployment status.  General OBL rows remain
intentionally deferred unless exact later evidence changes them.
