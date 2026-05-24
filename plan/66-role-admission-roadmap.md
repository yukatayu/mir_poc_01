# plan/66 — role admission roadmap

## purpose

This document is repository memory for
`specs/41-role-admission-and-capability-grant.md`.

It records the Surface Mir rule that role claim is separate from authority.

Current package status: `P-SURF-05` is closed as a report-level role
admission / capability grant evidence floor. The promoted next package for the
Surface line is `P-SURF-06 source patch hot-plug`.

## current decision

Decided:

- role claim is not authority.
- authority is a membership / capability / witness grant from an admission
  locus.
- spoofed role claims do not grant server writes.
- stale membership epoch / incarnation rejects messages.
- optional package/runtime hash binding is report metadata unless a later spec
  makes stronger attestation claims.

Not decided:

- production identity provider.
- hardware attestation.
- WAN/federation admission.
- final public auth schema.

## package order

| Package | Role | Completion gate |
|---|---|---|
| `P-SURF-00B` | docs/spec rebaseline | role/admission boundary and roadmap exist |
| `P-SURF-05` | role admission implementation | closed: role claim, admission request, grant, witness, stale rejection, and hash metadata rows exist |
| `P-SURF-06` | source patch hot-plug | next: preserve admission evidence across patch admission without claiming runtime identity lifecycle |
| `P-SURF-07` | operational source rows | WorldCore / MembershipChat / role-admission roots exercise admission |
| `P-SURF-08` | devtools | claimed role, admitted role, grants, stale rejects, and spoof attempts are visible |

## planned rows

- `ROLE-01` BrowserClient join accepted; grant-backed write accepted.
- `ROLE-02` role claim without grant cannot write server state.
- `ROLE-03` stale membership message and post-stale write rejected.
- `ROLE-04` package/runtime hash binding appears as optional report metadata.

## validation anchors

Current anchors:

```bash
python3 scripts/surface_mir_samples.py run ROLE-01 --format json
python3 scripts/surface_mir_samples.py run ROLE-02 --format json
python3 scripts/surface_mir_samples.py run ROLE-03 --format json
python3 scripts/surface_mir_samples.py run ROLE-04 --format json
cargo test -p mir-semantics --test role_admission_capability_grant -- --nocapture
```

P-SURF-05 actualized the role-admission root at
`samples/full-system-v1-surface/role-admission/`; `ROLE-01` includes a
grant-backed accepted World-owned indexed-state write, while `ROLE-02` keeps
the missing-grant negative row separate.

## stop lines

- do not collapse auth / membership / capability / witness into transport.
- do not treat role claim as authority.
- do not treat signature/provenance as semantic safety.
