# 14 — First spec draft outlines

This file gives section outlines for the new specs. Codex should turn these into polished repository documents.

## specs/13-type-system-lifetime-fallback.md

Suggested sections:

1. Role of this spec
2. Decision level and relation to Mir-0/current L2
3. Core distinction: target lifetime vs access path availability
4. Guarded access path model
5. Option fields: target, lease, contract, capability, label, lineage
6. Raw reference vs guarded reference vs inherited chain vs snapshot selected
7. Canonical fallback chains
8. Static evidence floor
9. Static rejection vs dynamic Reject
10. Read/write/observe capability variance
11. Remote observed reference model
12. Rollback, atomic_cut, load, and no resurrection
13. Bird/sparkle anchor example
14. Proof obligations
15. Required samples
16. Deferred syntax and theory questions

## specs/14-contract-subtyping-layer-compatibility.md

Suggested sections:

1. Role of this spec
2. Contract shape
3. Transparent overlay substitutability
4. Input/output variance
5. Precondition/postcondition rules
6. Effect row containment
7. Failure row containment
8. Capability monotonicity
9. Cost/latency contracts
10. Observation label, redaction, retention
11. Layer kinds
12. Attach points
13. Auth/rate-limit special cases
14. Debug layer authority
15. Proof obligations
16. Required samples
17. Deferred questions

## specs/15-cut-save-load-checkpoint.md

Suggested sections:

1. Role of this spec
2. `atomic_cut` retained boundary
3. Event DAG and causal order
4. Consistent cut predicate
5. Save state contents
6. Load/rollback constraints
7. In-flight messages
8. Witness/publish/hot-plug closure
9. Z-cycle/useless checkpoint boundary
10. Local save/load alpha scope
11. Distributed save/load deferred scope
12. `durable_cut` deferred boundary
13. External irreversible effects and compensation
14. Proof obligations
15. Required samples
16. Deferred questions

## specs/16-runtime-package-adapter-hotplug.md

Suggested sections:

1. Role of this spec
2. Runtime package concept
3. Avatar non-core decision
4. Abstract avatar/object roles
5. RuntimePackageManifest alpha shape
6. Hot-plug admission flow
7. Capability/effect/failure manifest
8. Native binary trust policy
9. VRM/VRChat/Unity/custom adapter skeletons
10. Unsupported runtime fallback
11. Package detach and active dependents
12. Proof/checker obligations
13. Required samples
14. Deferred questions

## specs/17-mirrorea-spaces-alpha-scope.md

Suggested sections:

1. Role of this spec
2. Mirrorea Spaces alpha definition
3. Relationship to VRChat-class functionality
4. Browser-like virtual world interpretation
5. Completion conditions
6. Non-goals
7. Stage A..F and upper layers
8. Required E2E demos
9. Required negative tests
10. User-visible artifacts
11. Relationship to Reversed Library
12. Relationship to PrismCascade
13. User decision blockers
14. Deferred product decisions

## plan/39-type-system-freeze-roadmap.md

Suggested sections:

1. Purpose
2. Current repo state
3. Decisions mirrored from specs/13
4. Checker implementation roadmap
5. Sample roadmap
6. Lean/proof roadmap
7. Deferred questions
8. Next package

## plan/40-layer-compatibility-freeze-roadmap.md

Suggested sections:

1. Purpose
2. Current repo state
3. Decisions mirrored from specs/14
4. Layer insertion implementation roadmap
5. Devtools/telemetry roadmap
6. Sample roadmap
7. Deferred questions
8. Next package

## plan/41-save-load-checkpoint-roadmap.md

Suggested sections:

1. Purpose
2. Current repo state
3. Decisions mirrored from specs/15
4. Local save/load roadmap
5. Consistent cut checker roadmap
6. Docker/distributed negative sample roadmap
7. Deferred durable/distributed protocol
8. Next package

## plan/42-runtime-package-avatar-roadmap.md

Suggested sections:

1. Purpose
2. Current repo state
3. Decisions mirrored from specs/16
4. Runtime package manifest roadmap
5. Avatar adapter skeleton roadmap
6. Native trust policy roadmap
7. Sample roadmap
8. Deferred questions

## plan/43-alpha-e2e-roadmap.md

Suggested sections:

1. Purpose
2. Stage A..F map
3. Phase 0..8 packages
4. Docker E2E path
5. Mirrorea Spaces alpha demo completion condition
6. Validation floor
7. Stop lines
8. Next package
