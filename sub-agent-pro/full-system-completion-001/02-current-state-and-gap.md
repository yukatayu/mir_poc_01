# 02 — Current State and Gap

## 現状の到達

現行 repo は以下まで到達している。

- Product Alpha-1 release-candidate workflow
- `mirrorea-alpha` CLI
- versioned `package.mir.json`
- local/Docker controlled runtime
- same-session hot-plug
- observer-safe devtools/viewer
- R0/R2 save evidence
- native host launch bundle
- installed-binary adoption probe
- canonical operational product sample suite
- Mir-owned computation first-floor evidence
- PoseGraph no-split-frame helper evidence
- projection/backend and engine/FFI/WASM planned inventory

## 重要な現状認識

### Product Alpha は強い

以下は bounded alpha として信頼してよい。

```text
check
run-local
session
attach
save
load
quiescent-save
transport local/docker
export-devtools
view
build-native-bundle
demo
release-check
```

### Mir Computational Core はまだ first floor

現在は:

- pure add_one row
- variables/scope rows
- arrays/bounds rows
- records/Vec3 rows
- control-flow rows
- imports/functions rows
- host read -> Mir transform -> host write rows

がある。

まだ無い:

- final textual grammar
- robust parser and diagnostics
- full interpreter over general source
- broad effectful computation
- computational core + publish/observe/witness/handoff integration
- native compiler backend

### PoseGraph は helper evidence

現在は:

- same-client same-observation-snapshot no-split-frame accepted row
- split-frame violation export row

まだ無い:

- runtime-integrated PoseGraph
- avatar/head/hand/shoulder transform runtime
- save/load with pose frontier
- devtools PoseGraph panels
- engine-renderer adapter execution

### Projection / backend は scaffold

現在は:

- projection manifest/inventory
- packet/FFI boundary inventory
- engine/provider inventory

まだ無い:

- server/client split compiler
- codegen
- LLVM/native backend
- WASM backend
- provider admission runtime

## 最大のズレ

過去のズレは、runtime/package/devtools alpha workflow が強くなったことで「Mir が実用計算言語として完成した」と錯覚しやすくなったこと。

今後は必ず次を分ける。

```text
host-boundary computation != Mir-owned computation
package manifest != source language
native host bundle != Mir-to-machine-code compiler
projection inventory != server/client codegen
engine provider manifest != engine integration
```

## 現時点のマイルストーン位置

```text
M0 docs/spec scaffold: done
M1 current-L2 runnable evidence: done
M2 product alpha release-candidate: done
M3 operational product sample suite: done
M4 Mir computational first floor: partial done
M5 PoseGraph helper evidence: partial done
M6 projection/backend inventory: done as inventory only
M7 engine adapter inventory: done as inventory only
M8 textual Mir + C-like interpreter: not done
M9 effectful Mir + runtime integration: not done
M10 projection compiler: not done
M11 backend/provider execution: not done
M12 final public SDK/API/distribution: not done
```
