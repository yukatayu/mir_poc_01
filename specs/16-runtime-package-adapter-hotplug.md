# 16 — Runtime Package / Adapter / Hot-plug（Alpha-0）

## role

この文書は、Mirrorea Spaces alpha line における
**runtime package / avatar adapter / native trust / hot-plug admission** の
alpha-local 規範を置く。

- avatar / VRM / VRChat / Unity concept を Mir core primitive にしない
- runtime package admission を typed hot-plug discipline に載せる
- signature-only safety claim を禁止する

## decision level

- `L1`
  - avatar support is non-core
  - runtime package admission is hot-plug checked
  - signature proves provenance, not semantic safety
- `L2`
  - current alpha manifest shape
  - abstract avatar/object role inventory
  - native sandbox/trust policy minimum

## runtime package concept

runtime package は、
role / effect / adapter / fallback representation を持つ hot-pluggable unit として読む。

examples:

- placeholder avatar runtime
- custom Mir avatar runtime
- VRM adapter skeleton
- VRChat compatibility adapter skeleton
- Unity-derived behavior adapter
- redaction / debug / policy helper package

## avatar non-core decision

current alpha-local core は abstract role だけを知る。
concrete avatar formats are adapter/runtime-package concern.

do not encode as core primitive:

- VRM
- VRChat SDK concept
- Unity GameObject/Prefab concept
- shader/material family
- engine-specific animation graph

## abstract roles

current alpha-local abstract role inventory は少なくとも次を含む。

- `EmbodiedPresence`
- `Renderable`
- `Animatable`
- `InputConsumer`
- `InteractionTarget`
- `AttachmentProvider`
- `AnchorProvider`
- `ExpressionProvider`
- `EffectProvider`
- `InspectableObject`
- `NetworkedStateParticipant`

## runtime package manifest alpha shape

current alpha-local manifest comparison は少なくとも次の field family を持つ。

```text
package_id
package_kind
version
provider
signature_or_provenance
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

exact public ABI / wire schema / final naming は deferred とする。

## hot-plug admission flow

runtime package admission は existing hot-plug discipline の widening として読む。

```text
HotPlugRequest
  -> well-formedness / provenance / capability / effect / sandbox / role checks
  -> HotPlugVerdict
  -> activation cut
  -> provider registration / graph update / audit event
```

minimum check families:

- manifest well-formedness
- signature/provenance policy
- capability/effect row containment
- sandbox/native policy
- role compatibility
- host support
- membership freshness
- authorization

## capability / effect / failure manifest rules

package / adapter admission では少なくとも次を守る。

- required capability must be declared
- provided role/effect must be declared
- undeclared effect widening is invalid
- undeclared failure widening is invalid
- observation labels / retention policy are explicit
- fallback representation is explicit

## native binary trust policy

signature is provenance only.
semantic safety needs additional policy.

minimum admissibility requirements:

- trusted signer policy
- explicit capability manifest
- effect-row containment
- resource limits
- sandbox / process isolation or equivalent containment
- crash containment
- revocation / versioning story
- telemetry / redaction boundary
- audit event

alpha-local recommendation:

- do not claim native execution as safe merely because it is signed
- skeleton-only sandboxed external-process path may exist
- unsigned native package reject sample is required

## adapter skeleton families

### VRM adapter skeleton

- exposes abstract avatar roles
- keeps unsupported shader/material behavior explicit
- does not become core semantics

### VRChat compatibility adapter skeleton

- compatibility intent only unless fully implemented
- maps supported features to abstract roles / effects
- unsupported features fall back or reject explicitly

### Unity adapter skeleton

- treats engine-specific behavior as adapter-managed concern
- must declare effects / capabilities / fallback
- unsafe opaque behavior is rejected or isolated

## unsupported runtime fallback

runtime unavailability or rejection must degrade monotonically and visibly.

examples:

- full runtime available -> full representation
- runtime unavailable -> placeholder / skeleton
- shader unsupported -> fallback material
- native code untrusted -> static representation or reject
- asset missing -> placeholder
- behavior unsupported -> local-only representation or reject

do not silently claim full functionality after fallback.

## package detach and active dependents

detach is not free-form deletion.

- active dependents may force reject or defer
- detach must respect guarded reference degradation and no dangling rule
- activation / detach boundary remains explicit
- durable migration / completed rollback / distributed activation ordering are separate later lines

## proof / checker obligations

future checker / proof line が preserve すべき obligation は次である。

1. no avatar format becomes core primitive
2. package manifest is well-formed
3. required capability/effect/failure rows are declared
4. signature is not treated as safety proof
5. sandbox / containment requirement is explicit for native package
6. fallback representation is monotone and visible
7. unsupported runtime does not silently overclaim
8. active dependent detach is reject/defer capable

## required sample references

current alpha-local required sample family は少なくとも次を含む。

- `AV-01` placeholder avatar runtime
- `AV-02` custom Mir avatar runtime
- `AV-03` VRM adapter skeleton
- `AV-04` VRChat compatibility adapter skeleton
- `AV-05` unsupported shader fallback
- `AV-06` untrusted native avatar rejected
- `AV-07` trusted native sandbox limited
- `AV-08` runtime unavailable placeholder
- `AV-09` adapter undeclared effect rejected
- `AV-10` package detach active avatar deferred
- `HP-11` unsigned native package rejected
- `HP-12` signed over-capability package rejected
- `HP-15` revoked/stale-signed package rejected

sample inventory の current repository memory は
`plan/42-runtime-package-avatar-roadmap.md` と
`samples/alpha/avatar-runtime/` / `samples/alpha/hotplug-runtime/` を参照する。

## deferred

この spec で intentionally deferred に残すのは次である。

- final avatar API
- final shader/material model
- full VRM compatibility
- full VRChat compatibility
- Unity runtime integration
- native binary production execution
- renderer choice
- physics / IK / facial animation detail

## stop line

- avatar format / engine concept を Mir core primitive と書かない
- signature alone を safety proof と書かない
- limited adapter skeleton を full compatibility と書かない
- runtime-private carrier / preview naming を final public ABI と書かない
- this alpha-local spec を final public package/catalog/API freeze と混同しない
