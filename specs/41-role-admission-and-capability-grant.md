# 41 — Role Admission and Capability Grant

## role

This document fixes the distinction between role claims, membership admission,
and authority-bearing capability grants in Surface Mir alpha.

Role claim is not authority. Authority is a capability grant issued by an
admission locus under explicit policy.

## decision level

- `L1`
  - role claim does not grant authority.
  - authority is a capability / membership / witness grant.
  - authentication, authorization, membership, capability, and witness must not
    collapse into transport metadata.
  - stale membership or stale incarnation does not resurrect authority.
- `L2`
  - alpha admission request / grant / witness carrier.
  - optional package/runtime hash binding fields.
  - devtools role/admission panel shape.

## role claim surface

```mir
role BrowserClient {
  supports renderer.pose_v1
  supports devtools.observer_safe
}

principal self

BrowserClient[self] {
  when start fails MissingCapability {
    join World as BrowserClient via WorldAdmission
  }
}
```

This source means:

```text
HostRoleClaim(BrowserClient)
PrincipalClaim(self)
AdmissionRequest(self, BrowserClient, World)
```

It does not mean:

```text
self has ServerAuthority
self may write World-owned state
self may bypass membership freshness
```

## admission authority

Admission is decided by a root / parent / admission place.

Candidate alpha surface:

```mir
WorldAdmission {
  when admit(req: JoinRequest) {
    require valid_principal(req.principal)
    require req.role == BrowserClient

    grant Member(World, req.principal)
    grant ObserveWorld(req.principal)
    grant SendRoomMessage(req.principal)

    publish participant_joined(req.principal)
      produces witness admission_witness
  }
}
```

Equivalent parser constructs are allowed in implementation packages if they
preserve the same semantic separation.

## Core expansion

The Core/admission expansion must carry:

```text
AdmissionRequest {
  principal,
  claimed_role,
  target_world_or_place,
  requested_capabilities,
  source_refs,
}

AdmissionVerdict {
  accepted | rejected | deferred,
  membership_epoch,
  member_incarnation,
  granted_capabilities,
  admission_witness_ref,
  failure_or_reason,
}
```

Messages after admission carry:

```text
principal
membership_epoch
member_incarnation
capability_refs
witness_refs when required
```

Stale messages are rejected.

## capability-ref validation

A `capability_ref` is not a bearer token by itself. It is valid only when its
grant lineage matches all of:

```text
originating AdmissionVerdict
principal
claimed/admitted role
target world or place
membership_epoch
member_incarnation
admission_witness_ref when required
grant policy version or source ref
```

Copied, replayed, stale-incarnation, wrong-target, or severed-provenance
capability refs are rejected. This inherits the provenance-preservation rule
from `specs/20-cut-save-load-semantics.md`.

## anti-spoofing rule

```text
role claim != authority
capability grant == authority
```

If a client claims `ServerRole` without `ServerAuthority`, server-only
operations reject. Transport connection, runtime kind, package name, or role
string does not change that result.

## optional hardening

Admission witness may bind:

```text
principal_id
role_claim
package_hash
runtime_hash
transport_session
membership_epoch
member_incarnation
```

Alpha does not require hardware attestation. If attestation is absent, docs and
reports must say so rather than treating package/runtime hash binding as a
semantic safety proof.

## leave / rejoin

Leave retires the current incarnation. Rejoin creates a new incarnation. Old
messages, witnesses, and grants do not recreate the participant.

Load / rollback must not resurrect stale membership or stale authority. This
inherits the cut/save/load rules from `specs/20-cut-save-load-semantics.md`.

## devtools requirements

Devtools must show:

- claimed role.
- admitted role.
- admission locus.
- granted capabilities.
- membership epoch / incarnation.
- witness refs without raw secret payloads.
- rejected spoof attempts.
- stale-message rejection rows.

## required alpha sample rows

- `ROLE-01`: BrowserClient join accepted through admission.
- `ROLE-02`: role claim without grant cannot write server-owned state.
- `ROLE-03`: stale membership message rejected.
- `ROLE-04`: optional package/runtime hash binding appears as report metadata,
  not semantic safety proof.

## non-claims

This document does not claim:

- final public auth schema.
- hardware attestation.
- production identity provider.
- WAN/federation admission completion.
- transport-owned authority.
