# Full System V1 roadmap summary

この文書は `P-FS-00 full-system-v1-roadmap-rebaseline` の短い summary である。

規範判断の正本は `../../specs/33-full-system-v1-scope.md` から `../../specs/38-engine-provider-admission.md`、repository memory は `../../plan/58-full-system-v1-roadmap.md` から `../../plan/63-engine-provider-roadmap.md` に置く。

## 現在地

repo は bounded Product Alpha-1 workflow、canonical operational product sample suite、Mir-owned computation first-floor evidence、bounded source-first WorldCore / MembershipChat / Sugoroku / Portal / TwoShard / Gradient operational evidence、bounded source-first PoseGraph runtime/save-load/devtools evidence、bounded projection IR + boundary-schema first-floor evidence、bounded same-binary local role-split first-floor evidence、bounded provider-admission first-floor evidence、bounded renderer pose backend first-floor evidence、product-alpha projection inventory、product-alpha engine/provider inventory を持つ。

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

`P-FS-00` は docs rebaseline、`P-MIR-01` は parser floor、`P-MIR-02` は typed checker floor、`P-MIR-03` は pure interpreter floor、`P-MIR-04` は bounded effectful runtime floor、`P-POSE-03` は bounded runtime PoseGraph floor、`P-POSE-04` は bounded pose save/devtools floor、`P-PROJ-02` は bounded projection IR floor、`P-PROJ-03` は bounded boundary-schema floor、`P-PROJ-04` は bounded same-binary local role-split floor、`P-ENG-02` は bounded provider-admission floor、`P-ENG-03` は bounded renderer pose backend floor、`P-FSV1-01` と `P-FSV1-02` は bounded source-first operational suite floor を 12-row まで actualize し、`P-FSV1-03` は bounded Full V1 release-check/report/viewer bundle lane を actualize した。`P-FSV1-99` は final validation / docs-report cleanup / claim-non-claim audit でその chain を close した。現在の promoted package はない。

大きな順序は次の通り。

- `FS-01`: textual Mir grammar MVP.
- `FS-02`: typed IR and checker.
- `FS-03`: Mir-owned computational interpreter.
- `FS-04`: effectful Mir integration.
- `FS-05`: PoseGraph runtime.
- `FS-06`: projection IR and boundary schemas.
- `FS-07`: local server/client split MVP.
- `FS-08`: engine/provider admission MVP.
- `FS-09`: devtools full alpha panels.
- `FS-10`: native host bundle plus optional backend gate.
- `FS-11`: release check and clean clone guide, then final audit cleanup.

## Non-claims

この roadmap は final public grammar、final ABI / SDK、Rust-level language completion、LLVM/native codegen、production WAN/federation、distributed durable save/load R3/R4、arbitrary native package execution、arbitrary WASM execution、Unity / Unreal / WASM / native provider execution completion を主張しない。
