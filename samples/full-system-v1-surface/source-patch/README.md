# Surface Mir Source Patch Samples

This family is the P-SURF-06 evidence root for source patch hot-plug. Source
patches enter through parse / typecheck / elaborate / compatibility / admission,
then produce HotPlugRequest, HotPlugVerdict, and activation_cut rows. They are
not direct eval.

Rows:

- `PATCH-01`: visible state patch is accepted and emits an activation cut.
- `PATCH-02`: undeclared generated failure row is rejected without mutation.
- `PATCH-03`: self-grant of ServerAuthority is rejected without mutation.
- `PATCH-04`: patch lifecycle/devtools evidence is accepted.

Non-claims:

- no final hot-plug ABI.
- no distributed durable migration.
- no production patch registry/signing workflow.
- no arbitrary native/WASM execution through patches.
