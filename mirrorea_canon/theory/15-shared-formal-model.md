---
id: theory/15-shared-formal-model
status: L1-fixed
maturity: reviewed
depends_on: [theory/01-mircore-v0, theory/03-elaboration, theory/04-ordering-and-cuts, theory/05-authority, theory/06-existence-fallback, theory/07-observation, theory/09-two-layer-time, theory/10-diagnostics, theory/13-evaluation-materialization, theory/14-maintained-relation-projection, adr/ADR-0020]
summary: M1--M4 を一つの concrete finite proof-facing model に統合する M5 shared semantic source。
open_items: []
---

# 15 — Shared formal model / metatheory

This chapter fixes the M5 finite shared semantic source.  It is a
proof-facing reference model for M6--M8, not a selected Surface grammar,
runtime implementation, public representation, or general theorem.

Theory/17 constrains its one finite M8 state using M5-aligned semantic
categories, with admitted-program identity, deterministic queues, patch
lifecycle, and runtime trace data.  The M8 Lean foundation is deliberately
fresh rather than an import/type alias of this M5 file; that implementation
fact establishes neither Rust/M5/M7 type identity nor a second M8 semantic
state.

## 1. One finite universe

The finite model has one concrete carrier family:

```text
SourceRef, SurfaceFragment, Core, StaticDiagnostic,
Config, SemanticStep, WellFormed, TraceRow, ObservationEvent,
ProjectedRelation, PresentationContext, AtomicCut, SaveObject
```

`SurfaceFragment` is an enumerated semantic-tag/source-reference domain.  It
does not choose tokens, precedence, source-file syntax, or final spans; those
remain M6.  Its deterministic elaboration returns exactly a concrete `Core`
operation or concrete static diagnostic.  Runtime validation/rejection is
separate from static diagnosis and is represented by a concrete typed reject
in a `SemanticStep` outcome.

The model defines separate nominal carriers:

```text
ResultFrontier       exact finite designated-result producer set
ResultVersion        finite designated-result version
RelationFrontier     binding activation / lineage frontier
Epoch                membership, binding, witness, anchor epoch values
PresentationContext  consumer-local read-side input
AtomicCut            finite cut history carrier
```

No coercion identifies these types.  In particular, an M3 `ResultFrontier` is
not an M4 `RelationFrontier`, and neither is an `AtomicCut` or an `Epoch`.

## 2. Shared Core, configuration, trace, and well-formedness

M5's finite configuration is the concrete profile of theory/01:

```text
ConfigM5 = ⟨H, Q, S, M, G, W, L, R, D, J, P0⟩
H  finite append-only TraceRow history
Q  pending request tags
S  finite owner state
M  membership epoch
G/W exact finite capability and witness lineage records
L  selected relation/lease liveness profile
R  explicit typed receipt slot
D  designated result/frontier/version/consumption slot
J  RelationDef plus owner-held BindingState and optional published ProjectedRelation
P0 explicitly inactive patch slot
```

`WellFormedM5` is a concrete decidable check: the relation is acyclic in the
declared two-anchor profile; capability and witness records each match the
owner, relation, binding epoch, and membership epoch; the lease is live; a
published projection, when present, exactly matches the current relation,
binding, and activation frontier; and the patch slot is inactive.  History is
an ordered finite occurrence list;
the M5 cut profile saves the complete current finite history only.  It does
not select a general DAG encoding or a distributed cut algorithm.

Every trace row and observation event carries `SourceRef`.  An observation is
a typed finite release carrying observer and derived restriction label, rather
than a debug side channel.

## 3. M3 carriers in the shared model

`EvalPlan` retains distinct evaluation kind, site, materialization, and
optional `ResultFrontier`.  In the finite profile:

```text
owner RMW          owner / store
explicit receipt   owner / store
designated result  evaluator / publish-value / ResultFrontier
relation consume   consumer / local-only
relation publish   owner / publish-relation
```

An unannotated cross-owner operand is a static diagnostic.  A stored receipt
is usable only with its finite request < serve < reply < receive chain and
admitted success release.  A designated duplicate preserves the stored result
instead of creating another version.  These are shared `R` and `D` carriers,
not M3 adapter types.

## 4. M4 relation, authority, presentation, and observation

`J` contains one declared finite `RelationDef`, owner-held `BindingState`, and
an optional published `ProjectedRelation` carrier.  The owner
bind/degrade/reacquire/publish steps check both stored current
capability and witness against the exact relation, binding epoch, membership
epoch, and owner principal.  Bad or stale lineage rejects without binding
mutation.  Degrade advances only primary → fallback; a fresh binding and
witness epoch starts one new lineage.  A successful owner
`publish-relation` constructs and stores the exact projected carrier from the
current relation/binding; degrade and reacquire clear it before their changed
binding becomes usable.

`PresentationContext` is a consumer-local argument to projection.  It is not
part of `Config`, `TraceRow` semantic state, authority, or `SaveObject`; it is
distinct from the published `ProjectedRelation` carrier.  Each required
primary/fallback sample independently checks consumer, admission, anchor,
anchor epoch, and relation activation frontier.  Missing, stale, unadmitted,
or split-frame samples reject.  The consumer `local-only` step reads an
already published carrier and records a typed projection outcome.  An attempt
by the consumer to store, publish a value, or mutate `J` rejects while
preserving owner state.

The finite label chain remains `public < restricted < private`.  A projected
observation label is the finite join of the relation label and each admitted
input label.  The bounded transform/coordinate domain is checked before
evaluation; out-of-domain transform or result rejects without wrap,
saturation, or fallback.

## 5. Cut, save, restore

`AtomicCut` holds the exact finite history at the end of a local cut.  A
`SaveObject` is admitted only from a well-formed cut-ending configuration and
contains that cut/history plus the concrete `Q,S,M,G,W,L,R,D,J,P0` profile,
including any published projected-relation carrier.
Its cut is consistent in the declared profile only when it is identical to
the saved finite history.  The interface deliberately has no
`PresentationContext` field or parameter.

Restore reconstructs a configuration only when the cut is consistent and
ends in a cut occurrence and the reconstructed configuration remains
well-formed.  In particular, mismatched/stale witness or capability lineage,
inactive lease failure, or invalid relation provenance rejects.  This is the
finite M5 interface, not THM-003, a general load algorithm, or a distributed
recovery claim.

## 6. Finite evidence boundary

The exact M5 finite obligations are OBL-040--047 in theory/11.  They cover
the shared elaboration, receipt/designated result, exact relation validation,
projection/admission/label/overflow, mixed consumer falsifier, finite
well-formedness preservation, cut/save/restore, and typed derived
observation.  Their evidence class and exact boundary are stated only in the
ledger.

The following remain deliberately outside this chapter: M6 grammar and final
diagnostics; arbitrary Core programs; arbitrary relation DAGs/nested
selectors/rewrite; general label lattice/noninterference; general chain
normalization; general time/stream theorem; general cut/save/load/Z-cycle;
patch semantics; transport/distributed execution; runtime conformance; and
public API/wire claims.  Existing general OBL rows remain deferred.
