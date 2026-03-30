# 04 — Mir Core

## 状態

Mir Core は、このプロジェクトで最も基盤的な部分である。
明示的に別記されていない限り、そのアーキテクチャは decision level L0 / L1 として扱うべきである。

この文書は次を区別する。

- **Mir 全体**: 後続の進化や統合作業も拘束する、より広い意味論的基盤。
- **Mir-0**: 最初に安定化すべき最小意味論カーネル。

明示的に別記されていない限り、以下の「Mir-0 の最小意味論コア」節が、Workstream A における Mir-0 の現在の焦点定義である。
これは先行する規範文書を置き換えるものではなく、`specs/01` から `specs/03` と `specs/09` を併読して読む。

## 四本柱

### 1. 因果と時間

- 計算は event の directed acyclic graph として表現される。
- finalization boundary は意味論上重要である。
- より広い Mir の cut vocabulary では以前から `barrier`、`atomic_cut`、`durable_cut` が言及されていたが、Mir-0 が固定するのは `atomic_cut` だけである。`barrier` と `durable_cut` の扱いは後段へ送る。
- システムは「何が何より前に起きたか」と「何がいつ final になったか」を説明できなければならない。

### 2. Effects と Contracts

- 外部作用は、隠れた実装詳細ではなく first-class effect である。
- Contracts（`require`、`ensure`、`invariant`）は実行可能意味の一部である。
- failure は汎用的な exception の入れ物ではなく、構造を持ち contract-aware でなければならない。

### 3. Ownership と Lifetime

- linear resource、あるいは ownership に類する resource は重要である。
- lifetime と fallback 風の degradation は monotone に保たれなければならない。
- preference chain と guarded reference は現在も活発な設計論点である。

### 4. 安全な進化

- patch は意味を保ち、graph discipline を尊重すべきである。
- 既定規則は arbitrary rewiring ではなく downstream addition による進化である。
- overlay は互換性を保ち、既存 API を silent shadowing してはならない。

Mir-0 は、patch、overlay、route rebinding の完全な運用意味論 (operational semantics) を固定しない。
ここで固定するのは、後続の mechanism が従うべき意味論的制約だけである。

## Mir-0 の最小意味論コア

ここで固定する最小 kernel は、意図的に狭い。
Mirrorea 固有、Prism 固有の machinery を持ち込まずに、因果、明示的 effect、rollback constraint、ownership discipline を述べるのに必要な primitive だけを含む。

### 1. Event DAG と因果

- Mir-0 の計算は semantic event の directed acyclic graph で表現される。
- graph は因果説明を明示しなければならない。すなわち、どの event がどの event より前に起き、どの boundary が先行 prefix を final にしたかを説明できなければならない。
- 意味の上で隠れた backward edge は許されない。

### 2. `place`

- Mir-0 は `place` の最小概念を含む。
- `place` は、local state、rollback scope、ownership transfer が解釈される最小の意味論的 locus である。
- Mir-0 は、same-place reasoning と cross-place interaction を区別できるだけの構造を要求する。
- 正確な routing、naming、distributed placement policy は Mir-0 の外にあり、後段の Mir または Mirrorea の作業に属する。

### 3. `perform`、effect、contract

- Mir-0 では、明示された contract の下で effect を要求する最小操作だけを仮定する。
- 本文ではその操作を便宜上 `perform` と表記するが、これは Mir-0 の規範的な表面構文としてはまだ確定しない。
- `perform` を最終的な予約語として採用するかどうかは **未決定** である。
- effect は、隠れた実装詳細ではなく first-class semantic action である。
- contract は admission と outcome space の両方を制約する。現在の vocabulary には `require`、`ensure`、`invariant` があるが、正確な表面構文はここでは固定しない。
- effect success、rejection、approximation、compensation は event graph 上で可視であり続けなければならない。

### 4. 最小 failure space

- Mir-0 は、少なくとも次の named outcome を持つ最小 structured failure space を固定する。
  - `Reject`
  - `Approximate`
  - `Compensate`
- contract は、どの degraded outcome が許可されるかを明示しなければならない。
- これら最小 class を超えた lattice-theoretic な正確な表現は **UNRESOLVED** である。

### 5. Primitive fallback

- Mir-0 は、明示的 failure または monotone degradation の後に、次の contract-compatible option へ進む primitive として fallback を含む。
- fallback は、隠れた backtracking、隠れた API shadowing、linear resource の duplication を導入してはならない。
- nested fallback / preference chain の完全な normalization は Mir-0 の外にある。

### 6. local rollback を伴う `try`

- Mir-0 は、local rollback semantics を持つ `try` を含む。
- rollback は current `place` に局所であり、finalizing cut を越えていない state だけを巻き戻せる。
- Mir-0 における `try` の rollback frontier は current `place` 内で閉じており、途中に `atomic_cut` があればそこで停止する。
- rollback 不可能な effect は rollback region の内部で禁止するか、そこから隔離するか、明示的 compensation で扱わなければならない。
- rollback は隠れた control-flow trick ではなく、明示的な failure / effect semantics の一部である。

### 7. `atomic_cut`

- Mir-0 は、最小の place-local finalizing cut primitive として `atomic_cut` を含む。
- Mir-0 において `atomic_cut` が確定するのは current `place` の rollback frontier だけである。
- `atomic_cut` は、単一 process 全体の同期点、複数 `place` をまたぐ合意点、永続化完了点を意味しない。
- `place` の内部では、`atomic_cut` は rollback が通過できない明示的 decision boundary を作る。
- `atomic_cut` の後で failure が表面化しても、同じ `place` の pre-cut 部分は rollback されない。以後は compensation、fallback、または明示的 failure として扱う。
- Mir-0 は `atomic_cut` の rollback / finalization 上の役割を固定するが、それだけで persistence や distributed finalization の semantics を決めることはしない。
- 後続計算が pre-cut effect を考慮する必要があるなら、implicit rollback across the cut ではなく、明示的 continuation または compensation を通じて扱わなければならない。

### 8. Linear resource と monotone lifetime

- Mir-0 は、rollback、fallback、後続の進化 machinery による duplication を防ぐために必要な最小 linear-resource discipline を含む。
- ownership transfer は明示的でなければならない。
- lifetime degradation は monotone である。
- 後続の convenience layer は、これらの規則を弱めるのではなく保持しなければならない。

## fallback / preference chain / `lease` の最小意味（L2）

この節は、Mir-0 を広げずに、primitive fallback と monotone lifetime から導かれる**より広い Mir の現在の最小読解**を L2 として記録する。
Mir-0 に残るのは従来どおり primitive fallback と monotone lifetime であり、ここで述べるのは、その先の正規形と guard の暫定的な最小解釈である。

- fallback 付き参照、または guarded reference は、1 つの論理的な access path に対して、有限個の contract-compatible option を優先順で結びつけたものとして読む。
- 各 option は、少なくとも access target、許可された contract surface、そしてその option がまだ利用可能かを決める lifetime guard を持つ。
- この lifetime guard を本文では `lease` と呼ぶ。`lease` は later cut vocabulary や新しい failure class ではなく、ある option の lifetime がまだ有効かどうかを定める guard に留まる。
- `GuardedRef` のような convenience vocabulary を将来導入する場合でも、その意味は「reference と、それに付随する guarded option chain」の sugar として解釈されるべきであり、独立した core primitive を増やすものではない。
- preference chain は一次 primitive ではなく、nested fallback を曖昧さなく書き下した正規形として扱う。`A > B > C` は、「`A` が explicit failure または monotone degradation で使えなければ `B`、さらに使えなければ `C`」という意味に読む。
- したがって fallback の推移律は、`A > B` と `B > C` が同じ論理的 access path 上の monotone degradation を表している限り、`A > B > C` への正規化が nested fallback と同じ outcome discipline を保つ、という**最小読解**でだけ扱う。
- chain の各段は、それより前の option から見て同等以下の guarantee しか持たない monotone degradation でなければならない。より強い capability への再昇格は許されない。
- `lease` の期限切れは、その option が以後の評価で success-side choice になれないことを意味する。期限切れは monotone degradation の一種であり、同じ semantic lineage の後段からその option を再び有効化してはならない。
- `try` の local rollback は current `place` の state を巻き戻せても、期限切れた `lease` や、すでに確定した degradation order を復活させてはならない。rollback は local state に作用するのであって、monotone lifetime order を逆転させるものではない。
- `atomic_cut` も `lease` を生成したり解除したりしない。`atomic_cut` の前後を問わず、期限切れや degradation で後段 option へ進んだなら、その lineage で earlier option へ暗黙に戻ってはならない。
- 書き込みは、現在選ばれている option の contract が write を許す場合にだけ成立する。より後段の option が read-only なら、fallback によって write capability も単調に弱くなりうる。
- write-capable option の `lease` が期限切れた場合、その option への write は成功してはならない。後段に write を許す option があり、かつ contract がその degrade を許すなら明示的 fallback でそちらへ進める。そうでなければ結果は explicit `Reject` である。
- 期限切れ後の write を hidden buffering や hidden resurrection で吸収してはならない。必要なら explicit fallback、explicit compensation、または explicit failure として可視でなければならない。
- 後段 option での成功が元の option より弱い guarantee を持つなら、その outcome は contract が許す範囲でのみ admissible である。正確に `Success` / `Approximate` のどちらへ分類するかは contract surface に従う。

### canonical normalization law（L2）

- canonical form は、単一の論理的 access path と単一の semantic lineage に属する fallback 候補を、優先順に並べた有限列 `o1 > o2 > ... > on` である。
- 各 option `oi` が canonical form に保持する最小情報は、access target、contract surface、lifetime guard (`lease`)、およびその option 自身で許される capability surface である。fallback successor は別 field ではなく、列内で直後の option が successor を表す。
- canonical form が一意に表すのは、同じ logical access path 上で観測される「候補の優先順」と「各候補に付随する guard / contract / capability」である。元の nested syntax における内側 / 外側の位置そのものは保持しない。
- 正規化は left-to-right flattening として与える。単独 option の canonical form はその singleton chain であり、`fallback(x, y)` の canonical form は `canon(x) ++ canon(y)` と読む。
- ただしこの law が適用できるのは、`x` と `y` が同じ logical access path / semantic lineage 上の候補を表し、かつ `canon(y)` の各段が `canon(x)` の後段として monotone degradation を保つ場合に限る。正規化は incompatible branch を黙って並べ替えたり修復したりしない。
- したがって `fallback(fallback(A, B), fallback(C, fallback(D, E)))`、`fallback(A, fallback(B, fallback(C, fallback(D, E))))`、および同じ順序を保つ他の nested form は、すべて同じ canonical chain `A > B > C > D > E` を表す。
- canonical chain の評価は leftmost viable option を選ぶ読みである。`A > B > C > D > E` において `A` と `B` が期限切れまたは explicit failure なら、最初に admissible な `C` が選ばれる。内側 / 外側の syntactic nesting はこの選択に追加の意味を与えない。
- write-after-expiry も同じ law に従う。先頭の write-capable option が期限切れたとき、canonical form は earlier option へ戻らず、次の write-admissible option へ明示的に進むか、該当候補がなければ `Reject` になる。
- `try` / rollback / `atomic_cut` は canonical form 自体を並べ替えない。これらは current `place` の state や rollback frontier を制約するが、canonical chain が表す優先順と degradation order は保存される。

### incompatible branch の rejection phase（L2）

- incompatible branch とは、canonical chain の同一列に載せる前提を満たさない nested fallback branch を指す最小補助語である。少なくとも次を含む。
  - same logical access path / semantic lineage を共有しない branch
  - 後段が前段より強い guarantee や capability を要求し、monotone degradation を壊す branch
  - 後段 option が前段 option の contract-compatible fallback successor になれないことが、declared contract surface / capability surface だけで分かる branch
- 上の incompatibility が declared access target、documented logical access path / semantic lineage 関係、declared contract surface、declared capability surface だけで判定できるなら、その nested fallback は evaluation に入る前に static rejection されるべきである。これは canonical law の失敗ではなく、「その branch は canonical form を持つ well-formed preference chain ではない」という判定である。
- dynamic `Reject` は、canonical chain 自体は well-formed だが、current evaluation で admissible option が尽きた場合の明示的 outcome として残す。`lease` 期限切れ、`require` 不成立、explicit failure、write-after-expiry 後に後段の write-admissible option が存在しない場合はこの側に属する。
- したがって static rejection は malformed fallback nest を弾くための前段判定であり、dynamic `Reject` は well-formed chain の runtime failure を表す。両者を混同して hidden repair や hidden backtracking を導入してはならない。
- `try` / rollback / `atomic_cut` は incompatible branch の分類そのものを変えない。static rejection される branch は rollback や cut の前後を問わず canonical chain に入らず、dynamic `Reject` は従来どおり explicit failure outcome として既存の rollback / cut discipline に従う。

### static evidence floor（L2）

- same logical access path / semantic lineage を static に扱うための最小 declared information は、各 option の `declared access target` と、fallback successor が前段 option と同じ semantic lineage を継続することを示す `documented lineage annotation` である。
- `declared access target` だけでは足りない。同じ target 風に見える option でも、same logical access path / semantic lineage を共有しているとは限らないためである。したがって current L2 では、static same-lineage 判定には `documented lineage annotation` を併せて要求する。
- contract-compatible fallback successor を static に扱うための最小比較対象は、前段 option と後段 option の `declared contract surface` と `declared capability surface` である。current L2 の最小読解では、後段 option が前段 option より強い capability を要求せず、かつ declared contract surface に successor 使用を明示的に打ち消す矛盾がないときだけ、static な successor 候補として残してよい。
- したがって current L2 で static rejection に寄せられるのは、少なくとも次である。
  - `declared access target` が一致しない場合
  - `documented lineage annotation` が same-lineage / same-path を否定する場合
  - `declared capability surface` の比較だけで後段が前段より強い capability を要求すると分かる場合
  - `declared contract surface` の比較だけで後段 option が fallback successor として明示的に矛盾すると分かる場合
- underdeclared fallback case とは、最小 declared information のどこかが欠けていて、same-lineage 判定または successor 判定を current L2 の floor まで持ち上げられない branch を指す。少なくとも次を含む。
  - `declared access target` はあるが `documented lineage annotation` がない場合
  - `declared access target` 自体が不足している場合（`documented lineage annotation` の有無を問わない）
  - `declared contract surface` または `declared capability surface` が不足し、contract-compatible fallback successor かどうかを最小比較できない場合
- current L2 では、これらの underdeclared fallback case は surface-level static error とする。理由は、canonical chain へ入るための最小証拠が欠けている以上、hidden acceptance も dynamic `Reject` への先送りも許さないためである。
- `elaboration obligation` は current L2 の admitted path には含めない。将来導入する場合でも、それは canonicalization や evaluation の前に上の static evidence floor を満たす形で discharge されなければならず、underdeclared branch を hidden に same-path 扱いする根拠にはならない。
- dynamic `Reject` に残るのは、上の最小 declared information だけでは incompatibility が証明されず、canonical chain 自体は well-formed だが current evaluation で admissible option が尽きる場合である。`lease` 期限切れ、current `require` failure、explicit failure、write-after-expiry 後の後段不在は従来どおりこの側に属する。
- **未決定**: `documented lineage annotation` の具体的な surface form と、その証拠を same-place / cross-place 文脈でどう書くか。
- **未決定**: `declared contract surface` の「明示的矛盾」を、`require` / `ensure` / `invariant` のどの粒度まで static に比較するか。
- **未決定**: `declared capability surface` を read / write 以上に広げずに扱えない mixed case を、将来 static rejection に寄せるか dynamic `Reject` 側に残すか。
- **未決定**: future extension として `elaboration obligation` を認める必要が本当にあるか。認めるなら、どの underdeclared case まで discharge 対象にしてよいか。

## Mir-0 の外に意図的に置く論点

- `durable_cut` は Mir-0 に含めない。Mir-1 側では、`atomic_cut` に abstract persistence requirement を伴う durable commit guarantee を追加する cut vocabulary 候補として扱う。
- `durable_cut` の最小意味は、successful として観測された pre-cut prefix が local rollback、process restart、route rebinding の後に未確定状態へ戻らないことである。Mir-1 はこの guarantee を意味づけるが、具体的な storage / replication / consensus mechanism は固定しない。
- `durable_cut` の failure は、durable guarantee を確立できず、その cut attempt を成功した cut として確定できなかったことを意味する。この段階では既存の failure lattice を再利用し、新しい failure class は導入しない。
- 既定では、durable guarantee を確立できなかった `durable_cut` は `Reject` として扱う。cut 後の外部化された obligations を明示的に巻き戻す必要がある場合だけ `Compensate` を使う。`Approximate` は contract が durability を弱めた代替を明示的に許す場合を除き、`durable_cut` failure の既定分類にはしない。
- local durable failure と distributed durable failure は、Mir-1 では別 failure class に分けない。どちらも同じ durable guarantee failure の実現上の差分であり、後者の distributed finalization は Mirrorea 側の実現責務に属する。
- distributed finalization は `durable_cut` の最小意味そのものではない。複数 `place` や実ノードをまたぐ `durable_cut` を成立させるときの operational realization 側に属する。
- Mirrorea は `durable_cut` を意味づける場所ではない。Mir-1 で定められた durable commit guarantee を、storage / replication / distributed cut realization / audit 上で実現する責務を持つ。
- `barrier` は Mir-0 の cut vocabulary に含めない。Mir-1 に残すとすれば、commit-like primitive ではなく ordering primitive 候補として扱う。
- **未決定**: `barrier` が Mir-1 で独立語彙として本当に必要か、それとも他の ordering / cut 構成に吸収されるか。
- **未決定**: `durable_cut` の persistence evidence をどの形式で観測・検証するか。
- **未決定**: contract が durability を弱めた代替結果を本当に許す場合、その degraded path を `Approximate` としてどこまで許容するか。
- **未決定**: `lease` を surface syntax / contract language でどの token にするか。また `GuardedRef` のような convenience vocabulary を本当に導入するか。
- **未決定**: preference chain の完全 normalization law、full algebra、および `documented lineage annotation` / `declared contract surface` / `declared capability surface` の詳細比較規則をどこまで固定するか。
- **未決定**: `lease` 期限切れの観測面を dedicated event として持つか、それとも fallback / failure 側の既存 event だけで表すか。
- **UNRESOLVED**: `emit`、effect handler、structured event routing の正確な関係。
- **UNRESOLVED**: suspension restriction と cut / rollback / patching との相互作用を含む coroutine support。
- **UNRESOLVED**: overlay alias detail、route rebinding detail、およびその他の Mirrorea 依存の safe-evolution mechanism。

これらはより広い Mir の設計空間には属しているが、この段階で固定する最小 kernel には含めない。

## Mir が何ではないか

- それは deployment fabric と同一ではない。
- それは route / overlay の制御プレーン（control plane）と同一ではない。
- それは domain-specific media runtime ではない。
- それは virtual reality engine ではない。
- それは type system だけではなく、operational boundary と evolution rule も含む。
