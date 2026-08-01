# Plan 246 - 目的起点の意味論統合と I1 入口

## 役割と権限

これは Canon と既存 LAB 証拠を読み直して作る **LAB repository memory / I1 semantic-cut decision packet** である。規範正本は常に `mirrorea_canon/` であり、本書は Core、Config、SaveObject、Surface grammar、failure universe、SCN、OBL、Gate、Phase、proof status、runtime、wire/API、公開契約を変更しない。

本書の目的は、official lifecycle が `T0` に留まる事実と、意味論を統合して I1 参照実装の準備を進める仕事を混同しないことにある。T0 governance profile の fixed-control drift は Gate/Phase の受理に関する未決であり、program meaning、SCN conformance、proof の意味論的前提ではない。したがってこれは可視の並行 lane として維持するが、Core integration、反例検査、bounded prototype、shared-model 設計の停止理由にはしない。Canon への昇格、official T1/T2/I1 受理、または production implementation authorization には通常の proposal / owner / Canon 手続が必要である。

## 目的の mirror

Mir を意味の正本として、正しい理論に基づき、正しく hot-plug でき、Place をまたいで実行・通信・検証・可視化できる仮想空間システムの基盤を実際に動かせる状態へ、研究・設計・実装・検証を段階的に進める。`World` と `Game` は Mir の組込み概念ではなく、利用者が Mir 上で定義する概念に保つ。Mir、Mirrorea、PrismCascade、Typed-Effect Wiring Platform、adapter/provider/view は境界を保って分離する。

直近の主線は、I1 の単一プロセス参照実装を実際に開始・検証できる最小の理論統合である。局所的 L3 countermodel や conditional lemma は、shared semantics、statement/profile、proof skeleton、I1 readiness の明確な consumer があるときだけ用いる。既存の runnable LAB は再利用可能な実験証拠であって、Gate/Phase/OBL/public completion ではない。統治上の hash、形式、report の drift は、安全性または権限境界を実際に損なわない限り、本線の意味論・実装準備を止めない。

## 現在地の二本立て

| Lane | 現在の事実 | この plan で進めること | 停止線 |
| --- | --- | --- | --- |
| official lifecycle | Canon は `T0`。v2 governance artifact は valid `fail`。G0/T1 entry record はない | evidence と将来の owner packet を保守する | G0 exit、T1/T2/I1 acceptance、Canon amendment |
| goal-first semantic integration | Core、型/failure、authority、fallback、cut、observation、patch と P012/P013/P015/P016/P017 の方向がある。共通 state machine は未統合 | S2-A で C1/C2 amendment hypothesis の exact delta と adverse trace を comparison し、ordinary selection 後だけ formal model/prototype へ進む | candidate が L0/L1・external contract・SCN expectation を変えると判明した時点 |
| runnable LAB | textual parser/checker/interpreter、Surface elaboration、same-process runtime、samples、Lean statement drafts が動く | front-end、fixture、trace/report、deterministic harness を再利用する | 既存 LAB を Canon conformance 又は semantic truth と読み替えない |

つまり **official T0 は維持するが、semantic integration は今から自走する**。この区別は phase shortcut ではなく、P016 の narrow T2 / separate I1-readiness という方向と、ADR-0013 の governance profile 非意味論性を運用へ反映するものである。

## 最終像と I1 の境界

```text
user-defined World / Game
  -> ordinary .mir source
  -> Surface parse / check / elaboration
  -> typed Core + obligations + generated edges
  -> deterministic one-process reference machine
  -> typed occurrence / authority / failure / cut / observation evidence
  -> I2 in-process multi-Place -> I3 transport -> I4 durable patch
  -> I5 View/projection -> I6 federation
```

I1 は小さな SCN subset ではない。frozen `SCN-01..10` を、単一プロセスの決定的 profile で C-static と C-runtime として扱える最小意味論である。I1 からは性能、socket/wire、fairness/retry、WAN identity、distributed durability、最終 ABI、最終 UI を外す。一方、SCN-07 の typed/redacted observation、SCN-09 の patch no-mutation、SCN-10 の cut/save/load の statement-level semantics は外さない。

## I1 semantic-cut: 既決・作業候補・後段を分ける

### 既に direction があるもの

| Topic | Direction source | I1 での読み |
| --- | --- | --- |
| ordinary source | NORTH-STAR, theory/01, theory/03 | source は placement/communication を直接書かなくてよい。elaboration は request/publish/observe と span/obligation を明示する |
| value / occurrence | P012 | V1 restricted result binding、R1 typed owner result + requester receipt、SW1 atomic served-write、conditional A2 composite admission を後続設計の direction として使う |
| request validation | P013 | M1 request-local claims は non-authoritative input。owner が live membership/lineage/witness/visibility を fail-closed で検査する |
| exchange residence | P017 | X1 relation-state envelope。request occurrence `q` を history anchor とし、side table / span / payload / transport を hidden correlation にしない |
| fallback / terminal | P015, theory/06 | scalar terminal/default は explicit。`return` は v0 source へ入れない。degradation は monotone、reacquire は fresh lineage |
| lifecycle | P016 | T2 は narrow。I1 readiness/bootstrap は別 record。C-static formal entry と I1 exit を混同しない |

### 作業用の cut と authority boundary

これは Canon amendment ではない。D0/D3/D4 は既存方向を照合する LAB working
candidate、D1/D2 は現行 Core/SCN をそのまま実装できないことを確かめるための
**ordinary Core/SCN amendment hypothesis** である。とくに D1 は現行
`[WRITE-CROSS]` の already-determined `v′` と sampling time を変え、D2 は
`Config` と dynamic causal facts の residence を追加しうる。いずれも本文書だけで
採択、実装、C-static artifact の変更、OBL への流入をしてはならない。

| Cut | Candidate | 意図的な非保証 | decisive falsifier |
| --- | --- | --- | --- |
| D0 front-end outcome | finite declared v0 fragment に対し、`Elab(input)` は `Ok(core, type, mode, effects, failures, constraints, obligations, edges)` 又は `Diagnostic(id, span, payload)` を返す total administrative result とする。structural equality は field-by-field、Diagnostic equality は id/span/payload に限る | final grammar、public diagnostic ABI、arbitrary dependent typing | well-scoped finite input が `Ok`/`Diagnostic` のいずれにもならない、または同一 input が異なる output になる |
| D1 read-dependent write | **C1-A amendment hypothesis.** SCN-02 の target owner に属する read だけを含む assignment を、target owner が service 時に owned reads、pure RHS calculation、validated mutation を一つの SW1 occurrence で行う owner transition にする。現行 `[WRITE-CROSS]` の determined `v′` を literal に実装するものではなく、SCN-02 dependency rows と sampling time を含む precise Core/SCN proposal が先に必要である。別 owner operand はこの hypothesis の scope 外であり、黙って V1/R1 binding に入れない | distributed transaction、複数 owner の atomicity、retry/cache/freshness、first-class closure、cross-owner operand semantics | 同じ owner の二 attack が stale read reply + blind write になり lost update を起こす、failure 後に dependent write が出る、又は transition が一 owner を越えて atomicity を主張する |
| D2 X1 exchange | **C2-A amendment hypothesis.** `Config` の将来候補 `X` は request occurrence `q` ごとの relation entry。entry は requester/resumption locus、M1 claims と consulted-provenance refs、owner outstanding/one terminal result-or-failure、semantic receipt state、one-shot use state を持つ。現行 static `G_e` request row は request/publish/observe/witness のまま保ち、owner result/failure、receipt、use は `X` の future dynamic transition facts として別に定義する。semantic receipt は既存 `G_e` row ではない。`q` は source/wire/public ID ではない | final field names、transport message、global exactly-once、fairness、timeout/cancel、cross-load global identity、new causal generator/event kind | equal span/payload/queue/transport data が二 request を併合する、owner failure が mutation する、receipt swap/duplicate が use を増やす、load が use budget を reset する、又は receipt を existing `G_e` row として扱う |
| D3 occurrence / authority | successful remote write は one `ServedWrite` occurrence の named service/mutation/validated-authority facets とする。admission は、issuance が別に observable/schedulable/failing でない I1 scope では one verdict occurrence の named membership/grant/witness facets とする | write acknowledgement、separate issuance protocol、production identity | a claimed facet の causal/lineage reference が作れない、または rejection が M/G/W/S を変える |
| D4 scalar / terminal | indexed state と scalar state を別の declared state categoryにする。scalar は explicit owner/init/visibility/store slot を持つ。chain terminal は declared constant 又は declared scalar target に明示的に解決し、type default / hidden singleton key / unbound terminal を使わない | final Surface spelling、custom keyspace expansion、global default registry | `SCN-08` が hidden participant membership、type default、または未宣言 value を必要とする |

`D1` は C1 を distributed transaction に拡大せずに検討する最小 amendment
hypothesis である。SCN-02 の target-owner reads と mutation を owner seriality の
内部で一つの semantic service にできるかを、現行 `[WRITE-CROSS]` との差分として
検査する。一方、複数 owner にまたがる snapshot、retry、cache、wire-level atomicity
は導入しない。ordinary source が通信の細部を書かずに済むことと、elaborated Core が
typed owner transition/dependency provenance を明示することの両方を満たせない場合は、
C1-A を凍結して alternative comparison に戻る。

### D1/D2 の即時停止線と contrast trace

- **C1-A の scope** は target owner に属する reads だけである。別 owner operand が
  必要になった時点で S2 は semantic result を出さず停止し、target owner が requester
  になるか、nested `q` と M1 validation をどう置くか、causal edge を何にするかを
  別の precise proposal choice として提示する。元の actor へ値を返して blind write へ
  戻すことは許さない。
- **C2-A の static/dynamic separation** は必須である。`G_e` は static
  elaboration output のままにし、result/failure/receipt/use は future `X` transition
  specification の候補としてのみ記す。新しい event/causal generator が必要なら、それを
  隠さず ordinary Canon proposal の amendment surface とする。
- **deferred cross-owner contrast**: 将来 C1-X を試す場合は、owner A が operand を
  return した後に A 自身が更新され、target owner B が受け取った値で B-local transition
  を commit する trace を必ず含める。この trace は common snapshot、lock、rollback、
  multi-owner transaction を持たないことを確認するためのものなので、C1-A の保証にも
  current semantics にも読み替えない。

### 必ず一緒に検証する既存 invariant

1. `H` は DAG。request-to-service、authority lineage-to-service、result/send-side fact-to-receipt、accepted receipt-to-use、use/dependency-to-later occurrence の根拠を一つずつ示す。
2. M1 claims、transport/session、locus は authority ではない。owner validation は epoch, incarnation, grant/witness lineage, admission, visibility を live state に照らす。
3. dynamic failure は declared row 内で explicit。static Diagnostic と dynamic failure を混ぜない。owner failure と requester-side receipt rejection も混ぜない。I1 candidate は receipt rejection を semantic transition 外として保持し、新 failure member は導入しない。
4. X entry は SaveObject/cut closure に入り、restore は branch を merge/duplicate/revalidate せず、consumed entry を unconsume しない。load successfulness に no-stale conclusion を埋め込んで循環させない。
5. X storage は observation ではない。export が必要なら theory/07 の subject, authority, label, redaction, retention, reason, span を持つ separate projection にする。
6. `World`、`Game`、transport、provider、viewer はこの machine の Core primitive にしない。

## Scenario と model の coverage

| SCN | I1 kernel consumer | required negative evidence |
| --- | --- | --- |
| 01 roll | D0, D2/D3, publish; owner-directed write then visible publication | undeclared `VisibilityDenied` is static diagnostic |
| 02 attack | D0, C1-A/C2-A hypothesis, M1, SW1; target-owner-local read/calculate/write contrast | missing capability has no mutation; no hidden local write; same-owner concurrent attacks distinguish C1-A from stale blind write; cross-owner operand is rejected/deferred rather than silently implemented |
| 03 late join | conditional A2, M1, D3 | pre-verdict / copied capref rejection |
| 04 owner leave | membership/incarnation, D2 restore facts | stale capref and stale rejoin rejection |
| 05 portal | two admission verdicts, target lineage, visibility | private field and wrong-target capref rejection |
| 06 shard | route failure classification | `RouteUnavailable`, no silent retry/hang |
| 07 observation | publish/observe projection | private/auth raw data absent; monotone redaction |
| 08 fallback | D4 plus monotone chain | hidden scalar/default and rollback repromotion rejection |
| 09 patch | patch pipeline/frontier and D3 no-mutation | self-grant/capability rejection, deferred frontier mismatch |
| 10 save/load | D2 restore, cut consistency, membership/lease state | stale resurrection and incomplete channel closure rejection |

## Existing code: reuse boundary

The Full System V1 textual parser/checker, deterministic runtime report, CLI and fixture harness are valuable implementation material. They are not a Canon semantic source. Only after an ordinary selection and implementation authorization may a future implementation package retain their parser/front-end APIs, JSON report transport, deterministic harness, fixture layout, expected JSON comparison and error plumbing. It must then replace or isolate their bounded state machine behind the selected common model, and add exact Canon-SCN fixtures instead of relabelling their current matrices as conformance.

Do not pull projection, provider, renderer, actual multi-process transport, or PrismCascade into I1 kernel work. Those are later typed boundaries and would create a second semantic centre.

## Execution sequence

| Package | Authority | Deliverable and validation | Completion boundary |
| --- | --- | --- | --- |
| S1 (this packet) | A/R | goal mirror, authority cut, semantic hypotheses, SCN/OBL coverage, reuse boundary, independent planner/Oracle review | review-corrected packet distinguishes a LAB candidate from a Core/SCN amendment hypothesis; no Canon decision asserted |
| S2-A bounded comparison and amendment packet | A/R | in this existing `plan/` / `docs/reports/` LAB lane only, make the C1-A/C1-B and C2-A alternatives, adverse traces, exact current-Core/SCN deltas, non-effects, and rollback explicit | produces an ordinary owner/Canon proposal packet; no new helper, schema, Lean source, runtime code, or implementation claim |
| S2-B shared kernel model | O/Canon selection, then A/R | after the selected amendment surface is authorized, construct the non-opaque `Surface/Core/Config/Step/WellFormed/Elab/SaveLoad` model and run permitted Lean/prototype checks | formal model imports the selected semantics; it does not retrofit an unselected candidate |
| S3 candidate-local statement preparation | A/R after S2-B | derive candidate-local OBL/G2/G3/G5 statement drafts and exact SCN explanation | no ledger/OBL/profile mutation or official T1 claim before ordinary acceptance |
| S4 narrow T2 + G5 | O/Canon after selected statement package | import-bearing OBL-020/021/002 skeletons plus separate save predicate, restore relation, post-state, checker, Z-cycle statements | avoids circular load theorem; official T2 close remains separate |
| S5 I1 bootstrap/readiness | A/R prepares, O/Canon accepts | P016-compatible all-SCN scope, deterministic profile hash, carrier baseline, evidence classifications, C-static wording | bootstrap/official authorization only after the bounded record exists |
| S6 I1 reference implementation | implementation owner + A/R | one `mir-parse/check/elab/run` route, exact C-static then C-runtime 10/10, no waiver, carrier freeze at I1 exit | no performance/distributed/public-completion claim |

The official governance lane runs beside S1--S3. It must be resolved before official lifecycle acceptance and implementation authorization, but it does not serially block S2-A's bounded comparison and owner-packet preparation.

## Research controls and reopen triggers

- Do not create more fixture-only WRKs merely to extend WRK-0045/0046. Their result is retained evidence; neither supplies a full exchange model.
- **S2-A entry guard:** its only retained locations are this Plan and its task reports; its
  read-only anchors are P012/P013/P015/P016/P017, theory 01/03/04/05/06/07,
  SCN-01..10, and ADR-0013/0014. It compares C1-A to a determined-value C1-B
  alternative and C2-A to no-receipt-state status quo; it records the listed
  falsifiers, has no Gate/Phase/OBL/implementation effect, and rolls back by
  freezing the hypothesis and issuing no proposal. It creates no helper family,
  schema, CI/Make surface, Lean source, runtime code, or new evidence lane.
- Because C1-A/C2-A touch a Core/Config/SCN boundary, a retained formal model,
  prototype, or implementation use is not delegated by this guard. It stops for
  an ordinary owner/Canon proposal before S2-B. An L3 record is not used to
  bypass that reserved boundary.
- Run each S2-A comparison against its listed falsifiers, `SCN-01..10`, and
  existing DAG/authority/failure/cut invariants before treating it as a proposal.
- User/Canon review is needed when a hypothesis must become a normative Core,
  Config, SaveObject, failure, SCN, or profile change. A contradiction or a
  counterexample freezes the hypothesis and returns to explicit alternatives,
  not to a forced fit with current wording.
- Small implementation representations remain replaceable until the I1 carrier freeze. Semantic state must never be hidden in an evaluator-only side table.

## Current recommendation

Proceed immediately with **S2-A bounded comparison and amendment packet**. This is
the smallest autonomous move: it turns the recorded directions into a precise
choice with falsifiers without implementing unselected Core/SCN semantics. The
first comparison focuses on C1-A/C1-B and C2-A/status quo. Only after ordinary
owner/Canon selection can S2-B formalize or prototype the selected common model.

## Non-claims

This plan does not change the official `T0` status, repair the governance profile, exit a Gate, close an OBL, establish C-static/C-runtime conformance, authorize I1 implementation, or claim public/product completion. It does not freeze final syntax, carrier field names, wire format, public API, identity provider, transport, persistence, renderer, or View.
