# 08 — Role Admission / Capability Grant

## 1. Problem

A participant may claim:

```text
I am a browser client.
```

This must not grant authority by itself.

## 2. Surface syntax

```mir
role BrowserClient {
  supports renderer.pose_v1
  supports devtools.observer_safe
}

principal self

BrowserClient[self] {
  when start {
    join World as BrowserClient via WorldAdmission
  }
}
```

## 3. Core expansion

```text
HostRoleClaim(BrowserClient)
PrincipalClaim(self)
AdmissionRequest(self, BrowserClient, World)
WorldAdmission checks request
MembershipGrant(self, epoch, incarnation)
CapabilityGrant(...)
AdmissionWitness
```

## 4. Admission authority

Root / parent / admission place decides.

```mir
WorldAdmission {
  admit participant(req: JoinRequest) by WorldRoot
    requires AdmitParticipant
  {
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

This syntax is alpha candidate; exact parser may use equivalent constructs.

## 5. Anti-spoofing principle

```text
role claim != authority
capability grant == authority
```

If a client claims `ServerRole` but lacks `ServerAuthority`, server-only operations are rejected.

## 6. Optional hardening

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

Alpha does not require hardware attestation.

## 7. Membership freshness

Messages carry:

```text
principal
membership_epoch
member_incarnation
```

Stale messages are rejected.

## 8. Leave / rejoin

Leave retires current incarnation.
Rejoin uses a new incarnation.
Old messages do not recreate participant.

## 9. Devtools

Show:

- claimed role.
- admitted role.
- granted capabilities.
- membership epoch/incarnation.
- rejected spoof attempts.
