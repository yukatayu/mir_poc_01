# plan/66 — role admission roadmap

## purpose

This document is repository memory for
`specs/41-role-admission-and-capability-grant.md`.

It records the Surface Mir rule that role claim is separate from authority.

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
| `P-SURF-05` | role admission implementation | role claim, admission request, grant, witness, and stale rejection rows exist |
| `P-SURF-07` | operational source rows | WorldCore / MembershipChat / role-admission roots exercise admission |
| `P-SURF-08` | devtools | claimed role, admitted role, grants, stale rejects, and spoof attempts are visible |

## planned rows

- `ROLE-01` BrowserClient join accepted.
- `ROLE-02` role claim without grant cannot write server state.
- `ROLE-03` stale membership message rejected.
- `ROLE-04` package/runtime hash binding appears as optional report metadata.

## validation anchors

Future anchors:

```bash
python3 scripts/surface_mir_samples.py run ROLE-01 --format json
python3 scripts/surface_mir_samples.py run ROLE-02 --format json
python3 scripts/surface_mir_samples.py run ROLE-03 --format json
```

## stop lines

- do not collapse auth / membership / capability / witness into transport.
- do not treat role claim as authority.
- do not treat signature/provenance as semantic safety.
