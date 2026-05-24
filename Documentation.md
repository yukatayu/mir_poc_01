# ドキュメント要約

## この文書の役割

この文書は、repo 全体の **current snapshot を短く正確に読む入口** です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/`
- workflow / evidence snapshot は `progress.md`
- current task map は `tasks.md`
- runnable sample dashboard は `samples_progress.md`
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
  `specs/19..24` / `plan/45..49` で定義した α-0.5 / α-0.8 / α-0.9 の same-session operational 条件
- **product/public-ready Mirrorea Spaces alpha-1**
  `specs/25` / `plan/50` で定義した alpha-stable CLI、versioned package format、same-session product demo、quiescent save、viewer、native launch bundle、clean-clone validation の product alpha line。final textual `.mir` grammar、WAN/federation、distributed durable save/load、arbitrary native execution、final viewer / telemetry service は non-goal
- **operational product sample suite**
  `specs/26` / `plan/51` で定義した `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation` suite、shared attach packages、deployment/projection inventory、retained portal/shard blueprint inventory。current line では `MembershipChat` の bounded room-oriented `ChatText` host boundary、`SugorokuWorld` の bounded roll / publish / witness / handoff / stale membership reject scenario、`projection.profile.json` の schema-backed target / packet / FFI inventory、`PortalWorldLink` の bounded same-session discrete handoff cut、`TwoShardHardBoundary` の bounded same-session two-shard offer / prepare / commit cutとobserver-visible old-owner / stale-config reject-event narration付き `missing_live_witness` rejection、そして `TwoShardGradientObservation` の bounded observer-only gradient view / handoff hint rowsとobserver-visible write-reject / stale-view-drop narration付き freshness rejection が `check`、runtime plan、observer-safe devtools / helper closeout に actualize 済み。`future/portal-worldlink/` と `future/two-shard-hard-boundary/` blueprint は残す
- **Mir Computational Core rebaseline**
  `specs/28..31` / `plan/53..56` で定義し直した docs/spec line。Product Alpha-1 の operational floor は保持するが、current typed external `AddOne` を Mir-owned computation の証拠とは読まない。front-half scaffold は維持しつつ、`P-COMP-02` により `samples/product-alpha1/computational/add-one-pure-mir/` と `scripts/mir_computational_samples.py` が one direct executable Mir-owned row を持つようになり、`P-COMP-03` により variables / arrays / records / control-flow / imports の positive / negative rows が helper-executable に actualize され、`P-COMP-04` により direct product-alpha accepted/check-rejection rows で host read/write boundary が explicit になった。ここで `required_capabilities` / `failure_tag` は checker-admission boundary declaration の evidence であり、broad effectful runtime semantics completion を意味しない。PoseGraph は `P-POSE-02` により same-client same-observation-snapshot no-split-frame の accepted/violation helper evidence を持つようになり、projection / engine-adapter は引き続き planned-only scaffold / inventory である。all-up closeout audit まで実行済みで、current self-driven chain は閉じた。
- **minimal alpha-1 pattern verification**
  `scripts/minimal_alpha1_patterns.py` と `docs/hands_on/minimal_alpha1_patterns_01.md` は、closed chain の上に置く reader-facing verifier。computational / PoseGraph / projection / engine-adapter の exact row count、expected rejection、inventory-only boundary を確認し、product release-candidate と operational Sugoroku は workflow anchor として同じ matrix で読む。これは新しい runtime claim ではなく、最小実用パターンの drift 検出である。
- **autonomous execution contract**
  `specs/32` / `plan/57` で定義した package-by-package 自律実行 line。`P-COMP-01 -> P-POSE-01 -> P-PROJ-01 -> P-ENG-01` の front-half を閉じてから implementation half に入り、user に途中質問せず、final distribution / catalog / ABI / backend admission のような user-spec-required gate は隔離し、lower-layer implementation を止めない。
- **Full System V1 source-first roadmap**
  `specs/33..38` / `plan/58..63` で定義した次段 roadmap。Product Alpha-1 と operational suite を alpha floor として保持しつつ、semantic source of truth を Mir source files へ戻し、textual Mir alpha grammar、typed IR、pure interpreter、effectful runtime integration、PoseGraph runtime、projection IR、provider admission、devtools / release check へ段階的に進む。`package.mir.json` は alpha compatibility / package artifact であり、final source authority ではない。
- **Surface Mir alpha source-authority line**
  `specs/39..43` / `plan/64..68` で定義した new promoted roadmap。Canonical place-scope syntax は `S { ... }` であり、`S[ ... ]` は sugar としても採用しない。Surface Mir は user-facing source、Core Mir は elaboration target であり、通信・publish・observe は自動生成して Core IR / devtools に明示する。`package.mir.json` は alpha artifact に留まり、semantic source authority は `.mir` files に戻す。`P-SURF-01` は parser/helper/sample floor、`P-SURF-02` は `crates/mir-semantics::surface_indexed_state` と `IDX-01..05` による indexed-state semantic checker/sample floor、`P-SURF-03` は `crates/mir-semantics::surface_to_core_elaboration` と `ELAB-01/02/04/05/06/07/08` による Surface-to-Core elaboration evidence floor、`P-SURF-04` は generated MessageEnvelope / publish / observe / private-field rejection evidence floor である。Runtime dispatch、role admission、source patch hot-plug completion はまだ主張しない。
- **final public product**
  final parser grammar、public checker/runtime/verifier API、public adapter / viewer / projection / hot-plug / transport surface、packaging、external contract まで含む最終形

現在 repo は、repo-local alpha-ready current layer、current-scope evidence、practical alpha-1 first-floor evidenceに加えて、次の runnable workflow を持っています。

- bounded operational α-0.5 / α-0.8 / α-0.9
- bounded practical α-1 integrated workflow carrier
- product alpha-1 release-candidate workflow
- installed-binary + generated native host launch bundle adoption probe
- `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation` の canonical operational product sample suite
- minimal alpha-1 pattern verifier for strict row / rejection / inventory drift checks
- Surface Mir brace parser floor plus indexed-state semantic checker, Surface-to-Core elaboration, and generated communication evidence floors, with `P-SURF-05 role admission capability grant` as the next promoted package

実用面では、外部開発者が documented commands で product demo と operational suite を `check`、`run-local`、`session`、`attach`、`save`、`quiescent-save`、`transport`、`export-devtools`、`view`、`build-native-bundle` まで再現できる段階です。current delivery unit は developer-built `mirrorea-alpha` binary + locally generated native host launch bundle だけで、current catalog scope は bounded product alpha-1 narrow showcase です。

まだ主張しないものは明確です。final public product、final textual grammar / ABI / SDK、Surface Mir runtime/helper implementation beyond the P-SURF-04 generated communication elaboration evidence floor、archive / installer / hosted service、final viewer / telemetry ABI、R3/R4 durable distributed save/load、WAN/federation、arbitrary native package execution、arbitrary WASM execution、final server/client binary split、continuous spatial sync、direct LLVM backend は別 gate です。broader distribution / final shared-space catalog breadth は user-spec-required decision です。`P-FS-00` は implementation 前の roadmap rebaseline、`P-MIR-01` は parser-floor actualization、`P-MIR-02` は typed checker floor、`P-MIR-03` は pure interpreter floor、`P-MIR-04` は `crates/mir-semantics::full_system_v1`、`crates/mir-runtime::full_system_v1_session`、`scripts/full_system_v1_samples.py`、`samples/full-system-v1/computational/runtime-matrix.json` による bounded effectful runtime floor actualization、`P-POSE-03` は `crates/mir-runtime::posegraph_runtime`、`samples/full-system-v1/avatar-pose/`、`scripts/posegraph_runtime_samples.py`、`cargo test -p mir-runtime --test posegraph_runtime -- --nocapture` による bounded PoseGraph runtime floor actualization、`P-POSE-04` はその同じ lane に pose-aware save/load admissibility と observer-safe PoseGraph/devtools export を actualize した package、`P-PROJ-02` は `crates/mir-semantics::full_system_v1::projection`、`crates/mir-runtime::full_system_v1_projection`、`samples/full-system-v1/projection/`、`scripts/projection_v1_samples.py`、`cargo test -p mir-runtime --test projection_ir -- --nocapture`、`cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture` により source-derived target manifests、source-owned capability/failure preservation、client-write authority rejection、unassigned-place rejection、save/load ownership rejectionを actualize した package、`P-PROJ-03` はその same root に packet schema、FFI schema、payload-shape mismatch rejection、same-shape heterogeneous effect-contract rejection、projection-artifacts/rejection-report outputsを widen した package、`P-PROJ-04` は `crates/mir-runtime::full_system_v1_local_split`、`samples/full-system-v1/server-client/`、`mirrorea-alpha run-full-v1-split` により same-binary local server/client role-run 1 accepted row と undeclared entry override 1 rejection rowを actualize した package、`P-ENG-02` は `crates/mir-runtime::full_system_v1_provider_admission`、`samples/full-system-v1/provider-adapter/`、`scripts/provider_admission_samples.py`、`cargo test -p mir-runtime --test provider_admission -- --nocapture`、`mirrorea-alpha admit-provider-v1` により bounded provider-admission lane の viewer-diagnostic inventory accepted row、WASM inventory-only accepted row、over-capability rejection、missing rollback policy rejection、native-disabled rejectionを actualize した package、`P-ENG-03` は `crates/mir-runtime::full_system_v1_renderer_pose_backend`、`samples/full-system-v1/provider-adapter/renderer-pose-*/`、`scripts/renderer_pose_backend_samples.py`、`cargo test -p mir-runtime --test renderer_pose_backend -- --nocapture`、`mirrorea-alpha render-pose-backend-v1` により renderer が matching binding_context と snapshot frontier を満たす bounded PoseGraph delivery evidence を受け取りつつ semantic owner にならない lane を actualize した package、`P-FSV1-01` と `P-FSV1-02` は `samples/full-system-v1/world-core/`、`membership-chat/`、`sugoroku-world/`、`portal-worldlink/`、`two-shard-hard-boundary/`、`gradient-observation/`、`scripts/full_system_v1_samples.py operational-matrix/check-operational-all`、`cargo test -p mir-runtime --test full_system_v1_session -- --nocapture` により bounded source-first operational suite floor を 12 executable rows まで actualize した packageです。`P-FSV1-03` は `scripts/full_system_v1_release_check.py`、per-command JSON reports、static `bundle.json` / `index.html` viewer、Product Alpha compatibility anchors、そして representative `mirrorea-alpha project-full-v1` / `run-full-v1-split` / `admit-provider-v1` / `render-pose-backend-v1` surfaces を束ねる bounded release-check workflow を actualize し、`P-FSV1-99` は full validation、docs/report cleanup、claim/non-claim audit を通してその line を close しました。ここでは source-derived safe C-like executionに加えて、host read/write、publish/observe、witness/handoff、local atomic-cut negative rows、same-client same-observation-snapshot no-split-frame runtime rows、anchor-switch frontier rejection、fallback-only reacquire requirement、bounded save/load admissibility、observer-safe panel export、projection IR、target manifests、boundary schemas、preservation report、bounded local role split、bounded provider admission、renderer pose backend demo、bounded WorldCore/MembershipChat/Sugoroku/Portal/TwoShard/Gradient source operational rows、bounded release-check/report/viewer bundle まで actualize しつつ、attested PoseGraph package provenance / final effect grammar / final packet or FFI transport semantics / final server-client binary split / broader provider execution / distributed save-load / final PoseGraph devtools family / final product workflow closure は主張しません。Full System V1 autonomous chain は `P-FSV1-99 final audit` まで close 済みです。

## current active floor

active canonical sample は `samples/clean-near-end/` です。base current-L2 corpus は `samples/current-l2/`、active Lean mechanization evidence は `samples/lean/` に置きます。

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
  `specs/13..17`、`plan/39..43`、`samples/alpha/` が current-scope evidence を担います。Stage A..F は evidence reference であり、operational α-0.5 / α-0.8 / α-0.9 completion ではありません。
- **practical alpha-1 first-floor / workflow line**
  `specs/18`、`plan/44`、`samples/practical-alpha1/` が first-floor toolchain を担います。`scripts/practical_alpha1_integrated_workflow.py check-all --format json` は bounded developer workflow を再現しますが、product/public-ready α-1 ではありません。
- **operational alpha theory-freeze line**
  `specs/19..24`、`plan/45..49` が α-0.5 local observable runtime、α-0.8 same-session hot-plug runtime、α-0.9 session-bound devtools readiness を固定します。対応 helper は bounded same-session workflows を再現します。
- **product alpha-1 line**
  `specs/25`、`plan/50`、`samples/product-alpha1/demo/`、`scripts/product_alpha1_release_check.py` が product alpha release-candidate workflow を担います。Docker 込みの release check が accepted のとき、controlled alpha product workflow として実用確認済みと読めます。
- **operational product sample suite**
  `specs/26..27`、`plan/51..52`、`samples/product-alpha1/operational/`、`scripts/operational_product_samples.py` が canonical operational suite を担います。six roots、shared attach packages、projection inventory、portal/shard/gradient cuts、template-only starter catalog、backend inventory、scope helper blocks を保持します。
- **Full System V1 source-first line**
  `specs/33..38`、`plan/58..63`、`progress.md`、`tasks.md` が current roadmap snapshot を担います。latest closed package は `P-FSV1-99 final audit` で、current promoted package はありません。ここでは Product Alpha-1 を final product に昇格せず、textual Mir、typed IR、bounded effectful runtime、PoseGraph runtime、bounded pose save/devtools、projection IR + boundary schemas + bounded local role split、bounded provider admission、renderer pose backend、source-first operational suites、portal/shard/gradient source suites、bounded release check、最後に claim/non-claim audit まで自走で閉じました。
- **Surface Mir alpha line**
  `specs/39..43`、`plan/64..68`、`progress.md`、`tasks.md` が current promoted Surface roadmap snapshot を担います。`P-SURF-01` has actualized the parser floor in `crates/mir-ast::surface_alpha`, `samples/full-system-v1-surface/syntax/`, and `scripts/surface_mir_samples.py`; `P-SURF-02` has actualized the indexed-state semantic checker floor in `crates/mir-semantics::surface_indexed_state` and `samples/full-system-v1-surface/indexed-state/`; `P-SURF-03` has actualized the Surface-to-Core elaboration evidence floor in `crates/mir-semantics::surface_to_core_elaboration` and `samples/full-system-v1-surface/elaboration/`; `P-SURF-04` has actualized generated MessageEnvelope / publish / observe / visibility-failure evidence in the same lane. Next promoted package is `P-SURF-05 role admission capability grant`. Canonical place-scope syntax is `S { ... }`; `S[ ... ]` is rejected and is not sugar.

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
- Surface Mir runtime / helper implementation beyond the P-SURF-04 generated communication elaboration evidence floor

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
