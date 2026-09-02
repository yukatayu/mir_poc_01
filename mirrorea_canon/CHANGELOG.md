---
id: root/changelog
status: L1-fixed
maturity: draft
depends_on: [meta/style-guide]
summary: canon 自体の版履歴と改定手続きの要約。
open_items: []
---

# Changelog

## 改定手続き(要約)

1. 提案(人間または AI)を `meta/proposals/PROPOSAL-###.md` として起票。
2. 人間(プロジェクトオーナー)が採否を決定。
3. 採択なら該当ファイルを改定し、L0/L1 に触れる場合は ADR を追記・改訂し、本ファイルに 1 行記録。
4. `python3 meta/build-index.py` で `INDEX.json` を再生成し、参照整合を検証。

## 履歴

- **2026-09-02** PROPOSAL-041 / ADR-0038でI3-1をclosedした。source/Core由来
  の6 carrier family / 12 edge、strict private adapter/encoding、bounded
  localhost QUIC reliable bidi seamを受理し、I3-2をsole active goalとした。
  official I3 lifecycleは未entryであり、public wire/API/ABI、一般proof、
  retry/reconnect、productionは未主張である。

- **2026-09-02** PROPOSAL-040 / ADR-0037により、同一private
  source/Core-bound nine-case receiver-child canaryをactual TLS-over-TCPとQUIC
  reliable streamで実行したI3-0を受理した。両候補はcriteria 1--7でtieし、criterion 8
  implementation/library maturityとcriterion 9 cross-platform maintainabilityには
  auditable/tested winnerがなかった。最初のmaterial differenceであるcriterion 10
  future browser relevanceによりCandidate B QUIC reliable streamをprivate selected
  adapterとした。Candidate A TLS-over-TCP framed reliable streamはrejected/deferred
  comparison/replacement baseline、QUIC datagramはexcludedのままである。Aの584 LOCと
  より短いcanaryはlower-ranked performance/C12 simplicity evidenceでありselectionを
  上書きしない。OPEN-032はこのbounded programだけresolvedした。
  public wire/codec/version/certificate/API/ABI/deployment/production/platformをfreezeせず、
  official I3 lifecycleは未entry、theory T1とbroad PHASE-I1は不変である。ALIGN-0/1/2と
  I3-0をclosed、I3-1をsole active、I3-2をnext/inactiveとした。

- **2026-09-01** PROPOSAL-039 / ADR-0036 / architecture/07によりALIGN-2の
  Browser/Host trust boundaryを受理した。BND-007をRuntime/Projection→Viewとして
  authoritative domain semantics禁止とpresentation-local computation許可に明確化し、
  BND-010--016でpackage admission、Browser→fabric、View→renderer、typed input、typed
  effect/provider、privileged raw FFI、resource/sandboxを分離した。trust tier T0--T4は
  Theory T0--T2とは別であり、package admission、signature、process/session/provider/
  rendererはsemantic grant又はauthorityでない。T1 packageにraw FFIはなく、resource/
  observation enforcementはfail-closedである。cross-edgeではcontent/instance/epoch/
  freshness binding、role分離、use-time revalidation、queued/in-flight revocation、
  ambiguous effect、metadata redaction、pre-limit accounting、T3 TCB consequenceを要求する。
  具体package/origin/signature、sandbox、
  engine/FFI/API/ABI/wire/storage/public/productをfreezeせず、実装はinactive I5に残した。
  runtime/sample/theory/proof/OBL/official lifecycle/transport selectionは変更せず、
  ALIGN-0/1/2をclosed、I3-0をsole active、I3-1をnext/not activeとした。

- **2026-09-01** PROPOSAL-038 / ADR-0035 / architecture/06によりALIGN-1の
  three-axis architectureを受理した。current semantic strataをS0 Surface--S6 Host、
  project/product responsibilityをPL-0--PL-6、lifecycleをT0--T2 / I1--I6として
  独立したmany-to-many座標に固定し、旧LAB realization S0--S7とfeature maturityの
  S再利用をcurrent semantic axisから除いた。PL-4はShared-Space/World-Webの
  responsibility-only horizon、PL-6はReversed Libraryのseparate application/project、
  PrismCascade / Typed-Effect Wiring Platformはsatelliteのままである。runtime、sample、
  theory/proof/OBL、official lifecycle、OPEN-032、transport/public/product/productionは
  変更せず、ALIGN-1をclosed、ALIGN-2をsole active goal、I3-0をnext/not activeとした。

- **2026-09-01** PROPOSAL-037 / ADR-0034 により Mirrorea I3 Distributed
  Foundation bounded programを開始した。accepted I2 cutsをregression baselineに、
  LAB Plan 250をsole current roadmap、ALIGN-0をactive goalとする。fixed sequenceは
  ALIGN-0--2、I3-0--6、NEXT-0。semantic strata / project-product layers / lifecycle
  phasesを分離し、Browser/Host/package/View/provider境界を具体APIなしに固定した後、
  TLS-over-TCP framed reliable streamとQUIC reliable streamを同じ2-process executable
  canaryで比較する。paper-only selection、QUIC datagram、transport authority、hidden retry /
  exactly-once、public freezeは禁止。I3-3はplan/05 failure matrix全行、I3-4はminimum
  observer-safe gate evidenceを担い、I3-5は既存factだけをjoinする。program activationは
  official I3 entry/exitではない。theory T1、broad PHASE-I1、official I2 exit、public /
  production non-claimsを維持し、両候補とOPEN-032は未決のままである。ALIGN-0は
  accepted I2 regressionとmeta-drift reviewを経たcut `2f198105...`でclosedした。
  後続ALIGN-1 acceptanceはPROPOSAL-038 / ADR-0035に記録する。

- **2026-08-28** PROPOSAL-036 / ADR-0033 / plan/05によりSYS-7 inactive I3
  entry contractを受理した。future goalはaccepted I2 per-locus artifactsとgenerated
  communicationを2つ以上のOS processへ写し、authority、typed failure、source/Core
  provenance、redaction、Mir abstract orderingを保つfinite C-distributed profileを閉じる
  ことである。候補A TLS-over-TCP framed reliable-stream adapterと候補B QUIC reliable-
  stream adapterはともにUNSELECTEDで、QUIC datagramはadmit/evaluateしない。version、
  codec、wire、library、certificate format、port、deploymentは未選定。transport/session/
  certificateはauthorityでなく、internal carrierとfuture public wireを分離し、complete
  failure matrix、explicit retry/no exactly-once、network-order refinement、SCN-01/02/03/06
  future C-distributed gatesを要求する。SYS-7とADR-0026 programをcloseし、Plan 249を
  closed recordとした。現在active program/roadmap/goalはない。theory T1、broad I1
  residual、official I2 exitは不変で、I3 inactive、OPEN-032 unresolved。future I3 workは
  new owner directionを必要とする。runtime/test/Lean/model/OBL/SCNは変更しない。

- **2026-08-28** PROPOSAL-035 / ADR-0032 / spec/15 により、accepted cut
  `5429712de89a7e41c46cfd7fb4a39c4a492864c4`のSYS-6 source-first finite I2
  conformance profileを受理した。actual SYS-2--SYS-5 producer evidenceとfixed
  22-row verifierを分離し、content-bound I2 identity、executed positive/falsifier、
  property-specific provenance、typed rejection、observer-safe serializationを要求する。
  21 rowsは`runtime-monitored`、no-source-free-authority rowは既存OBL-058を参照する
  `model-checked-bounded`、aggregate OBL-063は`runtime-monitored`である。SYS-6
  25/25 + CLI 8/8、SYS-2 28/28、SYS-3 28/28、SYS-4 104/104、SYS-5 62/62、
  M10 67/67 + CLI 4/4、workspace、format/Clippy/diff、final independent ACCEPTを
  受理した。ADR-0025 entry contractとpre-existing I2 exit criteriaが満たされたため、
  official I2 entry後exitを適用した。theory T1、broad PHASE-I1、OPEN-026/027/full
  carrier freeze、public contract、real transport、productionは不変。SYS-6はcompleted、
  SYS-7がsole active goal、I3はinactive。OPEN-032はfuture owner-authorized I3
  decisionまで未決であり、I2 exit時transport選定というstale triggerを削除した。

- **2026-08-28** PROPOSAL-034 / ADR-0031 により、accepted cut
  `53a21e64b5a17e24b522f720db10b6e539c058e0`のSYS-5 four-locus local toy
  fabricとtyped devtoolsを受理した。provisional explicit relation-anchor locusを
  AST/M6/M7/Core/projection/source mapに保存し、source-bound ParticipantA leaveはexact
  M9 membership/capability/witness lineageをretireしてからParticipantB-owned relationを
  fallbackへ公開する。fresh reacquireはexact tombstoneからdistinct epoch/
  incarnationとfresh lineageを発行し、ownerがprimaryを再公開する。selected ST
  lifecycleはfailure-atomic candidateで、post-leave cut/restoreもexact retired lineageを保存する。
  `project-loci` / `run-local` / `inspect`はowner RMW、designated value、relation/fallback/
  presentation gap/fresh reacquire、auth failure、save/restore、accepted/rejected patch、optional
  verificationをactual generated endpoints上で実行し、source/Core/artifact/edge/runtime
  occurrenceをobserver-safeな単一のjoined reportに示す。AST 10/10、M7 27/27、SYS-3
  28/28、workflow 8/8、relation 17/17、cut/patch 12/12、CLI 3/3、M9 lifecycle
  4/4、full `mir-runtime` all-targets、M10 2/4/67、format/Clippy/diff/manual redaction
  check、独立semantics/usability/security/authority reviewを受理した。OBL-062はこの
  finite evidenceのみ`runtime-monitored`。theory T1、broad PHASE-I1、official I2
  entry/exit、public CLI/API/ABI/wire/JSONは不変。SYS-5はcompleted、SYS-6がsole
  active goal、SYS-7がnextである。

- **2026-08-27** PROPOSAL-033 / ADR-0030 により、accepted cut
  `22196f93b0112b8fd2987ec078021c8865b71651`のcrate-private SYS-4 in-process
  fabricを受理した。SYS-3のowned per-locus artifacts、generated communication plan、
  complete sealed M9 admissionだけからlocus runtime/endpointsを起動し、source再parse、
  fixture-plan lookup、manual route、authority/result injectionなしにST multi-owner dispatchと
  eligible OW1 selected correspondenceを実行する。bounded designated-result endpointはfirst
  deliveryでexactly one M8 semantic consumeを行い、exact same-consumer retryをstored typed
  decision/no-new-consumeとして返す。typed faults/quarantine、observer-safe redaction、OW1
  snapshot failure/absence分離、ST whole-fabric local cut/restore、quiescent designated-only
  checked patchを有限受理した。cut/restoreとpatchのOW1 pathは`BackendIneligible`、public
  CLI/API/ABI/wire/JSON、real transport、durable persistence、general theoremは非主張である。
  99 focused SYS-4 tests、179 runtime library tests、preserved M10 regression、format、
  warnings-denied Clippy、diff validation、independent reviewを受理し、OBL-061をこのfinite
  runtime correspondenceだけの`runtime-monitored`として追加した。theory T1、broad
  PHASE-I1、official I2 entry/exitは不変。SYS-4はcompleted、SYS-5がsole active goal、
  SYS-6がnextである。

- **2026-08-27** PROPOSAL-032 / ADR-0029 により、exact checked-program identityへ
  boundされたlogical locus inventoryと`CheckedSurfaceV0`から、placement-specific typed
  Coreをownするper-locus fragments、generated communication/effect/observation/
  persistence plans、source/Core/artifact/edge correspondenceを純粋かつ決定的に作る
  SYS-3 internal projectorを採用した。same-owner RMWはowner artifact、relationはowner
  publish/consumer-local projection、designated remote inputはsource-owner serviceと
  evaluator consumeへ分離し、各edgeはreal source/target fragment refとauthority-neutral
  typed carrier contractを保持する。初回close reviewはcandidate cut
  `ded622fef91bab2cadc571ba944e5ee2c69a7b63`に、ordinary source/M6/M7 Coreから
  evaluator→named consumerを導くE-CONSUME pathがない反例を発見したため、同cutは
  partial regression evidenceへ戻してSYS-3を一度reopenした。bounded non-final internal
  clause `designated consume E.result at C`をdistinct AST/M6/M7 Core edgeとして追加し、
  exactly-one consumer、same-consumer retryはexisting
  decided result/no-new-consume、competing consumer typed conflictを要求する。consumerは
  topology/schedule/relationから推論しない。このretryはtheory/13由来のnew static SYS-4
  endpoint refinement requirementであり、legacy M8 same-delivery `AlreadyConsumed`と
  accepted M10 duplicate-delivery rejectionは変更・再解釈しない。SYS-3はsemantic identity/
  contractだけをencodeし、actual carrier-side idempotent return/wrapperとendpoint testsは
  SYS-4が担う。production relation graphはcurrent checked two-anchor
  shapeのみで、source-bound deeper/shared DAGはtest-only extension pressure nonclaimに
  留める。accepted source/evidence cut
  `3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9`はsource-test commits
  `b39f3e76`、`f37be73c`、`27e42658`、`30be30bb`を経て、M6 metadata P1、
  missing-producer P2、silent signature shadow P1を修正した。AST Surface M6 9/9、
  M6 classification 13/13、M7 pipeline 25/25、M9 8/8、SYS-3 27/27、M8 admission
  7/7、M10 source 2/2、M10 conformance 67/67、full `mir-runtime` / workspace、format、
  scoped warnings-denied Clippy、diff checkと最終semantic/code-quality ACCEPTを受理した。
  OBL-060はこのstatic finite compiler/projector evidenceだけを`runtime-monitored`とする。
  Lean/general theorem、runtime admission/dispatch、actual occurrence、public
  API/ABI/wire、multi-consumer、broad PHASE-I1/I2 lifecycleは主張しない。
  SYS-3はcompleted、SYS-4がsole active goal、SYS-5がnextである。

- **2026-08-27** PROPOSAL-031 / ADR-0028 により、SYS-1 kernel fragmentの
  deterministic STと、exactly one combined semantic owner/source-owner locusを
  dedicated workerが排他的に所有するOW1 backendを採用した。successful owner mutationの
  linearization/reads-from/coherenceはactual M8 enqueue/`OwnerRead`/`OwnerWrite` traceへ
  結び、designated remote inputはacknowledged source-owner readからreplyを導出する。
  same-seam M9 publisherはactual revoke後にcomplete inventoryを再translationし、strict
  successor・monotone tombstone・unrelated owner/designated lineage retentionを検査して、
  ST install又はOW1 worker ack後にのみgenerationを公開する。accepted source cutは
  `920d3fe050b8b909253f8511d9ad897272323ced`。OBL-058を
  `model-checked-bounded`、OBL-059を`runtime-monitored`として追加したが、Lean/general
  theorem、multi-owner OW、Surface `memory_order_*`、public API/ABI/wire、broad
  PHASE-I1/I2 lifecycleは主張しない。SYS-2はcompleted、SYS-3がactive、SYS-4がnextである。

- **2026-08-26** PROPOSAL-030 / ADR-0027 により、ordinary `run_source` とgeneric
  checked `OwnerEvent`のproduction pathをcrate-private `SemanticRuntimeKernel`へ分離し、
  sealed M9 seamからadmitted M8 runtimeを所有・抽出するdependency directionを採用した。
  owner requestのrequest→serve→reply→receive/receiptと、designated remote-inputの
  request→source-owner serve→reply→receive/receipt→evaluator consumeを、exact checked
  source/Core provenance、authority lineage、epoch/incarnation、capability/witness、
  effect/failure、visibility/redaction、frontier/consumptionを保持するI2-internal contractに
  固定した。accepted source cutは
  `94e3707c7bc98d4a0764c51f13a12b1dae1968c6`、evidenceはruntime-monitoredである。
  OPEN-030はこのnarrow internal contractに限りresolved。architecture/04はL2-working、
  OPEN-026/027とfull carrier freeze、revoke-after-enqueue/serve visibilityは残り、SYS-2が
  active、SYS-3がnextである。specialized M10 SCN-04/09/10/route-patchはregression-only、
  public API/ABI/wire、broad PHASE-I1/I2 lifecycle、proof ledger、theory T1は不変である。

- **2026-08-26** PROPOSAL-029 の owner disposition と ADR-0026 により、accepted
  M10 semanticsからmeaning-preserving per-locus executable artifacts、generated
  communication、process-internal dispatch、typed devtoolsへ進む Mirrorea I2
  Systems Foundation SYS-0--SYS-7 bounded programを開始した。sole current LAB
  roadmapはPlan 249。SYS-0はaccepted integration cutでcompleted、active goalは
  SYS-1、next goalはSYS-2である。goal statement、
  direct-consumer/candidate/stop/report/review/evidence規律とowner-reserved stop lineを
  採用した。ADR-0015/Plan 247/M10 cutはclosed immutable baselineとして保持する。
  program activationはofficial theory T1を動かさず、broad PHASE-I1 exit、I2 lifecycle
  entry/exit、proof ledger、public grammar/API/ABI/wire、real transport、durable
  distributed persistence、production、browser/View renderer、I3 implementationを
  受理・固定・開始しない。

- **2026-08-05** ADR-0015 の owner delegation と ADR-0025 acceptance amendment により、
  source/validation cut `23f5a8130334bf0c8516d51e9dcea38b92f50db1` の M10 I1+
  deterministic reference profileを受理した。fresh remote cloneで同一output digestを
  二回再現し、26 static + 47 runtime correspondence rows、zero mismatch/missing、
  release-anchor match、waiverなし、独立review ACCEPTを確認した。theory lifecycleは
  T1、proof ledgerは不変である。architecture/04 L2 carrier freeze / OPEN-030、広い
  PHASE-I1 exit、public API/ABI/wire、C-distributed、I2 activation、deploymentは
  非主張として残し、I2には新しいowner directionを要求するentry contractだけを置いた。

- **2026-08-05** PROPOSAL-028 と ADR-0025 により、凍結SCN-01..10の primary ordinary
  `.mir` source（SCN間共有可）と有限の named negative source variants を direct path の
  typed terminal まで通す有限M10 conformance profileを採用した。SCN-09には有限の
  named candidate patch sources を追加する。各 source unit hash、finite carrier
  correspondence、exogenous schedule action、expectation predicateはprofileに束縛され、
  generatorとverifierの分離、program/scheduleのdual provenance、SCN-08のnon-Surface
  three-option typed fallback carrierを要求する。setup prose、expected JSON/report/
  fixture-name、waiverはsemantic shortcutにできない。field declarations後に一つだけ置ける
  `visible observer_safe fields (...)` をM10 direct consumerの唯一のM6/M7 seamとして
  採用し、private-by-default、source-bound observer publish、`VisibilityDenied` failureを
  定めた。M10はgeneral proof/OBL、C-distributed、public grammar/API/ABI/wire、deployment、
  product/I2+、SCN-11/12の10/10昇格を主張しない。
- **2026-08-05** accepted Rust correction と independent finite-only review により、
  OBL-028 を `model-checked-bounded` に更新した。evidence は
  `crates/mir-semantics/src/m9_model_check_auth.rs` と
  `crates/mir-semantics/tests/m9_model_check_auth.rs` の one-subject /
  one-capability、bound 4、`admit` / `grant` / `revoke` / `use` /
  `reacquire` の reachable-state graph に限る。input-sensitive
  revocation/use/rejected-use/reacquire、monotone revocation、
  rejected-use-no-M8-mutation、concrete fault counterexample を記録するが、
  general proof、action-sequence enumeration、authorization composition は主張しない。
- **2026-08-04** PROPOSAL-027 と ADR-0024 により、M8の`AuthDeferred` /
  `VerifyDeferred`を保持したM7/M8 identity/source-mapから外部M9 resolutionへ渡す有限 seamを採用した。MembershipAuth / CapabilityAuthはnon-transparent ContractUpdateのpolicy lane、`finite_refinement`はEvidence / Diagnostic / Residualだけを返す verifier laneであり、evidenceはgrantではない。OBL-026はexact SourceRef/identity/deferred-row resolution、M8-only deferral、verifier non-authority、二つのtransparent overlayの`ContractRef` equalityだけを記録する。OBL-028はinput-sensitive model/Rust behaviorを主張せず、accepted Rust correctionとaccepted bounded evidenceまでintentionally-deferredである。M10/SCN、general theorem、transport、public ABI/wireは主張しない。
- **2026-08-04** PROPOSAL-026 と ADR-0023 により、M7 immutable checked artifact
  だけをsource-program inputとする有限 M8 runtime admission を採用した。checked
  identity は static environment、evaluation/Core、effect/obligation、stable source
  map の shape を覆い、residual evidence はidentity/source-ref boundである。relation
  / designatedのbase residualだけを有限に検査でき、`AuthDeferred` / `VerifyDeferred`
  はM9まで`DeferredToM9`に留まる。M8のfresh Lean carrierはM5/M7/Rust typeの合成を
  主張せず、M5-aligned categoriesを持つ一つのfresh runtime semantic state、deterministic queue/replay、local
  cut、bounded rejected/deferred patch、observer-safe projectionのexact evidenceだけを
  記録する。M10のfresh official SCN conformance、M9、transport、general theorem、public
  ABI/wireは採用・主張しない。
- **2026-08-04** PROPOSAL-025 と ADR-0022 により、M6 fixed source input を
  finite checked elaboration に精密化した。M7 は M6 diagnostic/span を保持し、
  typed Core/evaluation axes/effect row/generated obligation/total stable
  source-to-Core map を返す。owner RMW、maintained relation、designated value は
  distinct のままであり、relation lifetime/visibility/fallback validity、designated
  value visibility/redaction、auth/verify は typed
  residual に残る。residual は static artifact を成功にしても authority/capability/
  effect/mutation/verdict を生成せず、residual row が空でない場合は execution
  admission は `ResidualCannotExecute` に fail-closed する。admission は nonempty
  checked evaluation と empty residual row を併せて必要とする。OBL-049 は exact finite
  Lean evidence のみを記録し、runtime/M9 semantics、conformance、public API/wire、
  final grammar は採用・主張しない。
- **2026-08-04** M7 final review correction: M6 の broad ordered expression-token
  collector は canonical `M6ExprToken` 全集合（`{ } [ ] ( ) : , . = + -` を含む）を
  span 付きで保持し、M7 はその後で finite ordered typed expression tree と `Int`
  arithmetic を検査する。`CheckedSurfaceV0` は accepted M6
  `SurfaceV0Classification` 全体を move 保持して公開し、root/source `SourceRef`、
  template、source-to-Core map を summary/rebuild に置き換えない。effect /
  generated-obligation row は source span / `SourceRef` 付きで enumerable にし、designated value は M3
  `InputFrontier`、deterministic evaluation policy、conservative observation policy、
  policy stamp を保持する。`Authority` / `AdmittedEvaluatorAuthority` は authority
  success ではなく obligation として残る。
- **2026-08-04** PROPOSAL-024 と ADR-0021 により、M6 bounded ordinary
  Surface を M5 shared model の source-facing reference として採用した。
  same-owner owner action は authority origin と owner evaluation を分け、
  request/write edge と local-RHS-dependency の別 source-to-Core entry、
  capability/witness obligation をもつ `ownerRmw` template に分類するが、receipt
  fact は生成しない。cross-owner dependency は明示 receipt-required diagnostic、
  maintained relation と designated result は nominally separate
  frontier / materialization template、auth/finite-refinement は successful
  non-executable typed deferred template とする。これは parser/checker、
  runtime/transport、M9 semantics、final public grammar/API/ABI を採用・主張しない。
- **2026-08-04** PROPOSAL-023 と ADR-0020 により、M1--M4 の有限
  `SurfaceFragment`/`Core`/`Config`/`Step`/`WellFormed`/trace/diagnostic/
  relation/cut-save carrier を一つの concrete M5 shared model に採用した。
  result frontier/version、relation frontier/epoch、presentation context、cut は
  distinct type のままとし、owner bind → `publish-relation` による exact
  relation/binding carrier の保持 → consumer-local projection と consumer
  materialization/J-mutation reject を有限 evidence にした。
  これは M6 grammar、general theorem、save/load algorithm、patch/runtime/transport、
  public contract を採用・主張しない。
- **2026-08-04** PROPOSAL-022 と ADR-0019 により、M4 の owner-held
  maintained relation と consumer-local late projection を採用した。relation は
  `publish-relation` で投影され、absolute value / adapter stream には具体化しない。
  semantic fallback は同一 lineage 内で単調、fresh reacquire は新 lineage、presentation
  gap は semantic state 非変異である。有限 Lean evidence は exact M4 rows のみを対象とし、
  general fallback / two-layer-time proof、grammar、wire/API、save/load 実装、runtime
  conformance は主張しない。
- **2026-08-04** PROPOSAL-021 と ADR-0018 により、M3 の有限 `EvalPlan` を採用した。
  owner RMW は caller authority と owner evaluation を分離して serial service し、
  other-owner operand は explicit receipt または Diagnostic とする。designated evaluator
  は frontier/versioned value を決定する。これは M6 grammar、wire/API、transaction、
  save/load/patch、I1/conformance/deployment を固定・主張しない。
- **2026-08-04** ADR-0017 / Plan 04 の M2 acceptance record に、revision-bound
  semantic-assertion v3 の fresh `pass` artifact (`LAB:plan/248`、digest
  `b32bd2c87e1dc77ca2a4f7a7426cda0bff8bcbf80155d19addd7db3a8288aa23`) を受理した。
  順に G0-D3、G0 exit、T1 entry を適用した。これは phase-governance のみであり、SCN、
  proof/OBL、runtime、I1、public contract/deployment は動かしていない。
- **2026-08-04** PROPOSAL-019 と ADR-0016 により、`root/design-constitution`
  を Mir Theory v0 / deterministic I1+ の横断判断として採用した。SCN-02 は
  requester authority origin と S-side owner RMW を区別するよう是正し、semantic
  fallback と consumer-local presentation fallback を分離した。pre-M6 Surface
  profile は historical compatibility candidate として保持し、final grammar を
  先取りしない。T0/G0/T1、OBL/proof、conformance、runtime、public contract は
  動かしていない。
- **2026-08-03** PROPOSAL-018 の owner disposition と ADR-0015 により、Mir
  Theory v0 / I1+ Milestones 0--10 に限る evidence-gated bounded autonomy、
  一つの semantic frontier、原則一 milestone 一 report、独立 review、実証済み
  proof-ledger 更新を採用した。ADR-0014 はこの program 外の default research
  route として残り、North Star・保証の弱化・final public contract・production
  deployment は owner-reserved のままである。
- **2026-07-29** PROPOSAL-017 の owner disposition として、V1/R1 の
  cross-locus read に限る `X1 relation-state envelope` を記録した。これは
  ADR-0014 適格な最小 L3 設計・反例 package を開くのみであり、Core、Config、
  SaveObject、failure、theory/11、SCN、Gate、Phase、runtime、source grammar、
  wire/API、public contract は変更しない。
- **2026-07-28** PROPOSAL-004/008/012/013/015/016 に owner disposition を
  記録した。Participant-only Surface v0、outcome totality の別 obligation、
  V1/R1/SW1/conditional A2、M1 request-local validation context、explicit
  scalar terminal fallback / `return` exclusion、narrow T2 と separate I1
  readiness/bootstrap が後続設計 package の方向となった。Core rule、grammar、
  ledger、SCN、Gate、Phase、runtime、public contract はこの記録だけでは変更しない。
- **2026-07-28** PROPOSAL-014 と ADR-0013 amendment により
  `phase-governance/t0-g0` version 2 を採用した。v1 artifact は
  nonconforming historical evidence として byte-preserved し、既存の fixed
  evidence/control predicates を rebase せず one-off fresh v2 artifact だけを
  許可する。G0-D3、G0 exit、T1 entry、I1 authorization は引き続き defer /
  non-effect である。
- **2026-07-24** Added PROPOSAL-013, an owner decision request for the semantic
  provenance of post-admission request validation context. It selects no Core,
  queue, wire, runtime, OBL, Gate, Phase, or public behavior.
- **2026-07-24** `PROPOSAL-009` の owner disposition を記録。将来の
  OBL-001 proof-facing package は THM-001 の既存 every-write Core `c` 条件を
  直接表明できる。この記録は再triage を許すだけで、新規 WRK、Core
  representation / traversal、OBL status、proof、Gate / Phase を選ばない。
- **2026-07-21** PROPOSAL-007 により、ADR-0014 の既存 L3 retained-evidence
  condition を WRK の append-only `Evidence commits:` と reachable-DAG audit
  で精密化した。これは L2 activation、reserved boundary、L0/L1 theory を変更せず、
  既存 LAB lane の証拠帰属を機械検査可能にする運用改定である。
- **2026-07-21** ADR-0014 を PROPOSAL-006 により改訂。owner-maintained exact
  editable-target table を standing bounded autonomy に置換し、agent-maintained
  canon surface を `working/` に限定した。L3 pre-registration、existing-lane
  evidence、L2 の author/reviewer distinct signed frozen-material review、forward rollback を要する。L0/L1、contracts、
  SCN/Gate/Phase、`theory/11`、final proof、public claim は引き続き留保する。
- **2026-07-21 (superseded operating detail)** ADR-0014 により、existing LAB lane における可逆な L2/L3
  working-theory research を委任した。canon update は owner-maintained exact
  editable-target row、rebased frozen evidence/diff、independent review、reviewed
  rollback を要する。L0/L1、external contract、SCN/Gate/Phase、全 `theory/11`、
  implementation/public status は委任していない。初期 editable-target row は空。
- **2026-07-15** ADR-0013 により `phase-governance/t0-g0` version 1 と、
  pinned evidence cut に対する G0-D1 acceptance / G0-D4 waiver を採択した。
  one-off LAB-derived JSON は許可したが、G0-D3 は defer のため G0 exit / T1
  entry / SCN conformance / implementation state は変更していない。
- **2026-07-14** `PROPOSAL-001` の owner disposition を記録。abstract
  OBL-020 Lean statement shape は、full OBL-020 completion を伴わない
  G1-supporting proposal-preparation scope としてのみ受理された。OBL status、
  proof、artifact identity / wrapper、Gate / Phase は変更していない。
- **v0.1.0** (2026-07-02) 初回生成。ADR-0001〜0012 制定。MirCore v0 初稿、Surface 文法 v0、SCN-01〜10 凍結、Gate/Phase 計画制定。全証明は OBL 台帳(未 discharge)。既存 repo mir_poc_01 を LAB に格下げ(手続きは meta/source-hierarchy.md)。
