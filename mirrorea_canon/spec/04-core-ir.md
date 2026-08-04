---
id: spec/04-core-ir
status: L2-working
maturity: draft
depends_on: [theory/01-mircore-v0, theory/03-elaboration]
summary: Core IR の交換形(JSON)、生成辺・義務・span の形。Core companion 記法の附録。
open_items: [OPEN-026]
---

# 04 — Core IR exchange form

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
