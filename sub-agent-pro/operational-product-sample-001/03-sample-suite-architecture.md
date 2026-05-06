# 03 — sample suite architecture

## 1. Target sample narrative

The operational sample suite must tell this story:

```text
A developer builds a virtual-space product in layers:

WorldCore
  defines a room/world runtime substrate.

MembershipChat
  imports WorldCore and defines join/leave/chat behavior.

SugorokuWorld
  imports MembershipChat and defines a small game with server and participant behavior.

HotPlugLayers
  add debug/auth/rate-limit/object/avatar-preview behavior to the running session.

Runtime and Devtools
  run it locally/Docker, save/load it, inspect it, and bundle it.

Portal/Spatial Future
  records how this can grow into WWW-like world links and future shard/federation.
```

## 2. Layers

### Layer OPS-L1: WorldCore

Responsibilities:

- world identity
- root Place / WorldServerPlace
- membership registry surface
- event DAG policy
- observation/redaction/retention policy
- minimal host adapter declaration

Non-responsibilities:

- game logic
- chat implementation
- avatar runtime
- portal / federation

### Layer OPS-L2: MembershipChat

Responsibilities:

- import WorldCore
- join participant
- leave participant
- chat message typed effect
- membership epoch/incarnation freshness
- observer-safe chat view
- rate-limit failure row declaration

Non-responsibilities:

- final chat product
- rich text/media moderation
- production auth provider

### Layer OPS-L3: SugorokuWorld

Responsibilities:

- import MembershipChat
- game Place
- dice owner
- roll / publish / witness / handoff
- stale action reject
- reset / epoch invalidation if feasible
- game state viewer panel

Non-responsibilities:

- production game engine
- multi-server consensus
- distributed durable save/load

### Layer OPS-L4: HotPlugLayers

Responsibilities:

- debug layer attach
- auth layer attach
- rate-limit layer attach
- placeholder object package
- custom-avatar-preview package
- unsupported-runtime fallback boundary
- deferred detach boundary

Non-responsibilities:

- accepted detach execution
- durable migration
- arbitrary native package execution
- full avatar compatibility

### Layer OPS-L5: Deployment / Projection

Responsibilities:

- local host deployment profile
- Docker deployment profile
- server/client target intent
- packet boundary inventory
- FFI boundary inventory
- future backend non-claim

Non-responsibilities:

- direct LLVM codegen
- placement optimizer
- emitted server/client native binaries

### Layer OPS-L6: Portal / Spatial Future

Responsibilities:

- portal / world-link skeleton
- finite two-shard hard-boundary plan
- gradient observation plan
- replication profile inventory
- static/model-check obligations

Non-responsibilities:

- working continuous federation
- WAN / durable replay
- distributed shard save/load

## 3. Operational flow

Minimum operational flow:

```bash
# Check layered packages
mirrorea-alpha check samples/product-alpha1/operational/world-core
mirrorea-alpha check samples/product-alpha1/operational/membership-chat
mirrorea-alpha check samples/product-alpha1/operational/sugoroku-world

# Run Sugoroku world as a session
MIRROREA_ALPHA_SESSION_DIR=<tmp> mirrorea-alpha run-local samples/product-alpha1/operational/sugoroku-world

# Inspect session
MIRROREA_ALPHA_SESSION_DIR=<tmp> mirrorea-alpha session session#operational-sugoroku

# Attach layers
MIRROREA_ALPHA_SESSION_DIR=<tmp> mirrorea-alpha attach session#operational-sugoroku samples/product-alpha1/operational/packages/debug-layer
MIRROREA_ALPHA_SESSION_DIR=<tmp> mirrorea-alpha attach session#operational-sugoroku samples/product-alpha1/operational/packages/auth-layer
MIRROREA_ALPHA_SESSION_DIR=<tmp> mirrorea-alpha attach session#operational-sugoroku samples/product-alpha1/operational/packages/rate-limit-layer

# Save/load / quiescent save
MIRROREA_ALPHA_SESSION_DIR=<tmp> mirrorea-alpha save session#operational-sugoroku --savepoint savepoint#ops-r0
MIRROREA_ALPHA_SESSION_DIR=<tmp> mirrorea-alpha quiescent-save session#operational-sugoroku --savepoint savepoint#ops-r2
MIRROREA_ALPHA_SESSION_DIR=<tmp> mirrorea-alpha load savepoint#ops-r0 --session session#operational-sugoroku

# Transport
MIRROREA_ALPHA_SESSION_DIR=<tmp> mirrorea-alpha transport session#operational-sugoroku --mode local
MIRROREA_ALPHA_SESSION_DIR=<tmp> mirrorea-alpha transport session#operational-sugoroku --mode docker

# Devtools
MIRROREA_ALPHA_SESSION_DIR=<tmp> mirrorea-alpha export-devtools session#operational-sugoroku --out <viewer-dir>
mirrorea-alpha view <viewer-dir> --check

# Bundle
mirrorea-alpha build-native-bundle samples/product-alpha1/operational/sugoroku-world --out <bundle-dir>
```

If the current CLI cannot run this exact flow, implement the minimum needed or document the planned step honestly.

## 4. Success statement

P-OPS-01 may say:

> The repo now contains an operational product sample suite that demonstrates a layered world-building process from WorldCore through MembershipChat and SugorokuWorld, with product-alpha CLI checks, local session execution, hot-plug layers, local/Docker transport, devtools export, save/load, and native host launch bundle where implemented, while recording projection/backend/portal/spatial federation as explicit future boundaries.

It must not say:

> Mir source now compiles to optimized LLVM server/client binaries.
