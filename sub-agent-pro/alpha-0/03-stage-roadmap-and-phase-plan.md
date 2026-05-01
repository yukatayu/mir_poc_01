# 03 — Stage roadmap and phase plan

The project should track two scales:

1. Large product/research stages: Stage A..F plus upper layers.
2. Concrete implementation packages/phases that Codex can close with reports, validation, commits, and pushes.

The stages are not strict implementation order. Work may cross layers, but progress documents must always state which stage and package the repository is currently advancing.

## 1. Large stages

### Stage A — Current repo-local alpha-ready floor

Current state: mostly reached.

Meaning:

- Mir current-L2 theory, samples, helpers, and evidence exist.
- Active sample suite is `samples/clean-near-end/`.
- Sugoroku, avatar follow, typed external preview, network canary, projection/codegen bridge, viewer prototype inventory, and hot-plug carrier floors exist as runnable or closeout-backed surfaces.
- This is not a final public product.

Main evidence:

- clean-near-end suite
- current-L2 base corpus
- Lean foundations / generated stubs
- Sugoroku world helper
- avatar follow active slice
- typed external `EXT-03/04`
- network `NET-02..05`
- projection manifest bridge
- viewer prototype inventory
- hot-plug request/verdict carrier and runtime skeleton

### Stage B — Alpha 0.5: Integrated local Mirrorea runtime

Goal:

- Move from helper-local emulators and report-local carriers toward an integrated local runtime.
- Multiple Places run inside one process or controlled local runtime.
- MessageEnvelope, MembershipRegistry, PlaceCatalog, HotPlugRequest, and HotPlugVerdict become ordinary runtime data rather than only report evidence.

Required user-visible outputs:

- local Sugoroku runtime through Rust runtime API
- event DAG export
- Place graph export
- fallback degradation trace
- hot-plug request/verdict trace

### Stage C — Alpha 0.7: Real transport narrow cut

Goal:

- Add narrow real transport across process/container boundary.
- Preserve transport/auth/membership/capability/witness separation.
- Docker E2E becomes part of validation.

Required outputs:

- Docker network test with world server and participant container
- envelope route trace
- stale membership negative test
- missing capability/witness negative tests
- observer-safe redacted route trace

### Stage D — Alpha 0.8: Hot-plug minimal lifecycle

Goal:

- Runtime package / layer / object attach and detach become actual runtime operations.
- Compatibility, authority, membership freshness, verdict, activation cut, and audit evidence are implemented.

Required outputs:

- attach SugorokuGame to empty world
- attach debug layer to running world
- attach rate-limit/auth layer with contract checks
- incompatible patch reject
- detach minimal contract with no dangling references

### Stage E — Alpha 0.9: Visualization/devtools

Goal:

- Event graph, Place graph, route trace, witness timeline, membership timeline, hot-plug lifecycle, fallback degradation, and redaction modes become inspectable.

Required outputs:

- HTML/devtools-style viewer or JSON viewer surface
- admin view vs observer-safe view
- on-demand trace attach/detach
- no raw witness/auth leak in observer mode

### Stage F — Alpha 1: Mirrorea Spaces alpha

Goal:

- A minimal browser-like shared virtual-space platform exists.
- It is not full VRChat, but it demonstrates the substrate for VRChat-class capabilities.

Required outputs:

- small world/room runtime
- participant/placeholder avatar presence
- object interaction
- runtime package hot-plug
- networked Place execution via Docker
- event/witness/route/membership/hot-plug visualization
- local save/load with explicit non-claim for distributed save if not implemented
- negative tests for missing capability, missing witness, stale membership, incompatible patch, invalid cut

### Upper Layer G — Mirrorea Spaces product expansion

Goal:

- Build practical social virtual-space capabilities.

Examples:

- avatar runtime ecosystem
- portal/world discovery
- friend/group/permission/moderation
- SDK / world authoring
- browser-like client
- safety policies

### Upper Layer H — Mirrorea Atlas

Goal:

- Multi-world graph, portal, and relation layer.

Examples:

- world graph
- route graph
- authority graph
- portal graph
- cross-world object references
- spatialized proximity by movement/use

### Upper Layer I — Reversed Library

Goal:

- Knowledge-space flagship application.

Examples:

- concepts/fields/documents/media/experiments as places or objects
- people and communities as active spatial relations
- citations, dependencies, conflicts, and extensions as visible paths
- collaborative learning/research/creation

## 2. Concrete Codex phases

### Phase 0 — Theory freeze preparation

Deliverables:

- `specs/13..17`
- `plan/39..43`
- `samples/alpha/` sample matrix scaffold
- docs synchronization
- new report

Validation:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

### Phase 1 — Typed IR and checker skeleton

Deliverables:

- `crates/mir-ir` or equivalent typed IR carrier
- `crates/mir-checker` or equivalent checker crate/module
- lifetime/fallback canonicalizer
- contract subtyping checker
- diagnostics schema
- positive/negative tests

Initial checks:

- dangling raw reference reject
- fallback evidence required
- capability promotion reject
- mutable invariance
- precondition/postcondition overlay checks
- effect/failure row containment

### Phase 2 — Executable semantics and event DAG

Deliverables:

- event DAG model
- fallback degradation runtime trace
- local rollback / `atomic_cut` semantics
- witness / publish / observe / handoff event relations
- local save skeleton

### Phase 3 — Local Mirrorea runtime

Deliverables:

- Place runtime
- queue and dispatch model
- MessageEnvelope dispatch
- membership freshness check
- local Sugoroku runtime
- local hot-plug carrier consumption

### Phase 4 — Layer insertion runtime

Deliverables:

- LayerKind
- AttachPoint model
- LayerContract
- compatibility/authority checks
- debug layer
- auth/rate-limit layer samples
- redaction layer

### Phase 5 — Network/Docker E2E

Deliverables:

- transport trait
- TCP or subprocess JSON transport
- Docker Compose E2E
- stale membership / missing capability / missing witness tests
- observer-safe route trace

### Phase 6 — Save/load and consistent cut

Deliverables:

- consistent cut predicate
- local save/load
- invalid distributed snapshot checker
- Z-cycle negative sample
- channel/in-flight message representation

### Phase 7 — Runtime package/avatar adapter

Deliverables:

- RuntimePackageManifest
- package capability/effect manifest
- placeholder avatar runtime
- custom Mir avatar runtime skeleton
- VRM/VRChat/Unity adapter skeletons as non-core planned lanes
- native provider reject/trust-policy samples

### Phase 8 — Mirrorea Spaces alpha demo

Deliverables:

- integrated local + Docker demo
- hot-plug module
- debug/devtools viewer
- local save/load
- avatar runtime fallback
- report and docs closeout

## 3. Progress.md requirements

`progress.md` must always show:

- current large stage: A/B/C/D/E/F/G/H/I
- current concrete phase: 0..8
- current package identifier
- current validation freshness
- current blockers
- next autonomous package
- user-decision blockers, if any

Do not let `progress.md` become an append-only package log. It should remain a current snapshot with a short recent log.

## 4. Samples progress requirements

`samples_progress.md` must show rows for:

- lifetime/fallback
- contract variance/layer insertion
- cut/save/load
- local runtime
- network Docker E2E
- hot-plug lifecycle
- visualization/devtools
- runtime package/avatar adapter
- Mirrorea Spaces alpha demo

Each row must include:

- stage/phase
- sample path
- runner/validation command
- progress percentage
- blocker
- report/evidence link
- whether it is active, planned, generated, helper-local, or deferred

## 5. Completion language

Never call Stage F complete unless there is a working demo with local runtime, network/Docker E2E, hot-plug, typed debug visualization, and negative tests.

Never call Reversed Library complete or implemented during Mirrorea alpha work.
