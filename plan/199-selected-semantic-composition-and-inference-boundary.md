# Plan 199 - 選択済み意味論の合成と推論境界

## 役割と権限

これは Canon と選択済み proposal を照合して作る **LAB repository memory** である。
規範正本は `mirrorea_canon/` であり、この計画は Core、Surface grammar、SCN、OBL、
Gate、Phase、proof status、runtime、wire、public contract を変更しない。

2026-07-28 に owner が記録した direction は、後続の比較・反例探索を許す範囲だけを
定める。具体的な Canon amendment、profile、実装認可は別の通常手続である。Oracle の
助言は local Canon source と照合した設計上の入力であり、それ自体を正本として扱わない。

## 目的

選択済みの方向を、実装を始められるという主張に飛躍させず、単一の proof-facing
operational model へ進むための残課題を明確にする。副目的は、Mir source が自明な情報を
不必要に繰り返さなくて済むよう、**安全な推論・desugaring の境界**を早期に検査すること
である。

## 記録済みの方向

| 境界 | 記録済みの方向 | この時点でまだ決まらないもの |
| --- | --- | --- |
| Surface v0 | P004 A: `Participant` indexed keyspace のみ | exact grammar/AST、custom keyspace、scalar Core 表現 |
| elaboration outcome | P008 A: determinism と分離した totality obligation | well-scoped domain、obligation identity、diagnostic universe |
| value/occurrence | P012: V1/R1/SW1/conditional A2 | request/receipt/occurrence carrier、causal relation、save/load rule |
| validation | P013 M1: request-local claims を authoritative facts と照合 | field name、queue/wire encoding、request identity |
| fallback/return | P015: scalar terminal/default は明示、`return` は v0 から除外 | scalar declaration/Core correspondence、diagnostic catalog |
| lifecycle | P016: narrow T2 + separate I1 readiness、bootstrap の後に C-static formal entry | profile、moratorium exception、implementation authorization |

これらは Canon rule の改訂ではない。特に `Participant`-only は indexed state の v0
closureであり、`room_anchor` を隠れた `Participant` singleton にしてよいという意味ではない。

## 現在の合成停止線

選択 package は方向として整合するが、まだ composition-closed ではない。共通 model を
書く前に、次の項目を反例と source anchor で閉じる。

| ID | 未解決点 | 最小の確認 | 止める条件 |
| --- | --- | --- | --- |
| C0 | exact Surface/Core domain と totality | `WellScoped` domain、accepted/rejected source class、Diagnostic coverage を分離する | 新しい grammar/Core/OBL を選ぶ必要が出る |
| C1 | SCN-02 cross-locus read/write | read set、評価 locus、snapshot、target read-mutate atomicity を決めずに lost-update trace を排除できるか | compound update 又は concurrency rule を暗黙選択する必要が出る |
| C2 | M1 request binding と replay | admitted-execution binding、semantic request identity、failure-class total mapping を設計比較する | hidden side table、transport identity、新 primitive が必要になる |
| C3 | V1/R1 pending control | typed pending state、linear `Delta`、receipt と success/failure continuation を定義できるか | dependent write の失敗時抑止を表せない |
| C4 | SW1 served write | 一つの served occurrence に service/mutation/authority facets を載せ、request-to-serve を保てるか | validation と mutation の間に無根拠な中間 event が必要になる |
| C5 | conditional A2 | verdict、membership、各 grant、witness を named facets として参照し、rejection が M/G/W を変えないことを表せるか | issuance が別に失敗・観測・schedule される |
| C6 | SCN-08 scalar terminal | scalar cell の owner/init/visibility/store well-formedness と terminal admissibility を閉じる | type default、hidden membership key、unbound default を導入する |
| C7 | source ergonomic inference | omitted fact が唯一に決まり、elaborated artifact から復元できるかを検査する | ambiguity、authority/failure/history 差、再構成不能がある |

`C1` の典型反例は、HP=10 に対し二つの attack が 3 と 4 を同時に読む場合である。
snapshot/evaluation rule なしに `x = x - y` と compound atomic update を同一視すると、
どちらかの減算が失われ得る。この問題は実装最適化ではなく、SCN-02 が必要とする
意味論の一部である。

### C1 の限定結果（WRK-0024）

WRK-0024 は、二つの read reply がともに HP=10 を返した後に、owner が 7 と 6 の
write を直列に適用する有限モデルを Lean で再現した。最終値は 6 で、owner で 3 と 4
を順に減算するモデルの 3 と異なる。これは **already-computed write の owner seriality
だけでは** atomic read-dependent update を導けないという反例である。

この結果は current Canon execution、SCN-02 failure、又は特定 repair の選択ではない。
必要な snapshot/evaluation/pending/request relation は未選択であり、Plan 199 C1 は
shared model 前の explicit decision boundary として残る。再現手順と scratch digest は
`plan/wrk-0024-scn02-read-write-snapshot-ambiguity.md` にある。

### C0 の登録済み falsifier（WRK-0025）

WRK-0025 は source inventory を実行する前に、登録済みの required-token check が
`spec/02` に存在しない `CallArgs` を要求したため `frozen` となった。`CallArgs` は
P004 の候補 EBNF の非終端であり、displayed `spec/02` は postfix `call` を prose で
列挙する。この失敗は「call が unclassified」又は「C0 は不要」という結果ではない。

したがって C0 の semantic domain / outcome question は未着手のままである。current-cut の
source authority subquestion は後続 WRK-0028 が literal に完了したため、C0-A を重複して
再登録しない。WRK-0029 は、displayed grammar、static source、BND-001 `WellScoped` input
の四つの opaque role と terminal branch が rank-increasing なら非循環、という条件文だけを
retained した。これは input role、`WellScoped`、outcome 又は Diagnostic を定義しない。
WRK-0030 は C2-A を source-tagged anti-collapse vocabulary audit として閉じた。payload、
claims、binding、request unit、attempt unit、replay classification は同じ種類の object と
仮定せず、WRK-local question label と source-owned subject の対応だけを保持する。その
documentary non-substitution は、payload equality、request identity、binding、attempt、
replay policy のいずれも選ばない。次の candidate は、この index を semantic result と
誤用せず、C0-C/C0-D、C1、C2-B、C6 を source cut と adverse case に照らして再選別した。
WRK-0031 は C0-C を source-local Diagnostic reference audit として閉じた。literal named-error
又は explicit carrier/family reference は記録したが、coverage、stage membership、accepted/
rejected domain、Diagnostic assignment、totality/coherence は扱わない。C0-D は P008 と既存
outcome-totality evidence の重複、C1/C6/C2-B は candidate comparison が snapshot/scalar/
identity の選択に直結するため、この時点の L3 route から除外する。次の自走 action は C0-C
を semantic result と誤用しない portfolio re-screen であり、凍結 record を修理又は再実行しては
ならない。

### C2 の登録済み falsifier（WRK-0026）

WRK-0026 は P013 に存在しない連続 token `copied/replayed requests` を登録済み
assertion が要求したため `frozen` となった。P013 は `copied/replayed, stale,
wrong-target, ... requests` と列挙する。これは M1 が replay discrimination を供給する
かどうかの結果ではない。C2 は未着手のままであり、再開するなら別番号で request
equality、request identity、duplicate policy を混同しない source inventory を
pre-register する必要がある。

この二つの command falsifier から、Canon の結論を推論してはならない。以後の
source-inventory successor は、一つの連続引用 token への依存を避け、各 source の
literal fact を独立に記録するか、pre-registration 前に read-only source check で
確定した最小 assertion だけを採る。これは LAB execution discipline であり、Canon の
研究権限・意味論・証拠基準を変更しない。

### C6 の literal source boundary（WRK-0027）

WRK-0027 は、SCN-08 が `live_pose[p: Participant]` と scalar
`room_anchor`、terminal `default_pose` を並べること、表示済み Surface grammar と
MirCore v0 の state declaration はいずれも indexed form であることを、登録後の
source comparison で確認した。static semantics も indexed state を
`Active(K, epoch) ⇀ A` と説明し、theory/06 は chain の各 option に declared access
target を要求する。P015 はこの比較と整合し、scalar terminal/default の明示的な
Surface/Core correspondence を将来 package の要件として残している。

この記録は SCN-08 が現在不適合である、又は state/terminal の候補を決めた、という
結論ではない。表示済みの source だけでは `room_anchor` を scalar cell としてどう
所有・保存・初期化・可視化するか、`default_pose` をどこで宣言し chain target として
どう解決するかを導けない、という bounded source fact である。後続比較は少なくとも
distinct scalar Core declaration と、既に明示された有限 domain への conservative
elaboration を候補として分け、init/default、target resolution、store
well-formedness、lineage law を同じ観点で検査する。hidden membership key、type
default、unbound terminal を推論又は desugaring で補うことはできない。

## 明示と推論の規律

source 上の fact を省略できるのは、次の二条件を同時に満たす場合だけとする。

1. 規範的 input から一意に決まる。
2. elaborated artifact が、その fact と根拠を検査・復元できる形で保持する。

一意性又は復元性が失われる場合は heuristic を置かず、診断する。これは使い勝手と
検証可能性を両立させるための設計制約であり、現時点での Canon rule ではない。

| source から省略を検討できる候補 | 必須の復元根拠 | 省略できない候補 |
| --- | --- | --- |
| state owner/type/visibility、lexical locus、`Participant` keyspace、source span、dependency/publish annotation | declaration/environment と deterministic elaboration trace | semantic request/pending/grant/witness identity |
| failure set（total ruleで一意な場合）、単一一致する `Delta` binding の cap/witness reference | selected total classifier 又は唯一の binding | read materialization、evaluation locus、snapshot/fusion choice |
| principal/epoch/incarnation（唯一の admitted-execution binding から administrative に注入する場合） | M1 claims を含む explicit binding | A2 verdict/grant/witness facets、owner-mediated authorization result |

fallback の canonical flattening は表層 sugar として検討できるが、target、guard、
capability、default/terminal evidence を省略してはならない。推論候補は final store の
一致だけでなく、value/receipt、occurrence、authority、failure、history が同値であることを
正負 trace で検査する。

## 自走する研究順序

```text
R0 common Canon-cut re-anchor
  -> C0-A complete-by-R0, C0-B retained conditional DAG, then C0-C..D diagnostic/totality split
  -> C2-A..E equality/identity/binding/replay/persistence split
  -> C1 candidate families, C6a scalar-cell and C6b terminal-target candidates
  -> C5-PRE source-local conditional-A2 issuance-guard audit
  -> ordinary Canon design for C5 facets, C4 SW1 identity, and C3 pending control
  -> C4b/C5/C3 integration only after the required reference/correlation boundary
  -> C7 inference/desugaring equivalence matrix
  -> shared Core / Config / Step / WellFormed / elaboration / history model
```

R0--C7 の詳細な task 境界、candidate-local micro-model と candidate-neutral observation
record の区別、adverse trace、stop condition は Plan 200 に置く。前半の独立調査は並行に
できるが、共通 carrier を確定したことにしてはならない。各 research package は
ADR-0014 の standing predicate を個別に満たすかを先に確認し、必要なら L3
`working/WRK-####` の pre-registration を作る。C0--C7 が新しい
Core/judgment/SCN/OBL/Gate/Phase/external contract を必要とした時点で止め、代替・反例・
non-effects を含む successor proposal に切り替える。

## 共有モデルへの受入条件

次の全てが揃うまで shared model を「accepted」又は implementation-ready と呼ばない。

1. exact accepted domain と total elaboration/diagnostic coverage。
2. SCN-02 の read snapshot と dependent write の正負 trace。
3. M1 claims、semantic request identity、replay/retry、total validation failure mapping。
4. V1/R1 pending-control linearity と SW1/A2 occurrence facets・causal edges。
5. SCN-08 scalar/terminal の well-formedness と no-hidden-default trace。
6. inference/desugaring ごとの source-to-elaborated evidence equivalence。
7. existing Canon invariant、DAG、failure-no-mutation、save/load、authority lineage との
   traceability。

この受入条件は proof discharge、ledger movement、T1/T2 exit、I1 authorization を意味
しない。これらは Plan 196、Plan 197、P016 の後続 profile/acceptance に残る。

## 次の成果物

R0 により source-anchor は current cut に固定された。C0-B は four-role domain-staging
conditional lemma として閉じ、WRK-0030 は C2-A の six source-tagged question labels と
documentary non-substitution だけを retained した。いずれも shared carrier を支持する semantic
result ではない。WRK-0031 は C0-C の source-local Diagnostic reference audit を closed
evidence として retained した。source span の literal reference の有無だけを記録し、coverage
又は stage/reject domain を導かない。C0-D、C1、C2-B、C6 は現行 evidence cut では L3 の
非重複な result に閉じない。C3/C5/C4 portfolio の local/Oracle screen は、C3 と C4 の最初の
有意な検査が pending/request/occurrence identity 又は carrier を選ぶため ordinary Canon design
boundary へ停止すると確認した。C5-PRE は通常 admission source span に P012 の conditional-A2
停止条件を明示する独立 issuance phase が literal に現れるかだけを監査し、WRK-0032 は P012 guard
direction と four named ordinary-admission theory/spec span の non-match を retained した。詳細と
patch-admission 除外は Plan 201 と WRK-0032 evidence に置く。この result は A2 atomicity、
compatibility、facet、carrier を支持せず、実装は common carrier の最小性を evidence が支持してから
扱う。次の autonomous action は C3/C4/C5 本体を先取りせず、remaining frontier に non-duplicate
existing-lane L3 candidate があるかを fresh preflight で再審査することであった。Plan 202 はその
再審査により、P012 の V1/R1 を二つの administrative presentation として比較する `C3-VR-PRE`
だけを選別した。これは pending/control/correlation/persistence の C3 本体設計ではなく、matching、
single-use、failure exclusion を明示した場合にのみ有限 LAB comparison ができるかを調べる候補である。
いかなる結果も source-level inference を自動承認せず、fact と一意な根拠が elaborated artifact から
復元できることを別途必要とする。WRK-0033 は登録後、administrative binding と one-slot machine
presentation の finite observation equality、並びに matching/single-use/failure exclusion を各々外した
三つの distinction を retained した。これは C3 pending/correlation/persistence の設計、full trace
equivalence、又は source inference を支持しない。次の autonomous action は C3/C4/C5 本体を先取りせず、
remaining frontier に non-duplicate existing-lane L3 candidate があるかを fresh preflight で再審査すること
である。

その再審査は Plan 203 の `C3-VR-SEQ-PRE` を選別し、WRK-0034 は finite state/reply/transition/
translation/observation/assumption を固定したまま one-step translation-preservation と arbitrary finite
reply list の local-observation equality を conditional lemma として retained した。full trace equivalence、
transport/scheduler/history、C3 pending/correlation/persistence、source inference を含まない。WRK-0034 後の
fresh ADR-0014 preflight は Plan 204 に provisional disposition として記録した。fixed model の追加定理は
既存結果の系又は重複である。一方 C7 は `erase`/`observe` を parameter にした carrier-neutral
factorization criterion を WRK-0035 で non-promoted L3 conditional lemma として retained した。これは
unique realized observation on `range erase` の extensional boundary に限られ、inspectable grounds、concrete
elaborated artifact、又は source omission rule を供給しない。C0-D/C1/C2-B/C3--C6 の有意な前進は ordinary Canon design/owner boundary を越える。C3 proper の
semantic design package は carrier-selecting boundary として記録するが、着手順序をこの LAB plan が決めない。
WRK-0035 後の fresh screen は、個別 factorization を同時 omission へ合成してはならないことを
Plan 199 の C7 matrix consumer に結ぶ fixed finite countermodel だけを Plan 206 で選別し、
WRK-0036 が `--trust=0` で retained した。これは source fact、grounds、artifact、又は rule の
組合せを定義せず、未来の final cumulative representation を直接検査する negative guard に限る。
Plan 207 の fresh frontier disposition は C0-D/C1/C2-B/C6/C7 を no-candidate、C3/C4/C5 を
ordinary Canon design boundary とした。次の LAB 作業は source rule を先取りせず、C2-B/C3 の
identity、correlation、pending boundary を比較可能な decision preparation として整理することに限る。
Plan 208 は P012 V1/R1 と P013 M1 を同じ C2-B/C3-alpha trace に置き、relation-first reference、
request-occurrence anchoring、nominal attempt alternative を比較する。どの carrier も採択せず、
reply/receipt、failure、held linear context、cut/save-load の explicit obligations を整理する。
Plan 209 はこの comparison を prefix-local に監査し、completed-success shorthand の四項 `Corr` を
sole relation とせず、pending / validation-outcome / reply / receipt / failure の staged relation と
restore configuration requirement を比較条件にする。これは carrier、source rule、or implementation を
採択しない。
Plan 210 はその各 obligation に relation-first Family A と request-occurrence Family B を対応させ、
DAG の ancestry/order も unlocated relation も semantic correlation/pending/receipt/load identity を
自動供給しないと記録する。A/B は conditional candidate のままであり、source omission を先取りしない。
Plan 211 は owner disposition を置換せず、B2-OPAQUE を有限 L3 experiment の pre-registration
候補に限って選別する。二つの opaque request atom と explicit q-indexed projection/injective restore を
使うが、それらを Core、Config、history、SaveObject の selected carrier と読まない。
WRK-0037 は registration/push 後にその fixed finite table を実行した。equal-incidental な二 atom、
staged view、one scoped receipt/resume extension、failure-no-mutation、grounded dependency、
involutive local reindexing を `--trust=0` で確認した。これは一表における non-promoted L3
evidence に限り、request carrier、restore/persistence rule、source inference、又は Canon design
selection を支持しない。特に incidental record について retained するのはこの二 atom を両方回復する
total left inverse がないことだけであり、一般の recovery/inference rule を否定しない。
WRK-0039 は、その exact table を全十の supplied `(Frontier, Request)` fiber ごとに independent
relation graph と bundled lookup の間で往復させる有限 L3 evidence を retained した。五 graph の
enumeration、全 receipt/resume `none` 結果、derived combined relation、restore graph、すべての
fiberwise round trip を `--trust=0` で検査した。bare `DirectView` から全十 key の request を回復する
single total function はないが、これは一部の fixed-frontier view が request を区別しないことを意味しない。
この evidence は key recovery、carrier、identity、persistence、source inference を選ばない。

Plan 214 は WRK-0039 後の同じ authority cut を再審査し、次の ADR-0014 L3 record を
作らない scoped disposition を記録した。既存表の bisimulation/path theorem 又は equality-class
inventory はこの有限結果の重複であり、cross-load coherence、pending、reply/receipt/failure、replay、
restore 又は source reconstruction は C2-B/C3/C7 の ordinary Canon design を選ばずに定式化できない。
Canonical premise の追加、authority cut の変更、具体的な downstream claim、又は WRK-0039 defect が
reopen trigger になるまで、有限 presentation lane を別名で拡張しない。

Plan 215 converts the resulting ordinary C2-B/C3 decision boundary into three
coupled LAB bundles: definitional correlation basis, branch/lifecycle
projections, and restore/one-shot/linearity scope. This is not a new Canon
decomposition or carrier selection. Its source-convenience condition is a
future model-relative elaboration proof, never reconstruction of identity or
authority from incidental source facts.

Plan 216 audits that packet across the existing theory boundaries before any
candidate comparison. It requires semantic residence for every staged fact,
the full admissible load state, static-versus-dynamic failure separation, and
trace-set rather than scheduler determinism. It changes no Canon proposition
or carrier selection.

Plan 217 corrects the comparison method itself: candidate cards use erased,
candidate-native observations and linkage relations, not a shared pending/key
signature. It can expose `CARRIER-GAP` or a countermodel without selecting a
candidate or a common lifecycle model.

## Non-claims

本計画は final grammar、parser/checker/runtime、request wire format、exactly-once
network protocol、transport authentication、SCN change、OBL proof、Phase movement、
production implementation、public release を選択又は主張しない。
