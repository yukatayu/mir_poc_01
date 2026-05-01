# 07 — Runtime package, avatar adapter, and browser-like compatibility model

Mirror normative content into `specs/16-runtime-package-adapter-hotplug.md` and roadmap memory into `plan/42-runtime-package-avatar-roadmap.md`.

## 1. Core decision

Avatar systems must not become Mir core primitives. VRChat compatibility, VRM, Unity-derived behavior, custom user-defined avatar formats, shaders, and native libraries are handled through runtime packages/adapters admitted by Mirrorea hot-plug policy.

## 2. Browser-like interpretation

Mirrorea Spaces should eventually behave like a browser for virtual worlds:

- A client loads a world package.
- The world package declares required runtime packages, effects, capabilities, and adapters.
- The client/runtime checks signatures, capabilities, trust policy, sandbox requirements, and compatibility.
- Accepted packages attach to well-defined AttachPoints.
- Rejected/unsupported packages fall back to placeholders, simplified representations, or explicit rejection according to contract.

## 3. Abstract avatar role

Core should define abstract roles, not concrete formats.

Candidate roles:

```text
EmbodiedPresence
Renderable
Animatable
InputConsumer
InteractionTarget
AttachmentProvider
AnchorProvider
ExpressionProvider
EffectProvider
InspectableObject
NetworkedStateParticipant
```

An avatar runtime package can implement some or all of these roles.

## 4. RuntimePackageManifest

A runtime package manifest should include, at minimum:

```text
package_id
package_kind
version
provider
signature/provenance fields
required_host_capabilities
provided_roles
provided_effects
required_effects
failure_row
resource_budget
observation_labels
retention_policy
sandbox_requirement
native_code_policy
adapter_targets
compatibility_claims
fallback_representation
```

Do not make this final public ABI in the first package. It is alpha-local until stabilized.

## 5. Hot-plug flow

Runtime package admission should use the same broad hot-plug discipline:

```text
HotPlugRequest
  attachpoint_ref
  patch/package_ref
  operation_kind
  requesting_principal
  auth_evidence_ref
  capability_refs
  witness_refs

Checks
  manifest well-formed
  signature/provenance policy
  capability/effect row containment
  sandbox/native policy
  role compatibility
  host support
  membership freshness
  authorization

HotPlugVerdict
  accepted | rejected | deferred
  compatibility_reason_refs
  authorization_reason_refs
  membership_freshness_reason_refs

Activation
  activation cut
  package provider registration
  Place/object graph update
  debug/audit event
```

## 6. Native binary policy

A signed native binary is not automatically safe. Signature proves origin/provenance only.

Admission requires:

- trusted signer policy
- explicit capability manifest
- effect row containment
- resource limits
- sandbox/process isolation or equivalent containment
- crash containment
- revocation/versioning story
- telemetry/redaction boundary
- audit event

Alpha recommendation:

- Do not execute native binaries in alpha.
- Add samples that reject unsigned native packages and over-capability signed packages.
- Optionally add a sandboxed external-process skeleton that does not claim full safety.

## 7. Unity / VRChat / VRM compatibility

Treat these as adapters/runtime packages.

### VRM adapter skeleton

- Parses or declares VRM-like avatar role support.
- Exposes abstract roles.
- Does not become core.
- Unsupported shaders/materials fall back.

### VRChat compatibility adapter skeleton

- Represents compatibility intent only.
- Does not claim full VRChat SDK compatibility unless implemented.
- Maps supported features to abstract roles/effects.

### Unity adapter skeleton

- Treats Unity-derived behavior as opaque or semi-opaque adapter.
- Must declare effects/capabilities.
- Unsupported/unsafe behavior is rejected or isolated.

## 8. Fallback behavior

If runtime support is absent or rejected:

```text
full runtime available -> full representation
runtime unavailable -> placeholder/skeleton
shader unsupported -> fallback material
native code untrusted -> static representation or reject
asset missing -> placeholder
behavior unsupported -> local-only representation or reject
```

Fallback must be monotone and visible. It must not silently claim full functionality.

## 9. Required samples

Minimum:

- placeholder avatar runtime accepted
- custom Mir avatar runtime accepted
- VRM adapter skeleton accepted as planned/limited
- VRChat compatibility adapter skeleton accepted as planned/limited
- unsupported shader fallback
- unsigned native provider reject
- signed but over-capability native provider reject
- sandboxed external-process skeleton accepted only with limited capabilities
- runtime unavailable fallback to placeholder
- package detach with active dependent object rejected/deferred

## 10. Deferred

- final avatar API
- shader model
- full VRM compatibility
- full VRChat compatibility
- Unity runtime integration
- native binary production execution
- asset pipeline
- renderer choice
- physics/IK/facial animation details
