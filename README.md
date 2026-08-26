# Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins; cite legacy `specs/` /
`plan/` as `LAB:` evidence unless mirrored into canon.

この repository は、4 系統を分離可能なまま扱う **specification-first research repo** です。

前提知識なしで目的・理論・システム構成・公式工程・LAB 実証・現在の判断点を
一度に確認する入口は `docs/mirrorea-project-overview.html` です。

## Current research governance

`mirrorea_canon/` remains the sole normative source. ADR-0015 and Plan 247 are
the closed authority/execution record for the accepted Mir Theory v0 / I1+
M0--M10 finite reference baseline. The owner has now adopted ADR-0026 for the
bounded Mirrorea I2 Systems Foundation SYS-0--SYS-7 program. Its sole current
LAB roadmap is `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.
SYS-0 and SYS-1 are completed and closed. ADR-0027 records the crate-private
semantic runtime kernel and the narrow I2-internal owner/designated-input
carrier at source cut `94e3707c...`; active goal is SYS-2 concurrency/memory/
effect-handler refinement and next goal is SYS-3 per-locus projection/artifact
generation. OPEN-030 is resolved only for that internal contract; the full
carrier freeze and OPEN-026/027 remain. SYS-1 completion and SYS-2 activation
claim neither broad PHASE-I1 exit nor I2 lifecycle acceptance; official theory
remains T1. North Star changes, weakened
safety/privacy guarantees, irreversible final public contracts, real transport
selection/implementation in this program, and production deployment remain
owner-reserved. Outside the program, ADR-0014 keeps research in the reversible
L3 `working/WRK-####` route and L2 remains fail-closed until an
owner-authenticated trust anchor exists. Candidates, countermodels,
experiments, and history remain LAB. The concise control view is
`docs/project-status.md`.

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

Full System V1 は、この alpha floor を final product と誤読せず、Mir source files を semantic source of truth に戻すための bounded LAB roadmap です。textual parser、typed IR/checker、pure/effectful runtime、PoseGraph、projection/local role split、provider/renderer evidence、operational roots、release check はそれぞれ分離したまま runnable evidence を持ちます。現在の computational matrix は parser 2 positive / 8 negative、checker 3 positive / 18 negative、runtime 8 positive / 9 negative、operational suite 12 rowsです。checker/runtime は private exact-pair host-adapter policy を共有し、既存 `read_int@host_input` / `write_int@host_output` の signature、operation-specific capability、transition context を runtime 前に照合します。duplicate record field と record/fixed-array equality も static rejection です。これは trusted runtime authorization、public effect ABI、final grammar、real transport / multi-process execution、C-distributed conformance、final product completionを主張しません。

Surface Mir alpha は、この closed Full System V1 floor の上に置く user-facing source authority line です。Canonical place-scope syntax は `S { ... }` で、`S[ ... ]` は sugar としても採用しません。`[]` は配列・Map・indexed state・role instance head の value-level indexing に残します。`P-SURF-01` で parser floor、`P-SURF-02` で indexed-state semantic checker floor、`P-SURF-03` で Surface-to-Core elaboration evidence floor、`P-SURF-04` で generated `MessageEnvelope` / visible publish / observe / `VisibilityDenied` evidence floor、`P-SURF-05` で role admission / capability grant report-level evidence floor、`P-SURF-06` で source patch hot-plug evidence floor、`P-SURF-07` で source-first operational evidence floor、`P-SURF-08` で Surface source / generated Core IR / semantic-checker-backed indexed-state map / generated communication / role admission / redacted patch lifecycle / source spans の static devtools diagnostics evidence floor を actualize し、`P-SURF-99` で full validation / claim-non-claim audit を close 済みです。
Surface Mir は user-facing source、Core Mir は elaboration target であり、通信・publish・observe は Surface から自動生成して Core IR / devtools に明示します。`state player[p: Participant]: Player` は S-owned Participant-indexed map で、key は authority ではありません。role claim は authority ではなく、authority は admission locus からの capability grant です。source patch hot-plug は direct eval ではなく parse / typecheck / elaborate / admit / activation_cut pipeline として扱います。
post-`P-SURF-99` の G1 LAB work では E-ROW failure-row diagnostics に non-final `lab_diagnostic_details` carrier、request / failure-row context、`E-ROW-002` / `VisibilityDenied` singleton (`ELAB-10`) と exact SCN-01 visible-write `VisibilityDenied` negative (`ELAB-17`) と `E-ROW-001` non-visibility singleton (`ELAB-13..16`) の LAB-only `suggested_repair[]` evidence、OBL-024 / OBL-025 compile-check-only Lean statement drafts、OBL-001/020/021 statement guard hardening、OBL-001 boundary audit、OBL-020/021 boundary audit and OBL-021 guard hardening、G1 ordinary-assignment bridge readiness/non-readiness map、G1 bridge handoff/blocker ledger、G1 acceptance-packet preflight、G1 OBL statement/status completion criteria inventory、G1 status proposal packet outline、G1 status evidence readiness dry-run、G1 requested-status options matrix、ordinary assignment claim-family drilldown、remaining claim-family priority map、repair shape / mixed-multi / set-insertion inventory、`ELAB-07` set-insertion gate review、`ELAB-04` mixed visibility branch inventory、`ELAB-07` set-insertion executable preflight、`ELAB-07` set-insertion assumption acceptance、`ELAB-07` set-insertion payload-model design、`ELAB-07` set-insertion executable payload prototype、`ELAB-07` set-insertion negative-guard hardening、`ELAB-07` set-insertion row-identity guard hardening、`ELAB-07` set-insertion exact-locus guard hardening、`ELAB-07` child / bundle / partial exclusion fixtures、`ELAB-04` mixed visibility payload-model preflight、OBL-025 branch-local non-coverage refinement、OBL-025 repair completeness guard hardening、OBL-024 executable diagnostic-soundness projection carrier、OBL-024 projection Rust fixture guard hardening、OBL-024 replay vocabulary preflight、OBL-024 Lean replay vocabulary refinement、OBL-024 Lean association vocabulary refinement、OBL-024 association guard hardening を追加しましたが、これは final diagnostic/repair ABI、OBL-020/021/024/025 discharge、general set-insertion support、bundle semantics support、visibility-repair ranking、G1 exit ではありません。`ELAB-04` は executable output では引き続き no-repair です。`plan/118` は `plan/70` の ordinary assignment row を traceability-only に分解する LAB memory で、`plan/119` は残り `plan/70` rows の priority map で、G0/G1..G7 exit、canon edit、OBL status movement、conformance、proof 昇格は主張しません。`plan/100` は `ELAB-07` だけについて one existing row-field edit / `element_insert_count = 3` の LAB source-locus edit assumption を受け入れ、`plan/101` は payload を one top-level set item / test matrix として設計し、`plan/102` は exact `ELAB-07` fact pattern だけに non-final `set_insertion` `suggested_repair[]` を実装し、`plan/103` は subset / padded / duplicate / multi-request variants に `set_insertion` repair を出さない Rust-only guard evidence を追加し、`plan/104` は public `target_ref` を変えずに internal association key を existing `when` source span で狭め、`plan/105` は current Surface-expressible omitted-row / retargeting proxies に `set_insertion` repair を出さない exact-locus guard を追加し、`plan/106` は exact `ELAB-07` payload が child singleton alternatives / bundle fields / partial guidance / textual-only guidance ではなく one complete top-level set item のままであることを Rust-only shape guard で固定し、`plan/107` は `ELAB-04` の mixed wrapper / base branch / visibility branch / association / ordering deferral を docs-only で整理し、`plan/108` は OBL-025 Lean draft に abstract branch-local non-coverage helper vocabulary を追加し、`plan/117` は OBL-001/020/021 statement drafts の body-level drift を sync guard で抑え、`plan/126` は OBL-020/021 の current boundary を監査し、OBL-021 required body links が comment-only で通らないよう sync guard を test-only で強化し、`plan/127` は post-`plan/126` の G1 bridge current LAB support / remaining blocker / forbidden claim を表に整理し、G1 exit readiness ではないことを明示し、`plan/128` はその blockers を human/canon acceptance、future statement/proof-package work、canon-open deferral、static LAB support-only、later runtime/conformance/product、reserve trigger に分類し、G1 statement/status と T2 proof discharge を分け、`plan/129` は future G1 acceptance packet の canon files / LAB evidence / statement-status blockers / OPEN-014 deferral / runtime-conformance-product exclusions を preflight routing checklist として整理し、`plan/130` は OBL-001/020/021 status movement を将来 proposal する前の criteria を current LAB support と human/canon decision に分けて整理し、`plan/131` はその criteria を future proposal packet の cover sheet / requested-status matrix / artifact identity / evidence trace / OPEN-deferral / non-claim / ledger delta placeholder / submission checklist へ展開する outline-only memory として整理し、`plan/132` は future packet が cite する OBL-001/020/021 Lean draft compile-check / sync guard / admitted-stub scan を dry-run で通した evidence-readiness memory として整理し、`plan/133` は OBL-001/020/021 の future requested-status 候補として `stated` / `lean-stated` を advisory-only に比較し、OBL-001 は `lean-stated` candidate、OBL-020 は full-row defer / scope acceptance 後の conditional `lean-stated`、OBL-021 は abstraction-boundary acceptance 後の conditional `lean-stated` と整理し、`plan/116` は placeholder repair arrays / repair ranking / all-repairs names / branch-local whole-gap coverage drift を sync guard で抑え、`plan/109` は OBL-024 Lean draft に diagnostic projection / reported failed premise / trace-local replay vocabulary を追加し、`plan/110` は current E-ROW `lab_diagnostic_details` に LAB-only `diagnostic_soundness_projection` を追加し、`plan/111` は `ELAB-04/07/10/13..17` の Rust fixture guard を強化し、`plan/112` は `trace_local_replay` を report-local anchor として future proof-level replay relation から分離し、`plan/113` は OBL-024 Lean draft に `ReportLocalReplayAnchor` と `ProofLevelReplayWitness` / `ProofLevelReplayRelation` の split を反映し、`plan/114` は `ReportLocalAssociationKey` と `ProofLevelAssociationWitness` / `ProofLevelAssociationRelation` の split を反映し、`plan/115` は key equality / branch-local association key への drift を static guard で抑えました。Surface alpha の LAB evidence / repository memory は `specs/39..43` と `plan/64..68`、G1 LAB memory は `plan/70..147` です。current promoted Surface package はありません。
`plan/120` は Product Alpha / Full System V1 / Surface evidence の LAB recut matrix、`plan/121` は G1 ordinary assignment に渡す `G1-MVS-ASSIGNMENT-STATIC` candidate map、`plan/122` は SCN-01 / SCN-02 static bullets を exact / structural support / explicit gap に分ける manifest、`plan/123` は SCN-01 visible-write `VisibilityDenied` negative gap を `ELAB-17` で actualize する LAB memory、`plan/124` は `ELAB-11/12/17` を既存 OBL-001 abstract predicate boundary が運べると監査し、Lean predicate refinement 不要と判断する LAB memory、`plan/125` は SCN-02 direct-local-write negative (b) を現 G1 bridge の即時 blocker ではないが exact executable negative evidence でもないと整理する LAB memory、`plan/126` は OBL-020/021 statement boundary を監査し、Lean predicate refinement 不要のまま OBL-021 guard weakness を test-only に補強する LAB memory、`plan/127` は G1 ordinary-assignment bridge の current LAB support / remaining blocker / forbidden claim を readiness/non-readiness map として整理する LAB memory、`plan/128` はその blockers を authority / next-owner 別に分ける handoff ledger、`plan/129` は future G1 acceptance packet のための preflight routing checklist、`plan/130` は OBL-001/020/021 status movement proposal criteria inventory、`plan/131` は proposal packet outline、`plan/132` は status evidence readiness dry-run、`plan/133` は requested-status options matrix です。いずれも canon edit、gate exit、requested status acceptance、status proposal acceptance、ledger movement、proof/conformance claim、runtime/product/API freeze、sample status relabel ではありません。

`plan/134` は OBL-020 full-row status movement を defer し、G1-supporting
statement-scope candidate を later human/canon review 用に明示する
scope-clarification-only memory です。canon edit、ledger movement、OBL-020
completion、proof/conformance claim、G1 exit は主張しません。

`plan/135` は現 LAB `OBL020StatementDraft` の direct citation を LAB evidence
に限定し、requested-status artifact identity の前に artifact annex /
wrapper decision が必要であることを整理する preflight-only memory です。
wrapper file creation、canon edit、ledger movement、OBL-020 completion、
proof/conformance claim、G1 exit は主張しません。

`plan/136` は、later requested-status packet 用の OBL-020 artifact annex
template として、canon target、LAB artifact path / namespace / constant、
scope label、fresh validation slots、artifact/scope decision slots、
unresolved items、non-claims をまとめる template-only memory です。
proposal submission、requested status acceptance、ledger movement、wrapper
creation、OBL-020 completion、proof/conformance claim、G1 exit は主張しません。

`plan/137` は現 LAB `THM001StatementDraft` の direct citation を LAB
evidence に限定しつつ、OBL-001 が later `lean-stated` の最有力候補で
あること、ただし requested-status artifact identity の前に artifact annex
/ wrapper decision が必要であることを整理する preflight-only memory です。
wrapper file creation、requested status acceptance、canon edit、ledger
movement、OBL-001 completion、OBL-002 proof/conformance claim、G1 exit は
主張しません。

`plan/138` は later OBL-001 `lean-stated` packet 用の artifact annex
template です。canon target、LAB artifact path / namespace / constant、
OPEN-014 deferral、simple assignment scope、fresh validation slots、
artifact/scope decision slots、unresolved items、non-claims をまとめますが、
proposal submission、requested status acceptance、ledger movement、wrapper
creation、OBL-001 completion、OBL-002 proof/conformance claim、G1 exit は
主張しません。

`plan/139` は現 LAB `OBL021StatementDraft` の direct citation を LAB
evidence に限定しつつ、OBL-021 が abstraction-boundary acceptance 後の
conditional later `lean-stated` candidate であること、ただし requested-status
artifact identity には artifact annex / wrapper decision が必要であることを
整理する preflight-only memory です。wrapper file creation、requested status
acceptance、canon edit、ledger movement、OBL-021 completion、proof/conformance
claim、final equality / Diagnostic ABI selection、runtime scheduling
determinism、G1 exit は主張しません。

`plan/140` は later OBL-021 conditional `lean-stated` packet 用の artifact
annex template です。canon target、LAB artifact path / namespace / constant、
abstraction-boundary decision、fresh validation slots、artifact/wrapper decision
slots、unresolved final equality / Diagnostic ABI / projection-totality items、
non-claims をまとめますが、proposal submission、requested status acceptance、
ledger movement、wrapper creation、OBL-021 completion、proof/conformance claim、
runtime scheduling determinism、G1 exit は主張しません。

`plan/141` は G1 OBL status packet shell です。`plan/138` / `plan/136` /
`plan/140` の artifact annex template を参照しつつ、requested status、ledger
delta、artifact identity acceptance、wrapper need、OPEN-014、OBL-020 scope、
OBL-021 abstraction boundary、proof/conformance/runtime/G1 exit をすべて
UNRESOLVED slot として残します。proposal submission、requested status
acceptance、canon edit、ledger movement、OBL completion、proof/conformance claim、
runtime readiness、G1 exit は主張しません。

`plan/142` は G1 status packet shell evidence dry-run です。`plan/141` の
fresh validation slot に対して OBL-001 / OBL-020 / OBL-021 Lean
compile-check、LAB statement sync guard、admitted-stub / placeholder scan、docs
/ source hierarchy validation、secret scan の実測結果を LAB evidence として
記録します。requested status selection、proposal submission、canon edit、
ledger movement、OBL completion、proof/conformance claim、runtime readiness、
G1 exit は主張しません。

`plan/143` は G1 OBL-021 equality / diagnostic abstraction decision packet
です。`plan/140` の OBL-021 artifact annex template を埋める前に、abstract
component result equivalence、abstract diagnostic equivalence、
projection-totality、fixed-input identity、LAB artifact / wrapper boundary について何を
human/canon review に問うかを整理します。requested status selection、
proposal submission、canon edit、ledger movement、OBL-021 completion、proof /
conformance claim、final equality / Diagnostic ABI selection、runtime
scheduling determinism、G1 exit は主張しません。

`plan/144` は G1 OBL-020 scope decision reuse / unresolved-slot audit です。
`plan/134` が OBL-020 full-row vs G1-supporting scope question の controlling
LAB packet であることを確認し、同じ scope matrix を重複して作らないよう
candidate next package を整理します。requested status selection、proposal
submission、canon edit、ledger movement、OBL-020 completion、proof /
conformance claim、wrapper creation、runtime readiness、G1 exit は主張しません。

`plan/145` は G1 OBL-001 artifact decision reuse / unresolved-slot audit
です。`plan/137` / `plan/138` が OBL-001 artifact identity / wrapper /
OPEN-014 / simple-assignment scope の current LAB decision surface であることを
確認し、同じ preflight / annex を重複して作らないよう candidate next package
を整理します。requested status selection、proposal submission、canon edit、
ledger movement、OBL-001 completion、OBL-002 proof / conformance claim、
wrapper creation、OPEN-014 resolution、runtime readiness、G1 exit は主張しません。

`plan/146` は G1 OBL-001 explanation-boundary sync guard hardening です。
`THM001StatementDraft.md` が LAB/canon hierarchy、canon
`MirCore.Elab.Soundness (stmt)` target との非同一性、artifact identity /
wrapper acceptance 未採択、OPEN-014 open を落とさないことを sync test で固定します。
requested status selection、proposal submission、canon edit、ledger movement、
OBL-001 completion、OBL-002 proof / conformance claim、wrapper creation、
OPEN-014 resolution、runtime readiness、G1 exit は主張しません。

`plan/147` は G1 next-line promotion-boundary audit です。広い自走依頼は
OBL-020 / OBL-001 review-facing extraction candidate の promotion ではなく、
次 line は user が明示的に OBL-020 または OBL-001 extraction を選んだときだけ
昇格する、と記録します。requested status selection、review request
extraction、proposal submission、canon edit、ledger movement、wrapper
creation、OPEN-014 resolution、proof / conformance claim、runtime readiness、
G1 exit は主張しません。

`plan/148` は storage workdir mountpoint guard hardening です。external
workdir を exact mountpoint として確認し、root filesystem 上の通常 directory
を mounted 扱いしないよう storage env / cleanup helper を強化します。cleanup、
mount provisioning、cache move、sample / workflow status、canon edit、OBL
status、proof / conformance、runtime readiness、G1 exit は主張しません。

`plan/149` は当時の historical phase-position reading です。そこで記録した
`T0/G0 rebaseline`、9 段階中 1 段階目、T0 late pre-exit / G0 exit 未達という読みは
M2 semantic-assertion v3 acceptance より前の状態であり、現在地ではありません。
現在の official lifecycle は T1 です。
LAB evidence が G1/T1 準備へ先行していることと、canon phase movement を
混同しません。

`plan/150` はその時点の historical phase-position validator guard です。当時の
`plan/149` / `progress.md` / `tasks.md` snapshot drift を検出した記録であり、現在の
T1 statusをT0へ戻すvalidator又はlifecycle authorityではありません。canon edit、
phase / gate movement、percentage-as-gate、runtime / sample / workflow status
change は主張しません。

`plan/151` は Discord webhook secret validator guard です。tracked docs /
source に concrete Discord webhook URL shape が入った場合に
`scripts/validate_docs.py` が path / line だけを出して失敗するようにします。
webhook credential の保存、出力、通知挙動変更、security completeness claim は
主張しません。

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
  LAB evidence は `specs/13..17`、repository memory は `plan/39..43`、phase-indexed sample scaffold は `samples/alpha/` に置きます。これは active runnable root の置換ではなく、current-scope evidence を蓄積する alpha-local scaffolding です。
- **Mirrorea Spaces practical alpha-1 line**
  LAB evidence は `specs/18-practical-alpha1-scope.md`、repository memory は `plan/44-practical-alpha1-roadmap.md` に置きます。ここは source front-door、checker、runtime、package/hot-plug、transport、devtools、local save/load、product preview を揃える first-floor toolchain line であり、operational α-0.5 / α-0.8 readiness そのものではありません。
  ただし、これは promoted work queue であり、active canonical runnable root への昇格を意味しません。2026-05-05 時点の latest package closeout は `P-A1-23` practical α-1 integrated workflow carrier です。
  2026-05-05 時点の practical alpha-1 evidence は `SRC` / `CHK` / `RUN` / `HP-A1` / `TR-A1` / `VIS-A1` / `SL-A1` / `AV-A1` / `PE2E` families に分かれます。詳細な row 一覧と carrier split は `samples/practical-alpha1/README.md`、検証 dashboard は `samples_progress.md`、長期 memory は `plan/44-practical-alpha1-roadmap.md` に集約します。
  root では、`PA1W-01..08` が first-floor evidence と operational α line を bounded practical developer workflow に束ねていることだけを押さえます。これは final public parser / viewer / telemetry ABI、distributed durable save/load、WAN/federation、product/public-ready α-1 を意味しません。

- **Operational alpha theory-freeze / session-runtime line**
  LAB evidence は `specs/19..24`、repository memory は `plan/45..49` に置きます。ここでは runtime を広げずに、verification stratification、`atomic_cut` / consistent cut / save-load semantics、auth / rate-limit / debug の contract-transformer 理論、typed observability、typed external host boundary、そして α-0.5 / α-0.8 / α-0.9 の operational readiness 条件を記録します。
  `P-A1-19`、`P-A1-20`、`P-A1-21`、`P-A1-22` により、same-session α-0.5 session carrier、typed external `AddOne` host-I/O adapter lane、debug / auth / rate-limit / object preview / deferred detach の same-session attach lane、そして event DAG / local route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / observer-safe redacted view / retention-on-demand trace の session-bound devtools export が actualize され、bounded operational α-0.5 / α-0.8 / α-0.9 line は揃いました。`AddOne` は host-boundary evidence であり、Mir-owned computational-core completion ではありません。`P-A1-23` はその line と practical first floors を bounded practical α-1 workflow として束ねました。final public viewer / telemetry ABI、durable audit、distributed durable save/load、final-public product hardening は引き続き未完です。

- **Product/Public-ready Mirrorea Spaces alpha-1 line**
  LAB evidence は `specs/25..27`、repository memory は `plan/50..52` に置きます。current line は `mirrorea-alpha`、versioned `package.mir.json`、local/Docker controlled runtime、non-final devtools/viewer、R0/R2 save evidence、native host launch bundle、product release check、installed-binary probe、operational product sample suiteを持ちます。
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

### Product Alpha Docker fixture の setup 入力例

通常は `mirrorea-alpha transport --mode docker` または release-check helper が
下表を内部で設定するため、手入力は不要です。Compose を直接確認する場合だけ、全て
absolute path で渡します。パスワードや実アカウントは不要で、fixture token は
accidental-use guard であって認証情報ではありません。

| 入力 | 具体例 | 用途 |
| --- | --- | --- |
| `MIRROREA_PRODUCT_ALPHA1_BINARY` | `"$PWD/target/debug/mirrorea-alpha"` | `cargo build -p mirrorea-cli` 後の実行可能 binary |
| `MIRROREA_ALPHA_SESSION_DIR` | `"/tmp/mirrorea-readme-session"` | `run-local` 前に指定する local session store |
| `MIRROREA_PRODUCT_ALPHA1_SESSION_FILE` | `"/tmp/mirrorea-readme-session/session_product-alpha1-demo.4c1aa1672b497b43.session.json"` | `run-local` JSON が返した `session_path`。`session` はこの既存 file を読む |
| `MIRROREA_PRODUCT_ALPHA1_OUTPUT_DIR` | `"/tmp/mirrorea-alpha1-docker"` | Compose の `world.json` / `participant.json` 出力先 |
| `MIRROREA_PRODUCT_ALPHA1_TRANSPORT_FIXTURE_TOKEN` | `"mirrorea-local-fixture-20260723"` | Docker fixture 専用の一致 token。password / public credential ではない |

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

この README の次は、まず canon source を読み、その後に task-specific LAB
evidence / repository memory を追います。

1. `mirrorea_canon/README.md`
2. `mirrorea_canon/MAP.md`
3. task に関係する canon files:
   `mirrorea_canon/meta/`、`mirrorea_canon/adr/`、
   `mirrorea_canon/plan/`、`mirrorea_canon/theory/`、
   `mirrorea_canon/spec/`、`mirrorea_canon/architecture/`、
   `mirrorea_canon/scenarios/`
4. `Documentation.md`
5. current status / roadmap / remaining steps を扱う task なら `progress.md` と `tasks.md`
6. phase recut / roadmap rewrite / progress/tasks reorganization を扱う task なら `.docs/progress-task-axes.md`
7. LAB follow-up として `specs/00-document-map.md`
8. LAB follow-up として `specs/01-charter-and-decision-levels.md`
9. LAB follow-up として `specs/02-system-overview.md`
10. LAB follow-up として `specs/03-layer-model.md`
11. LAB follow-up として `specs/09-invariants-and-constraints.md`
12. 必要な subsystem legacy spec と `plan/00-index.md`
13. practical alpha-1 を扱う task なら LAB evidence として `specs/18-practical-alpha1-scope.md` と `plan/44-practical-alpha1-roadmap.md`
14. operational readiness / theory freeze を扱う task なら LAB evidence として `specs/19..24` と `plan/45..49`
15. product/public-ready alpha-1 を扱う task なら LAB evidence として `specs/25` と `plan/50`
16. operational product sample suite を扱う task なら LAB evidence として `specs/26..27` と `plan/51..52`

task が specific `sub-agent-pro/*.md` handoff を名指しした場合は、その handoff を user 指示順で先に読みます。
ただし handoff は規範正本ではなく、必要な内容は `mirrorea_canon/` または
LAB source hierarchy (`specs/` / `plan/` / docs / report) へ mirror して扱います。

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
