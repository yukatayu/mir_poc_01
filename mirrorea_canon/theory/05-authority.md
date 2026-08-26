---
id: theory/05-authority
status: L1-fixed
maturity: draft
depends_on: [theory/01-mircore-v0, adr/ADR-0005, adr/ADR-0028]
summary: role claim / admission / capability系譜 / witness / incarnation / bounded live-generation visibilityの権限代数とTHM-004。
open_items: [OPEN-017]
---

# 05 — Authority

## Claims vs grants

```text
role claim   ≠ authority
capability grant == authority
```

`join ℓ as R via ℓₐ` means AdmissionRequest(π, R, ℓ) judged at ℓₐ. Carriers:

```text
AdmissionRequest = { principal, claimed_role, target, requested_caps, spans }
AdmissionVerdict = { accepted|rejected|deferred, membership_epoch,
                     member_incarnation, granted_capabilities,
                     admission_witness_ref, failure_or_reason }
```

Post-admission messages carry (principal, epoch, incarnation, capability_refs,
witness_refs where required); stale messages are rejected.

## Capability-ref validation

A capref ρ is not a bearer token. Valid use requires lineage match on all of:
originating verdict, principal, admitted role, target locus/world, membership
epoch, member incarnation, admission witness ref (where required), grant
policy version. Copied / replayed / stale-incarnation / wrong-target /
severed-provenance refs are rejected (fail-closed at [E-SERVE]).

## Non-authority list (ADR-0005, restated as axioms)

Indexed key is not authority ("Alice is the key of player[Alice]" ⇏ Alice may
write it). Locus name, apparent location, runtime kind, transport session,
provider name, package name, engine brand: none confer authority. Signature is
provenance only; native/package admission additionally needs capability
manifest, effect/failure containment, resource limits, sandboxing, revocation
story, audit boundary.

## Lifecycle

join: epoch+1, new incarnation, entries initialized. leave: epoch+1,
incarnation retired, entries tombstoned (not dropped). rejoin: **new**
incarnation; old grants/witnesses do not recreate the participant. Revocation
is monotone unless a new epoch/evidence is issued. Load/rollback never
resurrects stale authority (with theory/04).

## Bounded live-generation visibility

ADR-0028 refines the selected SYS-1 runtime fragment with immutable authority
generations. Only the retained M9 publisher for the same admitted program may
produce a production successor. It performs revocation through M9, translates
the complete authority inventory, preserves unrelated admitted lineages, and
retains every earlier revocation tombstone. The successor is visible to the
kernel only after its owner runtime acknowledges installation.

Generation number, mailbox order, worker identity, transport/session data,
and a reply/receipt remain non-authority. They may witness causal order but
cannot grant a capability or repair a revoked lineage. A use whose lineage is
revoked in the acknowledged current generation rejects without state
mutation; reacquisition still requires new admitted evidence under the
existing lifecycle rule.

## THM-004 — Authority soundness

```text
Every state-mutation occurrence at owner ℓ in a well-formed trace is causally
preceded by a grant whose lineage validates the mutating use for
(principal, role, target, epoch, incarnation), or is owner-local under ℓ's
own declared transitions.
```

(OBL-015 statement, OBL-016 proof.) Anti-spoofing corollary: claiming
ServerRole without ServerAuthority leaves server-only operations rejected
regardless of transport/runtime/package identity.

OPEN-017: attestation (package/runtime hash binding) stays report metadata,
not semantic proof; production identity providers are out of scope until
PHASE-I6.
