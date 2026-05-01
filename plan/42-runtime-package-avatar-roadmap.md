# plan/42 — runtime package avatar roadmap

## purpose

この文書は、Alpha-0 line における
`specs/16-runtime-package-adapter-hotplug.md`
の repository-memory roadmap を置く。

ここで保持するのは avatar non-core / runtime package / native trust policy の current line であり、
full compatibility or final public avatar API ではない。

## current repo state

- existing repo already fixes:
  - hot-plug request/verdict carrier
  - runtime-private engine-state narrow floor
  - avatar representative slice as sample/runtime evidence
  - `FAIRY-05` residual planned family
- Alpha-0 avatar/runtime-package sample families are now organized under `samples/alpha/avatar-runtime/` and `samples/alpha/hotplug-runtime/`

## decisions mirrored from specs/16

- avatar format is not Mir core primitive
- runtime package admission uses typed hot-plug checks
- signature proves provenance only
- native package needs capability/effect/sandbox/resource policy
- unsupported runtime fallback is monotone and visible

## runtime package manifest roadmap

### first safe cut

- manifest field inventory in docs/spec
- planned sample skeletons for placeholder/custom/runtime-unavailable/native-reject rows
- no final public ABI naming

### later executable cut

- manifest validator / admission checker
- runtime-private package admission carrier
- limited sandboxed external-process skeleton if honestly scoped

## avatar adapter skeleton roadmap

planned families to keep explicit:

- placeholder avatar runtime
- custom Mir avatar runtime
- VRM adapter skeleton
- VRChat compatibility adapter skeleton
- Unity adapter remains conceptual comparison only in the current package; no dedicated Alpha-0 sample row is claimed yet

all remain adapter/runtime-package concern, not core semantics.

## native trust policy roadmap

required policy rows:

- trusted signer policy
- capability manifest
- effect-row containment
- resource limit
- sandbox / isolation
- crash containment
- revocation story
- audit + redaction boundary
- revoked / stale signature reject coverage

## sample roadmap

sample family roots:

- `samples/alpha/avatar-runtime/`
- `samples/alpha/hotplug-runtime/`

must keep visible:

- placeholder accepted
- custom Mir runtime accepted
- unsupported shader fallback
- unsigned native reject
- signed over-capability reject
- revoked/stale-signed reject
- runtime unavailable placeholder fallback
- active dependent detach deferred/rejected

## deferred questions

- final avatar API
- final shader model
- native execution beyond skeleton policy
- renderer / physics / IK detail
- full VRM / VRChat / Unity compatibility

## next package

- after checker / cut packages:
  widen runtime-private hot-plug and avatar sample bridge without promoting any concrete format to core
