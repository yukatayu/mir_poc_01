# Plan 246 - 目的起点の意味論統合と I1 入口

## 役割と権限

これは Canon と既存 LAB 証拠を読み直して作る **LAB repository memory / I1 semantic-cut decision packet** である。規範正本は常に `mirrorea_canon/` であり、本書は Core、Config、SaveObject、Surface grammar、failure universe、SCN、OBL、Gate、Phase、proof status、runtime、wire/API、公開契約を変更しない。

本書の目的は、official lifecycle が `T0` に留まる事実と、意味論を統合して I1 参照実装の準備を進める仕事を混同しないことにある。T0 governance profile の fixed-control drift は Gate/Phase の受理に関する未決であり、program meaning、SCN conformance、proof の意味論的前提ではない。したがってこれは可視の並行 lane として維持するが、S2-A の比較、反例検査、owner packet 準備の停止理由にはしない。Core integration、bounded prototype、shared-model 設計は、当該 Core/SCN amendment が通常の proposal / owner / Canon 手続で選択された後に進める。official T1/T2/I1 受理または production implementation authorization も同じく通常手続を要する。

## 目的の mirror

Mir を意味の正本として、正しい理論に基づき、正しく hot-plug でき、Place をまたいで実行・通信・検証・可視化できる仮想空間システムの基盤を実際に動かせる状態へ、研究・設計・実装・検証を段階的に進める。`World` と `Game` は Mir の組込み概念ではなく、利用者が Mir 上で定義する概念に保つ。Mir、Mirrorea、PrismCascade、Typed-Effect Wiring Platform、adapter/provider/view は境界を保って分離する。

直近の主線は、I1 の単一プロセス参照実装を実際に開始・検証できる最小の理論統合である。局所的 L3 countermodel や conditional lemma は、shared semantics、statement/profile、proof skeleton、I1 readiness の明確な consumer があるときだけ用いる。既存の runnable LAB は再利用可能な実験証拠であって、Gate/Phase/OBL/public completion ではない。統治上の hash、形式、report の drift は、安全性または権限境界を実際に損なわない限り、本線の意味論・実装準備を止めない。

**Owner-directed stop condition (2026-08-01):** selected semantics, the shared
kernel model, the necessary I1-readiness record, all-SCN implementation scope,
and explicit implementation authorization must be sufficient to begin I1. At
that point, this autonomous mainline stops before the first I1 implementation
package. The closeout must state the executable entry contract, guarantees,
non-guarantees, remaining later-layer boundaries, and exact implementation
inputs; it must not silently start S6.

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
| D1 read-dependent write | **C1-A-r conditional amendment hypothesis.** SCN-02 の target owner に属する read だけを含む assignment を、target owner が service 時に owned reads、pure RHS calculation、validated mutation を一つの SW1 occurrence で行う owner transition にする。現行 `[WRITE-CROSS]` の determined `v′` を literal に実装するものではなく、SCN-02 dependency rows、sampling time、read/visibility authority を含む precise Core/SCN proposal が先に必要である。別 owner operand は scope 外であり、黙って V1/R1 binding に入れない | distributed transaction、複数 owner の atomicity、retry/cache/freshness、first-class closure、cross-owner operand semantics、write-capability による暗黙の private-read authority | 同じ owner の二 attack が stale read reply + blind write になり lost update を起こす、failure 後に dependent write が出る、private operand を無権限で利用する、又は transition が一 owner を越えて atomicity を主張する |
| D2 X1 exchange | **C2-A-r recommended amendment hypothesis.** P017 X1 の named relation-state direction を出発点に、`Config` の将来候補 `X` を request occurrence `q` ごとの relation entry とする。P017 が未選択の presentation として残す owner service/result、requester receipt、one-shot consumption の occurrence/transition 形を、この候補では明示的に提案する。entry は requester/resumption locus、M1 claims と consulted-provenance refs、owner outstanding/one terminal result-or-failure、semantic receipt state、one-shot use state を持つ。dynamic result/failure、receipt、use は existing static `G_e` row ではない。`q` は source/wire/public ID ではない | final field names、transport message、global exactly-once、fairness、timeout/cancel、cross-load global identity、new causal generator/event kind | equal span/payload/queue/transport data が二 request を併合する、owner failure が mutation する、receipt swap/duplicate が use を増やす、load が use budget を reset する、又は receipt を existing `G_e` row として扱う |
| D3 occurrence / authority | successful remote write は one `ServedWrite` occurrence の named service/mutation/validated-authority facets とする。admission は、issuance が別に observable/schedulable/failing でない I1 scope では one verdict occurrence の named membership/grant/witness facets とする | write acknowledgement、separate issuance protocol、production identity | a claimed facet の causal/lineage reference が作れない、または rejection が M/G/W/S を変える |
| D4 scalar / terminal | indexed state と scalar state を別の declared state categoryにする。scalar は explicit owner/init/visibility/store slot を持つ。chain terminal は declared constant 又は declared scalar target に明示的に解決し、type default / hidden singleton key / unbound terminal を使わない | final Surface spelling、custom keyspace expansion、global default registry | `SCN-08` が hidden participant membership、type default、または未宣言 value を必要とする |

`D1` は C1 を distributed transaction に拡大せずに検討する最小 amendment
hypothesis である。SCN-02 の target-owner reads と mutation を owner seriality の
内部で一つの semantic service にできるかを、現行 `[WRITE-CROSS]` との差分として
検査する。一方、複数 owner にまたがる snapshot、retry、cache、wire-level atomicity
は導入しない。ordinary source が通信の細部を書かずに済むことと、elaborated Core が
typed owner transition/dependency provenance を明示することの両方を満たせない場合は、
C1-A-r を凍結して alternative comparison に戻る。

### D1/D2 の即時停止線と contrast trace

- **C1-A-r の scope** は target owner に属する reads だけである。別 owner operand が
  必要になった時点で S2 は semantic result を出さず停止し、target owner が requester
  になるか、nested `q` と M1 validation をどう置くか、causal edge を何にするかを
  別の precise proposal choice として提示する。元の actor へ値を返して blind write へ
  戻すことは許さない。
- **C2-A-r の static/dynamic separation** は必須である。`G_e` は static
  elaboration output のままにし、result/failure/receipt/use は future `X` transition
  specification の候補としてのみ記す。新しい event/causal generator が必要なら、それを
  隠さず ordinary Canon proposal の amendment surface とする。
- **deferred cross-owner contrast**: 将来 C1-X を試す場合は、owner A が operand を
  return した後に A 自身が更新され、target owner B が受け取った値で B-local transition
  を commit する trace を必ず含める。この trace は common snapshot、lock、rollback、
  multi-owner transaction を持たないことを確認するためのものなので、C1-A-r の保証にも
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
| 01 roll | D0, D3, publish; owner-directed write then visible publication | undeclared `VisibilityDenied` is static diagnostic |
| 02 attack | D0, C1-A-r **or** C1-B plus a selected R1 residence, M1, SW1; target-owner-local read/calculate/write contrast | missing capability has no mutation; no hidden local write; same-owner concurrent attacks distinguish C1-A-r from stale blind write; cross-owner operand is rejected/deferred rather than silently implemented |
| 03 late join | conditional A2, M1, D3 | pre-verdict / copied capref rejection |
| 04 owner leave | membership/incarnation; if a selected C2 candidate has an in-flight exchange, its restore facts | stale capref and stale rejoin rejection |
| 05 portal | two admission verdicts, target lineage, visibility | private field and wrong-target capref rejection |
| 06 shard | route failure classification | `RouteUnavailable`, no silent retry/hang |
| 07 observation | publish/observe projection | private/auth raw data absent; monotone redaction |
| 08 fallback | D4 plus monotone chain | hidden scalar/default and rollback repromotion rejection |
| 09 patch | patch pipeline/frontier and D3 no-mutation | self-grant/capability rejection, deferred frontier mismatch |
| 10 save/load | cut consistency and membership/lease state; selected C2 exchange closure only if result-dependent computation is selected | stale resurrection and incomplete channel closure rejection |

## Existing code: reuse boundary

The Full System V1 textual parser/checker, deterministic runtime report, CLI and fixture harness are valuable implementation material. They are not a Canon semantic source. Only after an ordinary selection and implementation authorization may a future implementation package retain their parser/front-end APIs, JSON report transport, deterministic harness, fixture layout, expected JSON comparison and error plumbing. It must then replace or isolate their bounded state machine behind the selected common model, and add exact Canon-SCN fixtures instead of relabelling their current matrices as conformance.

Do not pull projection, provider, renderer, actual multi-process transport, or PrismCascade into I1 kernel work. Those are later typed boundaries and would create a second semantic centre.

## Execution sequence

| Package | Authority | Deliverable and validation | Completion boundary |
| --- | --- | --- | --- |
| S1 (this packet) | A/R | goal mirror, authority cut, semantic hypotheses, SCN/OBL coverage, reuse boundary, independent planner/Oracle review | review-corrected packet distinguishes a LAB candidate from a Core/SCN amendment hypothesis; no Canon decision asserted |
| S2-A bounded comparison and amendment packet | A/R | in this existing `plan/` / `docs/reports/` LAB lane only, make the C1-A-r/C1-B and C2-A-r alternatives, adverse traces, exact current-Core/SCN deltas, non-effects, and rollback explicit | completed packet; now waits for the separable owner decisions; no new helper, schema, Lean source, runtime code, or implementation claim |
| S2-B shared kernel model | O/Canon selection, then A/R | after the selected amendment surface is authorized, construct the non-opaque `Surface/Core/Config/Step/WellFormed/Elab/SaveLoad` model and run permitted Lean/prototype checks | formal model imports the selected semantics; it does not retrofit an unselected candidate |
| S3 candidate-local statement preparation | A/R after S2-B | derive candidate-local OBL/G2/G3/G5 statement drafts and exact SCN explanation | no ledger/OBL/profile mutation or official T1 claim before ordinary acceptance |
| S4 narrow T2 + G5 | O/Canon after selected statement package | import-bearing OBL-020/021/002 skeletons plus separate save predicate, restore relation, post-state, checker, Z-cycle statements | avoids circular load theorem; official T2 close remains separate |
| S5 I1 bootstrap/readiness | A/R prepares, O/Canon accepts | P016-compatible all-SCN scope, deterministic profile hash, carrier baseline, evidence classifications, C-static wording | bootstrap/official authorization only after the bounded record exists; this plan then stops before S6 |
| S6 I1 reference implementation | implementation owner + A/R | one `mir-parse/check/elab/run` route, exact C-static then C-runtime 10/10, no waiver, carrier freeze at I1 exit | **held by owner instruction:** do not start from this autonomous mainline; resume only after the I1-entry closeout is reviewed |

The official governance lane runs beside S1--S3. It must be resolved before official lifecycle acceptance and implementation authorization, but it does not serially block S2-A's bounded comparison and owner-packet preparation.

## Research controls and reopen triggers

- Do not create more fixture-only WRKs merely to extend WRK-0045/0046. Their result is retained evidence; neither supplies a full exchange model.
- **S2-A entry guard:** its only retained locations are this Plan and its task reports; its
  read-only anchors are P012/P013/P015/P016/P017, theory 01/03/04/05/06/07,
  SCN-01..10, and ADR-0013/0014. It compares C1-A-r to a determined-value C1-B
  alternative and C2-A-r to no-receipt-state status quo; it records the listed
  falsifiers, has no Gate/Phase/OBL/implementation effect, and rolls back by
  freezing the hypothesis and issuing no proposal. It creates no helper family,
  schema, CI/Make surface, Lean source, runtime code, or new evidence lane.
- Because C1-A-r/C2-A-r touch a Core/Config/SCN boundary, a retained formal model,
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

## S2-A comparison result — C1/C2 amendment packet (2026-08-01)

### Authority and baseline reconciliation

This is LAB comparison evidence only. It was checked against theory 01/03/04/05/06,
P012/P013/P017, ADR-0014, and SCN-01..10, with independent planner and
temporary Oracle review. It changes no Canon text.

The current baseline is intentionally incomplete: `[WRITE-CROSS]` carries a
determined `v′`; `OPEN-011` leaves successful read reply/receipt open; and
`Config = <H,Q,S,M,G,W,L,P>` has no pending/result/receipt/use relation.
P012 V1/R1/SW1, P013 M1, and P017 X1 are owner-recorded directions only.

Before C1 selection, reconcile SCN-02: it has two state dependencies and
requires both dependency rows, but theory/03's worked shape displays only the
`atk` dependency. SCN-02 calls both reads cross-locus, while `[READ-CROSS]`
requires visibility/observe authority and its handler has neither visibility
declaration nor `VisibilityDenied`. The existing write capability alone does
not settle this. This is an **UNRESOLVED elaboration/authority boundary**, not
something this LAB packet may repair.

### C1: sampling and assignment semantics (owner decision required)

| Option | Semantics to select, not current fact | Required boundary |
| --- | --- | --- |
| **C1-A-r: owner-sampled atomic update** (conditional recommendation) | For one write owner `O`, every dynamic RHS state dependency is owned by `O`; all other inputs are already-determined values with explicit provenance. One actor-authorized update request carries dependency descriptions, M1 claims, spans, and authority obligations. After validation, `O` evaluates the bounded pure RHS once against one owner pre-state and mutates once as SW1. It returns no RHS result. | New Core/elaboration/SCN rule. Preserve read/visibility authority by default; write capability never silently authorizes private read/declassification. Direct and indirect other-owner dependencies reject/defer. Publication stays separate. |
| **C1-B: requester-sampled determined value** | Keep `[WRITE-CROSS](...,v′,...)`: each cross-locus RHS read is served, received through V1/R1, consumed once at the requester, then a bounded pure computation emits a later independently validated SW1 write. | Needs C2-A-r or another selected R1 residence, deterministic multi-read order/join, no dependent write after failure, and save/load closure. It has no read-modify-write atomicity: two hp=100 reads and two writes 90 may end at 90. |
| **C1-D: defer** | Retain the rule sketch only. | No proof/runtime/conformance claim may say how SCN-02 produces `v′`. |

SW1 does not select C1: it fixes validation-plus-mutation occurrence identity,
not RHS sampling. The lost-update trace distinguishes C1-A-r (`100 -> 90 ->
80`) from C1-B (`100 -> 90` possible), but frozen SCN-02 has no concurrent
expectation, so it is not a current-Canon falsifier. Owner selection must state
whether cumulative same-owner attacks are promised and whether C1-A-r preserves
read authority or opens a separately reviewed operation/declassification
authority. The safety-preserving recommendation is C1-A-r with preserved read
authority; the latter authority alternative is a separate reserved decision.

### C2: cross-locus result residence (direction recommended)

| Option | Semantics to select, not current fact | Required boundary |
| --- | --- | --- |
| **C2-A-r: candidate extension of X1 with explicit mapping** (recommend) | A future `X` relation is injectively anchored to each in-scope request occurrence `q`. Separate owner/requester projections hold M1 claims plus consulted provenance; owner outstanding or one terminal result/failure; requester receipt-pending/accepted and one-shot consumed state; restricted binding context; and restore correspondence. This candidate proposes result as an owner-service facet, receipt as a separate requester occurrence, and consumption as a zero-occurrence state transition. P017 X1 alone selects none of those presentations. | The amendment must make the static/dynamic response path explicit. Candidate choices include amending an existing static read-request row or proposing a distinct static row; P017 does not force either. Dynamic result/receipt/use are not existing `G_e` rows. This candidate would need H/cut/SaveObject/restore mapping; typed redacted observation is needed only if an exchange fact is exported. |
| **C2-D: no receipt state / defer** | Retain `Config`, `[E-SERVE]` prose, and `OPEN-011` without semantic pending/result/receipt/use state. | Honest defer only: it cannot realize V1/R1-dependent computation, one-shot use, receipt/save-load closure, or C1-B. It may not use span, payload, queue order, transport, or evaluator state as hidden correlation. |

C2-A-r extends the owner-recorded X1 direction with candidate-specific
presentation choices; it is not a concrete carrier acceptance and must not be
abbreviated as X1 itself. Do not bundle it with C1-A-r: SCN-02 under C1-A-r
needs no requester receipt for same-owner RHS reads; C1-B does. If selected,
C2-A-r would map `request -> owner service/result -> requester receipt ->
dependent use`, leave `OPEN-010` unchanged unless separately amended, and
distinguish owner failure from requester receipt rejection.

### Adverse traces and rollback

| Trace | Required result / falsifier |
| --- | --- |
| Two concurrent attacks over hp=100, atk=10 | C1-A-r serial services yield 80; C1-B may yield 90. This selects semantics; it does not retroactively falsify current Canon. |
| Write cap but no observe authority for private RHS operand | C1-A-r must preserve read/visibility authority or stop for separately named operation authority. |
| Other-owner value held indirectly in `Gamma` | C1-A-r must reject/defer it or explicitly classify it as an already-determined non-snapshot input; direct syntax inspection is insufficient. |
| Equal `q1`/`q2` span, payload, claims, queue position, or transport metadata | C2-A-r keeps distinct pending/result/receipt/use branches. Merge or receipt swap freezes it. |
| Duplicate raw delivery; owner failure then attempted use | At most one receipt/consumption; owner failure leaves store unchanged and enables no success use. |
| Save after owner success before receipt; restore consumed as unconsumed | Preserve stage and one-shot budget, or make load inadmissible. Reset, revalidation, merge, or duplicate freezes C2-A-r. |
| Result/receipt as static `G_e` or raw debug export | Immediate C2-A-r falsifier; dynamic facts need explicit mapping and observation needs theory/07 projection. |

Rollback is forward-only: freeze the candidate, issue no amendment, return to
C1-B/C1-D or C2-D, and change no Canon text.

### SCN-01..10 impact audit

| SCN | C1 impact | C2 impact | Conclusion |
| --- | --- | --- | --- |
| 01 roll | Same-owner RMW shape; C1-A-r serializes concurrent increments. C1-B needs an R1-compatible result residence before `v′`. Single `0 + 3 = 3` stays unchanged. | C1-A-r needs no requester result; C1-B brings a selected C2-style surface. Publish remains separate. | C1-only under C1-A-r; C1+C2 under C1-B. |
| 02 attack | Primary choice; baseline dependency/authority reconciliation mandatory. | Needed for C1-B, not for C1-A-r's same-owner reads. | Owner decision required. |
| 03 late join | No RHS state read in shown write; M1/SW1 lineage unchanged. | No success-result consumer shown. | No extension selected. |
| 04 owner leave | C1-A-r validates before read/mutation; C1-B may expose an earlier result before later write rejection. | In-flight X cannot revive stale epoch/incarnation/capability on load. | Failure/restore boundary. |
| 05 portal | No RMW promise; admission choreography unchanged. | Existing publish/observe may cover the stated positive path. Only a read-request whose result feeds dependent computation needs a selected R1 residence; C2-A-r is one candidate. Private key never becomes receipt. | Visibility boundary. |
| 06 two shard | Cannot bypass RouteUnavailable. | Route failure is not success/receipt. Retry, fairness, and no-hang remain separate profile concerns, not C2-A-r consequences. | Failure boundary. |
| 07 observation | Publication remains separate from service. | If an exchange result/provenance is exported, it needs a typed redacted projection; C2-A-r does not itself require export. | Conditional observation boundary. |
| 08 fallback | No fallback change. | Result use cannot re-promote a chain or bypass lease monotonicity. | Preserve theory/06. |
| 09 patch | No patch change. | No hidden exchange authority or side channel. | Preserve no-mutation. |
| 10 save/load | Queued C1-A-r update is in request/in-flight closure; service is unsplittable. | X stages require SaveObject/cut closure; restore cannot merge/reset/resurrect. | C2-A-r needs ordinary amendment. |

No OBL or `theory/11` wording changes follow. A later ordinary amendment must
review THM-001 / OBL-001--004, OBL-020--021, THM-003 / OBL-010--014,
THM-004 / OBL-015--016, and the observation boundary without pre-claiming a
ledger result.

### Owner packet and stop line

Prepare three separable owner sections: (1) reconcile SCN-02's two dependencies
and their authority/failure-row treatment; (2) choose C1-A-r, C1-B, or defer,
and separately decide any operation/declassification authority; (3) choose
C2-A-r or defer with its H, static request-response, failure, cut, SaveObject,
and restore mapping. Only after these decisions and a frozen ordinary amendment
may S2-B create a model or prototype. Any weakening of read authority, new
Core/occurrence/causal/`G_e` primitive, failure/OPEN-010 change, Config or
SaveObject change, theory/11 change, or I1 claim stops for the full Canon
process.

## Current recommendation

**S2-A is complete.** Its smallest correct next step is not a new model or a
prototype: first use the three-section owner packet to reconcile SCN-02, choose
`C1-A-r` / `C1-B` / defer, and choose `C2-A-r` / defer. The safety-preserving
working recommendation is `C1-A-r` with existing read/visibility authority
preserved, plus the `C2-A-r` amendment direction. That is still a recommendation,
not a Canon selection. Only after the ordinary selection is frozen may S2-B
formalize or prototype the selected common model.

## Non-claims

This plan does not change the official `T0` status, repair the governance profile, exit a Gate, close an OBL, establish C-static/C-runtime conformance, authorize I1 implementation, or claim public/product completion. It does not freeze final syntax, carrier field names, wire format, public API, identity provider, transport, persistence, renderer, or View.
