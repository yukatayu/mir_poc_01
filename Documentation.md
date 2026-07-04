# ドキュメント要約

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins; cite legacy `specs/` /
`plan/` as `LAB:` evidence unless mirrored into canon.

## この文書の役割

この文書は、repo 全体の **current snapshot を短く正確に読む入口** です。

- 規範判断の正本は `mirrorea_canon/`
- 旧 `specs/` と `plan/` は LAB evidence / repository memory
- workflow / evidence snapshot は `progress.md`
- current task map は `tasks.md`
- runnable sample dashboard は `samples_progress.md`
- agent / operational policy は `.docs/`
- 実行証跡は `docs/reports/`

## まず repo をどう読むべきか

この repo は、Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform を **意図的に separable** に保った研究用 workspace です。主眼は Mir current-L2 と、その上に積む Mirrorea shared-space / hot-plug / observability / host-boundary line にあります。

読み分けで重要なのは、次の層を混同しないことです。

- **repo-local alpha-ready current layer**
  `samples/clean-near-end/`、helper、Lean foundation、report まで含めて動かせる current-L2 の実行足場
- **current-scope evidence**
  `samples/alpha/` と `specs/13..17` / `plan/39..43` による alpha-0 helper-local / runtime-private closeout 群
- **practical alpha-1 first-floor evidence**
  `samples/practical-alpha1/` と `specs/18` / `plan/44` による front-door / checker / runtime / hot-plug / transport / devtools / save-load / preview の first-floor toolchain
- **operational alpha readiness**
  LAB package-line memory in `specs/19..24` / `plan/45..49` が記録する α-0.5 / α-0.8 / α-0.9 の same-session operational 条件
- **product/public-ready Mirrorea Spaces alpha-1**
  LAB product-alpha line documented in `specs/25` / `plan/50` が記録する alpha-stable CLI、versioned package format、same-session product demo、quiescent save、viewer、native launch bundle、clean-clone validation。final textual `.mir` grammar、WAN/federation、distributed durable save/load、arbitrary native execution、final viewer / telemetry service は non-goal
- **operational product sample suite**
  LAB suite memory in `specs/26` / `plan/51` が記録する `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation` suite、shared attach packages、deployment/projection inventory、retained portal/shard blueprint inventory。current line では `MembershipChat` の bounded room-oriented `ChatText` host boundary、`SugorokuWorld` の bounded roll / publish / witness / handoff / stale membership reject scenario、`projection.profile.json` の schema-backed target / packet / FFI inventory、`PortalWorldLink` の bounded same-session discrete handoff cut、`TwoShardHardBoundary` の bounded same-session two-shard offer / prepare / commit cutとobserver-visible old-owner / stale-config reject-event narration付き `missing_live_witness` rejection、そして `TwoShardGradientObservation` の bounded observer-only gradient view / handoff hint rowsとobserver-visible write-reject / stale-view-drop narration付き freshness rejection が `check`、runtime plan、observer-safe devtools / helper closeout に actualize 済み。`future/portal-worldlink/` と `future/two-shard-hard-boundary/` blueprint は残す
- **Mir Computational Core rebaseline**
  legacy LAB docs/spec memory in `specs/28..31` / `plan/53..56` が記録する computational-core package line。Product Alpha-1 の operational floor は保持するが、current typed external `AddOne` を Mir-owned computation の証拠とは読まない。front-half scaffold は維持しつつ、`P-COMP-02` により `samples/product-alpha1/computational/add-one-pure-mir/` と `scripts/mir_computational_samples.py` が one direct executable Mir-owned row を持つようになり、`P-COMP-03` により variables / arrays / records / control-flow / imports の positive / negative rows が helper-executable に actualize され、`P-COMP-04` により direct product-alpha accepted/check-rejection rows で host read/write boundary が explicit になった。ここで `required_capabilities` / `failure_tag` は checker-admission boundary declaration の evidence であり、broad effectful runtime semantics completion を意味しない。PoseGraph は `P-POSE-02` により same-client same-observation-snapshot no-split-frame の accepted/violation helper evidence を持つようになり、projection / engine-adapter は引き続き planned-only scaffold / inventory である。all-up closeout audit まで実行済みで、current self-driven chain は閉じた。
- **minimal alpha-1 pattern verification**
  `scripts/minimal_alpha1_patterns.py` と `docs/hands_on/minimal_alpha1_patterns_01.md` は、closed chain の上に置く reader-facing verifier。computational / PoseGraph / projection / engine-adapter の exact row count、expected rejection、inventory-only boundary を確認し、product release-candidate と operational Sugoroku は workflow anchor として同じ matrix で読む。これは新しい runtime claim ではなく、最小実用パターンの drift 検出である。
- **autonomous execution contract**
  LAB execution memory in `specs/32` / `plan/57` が記録する package-by-package 自律実行 line。`P-COMP-01 -> P-POSE-01 -> P-PROJ-01 -> P-ENG-01` の front-half を閉じてから implementation half に入り、user に途中質問せず、final distribution / catalog / ABI / backend admission のような user-spec-required gate は隔離し、lower-layer implementation を止めない。
- **Full System V1 source-first roadmap**
  LAB roadmap memory in `specs/33..38` / `plan/58..63` が記録する次段 roadmap。Product Alpha-1 と operational suite を alpha floor として保持しつつ、semantic source of truth を Mir source files へ戻し、textual Mir alpha grammar、typed IR、pure interpreter、effectful runtime integration、PoseGraph runtime、projection IR、provider admission、devtools / release check へ段階的に進む。`package.mir.json` は alpha compatibility / package artifact であり、final source authority ではない。
- **Surface Mir alpha source-authority line**
  LAB roadmap memory in `specs/39..43` / `plan/64..68` が記録する closed alpha evidence roadmap。Canonical place-scope syntax は `S { ... }` であり、`S[ ... ]` は sugar としても採用しない。Surface Mir は user-facing source、Core Mir は elaboration target であり、通信・publish・observe は自動生成して Core IR / devtools に明示する。`package.mir.json` は alpha artifact に留まり、semantic source authority は `.mir` files に戻す。`P-SURF-01` は parser/helper/sample floor、`P-SURF-02` は indexed-state semantic checker/sample floor、`P-SURF-03` は Surface-to-Core elaboration evidence floor、`P-SURF-04` は generated MessageEnvelope / publish / observe / private-field rejection evidence floor、`P-SURF-05` は role admission evidence floor、`P-SURF-06` は source patch hot-plug evidence floor、`P-SURF-07` は source-first operational evidence floor、`P-SURF-08` は Surface source / generated Core IR / semantic-checker-backed indexed-state map / generated communication / role admission / redacted patch lifecycle / source spans の static devtools diagnostics evidence floor、`P-SURF-99` は final validation / claim-non-claim audit closeout である。Runtime MessageEnvelope dispatch、production identity provider、hardware attestation、WAN admission、final source patch ABI、distributed durable migration、final operational runtime/transport、final devtools viewer / telemetry ABI はまだ主張しない。
- **final public product**
  final parser grammar、public checker/runtime/verifier API、public adapter / viewer / projection / hot-plug / transport surface、packaging、external contract まで含む最終形

現在 repo は、repo-local alpha-ready current layer、current-scope evidence、practical alpha-1 first-floor evidenceに加えて、次の runnable workflow を持っています。

- bounded operational α-0.5 / α-0.8 / α-0.9
- bounded practical α-1 integrated workflow carrier
- product alpha-1 release-candidate workflow
- installed-binary + generated native host launch bundle adoption probe
- `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation` の canonical operational product sample suite
- minimal alpha-1 pattern verifier for strict row / rejection / inventory drift checks

Surface Mir alpha は別枠の evidence-closed line です。

- Surface Mir brace parser floor plus indexed-state semantic checker, Surface-to-Core elaboration, generated communication, role-admission, source-patch hot-plug, source operational, static devtools diagnostics evidence floors, and `P-SURF-99` final validation / claim-non-claim audit closeout

実用面では、外部開発者が documented commands で product demo と operational suite を `check`、`run-local`、`session`、`attach`、`save`、`quiescent-save`、`transport`、`export-devtools`、`view`、`build-native-bundle` まで再現できる段階です。current delivery unit は developer-built `mirrorea-alpha` binary + locally generated native host launch bundle だけで、current catalog scope は bounded product alpha-1 narrow showcase です。

まだ主張しないものは明確です。final public product、final textual grammar / ABI / SDK、final Surface operational runtime / transport、final Surface devtools viewer / telemetry ABI、final source patch hot-plug ABI、distributed durable patch migration、production patch registry/signing workflow、archive / installer / hosted service、final viewer / telemetry ABI、R3/R4 durable distributed save/load、WAN/federation、production identity provider、hardware attestation、arbitrary native package execution、arbitrary WASM execution、final server/client binary split、continuous spatial sync、direct LLVM backend は別 gate です。broader distribution / final shared-space catalog breadth は user-spec-required decision です。Full System V1 autonomous chain は `P-FSV1-99 final audit` まで close 済みです。
Surface Mir line は P-SURF-99 final audit の後、G1 dependency-gap evidence、OBL-020/021 dependency inventory、LAB statement drafts、OBL-001/020/021 statement guard hardening、OBL-001 boundary audit、OBL-020/021 boundary audit and OBL-021 guard hardening、G1 ordinary-assignment bridge readiness/non-readiness map、ordinary assignment claim-family drilldown、remaining claim-family priority map、repo-triage recut matrix、G1 minimal vertical slice candidate map、G1 SCN exact static slice manifest、G1 SCN-01 visibility negative actualization、G1 SCN-02 direct-local-write blocker review、E-ROW diagnostic alignment、diagnostic carrier inventory、OBL-024/025 statement-shape inventory、E-ROW repair payload inventory、E-ROW carrier-only diagnostic detail prototype、E-ROW carrier precondition hardening、E-ROW-002 visibility repair carrier prototype、OBL-024 / OBL-025 Lean statement draft、E-ROW repair shape inventory、E-ROW-001 non-visibility singleton fixture、E-ROW-001 base singleton fixture closure、E-ROW-001 singleton repair assumption gate、E-ROW-001 singleton repair prototype、E-ROW mixed / multi repair decomposition inventory、E-ROW set-insertion / bundle payload inventory、ELAB-07 set-insertion gate review、ELAB-04 mixed visibility branch inventory、ELAB-07 set-insertion executable preflight、ELAB-07 set-insertion assumption acceptance、ELAB-07 set-insertion payload-model design、ELAB-07 set-insertion executable payload prototype、ELAB-07 set-insertion negative-guard hardening、ELAB-07 set-insertion row-identity guard hardening、ELAB-07 set-insertion exact-locus guard hardening、ELAB-07 child / bundle / partial exclusion fixtures、ELAB-04 mixed visibility payload-model preflight、OBL-025 branch-local non-coverage refinement、OBL-025 repair completeness guard hardening、OBL-024 executable diagnostic-soundness projection carrier、OBL-024 projection Rust fixture guard hardening、OBL-024 replay vocabulary preflight、OBL-024 Lean replay vocabulary refinement、OBL-024 Lean association vocabulary refinement、OBL-024 association guard hardening を加え、53 sample rows / 54 `.mir` source files を持ちますが、これは alpha/LAB evidence and repository memory であり final runtime/transport/API、diagnostic/repair ABI freeze、OBL-020/021/024/025 discharge、proof discharge、G1 exit ではありません。`plan/118` は `plan/70` の ordinary assignment row を `plan/71 -> plan/72` 中心で分解した traceability-only 文書であり、`plan/119` は remaining `plan/70` rows を今すぐ drilldown しないための priority map であり、`plan/120` は Product Alpha / Full System V1 / Surface evidence の LAB management overlay、`plan/121` は G1 ordinary assignment に渡す `G1-MVS-ASSIGNMENT-STATIC` 候補 map、`plan/122` は SCN-01 / SCN-02 static bullets を exact / structural support / explicit gap に分ける manifest、`plan/123` は SCN-01 visible-write `VisibilityDenied` negative gap を `ELAB-17` で exact current executable evidence にする actualization record、`plan/124` は `ELAB-11/12/17` が既存 OBL-001 abstract predicate boundary で足りるため Lean predicate refinement 不要とする boundary audit、`plan/125` は SCN-02 direct-local-write negative (b) が現 G1 bridge の即時 blocker ではないが exact executable negative evidence でもないと整理する blocker review、`plan/126` は OBL-020/021 statement boundary を監査し、Lean predicate refinement 不要のまま OBL-021 guard weakness を test-only に補強する boundary audit / guard hardening、`plan/127` は post-`plan/126` の G1 bridge current LAB support / remaining blocker / forbidden claim を整理し、G1 exit readiness ではないことを明示する readiness/non-readiness map です。いずれも canon edit、G0/G1..G7 exit、OBL status movement、conformance、proof 昇格、sample status relabel は主張しません。`suggested_repair[]` evidence は `E-ROW-002` / `VisibilityDenied` singleton (`ELAB-10`)、exact SCN-01 visible-write `VisibilityDenied` negative (`ELAB-17`)、`E-ROW-001` non-visibility singleton (`ELAB-13..16`)、および exact `ELAB-07` set-insertion payload prototype に限って出ています。mixed visibility row (`ELAB-04`) には executable output では出していません。OBL-001/020/021 Lean statement drafts は body-level sync guard を持ちますが、proof skeleton / completion / final equality / runtime dispatch ではありません。OBL-024 Lean statement draft は diagnostic projection / reported rule / failed premise / bindings / report-local association key / future proof-level association relation / report-local replay anchor / future proof-level replay relation を抽象 predicate として持つ compile-check-only `Prop` 形であり、proof ではありません。current executable E-ROW carrier は `lab_diagnostic_details` に LAB-only `diagnostic_soundness_projection` を持ち、report-local diagnostic id / report-local association key / reported bindings / trace-local replay anchor を固定しますが、final Diagnostic JSON / association/replay ABI ではありません。`plan/112` はこの `trace_local_replay` を report-local anchor として future proof-level replay relation から分離し、proof-level exactness / replay engine / replay ABI は OPEN のまま残します。`plan/113` はその split を Lean draft に `ReportLocalReplayAnchor` と `ProofLevelReplayWitness` / `ProofLevelReplayRelation` として反映します。`plan/114` は current `lab_association_key` を report-local association key として future proof-level association relation から分離し、association-key ABI は OPEN のまま残します。`plan/115` は report-local association key が semantic association by key equality や branch-local association key に drift しないよう static guard を追加します。OBL-025 Lean statement draft は placeholder repair arrays / repair ranking / all-repairs names / branch-local whole-gap coverage drift を guard し、whole rejected gap / set insertion / grouped multi-edit / complete local repair / partial guidance non-coverage / branch-local non-coverage を抽象 predicate として持つ compile-check-only `Prop` 形であり、proof ではありません。non-visibility singleton repair は `plan/93` の LAB single-edit / no-placeholder gate を `plan/94` で通した current LAB prototype です。`plan/95` は mixed / multi repair 分解、`plan/96` は set-insertion / conjunctive bundle / partial guidance payload vocabulary、`plan/97..101` は `ELAB-07` の gate / assumption / payload design、`plan/98` は `ELAB-04` mixed branch no-repair gate、`plan/102` は exact `ELAB-07` fact pattern だけの non-final executable set payload prototype、`plan/103` は subset / padded / duplicate / multi-request variants に `set_insertion` repair を出さない Rust-only guard evidence、`plan/104` は public `target_ref` を変えない internal association key hardening、`plan/105` は omitted-row / retargeting proxies を exact current `ELAB-07` locus から外す guard、`plan/106` は exact `ELAB-07` payload が one complete top-level set item であり child singleton alternatives / bundle fields / partial guidance / textual-only guidance ではないことを固定する Rust-only shape guard、`plan/107` は `ELAB-04` の mixed wrapper / base branch / visibility branch / association / ordering deferral を docs-only で整理する preflight、`plan/108` は OBL-025 draft に abstract `RepairBranch` / branch-local non-coverage helper vocabulary を足す LAB refinement、`plan/117` は OBL-001/020/021 statement guard hardening、`plan/126` は OBL-020/021 boundary audit and OBL-021 guard hardening、`plan/127` は G1 bridge readiness/non-readiness map、`plan/116` は OBL-025 repair completeness guard hardening、`plan/109` は OBL-024 draft に diagnostic projection / trace-local replay vocabulary を足す LAB refinement、`plan/110` は executable E-ROW carrier に OBL-024 projection evidence を足す LAB hardening、`plan/111` はその projection evidence を Rust fixture guard で固定する test-only hardening、`plan/112` は replay vocabulary の docs-first preflight、`plan/113` は Lean replay vocabulary refinement、`plan/114` は Lean association vocabulary refinement、`plan/115` は association guard hardening です。general set-insertion support、repair ranking、multi-edit、bundle semantics support、visibility-repair ranking、whole-program repair success、final repair ABI はまだ主張しません。

## current active floor

active LAB clean sample evidence は `samples/clean-near-end/` です。base current-L2 corpus は `samples/current-l2/`、active Lean mechanization evidence と LAB-only statement-shape drafts は `samples/lean/` に置きます。

- `typing/`
  finite-index first strong typing layer
- `order-handoff/`
  publication / witness / handoff relation family
- `model-check/`
  Peterson / broken mutex による second-line verification
- `modal/`
  `stable` / `later` / `published(room)` / `witnessed(...)` の current mode line
- `sugoroku-world/`
  empty world server へ SugorokuGame を runtime attach する Mir / Mirrorea vertical slice
- `samples/lean/lab-statements/`
  compile-check only の Lean statement-shape draft。current draft は
  OBL-001 / OBL-020 / OBL-021 / OBL-024 / OBL-025 で、canon OBL status movement、proof discharge、
  final theorem contract ではない。

旧 active sample line は active path から外し、archive に退避しています。

## Mirrorea line の現在地

```text
OS/network substrate
  -> Mir core
  -> Typed external boundary
  -> Mirrorea fabric/runtime
  -> practical toolchain
  -> Spaces product
  -> Reversed Library
```

- **alpha-0 evidence line**
  LAB memory in `specs/13..17`、`plan/39..43`、`samples/alpha/` が current-scope evidence を記録します。Stage A..F は evidence reference であり、operational α-0.5 / α-0.8 / α-0.9 completion ではありません。
- **practical alpha-1 first-floor / workflow line**
  LAB memory in `specs/18`、`plan/44`、`samples/practical-alpha1/` が first-floor toolchain を記録します。`scripts/practical_alpha1_integrated_workflow.py check-all --format json` は bounded developer workflow を再現しますが、product/public-ready α-1 ではありません。
- **operational alpha theory-freeze line**
  LAB memory in `specs/19..24`、`plan/45..49` が α-0.5 local observable runtime、α-0.8 same-session hot-plug runtime、α-0.9 session-bound devtools readiness を記録します。対応 helper は bounded same-session workflows を再現します。
- **product alpha-1 line**
  LAB memory in `specs/25`、`plan/50`、`samples/product-alpha1/demo/`、`scripts/product_alpha1_release_check.py` が product alpha release-candidate workflow を記録します。Docker 込みの release check が accepted のとき、controlled alpha product workflow として実用確認済みと読めます。
- **operational product sample suite**
  LAB memory in `specs/26..27`、`plan/51..52`、`samples/product-alpha1/operational/`、`scripts/operational_product_samples.py` が canonical operational suite を記録します。six roots、shared attach packages、projection inventory、portal/shard/gradient cuts、template-only starter catalog、backend inventory、scope helper blocks を保持します。
- **Full System V1 source-first line**
  LAB memory in `specs/33..38`、`plan/58..63`、`progress.md`、`tasks.md` が current roadmap snapshot を記録します。latest closed package は `P-FSV1-99 final audit` で、current promoted package はありません。ここでは Product Alpha-1 を final product に昇格せず、textual Mir、typed IR、bounded effectful runtime、PoseGraph runtime、bounded pose save/devtools、projection IR + boundary schemas + bounded local role split、bounded provider admission、renderer pose backend、source-first operational suites、portal/shard/gradient source suites、bounded release check、最後に claim/non-claim audit まで自走で閉じました。
- **Surface Mir alpha line**
  LAB memory in `specs/39..43`、`plan/64..68`、`progress.md`、`tasks.md` が closed Surface alpha roadmap snapshot を記録します。`P-SURF-01` has actualized the parser floor in `crates/mir-ast::surface_alpha`, `samples/full-system-v1-surface/syntax/`, and `scripts/surface_mir_samples.py`; `P-SURF-02` has actualized the indexed-state semantic checker floor; `P-SURF-03` / `P-SURF-04` have actualized elaboration and generated communication evidence; `P-SURF-05` has actualized role admission / capability grant evidence; `P-SURF-06` has actualized source patch hot-plug evidence; `P-SURF-07` has actualized source operational roots for WorldCore / MembershipChat / Sugoroku / Portal / TwoShard / Gradient with `E2E-SURF-01..12`; `P-SURF-08` has actualized static devtools diagnostics rows `DEV-01..02`; `P-SURF-99` reran full validation and compatibility anchors. Current promoted Surface package is none. Canonical place-scope syntax is `S { ... }`; `S[ ... ]` is rejected and is not sugar.

## いま何があり、何がまだ無いか

既にあるもの:

- practical checker / runtime / hot-plug / transport / devtools / save-load / preview の **distinct carrier split**
- event DAG export、observer-safe route trace、membership timeline export、fallback degradation export、redacted observer view、report-local retention query trace
- local-only save/load roundtrip と stale-membership non-resurrection first-floor rows
- attach-time auth / rate-limit / object preview / deferred detach の first-floor rows
- bounded α-0.5 session carrier 上の minimal typed external `AddOne` host-I/O adapter lane
- bounded α-0.8 same-session hot-plug runtime over the same session carrier
- bounded α-0.9 session-bound devtools export over the same carrier
- bounded practical α-1 integrated workflow carrier over the existing first-floor and operational evidence

Correction:

- `AddOne` in current alpha evidence is a typed external host-I/O adapter lane.
- It is not evidence that Mir already owns general arithmetic, variables, arrays, records, or control-flow computation.
- The first future proof point is pure `add_one` represented and executed by Mir, with host input/output kept at typed external boundaries.

まだ無いもの:

- final public viewer / telemetry ABI
- durable audit backend / remote retained-artifact retrieval
- distributed durable save/load
- final-public product hardening beyond the product alpha release-candidate workflow
- final Surface Mir devtools viewer / telemetry ABI beyond the P-SURF-08 static diagnostics evidence floor
- final source patch hot-plug ABI / distributed durable migration planner / production patch registry

## 重要な境界

- `Place` は participant と同一ではなく、state / queue / capability / visibility / observation frontier を持つ execution locus として読む
- standard I/O は Mir core primitive ではなく、typed external adapter boundary 側に残す
- auth / authorization / membership / capability / witness は transport に潰さない
- visualization / telemetry は label / authority / redaction / retention を持つ typed effect として扱う
- `atomic_cut` は place-local rollback frontier であり、durable/distributed commit ではない
- local save/load は distributed durable save/load と同一ではない

## どこを次に読むか

- live status / next reopen point:
  `progress.md`、`tasks.md`
- runnable sample dashboard:
  `samples_progress.md`
- practical alpha-1 first-floor:
  `specs/18-practical-alpha1-scope.md`、`plan/44-practical-alpha1-roadmap.md`
- operational alpha theory freeze:
  `specs/19-verification-stratification.md`
  `specs/20-cut-save-load-semantics.md`
  `specs/21-auth-layer-algebra.md`
  `specs/22-observability-devtools-semantics.md`
  `specs/23-typed-external-host-boundary.md`
  `specs/24-operational-alpha05-alpha08-readiness.md`
  `plan/45-operational-alpha05-roadmap.md`
  `plan/46-operational-alpha08-roadmap.md`
  `plan/47-operational-alpha09-devtools-roadmap.md`
  `plan/48-theory-freeze-proof-obligations.md`
  `plan/49-host-io-and-session-runtime-roadmap.md`
- product/public alpha-1 boundary:
  `specs/25-product-alpha1-public-boundary.md`
  `plan/50-product-alpha1-public-boundary-roadmap.md`
- operational product sample suite:
  `specs/26-operational-product-sample-suite.md`
  `specs/27-spatial-portal-and-shard-extension-boundary.md`
  `plan/51-operational-product-sample-roadmap.md`
  `plan/52-portal-spatial-world-roadmap.md`
- Mir computational core / PoseGraph / projection-backend boundary:
  `specs/28-mir-computational-core.md`
  `specs/29-transform-posegraph-semantics.md`
  `specs/30-projection-and-backend-boundary.md`
  `specs/31-engine-wasm-ffi-adapter-boundary.md`
  `plan/53-mir-computational-core-roadmap.md`
  `plan/54-transform-posegraph-roadmap.md`
  `plan/55-projection-backend-roadmap.md`
  `plan/56-engine-adapter-roadmap.md`
- autonomous execution contract:
  `specs/32-autonomous-execution-and-completion-contract.md`
  `plan/57-autonomous-computational-core-master-plan.md`
- Full System V1 source-first roadmap:
  `specs/33-full-system-v1-scope.md`
  `specs/34-textual-mir-alpha-grammar.md`
  `specs/35-mir-typed-ir-and-interpreter.md`
  `specs/36-projection-ir-and-boundary-preservation.md`
  `specs/37-posegraph-runtime-semantics.md`
  `specs/38-engine-provider-admission.md`
  `plan/58-full-system-v1-roadmap.md`
  `plan/59-textual-mir-roadmap.md`
  `plan/60-computational-runtime-roadmap.md`
  `plan/61-posegraph-runtime-roadmap.md`
  `plan/62-projection-backend-roadmap.md`
  `plan/63-engine-provider-roadmap.md`
  `docs/hands_on/full_system_v1_roadmap_01.md`
  `docs/research_abstract/full_system_v1_roadmap_01.md`
- Surface Mir alpha source-authority roadmap:
  `specs/39-surface-mir-placement-elaboration.md`
  `specs/40-indexed-state-semantics.md`
  `specs/41-role-admission-and-capability-grant.md`
  `specs/42-source-patch-hotplug-semantics.md`
  `specs/43-surface-mir-v1-alpha-scope.md`
  `plan/64-surface-mir-placement-roadmap.md`
  `plan/65-indexed-state-roadmap.md`
  `plan/66-role-admission-roadmap.md`
  `plan/67-source-patch-hotplug-roadmap.md`
  `plan/68-surface-full-system-v1-roadmap.md`
  `docs/hands_on/surface_mir_alpha_01.md`
  `docs/hands_on/source_patch_hotplug_01.md`
  `docs/research_abstract/surface_mir_alpha_01.md`
- hands-on product alpha commands:
  `docs/hands_on/product_alpha1_01.md`
  `docs/hands_on/operational_product_sample_01.md`
  `docs/hands_on/operational_package_authoring_01.md`
  `docs/hands_on/operational_backend_inventory_01.md`
  `docs/hands_on/operational_portal_shard_starter_boundary_01.md`
- docs-first computational / PoseGraph guides:
  `docs/hands_on/minimal_alpha1_patterns_01.md`
  `docs/hands_on/mir_computational_core_01.md`
  `docs/hands_on/transform_posegraph_01.md`
  `docs/hands_on/autonomous_execution_01.md`
  `docs/research_abstract/minimal_alpha1_patterns_01.md`
  `docs/research_abstract/mir_computational_core_01.md`
  `docs/research_abstract/autonomous_execution_01.md`
- legacy hands-on closeout commands:
  `docs/hands_on/current_phase_closeout_01.md`

## snapshot の読み方

- `progress.md` と `samples_progress.md` は進捗率ではなく workflow status / evidence classification を primary metric にする
- helper / sidecar / report / expected JSON / first-floor runner は completion ではなく evidence として読む
- `100%` は外部開発者が実際に再現・使用できる operational workflow または product/public layer だけに使う
- `PA1W-*` は bounded practical workflow ready として読み、product/public-ready α-1 とは読まない

## agent / review operations

- `.docs/oracle-chatgpt-pro-operations.md` は、ChatGPT 5.5 Pro Extended
  Oracle browser consults の repo-local 運用メモである。
- Oracle consult は advisory review input であり、`mirrorea_canon/`、
  legacy `specs/` / `plan/`、`progress.md`、`tasks.md`、
  `docs/reports/` の source hierarchy を置き換えない。
- Oracle 系コマンドは分単位で待つ。遅い場合も重複起動せず、まず
  `oracle status` / `oracle session` で状態を確認する。
