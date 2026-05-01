# Mirrorea future axis 01

## この文書の役割

この文書は、Mirrorea の **docs-first / repo-local integration roadmap-memory family** を日本語で短く読むための summary です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/`
- 実行証跡は `docs/reports/`
- live queue authority は `progress.md` / `tasks.md` に置き、ここは repository-memory reading と closeout-memory gate を reader-facing にまとめる入口です

## 現在地

repo が実装済みの current line は、**repo-local alpha-ready current layer** です。
Mirrorea full runtime、final public API、real network、final auth stack、final visualization API まで完了したことを意味しません。

current executable floor は次です。

| area | current floor | evidence entry |
|---|---|---|
| Mir current-L2 | `samples/current-l2/`、finite-index first strong typing、order / handoff relation、model-check second line、Lean foundation / generated stub | `python3 scripts/current_l2_guided_samples.py closeout --format json` |
| clean near-end | typing / order-handoff / model-check / modal suite | `python3 scripts/clean_near_end_samples.py closeout` |
| shared-space representative slices | Sugoroku world runtime attachment vertical slice、avatar follow representative slice | `python3 scripts/sugoroku_world_samples.py closeout --format json`, `python3 scripts/avatar_follow_samples.py closeout --format json` |
| typed external / transport | `EXT-03/04` helper subset、reported `NET-01` parity anchor + runnable `NET-02..05` helper-local canary | `python3 scripts/typed_external_boundary_samples.py closeout --format json`, `python3 scripts/network_transport_samples.py closeout --format json` |
| projection / visualization | helper projection preview、committed generated bridge manifest、typed viewer prototype inventory | `python3 scripts/projection_codegen_samples.py closeout --format json`, `python3 scripts/visual_debugger_viewer_samples.py closeout --format json` |
| hot-plug Rust floor | `mirrorea-core` request/verdict carrier、`mir-runtime` skeleton / engine-state report | `cargo test -p mirrorea-core`, `cargo test -p mir-runtime --test hotplug_runtime_skeleton` |

Mirrorea future-axis 側の repository-memory family は、major family ごとに次のように読みます。

| family | closeout memory | still not claimed |
|---|---|---|
| `P4` / `P5` signatures | `TermSignature` lanes and scope split、`LayerSignature` lanes and helper/runtime inventory split | final public signature schema / final public layer law schema |
| `P6` / `P7` envelopes and views | `MessageEnvelope` / `AuthEvidence` seam、typed visualization / telemetry security envelope | final auth protocol / final viewer or telemetry service |
| `P10` / `P11` core substrate | `LayerSignature` / `PrincipalClaim` / `AuthEvidence` / `MessageEnvelope` carriers、`MembershipRegistry` / `PlaceCatalog` / `LogicalPlaceRuntimeShell` | full Mirrorea runtime |
| `P12` / `P13` adapters and transport | helper-local host-boundary inventory、process-boundary canaries | final host schema / production socket or broker / durable replay |
| `P14` / `P15` / `P16` / `P17` | hot-plug helper inventory、projection generated bridge evidence、typed viewer prototype inventory、storage / LLVM guardrail | final hot-plug runtime lifecycle / final emitted executable / final backend |
| `P18` / `U1` | public-boundary inventory and true user-spec option matrix | actual product commitment / final public freeze |
| `R1..R7` | verification widening, AttachPoint minimal contract, `FAIRY-05` recommendation, hot-plug kept-later boundaries and owner split | completed rollback / migration / distributed ordering / public ABI |
| `P19` / `P20` / `P21` | engine-neutral request/verdict carrier、thin runtime skeleton、runtime-private engine-state progression | rollback protocol / durable migration engine / distributed activation protocol / final public hot-plug ABI |
| post-`P21` trilogy | rollback-durable family, distributed activation ordering family, final public hot-plug ABI family docs-first closeout | actual public ABI freeze |

The current hot-plug docs-first stop line is deliberately narrow: `freeze prerequisite fixed; public ABI still unfrozen`。
current snapshot reading では、追加の self-driven post-`P21` docs-first family はなく、actual `U1` commitment hold line が残っています。
active maintenance line と package order の current snapshot authority は `progress.md` / `tasks.md` に残します。

Current none-auth baseline では、transport / auth / membership / capability / witness を separate lane で読めるようにしています。
`auth none` は temporary repo-local baseline であり、session token / signature / federation protocol を fixed した意味ではありません。

## 主軸

守るべき project axis は次です。

```text
正しい理論に基づき、
正しく hot-plug でき、
Place をまたいで実行・通信・検証・可視化できる
仮想空間システムを作る。
```

局所最適化でこの軸を崩してはいけません。

## 高位の読み

### Place

`Place` は participant そのものではありません。

- physical process
- machine / node
- virtual thread
- authority / permission compartment
- state / queue / visibility / observation frontier を持つ execution locus

したがって `Alice is a Place` ではなく、
`Alice` は participant / principal、
`ParticipantPlace[Alice]` は Place と読みます。

### external world boundary

- standard I/O は Mir core primitive ではありません。
- `stdin` / `stdout` / `stderr` を core built-in として固定しません。
- external world とは typed external effect adapter で接続します。

ここでいう adapter は、unsafe FFI をそのまま privileged 化するものではなく、
contract と effect を保った boundary です。

### effect-based OS-like substrate

Mirrorea / adapter / visualization / telemetry の内側を、
effect-based OS-like substrate として読むこと自体は有用です。

ただしこれは **内側の解釈** です。

- Mir core に privileged standard I/O primitive を足す理由にはしない
- Mir / Mirrorea / Typed-Effects Wiring Platform を 1 つの runtime へ collapse しない
- authentication / authorization / membership / capability / witness を transport に潰さない

### transport と auth の分離

次の層は潰さずに分けます。

- transport
- authentication
- authorization
- membership
- capability
- witness / audit

current repo-local line では Sugoroku sample が membership epoch / incarnation / witness を先に示しており、
helper-local / report-local first cut では `MessageEnvelope` carrier で transport insertion seam を visible にしました。
未決なのは final public `AuthEvidence` kind、real transport、session / signature schema です。

### visualization / telemetry

可視化は optional debug toy ではありません。
ただし visualization も telemetry も、外へ情報を出す effect です。

そのため次を要求します。

- label
- authority
- redaction
- retention
- evidence-oriented rendering

`untyped debug leak` として出してよいものではありません。
performance telemetry も同様に information-bearing effect であり、typed telemetry として扱います。current line では helper/runtime security envelope に `retention_scope` を持たせ、NET-05 observer route trace は fail-closed にしています。
さらに `P16` current first-cut closeout では helper/runtime surface を `viewer_panel_lanes` / `viewer_telemetry_lanes` と `P16-VIEW-01..05` に正規化し、`typed public prototype inventory over helper/runtime surfaces; not a final public viewer API` を current stop line にしました。

### VerificationLayer composition

`VerificationLayer` は、finite-index checker、theorem bridge、model-check second line、
runtime policy preview、visualization / telemetry lane を typed layer として合成する current reading を指します。current emitted floor は helper `verification_handoff_witness` と runtime `verification_model_check` までです。

- helper-local signature dump や report-local inventory は first cut の evidence carrier に留める
- hidden verifier builtin や final public verifier contract を既成事実化しない
- exact public law surface と composition contract は `UNRESOLVED` のまま残す

### projection / placement

高位の source から、後で place-specific program へ projection できる性質を保ちます。

- server-side authoritative path
- participant-side view / local debug path
- adapter path
- visualizer path

のどれかに source principal を早期固定しないことが重要です。

current repo では、`plan/20-projection-and-placement-roadmap.md` に
system-wide source / place-specific program distinction、place split、validity checklist、helper/report-local preview floor、stop line を置いてあります。
さらに helper-local / report-local current evidence として、

- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`

で `projection_view` と `cross_place_projection` を読めます。
さらに `python3 scripts/projection_codegen_samples.py closeout --format json`
で committed generated bridge evidence と live-anchor alignment surface を読めます。
ただし、これは final projection IR や emitted executable place program を意味しません。

### hot-plug

Mirrorea の中心は runtime hot-plug です。

- Patch
- AttachPoint
- compatibility check
- activation cut
- migration contract

を今後の principal package として扱います。

current repo では、`plan/21-hotplug-attachpoint-roadmap.md` に
compatibility checklist、activation cut、migration stop line、`SUG-01` attach と `SUG-09` detach TODO boundary を置いてあります。
さらに helper-local executable widening として、Sugoroku helper の `hotplug_lifecycle`、
`--debug hotplug`、`detach_request#1` auth-none envelope canary、attach-detach telemetry / visualization current view も actualize しました。
storage detach script は operational cleanup concern であり、この runtime hot-plug lifecycle そのものではありません。

### network transport

transport widening は `local_queue` baseline をそのまま final public transport ABI にしないための docs-first line です。

- loopback
- reconnect
- membership epoch / member incarnation guard
- typed transport failure
- redacted route trace

current repo では、`plan/22-network-transport-roadmap.md` に
current anchor、reported `NET-01` parity anchor、runnable `NET-02..05` canary、transport widening invariant、stop line を置いてあります。
`NET-01` は Sugoroku loopback parity を reported anchor として残し、
`NET-02..05` は helper-local process-boundary canary として runnable に actualize しています。
ここでも transport と auth / membership / capability / witness / visualization を collapse しません。

### compiler/backend/LLVM preparation

toolchain / backend 側では、small VPS と detachable workdir を前提に、
root disk を build cache や LLVM artifact で既成事実化しない guardrail を先に固定します。

- external workdir
- `CARGO_TARGET_DIR`
- `CARGO_HOME`
- LLVM path readiness
- non-destructive cleanup

repo では、`plan/23-compiler-backend-llvm-guardrail-roadmap.md` に
guardrail anchor、non-destructive probe floor、stop line を置いてあります。
actual LLVM build、final backend choice、final packaging / FFI / engine adapter target はまだ fixed しません。

closeout を実行コマンド付きで追う landing page は `docs/hands_on/current_phase_closeout_01.md` です。

## representative slices

### current

- Sugoroku world runtime attachment vertical slice
- avatar fairy follow representative slice

### residual candidate

- `FAIRY-05` reacquire-after-return widening

avatar fairy follow では、`scripts/avatar_follow_samples.py` と
`samples/clean-near-end/avatar-follow/` によって、
`FAIRY-01` / `FAIRY-02` / `FAIRY-03` / `FAIRY-04` / `FAIRY-06` を widened active representative slice として追えます。
`FAIRY-05` だけが residual planned family に残ります。
2026-04-27 の docs-first fixation では、`FAIRY-05` を planned family に留め、
visibility-return witness をどの carrier に載せるかを `UNRESOLVED` のまま残しました。
2026-04-28 の `P9` helper/test/docs hardening で、reopen 前に positive
reacquire-after-return sample、negative missing-return-witness または stale-membership companion、
explicit `state_timeline` / `anchor_switch` evidence、docs/report/snapshot sync が要ることを
helper closeout / dashboard / snapshot docs に actualize しました。
current helper closeout は `planned_sample_paths` と `fairy05_reopen_gate` を返し、
この reopen gate を planning-only inventory として visible にします。

## 文書の読み分け

混同しやすいものは次のように分けて読みます。

- 規範 spec:
  `specs/`
- repository memory:
  `plan/`
- historical report:
  `docs/reports/`
- current sample:
  `samples/clean-near-end/`
- active base source corpus:
  `samples/current-l2/`
- active proof evidence:
  `samples/lean/`
- planned sample:
  `samples/not_implemented/`
- prototype / compatibility anchor:
  `samples/prototype/`
- historical old sample:
  `samples/old/`
- generated artifact reserve:
  `samples/generated/`
- helper-local debug output:
  script の `--debug` や detached artifact
- final public API:
  **まだ deferred**
- deferred mixed gate:
  parser/public API、auth/public contract、visualization/public contract、projection/public API、hot-plug/public API

## closeout memory and snapshot reading

`P0` / `P1` から `P21` までの repo-side package、`R1..R7` の docs-first boundary package、
post-`P21` later-family trilogy は close 済みです。
これは Mirrorea system completion ではなく、current repo-local floor と later gate の分離が
文書・sample・helper・report に mirror された状態を意味します。

closed closeout memory として読む入口は次です。

| family | reading | entry point |
|---|---|---|
| public-boundary inventory | `P18` repo-side first cut。final public freeze ではない | `plan/27-public-api-parser-gate-roadmap.md`, `public_api_parser_gate_plan_01.md` |
| true user-spec option matrix | `U1` option inventory。actual commitment は未実施 | `plan/28-post-p18-true-user-spec-hold-option-matrix.md`, `post_p18_true_user_spec_hold_option_matrix_01.md` |
| verification widening threshold | helper/runtime verification floor と widening threshold | `plan/29-verification-layer-widening-threshold.md`, `verification_layer_widening_threshold_01.md` |
| AttachPoint minimal contract | helper-local lifecycle / detach TODO boundary の minimal contract row | `plan/30-attachpoint-detach-minimal-contract.md`, `attachpoint_detach_minimal_contract_01.md` |
| `FAIRY-05` carrier bundling | planned family の carrier recommendation。active sample 化ではない | `plan/31-fairy05-visibility-return-carrier-bundling.md`, `fairy05_visibility_return_carrier_bundling_01.md` |
| rollback / migration boundary | helper-local evidence が証明していない hot-plug kept-later matrix | `plan/32-hotplug-real-migration-rollback-boundary.md`, `hotplug_real_migration_rollback_boundary_01.md` |
| runtime-crate owner split | helper preview / core carrier / runtime assembly の owner split | `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`, `runtime_crate_hotplug_engine_ownership_cut_01.md` |
| carrier admission cut | first admissible Rust-side hot-plug family = engine-neutral request / verdict carrier | `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`, `runtime_crate_hotplug_carrier_admission_cut_01.md` |
| post-`P20` package inventory | `P21` runtime-private engine-state floor への historical closeout bridge | `plan/35-post-p20-hotplug-next-package-inventory.md`, `post_p20_hotplug_next_package_inventory_01.md` |
| rollback / durable migration family | closed post-`P21` trilogy の historical first boundary family | `plan/36-post-p21-rollback-durable-migration-family.md`, `post_p21_rollback_durable_migration_family_01.md` |
| distributed activation ordering family | closed post-`P21` trilogy の historical second boundary family | `plan/37-post-p21-distributed-activation-ordering-family.md`, `post_p21_distributed_activation_ordering_family_01.md` |
| final public hot-plug ABI family | closed post-`P21` trilogy の last historical boundary family: `freeze prerequisite fixed; public ABI still unfrozen` | `plan/38-post-p21-final-public-hotplug-abi-family.md`, `post_p21_final_public_hotplug_abi_family_01.md` |

現在の snapshot reading は、追加の self-driven post-`P21` docs-first family ではありません。
`progress.md` / `tasks.md` に記録された actual `U1` commitment hold line です。

- installed binary / packaging adoption target
- FFI / engine adapter / host integration target
- first shipped public surface scope
- final shared-space operational catalog breadth

これらを user-facing decision として確定するまで、final public package structure、
public hot-plug ABI、public viewer / adapter / projection / verifier API は freeze しません。

## stop line

この summary を読んでも、次を「すでに実装済み」とは読まないでください。

- real network transport
- multi-server consensus
- durable distributed commit
- final public auth protocol
- final public visualization API
- final public projection API
- final public hot-plug API
- final parser / checker / runtime / verifier API
- final product completion
