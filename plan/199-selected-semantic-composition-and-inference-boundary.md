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

したがって C0 は未着手のままである。再開するなら、別番号の pre-registration で
displayed grammar と P004 candidate grammar を明示的に分け、token assertion 自体の
alternative/falsifier を再登録する。凍結 record を修理又は再実行してはならない。

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
C0 exact domain
  -> C1 read/write countermodels, C2 request/replay, C6 scalar terminal
  -> C3 pending value flow, C4 served-write facets, C5 admission facets
  -> C7 inference/desugaring equivalence matrix
  -> shared Core / Config / Step / WellFormed / elaboration / history model
```

前半の独立調査は並行にできるが、共通 carrier を確定したことにしてはならない。各
research package は ADR-0014 の standing predicate を個別に満たすかを先に確認し、
必要なら L3 `working/WRK-####` の pre-registration を作る。C0--C7 が新しい
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

最初の自走 package は C0--C2/C6 の source-anchor と countermodel matrix である。
そこでは候補を比較しても Canon rule を改訂せず、各行について「決定済み事実」「必要な
追加表現」「falsifier」「既存 invariant への影響なし」を記録する。C3--C7 と実装は、
その matrix が共通 carrier の最小性を支持してから扱う。

## Non-claims

本計画は final grammar、parser/checker/runtime、request wire format、exactly-once
network protocol、transport authentication、SCN change、OBL proof、Phase movement、
production implementation、public release を選択又は主張しない。
