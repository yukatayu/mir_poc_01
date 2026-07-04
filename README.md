# Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins; cite legacy `specs/` /
`plan/` as `LAB:` evidence unless mirrored into canon.

この repository は、4 系統を分離可能なまま扱う **specification-first research repo** です。

- **Mir**
  因果、effect、ownership、lifetime、contract、安全な進化を扱う意味論コア
- **Mirrorea**
  logical naming、routing、overlay insertion、audit、dynamic reconfiguration を扱う fabric/runtime 層
- **PrismCascade**
  media domain の独立 kernel
- **Typed-Effect Wiring Platform**
  inspectable / routable な effect integration 層

repo が主として維持しているのは、Mir current-L2 の **repo-local alpha-ready current layer**、Mirrorea Spaces の **product alpha release-candidate workflow**、その次段の **canonical operational product sample suite**、現在 rebaseline 済みの **Mir Computational Core docs/spec line**、closed **Full System V1 source-first bounded release-check line**、そして closed **Surface Mir brace syntax / source-authority alpha evidence line** です。
これは final public product ではありませんが、docs-only の構想メモでもありません。active sample、helper CLI、Lean foundations、product alpha CLI、operational suite helper、release-check、report 群を通して、現時点でどこまで実装と検証が進んでいるかを repo 内で再確認できます。
current alpha-1 usable surface は、developer-built `mirrorea-alpha` binary、versioned `package.mir.json`、local/Docker controlled runtime、observer-safe devtools/viewer、R0/R2 save evidence、native host launch bundle、product release check、installed-binary probe、operational suite helperです。operational suite は six runnable roots、shared attach packages、projection inventory、template-only starter catalog、retained portal/shard future inventory を持ちます。backend については `native host launch bundle` だけが actualized で、WASM / LLVM は docs-first inventory に留めます。broader distribution と final shared-space catalog breadth は user-spec-required gate です。一方で、current typed external `AddOne` は host-boundary evidence であり、Mir-owned arithmetic / variables / arrays / records / control-flow の completion 証拠ではありません。最小実用パターンを横断検証する入口として `scripts/minimal_alpha1_patterns.py check-all --format json` と `docs/hands_on/minimal_alpha1_patterns_01.md` を追加しています。

Full System V1 は、この alpha floor を final product と誤読せず、Mir source files を semantic source of truth に戻すための次段 roadmap です。`package.mir.json` は alpha compatibility / package artifact として残し、textual Mir alpha grammar、typed IR、interpreter、effectful runtime integration、PoseGraph runtime、projection IR、provider admission、renderer pose backend、devtools / release check を staged に進めます。`P-MIR-01` により `crates/mir-ast::textual_alpha` と parser-floor lane が actualize され、`P-MIR-02` により `crates/mir-semantics::full_system_v1`、`scripts/full_system_v1_samples.py`、`samples/full-system-v1/computational/typed-ir-matrix.json` の checker-floor lane が actualize され、`P-MIR-03` により source-derived pure interpreter lane が actualize され、`P-MIR-04` により transition/effect lane が `runtime-matrix.json`、`expected/run.json`、`crates/mir-runtime::full_system_v1_session`、compute trace、effect-session summary、static/runtime rejection split、host read/write、publish/observe、witness/handoff、local atomic-cut negative rowsまで widened され、`P-POSE-03` と `P-POSE-04` により `crates/mir-runtime::posegraph_runtime`、`samples/full-system-v1/avatar-pose/`、`scripts/posegraph_runtime_samples.py` の runtime PoseGraph lane が pose-aware save/load admissibility と observer-safe devtools export まで actualize され、`P-PROJ-02` により `crates/mir-semantics::full_system_v1::projection`、`crates/mir-runtime::full_system_v1_projection`、`samples/full-system-v1/projection/`、`scripts/projection_v1_samples.py` の projection IR lane が source-derived target manifest、preservation report、client-write authority rejection、unassigned-place rejection、save/load ownership rejectionまで actualize され、`P-PROJ-03` によりその same root が packet schema、FFI schema、payload-shape mismatch rejection、same-shape heterogeneous effect-contract rejectionまで widened され、`P-PROJ-04` により `crates/mir-runtime::full_system_v1_local_split`、`samples/full-system-v1/server-client/`、`mirrorea-alpha run-full-v1-split` の same-binary local role-split lane が 1 accepted row と 1 undeclared-entry rejection rowまで actualize され、`P-ENG-02` により `crates/mir-runtime::full_system_v1_provider_admission`、`samples/full-system-v1/provider-adapter/`、`scripts/provider_admission_samples.py`、`mirrorea-alpha admit-provider-v1` の bounded provider-admission lane が viewer-diagnostic inventory accepted row、WASM inventory-only accepted row、over-capability rejection、missing rollback policy rejection、native-disabled rejectionまで actualize され、`P-ENG-03` により `crates/mir-runtime::full_system_v1_renderer_pose_backend`、`samples/full-system-v1/provider-adapter/renderer-pose-*/`、`scripts/renderer_pose_backend_samples.py`、`mirrorea-alpha render-pose-backend-v1` の bounded renderer pose backend lane が 1 accepted structural binding-context + snapshot-frontier row と 2 blocked rowsで actualize され、`P-FSV1-01` が `world-core/`、`membership-chat/`、`sugoroku-world/` を、`P-FSV1-02` が `portal-worldlink/`、`two-shard-hard-boundary/`、`gradient-observation/` を actualize したことで、bounded source-first operational suite は 12 executable rows と generated package-manifest/runtime expectations を持つようになりました。`P-FSV1-03` はその上に `scripts/full_system_v1_release_check.py`、per-command JSON reports、static `bundle.json` / `index.html` viewer、Product Alpha compatibility anchors、そして representative `mirrorea-alpha project-full-v1` / `run-full-v1-split` / `admit-provider-v1` / `render-pose-backend-v1` surfaces を束ねる bounded release-check workflow を actualize し、`P-FSV1-99` はその lane を full validation / claim-non-claim audit / docs-report cleanup で close しました。ここでの accepted rows は source/provider/entry/boundary の binding context と frontier 一致、bounded operational world bootstrap / room-message transform / roll-publish-handoff/local-cut / portal resolve-admit-fallback / two-shard offer-prepare-commit / observer-only gradient view を示す evidence であり、TwoShard old-owner/stale-config と Gradient write-reject/stale-view-drop は observer-visible reject-event narration として記録される一方、enforced negatives は `missing_live_witness` と freshness `contract_require_failed` に留まります。attested PoseGraph package provenance や final product workflow completion はまだ主張しません。現在 runnable なのは 2 positive / 8 negative の parser matrix、3 positive / 9 negative の checker matrix、8 positive / 9 negative の bounded effectful runtime matrix、12-row source operational suite、5 accepted / 1 violation / 3 runtime rejection の PoseGraph runtime matrix、projection IR 4 rows + local role-split 2 rows を含む 6-row projection/backend helper、2 accepted / 3 rejection の provider-admission helper、1 accepted / 2 blocked の renderer-pose helper、そして accepted Full System V1 release-check であり、Full System V1 autonomous chain は `P-FSV1-99 final audit` まで close 済みです。

Surface Mir alpha は、この closed Full System V1 floor の上に置く user-facing source authority line です。Canonical place-scope syntax は `S { ... }` で、`S[ ... ]` は sugar としても採用しません。`[]` は配列・Map・indexed state・role instance head の value-level indexing に残します。`P-SURF-01` で parser floor、`P-SURF-02` で indexed-state semantic checker floor、`P-SURF-03` で Surface-to-Core elaboration evidence floor、`P-SURF-04` で generated `MessageEnvelope` / visible publish / observe / `VisibilityDenied` evidence floor、`P-SURF-05` で role admission / capability grant report-level evidence floor、`P-SURF-06` で source patch hot-plug evidence floor、`P-SURF-07` で source-first operational evidence floor、`P-SURF-08` で Surface source / generated Core IR / semantic-checker-backed indexed-state map / generated communication / role admission / redacted patch lifecycle / source spans の static devtools diagnostics evidence floor を actualize し、`P-SURF-99` で full validation / claim-non-claim audit を close 済みです。
Surface Mir は user-facing source、Core Mir は elaboration target であり、通信・publish・observe は Surface から自動生成して Core IR / devtools に明示します。`state player[p: Participant]: Player` は S-owned Participant-indexed map で、key は authority ではありません。role claim は authority ではなく、authority は admission locus からの capability grant です。source patch hot-plug は direct eval ではなく parse / typecheck / elaborate / admit / activation_cut pipeline として扱います。
post-`P-SURF-99` の G1 LAB work では E-ROW failure-row diagnostics に non-final `lab_diagnostic_details` carrier、request / failure-row context、`E-ROW-002` / `VisibilityDenied` singleton (`ELAB-10`) と `E-ROW-001` non-visibility singleton (`ELAB-13..16`) の LAB-only `suggested_repair[]` evidence、OBL-024 / OBL-025 compile-check-only Lean statement drafts、OBL-001/020/021 statement guard hardening、ordinary assignment claim-family drilldown、repair shape / mixed-multi / set-insertion inventory、`ELAB-07` set-insertion gate review、`ELAB-04` mixed visibility branch inventory、`ELAB-07` set-insertion executable preflight、`ELAB-07` set-insertion assumption acceptance、`ELAB-07` set-insertion payload-model design、`ELAB-07` set-insertion executable payload prototype、`ELAB-07` set-insertion negative-guard hardening、`ELAB-07` set-insertion row-identity guard hardening、`ELAB-07` set-insertion exact-locus guard hardening、`ELAB-07` child / bundle / partial exclusion fixtures、`ELAB-04` mixed visibility payload-model preflight、OBL-025 branch-local non-coverage refinement、OBL-025 repair completeness guard hardening、OBL-024 executable diagnostic-soundness projection carrier、OBL-024 projection Rust fixture guard hardening、OBL-024 replay vocabulary preflight、OBL-024 Lean replay vocabulary refinement、OBL-024 Lean association vocabulary refinement、OBL-024 association guard hardening を追加しましたが、これは final diagnostic/repair ABI、OBL-024/025 discharge、general set-insertion support、bundle semantics support、visibility-repair ranking、G1 exit ではありません。`ELAB-04` は executable output では引き続き no-repair です。`plan/118` は `plan/70` の ordinary assignment row を traceability-only に分解する LAB memory で、G0/G1 exit、canon edit、OBL status movement、conformance、proof 昇格は主張しません。`plan/100` は `ELAB-07` だけについて one existing row-field edit / `element_insert_count = 3` の LAB source-locus edit assumption を受け入れ、`plan/101` は payload を one top-level set item / test matrix として設計し、`plan/102` は exact `ELAB-07` fact pattern だけに non-final `set_insertion` `suggested_repair[]` を実装し、`plan/103` は subset / padded / duplicate / multi-request variants に `set_insertion` repair を出さない Rust-only guard evidence を追加し、`plan/104` は public `target_ref` を変えずに internal association key を existing `when` source span で狭め、`plan/105` は current Surface-expressible omitted-row / retargeting proxies に `set_insertion` repair を出さない exact-locus guard を追加し、`plan/106` は exact `ELAB-07` payload が child singleton alternatives / bundle fields / partial guidance / textual-only guidance ではなく one complete top-level set item のままであることを Rust-only shape guard で固定し、`plan/107` は `ELAB-04` の mixed wrapper / base branch / visibility branch / association / ordering deferral を docs-only で整理し、`plan/108` は OBL-025 Lean draft に abstract branch-local non-coverage helper vocabulary を追加し、`plan/117` は OBL-001/020/021 statement drafts の body-level drift を sync guard で抑え、`plan/116` は placeholder repair arrays / repair ranking / all-repairs names / branch-local whole-gap coverage drift を sync guard で抑え、`plan/109` は OBL-024 Lean draft に diagnostic projection / reported failed premise / trace-local replay vocabulary を追加し、`plan/110` は current E-ROW `lab_diagnostic_details` に LAB-only `diagnostic_soundness_projection` を追加し、`plan/111` は `ELAB-04/07/10/13..16` の Rust fixture guard を強化し、`plan/112` は `trace_local_replay` を report-local anchor として future proof-level replay relation から分離し、`plan/113` は OBL-024 Lean draft に `ReportLocalReplayAnchor` と `ProofLevelReplayWitness` / `ProofLevelReplayRelation` の split を反映し、`plan/114` は `ReportLocalAssociationKey` と `ProofLevelAssociationWitness` / `ProofLevelAssociationRelation` の split を反映し、`plan/115` は key equality / branch-local association key への drift を static guard で抑えました。Surface alpha の LAB evidence / repository memory は `specs/39..43` と `plan/64..68`、G1 LAB memory は `plan/70..118` です。current promoted Surface package はありません。

## 現在の到達点

- active sample suite は `samples/clean-near-end/`
- runnable sample dashboard は `samples_progress.md`
- first strong typing layer は **finite decidable index fragment**
- authority hierarchy / security label hierarchy / capture / region / cost は **user-defined finite theory**
- order / handoff は `publication_order`、`witness_order`、`scoped_happens_before` などの高水準関係で扱う
- mutex / weak-memory / broken mutex は **model-check second line**
- Sugoroku world vertical slice は `samples/clean-near-end/sugoroku-world/`
  と `scripts/sugoroku_world_samples.py` で repo-local に実行可能
- Lean 側は
  - `samples/lean/foundations/` の小さな実証明
  - `samples/lean/clean-near-end/` の generated theorem stub
  に分かれている

## 明示的にまだ完了していないもの

- final public parser grammar
- final Surface Mir devtools viewer / telemetry ABI beyond the P-SURF-08 static diagnostics evidence floor
- final source patch hot-plug ABI / distributed durable migration planner / production patch registry
- final public parser / checker / runtime / verifier API
- final public auth / adapter / visualization / projection / hot-plug / transport surface
- full dependent type theory
- concrete theorem prover / model-checker への production binding
- low-level `memory_order_*` を source principal syntax としてどう公開するか
- final public witness / provider / emitted-artifact contract
- final public packaging / installed distribution hardening
- broader public distribution beyond the current developer-built binary + generated host launch bundle
- FFI / engine adapter / host integration target
- final shared-space operational catalog breadth
- broader Mir-owned computational publish/observe/witness/handoff widening and PoseGraph save/load/devtools evidence beyond the current bounded local/source-first runtime rows
- Full System V1 final packet/FFI transport semantics, executable server/client split, and broader source-first sample realization beyond the current bounded release-check workflow
- final Surface Mir devtools viewer / telemetry ABI beyond the current parser + indexed-state + elaboration + source operational + static diagnostics evidence floors
- LLVM/native codegen and server/client split compiler completion
- arbitrary WASM execution

## Mir Computational Core rebaseline

2026-05-21 時点の current docs/spec line は、front-half scaffold actualization と implementation half を all-up closeout audit まで閉じた段階です。Product Alpha-1 の runnable workflow はそのまま execution / observation floor として保持しつつ、`samples/product-alpha1/computational/` では `host input -> Mir add_one -> host output` の direct product-alpha row、variables / arrays / records / control-flow / imports の positive / negative first-floor rows、そして `host input -> Mir transform -> host output` を declared effect / failure / capability boundary と合わせて扱う accepted/check-rejection rows が actualize されました。ここで `required_capabilities` / `failure_tag` は checker-admission boundary declaration の evidence であり、broad effectful runtime semantics completion を意味しません。PoseGraph では `pose-04` accepted row と `pose-05` violation-export row が helper-backed no-split-frame evidence として actualize され、projection / engine-adapter は引き続き planned-only inventory に保たれています。

LAB evidence / repository memory は次です。

- `specs/28-mir-computational-core.md`
- `specs/29-transform-posegraph-semantics.md`
- `specs/30-projection-and-backend-boundary.md`
- `specs/31-engine-wasm-ffi-adapter-boundary.md`
- `plan/53-mir-computational-core-roadmap.md`
- `plan/54-transform-posegraph-roadmap.md`
- `plan/55-projection-backend-roadmap.md`
- `plan/56-engine-adapter-roadmap.md`
- `specs/32-autonomous-execution-and-completion-contract.md`
- `plan/57-autonomous-computational-core-master-plan.md`

`P-COMP-01`、`P-POSE-01`、`P-PROJ-01`、`P-ENG-01` は front-half close 済みで、そのうえで `P-COMP-02` が `samples/product-alpha1/computational/add-one-pure-mir/` を executable に昇格させ、`P-COMP-03` が variables / arrays / records / control-flow / imports の 10 row を helper-executable positive / runtime-rejection evidence に広げ、`P-COMP-04` が direct product-alpha accepted 1 row と `check`-time rejection 3 row で host read/write boundary を explicit にし、`P-POSE-02` が `samples/product-alpha1/posegraph/` に accepted 1 row と violation-export 1 row を加えました。all-up closeout audit では focused helper suite、Cargo regression、product alpha release check、installed-binary probe、operational suite、docs validator を再実行して current chain 全体を再確認しています。`scripts/mir_computational_samples.py check-all --format json` は accepted 7 row、expected runtime rejection 5 row、expected check rejection 3 rowを確認でき、`scripts/posegraph_samples.py check-all --format json` は accepted 1 row、violation 1 row、planned 7 rowを確認できます。current self-driven chain はここで close 済みで、残る stop line は final grammar / projection code generation / public ABI / later PoseGraph save-load-devtools widening です。

`P-PAT-01` はこの closed chain の上に置く薄い sample verification package です。`scripts/minimal_alpha1_patterns.py` は computational / PoseGraph / projection / engine-adapter の期待行数、expected rejection、inventory-only 境界を exact に確認します。product release-candidate と operational Sugoroku workflow は workflow anchor として同じ matrix に出しますが、default check では heavy workflow を毎回走らせません。必要な場合は `check-all --include-workflows --out <dir>` で含めます。

Autonomous execution の default は `specs/32` / `plan/57` に置きます。一度実行を依頼された後は package-by-package で進み、final distribution / final catalog / final ABI のような user-spec-required gate は lower-layer implementation を止めずに隔離します。

## Full System V1 rebaseline

`P-FS-00` は実装開始前の roadmap rebaseline です。current truth は次のように読む。

- Product Alpha-1 release-candidate workflow は useful alpha floor であり、final product ではない。
- `typed_host_io.add_one` は host-boundary evidence であり、Mir-owned computation の証明ではない。
- 現行 computational rows は first-floor evidence であり、Rust-level language completion ではない。
- Unity / Unreal / WASM / native / FFI は semantic owner ではなく typed backend/provider boundary である。
- Direct LLVM/native codegen は重要だが、typed IR / projection / boundary schema が先である。
- Debug / devtools は optional polish ではなく、開発・検証基盤である。

LAB evidence / repository memory は次です。

- `specs/33-full-system-v1-scope.md`
- `specs/34-textual-mir-alpha-grammar.md`
- `specs/35-mir-typed-ir-and-interpreter.md`
- `specs/36-projection-ir-and-boundary-preservation.md`
- `specs/37-posegraph-runtime-semantics.md`
- `specs/38-engine-provider-admission.md`
- `plan/58-full-system-v1-roadmap.md`
- `plan/59-textual-mir-roadmap.md`
- `plan/60-computational-runtime-roadmap.md`
- `plan/61-posegraph-runtime-roadmap.md`
- `plan/62-projection-backend-roadmap.md`
- `plan/63-engine-provider-roadmap.md`

reader-facing summary は `docs/hands_on/full_system_v1_roadmap_01.md` と `docs/research_abstract/full_system_v1_roadmap_01.md` です。current source-first commands は `python3 scripts/textual_mir_samples.py check-all --format json`、`python3 scripts/full_system_v1_samples.py operational-matrix --format json`、`python3 scripts/full_system_v1_samples.py check-operational-all --format json`、`python3 scripts/full_system_v1_samples.py check-all --format json`、`python3 scripts/posegraph_runtime_samples.py check-all --format json`、`python3 scripts/projection_v1_samples.py check-all --format json`、`python3 scripts/provider_admission_samples.py check-all --format json`、`python3 scripts/renderer_pose_backend_samples.py check-all --format json`、`python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release`、`cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture`、`cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`、`cargo test -p mir-runtime --test posegraph_runtime -- --nocapture`、`cargo test -p mir-runtime --test projection_ir -- --nocapture`、`cargo test -p mir-runtime --test provider_admission -- --nocapture`、`cargo test -p mir-runtime --test renderer_pose_backend -- --nocapture`、`cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`、`cargo run -q -p mirrorea-cli -- run-full-v1-split samples/full-system-v1/server-client/role-split-positive/main/src/role-split-positive.mir --request samples/full-system-v1/server-client/role-split-positive/projection.request.json --input 40 --format json`、`cargo run -q -p mirrorea-cli -- admit-provider-v1 samples/full-system-v1/provider-adapter/viewer-diagnostic-positive/main/src/viewer-diagnostic-positive.mir --request samples/full-system-v1/provider-adapter/viewer-diagnostic-positive/projection.request.json --provider samples/full-system-v1/provider-adapter/viewer-diagnostic-positive/provider.manifest.json --format json`、`cargo run -q -p mirrorea-cli -- render-pose-backend-v1 samples/full-system-v1/provider-adapter/renderer-pose-positive/main/src/renderer-pose-positive.mir --request samples/full-system-v1/provider-adapter/renderer-pose-positive/projection.request.json --provider samples/full-system-v1/provider-adapter/renderer-pose-positive/provider.manifest.json --posegraph-package samples/full-system-v1/provider-adapter/renderer-pose-positive/package.mir.json --format json` で、Full System V1 autonomous chain は `P-FSV1-99 final audit` まで close 済みです。

## Mirrorea の次軸

Mirrorea future-axis は current promoted line ではなく、docs-first / repo-local integration の roadmap-memory family です。2026-05-05 時点では、その中に 4 つの line を分けて持ちます。

- **Mirrorea Spaces alpha-0 evidence line**
  規範判断は `specs/13..17`、repository memory は `plan/39..43`、phase-indexed sample scaffold は `samples/alpha/` に置きます。これは active runnable root の置換ではなく、current-scope evidence を蓄積する alpha-local scaffolding です。
- **Mirrorea Spaces practical alpha-1 line**
  規範判断は `specs/18-practical-alpha1-scope.md`、repository memory は `plan/44-practical-alpha1-roadmap.md` に置きます。ここは source front-door、checker、runtime、package/hot-plug、transport、devtools、local save/load、product preview を揃える first-floor toolchain line であり、operational α-0.5 / α-0.8 readiness そのものではありません。
  ただし、これは promoted work queue であり、active canonical runnable root への昇格を意味しません。2026-05-05 時点の latest package closeout は `P-A1-23` practical α-1 integrated workflow carrier です。
  2026-05-05 時点の practical alpha-1 evidence は `SRC` / `CHK` / `RUN` / `HP-A1` / `TR-A1` / `VIS-A1` / `SL-A1` / `AV-A1` / `PE2E` families に分かれます。詳細な row 一覧と carrier split は `samples/practical-alpha1/README.md`、検証 dashboard は `samples_progress.md`、長期 memory は `plan/44-practical-alpha1-roadmap.md` に集約します。
  root では、`PA1W-01..08` が first-floor evidence と operational α line を bounded practical developer workflow に束ねていることだけを押さえます。これは final public parser / viewer / telemetry ABI、distributed durable save/load、WAN/federation、product/public-ready α-1 を意味しません。

- **Operational alpha theory-freeze / session-runtime line**
  規範判断は `specs/19..24`、repository memory は `plan/45..49` に置きます。ここでは runtime を広げずに、verification stratification、`atomic_cut` / consistent cut / save-load semantics、auth / rate-limit / debug の contract-transformer 理論、typed observability、typed external host boundary、そして α-0.5 / α-0.8 / α-0.9 の operational readiness 条件を固定します。
  `P-A1-19`、`P-A1-20`、`P-A1-21`、`P-A1-22` により、same-session α-0.5 session carrier、typed external `AddOne` host-I/O adapter lane、debug / auth / rate-limit / object preview / deferred detach の same-session attach lane、そして event DAG / local route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / observer-safe redacted view / retention-on-demand trace の session-bound devtools export が actualize され、bounded operational α-0.5 / α-0.8 / α-0.9 line は揃いました。`AddOne` は host-boundary evidence であり、Mir-owned computational-core completion ではありません。`P-A1-23` はその line と practical first floors を bounded practical α-1 workflow として束ねました。final public viewer / telemetry ABI、durable audit、distributed durable save/load、final-public product hardening は引き続き未完です。

- **Product/Public-ready Mirrorea Spaces alpha-1 line**
  規範判断は `specs/25..27`、repository memory は `plan/50..52` に置きます。current line は `mirrorea-alpha`、versioned `package.mir.json`、local/Docker controlled runtime、non-final devtools/viewer、R0/R2 save evidence、native host launch bundle、product release check、installed-binary probe、operational product sample suiteを持ちます。
  product demo は `samples/product-alpha1/demo/`、operational suite は `samples/product-alpha1/operational/` です。operational suite は `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation` の six runnable roots、shared attach packages、projection inventory、template-only starter catalog、retained future inventoryを持ちます。
  current delivery unit は developer-built `mirrorea-alpha` binary + locally generated native host launch bundleだけです。current catalog scope は bounded product alpha-1 narrow showcaseです。broader distribution / final shared-space operational catalog breadth は user-spec-required gateであり、final textual `.mir` grammar、final public ABI、WAN/federation、distributed durable save-load、arbitrary native execution、signature-is-safety、final viewer / telemetry serviceは claimしません。

現行の Stage A..F は current-scope evidence、practical alpha-1 first-floor row は first-floor evidence として読みます。helper / sidecar / report / expected JSON / first-floor runner は completion ではなく evidence です。`100%` は、外部開発者がその layer を実際に再現・使用できる operational workflow または product/public layer だけに使います。live queue authority と next reopen point は `progress.md` / `tasks.md` を参照してください。

current line で reader が押さえるべき点は次です。

- 境界条件:
  `Place` は participant と同一ではなく、state / queue / capability / visibility / observation frontier を持つ execution locus として読みます。standard I/O は Mir core に入れず、external world とは typed effect adapter で接続します。
- security / observation split:
  authentication / authorization / membership / capability / witness は transport に潰さず、visualization / telemetry も `label` / `authority` / `redaction` / `retention` を持つ typed effect layer として扱います。
- current repo-local evidence surfaces:
  Sugoroku world runtime attachment vertical slice、avatar fairy follow representative slice、typed external `EXT-03/04` preview、network `NET-02..05` canary、projection/codegen manifest bridge evidence、viewer prototype inventory、hot-plug runtime narrow floor は runnable または closeout-backed です。加えて `samples/alpha/` には alpha-local theory-freeze / checker skeleton / runtime roadmap 用の sample matrix scaffold を置きますが、これは expected-verdict 付き skeleton であり、まだ runnable root ではありません。`FAIRY-05` は `samples/not_implemented/avatar-fairy-follow/` に residual planned family として残しています。
  projection/codegen current `equivalence` reading は committed generated manifest と helper/report-local anchor の review-category alignment inventory に限り、generated place-program synthesis、placement optimizer、deployment planner、cross-place equivalence checker、proof completion、final emitted executable family、final public emitted-program ABI ではありません。
- current runtime / carrier floor:
  `TermSignature` / `LayerSignature` / `MessageEnvelope` / `AuthEvidence` / helper `verification_handoff_witness` / runtime `verification_model_check`、`MembershipRegistry` / `PlaceCatalog` / `LogicalPlaceRuntimeShell`、engine-neutral `HotPlugRequest` / `HotPlugVerdict`、runtime-side hot-plug skeleton/engine reports までは repo-local actualization 済みです。
- kept-later boundaries:
  final public auth / adapter / visualization / projection / hot-plug / transport surface、real socket / session / durable replay、rollback / durable migration / distributed activation ordering、final public viewer API / telemetry service、exact host schema、final public packaging / backend / distribution hardening は unresolved または deferred のままです。

詳細は次を使い分けてください。

- short current snapshot:
  `Documentation.md`
- live status / next reopen point:
  `progress.md`、`tasks.md`
- practical alpha-1 scope / roadmap:
  `specs/18-practical-alpha1-scope.md`、`plan/44-practical-alpha1-roadmap.md`
- hands-on closeout commands:
  `docs/hands_on/current_phase_closeout_01.md`
- reader-facing roadmap summary:
  `docs/research_abstract/mirrorea_future_axis_01.md`
- operational alpha theory freeze / roadmap:
  `specs/19-verification-stratification.md`、`specs/20-cut-save-load-semantics.md`、`specs/21-auth-layer-algebra.md`、`specs/22-observability-devtools-semantics.md`、`specs/23-typed-external-host-boundary.md`、`specs/24-operational-alpha05-alpha08-readiness.md`
  `plan/45-operational-alpha05-roadmap.md`、`plan/46-operational-alpha08-roadmap.md`、`plan/47-operational-alpha09-devtools-roadmap.md`、`plan/48-theory-freeze-proof-obligations.md`、`plan/49-host-io-and-session-runtime-roadmap.md`
- product/public alpha-1 boundary:
  `specs/25-product-alpha1-public-boundary.md`、`plan/50-product-alpha1-public-boundary-roadmap.md`
- operational product sample suite:
  `specs/26-operational-product-sample-suite.md`、`specs/27-spatial-portal-and-shard-extension-boundary.md`
  `plan/51-operational-product-sample-roadmap.md`、`plan/52-portal-spatial-world-roadmap.md`
- Mir computational core / PoseGraph / projection-backend boundary:
  `specs/28-mir-computational-core.md`、`specs/29-transform-posegraph-semantics.md`、`specs/30-projection-and-backend-boundary.md`、`specs/31-engine-wasm-ffi-adapter-boundary.md`
  `plan/53-mir-computational-core-roadmap.md`、`plan/54-transform-posegraph-roadmap.md`、`plan/55-projection-backend-roadmap.md`、`plan/56-engine-adapter-roadmap.md`
- autonomous execution contract:
  `specs/32-autonomous-execution-and-completion-contract.md`、`plan/57-autonomous-computational-core-master-plan.md`
- Full System V1 source-first roadmap:
  `specs/33-full-system-v1-scope.md`、`specs/34-textual-mir-alpha-grammar.md`、`specs/35-mir-typed-ir-and-interpreter.md`、`specs/36-projection-ir-and-boundary-preservation.md`、`specs/37-posegraph-runtime-semantics.md`、`specs/38-engine-provider-admission.md`
  `plan/58-full-system-v1-roadmap.md`、`plan/59-textual-mir-roadmap.md`、`plan/60-computational-runtime-roadmap.md`、`plan/61-posegraph-runtime-roadmap.md`、`plan/62-projection-backend-roadmap.md`、`plan/63-engine-provider-roadmap.md`
- Surface Mir alpha source-authority roadmap:
  `specs/39-surface-mir-placement-elaboration.md`、`specs/40-indexed-state-semantics.md`、`specs/41-role-admission-and-capability-grant.md`、`specs/42-source-patch-hotplug-semantics.md`、`specs/43-surface-mir-v1-alpha-scope.md`
  `plan/64-surface-mir-placement-roadmap.md`、`plan/65-indexed-state-roadmap.md`、`plan/66-role-admission-roadmap.md`、`plan/67-source-patch-hotplug-roadmap.md`、`plan/68-surface-full-system-v1-roadmap.md`
- future-axis repository memory:
  `plan/28-post-p18-true-user-spec-hold-option-matrix.md` と `plan/29..68`

## 何が built-in で、何が user-defined か

current clean near-end layer では、次を built-in vocabulary として扱います。

- `module`
- `index`
- `policy`
- `principal`
- `resource`
- `effect`
- `place`
- `option`
- `chain`
- `fallback`
- `lineage`
- `perform`
- `via`
- `require`
- `ensure`
- `atomic_cut`
- `transition`
- `stage`
- `publish`
- `observe`
- `handoff`
- `witness`
- `model`
- `property`

一方で、次のような domain vocabulary は built-in ではありません。

- `SecurityLabel`
- `FingerprintAuthority`
- `CaptureScope`
- `Region`
- `CostBudget`
- `FingerprintReleasePolicy`
- `Public`
- `KeyMaterial`
- `Observer`
- `Releaser`
- `Admin`
- `RoomHistory`
- `EphemeralToken`

つまり、旧来の権限専用 predicate 名を magical builtin として言語が暗黙に持つのではなく、sample 側が有限理論として宣言し、その上で checker / helper が読む構成です。

## まず実行するコマンド

active clean near-end suite の確認:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/current_l2_guided_samples.py smoke-all --format json
python3 scripts/current_l2_guided_samples.py closeout --format json
```

family ごとの確認:

```bash
python3 scripts/clean_near_end_samples.py run typing --format json
python3 scripts/clean_near_end_samples.py run order-handoff --format json
python3 scripts/clean_near_end_samples.py run model-check --format json
python3 scripts/clean_near_end_samples.py run modal --format json
python3 scripts/clean_near_end_samples.py matrix --format json
```

Sugoroku world runtime attachment vertical slice:

```bash
python3 scripts/sugoroku_world_samples.py list
python3 scripts/sugoroku_world_samples.py check-all
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes
python3 scripts/sugoroku_world_samples.py model-check
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/visual_debugger_viewer_samples.py closeout --format json
```

sample code viewer:

```text
mir_hilight.html
```

`mir_hilight.html` は repo 直下の単一 HTML です。ブラウザで開くと
`samples/clean-near-end/**/*.mir` の current active sample を Solarized Dark
標準で表示し、行番号、スマホ対応、theme 切替、予約語と sample 内定義名の
別色 highlight を確認できます。custom source panel に任意の Mir 風コードを貼ると、
同じ highlighter で browser-local preview できます。CSS は外部 framework ではなく
HTML 内の hand-written original CSS です。final parser / checker ではなく readable viewer
です。文法、active sample path、reserved keyword、定義宣言形、custom input UI が変わった場合は、
HTML 内の embedded samples / syntax token list / symbol extraction rule と docs
を同じ task で更新してください。

Lean foundations と generated stub の同期:

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

practical alpha-1 front-door / checker / first local-runtime floor:

```bash
cargo test -p mir-ast practical_alpha1_front_door -- --nocapture
cargo test -p mir-ast practical_alpha1_checker -- --nocapture
cargo test -p mir-ast practical_alpha1_runtime_plan -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture
cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture
cargo test -p mir-ast --test practical_alpha1_transport_plan -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_transport -- --nocapture
cargo test -p mir-ast --test practical_alpha1_save_load_plan -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_save_load -- --nocapture
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_transport.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py check-all --format json
python3 scripts/practical_alpha1_save_load.py check-all --format json
python3 scripts/practical_alpha1_product_preview.py check-all --format json
python3 scripts/practical_alpha1_product_preview.py render-html PE2E-07 --format json
python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json
```

## 読み始める順序

この README の次は、原則として次の順で読みます。

1. `Documentation.md`
2. current status / roadmap / remaining steps を扱う task なら `progress.md` と `tasks.md`
3. phase recut / roadmap rewrite / progress/tasks reorganization を扱う task なら `.docs/progress-task-axes.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. 必要な subsystem spec と `plan/00-index.md`
10. practical alpha-1 を扱う task なら `specs/18-practical-alpha1-scope.md` と `plan/44-practical-alpha1-roadmap.md`
11. operational readiness / theory freeze を扱う task なら `specs/19..24` と `plan/45..49`
12. product/public-ready alpha-1 を扱う task なら `specs/25` と `plan/50`
13. operational product sample suite を扱う task なら `specs/26..27` と `plan/51..52`

task が specific `sub-agent-pro/*.md` handoff を名指しした場合は、その handoff を user 指示順で先に読みます。
ただし handoff は規範正本ではなく、必要な内容は `specs/` / `plan/` / docs / report へ mirror して扱います。

## いま参照すべき docs

- `Documentation.md`
  現在の repo を短く読むための入口
- `progress.md`
  現在地、rough progress、recent log
- `tasks.md`
  自走可能な package と mixed gate / user-spec gate の整理
- `samples_progress.md`
  phase / layer ごとの runnable sample、E2E、debug surface、build / storage 環境の dashboard
- `samples/README.md`
  active / base corpus / planned / prototype / archive / generated sample の置き場所
- `scripts/README.md`
  active runner、repo-local helper、detached loop、storage/env script の current taxonomy
- `docs/research_abstract/README.md`
  日本語での短い研究概要と `_detail` への導線
- `docs/hands_on/README.md`
  実行コマンド付きの hands-on landing page
- `docs/hands_on/current_phase_closeout_01.md`
  current phase closeout memory、remaining mixed gate、snapshot docs への入口
- `docs/hands_on/visual_debugger_viewer_01.md`
  `P16` typed public prototype inventory の最短入口
- `docs/hands_on/network_transport_canaries_01.md`
  phase 13 helper-local canary の入口
- `docs/reports/`
  実行証跡と変更履歴

## active path と archive path

- active sample:
  `samples/clean-near-end/`
- active base current-L2 corpus:
  `samples/current-l2/`
- active Sugoroku world vertical slice:
  `samples/clean-near-end/sugoroku-world/`
- active Lean material:
  `samples/lean/`
- alpha-local scaffold root:
  `samples/alpha/`
- planned skeleton family:
  `samples/not_implemented/` (residual planned family)
- prototype / compatibility anchor:
  `samples/prototype/`
- historical archive:
  `samples/old/2026-04-22-pre-clean-near-end/`
  と
  `samples/lean/old/2026-04-22-pre-clean-near-end/`
- generated artifact reserve:
  `samples/generated/`

`samples/not_implemented/` は archive ではなく planned family です。
archive は比較用の履歴であり、active canonical sample としては扱いません。
generated artifact reserve と helper-local preview は source sample と混同しません。
