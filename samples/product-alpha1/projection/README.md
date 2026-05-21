# samples/product-alpha1/projection

This root is the planned-only projection boundary inventory for `P-PROJ-01`.

## Current Status

- The root exists as scaffold actualization only.
- All rows are `planned_only`.
- Server/client target manifests, packet schemas, FFI schemas, and provider compatibility rows are inventory only.
- Current executable/native truth remains `host_launch_bundle_only`.
- The compatibility inventory includes one accepted and one rejected `ManifestProviderCompatibility` row as machine-readable planned evidence.

## Current Validation Anchor

Use the helper classification commands:

```bash
python3 -m unittest scripts.tests.test_projection_boundary_samples
python3 scripts/projection_boundary_samples.py matrix --format json
python3 scripts/projection_boundary_samples.py check-all --format json
python3 scripts/projection_boundary_samples.py run proj-01-server-client-target-manifest --format json
```

`run proj-01-server-client-target-manifest` must reject as `planned_only` until a later projection realization package exists.

## Rows

- `proj-01-server-client-target-manifest`
- `proj-01-packet-boundary-schema`
- `proj-01-ffi-boundary-schema`
- `proj-01-manifest-provider-compatibility`

Representative JSON files in this root are inventory artifacts only. They are not generated projection IR, not executable server/client output, and not final backend admission.
