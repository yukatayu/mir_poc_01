# 01 — Canonical decisions for Alpha-0

This file records decisions that should guide the next Codex tranche. Where these decisions become normative, mirror them into `specs/`. Where they are roadmap memory, mirror them into `plan/`. Do not leave them only in this handoff package.

## 1. Product and layer decisions

### D-A0-001 — Immediate target

The immediate target is **Mirrorea Spaces alpha**, not the Reversed Library, not PrismCascade, and not a final public language release.

Mirrorea Spaces alpha means:

> A minimal virtual-space runtime/fabric where multiple Places can execute, exchange typed MessageEnvelopes, admit runtime packages or layers by HotPlugRequest / HotPlugVerdict / activation cut, and expose event DAG / witness / route / membership / capability / telemetry evidence through a typed debug/visualization surface.

### D-A0-002 — VRChat relationship

VRChat-class functionality is a long-term functional lower bound. The platform should eventually support at least the non-payment-related capabilities expected of a social VR platform: worlds, avatars, instances, portals, object interaction, user-generated content, permissions, moderation, safety, and social presence.

However, the target is not a VRChat clone. The target is a **browser-like virtual world platform**: clients are closer to browsers, worlds are closer to linked/inspectable web documents or web applications, and runtime packages are contract-bearing modules rather than opaque platform-specific uploads.

### D-A0-003 — Mir / Mirrorea / Spaces / Atlas / Reversed Library naming

Use this naming boundary unless explicitly changed later:

- `Mir`: semantic core language and theory.
- `Mirrorea`: fabric/runtime for Place, route, hot-plug, membership, witness, audit, visualization, and transport separation.
- `Mirrorea Spaces`: social/shared virtual-space product layer.
- `Mirrorea Atlas`: future multi-world graph / portal / relation navigation layer.
- `Reversed Library` / `裏返した図書館`: flagship upper-layer knowledge-space application.
- `PrismCascade`: separate media-processing kernel candidate, not in Mirrorea alpha scope.

Do not write `Mirrorea = Reversed Library`. Mirrorea is the substrate; Reversed Library is an application/experience on top of it.

### D-A0-004 — Implementation shape

For alpha, use **library-first + helper CLI + HTML/devtools-style viewer + Docker E2E**.

Do not commit yet to installed binary, browser host, engine adapter, Unity/Unreal host, or final packaging adoption. Those are later user-facing decisions.

### D-A0-005 — Parser grammar

Do not freeze the final public parser grammar. First freeze the typed IR, checker obligations, sample matrix, and executable semantics. Parser syntax may remain source-ish / companion notation until the theory is stable.

## 2. Type/lifetime/fallback decisions

### D-A0-101 — Remote references

Values in other processes, Places, or machines are not raw pointers. They are observed/guarded logical access paths with explicit target, lease/freshness guard, capability, contract, witness, and lineage evidence where needed.

Read-only/observe references should be the default for remote values. Write-capable remote access must require explicit capability and contract.

### D-A0-102 — Fallback intent

Fallback is intended to prevent dangling by extending **access path availability**, not by extending the lifetime of the original target.

Correct wording:

> `c fallback a` does not make `c` live as long as `a`. It creates a guarded access path that first tries `c` while its guard is valid and then monotonically degrades to `a` if the contract permits.

### D-A0-103 — Default fallback propagation

Fallback-chain propagation is not implicit. The default interpretation of a variable or reference cell is a reference to that cell/object, not an automatic inheritance of the fallback chain stored inside it.

If a chain should be inherited/spliced into another chain, this must be explicit and must carry static evidence that the options belong to the same logical access path / semantic lineage.

### D-A0-104 — Barrier vs inherit syntax

Do not add a core propagation-prevention operator as the default solution. Prefer explicit propagation/inheritance syntax or IR fields, e.g. `inherit_chain`, `splice_lineage`, or equivalent. A future surface syntax may include a barrier sugar, but the semantic default is non-propagating.

### D-A0-105 — Underdeclared fallback

Underdeclared fallback is a static error in the current alpha theory. Missing access target, missing lineage annotation, missing contract surface, or missing capability surface must not silently become dynamic Reject or hidden acceptance.

### D-A0-106 — No re-promotion

Once a fallback chain has degraded past an option, the same semantic lineage must not silently re-promote to that earlier/stronger option. Reacquire may exist later, but it must be an explicit new event with new evidence, not implicit resurrection.

### D-A0-107 — Read/write variance split

Read-only references may be covariant when observation labels and contracts permit. Mutable/read-write references are invariant. Write capabilities must not be obtained through fallback or subtyping promotion.

## 3. Layer insertion and contract decisions

### D-A0-201 — Layer insertion

Debug, telemetry, authentication, authorization, rate-limit, adapter, redaction, and visualization layers may be inserted later, but only as typed/contract-aware layers. They are not untyped middleware and not hidden proxies.

### D-A0-202 — Compatibility-preserving overlay

A transparent overlay must not strengthen preconditions, weaken guarantees, widen undeclared effect/failure rows, require stronger capabilities, leak higher-label observations, or worsen cost/latency bounds unless the contract explicitly permits that degradation or a versioned contract change occurs.

### D-A0-203 — Auth and rate-limit layers

Auth and rate-limit layers are not automatically transparent. If they introduce `AuthFailed`, `MissingCapability`, `RateLimited`, `Deferred`, or similar outcomes, those outcomes must already be in the declared failure row or must be introduced through an explicit contract update/activation cut.

### D-A0-204 — Debug layer authority

Debug layers are security-sensitive. They must require explicit authority and observation labels, redaction, and retention policy. Internal variables and witnesses must not leak through debug layers by default.

## 4. Cut/save/load decisions

### D-A0-301 — `atomic_cut`

Keep `atomic_cut` place-local. It finalizes the current Place's rollback frontier only. It does not mean distributed agreement, global synchronization, durable persistence, or process-wide finalization.

### D-A0-302 — Save/load and consistent cuts

Distributed save/load is only valid on consistent cuts. A receive cannot be in a saved cut unless the corresponding send is also in the cut or the message is represented as in-flight channel state. Witness use requires witness creation. Observe requires publish. Hot-plug activation requires request and verdict.

### D-A0-303 — Z-cycle boundary

A checkpoint participating in a Z-cycle / useless-checkpoint dependency is not an admissible recoverable global checkpoint. Alpha may initially implement local save/load only, but it must not claim distributed save/load without a consistent-cut predicate and a Z-cycle/checkpoint-dependency story.

### D-A0-304 — No resurrection by load

Load must not resurrect expired leases, stale witnesses, stale membership epochs, or degraded fallback positions unless a new explicit epoch/event creates fresh evidence.

### D-A0-305 — Irreversible external effects

Rollback of irreversible external effects is forbidden unless the effect is isolated outside the rollback region or has explicit compensation obligations. Hidden rollback across external effects is not allowed.

## 5. Runtime package/avatar decisions

### D-A0-401 — Avatar is not core-special

Do not encode VRChat, VRM, Unity, or any avatar format as a Mir core primitive. Avatar support is provided by runtime packages/adapters that implement abstract roles/contracts such as renderable, animatable, embodied presence, input consumer, effect provider, and interaction target.

### D-A0-402 — Runtime libraries as hot-plug packages

VRChat compatibility, VRM support, Unity-derived behavior, custom Mir avatar scripts, and native providers should all be modeled as runtime packages or adapters admitted through hot-plug checks.

### D-A0-403 — Native binary trust

Digital signatures prove provenance, not semantic safety. A native binary may only be admitted under an explicit trust policy that also includes capability manifest, sandbox/process isolation or equivalent containment, resource limits, crash containment, revocation story, audit, and effect-row restrictions.

### D-A0-404 — Unsupported runtime fallback

If a client lacks a runtime package or rejects it, the world should be able to fall back monotonically to a placeholder, simplified skeleton, static representation, or rejected/hidden representation according to contract and safety policy.

## 6. Deferred decisions

Do not decide these during the first Alpha-0 integration package unless explicitly instructed:

- final parser grammar
- first public API surface
- installed binary packaging
- browser/native/engine host target
- production theorem prover binding
- production model checker binding
- durable distributed save/load algorithm
- WAN federation protocol
- full avatar/shader compatibility
- native binary execution beyond skeleton policy
- Reversed Library product implementation
- PrismCascade integration
