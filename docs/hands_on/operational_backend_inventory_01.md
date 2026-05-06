# Operational Backend Inventory 01

この guide は、operational product sample suite に対する
**current backend feasibility inventory** を最短で確認する入口です。

ここでいう backend inventory は、実装済み backend ではありません。
current line で actualize 済みなのは `native host launch bundle` だけです。
WASM client host と direct LLVM/native projection backend は、現時点では
docs-first comparison inventory としてだけ扱います。

## まず確認する current executable path

```bash
bundle_dir=$(mktemp -d /tmp/mirrorea-ops-bundle-XXXXXX)
cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/operational/sugoroku-world --out "$bundle_dir" --format json
sh "$bundle_dir/run.sh" check
sh "$bundle_dir/run.sh" view
python3 scripts/operational_product_samples.py check-all --format json
```

この path が示すのは次だけです。

- current native output は `native host launch bundle`
- bundle の中身は compiled Rust CLI、versioned package files、devtools assets、reports、manifest、run script
- package-native execution や direct Mir-to-machine-code は claim しない

## backend comparison matrix

| Option family | Current status | What exists now | What must stay explicit |
|---|---|---|---|
| host launch bundle | actualized | `build-native-bundle`, `run.sh`, compiled Rust CLI, versioned package files, observer-safe devtools assets, reports | not package-native execution, not direct LLVM codegen, not signature-is-safety |
| WASM client host | planned inventory only | projection intent, packet boundary inventory, FFI boundary inventory, future host-kind wording | no browser/runtime implementation, no final client ABI, no final placement optimizer |
| LLVM/native projection backend | planned inventory only | projection profile, packet/FFI boundary requirements, backend stop-line wording | no direct Mir-to-machine-code, no emitted server/client binaries, no final optimization pipeline |

## current recommendation

For alpha-1 operational work:

1. use `native host launch bundle` as the only executable backend-adjacent path
2. keep `projection.profile.json` as target intent / packet / FFI inventory
3. describe WASM and LLVM only as future boundary inventory

## feasibility requirements before a later reopen

Any later WASM or LLVM line must preserve:

- checked package / projection contract preservation
- typed packet boundary
- typed FFI / adapter boundary
- auth / membership / capability / witness lanes not bypassed by codegen
- observer-safe devtools exportability of the resulting boundary
- `NativeExecutionPolicy` / provenance / sandbox policy wording

If these are not explicit, the line is still inventory-only.

## non-claims

- no actual WASM runtime
- no direct LLVM backend
- no emitted server/client native binaries
- no final server/client split
- no final public ABI / SDK
- no arbitrary native package execution

## related

- `operational_product_sample_01.md`
- `operational_package_authoring_01.md`
- `compiler_backend_llvm_preparation_01.md`
- `../research_abstract/operational_backend_inventory_01.md`
- `../../specs/26-operational-product-sample-suite.md`
- `../../plan/23-compiler-backend-llvm-guardrail-roadmap.md`
- `../../plan/51-operational-product-sample-roadmap.md`
