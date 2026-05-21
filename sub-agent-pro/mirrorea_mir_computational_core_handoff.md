# Mir / Mirrorea の認識ズレ修正と「計算言語としての Mir」再設計ハンドオフ

- 対象リポジトリ: `https://github.com/yukatayu/mir_poc_01`
- 想定作業者: 新規コンテキストの Codex
- 目的: Product Alpha-1 の runtime / package / devtools 成果を維持しつつ、Mir が本来担うべき「計算言語としての表現力」と「仮想空間の意味論を Mir 側で所有する」方向へ、設計・理論・実装計画を正しく戻す。
- 注意: これは方針転換というより、現行成果の上に **Mir Computational Core** と **Transform / PoseGraph / Projection Boundary** を追加して、実用的な言語・実用的な仮想空間基盤へ進めるための再整理である。

---

## 0. この文書の結論

これまでの議論と repo の現状を踏まえると、認識ズレの中心は次である。

> Product Alpha-1 は、`mirrorea-alpha` CLI、`package.mir.json`、same-session runtime、hot-plug、save/load、devtools、native host launch bundle などの **実行基盤・運用基盤** としてはかなり進んだ。  
> しかし、Mir 言語そのものが C/Rust に近い基礎計算能力を持ち、`AddOne` のような計算を Mir source / typed IR / checker / runtime semantics / compiler target として扱う段階には、まだ十分に到達していない。

したがって、今後の大きな修正目標は次である。

1. **Mir Computational Core** を作る。  
   C 言語程度、最終的には Rust 程度の基礎的な変数宣言・関数・構造体・配列・算術・制御構文・module/import を備える。ただし raw pointer や unchecked memory 操作は入れない。

2. **AddOne を external adapter ではなく Mir 内の計算として実装する。**  
   外部 adapter は input/output boundary だけを担当し、`x + 1` は Mir 側の関数として型検査・実行・観測されるべき。

3. **VR / 仮想空間の意味ある状態を Mir 側に置く。**  
   avatar head transform、object anchor、pose version、UI state、world state、synchronization policy、fallback、capability、observation、save/load は Mir / Mirrorea 側に現れるべき。Unity / UE / renderer / WASM / native library は backend / provider / sandbox であり、意味論を隠す場所ではない。

4. **server/client/adapter 分割は人間が手で全部合わせるのではなく、projection / deployment profile / packet boundary / FFI boundary によって生成・検査可能にする。**

5. **Product Alpha-1 は捨てない。**  
   既存の `mirrorea-alpha` CLI、`package.mir.json`、devtools、hot-plug、save/load、transport、native host launch bundle は、この新しい Mir Computational Core の上に載せる実行基盤として維持する。

---

## 1. 認識ズレが発生した原因

### 1.1 `AddOne` の意味を取り違えていた

現行 Product Alpha-1 の `AddOne` は、概ね次のような `package.mir.json` 入力で表されている。

```json
{
  "runtime_input": {
    "entry_place": "Place[ProductDemoRoom]",
    "host_io": {
      "adapter_kind": "AddOne",
      "effect_ref": "typed_host_io.add_one",
      "request_payload": { "kind": "int", "value": 41 },
      "expected_response": { "kind": "int", "value": 42 }
    }
  }
}
```

これは **typed external boundary demo** としては価値がある。外部処理を typed effect として呼び、event DAG や devtools に観測可能にするという意味では正しい。

しかし、これは「Mir 言語が `x + 1` を計算した」ことの証拠ではない。現状の読みでは、`AddOne` の計算は host adapter 側にあり、Mir 側は入出力 schema と effect ref を管理しているだけに近い。

本来欲しいのは、たとえば次のような構造である。

```mir
module Example.AddOne

fn add_one(x: Int64) -> Int64 {
  let y: Int64 = x + 1
  return y
}

transition main at HostPlace {
  x <- perform read_int via host_input
  y <- add_one(x)
  perform write_int(y) via host_output
}
```

ここでは、`add_one` の本体 `x + 1` が Mir 側に現れている。型検査・実行時意味論・将来のコンパイル対象も Mir 側にある。外部 adapter は input/output だけを担う。

この区別を曖昧にしたため、「計算能力がある」という説明が実際より強く見えた。

### 1.2 Product Alpha runtime と Mir language core を混同した

Product Alpha-1 は、次の点でかなり進んでいる。

- `mirrorea-alpha` CLI
- versioned `package.mir.json`
- local same-session runtime
- attach / hot-plug
- local/Docker transport
- R0 save / bounded R2 quiescent-save
- observer-safe devtools / viewer
- native host launch bundle
- release check
- operational product sample suite

しかし、これは **runtime / package / orchestration / observability** の成熟であって、Mir source language の基礎計算力そのものの成熟ではない。

Mir 言語としては、まだ次が足りない。

- 一般的な関数定義
- `let` / mutable local variables
- arrays / bounds checked indexing
- records / structs / enums
- loops / conditionals / pattern matching
- source-level arithmetic
- computational source -> typed IR -> runtime/compiler path
- projection / codegen correctness

### 1.3 native host launch bundle と native compiler を混同しやすかった

現行の `build-native-bundle` は、compiled Rust CLI、package files、viewer assets、reports、manifest、run script をまとめる **native host launch bundle** である。

これは便利だが、次ではない。

- Mir source -> LLVM -> native binary
- server/client target binary generation
- arbitrary native package execution

したがって、「native output がある」という言い方は、必ず `host launch bundle` と限定する必要がある。

### 1.4 外部 backend の役割を広く見すぎていた

Unity / UE / WASM / FFI / native library を「何でも外に出して良い」と扱うと、Mirrorea の意味が薄れる。

避けたい状態:

```text
Mir には少しだけ package metadata がある。
実際の world logic は Unity script にある。
avatar logic は WASM にある。
server/client split は人間が別々に書く。
packet schema も人間が合わせる。
debug はログを見る。
```

これでは、Mirrorea が「正しい理論に基づき、正しく hot-plug でき、検証・可視化できる仮想空間システム」ではなく、ただの外部 script aggregator になってしまう。

正しい方向:

```text
Mir / Mirrorea 側:
  world state, avatar transform, object relation, UI state, effect, contract,
  capability, synchronization, fallback, observation, save/load

外部 backend:
  rendering, device input, asset decoding, shader, native library calls,
  unavoidable engine-specific operations
```

---

## 2. 修正後の project axis

今後の project axis は次のように明確化する。

```text
Mir:
  Rust 程度の最終表現力を目指す計算言語・意味論コア。
  ただし最初は C-like baseline から始める。

Mirrorea:
  Mir で書かれた world / object / effect / contract を、Place、network、runtime、hot-plug、devtools、save/load に載せる fabric。

External backend:
  rendering、device input、asset decode、shader、native library、WASM sandbox など、どうしても外に出る境界。
  ただし boundary は Mir 側で typed / contracted / observable にする。
```

### 2.1 最終的な表現力の目安

ユーザ意図として、最終的には Rust 程度の表現力が欲しい。ただし、いきなり Rust full feature を目指さない。

最初の baseline は C から raw pointer / goto / unchecked memory 操作を除いた程度。

#### 必要な基礎機能

```text
primitive:
  Bool, Int64, UInt64, Float64, Text, Unit

variables:
  let, mut, lexical scope

control:
  if / else, match, while, for, return, block

functions:
  fn, effectful fn, module, import

compound data:
  tuple, record/struct, enum/variant

arrays:
  fixed array, vector, indexing, length, iteration

computation:
  arithmetic, comparison, boolean operators, numeric conversions

effects:
  perform, require, ensure, publish, observe, witness, handoff

resource discipline:
  no raw pointer by default, no unchecked pointer arithmetic,
  no hidden global mutable state
```

#### 将来的に Rust へ寄せる機能

```text
ownership / borrowing-like discipline
traits / interfaces / typeclass-like capability, but limited
pattern matching
generic functions, bounded and staged
modules / packages
async / effect handling via Mir effect semantics
FFI boundary with explicit schema
```

#### まだ贅沢すぎるもの

```text
full Haskell typeclass ecosystem
full TypeScript structural type universe
full F* dependent verification language
arbitrary dependent source terms
Mir as proof assistant
```

---

## 3. 新しい層構造

### L0: External substrate

```text
OS, process, socket, filesystem, GPU, renderer, device input, network
```

Mirrorea の外部前提。ここは project が所有しない。

### L1: Mir Computational Core

C-like / Rust-like に向かう基礎計算層。

```text
values, variables, functions, arrays, records, enums, arithmetic, control flow
```

この層が弱いと、Mir は「意味論 DSL」になってしまい、実用言語にならない。

### L2: Mir Effect / Contract Core

既存の Mir current-L2 の中心。

```text
effect, contract, require, ensure, perform,
publication, observation, witness, handoff,
fallback, lifetime, atomic_cut
```

### L3: Verification Layer

```text
Line 1: decidable static checker
Line 2: model-check second line
Line 3: proof side line
```

Mir を proof assistant にするのではなく、obligation を明示して外部検証に渡す。

### L4: Mirrorea Runtime / Fabric

```text
Place, MessageEnvelope, MembershipRegistry, capability, auth, witness,
HotPlugRequest, HotPlugVerdict, transport, save/load, devtools
```

### L5: Projection / Deployment / Compiler Boundary

```text
system-wide source
  -> typed IR
  -> projection IR
  -> server/client/adapter target manifest
  -> packet boundary schema
  -> FFI boundary schema
  -> native host bundle / future compiler backend
```

### L6: Host / Client / Backend

```text
server host
client/browser-like host
headless client
renderer backend
Unity / UE / native / WASM provider
```

### L7: Application / Spatial World

```text
Mirrorea Spaces
WorldCore
MembershipChat
SugorokuWorld
PortalWorldLink
Shard / region / gradient observation
Reversed Library
```

---

## 4. 理論的裏付け

### 4.1 型判定の基本形

既存の verification stratification を維持し、Mir Computational Core をこの判定に載せる。

```text
Σ ; Ψ ; Γ ; Δ ⊢ e : A @ μ ! ε ⇝ C ; O
```

意味:

```text
Σ  finite user-defined theory
Ψ  place / phase / observation / cut frontier context
Γ  unrestricted variables
Δ  linear / affine / capability resources
A  value type
μ  mode
ε  effect row
C  decidable constraints
O  residual obligations
```

Mir Computational Core は `Γ` と `A` と expression semantics を強化する。Mir Effect Core は `Δ`、`ε`、`Ψ` と interaction する。

### 4.2 基礎計算の型安全性

#### Preservation

```text
If Γ ⊢ e : A and e -> e', then Γ ⊢ e' : A.
```

effectful expression では、effect row を含めて次のように読む。

```text
If Σ;Ψ;Γ;Δ ⊢ e : A @ μ ! ε ⇝ C;O
and e steps to e' with event ev,
then e' has compatible type/effect context, and ev ∈ ε or is structurally allowed.
```

#### Progress

```text
If ∅ ⊢ e : A and all required effect providers / capabilities / witnesses are available,
then e is either a value, a declared Reject, or can step.
```

ここで `Reject` は失敗を隠す例外ではなく、declared failure row に属する outcome。

### 4.3 Arrays の安全性

```text
Array<T, n>
Vector<T>
```

indexing は bounds checked。

```text
if 0 <= i < len(a) then a[i] : T
else Reject(IndexOutOfBounds) or static reject if statically known
```

unchecked pointer arithmetic は入れない。

### 4.4 Records / structs

```text
struct Vec3 { x: Float64, y: Float64, z: Float64 }
```

field access:

```text
if Γ ⊢ r : { f : A, ... } then Γ ⊢ r.f : A
```

### 4.5 Effects と pure computation の分離

```text
fn add_one(x: Int64) -> Int64
```

は pure。

```text
effect read_int -> Int64
effect write_int(Int64) -> Unit
```

は effectful。

`AddOne` は pure function として Mir に書く。host input/output は effect boundary。

### 4.6 External boundary soundness

外部 backend を呼ぶ場合、次を必須にする。

```text
ExternalProvider P has:
  input_schema
  output_schema
  effect_row
  failure_row
  capability_requirements
  resource_policy
  observation_policy
  sandbox/native_policy
```

Soundness condition:

```text
If Mir code calls external provider P,
then all communication with P occurs through declared input/output schemas,
and any failure/effect emitted by P is contained in declared rows.
```

### 4.7 Projection correctness

system-wide source を server/client/adapter に分ける時の最小 theorem。

```text
If source system S type-checks,
and projection π produces targets T_server, T_client, T_adapter,
and all packet/FFI boundaries preserve declared schemas,
then every cross-target interaction in T corresponds to a declared effect/message boundary in S.
```

これにより、人間が server/client/WASM/FFI を別々に書いて合わせる状態を防ぐ。

### 4.8 Transform / PoseGraph consistency

#### 型

```text
Vec3 = { x: Float64, y: Float64, z: Float64 }
Quat = { x: Float64, y: Float64, z: Float64, w: Float64 }
Transform = { position: Vec3, rotation: Quat, scale: Vec3, space: SpaceId, pose_version: PoseVersion }
TransformRef<Target>
Anchor<Target>
```

#### No split-frame dependency

目的:

```text
同じ client の同じ render frame 内で、
head と head に追従する object が異なる pose version を参照してはならない。
```

形式:

```text
For any RenderFrame(client, frame_id, snapshot_version v),
if object O is anchored to target T,
then render(O) and render(T) must use the same target pose version v_T
from the same snapshot.
```

これは「全 client が全く同時に同じ座標を見る」保証ではない。ネットワーク上、それは一般には不可能。保証するのは、**各 client の一つの observation frame 内で依存関係が裂けないこと**。

### 4.9 Save / cut consistency

Mir Computational Core と Transform / PoseGraph を入れても、既存の cut 理論を保つ。

```text
atomic_cut:
  place-local rollback frontier

R0:
  local save

R2:
  bounded quiescent save with NoInFlight / AllPlacesSealed / NoPostCutSend

R3/R4:
  distributed durable save/load; still later
```

### 4.10 multi-shot continuation の扱い

今後 continuation を入れるなら、次が必要。

```text
MultiShot continuation may only capture unrestricted / copyable / replay-safe context.
It may not capture linear resource, mutable reference, one-shot witness,
open transport obligation, irreversible external effect, or state past atomic_cut.
```

これは今回の immediate implementation には入れないが、設計上の stop line として残す。

---

## 5. 実装計画

作業ブランチ案:

```text
feature/mir-computational-core-001
```

主要 package line:

```text
P-COMP-00 recognition-rebaseline
P-COMP-01 Mir computational core spec and sample scaffold
P-COMP-02 pure AddOne in Mir
P-COMP-03 variables / arrays / records / control-flow first floor
P-COMP-04 effect boundary around internal computation
P-POSE-01 Transform / PoseGraph spec and sample scaffold
P-POSE-02 avatar head + anchored object no-split-frame sample
P-PROJ-01 projection boundary and packet/FFI schema inventory
P-ENG-01 engine/backend adapter boundary spec
```

---

## 6. リポジトリに追加・更新すべき文書

### specs

```text
specs/28-mir-computational-core.md
specs/29-transform-posegraph-semantics.md
specs/30-projection-and-backend-boundary.md
specs/31-engine-wasm-ffi-adapter-boundary.md
```

### plan

```text
plan/53-mir-computational-core-roadmap.md
plan/54-transform-posegraph-roadmap.md
plan/55-projection-backend-roadmap.md
plan/56-engine-adapter-roadmap.md
```

### docs

```text
docs/hands_on/mir_computational_core_01.md
docs/hands_on/transform_posegraph_01.md
docs/research_abstract/mir_computational_core_01.md
```

### samples

```text
samples/product-alpha1/computational/
  add-one-pure-mir/
  variables-scope/
  arrays-bounds/
  records-vec3/
  control-flow/
  imports-functions/
  host-io-internal-transform/

samples/product-alpha1/posegraph/
  avatar-head-transform/
  anchored-object/
  sparkle-fallback-anchor/
  no-split-frame-positive/
  split-frame-negative/

samples/product-alpha1/projection/
  server-client-target-manifest/
  packet-boundary-schema/
  ffi-boundary-schema/
```

### scripts

```text
scripts/mir_computational_samples.py
scripts/posegraph_samples.py
scripts/projection_boundary_samples.py
```

---

## 7. 必須サンプル詳細

### COMP-01 Pure AddOne in Mir

目的:

```text
AddOne を host adapter ではなく Mir function として定義する。
```

Representative source:

```mir
module Samples.Computational.AddOne

fn add_one(x: Int64) -> Int64 {
  let y: Int64 = x + 1
  return y
}

transition main at HostPlace {
  input <- perform read_int via host_input
  result <- add_one(input)
  perform write_int(result) via host_output
}
```

期待:

```text
input 41 -> output 42
`x + 1` は Mir computation として typed / executed される
host adapter は read/write boundary だけ
```

合格条件:

```text
- source/package check accepted
- runtime event shows internal function call / computation step
- devtools shows host input, Mir computation, host output separately
- report explicitly says AddOne computation is Mir-owned
```

### COMP-02 Variables and scope

```mir
fn f(x: Int64) -> Int64 {
  let a: Int64 = x
  let mut b: Int64 = a + 1
  b = b * 2
  return b
}
```

期待:

```text
lexical scope
mutable local variable
arithmetic
```

### COMP-03 Arrays

```mir
fn sum3(xs: Array<Int64, 3>) -> Int64 {
  let mut s: Int64 = 0
  for i in 0..3 {
    s = s + xs[i]
  }
  return s
}
```

negative:

```text
xs[3] is rejected or dynamic Reject(IndexOutOfBounds)
```

### COMP-04 Records / Vec3

```mir
struct Vec3 { x: Float64, y: Float64, z: Float64 }

fn add(a: Vec3, b: Vec3) -> Vec3 {
  return Vec3 { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z }
}
```

### COMP-05 Control flow

```mir
fn clamp(x: Float64, lo: Float64, hi: Float64) -> Float64 {
  if x < lo { return lo }
  if x > hi { return hi }
  return x
}
```

### COMP-06 Imports

```mir
module Samples.WorldCore
struct Participant { id: Text }

module Samples.Chat
import Samples.WorldCore
```

### COMP-07 Host I/O around internal computation

目的:

```text
host input/output は external boundary、変換は Mir 内部。
```

### POSE-01 Avatar head transform

```mir
struct Transform { position: Vec3, rotation: Quat, scale: Vec3, pose_version: PoseVersion }

place Avatar[Alice] {
  var head: Transform
}
```

### POSE-02 Anchored object

```mir
place SparkleEffect {
  anchor: TransformRef = Avatar[Alice].head
}
```

### POSE-03 Fallback anchor

```mir
anchor = Avatar[Alice].head
  fallback Avatar[Alice].shoulder
  fallback World.origin
```

### POSE-04 No split-frame positive

Expectation:

```text
head and anchored sparkle render using same pose_version in one observation frame
```

### POSE-05 Split-frame negative

Construct an invalid sample where:

```text
head uses pose_version v2
sparkle uses head pose_version v1
```

Expected:

```text
static/model-check/devtools rejection depending on exact carrier
```

### PROJ-01 Server/client projection manifest

Purpose:

```text
Do not yet generate LLVM binaries.
Generate projection inventory:
  server target
  client target
  packet boundaries
  FFI boundaries
```

### PROJ-02 Packet boundary schema

```text
RollRequestPacket
ChatMessagePacket
TransformUpdatePacket
PoseSnapshotPacket
```

### PROJ-03 FFI boundary schema

```text
RenderBackend.draw_mesh
InputBackend.read_head_pose
AssetBackend.load_avatar
```

---

## 8. コマンド例

Expected future commands:

```bash
python3 scripts/mir_computational_samples.py list --format json
python3 scripts/mir_computational_samples.py run COMP-01 --format json
python3 scripts/mir_computational_samples.py check-all --format json

python3 scripts/posegraph_samples.py run POSE-04 --format json
python3 scripts/posegraph_samples.py run POSE-05 --format json
python3 scripts/posegraph_samples.py check-all --format json

python3 scripts/projection_boundary_samples.py check-all --format json
```

If integrated into `mirrorea-alpha`:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/computational/add-one-pure-mir --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/computational/add-one-pure-mir --format json
cargo run -q -p mirrorea-cli -- export-devtools 'session#comp-add-one' --out /tmp/mir-comp-devtools --format json
cargo run -q -p mirrorea-cli -- view /tmp/mir-comp-devtools --check --format json
```

---

## 9. Codex に渡すべき実装指示

### 9.1 最初に読むべきファイル

```text
README.md
Documentation.md
progress.md
tasks.md
samples_progress.md
specs/19-verification-stratification.md
specs/20-cut-save-load-semantics.md
specs/23-typed-external-host-boundary.md
specs/24-operational-alpha05-alpha08-readiness.md
specs/25-product-alpha1-public-boundary.md
samples/product-alpha1/README.md
samples/product-alpha1/operational/README.md
```

### 9.2 実装前の判断

Codex は次を固定してから作業する。

```text
- AddOne external adapter demo is not sufficient as Mir computational capability.
- Mir-owned computation must appear in Mir source / typed IR / runtime event / devtools.
- External adapter handles only boundary, not the semantic computation.
- Product Alpha runtime remains useful and should not be discarded.
- Direct LLVM backend is later; start with interpreter / typed IR / projection inventory.
- Unity / UE / WASM / native libraries are backend/provider boundaries, not semantic owners.
```

### 9.3 Stop lines

Codex must not claim:

```text
- final Rust-level expression power completed
- final textual .mir grammar completed
- direct Mir-to-machine-code implemented
- LLVM backend implemented
- server/client split binary implemented
- arbitrary native / WASM execution safe by default
- Unity / UE logic can hide world semantics
- AddOne adapter proves Mir computational core
```

### 9.4 Validation

Minimum validation per package:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Behavior packages add:

```bash
cargo test -p mir-ast -- --nocapture
cargo test -p mir-runtime -- --nocapture
cargo test -p mirrorea-cli -- --nocapture
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/posegraph_samples.py check-all --format json
```

### 9.5 Report requirement

Every non-trivial package must create report:

```text
docs/reports/<next-number>-p-comp-XX-*.md
```

Report must include:

```text
objective
scope
files changed
commands run
evidence
non-claims
reviewer findings
commit/push status
```

---

## 10. Success criteria

### P-COMP-01 success

```text
specs/28 and plan/53 exist
sample matrix exists
no runtime overclaim
progress/tasks/samples_progress synced
```

### P-COMP-02 success

```text
AddOne is defined as Mir-owned computation
host input/output are external boundary only
runtime event distinguishes:
  host input
  Mir compute add_one
  host output
```

### P-COMP-03 success

```text
variables / arrays / records / control-flow samples exist
positive and negative cases pass
```

### P-POSE-01 success

```text
Transform / PoseGraph theory spec exists
no-split-frame invariant stated
sample scaffold exists
```

### P-POSE-02 success

```text
avatar head + anchored object sample runs
same observation frame uses same pose version
negative split-frame sample is rejected or counterexampled
```

### P-PROJ-01 success

```text
projection inventory exists
server/client target manifests exist
packet/FFI boundaries explicit
no LLVM/codegen claim
```

---

## 11. 最終的な目標像

最終的には、次のような workflow を目指す。

```text
1. Developer writes Mir source:
   world core, membership/chat, avatar transforms, game logic, UI state.

2. Checker verifies:
   types, effects, failures, capabilities, fallback, pose consistency, save/load obligations.

3. Runtime/projection decides:
   server target, client target, adapter target, packet boundary, FFI boundary.

4. Host runs:
   server process, client/browser-like process, renderer backend, device input backend.

5. Devtools show:
   source/import graph, projection graph, Place graph, event DAG, messages,
   witness, hot-plug, pose snapshots, save/load, auth/capability.

6. External backend only handles:
   rendering, device input, asset decode, shader/native operations.
   It does not secretly own world semantics.
```

This is the corrected direction.

---

## 12. 最終チェックリスト

Before claiming progress, verify:

```text
[ ] Is the computation actually in Mir, not just in adapter?
[ ] Is external boundary typed and observable?
[ ] Are effect/failure/capability rows declared?
[ ] Are devtools able to show what happened?
[ ] Is native/codegen claim limited to actual implementation?
[ ] Does the sample distinguish source semantics from backend execution?
[ ] Does this avoid humans manually splitting Mir/WASM/FFI/server/client logic?
[ ] Are non-claims explicit?
```

If any answer is no, do not call the package complete.

---

## 13. Codex short prompt

Use the following as the operative instruction summary:

```text
Work on yukatayu/mir_poc_01.
The current Product Alpha runtime is useful, but it over-demonstrates typed external adapters and under-demonstrates Mir as a computational language.
Create the Mir Computational Core line.
Do not treat the current AddOne host adapter as proof of Mir computation.
Define and begin implementing samples where AddOne, variables, arrays, records, control flow, and transforms are Mir-owned source/typed-IR/runtime semantics.
Keep host I/O as boundary only.
Add Transform/PoseGraph theory and samples so VR object anchoring is Mir-managed, not hidden in Unity/UE/WASM/FFI.
Add projection/backend boundary docs and samples, but do not implement or claim LLVM/native codegen yet.
Preserve Product Alpha CLI and operational suite; extend it carefully.
Always update specs, plan, progress, tasks, samples_progress, docs, tests, reports, and validation commands.
```
