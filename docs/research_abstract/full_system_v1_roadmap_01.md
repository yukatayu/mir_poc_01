# Full System V1 roadmap summary

この文書は `P-FS-00 full-system-v1-roadmap-rebaseline` の短い summary である。

規範判断の正本は `../../specs/33-full-system-v1-scope.md` から `../../specs/38-engine-provider-admission.md`、repository memory は `../../plan/58-full-system-v1-roadmap.md` から `../../plan/63-engine-provider-roadmap.md` に置く。

## 現在地

repo は bounded Product Alpha-1 workflow、canonical operational product sample suite、Mir-owned computation first-floor evidence、PoseGraph helper evidence、projection/backend inventory、engine/provider inventory を持つ。

ただしこれは final product ではない。`package.mir.json` は alpha compatibility / package artifact であり、本来の semantic source は Mir source files へ移す。

## Full System V1 の狙い

Full System V1 は次の流れを source-first に揃える roadmap である。

```text
.mir source files
  -> parser / AST
  -> typed IR
  -> checker / residual proof-model obligations
  -> interpreter and runtime session
  -> projection IR / deployment plan
  -> server / client / adapter artifacts
  -> provider boundary and devtools evidence
```

## Package order

`P-FS-00` は docs rebaseline、`P-MIR-01` は parser floor、`P-MIR-02` は typed checker floor を actualize した。次に進む package は `P-MIR-03 computational interpreter` である。

大きな順序は次の通り。

- `FS-01`: textual Mir grammar MVP.
- `FS-02`: typed IR and checker.
- `FS-03`: Mir-owned computational interpreter.
- `FS-04`: effectful Mir integration.
- `FS-05`: PoseGraph runtime.
- `FS-06`: projection IR.
- `FS-07`: local server/client split MVP.
- `FS-08`: engine/provider admission MVP.
- `FS-09`: devtools full alpha panels.
- `FS-10`: native host bundle plus optional backend gate.
- `FS-11`: release check and clean clone guide.

## Non-claims

この roadmap は final public grammar、final ABI / SDK、Rust-level language completion、LLVM/native codegen、production WAN/federation、distributed durable save/load R3/R4、arbitrary native package execution、arbitrary WASM execution、Unity / Unreal / WASM / native provider execution completion を主張しない。
