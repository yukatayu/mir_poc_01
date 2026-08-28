---
id: spec/04-core-ir
status: L2-working
maturity: draft
depends_on: [theory/01-mircore-v0, theory/03-elaboration, theory/13-evaluation-materialization, theory/14-maintained-relation-projection, theory/15-shared-formal-model, adr/ADR-0021, adr/ADR-0025, adr/ADR-0029, adr/ADR-0031]
summary: M6 CoreTemplate と将来の Core IR 交換形。生成辺・義務・span・provisional relation anchor locus の形、Core companion 記法の附録。
open_items: [OPEN-026]
---

# 04 — M6 CoreTemplate and later Core IR exchange form

## M6 lowering template (non-wire, non-final)

M6 emits inspectable typed `CoreTemplate` records before exchange JSON exists.
M7 subsequently refines those records only through spec/08; the M6
parser/classifier profile itself retains:

```text
source_span
state_observer_safe_fields declaration-only source-bound field subset
m5_core                   present only for accepted ownerRmw
result_frontier/version   designated-result fields only
designated_consumer       explicit evaluator/result ref + one named consumer
binding_frontier          maintained-relation field only
relation_anchor_loci      optional explicit primary/fallback existence loci,
                          each with its exact source span
owner_publication_kind    PublishRelation for maintained relation
published_relation_carrier true only for accepted maintained relation
consumer_projection_site  optional consumer-local relation projection
deferred_policy_kind      WithAuth | Verify, non-executable only
source_to_core_map        source span -> ownerRmw/local-read/local-write |
                          designated-decision | publish-relation |
                          designated-result-consume |
                          consumer-local-projection | deferred-policy |
                          observer-publish
authority_audit           Role authority origin, nested owner site,
                          required authority names
```

`result_frontier` and `binding_frontier` are distinct nominal carriers. A
`Role[self] at L_actor` action nested at `L_owner` has M5 `ownerRmw`: its
authority origin remains `L_actor`, its evaluation/request site is `L_owner`,
and the two loci may differ. The target state's declared owner must equal
`L_owner`; same-owner RHS reads resolve there, not at `L_actor`. The template
retains `store`, request-to-owner and owner-write edges, `RouteUnavailable`,
and separate capability/witness obligations. Its source-to-Core map separately
records the owner-local RHS dependency; it emits no receipt or receipt-release
fact. This does not claim that a cross-owner action has no receipt: such an
operand is rejected with the receipt-required diagnostic rather than lowered
through a hidden receipt path.

`state_observer_safe_fields` records only the explicitly declared
`visible observer_safe fields (...)` subset; omission is private. A write to a
listed field adds a source-bound `observer-publish` effect/map entry and the
typed `VisibilityDenied` failure entry. A private-field write adds neither.
This is one facet of M10's bounded M6/M7 direct-consumer seam, not a final
observer API or a runtime publication instruction.

No owner `CoreTemplate` exists for a non-literal role actor, a fieldless
assignment target, or a target whose declared state owner is outside the
action locus. They remain respectively `RoleActorMustBeLiteralSelf` (parser
diagnostic at the actor token), `FieldlessAssignmentTarget`, and
`CrossOwnerWriteTargetOutsideActionLocus` (classifier diagnostics at the
target reference). These are distinct from
`CrossOwnerOperandRequiresReceipt`, which describes an RHS operand.

Maintained relation lowering retains an owner publication of the relation
carrier and an optional consumer-local projection, with only a binding
frontier. Designated evaluation retains `publish-value`, a result frontier, and
a result version, never a binding frontier. The bounded clause
`designated consume E.result at C` lowers separately to a source-bound
designated-result-consume template that preserves the producer result identity,
input/result frontiers, version, observation policy, retry contract, and named
consumer. It contains neither the evaluator expression nor raw remote input and
cannot be synthesized from topology or a relation projection. The retry field
is a static SYS-4 refinement requirement, not current M8 behavior/evidence;
actual idempotent return is outside this Core. `with auth` and `verify` emit
successful non-executable typed deferred templates; the former only supplies a
required-authority name. Neither settles M9 semantics. Every listed node
preserves the canonical source span.

The SYS-5 provisional relation refinement retains an explicit primary or
fallback anchor locus as part of the corresponding Core anchor, with its exact
source reference. It remains distinct from relation owner and consumer locus.
An omitted locus remains absent and cannot be filled by topology/projector/
runtime inference. This internal field supports source-bound lifecycle
placement; it is not a final Core exchange field or public ABI.

This template is not M5 Core, a final AST, JSON, ABI, wire record, runtime
instruction, or public contract. The designated-consume template is exactly a
bounded internal Surface-v0/M6 seam for SYS-3; it does not freeze syntax or
compatibility. This template does not add a `PresentationContext` field,
receipt source syntax, transport, or execution behavior.

## Later exchange form (still L2-working)

Purpose: a stable, inspectable JSON form of elaboration output for checker,
runtime, projector, devtools. Shape (field names L2-working):

```json
{ "module": "Surface.E2E.SugorokuPositive",
  "core_items": [
    { "kind": "transition", "at": "World", "name": "serve_roll", "ops": [
        { "kind": "write", "state": "player", "key_from": "req.principal",
          "field": "position", "expr": "...", "span": {"file":"...","range":[l,c,l,c]} } ] },
    { "kind": "handler_entry", "locus_role": "BrowserClient[self]",
      "name": "roll", "fails": ["MissingCapability","StaleMembership", "..."] } ],
  "generated_edges": [
    { "kind": "request", "from": "BrowserClient[self]", "to": "World",
      "op": "write(player[self].position)", "caps": ["cap_move"],
      "witnesses": [], "fails": ["..."], "span": {...} },
    { "kind": "publish", "at": "World", "state": "player", "field": "position",
      "visibility": "observer_safe", "span": {...} } ],
  "obligations": [
    { "obligation_id": "...", "obligation_kind": "capability|proof|model",
      "source_refs": ["span..."], "suggested_target": "checker|model_check|proof",
      "current_status": "undischarged" } ],
  "source_map": [ {"core_ref": "...", "span": {...}} ] }
```

Invariants: every generated edge and core op carries a span (BND-001); the
obligations array is exactly the judgment's O; nothing in the runtime may
execute Core IR that lacks a checker verdict (BND-004). Before M6, the
parser-free reference has no Surface span: it carries a deterministic operation
key in every `EvalPlan` and trace row as the bounded BND-001 surrogate. It
cannot be presented as an M6 source-map implementation.

## M3 evaluation delta (non-wire, non-final)

The M3 internal Core adds an `eval_plan` on each evaluation-bearing operation:
`form`, `site`, `trigger`, `authority_origin`, and `materialization`, plus a
declared finite policy and required frontier for a designated materialization.
`eval` is the Core operation; owner transition, remote result/receipt,
designated evaluation, and result consumption are its normalized forms or
occurrence categories in theory/13. They must be visible in generated
dependency/occurrence rows with a source span or, in the parser-free M3
reference only, its deterministic operation key. This paragraph does
not select JSON names or make the exchange form public; OPEN-026 remains.

## M4 maintained-relation delta (non-wire, non-final)

The M4 internal Core carries a domain-neutral `RelationDef` and owner-held
`BindingState`, plus an admitted `ProjectedRelation` publication. The relation
publication is the M3 `publish-relation` materialization; it contains relation
reference, required anchor refs/epochs, selected anchor, relative transform,
lineage/binding epoch,
activation frontier, and relation label. The evaluated/observed derived result
uses the greatest restriction of that label and its admitted inputs. It is not
a `publish-value` or `adapter-stream` record for an absolute derived pose.

The consumer's presentation-frame `eval` remains `local-only` and must cite a
coherent `PresentationContext` with all required anchors at one frontier and
the expected anchor epochs. Its Core outcome is a local presentation result or
an M4 semantic reject category (cycle, stale anchor, split frame, non-owner
mutation, weak release), not an inferred owner mutation. These paragraphs do
not select exchange JSON names, Surface syntax, wire fields, or final public
ABI.

## Appendix — Core companion textual notation (non-Surface)

For theory prose, fixtures, and Core-level tests (inherits LAB D-030..D-044):

```text
perform op on target            direct effect request
    require pred                statement-local clauses attach to the perform
    ensure pred
perform op via chain_ref        request through a canonical chain
option name on target capability cap lease guard
    admit pred                  option-local admission metadata
chain ref = head
    fallback successor
        @ lineage(pred -> succ)
try { ... } fallback { ... }    local rollback + explicit branch
atomic_cut
```

These tokens are rejected in Surface v0 (E-PARSE-005). OPEN-026: field-name
freeze for the JSON form happens at PHASE-I1 exit.
