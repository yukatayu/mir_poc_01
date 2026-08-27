---
id: arch/03-toolchain
status: L2-working
maturity: draft
depends_on: [arch/02-boundary-contracts, spec/04-core-ir, spec/12-sys3-per-locus-projection, spec/13-sys4-in-process-generated-dispatch, adr/ADR-0029, adr/ADR-0030]
summary: toolchain各componentの責務、accepted SYS-3 internal projection compiler、SYS-4 generated-plan-only runtime boundary。
open_items: [OPEN-029]
---

# 03 — Toolchain の責務

| Tool | 入力 → 出力 | してはならないこと | LAB 前身 |
|---|---|---|---|
| mir-parse | .mir → AST(+span) | 意味判断、糖の隠れ展開 | mir-ast (surface parser) |
| mir-check | AST → 判定結果+Diagnostics+Obligations | 未討義務の成功扱い | mir-semantics 各 checker |
| mir-elab | AST → Core IR(spec/04) | 隠れ辺、span 欠落、権限創出 | surface_to_core_elaboration |
| mir-run | Core IR+script → 実行+occurrence rows(spec/05 profile) | verdict なし実行、fail-open | mir-runtime |
| mir-project | Core IR+topology Π → per-locus 成果物+通信境界 | 意味を落とす最適化(BND-006) | projection IR 線 |
| mir-transport | Envelope 配送(I3 以降) | auth の吸収(BND-005) | network-docker canary |
| mir-devtools | occurrence rows → panels/export | redaction 破り、H 非由来行 | viewer/telemetry 線 |
| mir-prove | Obligations → Lean project 骨格+状態同期 | 証明の代筆主張 | lean/ 基盤 |
| mir-conform | SCN suite 実行 → 合否(spec/06) | expectation の暗黙緩和 | scripts/ 検証群 |

共通則: 各 tool は自分の Diagnostic family(spec/07)だけを発行し、下流の失敗を握り潰さない。全 tool は `--format json` で carrier(arch/04)を吐けること。

## SYS-3 selected internal projection boundary

ADR-0029 / spec/12 は、`mir-project` の最初の executable direct-consumer seamを
crate-private pure functionとして固定する。

```text
CheckedSurfaceV0 + exact identity-bound logical locus inventory
  -> owned LocusProgram[locus]
  + generated communication/effect/observation/persistence plans
  + source/Core/artifact correspondence
  | typed ProjectionDiagnostics
```

Logical topologyはlocus inventoryだけを与え、edge、authority、failure、handler、
schema、deployment hostを与えない。projectorはAST/source、M10 conformance facade、
SYS-1/2 runtime stateを再入力にせず、checked Coreからplacementとedgeを構成する。
outputはSYS-4がsourceを再parseせずiterateできるplacement-specific checked fragmentを
ownする。

Close review exposed one required upstream source fact that this no-reparse
boundary cannot invent: `designated consume E.result at C`. The provisional
internal Surface-v0 parser/classifier/checker must preserve that clause as a
distinct AST/M6/M7 checked Core edge before `mir-project` may derive the
evaluator-to-consumer fragment and delivery plan. This is a bounded SYS-3
compiler input correction, not a final/public grammar or artifact ABI. The
direct-consumer seam is accepted at source/evidence cut
`3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9`. Its
`ReturnExistingNoNewConsumption` row is only a static source/Core refinement
requirement. SYS-4, not the accepted compiler step, must implement a carrier-side
idempotent return/wrapper before legacy M8 consumption and supply actual
endpoint tests; current M8/M10 duplicate-delivery behavior remains unchanged.

この選択はinternal compiler boundaryであり、CLI spelling、`--format json` encoding、
public artifact ABI、deployment mapping、runtime admission/dispatchを実装又はfreezeしない。
SYS-4はこのartifactを実行して初めてruntime occurrenceを作れる。

## SYS-4 selected internal execution boundary

ADR-0030 / spec/13 は `mir-run` と future `mir-project`/`mir run-local` facade の下に、
crate-privateなprocess-local direct-consumer seamを固定する。

```text
owned GlobalProjectionResult + complete sealed M9 admission
  -> LocalFabric::bootstrap
  -> locus-tagged runtimes + plan-derived endpoints
  -> staged dispatch / typed failure / source-to-occurrence evidence
```

runtimeはSYS-3 artifactsとgenerated planをiterateし、source/ASTを再入力にせず、fixture名
又はexpected resultからplanを選ばず、manual edgeやauthorityを受け取らない。STは複数の
semantic locusごとに独立M8 sessionを持ち、eligible OW1は同じartifact/planを
worker-exclusive M8 session上で実行する。external actionはsource-derived handler args、
declared tick、bounded faultだけを与え、semantic target/Core/state/grantを与えない。

SYS-4 cut/restoreとchecked patchもruntime-internal seamである。ST cutはwhole-fabric
consistent stateを扱い、patch inputは既にchecked/projected/complete M9-admittedな
designated-only candidateだけを受ける。OW1 cut/patch、CLI spelling、public artifact format、
`--format json` encoding、deployment、wire/transportは未実装・未凍結である。

OPEN-029: mir-lsp(エディタ統合)は I5 で検討。CLI 名は仮称であり、LAB の `mirrorea-alpha` 系列を改称して流用してよい。
