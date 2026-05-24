# Surface Mir Role Admission Samples

This family is the P-SURF-05 evidence root for role claim, admission request,
capability grant, admission witness, stale membership rejection, and optional
package/runtime hash metadata.

Rows:

- `ROLE-01`: BrowserClient join is accepted through `WorldAdmission`, and the
  resulting grant authorizes a World-owned indexed-state write.
- `ROLE-02`: role claim alone cannot write World-owned indexed state.
- `ROLE-03`: stale membership message and post-stale write are rejected.
- `ROLE-04`: package/runtime hash binding is metadata, not semantic safety proof.

Non-claims:

- no production identity provider.
- no hardware attestation.
- no WAN/federation admission.
- no runtime membership lifecycle or rejoin execution.
- no final public auth schema.
