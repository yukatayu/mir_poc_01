# 15 — Glossary and stop lines

Use this glossary to avoid ambiguous wording.

## 1. Glossary

### Mir

Semantic core language and theory for effects, contracts, ownership, lifetime, fallback, rollback, cut, and safe evolution.

### Mirrorea

Fabric/runtime for Place, route, membership, capability, witness, message envelope, hot-plug, audit, visualization, and transport separation.

### Mirrorea Spaces

Social/shared virtual-space platform built on Mirrorea. Long-term functional lower bound includes VRChat-class capabilities, but with browser-like openness, typed effects, inspection, and hot-plug.

### Mirrorea Atlas

Future multi-world graph/portal/relation layer.

### Reversed Library / 裏返した図書館

Upper-layer knowledge-space flagship application. Not the Mirrorea runtime.

### PrismCascade

Separate media-processing kernel candidate. Not in Mirrorea alpha scope.

### Place

Execution locus with local state, queue, rollback scope, capabilities/visibility/observation frontier. Not equal to participant.

### Participant

Actor/user/entity taking part in a space. May be associated with one or more Places, but is not the same as Place.

### Principal

Authority/security identity used for authentication/authorization decisions.

### Guarded access path

Finite ordered set of guarded options for accessing a logical role/path. Fallback extends the availability of this path, not the lifetime of the selected target.

### Lease

Lifetime/freshness guard for an option in a guarded access path.

### Fallback

Monotone degradation from an earlier option to a later contract-compatible option.

### Inherited chain

Explicit splicing of another guarded chain's options into a new chain, requiring lineage evidence.

### Snapshot selected

Capturing the currently selected target without inheriting the whole fallback chain.

### Contract

Declared interface including input/output types, preconditions, postconditions, effects, failures, capabilities, cost, observation, and retention.

### Overlay/layer

Typed/contract-aware wrapper or insertion that may observe, transform, reject, authenticate, rate-limit, redact, or visualize, subject to compatibility rules.

### MessageEnvelope

Carrier for cross-Place/process/network message with separated transport, auth, membership, capability, authorization, witness, dispatch outcome, and notes.

### HotPlugRequest / HotPlugVerdict

Carrier for runtime package/layer/object attach/detach admission and verdict.

### Atomic cut

Place-local rollback frontier. Not distributed commit or durable save.

### Consistent cut

Prefix-closed event set under causal order, required for distributed save/load.

### Z-cycle

Checkpoint dependency cycle that can make a checkpoint unusable for a consistent global checkpoint.

### Runtime package

Hot-pluggable package implementing roles/effects/adapters with manifest, capabilities, and compatibility claims.

### Avatar adapter

Runtime package that implements avatar-related abstract roles for a concrete format or behavior system. Not core.

## 2. Stop lines

### Do not claim final parser grammar

Alpha theory may use source-ish notation and typed IR. Final grammar remains deferred.

### Do not claim production runtime

Helper-local runners, local runtime skeletons, and Docker E2E are not production runtime unless explicitly completed and validated.

### Do not claim production transport

Network canaries and Docker E2E are not WAN federation or production transport.

### Do not claim distributed save/load

Local save/load or cut checkers do not equal distributed durable save/load.

### Do not claim durable migration

Hot-plug runtime skeleton and activation cut do not equal durable migration/rollback protocol.

### Do not claim final viewer API

Viewer prototypes and JSON exports are not final public viewer/telemetry API.

### Do not claim avatar compatibility

Adapter skeletons are not full VRChat/VRM/Unity compatibility.

### Do not claim native binary safety from signature

Signature proves provenance only. Safety needs sandbox/capability/effect/resource policy.

### Do not claim Reversed Library implementation

Reversed Library remains upper-layer future application until explicitly implemented.

### Do not include PrismCascade in alpha scope

PrismCascade is separate. Only future adapter/trace linkage may be mentioned.

## 3. Preferred wording

Use:

- `repo-local alpha floor`
- `alpha-local specification`
- `helper-local evidence`
- `report-local inventory`
- `generated bridge evidence`
- `planned skeleton`
- `deferred public surface`
- `not final public API`

Avoid:

- `complete` without current-scope evidence
- `production` for helper-local tools
- `runtime` for static carrier-only evidence
- `distributed save` for local save
- `avatar support` for adapter skeletons
- `safe native binary` for signed binary

## 4. Canonical one-paragraph summary

Mirrorea Spaces alpha is a browser-like virtual-space substrate built on Mir and Mirrorea. Mir provides the semantic core for effects, contracts, lifetime, fallback, rollback, and cut discipline. Mirrorea provides Place, route, membership, capability, witness, hot-plug, audit, transport separation, and visualization. The alpha path freezes lifetime/fallback typing, contract/layer variance, and save/load/consistent-cut theory before expanding runtime, network, hot-plug, devtools, and runtime package/avatar adapter support. Reversed Library is a future flagship knowledge-space application. PrismCascade is a separate media kernel candidate.
