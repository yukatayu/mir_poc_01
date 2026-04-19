# 458 — current L2 FAQ 006 drift audit, first-line / stop-line integration, and queue reconstruction

## 目的

`faq_006.md` が示した

- promotion / stop-line / contract concretization 段階への移行
- `current self-driven queue = 0 package` 読みの drift
- p06 / p07 / p08 corrected prototype tranche close 後に残る理論線

を、既存の comparison / threshold / tranche docs に接続した上で、
current first line、retained alternative families、stop line、true user-spec gate を
1 本に再統合する。

ここで fixed するのは **current first line の integration** であり、

- final parser grammar
- final public verifier contract
- final shared-space catalog
- concrete theorem / model-check production binding
- final source-surface order / handoff syntax

は fixed しない。

## input delta の扱い

- `faq_006.md` は current explanation であり、規範判断の正本ではない。
- ただし 2026-04-17 時点の current reading delta として、
  current snapshot / queue / stop-line の drift を監査する source として使ってよい。
- 規範判断の正本は引き続き `specs/` に残る。

## drift audit

### current drift

current repo には、次の drift があった。

1. `faq_006.md` は、repo が
   「未着手」ではなく
   「promotion / stop-line / contract concretization」
   の段階に入ったと説明している。
2. 一方で `progress.md`、`tasks.md`、`plan/01`、`plan/11`、`plan/17` は、
   `p06` / `p07` / `p08` tranche close を
   `current self-driven queue = 0 package`
   と読み替え、
   theory-lab line 全体が mixed gate only へ入ったように見せていた。
3. しかし `specs/examples/412`、`plan/12`、`plan/18`、そして
   `specs/examples/444...447`
   が fixed しているのは、
   actual adoption ではなく
   threshold / later-gate / stop-line framing である。

### current judgment

current L2 で最も自然なのは、

- corrected prototype tranche close
- mixed-gate actual adoption

の間に、
**self-driven integrator package**
を明示的に置き直すことである。

## 問題 1 — typed / theorem / model-check

### property-to-boundary matrix refresh

| property family | principal boundary | current first line | kept later |
|---|---|---|---|
| obligation owner / typed attachment allocation | `core_static_checker` | checker-adjacent semantic carrier principal | full type calculus / public typed API |
| source-visible typed cue | `core_static_checker` | structural marker family first、request / predicate / `try` cluster は reserve | stronger typed surface adoption / shared attachment shape |
| semantic-core lemma family | `theorem_prover_boundary` | `canonical_normalization_law -> no_re_promotion -> rollback_cut_non_interference` | wider theorem library |
| notebook-first theorem consumer | `theorem_prover_boundary` | `proof_notebook_review_unit` + symbolic-ref-only admissible evidence + abstract discharge-entry reserve | discharge result / receipt / proof object public contract |
| row-local model-check carrier | `protocol_verifier_boundary` | `model_check_concrete_carriers` principal | settled property language / concrete tool schema |
| same-subject stage-local projection | `protocol_verifier_boundary` | small-cluster projection reserve | room protocol / fairness / replay projection |
| fairness / replay / provider receipt | `runtime_policy_boundary` | current principal line の外に保つ | final operational profile / final policy contract |

### current first line

current L2 で source-backed な current first line は次である。

1. principal typed source は checker-adjacent semantic carrier に残す。
2. first source-visible typed cue は structural marker family に置く。
3. request / predicate / `try` cluster は grouped reserve cue に留める。
4. theorem line は notebook-first consumer と symbolic-ref-only admissible evidence floor に留める。
5. theorem discharge transport seam と public-contract seam は coupled だが distinct な later gate に留める。
6. model-check line は row-local carrier first と same-subject stage-local small-cluster projection reserve に留める。
7. first settled property language と concrete tool seam は adjacent later gate に留める。

### retained alternative families

- stronger typed surface actual promotion
- request / predicate / `try` cluster の source-visible promotion
- discharge-result / receipt family
- settled property language family
- concrete theorem prover / model-check tool seam

### current recommendation

current L2 で最も自然なのは、
**stronger typed surface を public recommendation へ上げず、
`p06` を sample-visible corrected prototype + helper-local preview に留める**
ことである。

同様に、

- theorem discharge は notebook-first + abstract discharge-entry reserve に留める
- model-check は row-local carrier + small-cluster projection reserve に留める

のが current first line である。

### stop line

- stronger typed surface の actual adoption
- full strong type system
- public theorem contract
- public verifier contract
- settled property language adoption
- concrete theorem / model-check tool binding

### true user-spec gate

problem 1 の immediate user-spec gate はまだ強くない。
current gate は主に **mixed gate / concrete consumer gate** であり、
true user-spec gate と読むのは次である。

- first external prover / checker brand を何に寄せるか
- public verifier / theorem contract をどの consumer class に合わせるか
- repo 外 integration target をどこに置くか

## 問題 2 — order / handoff / `memory_order`

### cut / order / handoff integration note

| family | current first line | retained alternatives | stop line |
|---|---|---|---|
| cut family | `atomic_cut` = place-local finalization nucleus | `snapshot_cut` / `consistent_cut` comparison vocabulary、`durable_cut`、`barrier` | `atomic_cut` を global snapshot / durable commit / global fence と同一視しない |
| order relation family | `program_order / dependency_order / publication_order / observation_order / witness_order / finalization_order / scoped_happens_before` | exact field naming / surface tokenization | relation 名を final public keyword と書かない |
| authority-handoff family | `authority_serial_transition_family` first、`witness_aware_commit_family` second | `event_tree_execution_view` derived/debug candidate | final operational profile / fairness catalog を hidden promotion しない |
| wording line | snake_case relation family 名 principal + plain-language stage wording explanation layer | low-level fence wording、room-macro wording | final source-surface handoff syntax を fixed しない |
| low-level mapping | `std::memory_order` + `std::kill_dependency` family は backend-aligned reference family | source-surface direct import | low-level exact family を current source core に輸入しない |

### thread / node parity note

bad wording:

- `thread == node`

current principal wording:

- thread も node も place として同じ causal language で書く
- 差は lowering / evidence / transport / failure / durability / policy に残す

### current recommendation

current L2 で最も自然なのは、
**relation decomposition を principal に保ち、
low-level `memory_order` family は backend-aligned reference family に留める**
ことである。

これにより、

- shared-memory lowering
- distributed authority-handoff lowering

を同じ causal language で比較しつつ、
source surface の premature freeze を避けられる。

### retained alternative families

- `witness_aware_commit_family`
- `event_tree_execution_view`
- `snapshot_cut` / `consistent_cut` naming family
- low-level fence-like public wording
- room-macro / transaction-like sugar

### stop line

- final source-surface handoff syntax
- final emitted-artifact schema
- final property language
- low-level `memory_order` final public stance
- fairness / replay operational profile

### true user-spec gate

problem 2 で true user-spec gate に入るのは次である。

- shared-space final activation / authority / fairness catalog
- concrete replay invalidation protocol
- concrete authority / provider binding
- application-specific room policy

## syntax / modality integration

### syntax / semantics coupling principle

current syntax candidate の principal axis は次である。

1. semantic honesty
2. checker legibility
3. modal adequacy
4. misreading resistance
5. lowering friendliness

### modal basis reading

- `lambda_circle_box` は partial basis candidate に留める
- guarded lambda-calculus、MDTT、MTT、Fitch-style multimodal は stronger candidate family として retained する
- exact reduction timing は threshold-gated parallel keep に留める
- internalization trigger は boundary-pressure trigger に置く

### stop line

- final foundation adoption
- final source-surface modality marker
- full dependent calculus
- checker / theorem / runtime integration binding

## sample / prototype placement

| prototype | current role | public / helper split | explicit non-goal |
|---|---|---|---|
| `p06-typed-proof-owner-handoff` | typed/theorem/model-check sample-visible corrected prototype | helper-local `verification_preview` / `artifact_preview` を使って current bridge floor を見せる | final typed calculus / public verifier contract / settled property language |
| `p07-dice-late-join-visible-history` | order/handoff corrected prototype | publication / handoff / observation の corrected runtime comparison | final shared-space contract / final handoff syntax |
| `p08-dice-stale-reconnect-refresh` | order/handoff corrected prototype | stale reconnect / rollback / refresh fallback の corrected runtime comparison | final replay protocol / final fairness profile |

## queue reconstruction

### package map

| package | question | candidate family | adequacy corpus | prototype / helper-local preview | kill criteria | promotion criteria |
|---|---|---|---|---|---|---|
| `A` drift audit + queue reconstruction | `faq_006.md` と snapshot / plan / spec はどこで drift したか | snapshot reading / package map | `faq_006.md`、`progress.md`、`tasks.md`、`plan/11`、`plan/17` | `p06` / `p07` / `p08` placement audit | `0 package` 読みが source-backed でないと確認された時点 | snapshot / plan / spec に current queue を戻す |
| `B` verifier-boundary / typed-theorem-model-check promotion package | stronger typed surface / theorem discharge / model-check seam を current first line と stop line でどう整理するか | checker attachment principal、notebook-first theorem、row-local model-check | `126`、`127`、`133`、`134`、`135`、`413`、`418`、`433`、`439`、`440`、`445`、`446`、`447` | `p06`、helper-local `verification_preview` / `artifact_preview` | theorem/model-check artifact を typed source of truth に繰り上げたら kill | current first line / retained alternatives / later gate が一貫したら promote |
| `C` order / handoff relation decomposition package | cut / relation / authority-handoff / wording line をどこで止めるか | relation decomposition principal、`authority_serial_transition_family` first | `405`、`406`、`407`、`408`、`409`、`411`、`416`、`421`、`436`、`442`、`448` | `p07`、`p08`、helper-local runtime comparison | `atomic_cut` を snapshot / durable cut / fence と collapse したら kill | current wording、retained alternatives、shared-space mixed gate を分離できたら promote |
| `D` syntax / modality package | semantic honesty と stronger foundation candidate をどこまで current reading に上げるか | `lambda_circle_box` partial basis + guarded / MDTT / MTT / Fitch-style multimodal keep | `409`、`410`、`422`、`437`、`444` | rough stimuli A-D、`samples/not_implemented/order-handoff/` | compactness-first / hidden precedence / one-family-only readingに崩れたら kill | comparison axis と trigger / stop line が一貫したら promote |
| `E` integration package | current recommendation / stop line / user-spec gate / next self-driven line をどう snapshot へ戻すか | integrator package | `458`、`Documentation.md`、`tasks.md`、`progress.md`、`plan/01`、`plan/11`、`plan/17` | prototype placement matrix と report chain | comparison only で recommendation を書けない状態なら kill | next queue と mixed/user-spec split が snapshot に同期したら promote |

### current judgment

- `Package A` はこの integration note と同 task の snapshot refresh で close してよい。
- current promoted self-driven queue には、少なくとも `B`、`C`、`D`、`E` を戻す。
- `E` は recurring integrator package であり、`B` / `C` / `D` の close 後に再度使ってよい。

## keep / drop summary

### keep

- mainline actualization と theory-lab line を分ける
- corrected prototype tranche close と actual adoption を分ける
- relation decomposition principal
- checker attachment principal
- notebook-first theorem line
- row-local carrier principal
- `lambda_circle_box` partial basis

### drop from current package

- `0 package` を theory line solved と読むこと
- helper-local preview を public contract と読むこと
- `atomic_cut` を global snapshot / durable commit と同一視すること
- stronger typed surface actual adoption を hidden promotion すること
- fairness / replay final profile を docs-first package に押し込むこと

