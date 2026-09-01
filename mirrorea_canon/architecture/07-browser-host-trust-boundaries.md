---
id: arch/07-browser-host-trust-boundaries
status: L1-fixed
maturity: reviewed
depends_on: [root/north-star, root/design-constitution, arch/02-boundary-contracts, arch/06-project-product-layers, theory/05-authority, theory/07-observation, theory/13-evaluation-materialization, adr/ADR-0036]
summary: Browser/Host package admission、View/input、typed provider、privileged raw FFI、resource/sandboxの非凍結trust boundary。
open_items: []
---

# 07 — Browser/Host trust boundaries

## Scope and direct consumers

```text
Direct consumer: I3-0 transport, I3-1 encoding/log, I3-5 joined view, NEXT-0/I5
Blocker reduced: package admission, semantic grant, View computation, provider
  access, raw native access, and resource enforcement previously had no separate
  Canon contracts
Acceptance use: reject any I3 or later design that derives Mir authority from a
  package, process, route, session, certificate, provider, renderer, or host
```

## Trust tiers are not lifecycle phases

These local **trust-tier** names are not theory lifecycle `T0--T2`, semantic
strata, project/product layers or a numeric privilege lattice. A tier crossing
never mints Mir authority.

| Trust tier | Responsibility | Required restraint |
|---|---|---|
| **T0 trusted Mir/Browser kernel** | checker, admission-policy enforcement, Mir semantic runtime base, and trusted adapters | implements and revalidates owner/grant rules; identity or trusted-code status alone is not authority |
| **T1 checked untrusted Mir package** | admitted checked package logic under declared capability/effect/resource bounds | no raw pointer/native call, ambient host access, direct store mutation, or authority minting |
| **T2 sandboxed external provider process** | typed provider request/result execution behind an adapter and isolation boundary | no ambient semantic state, grant minting, hidden retry, or direct mutation; a native executable with ambient host access is T3, not T2 |
| **T3 privileged native provider/plugin** | explicitly reviewed native integration needing broader host access | separate high-risk admission, least privilege, provenance, resource/data-access declaration, crash boundary and revocation; process isolation is preferred |
| **T4 host browser/engine/OS** | physical process, browser engine, renderer, device, storage and isolation substrate | substrate identity and session/certificate/process facts are evidence, never Mir grant or semantic ownership |

T0 enforces Mir policy; T4 supplies its substrate. An in-process T3 must narrow
its crash-containment/isolation claim rather than claim T2 equivalence.

## Contract matrix

Each contract records input, verdict, validation owner, authority, failure,
revocation/termination, observation/redaction and non-freeze.

### BND-010 Package → Browser Admission

- **Required input:** package/content identity and provenance; source/Core
  correspondence needed to parse, check, elaborate and verify; requested
  capabilities, effects, resources, data/device access and residual obligations.
- **Output/verdict:** typed admit or reject verdict bound to exact checked content,
  policy version and declared requests. Admission does not activate a locus.
- **Validation owner:** T0 admission kernel and checker under host policy.
- **Authority consequence:** none. Package name, origin, publisher, signature,
  checksum, successful verification and admission are provenance/evidence, not a
  membership, capability, witness, owner or mutation grant.
- **Typed failure:** malformed/uncheckable content, provenance mismatch, residual
  policy rejection, undeclared/overbroad capability/effect/resource request, or
  unavailable enforcement.
- **Revocation/termination:** verdicts and running instances are revocable by
  policy/content identity. Revocation prevents later activation/use; it does not
  rewrite already-authorized semantic history.
- **Observation/redaction:** diagnostics reveal only observer-authorized source,
  provenance and reason references; no credential, witness payload or private
  package data is emitted.
- **Non-freeze:** package container, origin/URL, signature, manifest field names,
  module system, marketplace and permission-dialog UX remain unresolved.

Primary falsifier: signature/package/origin/check/admission directly creates a
grant or activates a participant.

### BND-011 Browser Runtime → Mirrorea Fabric

- **Required input:** exact admitted package-instance identity, checked artifact
  identity, explicit locus-allocation request, requested membership/capability,
  storage-namespace request, and lifecycle operation (activate, update, revoke,
  terminate).
- **Output/verdict:** independently accepted or rejected allocation, namespace,
  membership and capability decisions, plus typed lifecycle result/failure.
- **Validation owner:** the Mir semantic owner/runtime at the use site; the
  Browser and Mirrorea placement/transport fabric carry but do not originate grants.
- **Authority consequence:** only an explicit Mir grant/witness lineage can
  authorize use. Admission, browser identity, process, session, endpoint,
  reconnect and deployment mapping cannot do so.
- **Typed failure:** artifact/provenance mismatch, wrong locus/owner, missing,
  stale or revoked grant, policy/resource refusal, lifecycle conflict, or no route.
- **Revocation/termination:** update, reconnect or new process incarnation must
  not resurrect a revoked/stale lineage. Termination stops future effects and
  releases the instance's resources without reporting semantic success.
- **Observation/redaction:** expose instance/artifact/locus and typed reason refs
  only within observer authority; transport/session facts remain evidence.
- **Non-freeze:** process topology, deployment syntax, membership protocol,
  storage layout and public lifecycle API remain unresolved.

Primary falsifier: admission becomes a grant, or reconnect/update reactivates
a revoked membership/capability lineage.

### BND-007 Runtime/Projection → View

- **Required input:** observer principal, observer-safe state/relation projection,
  visibility/redaction decision, presentation frontier and semantic version,
  with source/reason references safe for that observer.
- **Output/verdict:** a presentation input or typed projection failure. View may
  transform only the admitted observer-safe representation.
- **Validation owner:** Mir runtime/projection validates semantic meaning,
  observer authority, visibility and redaction before export.
- **Authority consequence:** View owns no authoritative domain semantics,
  mutation authority, membership/capability grant, persistent-state source,
  semantic fallback lineage, patch admission or information-flow policy.
- **Typed failure:** projection unavailable/stale, visibility denial, redaction
  mismatch, unsupported presentation version or resource refusal.
- **Revocation/termination:** later revocation invalidates future projection and
  input use. View cache termination cannot mutate or roll back Mir state.
- **Observation/redaction:** redaction is monotone. Private state/source, raw
  witness/capability material and hidden relations never become presentation data.
- **Non-freeze:** View schema, renderer, coordinate system, frame protocol,
  browser/engine choice and public projection API remain unresolved.

Presentation-local computation may include coordinate conversion, interpolation,
camera, inverse kinematics (IK), animation, culling, late latching, cosmetic
physics, audio spatialization, cache and frame-local relation evaluation. It must
not reinterpret domain law, weaken redaction or change semantic state.

Primary falsifier: View recomputes authoritative domain meaning, writes a store,
or reveals data excluded by the Mir projection.

### BND-012 View → Renderer/Engine

- **Required input:** already-redacted presentation representation plus explicit
  frame/device/resource parameters.
- **Output/verdict:** frame/audio/presentation result or typed renderer failure.
- **Validation owner:** trusted View adapter validates the presentation boundary;
  T4 engine/renderer enforces its host resource boundary.
- **Authority consequence:** engine state, pose, cache, frame order and device
  identity are never semantic ownership, grant or persistent-state authority.
- **Typed failure:** unsupported representation/device, resource exhaustion,
  engine rejection/crash, or stale presentation frontier.
- **Revocation/termination:** stop frame/device work and discard presentation
  cache; do not convert termination into a Mir semantic transition.
- **Observation/redaction:** renderer receives no more than BND-007 admitted;
  engine logs follow the same observer/redaction policy.
- **Non-freeze:** Unity/Unreal/browser/native choice and plugin/render ABI remain
  unresolved.

Primary falsifier: renderer cache/pose becomes semantic truth or mutation authority.

### BND-013 Input → Mir Command

- **Required input:** device/input occurrence, principal, requested typed command,
  handler identity, capability/witness reference and relevant presentation frontier.
- **Output/verdict:** semantic transition/result or typed rejection/failure after
  Mir-side validation.
- **Validation owner:** T0 Mir runtime and the semantic owner/handler; View and
  renderer only acquire and encode candidate input.
- **Authority consequence:** device presence, focus, event source, session or
  input data does not grant mutation authority.
- **Typed failure:** unknown command/handler, stale frontier, invalid payload,
  missing/stale/revoked capability, policy denial or semantic handler failure.
- **Revocation/termination:** revocation before admission rejects the command;
  termination drops unadmitted input without silently mutating state.
- **Observation/redaction:** input and failures are observed only under declared
  typed policy; sensitive device data is minimized/redacted.
- **Non-freeze:** input event schema, device API, command grammar and UI remain
  unresolved.

Primary falsifier: raw input writes semantic storage or bypasses capability.

### BND-014 Typed Effect → Provider

- **Required input:** declared typed effect and failure row, principal/capability,
  policy/resource request, request identity and observer-safe provenance.
- **Output/verdict:** validated typed result/failure associated with the request;
  any semantic transition is subsequently performed by Mir.
- **Validation owner:** trusted adapter/T0 policy gate before and after the T2
  provider; the semantic owner validates the resulting transition.
- **Authority consequence:** provider identity, connection and result cannot mint
  a grant, change the owner, or directly become semantic state.
- **Typed failure:** admission/policy/resource denial, timeout, provider crash,
  malformed result, provenance mismatch, unavailable service or explicit
  ambiguous occurrence. Retry is never hidden.
- **Revocation/termination:** revoke future requests and terminate bounded work;
  an in-flight occurrence remains explicit rather than becoming false success.
- **Observation/redaction:** request/result/failure logs contain only authorized
  fields and reason refs; credentials, raw witnesses and private payloads are not
  observer output.
- **Non-freeze:** provider protocol, IPC/FFI, wire/codec, service discovery and
  public provider API remain unresolved.

Primary falsifier: provider output mutates state, mints authority or hides failure/retry.

### BND-015 Privileged Native Plugin / Raw FFI

- **Required input:** explicit privileged admission, exact publisher/provenance,
  least-privilege capability, data/device/resource-access declaration, crash and
  revocation policy, and a named trusted adapter consumer.
- **Output/verdict:** separately admitted T3 integration or typed denial; calls and
  results still cross a validated adapter boundary.
- **Validation owner:** T0 privileged-plugin policy owner plus T4 isolation and
  resource enforcement. Ordinary T1 packages cannot select this route.
- **Authority consequence:** native code and publisher identity do not mint Mir
  authority. T3 is not entitled to the T1/T2 safety claim.
- **Typed failure:** provenance/policy denial, unavailable isolation, native crash,
  invalid result, resource/data-access violation or revocation.
- **Revocation/termination:** support explicit disable/kill and resource cleanup;
  prefer process isolation. In-process use must state its narrower crash boundary.
- **Observation/redaction:** native diagnostics are treated as untrusted input and
  filtered before observer output; secret/private memory is never debug payload.
- **Non-freeze:** native ABI, plugin format, publisher mechanism, process protocol
  and engine integration remain unresolved.

Primary falsifier: T1/T2 reaches raw native access, or T3 claims T1/T2 isolation.

### BND-016 Resource / Sandbox Envelope

- **Required input:** per-instance CPU/time, memory, storage namespace/quota,
  effect/network rate, device/data access, loop/allocation and observation budgets,
  plus termination policy.
- **Output/verdict:** enforceable activation envelope or typed fail-closed denial;
  runtime usage and violations are typed evidence.
- **Validation owner:** T0 admission/runtime policy with T4 enforcement substrate;
  T2/T3 adapters also enforce their narrower provider limits.
- **Authority consequence:** quota, budget, process isolation, device possession
  and scheduler priority never create semantic authority.
- **Typed failure:** CPU/time or memory exhaustion, storage quota/namespace fault,
  rate/backpressure, prohibited device/data use, nontermination/allocation abuse,
  observation abuse, or enforcement unavailable.
- **Revocation/termination:** bounded cancellation/kill, namespace/resource cleanup
  and future-request denial. Termination is not semantic success and must not
  resurrect stale state or authority.
- **Observation/redaction:** usage evidence is typed, rate-limited and redacted;
  observation itself is budgeted so telemetry cannot bypass information flow.
- **Non-freeze:** sandbox runtime, scheduler, quota units, storage engine, process
  model, operating system mechanism and permission UI remain unresolved.

Primary falsifier: unenforceable limits still activate, or termination/quota/device
possession becomes semantic success or grant.

## Cross-boundary invariants

1. Admission/provenance/deployment/transport never replace a Mir owner/grant/witness.
2. Projection exports observer-safe data; reverse input proposes a typed command or
   effect that is revalidated before transition.
3. T1 has no raw FFI; T2 is typed/sandboxed; T3 is privileged/revocable. None owns state.
4. Resource/observation failures fail closed; crash/retry/termination are not success.
5. I3 preserves but does not implement/freeze these inactive-I5 boundaries.

All cross-edge security rules are normative in `arch/08-browser-host-security-invariants`.
