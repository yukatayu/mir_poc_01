# 33 — Full System V1 Scope

## role

This document fixes the docs-first scope for **Full System V1**.

Full System V1 is not production WAN/federation and not final public SDK completion.
It is the first source-first system cut where Mir source files are the semantic authoring surface, the current product alpha operational floor is preserved, and the missing language / runtime / projection / provider gaps are made explicit.

## decision level

- `L1`
  - Mir source files are the intended semantic source of truth.
  - `package.mir.json` remains valid as alpha compatibility surface, manifest, or generated package artifact; it is not the final semantic owner.
  - Product Alpha-1 release-candidate workflow is useful and preserved, but it is not the final product.
  - Current computational rows are first-floor evidence, not Rust-level language completion.
  - Projection / backend / engine-provider work must preserve Mir / Mirrorea semantic ownership.
- `L2`
  - Full System V1 milestones `FS-00..FS-11` are the current roadmap cut.
  - Textual Mir alpha grammar, typed IR, interpreter, projection IR, PoseGraph runtime, and provider admission are staged and may revise details as implementation evidence lands.

## project axis

```text
Mir source files に system-wide semantics を書き、
それを型検査・検証・投影・実行することで、
Place をまたいで実行・通信・hot-plug・save/load・可視化できる
仮想空間システムを作る。
```

This axis does not collapse Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform into one runtime.

## final ideal for this roadmap

The ideal source-to-runtime path is:

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

`package.mir.json` may remain an accepted compatibility input and generated artifact during alpha. It must not be used to avoid building a real textual Mir source line.

## current position before Full System V1

The repo already has:

- Product Alpha-1 release-candidate workflow.
- `mirrorea-alpha` command family.
- versioned `package.mir.json` alpha package surface.
- local/Docker controlled runtime.
- same-session hot-plug.
- observer-safe devtools/viewer.
- R0/R2 save evidence.
- native host launch bundle.
- installed-binary adoption probe.
- canonical operational product sample suite.
- Mir-owned computation first-floor evidence.
- PoseGraph no-split-frame helper evidence.
- projection/backend and engine/provider boundary inventories.

The repo still lacks:

- alpha textual Mir parser over the source-first sample set.
- typed IR and checker connected to that parser.
- general C-like interpreter over typed IR.
- broad effectful Mir integration for publish / observe / witness / handoff.
- runtime-integrated PoseGraph state and devtools panels.
- projection IR realization and local server/client split.
- provider admission beyond inventory-only rows.
- Full System V1 release check over the source-first suite.

## milestone map

| Milestone | Status after `P-FS-00` | Gate |
|---|---|---|
| `FS-00` documentation rebaseline | `boundary-fixed` | specs/plan/snapshot docs explain the whole roadmap and non-claims |
| `FS-01` textual Mir grammar MVP | `planned` | alpha parser accepts representative source samples and rejects negative syntax |
| `FS-02` typed IR and checker | `planned` | AST lowers to typed IR with explicit effect, failure, and capability rows |
| `FS-03` Mir-owned computational interpreter | `planned` | safe C-like subset executes from typed IR with compute trace |
| `FS-04` effectful Mir integration | `planned` | perform / publish / observe / witness / handoff / fallback / cut enter runtime explicitly |
| `FS-05` PoseGraph runtime | `planned` | Transform / Anchor / no-split-frame evidence is session-runtime visible |
| `FS-06` projection IR | `planned` | target manifest, packet schema, FFI schema, and preservation report are generated |
| `FS-07` server/client runtime split MVP | `planned` | local/Docker roles run from projection manifest |
| `FS-08` engine/provider admission MVP | `planned` | provider manifest admission and over-capability rejection are runtime visible |
| `FS-09` devtools full alpha panels | `planned` | source / IR / runtime / projection / PoseGraph / provider surfaces are visible |
| `FS-10` native host bundle plus optional backend gate | `planned` | bundle includes sources, IR/projection artifacts, devtools, reports, launch scripts |
| `FS-11` release check and clean clone guide | `planned` | clean clone reproduces Full System V1 sample with positive and negative evidence |

## completion rule

A Full System V1 milestone is complete only when:

- at least one positive sample passes.
- at least one relevant negative sample fails for the expected reason.
- checker/runtime/devtools/report evidence explains the result.
- docs, dashboards, and repository memory are updated.
- a report is written.
- validation is run and recorded.
- commit and push status are recorded.

## non-claims

Full System V1 does not by itself claim:

- final public grammar completion.
- final ABI / SDK completion.
- Rust-level language completion.
- LLVM/native codegen completion.
- production WAN/federation.
- distributed durable save/load R3/R4.
- arbitrary native package execution.
- arbitrary WASM execution.
- final Unity / Unreal / WASM / native provider execution.
- hosted service / marketplace.
- Reversed Library application completion.
