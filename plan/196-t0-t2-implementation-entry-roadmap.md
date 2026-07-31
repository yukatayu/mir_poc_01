# Plan 196 - T0 から T2 / I1 入口までの条件付き自走ロードマップ

## 役割と権限

これは、現在の Canon と LAB 証拠から導いた **LAB repository memory** である。
規範正本は `mirrorea_canon/` であり、この計画は Gate / Phase / SCN / OBL /
proof status / Core / contract を変更しない。owner の判断が必要な箇所を agent が
選んだことにもならない。

## Current disposition note (2026-07-31)

This roadmap preserves its original conditional comparison for repository
memory. Its earlier LAB recommendation to integrate T2 and I1 readiness at one
checkpoint is superseded by the owner disposition in `PROPOSAL-016`: **narrow
T2**, followed by a separately accepted I1-readiness/bootstrap record, with an
explicit bootstrap before C-static formal I1 entry. The actual profiles,
evidence-class mapping, wording reconciliation, and authorization remain
future ordinary Canon work. This note changes no Canon criterion or lifecycle
state.

目的は、次の二つを混同せずに T2 までの実行順を固定することである。

1. ADR-0014 の範囲で agent が自律的に進められる研究・検証。
2. 公式 Gate / Phase を成立させ、I1 実装へ入るための owner / Canon action。

## 結論

> 2026-07-28 status update: O0 was applied through profile v2 and its single
> fresh artifact. The artifact is a valid `fail` on unchanged fixed controls,
> so this roadmap's former C0/C1 repair route is closed without opening G0-D3
> or official T1. A control rebase or another artifact now requires a new owner
> / Canon decision; the rest of this historical conditional roadmap is unchanged.

**現在の source cut から official T2 exit までを、user input なしで連続自走することは
できない。** 現在は official `T0` で、全 OBL は `open`、かつ直前の autonomous
rescreen は当時の delta から新しい L3 package を選んでいない。これはその source cut
での優先順位判断であり、ADR-0014 の standing eligibility を狭める規範ではない。
即時の official lifecycle 停止理由は Lean や実装能力ではなく、owner-reserved な
exact-contract / semantic / lifecycle 境界である。

owner 判断前でも、既存 Canon の literal transcription 又は既存 lane の conditional
lemma として、意味論を選ばない保守的な signature / statement package が
ADR-0014 の五条件を満たすかを再審査できる。合格する candidate だけは L3 LAB
evidence として自走できるが、Gate criterion、OBL identity/status、Core relation を
補ってはならず、official T1/T2 を進めない。

owner が必要な boundary を明示的に disposition した後は、次の owner checkpoint
までの比較、反例探索、形式モデル、Lean statement / proof skeleton、SCN
traceability、bounded implementation validation、独立レビュー、検証、report、
commit / push を package 単位で進められる。ただし各 package は個別に ADR-0014 の
standing predicate、existing-lane、pre-registration、falsifier、non-effects、
rollback 条件を満たす範囲だけ自走できる。Canon integration、ledger movement、
production implementation、新 lane/helper は owner action のままである。したがって
採るべき運用は **少数の owner checkpoint で区切った長い research ratchet** である。

> 2026-07-28 update: P004/P008/P012/P013/P015/P016 の方向は owner が記録した。
> ただし shared model の前に C0--C7 の composition/falsifier 研究が必要である。
> 詳細は Plan 199。これは Canon rule、Gate/Phase、実装認可を変更しない。

## 現在確認できる事実

| 項目 | 現在の状態 | 意味 |
| --- | --- | --- |
| official lifecycle | `T0`; G0 exit / T1 entry record なし | 後段 LAB evidence は lifecycle を進めない |
| T0 profile | v2 adopted; sole fresh artifact is valid `fail` on fixed-control drift | `plan/155` is nonconforming history; `plan/198` cannot support G0-D3 |
| proof ledger | OBL-001..028 が全件 `open` | Lean compile や countermodel は proof status ではない |
| autonomous frontier | current source cut では新 WRK なし | frozen record の修理や重複実験で進捗を作らない |
| common formal model | Canon-aligned Core / Config / Step / WellFormed / elaboration relation なし | opaque LAB draft は exit artifact にならない |
| phase profiles | T0 profile のみ存在 | T1 / T2 exit の機械可読 profile が未定義 |
| delegated L2 | owner-authenticated trust anchor 未構成で fail-closed | L3 research は可能だが agent-managed L2 promotion は不可 |
| implementation | runnable bounded LAB evidence あり | C-static / C-runtime / C-distributed や official I1 ではない |

## T2 と「理論が固まり I1 に入れる」の差

Canon の現在の T2 exit 表記は次に限られる。

- OBL-020 / OBL-021 / OBL-002 の proof skeleton。
- G5 statement 群。

この条件だけでは、user が意図する「理論部分を固めて実装に入る入口」を自動的には
保証しない。少なくとも次の対応が未記載である。

- `proof skeleton` と `theory/11` の status literal の対応。
- G1 の「OBL-020/021 完了」と、T2 での proof skeleton の時間順。
- G2 が OBL-005..008 を「statement」と呼ぶ一方、OBL-008 は proof である点。
- G4 / G6 / G7 と I1 entry の関係。
- 有限 fragment の constructivity boundary を担う OBL-003 と、cut/save-load の
  attribution boundary を担う OBL-027 を、I1 前にどの evidence class で要求するか。
- T1 / T2 の exact phase-governance profile。
- I1 が必要とする全 SCN の static / runtime expectation と、T2 で固定する theory
  interface の対応。
- T0-T2 の implementation freeze と、I1 entry に必要な parser/checker をいつ
  actualize するか。

したがって、T2 exit を I1 entry authorization と結びつける場合、owner は T2 close
前に次のどちらかを明示する必要がある。狭い T2 を独立に閉じる場合、I1-entry
readiness は T2 後の別 profile / acceptance に残してよい。

1. **狭い Canon T2**: 現行表の proof skeleton / G5 statement checkpoint として閉じ、
   I1-entry readiness は別の後続 profile で判定する。
2. **T2 = I1-entry readiness**: 現行 T2 profile に、I1 が実装を開始しても Core /
   SCN / proof interface の手戻りを抑えられる追加条件を明示する。全十 SCN に必要な
   G0-G7 の statement-level criterion を閉じるか、I1 の対象 SCN / fragment を明示的に
   狭める。

この計画が元の source cut で user の表現に合わせていた 2 の推奨は、後に記録された
P016 の narrow T2 + separate I1-readiness disposition により superseded である。
現在の正本読解では、T2 profile と I1-readiness/bootstrap record を別々に扱う。
これは profile の内容・受理・実装認可を決めるものではない。

## 依存 DAG

`O` は owner / Canon action、`A` は autonomous LAB package、`R` は独立レビューを
表す。

```text
official T0
  |
  +--> A0 [A/R] conservative signature / statement eligibility preflight
  |        literal transcription 又は conditional lemma のみ
  |        reserved relation に達した candidate は採択せず escalation
  |
  v
C0 [O] T0 profile の success literal と artifact continuity を修正
  |
  v
C1 [A/R] 修正版 profile に対する fresh exact evaluation
  |
  v
C2 [O] exact digest を受理する G0/T0 exit record
  |
  v
official T1 entry

並行して準備可能だが、Canon integration は owner disposition 後:

D1 [O] recorded: P008 totality + P012 V/R/S/A + P013 request-validation context
D2 [O] recorded: P004 / return / SCN-08 の closure direction
D3 [O] recorded direction: narrow T2 / separate I1 readiness / bootstrap-C-static
P009-A は記録済み
          \                |                /
           +---------------+---------------+
                           |
                           v
M0 [A/R] composition and inference-boundary research (Plan 199 C0--C7)
                           |
                           v
M1 [A/R -> O] shared formal model
   Core / outcome / value flow / request validation / occurrence identity
   Config / Step / WellFormed / frame-freshness / history
                           |
          +----------------+----------------+
          |                                 |
          v                                 v
G1/G2/G3 statement packages          shared save/load/checkpoint model
          |                                 |
          v                                 v
T1 profile + human acceptance        T2 skeletons + G5 statements
          |                                 |
          +----------------+----------------+
                           |
                           v
T2 profile + human acceptance
                           |
                           v
official T2 exit
                           |
                           v
              [O] T2 と I1 authorization の関係
                    /                    \
      integrated criteria accepted       narrow T2
                  |                          |
                  v                          v
          I1 entry authorization     separate I1 readiness
                                     profile + owner acceptance
```

Canon の `depends_on` は knowledge dependency であり循環を許す。上図は
execution dependency であり、`INDEX.json` の dependency graph をそのまま実行順と
解釈していない。

## Package 計画

| Package | 主担当 | 産物 | close 条件 | 次の停止線 |
| --- | --- | --- | --- | --- |
| P0 現在地監査 | A/R | 本計画、blocker 分類、current evidence cut | Canon/LAB source check と独立 review が一致 | owner checkpoint 1 / P0A |
| P0A conservative statement preflight | A/R | 既存 Canon literal だけで閉じる signature / statement 候補の eligibility matrix | ADR-0014 五条件、非重複、既存 lane、明示 falsifier、reserved-boundary exclusion が全て通る候補だけを選ぶ | candidate がなければ owner checkpoint 1 |
| P1 T0 profile repair design | A prepares, O decides | `pass` に統一する profile revision と旧 artifact の扱い | proposal、代替、互換性、non-claims、移行規則を owner が disposition | Canon edit 前 |
| P2 T0 fresh evaluation | A/R | 修正版 exact contract に従う一回限りの artifact | exact Git blob、順序、cardinality、RFC 8785 digest、3 check、non-claims が一致 | G0-D3 |
| P3 lifecycle contract | LAB preparation complete; O/Canon decides | P016, Plan 196, and Plan 197 already identify the Gate/ledger mapping, T1/T2 profile, proof-skeleton evidence, and T2-I1 boundary that the ordinary Canon package must bind | no separate autonomous LAB preparation remains without a new source delta; the future Canon package fixes artifact identity, status literal, and human acceptance | ordinary Canon lifecycle action |
| P4 semantic directions | A/O recorded; follow-up is A/R | P004/008/012/013/015/016 の bounded direction | direction と non-effects を proposal に記録。具体的 rule は未改訂 | composition research 前 |
| P4A composition/inference boundary | A/R | Plan 199 C0--C7 source anchors、countermodels、safe omission matrix | shared carrier が hidden identity/authority/default を要さず、停止条件が明示される | common model 前 |
| P5 shared formal model | A/R, O integrates | Canon-aligned formal domains / relations | opaque placeholder でなく、P4A の正負例、coverage、non-effects、source anchors が揃う | ledger movement |
| P6 G1 package | A/R, O accepts | OBL-001 / 020 / 021 exact statement、SCN-01/02 explanation | Core-write coverage、global-step coverage、outcome/equality が明示 | G1 exit |
| P7 G2/G3 package | A/R, O accepts | OBL-005..007 / 015 statement と OBL-008 owner-defined proof/status package、SCN-03/04/08 explanation | P3 が OBL-008 の required status/artifact/acceptance を固定し、chain/lineage/reacquire と mutation/grant relation が同一 model に束縛 | T1 exit |
| P8 T1 close | A prepares, O accepts | SCN finalization record、T1 profile artifact、exit record | exact statuses、profile pass、human acceptance | official T2 entry |
| P9 T2 skeletons | A/R, O accepts status | OBL-020 / 021 / 002 proof skeleton | import-bearing Lean、明示 assumptions、case coverage、adverse evidence | ledger movement |
| P10 G5 package | A/R, O accepts | OBL-009..014 statements、SCN-10 explanation | saved-object predicate、restore relation、live-state postcondition、checker、Z-cycle domain を分離 | G5 statement acceptance |
| P11 I1-entry audit | A/R | all-SCN / all-Core / G0-G7 interface readiness matrix | OBL-003 / 027 を含む unresolved item を I1-blocking / later / implementation choice に全分類。integrated route では T2 前、narrow route では T2 後に置ける | owner-defined T2-I1 relation |
| P12 T2 close | A prepares, O accepts | T2 profile artifact、accepted evidence cut、exit record | exact profile pass、human acceptance。I1 authorization は別の owner-defined relation に従う | separate I1 readiness / authorization |

## Immediate owner checkpoint 1

### 1. T0 profile revision

現在の v1 artifact は、矛盾した v1 source blob を自ら bind している。新しい revision
で source text だけを `pass` に直しても、旧 artifact の `source_revision` /
`source_sha256` は自動で修復されない。また ADR-0013 は一個の artifact route を固定する。

LAB recommendation は次である。

- v1 artifact は履歴証拠として保持し、fully conforming と再分類しない。
- `phase-governance/t0-g0` v2 を定義し、root / check result を `pass` /
  `pending` / `fail` に統一する。
- v2 に対して一回限りの新 artifact を明示的に許可する。
- v2 artifact の生成・検証と G0-D3 acceptance を別 action に保つ。

「v1 の単純 corrigendum」で旧 digest をそのまま採用する案は、self-binding と exact
contract の説明が弱いため推奨しない。

### 2. G0-D3

G0-D3 は現在 explicit defer である。P1/P2 が成功しても、owner が exact digest を
受理し canonical exit record を作るまで official T1 entry は成立しない。profile
repair の承認を G0 exit の承認として扱わない。

### 3. T2 target

本計画の元の「同じ checkpoint」推奨は superseded である。P016 の recorded
direction は narrow T2 の後に separate I1-readiness/bootstrap record を置く。
具体的な profile wording と acceptance は引き続き owner / Canon action である。

## Semantic direction checkpoint 2

### Critical path

| Boundary | 必要な理由 | 現在の LAB recommendation |
| --- | --- | --- |
| PROPOSAL-008 | OBL-021 coherence だけでは outcome existence が出ない | **A recorded**。exact domain/total diagnostic coverage は P4A |
| PROPOSAL-012 V/R/S/A | value/receipt/service/admission identity がない | **V1/R1/SW1/conditional A2 recorded**。pending/facet/causal composition は P4A |
| PROPOSAL-013 | validation claims の保持又は導出が未選択 | **M1 recorded**。request binding/replay/failure classification は P4A |
| PROPOSAL-004 | exact Surface parser / SCN finalization ができない | **A recorded**。exact accepted domain は P4A |
| SCN-08 / `return` | scalar/indexed mismatch と unelaborated control token | **P015 recorded**。scalar correspondence と diagnostic rule は P4A/normal process |
| lifecycle | T2/I1 relation と C-static timing | **P016 recorded**。profile/authorization は後続 normal process |

方向の記録は tuple の互換性を証明しない。P4A では V/R/S/A/M、SCN-02、SCN-08、
totality の composition と adverse cases を比較し、必要な Canon amendment は別 proposal
として停止する。

### Deferrable from the explicit T0-T2 critical path

- PROPOSAL-003 は proof package の organization であり、具体的 `Step` / frame /
  proof interface を選ばない。package-local organization で進めるなら T1 model の
  semantic blocker ではない。
- PROPOSAL-010 は locus hierarchy を使わない限り overview wording の修正として
  独立に扱える。既存 principal/admission summary へ直す A が最小である。
- PROPOSAL-011 は OBL-026 / overlay cost preservation に関係する。狭い現行 T2 の
  explicit row ではない。T2=I1-entry profile が patch compatibility まで要求する場合だけ
  T2 前へ繰り上げる。
- L2 trust anchor は delegated L2 promotion を止めるが、L3 research と owner による
  direct Canon adoption は止めない。長い自走の効率改善には有用だが、理論内容の前提ではない。

## Shared formal model の最小責務

P5 は runtime 実装ではなく、Canon statement を同じ carrier 上で比較・証明するための
proof-facing model である。最低限、次を明示する。

1. Surface assignment / handler / locus block の対象 fragment。
2. Core `read` / `write` / `request` / publish / observe / grant / witness /
   admit / cut / patch / `seq` / `cond` / `pure`。
3. elaboration input tuple と success / Diagnostic outcome。
4. result value flow、request/result correlation、failure resumption。
5. Config の H / Q / S / M / G / W / L / P と component frame。
6. occurrence insertion、freshness、causal edges、acyclicity。
7. request validation context と authoritative store との比較。
8. five Canon WellFormed clauses と、必要なら別 predicate として置く queue /
   patch admissibility。
9. chain normalization、lineage、lease、reacquire。
10. SaveObject input predicate、restore relation、restored-state postcondition。

実験用 `Result`、opaque `Config` / `Step` / `WellFormed`、helper JSON、runtime log、
transport session を Canon object と同一視しない。

## Proof skeleton の推奨 operational definition

Canon に status mapping がないため、次は P3 で検討する LAB recommendation である。

- target proposition と全 public lemma signature が import-bearing Lean で typecheck する。
- theorem を `True`、opaque predicate、未記録 axiom へ置換しない。
- `sorry` / `admit` / hidden `Classical.choice` / accidental axiom profile を scan する。
- OBL-020 は全 Canon step family と五つの WF clause の coverage matrix を持つ。
- OBL-021 は outcome existence、success equality、Diagnostic equality、success/reject
  exclusionを分ける。
- OBL-002 は OBL-001 の exact Core-write statement に対し local / cross write と
  `seq` / `cond` case を明示する。
- 未証明 leaf は「証明済み theorem」として宣言せず、dependency / premise /
  falsifier / owner boundary を machine-readable でなくても exact に列挙する。
- ledger status は owner が既存 vocabulary のどれに対応させるかを決定する。

この定義は `lean-proved` を意味しない。T2 exit profile が skeleton を
`lean-stated` + reviewed decomposition evidence と読むか、別 criterion を設けるかは
owner action で固定する。

## G5 package の循環回避

現在の load admissibility は「stale resurrection がない」を precondition に含み、
THM-003 も同じ性質を conclusion に含む。P10 は次を分離しなければならない。

1. saved-object に対して検査可能な predicate。
2. saved object と restored Config の restoration relation。
3. restoration 後の membership / witness / lease / provenance liveness property。
4. Consistent cut checker の input と soundness statement。
5. checkpoint graph と Netzer-Xu characterization の correspondence。

desired conclusion を success predicate に埋め込んで proof と呼ばない。

## I1-entry readiness audit

P11 は official Gate を追加しない LAB audit である。P3/P12 の profile design に、
最低限次を返す。

| 領域 | I1 前に必要 | 後段へ defer 可 |
| --- | --- | --- |
| Surface/Core | I1 が実装する exact fragment と elaboration relation | final public grammar / ABI |
| type/effect/failure | checker が拒否・受理できる finite rules | final cost algebra、arbitrary dependent types |
| runtime | Config / Step / observable failure behavior | performance、real transport、durability |
| authority | validation claims / authoritative comparison / stale rejection | production identity provider |
| history/cut | occurrence insertion / consistent cut / local save-load statement | distributed durable realization |
| observation | I1 SCN が必要とする typed/redacted semantics | final viewer / telemetry ABI |
| patch | I1 SCN-09 を解釈できる check/admit/no-mutation semantics | final hot-plug ABI / migration engine |
| projection | BND-006 の preservation list と I1 の非分散 boundary | optimizer / codegen / final wire |

加えて、全十 SCN に対する G0-G7 の criterion、有限 fragment の OBL-003、cut /
save-load attribution の OBL-027 を、`statement-accepted`、`lean-stated`、
`lean-proved`、`external` のどの evidence class で I1 前に要求するかを明記する。
G4/G6/G7 を T2 前に exit させるか、I1 中に statement を actualize するかは現在の
Canon から一意に決まらない。I1 を全 SCN 実装と読むなら前者の statement-level
closure が保守的であり、後者を採るなら I1 対象 fragment を狭める Canon action が
必要である。

## 並行化

owner disposition 後も、各 package が ADR-0014 の standing predicate と existing-lane
条件を個別に満たす範囲で、次は並行に進められる。

- G1 elaboration、G2 chain、G3 authority の model preparation。
- G2 の OBL-005/006 と OBL-007/008。ただし chain/lineage carrier は共有する。
- G5 の OBL-010 / 014 と OBL-009。ただし history/checkpoint domain は共有する。
- OBL-020 / 021 / 002 skeleton と G5 statement。ただし accepted shared model 後。
- runnable LAB maintenance。ただし lifecycle evidence と混ぜない。

次は順序を崩さない。

- T0 profile revision -> fresh artifact -> G0-D3 acceptance。
- semantic disposition -> shared model -> proof-facing statement。
- statement identity acceptance -> proof skeleton。
- owner disposition / independent review -> Canon integration。
- T1 profile acceptance -> official T2 entry -> T2 profile acceptance。

## 自走 package 共通 close 条件

各 package は次を満たすまで close しない。

1. authority cut、Canon anchors、LAB source cut、alternative / falsifier、
   non-effects、rollback / reopen trigger が明示される。
2. 正例と負例、又は「なぜ実行不能か」の再現可能な source evidence がある。
3. Lean artifact は `--trust=0`、import path、tool version、axiom profile を記録する。
4. SCN / THM / OBL / OPEN / BND の traceability がある。
5. runtime/sample evidence を Canon proof / conformance と言い換えない。
6. focused validation、docs/source hierarchy、diff check を通す。
7. author と異なる reviewer が semantic delta / hidden assumption / overclaim を確認する。
8. report、snapshot update、commit `--no-gpg-sign`、push、remote parity を確認する。

## Mandatory stop conditions

次のいずれかで package を止め、owner decision bundle を作る。

- L0/L1、Core / authority / effect / failure / judgment primitive の選択。
- SCN expectation、Gate / Phase criterion、`theory/11` の変更。
- external/public/wire/serialization/provider/transport contract の選択。
- counterexample が intended theorem 又は settled invariant を破る。
- exact proof interface が二つ以上残り、Canon から選べない。
- new evidence lane / helper / schema / CI / Make surface が必要。
- production implementation を main に入れる必要。
- T2 criteria と I1 entry requirement の対応が一意でない。

## Finite autonomous horizon

現在の **official lifecycle** horizon は P0 で止まり、owner action なしに T0/G0 を
動かせない。一方、research horizon は P0A まで開いている。P0A では、例えば既存
Core 文面の direct statement、totality を仮定しない relation uniqueness、既記載の
有限 predicate / lineage / mutation-use / event-only cut propertyについて、literal
transcription 又は conditional lemma が ADR-0014 の standing predicate を満たすかを
一件ずつ審査する。

P0A は「T2 の proof skeleton を先に作る」許可ではない。current source-cut screen の
重複排除を再確認し、既存 lane と現行 consumer を特定し、reserved relation を選ばずに
正負 branch を区別できる候補だけを pre-register する。候補がなければ追加 WRK を開かず、
owner checkpoint 1 を待つ。frozen record を修理せず、既存 toy relation を包み直さない。
Plan 226 は、この規律で post-WRK-0043 の G5 restore-quantifier candidate を再審査し、
T-RESEARCH-014 / Report 2267 の coupled restoration-interface boundary を弱く言い換えた
重複と確認した。新しい L3 record は作らない。

owner input 後は次のように一 package ずつ委任する。

1. P1/P2: T0 profile v2 design と fresh exact evaluation。G0-D3 前で停止。
2. P4/P5: 選択済み semantic tuple に対する shared formal model。ledger movement
   前で停止。
3. P6/P7: accepted model に対する T1 statement package。Gate/T1 acceptance 前で停止。
4. P9/P10: accepted T1 statement に対する T2 skeleton / G5。T2 acceptance 前で停止。
   P11 は integrated T2-I1 route のときだけ T2 前に置き、narrow T2 では T2 後の
   I1-readiness acceptance 前で停止する。

この刻みなら、routine research target を毎回 user に訊かず、macro semantic /
lifecycle decision だけを明確な checkpoint に集約できる。

## 次に owner が判断する最小事項

official lifecycle を進める最初の判断は P1 の scope である。P0A の eligibility
preflight はこの判断と並行して自走できるが、判断の代替にはならない。

> T0 profile は `pass` を success literal とする version 2 に更新し、v1 artifact を
> fully conforming と再分類せず履歴証拠として保持し、v2 に対する一回限りの fresh
> artifact を許可する。これは G0-D3 / G0 exit をまだ承認しない。

この判断が得られれば、P1 の Canon proposal / revision packet と P2 の exact
evaluation までは自走できる。G0 exit の要否は、その fresh digest を見て別に owner が
判断する。

## Non-claims

- official T0 / G0、OBL status、proof、conformance、sample status は動かしていない。
- T1 / T2 profile、proof skeleton definition、I1-entry criterion は未採択である。
- P008 / P012 / P013 / P004 等の recommendation は owner disposition ではない。
- LAB runtime / Product Alpha / Full System V1 / Surface evidence は、formal model や
  Canon implementation の代替ではない。
- Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform は分離したままである。
