# alpha samples

この root は、Mirrorea Spaces Alpha-0 line の sample matrix scaffold を置く。

- active runnable root ではない
- current package では planned skeleton / expected-verdict sidecar のみを置く
- active runnable evidence は引き続き `samples/clean-near-end/` と related helpers にある
- `samples/not_implemented/` は pre-existing residual planned families を保持する legacy planned root として残す

## current families

- `lifetime-fallback/`
- `contract-variance/`
- `cut-save-load/`
- `local-runtime/`
- `layer-insertion/`
- `network-docker/`
- `hotplug-runtime/`
- `avatar-runtime/`
- `visualization/`
- `e2e/`

## validation anchor for the scaffold package

```bash
find samples/alpha -maxdepth 3 -type f | sort
```

## stop line

- do not mark these files active/runnable merely because they exist
- do not treat `.expected.json` sidecars as proof of implementation
- do not silently move existing `samples/not_implemented/` residual families into active roots
