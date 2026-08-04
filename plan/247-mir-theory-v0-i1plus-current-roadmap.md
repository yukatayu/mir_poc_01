# Plan 247 - Mir Theory v0 / I1+ current execution roadmap

最終更新: 2026-08-04 10:36 JST

## 役割と authority

これは ADR-0015 / PROPOSAL-018 で owner が承認した bounded program

```text
Mir Theory v0 + Mir I1+ deterministic reference system
```

の **唯一の current execution roadmap** である。規範正本は
`mirrorea_canon/`、公式 Gate / Phase の実装状態は
`mirrorea_canon/plan/01-phases.md`、proof status は
`mirrorea_canon/theory/11-metatheory-ledger.md` だけが決める。この文書は LAB の
実行順・依存・受入条件を保持するが、単独では Canon、Gate、Phase、SCN、OBL、
conformance 又は public status を変更しない。

- program start revision:
  `b9dcaa054c548112a7977776723418559b8ba8b2`
- completed milestone: **M2 semantic-assertion T0/G0 closeout**
- active milestone: **M3 evaluation/materialization calculus**
- next milestone: **M4 maintained relation/late projection**
- active frontier limit: 一つの milestone
- current direct blocker: M3 の shared evaluation/materialization calculus を、M1
  Constitution と accepted T1 entry に沿って定義し、same-owner RMW と cross-owner
  diagnostic の positive/negative evidence を閉じること
- direct consumer: M4 maintained relation/late projection semantics

Plans 196 / 197 / 246 と、それ以前の numbered plans は削除しない。これらは
historical LAB evidence / repository memory であり、Plan 247 と並行する active queue
ではない。`progress.md`、`tasks.md`、`docs/project-status.md`、
`Documentation.md` はこの roadmap の派生 snapshot である。

## M0 start-state pin と non-effects

M0 は repository / agent / governance bootstrap だけを扱う。開始時の公式状態は次の
とおりであり、M0 close でも別の exact acceptance record がない限り変えない。

| 項目 | M0 start state | M0 の扱い |
| --- | --- | --- |
| official lifecycle | `T0`; G0 exit / T1 entry なし | unchanged |
| proof ledger | OBL-001..028 はすべて `open` | unchanged |
| SCN | SCN-01..10 は frozen conformance definition | expectation / status unchanged |
| conformance | C-static / C-runtime / C-distributed の official pass claim なし | unchanged |
| implementation | bounded LAB evidence のみ | production / I1 work を開始しない |
| public surface | final grammar / API / ABI / wire / product は未確定 | freeze しない |

したがって、M0 の成功は T0/G0 closeout、proof、SCN conformance、I1 authorization、
又は final-public completion を意味しない。semantic Constitution は M1 の direct
consumer であり、M0 で materialize 済みとは扱わない。

## 共通 milestone close contract

各 milestone は、該当する範囲について次を一つの integration unit として閉じる。
該当しない項目は report で理由を明示して `not applicable` とする。

1. authoritative Canon rule 又は、この milestone が規範変更を行わないという明示的
   non-effect。
2. executable reference behavior と direct consumer。実装前 milestone では、実行可能な
   formal model / checker / bounded evaluator を reference behavior としてよい。
3. 少なくとも一つの positive case と一つの negative case / falsifier。
4. formal evidence を `lean-proved`、`lean-stated`、
   `model-checked-bounded`、`runtime-monitored`、`intentionally-deferred` のいずれかで
   正確に分類する。分類変更は actual evidence と ledger correspondence を要する。
5. author と別の independent reviewer による falsification review。
6. focused validation、Canon index / source hierarchy / docs validation、変更層の回帰確認。
7. 原則一つの milestone report と、Plan 247 / derived snapshots の同期。
8. integration commit、push、`HEAD == origin/main` の remote parity。

一つでも満たさなければ milestone は active のままであり、次の semantic milestone へ
進まない。失敗 evidence は同じ milestone に保持し、成功へ読み替えない。

## 固定 execution order

順番は owner direction で固定されている。milestone の追加、入替え、又は並行する
semantic frontier は作らない。

```text
M0 Bootstrap
 -> M1 Constitution
 -> M2 semantic-assertion T0/G0 closeout
 -> M3 evaluation/materialization
 -> M4 maintained relation/late projection
 -> M5 shared formal model/metatheory
 -> M6 Surface
 -> M7 checker/elaborator
 -> M8 deterministic runtime
 -> M9 auth/verification
 -> M10 conformance/closeout
```

## Milestone acceptance map

### M0 - Bootstrap (completed 2026-08-03)

**Intended outcome:** ADR-0015 program を再現可能に実行する repository / agent /
governance floor を作り、Plan 247 を一つだけ current queue として指定する。

**Dependencies:** owner-approved PROPOSAL-018 / ADR-0015 と start revision
`b9dcaa054c548112a7977776723418559b8ba8b2`。

**Acceptance criteria:**

- owner direction、bounded authority、owner-reserved conditions、non-effects が Canon の
  通常手続と index に反映され、`canon > LAB` が維持される。
- agent configuration は role / sandbox / instruction contract を機械検証できる。
- Plan 247 が sole current roadmap として Canon operating model、`plan/00-index.md`、
  derived snapshots から一意に参照される。Plans 196 / 197 / 246 は historical LAB
  memory として残る。
- milestone report、single-writer、independent review、validation、commit / push の
  operating contract が相互に矛盾しない。
- `python3 scripts/validate_agent_configs.py`、`make docs`、focused tests、
  `git diff --check` と independent review が fresh に完了する。
- M0 report が start / dirty state、変更、commands、evidence、skipped validation、
  snapshot status、review、commit / push / sub-agent close を記録する。
- 上の M0 start-state pin は変わらない。

**Ownership:** Canon / integration / commit / push は parent-orchestrator、agent config は
delegated config writer、Plan 247 と派生 planning/status は planner、tests は test author、
review は同じ変更を書いていない independent reviewer。

**Close evidence:** payload commit `be5928a168fd519c05867fba2746ddd833a3bde5` was
pushed after fresh validation and independent review. The closeout report/snapshot commit
records the final remote-parity check without changing M0's pinned official state.

**Direct consumer:** M1 Constitution input bundle。

### M1 - Constitution (completed 2026-08-04)

**Intended outcome:** North Star と preserved invariants を短い、矛盾のない Mir Theory v0
Constitution に集約し、後続 milestone の decision priority と禁止事項を固定する。

**Acceptance criteria:**

- Constitution の Canon path / id を milestone 内で一意に決めて index へ登録する。
- evaluation locus、authority origin、trigger、semantic form、materialization、failure、
  observation、evolution を混同しない原則を明示する。
- communication は checked meaning の projection、domain vocabulary は Core primitive で
  ない、stdio は typed adapter boundary、authentication は transport でない、
  visualization は typed information release であることを維持する。
- v0/I1+ scope、guarantee、deferred scope、decision priority、falsifier、rollback を
  positive / adverse examples と independent review で検査する。
- M2 が参照できる semantic-assertion requirements と acceptance vocabulary を出力する。

**Evidence:** `NORTH-STAR.md`、ADRs、theory/spec/scenarios の contradiction audit、
M1 report、Canon validation。`root/design-constitution` (`DESIGN-CONSTITUTION.md`) と
ADR-0016 が canonical decision filter を採用した。SCN-02 の owner-side RMW、semantic /
presentation fallback、pre-M6 grammar status を同じ cut で整合させ、独立 review と
one correction cycle を pass し、payload `aa0771ecdec4a7cec8f9f454dcbb455025ede8dc` を
push した。

**Direct blocker / consumer:** M0 accepted governance cut / M2 T0/G0 closeout。

### M2 - Semantic-assertion T0/G0 closeout (completed 2026-08-04)

**Intended outcome:** M1 Constitution に対して意味のある T0/G0 assertion とその evidence
binding を定義・評価・受理し、公式 T0/G0 を一度だけ閉じる。

**Acceptance criteria:**

- semantic assertion の subject、source revision、required evidence、result vocabulary、
  digest / identity、re-evaluation policy、program-defined acceptance record を Canon に定義する。
- historical v1/v2 artifact を上書き、再解釈、又は current pass に昇格しない。
- fresh artifact / evaluation が exact contract に従い、negative control が失敗を検出する。
- profile result と Gate / Phase acceptance record を分離する。ADR-0015 scope 内では
  required evidence が実在するとき agent がその record を更新できる。
- exact acceptance record が成立した場合にだけ G0 exit / T1 entry を記録し、
  `plan/01-phases` と derived status を同期する。
- SCN conformance / proof / I1 authorization でないことを artifact と report に保持する。

**Evidence:** `mirrorea_canon/plan/04-t0-g0-semantic-assertion-profile.md`、ADR-0017、
`plan/248-t0-g0-semantic-assertion-v3-evaluation.json` の Git-blob / canonicalization /
digest checks、negative controls、independent review。revision
`644ec1cdfa7d69600af3463ab60a6b7d745913c8` の reproduced `pass` digest
`b32bd2c87e1dc77ca2a4f7a7426cda0bff8bcbf80155d19addd7db3a8288aa23` を受理し、
G0-D3 → G0 exit → T1 entry を適用した。v1/v2 は historical evidence のままである。

**Direct blocker / consumer:** completed; the direct consumer is M3 evaluation-materialization calculus。

### M3 - Evaluation / materialization

**Intended outcome:** source-level meaning の semantic form、evaluation site、trigger /
clock、authority origin、materialization を直交して保持する deterministic calculus と
executable reference を閉じる。

**Acceptance criteria:**

- semantic form (`value | state | relation | computation`)、evaluation site
  (`owner | locus | designated evaluator | consumer | provider`)、trigger/clock、
  authority origin、materialization policy を同じ typed judgment に定義する。
- owner-side mutation、designated evaluator の frontier/versioned materialization、
  consumer-local relation evaluation、provider-side high-rate evaluationを、positive and
  adverse examples で区別する。inference は同じ input に対して決定的で、曖昧なら
  最小 annotation を要求する Diagnostic を返す。
- same-owner mutable read-dependent write は owner service の一つの bounded transition
  として評価し、二つの attack が `100 → 90 → 80` となることを実証する。blind stale
  write / lost update は falsifier とする。
- cross-owner operand は explicit remote result/receipt path へ elaborate するか v0
  Diagnostic とする。hidden transaction、hidden snapshot、requester-side private read、
  authority inference を導入しない。
- evaluation site と authority origin の分離、evaluation choice の Core/trace 可視性、
  explicit effect/failure/no-mutation を Canon/Lean/bounded model/reference behavior に
  対応付け、実証 class を正確に記録する。

**Evidence:** theory/01--03、SCN-01/02、historical LAB Plans 196/246 と countermodels、
focused model / executable tests、independent review。

**Direct blocker / consumer:** M2 close / M4 relation calculus。

### M4 - Maintained relation / late projection

**Intended outcome:** maintained relation、guarded reference、fallback、relation DAG と
consumer-local late projection を同じ semantic model に置き、value stream へ早期具体化
しない reference behavior を閉じる。

**Acceptance criteria:**

- `B owns bird; bird follows A.shoulder; fallback B.shoulder; C renders A,B,bird`
  を Surface → Core → runtime → projection で通す。relation owner は B/authoritative
  owner、C は semantic owner ではない。C には bird absolute-pose stream を要求せず、A/B
  と同じ presentation sample/contextから local evaluator がbird poseを導出する。
- guarded reference、relation composition、activation frontier、derived observation policy、
  relation save/load state と `project then evaluate ≃ evaluate then project` の有限 fragment
  を定義する。relative-offset coherence を positive case、split-frame/stale-anchor use を
  typed rejection/violation の negative case とする。
- semantic fallback（existence/membership/incarnation/lease/authority loss）は occurrence /
  lineage/frontier に反映し、option index は単調。以前の anchor への復帰は fresh
  witness/epoch を伴う explicit reacquire のみとする。短い packet不足や latency は
  consumer-local presentation fallback であり semantic lineage を変更しない。
- anchor visibility/redaction を弱めない derived label、no stale sample、no hidden
  communication/authority/value materializationを検査する。final wire/API/ABIは固定しない。

**Evidence:** M3 calculus、theory/04--07、existing relation/fallback evidence、new bird
scenario and finite projection model、independent review。P017/WRK evidence は historical
LAB evidence であり、新しい Canon relation の代用にしない。

**Direct blocker / consumer:** M3 materialization semantics / M5 shared model。

### M5 - Shared formal model / metatheory

**Intended outcome:** M1--M4 を一つの non-opaque proof-facing model に統合し、Surface、
checker、runtime が共有する semantic source を固定する。

**Acceptance criteria:**

- Surface fragment、Core、Config、Step、WellFormed、elaboration、history、relation state、
  cut、SaveObject / restore、authority、observation の interface が同じ model 上にある。
- statement / theorem / implementation target の identity と coverage matrix がある。
- positive trace、adverse trace、bounded model check、Lean compilation、axiom / placeholder
  scan を実行し、ledger movement は evidence が満たす行だけに限定する。
- T1 / narrow T2 / separate I1-readiness の exact relationship と required acceptance records を
  Canon に整合させ、M6/M7 が着手可能な scope を明示する。
- opaque predicate、`True` stub、bounded enumeration を general proof に読み替えない。

**Evidence:** theory/11 ledger、Gate / Phase sources、M3/M4 artifacts、Lean / model checks、
independent review。

**Direct blocker / consumer:** accepted M3/M4 semantics / M6 Surface。

### M6 - Surface

**Intended outcome:** shared model へ total に elaboration できる bounded Surface v0 と
diagnostic contract を定義する。

**Acceptance criteria:**

- grammar、name / locus / index / chain / effect / failure syntax、accepted source domain、
  Surface-to-Core relation、source spans を Canon に固定する。
- every accepted form が Core 又は explicit Diagnostic へ分類され、hidden default / scalar /
  authority / communication を導入しない。
- SCN-01..10 の positive / negative source corpus と expected static outputs を frozen model に
  bind する。
- final public grammar / API を不可逆に固定せず、I1 reference scope と後段 extension point を
  区別する。

**Evidence:** spec/02--04、SCN sources、historical Surface LAB evidence、grammar and
elaboration fixtures、independent review。

**Direct blocker / consumer:** M5 model/readiness / M7 checker-elaborator。

### M7 - Checker / elaborator

**Intended outcome:** M6 Surface を parse / check / elaborate し、M5 model と一致する
deterministic reference toolchain を作る。

**Acceptance criteria:**

- one source-first `parse -> check -> elaborate` route があり、internal bypass を conformance
  route に使わない。
- C-static SCN-01..10 が waiver なし 10/10 で、negative は expected id / span / reason を返す。
- generated Core、communication / authority / effect / failure obligations、source traceability が
  inspectable で、hidden edge を持たない。
- elaborator determinism と explanation claims を shared model / ledger target に対応付け、
  actual evidence class だけを記録する。

**Evidence:** executable tests、goldens、fresh C-static artifact、formal correspondence、
independent review。

**Direct blocker / consumer:** M6 Surface / M8 deterministic runtime。

### M8 - Deterministic runtime

**Intended outcome:** M7 output を M5 semantics に従って実行する single-process
deterministic reference runtime を閉じる。

**Acceptance criteria:**

- explicit state / effect / failure / witness / history / cut trace を持つ `run` route がある。
- C-runtime SCN-01..10 が waiver なし 10/10 で、replay は frozen profile に対して
  deterministic である。
- rejection-no-mutation、owner seriality、no stale resurrection、redaction、patch frontier を
  negative / recovery tests で検査する。
- helper-local output や evaluator side table を semantic carrier / public ABI にしない。
- I1 carrier freeze を Canon の accepted reference scope に限定し、final public / wire contract
  と混同しない。

**Evidence:** runtime traces、state/failure assertions、replay hashes、regression tests、
model correspondence、independent review。

**Direct blocker / consumer:** M7 C-static toolchain / M9 typed auth-verification extension。

### M9 - Auth / verification

**Intended outcome:** deterministic reference system に typed authority / authentication /
verification extensions を加え、transport と観測から分離して検証する。

**Acceptance criteria:**

- authentication、authorization、membership、capability、witness、admission、revocation を別の
  typed facts / relations として保持する。
- transport/session/locus/name を authority として採用せず、live validation context、lineage、
  epoch/incarnation、visibility/redaction を検査する。
- static checking、bounded model checking、proof の三 assurance line を別々に報告する。
- stale/copied/replayed grant、wrong-target use、unauthorized observation、rejected patch mutation
  の negative evidence がある。
- M8 deterministic and SCN behavior を regression し、verification layer が hidden effect 又は
  untyped debug leak を作らない。

**Evidence:** auth/verification tests、formal statements、bounded models、runtime traces、
security-focused independent review。

**Direct blocker / consumer:** M8 runtime / M10 conformance closeout。

### M10 - Conformance / closeout

**Intended outcome:** Mir Theory v0 + Mir I1+ deterministic reference system の evidence、
claims、non-claims、reproduction route を一つの closeout cut に固定する。

**Acceptance criteria:**

- M1--M9 の normative rules、artifacts、reports、proof classifications、open obligations が
  traceable で、stale current references がない。
- C-static / C-runtime は SCN-01..10 waiver なし 10/10 の fresh evidence を持つ。
- claimed theorem は trusted compilation / axiom scan / implementation correspondence を満たし、
  remaining rows は正確に `open` 又は delegated evidence class を保つ。
- clean-clone reproduction、docs、samples / commands、diagnostics、status snapshots、release
  checks、independent adversarial review が一致する。
- repo-local v0/I1+ completion と final public product / production deployment / C-distributed /
  I2--I6 completion を明示的に分離する。
- program report、commit / push / remote parity を閉じ、次の current roadmap は新しい owner
  direction があるまで作らない。

**Evidence:** full focused validation matrix、fresh conformance artifacts、proof audit、clean-clone
reproduction、final independent review。

**Direct blocker / consumer:** M9 acceptance / owner-defined post-program direction。

## Milestone dependency / ownership summary

| Milestone | Primary ownership | Normative input | Required validation emphasis |
| --- | --- | --- | --- |
| M0 | orchestrator + config/test/planner writers | ADR-0015 program grant | config, Canon index, hierarchy, docs, review |
| M1 | theory/governance writer | North Star + preserved invariants | contradiction and scope/falsifier audit |
| M2 | governance/evidence writer | accepted Constitution | exact artifact, digest, negative control, exit record |
| M3 | semantics + formal/test writers | Constitution + T1 entry | evaluation/materialization model and adverse traces |
| M4 | semantics + formal/test writers | M3 calculus | relation/projection preservation and anti-collapse tests |
| M5 | formalization writer | M3/M4 accepted rules | shared-model coverage, Lean/model checks, lifecycle mapping |
| M6 | language/spec writer | M5 shared model | grammar/elaboration totality and SCN source matrix |
| M7 | implementer + test/formal writers | M6 Surface | C-static 10/10 and correspondence |
| M8 | implementer + test/formal writers | M7 Core output | C-runtime 10/10, deterministic replay, safety regressions |
| M9 | auth/security + test/formal writers | M8 runtime | lineage/validation/redaction negative evidence |
| M10 | orchestrator + independent reviewer | M1--M9 accepted cuts | fresh full matrix and clean-clone closeout |

同一変更の writer はその milestone の independent reviewer を兼ねない。production Rust は
原則一 writer、test / planner / review は別 ownership とする。

## Risks, assumptions, and decision checkpoints

### Assumptions

- owner が milestone の名前と順番を承認したことは ADR-0015 / PROPOSAL-018 に記録済み。
- future milestone の internal carriers / algorithms / provisional syntax は、Constitution と
  acceptance criteria を満たす範囲で evidence-gated に選べる。
- M0 より詳しい Constitution 内容や semantic rules はまだ accepted fact ではなく、該当
  milestone で falsifiable に選ぶ。

### Main risks and mitigations

| Risk | Trigger | Mitigation / rollback |
| --- | --- | --- |
| multiple current queues | older plan が「current next」を名乗る | Plan 247 だけを queue とし、older plan は historical label |
| semantic work leaks forward | current milestone 未closeで後段 model/code開始 | active frontier=1; changes stay unintegrated or rollback |
| proof/conformance laundering | bounded/placeholder evidence を proof/pass 扱い | exact evidence classification, axiom/placeholder scans |
| hidden authority/effect/projection | transport/debug/evaluator state が意味を保持 | typed carrier, negative tests, late projection review |
| lifecycle overclaim | M0/M1 evidence から T0/G0/T1 を推測 | only M2 exact acceptance record may move T0/G0 |
| premature public freeze | M6--M9 reference representation が public contract 化 | explicitly reference-only; owner-reserved stop before irreversible freeze |
| shared-worktree overwrite | concurrent writer surface が衝突 | single-writer ownership, focused diff, preserve unrelated edits |
| stale status/report growth | snapshot と historical memory が queue 化 | one report per milestone, derived snapshot recut at close |

### Owner-reserved stop conditions

次のいずれかに到達した場合だけ owner input を待つ。その他の milestone-local choice は
ADR-0015 の evidence gate と independent review で進める。

- North Star の変更。
- safety / authority / privacy / redaction / no-stale-resurrection guarantee の弱化。
- World / Avatar 等の domain vocabulary の Core promotion。
- v0 non-goal の mandatory scope 化。
- final public API / ABI / wire format の不可逆な固定。
- production deployment / external publication。
- project priorities で順序付けられない同順位・不可逆な選択肢。
- current user data / secret の破壊又は公開リスク。

checkpoint では `escalated` bundle に exact choice、consumer、evidence、falsifier、
alternatives、rollback、影響する milestone を記録する。単なる難しさ、validation failure、
又は未完成は owner-reserved stop ではなく、active milestone 内で修正する。

## Deferred scope

次は M0--M10 の current target に含めず、M10 でも completion claim をしない。

- final public grammar / API / ABI / wire compatibility。
- production deployment、external publication、一般公開 product。
- C-distributed、real multi-process transport、I2--I6、WAN federation、distributed durable
  persistence / exactly-once / distributed transaction guarantee。
- performance / scale guarantee、single final backend、single viewer / renderer / engine。
- PrismCascade、Typed-Effect Wiring Platform、Mirrorea fabric、upper application の統合完了。
- owner-authenticated L2 trust anchor の構築。ADR-0014 program-outside route は別管理。

これらの bounded LAB evidence は regression / design evidence として再利用できるが、
active milestone 又は public completion へ暗黙昇格しない。

## Recommended next action

M3 の evaluation/materialization calculus を、M1 Constitution と accepted T1 entry から
定義する。まず evaluation site、trigger/clock、authority origin、materialization を直交する
shared judgment に置き、same-owner RMW の `100 → 90 → 80` positive case と cross-owner
hidden transaction rejection を同じ reference evidence で閉じる。M3 report、independent
review、focused validation、commit/push/remote parity が閉じるまで M4 は開始しない。
