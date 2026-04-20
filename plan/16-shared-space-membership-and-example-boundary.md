# plan/16 — shared-space membership と example boundary

## 目的

この文書は、shared space / session membership / participant churn を含む上位層の議論について、

- current repo で **どこまで self-driven に詰めてよいか**
- どこから先は **user 仕様確認が必要か**
- practical example を使うと **どの layer の責務が見えやすいか**

を長期参照しやすい形で整理する。

ここで扱うのは **current working boundary** であって、L3 shared space や L5 application の最終規範仕様ではない。
規範判断の正本は引き続き `specs/` に残る。

## source-backed な現在地

### 規範 source から読めること

- `specs/03-layer-model.md`
  - L3 の責務は `session / space abstraction`、`shared object` の同期モデル、consistency mode の選択、space 間 movement / linkage である。
- `specs/05-mirrorea-fabric.md`
  - Mirrorea は logical naming、routing、overlay、audit、reconfiguration を担う。
  - route change を単一の consensus algorithm に強く結びつけて仕様化してはならない。
- `specs/11-roadmap-and-workstreams.md`
  - 将来 workstream G では、小さな synchronized shared-space 例を少なくとも 1 つ扱う。
- `specs/01-charter-and-decision-levels.md`
  - 単一の consensus algorithm に commit しない。
- `specs/10-open-questions.md`
  - 上位空間、virtual reality social mode、synchronized web browsing、Reversed Library は未決のまま残る。

### current plan / report chain から読めること

- `plan/12-open-problems-and-risks.md`
  - dynamic membership / causal metadata は OPEN / FUTURE である。
  - participant churn を plain vector clock deletion に押し込むと、membership change と causal history が混線しやすい。
- `docs/reports/0111-faq-unresolved-issues-current-state.md`
  - participant churn は current L2 mainline の外にある serious open problem であり、membership reconfiguration と causal metadata を分離する方向が repo の explicit boundary 原則と整合する。

## current working boundary

### いま言ってよいこと

1. participant 集合を **Mir core の plain array 型** として固定するのは早い。
2. participant churn は **session membership** と **causal metadata** を分けて扱う方が自然である。
3. join / leave / rejoin は、同じ `member_id` を再利用できるとしても、少なくとも **incarnation** またはそれに相当する再参加識別子を持つ方が自然である。
4. shared-space の consistency mode は application rule に依存する。
5. `Raft` / `Paxos` / その他の consensus family は **Mir language spec ではなく Mirrorea / shared-space の operational realization 側**に置くべきである。

### まだ言ってはいけないこと

1. participant 集合の final carrier 名
2. final parser syntax
3. activation rule の final profile
4. authoritative single-writer を全 shared-space の既定規則にすること
5. vector clock / dotted version vector / matrix clock のどれかに commit すること
6. `Raft` / `Paxos` / `VRR` / CRDT のどれかを唯一の正解として固定すること

## current staged sequencing

shared-space line の docs-first reorder としては、少なくとも次の順で narrow に進めるのが自然である。

1. membership identity core
2. admission / compile-time visibility
3. authority / resource ownership
4. small room-profile working subset
5. fairness / replay strengthening
6. final operational catalog / algorithm binding / app-specific profile

ここで 1〜4 は current docs-first reopen chain と整合しており、
4 は authoritative room baseline / append-friendly contrast room / first default profile の current package 読みと矛盾しない。
5 は reserve strengthening / vertical-slice ratchet、
6 は user-spec-required gate と読む。

### fairness / replay mixed-gate boundary の current reading

- `specs/examples/432` と `specs/examples/448` で、fairness / replay line は final operational catalog ではなく mixed-gate boundary までを current cut と読む line を追加した。
- `specs/examples/467` で、first actual adoption profile は
  - `authority-ack`
  - `single room authority`
  - `authoritative serial transition`
  - `authority_rng`
  - late join visible history as past
  - stale reconnect fail then refresh
  - replay invalidation rather than silent merge
  - no distributed fairness theorem required in first completion line
  を current default に上げた。
- `specs/examples/570` で、`run-source-sample` helper summary の `authoritative_room_first_scenario_actual_adoption` に
  - `p07 / p08` reached representative pair
  - `p09` delegated RNG reserve route
  - `p13 / p14` late-join negative static-stop pair
  を actualize し、first actual adoption profile の practical meaning を CLI / artifact / docs で drift なく読める cut に揃えた。
- `specs/examples/571` で、`run-source-sample` helper summary の `authoritative_room_reserve_strengthening_lane` に
  - `p07` witness strengthening + model-check second line
  - `p08` model-check second line
  - `p09` delegated RNG practical route + model-check second line
  - `p05` guard-only
  を separate status のまま actualize し、first completion line と reserve package 群の boundary を drift なく読める cut に揃えた。
- preserve する principal axes は、
  authority placement、
  provider placement、
  witness requirement、
  fairness source / trust model、
  replay attachment、
  payload / observation visibility
  の 6 つである。
- replay / fairness / provider-receipt family は主に `protocol_verifier_boundary` と `runtime_policy_boundary` に残し、room-core semantics や checker floor に collapse しない。
- concrete replay protocol/profile、distributed fairness protocol、concrete authority binding、`control_epoch` actualization timing は still later に残す。
- current snapshot では、first actual adoption profile と helper-local vertical-slice ratchet、authoritative-room first scenario helper-summary tightening、authoritative-room reserve strengthening lane tightening、`auditable_authority_witness` strengthening、`delegated_rng_service` practical actualization、witness-provider-artifact public-shape actual adoption、witness/provider public-contract / emitted-handoff coupled-later gate、witness/provider public-schema coupled-later gate、witness/provider route actual adoption、witness/provider schema route actual adoption、witness/provider final public-contract reopen threshold、order-handoff / witness-provider final public-seam compression、order-handoff/shared-space public-seam helper mirror、phase4 shared-space self-driven closeout threshold helper mirror、phase5 proof/protocol/runtime-policy handoff closeout threshold helper mirror、guided sample entrypoint closeout、Problem 2 residual bundle matrix closeout、authoritative-room scenario bundle actualization、parser-side companion surface bundle、parser-side bundle-to-helper bridge、parser-side request-head / clause-bundle inspector、parser-side representative mapping matrix、explained representative problem sample bundles、representative problem bundle smoke runner、representative problem bundle aggregate smoke summary、representative problem bundle failure-focused smoke diagnostics、representative problem bundle quickstart walkthrough hardening、representative problem quickstart CLI mirror、representative problem quickstart parity checks は fixed 済みであり、shared-space side の remaining self-driven line は Package 114 representative problem mixed-gate reopen map refresh と final public witness/provider/artifact contract actual adoption mixed gate、exhaustive catalog/user-spec residual に移った。
- したがって fairness / replay stronger package が mixed/user-spec gate に残っていても、theory-lab queue 全体を空と読まない。

### confusion / replay hardening note

- future docs では `logical_member_id`、`incarnation`、admission decision、authority holder、replay receipt / refusal を separate concern として扱う方がよい。
- stale route、stale membership snapshot、stale replay receipt、new incarnation を一つの generic `invalid participant` bucket に潰さない方が confusion を減らせる。
- exact threat table と concrete protocol profile は still OPEN である。

## order / handoff minimal scenario の current reading

shared-space line の order / handoff は、final protocol や final syntax を固定せずに、
**authoritative room の minimal scenario** から docs-first に比較するのが自然である。

### preserve すべき軸分解

- authority placement
- provider placement
- witness requirement
- fairness source / trust model
- replay attachment
- payload / observation visibility

この 6 軸は 1 つの `room profile` field に潰さない。
特に provider placement と witness requirement は、current repo では別軸に保つ。

### docs-first scenario family

1. authoritative game room
   - A rolls
   - publication
   - handoff to B
   - B sees that handoff implies the roll result is already in the past / visible / witnessed
2. same-process / shared-memory analog
   - same causal pattern を thread-local / place-local lowering に落とした比較材料
3. rollback before local finalization
4. rollback after local finalization
5. snapshot-only observation cut vs durable-cut difference
6. late join / rejoin / stale message
7. handoff-before-publication / handoff-without-witness negative
8. duplicate receipt / stale receipt / epoch mismatch negative
9. provider-authority mismatch and fairness-fails-but-safety-holds negative

### falsifier reading

- positive motivating scenario だけでは authority-handoff family を過大に読みやすい。
- current docs-first hardening では、
  - publication omission
  - witness omission
  - replay / epoch drift
  - provider / authority collapse
  - fairness overclaim
  を separate negative family として保つのが自然である。

### current judgment

- authoritative room scenario は current shared-space docs-first line と整合する。
- ただし、authoritative room を shared-space 全体の exhaustive final catalog と同一視しない。
- `atomic_cut` は local finalization の nucleus に留め、room-level authority handoff や snapshot-only observation を同一 primitive として説明しない。
- same-process analog は useful comparison material だが、shared-memory lowering と distributed authority-handoff lowering の差まで消してはならない。
- source-facing companion wording では explicit edge-row / vertical continuation を principal に保ち、`stage` / `after` / `witness` family は secondary candidate、`serial` sugar は authoritative-room-specific reserve に留める。
- `specs/examples/533` により、`p07 / p08` reached・`p09` guard の order-handoff/shared-space public-seam residual も `run-source-sample` helper summary で inspectable に保てる。
- first actual adoption profile は current default に上げてよいが、
  exhaustive final catalog と stronger fairness / witness / provider profile は mixed gate / user-spec gate に残す。
- `run-source-sample` helper summary では first scenario summary を direct に見せてよいが、これは final shared-space catalog や final public witness/provider/artifact contract を意味しない。

## participant を array-like にするか

### 結論

**source of truth としての plain array は避け、array-like view は derived snapshot に留める**のが current working model である。

### 理由

plain array を source of truth にすると、少なくとも次が混ざりやすい。

- membership の存在判定
- activation / deactivation の lifecycle
- ordering / index
- 再参加時の identity
- path / routing 参照
- causal metadata の actor key

これらは同じではない。
特に participant churn があると、`[A, B, C]` から `C` を消した後で古い message が届いた時に、

- 「古い C の message」
- 「新しく join した C」

を array だけでは区別しづらい。

### current working model

source of truth は概念上、次のような **session-scoped membership registry** として考える方が自然である。

```text
SessionMembers<Role> = {
  members: {
    logical_member_id -> {
      role_tag,
      incarnation,
      activation_state,
      route_ref?,
      joined_at?,
      left_at?
    }
  },
  membership_epoch
}
```

ここで大事なのは field 名ではなく、**役割の分離**である。

- `logical_member_id`
  - 長期的に同じ参加者を指したい時の識別子
- `incarnation`
  - 再参加を古い参加と区別するための識別子
- `activation_state`
  - 参加宣言済みだがまだ active ではない、leave 済み、などを区別する
- `membership_epoch`
  - membership change の境界を表す control-plane 側の世代

array-like な一覧が欲しいなら、これは

```text
active_members_snapshot(space)
```

のような derived view として取り出す方がよい。

## participant は tree-like JSON とみなしてよいか

### 直感として自然な点

user-facing には、

- room
  - participants
  - board state
  - notices

のような **tree-like document** で shared space を眺めたくなる。
これは自然であり、snapshot serialization や UI state としてはむしろ分かりやすい。

### それでも source of truth を registry に寄せる理由

理論上の問題は、participant が単なる「木の 1 ノード」ではなく、少なくとも次の役割を同時に持つことである。

- membership identity
- activation / deactivation の lifecycle
- rejoin を区別する incarnation
- routing / authority / audit の参照先
- causal metadata と整合する actor identity
- UI 上の list / tree / map など複数 view への投影元

tree source-of-truth にすると、これらを structural position で同一視しやすい。
しかし current repo が気にしているのは structural equality ではなく、

- **同じ participant か**
- **同じ incarnation か**
- **現在 active か**
- **どの authority / route / audit context に属するか**

である。

特に rejoin があると、

```text
room
  participants
    - C
```

のような木だけでは、

- 古い `C`
- 新しい `C`

を hidden identifier なしに区別しづらい。

### current working judgment

- **tree-like 参加者表現自体は否定しない**
- ただしそれは **derived snapshot / UI / serialization view** に留める
- source of truth は引き続き **session-scoped membership registry**

とする方が、identity / activation / incarnation / causality を混ぜずに済む。

要するに、**tree view は見え方として自然だが、理論上の carrier は registry の方がきれい**である。

### event tree / leaf-to-root event bubbling はどう扱うか

tree-like room view を前提にすると、leaf から root へ event を上げる Elm 的な execution view は直感的である。
current working judgment では、これも source-of-truth carrier ではなく **derived execution / debug / explanation view** として比較するのが自然であり、membership registry、authority slot、transition log を直接 tree topology に埋め込まない方が理論上きれいである。

## vector clock から participant を消す問題

### 問題の芯

`leave` 後に actor key を単純削除すると、古い message を受け取った側が

- 「古い incarnation の event」
- 「新しい join」

を区別できない。

これは plain vector clock の問題というより、**membership change と causal metadata を同じ carrier に押し込んでいること**が問題である。

### 比較したい 3 案

1. plain vector deletion
2. epoch / incarnation split
3. control-plane separated carrier

### 案A — plain vector deletion

#### 読み

- participant ごとに causal slot を持つ
- leave 後は slot を削除するか undefined 扱いにする

#### 利点

- 最も単純に見える
- churn が少ない closed-world なら直感的

#### 欠点

- old message と new join を区別しにくい
- historical actor identity を消しやすい
- membership change と causality を同じ carrier に押し込む

### 案B — epoch / incarnation split

#### 読み

少なくとも conceptual には、次を分ける。

1. membership reconfiguration / activation event
2. causal metadata

そのうえで、message / event 側は少なくとも

- `membership_epoch`
- `logical_member_id`
- `incarnation`

のどれか、できれば全部に相当する情報と整合する形で読む。

#### 利点

- reconnect / in-flight action invalidation line と自然に接続できる
- old incarnation と rejoined incarnation を区別しやすい
- current `member_incarnation` working line と整合する

#### 欠点

- carrier が 1 段増える
- epoch と incarnation の relation を audit / docs で説明する必要がある

### 案C — control-plane separated carrier

#### 読み

- activation / leave / rejoin / authority handoff は control-plane log が authoritative
- data-plane event は current active config / incarnation reference を持つ

#### 利点

- membership reconfiguration と data-plane causality を最もきれいに分けられる
- authority handoff / reconnect / late join を control-plane に集約しやすい
- old message の解釈を config boundary で fail-closed にしやすい

#### 欠点

- carrier と audit path がさらに 1 段増える
- current phase では最も operational realization 寄りである

### 何が防げるか

たとえば、

1. `C` が `incarnation = 4` で active
2. `leave(C, 4)` が成立
3. A はそれを知っている
4. B はまだ知らず、古い `C(4)` に言及した message を送る

という時でも、A はその message を見て

- 「B が古い membership view で話している」

と読める。

ここで **新規参加の成立条件を explicit join / activation event の受理に限定**しておけば、
古い message だけで `C` を復活させる必要がない。

### current working judgment

- **plain vector deletion は current first choice にしない**
- **epoch / incarnation split を first practical candidate に置く**
- **control-plane separated carrier は next stronger candidate として比較を続ける**
- **current threshold judgment としては、authority handoff / provider binding / activation frontier を room rule 側へ上げない限り、この stronger candidate を current default に reopen しない**
- **ただし reopen するなら、first cut は full control-plane log ではなく `control_epoch` 相当の lightweight split に留める**

したがって current repo では、少なくとも

- `membership_epoch`
- `member_incarnation`

を分けて考え、leave / rejoin を causal metadata の slot deletion そのものとは同一視しないのが最も自然である。

### 何がまだ OPEN か

- causal metadata の final carrier
- membership epoch と route / patch epoch を共有するか
- control-plane separated carrier を reopen する threshold
- deactivation を immediate に visible にするか
- explicit leave が届かなかった場合の扱い

## activation を compile-time にどこまで見抜けるか

### 問題

join / leave / rejoin の active 化について、

- 「誰がその変化を知る必要があるか」
- 「誰の承認が visibility 条件になるか」

を compile-time に先回りして決められないか、という方向性は自然である。

### current working answer

**closed-world の範囲なら compile-time で over-approximation はできるが、runtime control-plane を不要にはできない**。

たとえば、

- room authority role
- declared observer role
- replicated observer class
- publish / notify の宛先

が declaration で固定されているなら、

```text
activation_visibility(room) =
  authority_role
  + declared_observer_roles
  + replicated_observer_class
```

のような「知る必要のある候補集合」は静的に絞れる。

しかし open room / late join / reconnect があると、compile-time 時点では

- future participant
- rebinding 後の authority replica
- reconnect で戻る observer

を closed に列挙できない。

### したがって何が言えるか

compile-time にできるのは主に次である。

1. **どの role が activation visibility に関与しうるか** の宣言
2. authority-ack / full-coverage-like activation / quorum-like activation の policy 候補の structural floor
3. publish / subscribe / notify の required path の over-approximation

一方で runtime に残るのは次である。

1. 実際の active member 集合
2. late join / reconnect の反映
3. authority handoff 後の可視 participant
4. in-flight action と membership change の衝突解決

### current working judgment

- compile-time は **activation visibility policy の over-approximation** に使ってよい
- ただし **actual activation / dissemination / reconciliation は runtime control-plane に残す**
- なお `full-coverage-like activation` / `quorum-like activation` は、Mir-1 `durable_cut` の `all_of` / quorum-like profile と**似た family comparison を shared-space 側に持ち込む**という意味であって、同じ語彙として固定するわけではない

これが current repo の layer separation に最も整合する。

## activation rule の候補比較

### 比較したい 3 案

current phase で比較対象として持つのは、少なくとも次の 3 案である。

1. `authority-ack`
2. `full-coverage-like activation`
3. `quorum-like activation`

ここで重要なのは、これは **shared-space membership activation の operational policy** であって、

- Mir core の value-level ownership
- Mir-1 `durable_cut` の exact profile 名
- final consensus algorithm

そのものではない、という点である。

### 案A — `authority-ack`

#### 読み

- join / leave / rejoin は room authority が明示的に受理した時点で active / inactive に遷移する
- その後、authority が visibility 対象へ dissemination する

概念上はたとえば次のように読む。

```text
join_request(C, incarnation = 5)
  -> room_authority accepts
  -> activation_event(C, 5, membership_epoch = 12)
  -> publish_to activation_visibility(room)
```

#### 利点

- current repo の layer separation に最も素直
- participant carrier と authority placement を分けたまま書ける
- authoritative room で single transition order を説明しやすい
- compile-time では `activation_visibility(room)` の over-approximation だけ見ればよい

#### 欠点

- authority の trust / liveness に依存する
- dissemination 遅延中は「authority は知っているが他 participant はまだ知らない」状態が生じうる
- replicated authority に進むと、authority group 内合意を別に要する

#### 向く room

- authoritative game room
- lock / reset / winner notification のように single transition order が重要な room

### 案B — `full-coverage-like activation`

#### 読み

- active 化は authority だけでなく、current activation visibility に入る全対象への反映が揃ってから成立する

概念上はたとえば次のように読む。

```text
activation_event(C, 5, epoch = 12)
  -> all known activation_visibility(room) acknowledge
  -> C becomes active
```

#### 利点

- 「active なのに必要 participant がまだ知らない」という状態を減らせる
- fixed small room では直感的に分かりやすい
- activation と visibility をほぼ一致させられる

#### 欠点

- churn / late join / reconnect に弱い
- 「誰が current activation visibility に入るか」を runtime で精密に追う必要がある
- participant 集合が動くと、成立条件そのものが揺れやすい
- availability より global acknowledgement を優先するので、current phase では重い

#### 向く room

- closed membership に近い small room
- active 化前に全員既知が本当に必要な workflow

### 案C — `quorum-like activation`

#### 読み

- active 化は authority group または activation committee の threshold / quorum で成立する

概念上はたとえば次のように読む。

```text
activation_event(C, 5, epoch = 12)
  -> quorum(activation_committee) acknowledges
  -> C becomes active
  -> lagging replicas / observers catch up later
```

#### 利点

- availability を取りやすい
- replicated authority と相性が良い
- single authority failure を前提にしなくてよい

#### 欠点

- participant room の active 化規則が consensus flavor に強く寄る
- straggler / catch-up / stale observer の扱いが必要になる
- current repo phase では activation だけのために carrier が重くなりやすい

#### 向く room

- operationally replicated authority を前提にした room
- future workstream としての distributed control-plane

### current working judgment

- **authoritative room の最小 operational candidate は `authority-ack`**
- `full-coverage-like activation` と `quorum-like activation` は **policy option comparison** に残す
- final activation profile はまだ固定せず、必要なら `authority-ack` の上に overlay 的な room policy として載せる方向を first practical reading に置く
- compile-time が持つべきなのは
  - `activation_visibility(room)` に関与しうる role 宣言
  - required publish / notify path の over-approximation
  までであり、
  actual active set と actual acknowledgement frontier は runtime control-plane に残す
- auth / admission layer が後から重なることも考えると、activation rule を language core の value-level rule として早く固定するより、shared-space / runtime / compile option 側の policy layer に留める方が current phase では自然である

### room 例との相性

#### authoritative game room

- current first choice は `authority-ack`
- 理由は、turn order / roll lock / winner reset を single transition order で説明しやすいから
- `full-coverage-like activation` は closed tournament room なら候補になるが、open join / leave には重い

#### append-friendly room

- membership 自体は `authority-ack` でもよい
- ただし append visibility は activation rule と別に、append-friendly consistency mode で流す方が自然
- つまり **membership activation と data-plane visibility を同じ acknowledgement rule にしない** 方が current repo の分離に合う

## practical example A — authoritative すごろく room

### 問題設定

user から出た例を current architecture に沿って整理すると、概ね次である。

- 誰でも途中参加できる
- 抜けた人は list から消える
- 盤面は全体で 1 つ
- サイコロ抽選中は誰も触れない
- 出目の数だけ進む
- goal を踏み越えたら goal に止まる
- 誰かが goal したら全員に winner を通知し、その後 reset
- 全員の位置はリアルタイムで見える

### この例が示すこと

この例は、**shared mutable state + exclusive action + membership churn + global reset** を同時に持つ。
したがって、eventual merge より **authoritative transition log** の方が自然である。

### layer 分担

| layer | この例で担うもの |
|---|---|
| Mir core | action の causal order、`require` / `ensure`、failure、cut 的境界 |
| Mirrorea | room authority への routing、path proof、audit、rebind |
| L3 shared space | room session、membership activation、shared board state、consistency mode |
| L5 app rule | すごろくの移動ルール、winner 通知、reset |

### 概念疑似コード

以下は **final syntax ではなく概念疑似コード** である。

```text
space SugorokuRoom {
  membership players: SessionMembers<Player>

  state positions: ByActiveMember<Player, CellIndex>
  state roll_lock: Idle | Rolling { owner: MemberRef<Player> }
  state winner: None | Some<MemberRef<Player>>

  on join(player) {
    activate players[player]
    if positions[player] is undefined {
      positions[player] := 0
    }
    publish snapshot(players.active(), positions, winner)
  }

  on leave(player) {
    deactivate players[player]
    delete positions[player]
    publish snapshot(players.active(), positions, winner)
  }

  action roll_and_move(player) {
    require players.is_active(player)
    require winner == None
    require roll_lock == Idle

    roll_lock := Rolling { owner: player }

    perform via dice_rng
      ensure result in 1..6

    atomic_cut

    positions[player] := min(GOAL, positions[player] + result)
    roll_lock := Idle

    if positions[player] == GOAL {
      winner := Some(player)
      notify_all(goal_reached(player))
      reset_all_positions_to_start()
      winner := None
    }
  }
}
```

### この形で保証したい性質

- `roll_lock == Idle` の間だけ次の action が始まる
- `Rolling` 中に二重実行しない
- active member だけが move できる
- leave 済み participant の古い message だけで復活しない
- winner 通知と reset は同じ authoritative transition sequence で説明できる

### ここでまだ未決な点

- `notify_all(goal_reached(player))` と reset の間に別 event を許すか
- `atomic_cut` に相当する境界を app rule で要求するか、単なる authoritative log entry で足りるか
- RNG を trusted authority に持つか、commit-reveal 的に分散するか
- authority を single / replicated のどこで切るか

## practical example B — shared notice board / presence room

### この例を置く理由

すごろく room は authoritative single transition が自然だが、すべての shared-space がそうではない。
対照として、append-only に近い room を置くと consistency mode の選択問題が見えやすい。

### 問題設定

- 誰でも join / leave できる
- active participant list が見える
- 誰でも note を 1 行ずつ append できる
- note は後から全員に見える
- simultaneous append は許す

### 概念疑似コード

```text
space NoticeRoom {
  membership participants: SessionMembers<User>
  state notes: AppendOnlyLog<Note>

  on join(user) {
    activate participants[user]
    publish snapshot(participants.active(), notes.tail())
  }

  on leave(user) {
    deactivate participants[user]
    publish snapshot(participants.active(), notes.tail())
  }

  action append_note(user, text) {
    require participants.is_active(user)
    perform append(notes, Note { author: user, body: text })
  }
}
```

### この例で見えること

- membership registry は同じでも、
- board state は authoritative lock が自然な場合と、append-only merge が自然な場合がある。

つまり **participant carrier と consistency mode は分けて考えるべき**である。

## membership model の比較

### 案A — plain array を source of truth にする

```text
players: [PlayerNode]
```

#### 利点

- 見た目が簡単
- UI 的には扱いやすい

#### 欠点

- leave / rejoin の区別が弱い
- index と identity が混ざる
- causal metadata と連携しにくい
- authority / routing / activation の責務が不明瞭になる

#### 向く保証

- ごく短命で membership churn がほぼない demo

### 案B — session membership registry を source of truth にする

```text
players: SessionMembers<Player>
active_view(players): [MemberRef<Player>]
```

#### 利点

- join / leave / rejoin を分けやすい
- array-like UI view を後段で作れる
- causal metadata と membership change を分離しやすい
- authoritative room / relaxed room の両方に使える

#### 欠点

- 最終 carrier 設計がまだ必要
- activation / deactivation rule を別に決める必要がある

#### 向く保証

- participant churn を含む shared-space 全般

### 案C — consensus-coupled replicated membership object を source of truth にする

```text
players: ReplicatedMembership<Player, ConsensusConfig>
```

#### 利点

- authoritative room と相性が良い
- activation の global rule を強くできる

#### 欠点

- consensus algorithm 依存が強い
- Mir / Mirrorea / L3 の boundary を早く固めすぎる
- language spec ではなく operational realization に寄りすぎる

#### 向く保証

- すでに authority / replication strategy を決めた implementation 段階

### current working judgment

**いま採るべきなのは案B** である。

- array-like view は derived に残す
- consensus-coupled object は implementation / operations に残す

## authority / consistency / RNG の次段 research line

### authority

shared-space の authority は、少なくとも次を比較対象として持つのが自然である。

1. single room authority
2. replicated authority
3. relaxed / merge-friendly room

理論上重要なのは、「participant carrier」と「authority placement」を別軸に保つことである。
`SessionMembers` を採ったから single authority になるわけではなく、同じ carrier の上で

- authoritative すごろく room
- append-only notice room

を分けて持てる方が自然である。

### consistency mode

consistency mode は最初から広い catalog にせず、当面は

1. authoritative serial transition
2. append-friendly room

程度の小 catalog から始めるのが current working line である。

これにより、

- exclusive lock / reset / winner notification
- simultaneous append / eventual visibility

を同一 rule に押し込まずに済む。

### RNG / fairness

RNG を tree の各ノードが子方向へ自由に決める model は、完全に不可能ではないが、**base model としては重い**。
理由は、

- randomness source が topology と結びつきすぎる
- fairness をどの node が保証するかが不明瞭になる
- audit 時に「誰が seed / proof を支配したか」が追いづらい

からである。

したがって current working model では、RNG は tree topology に埋め込まず、**explicit provider capability** として切る方が自然である。

概念上はたとえば次の 3 候補がある。

```text
perform via authority_rng
perform via delegated_rng_service
perform via distributed_randomness_provider
```

- `authority_rng`
  - 最小
  - room authority が randomness を供給する
- `delegated_rng_service`
  - authority は rule を持つが、RNG 実体は別 service に委譲する
- `distributed_randomness_provider`
  - commit-reveal など後段候補

### `authority_rng`

#### 読み

- room authority 自体が randomness source を持つ
- roll / draw / random pick は authoritative transition sequence の一部として確定する

#### 利点

- current phase では最小
- authoritative game room と相性が良い
- lock / roll / move / reset を 1 本の audit sequence で追いやすい

#### 欠点

- fairness を authority trust に強く依存する
- entropy source の差し替えや HW 連携を authority 実装と切り離しにくい

### `delegated_rng_service`

#### 読み

- room authority は rule と commit point を持つ
- randomness 実体だけを別 service / provider に委譲する

概念上はたとえば次のように読む。

```text
room_authority requests random draw
  -> delegated_rng_service returns draw + audit reference
  -> room_authority commits transition
```

#### 利点

- authority placement と randomness provider placement を分けやすい
- HW entropy、外部 service、差し替え可能な debug provider を入れやすい
- portability / observability hook を replaceable layer に残しやすい

#### 欠点

- provider identity / audit reference / failure policy を別に持つ必要がある
- authority と provider の責務境界を曖昧にすると hidden trust shift が起きやすい

#### authoritative room 側の current practical cut

current phase では、authoritative room 側でも `delegated_rng_service` を次の cut で practical candidate として読んでよい。

- authority は request / lock / commit / publish の owner のまま
- provider は `draw_result` を返すが、room state mutation を commit しない
- `auditable_authority_witness` を使う場合でも witness core は

```text
witness_kind + action_ref + draw_slot + draw_result
```

  のままでよい
- provider receipt / draw ref は current witness core に入れず、audit / receipt side optional attachment に留める

この judgment は `specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md` に切り出した。

repo-level actualization としては、`specs/examples/477-current-l2-delegated-rng-service-practical-actualization.md` がこの provider-placement cut を helper-local practical manifest と narrow prototype `p09-dice-delegated-rng-provider-placement` に ratchet した。current reading は次である。

- reached sample は `p09-dice-delegated-rng-provider-placement`
- `p07-dice-late-join-visible-history` と `p08-dice-stale-reconnect-refresh` は authoritative-room baseline contrast として guard-only に留める
- first practical claim は `fairness_claim = opaque_authority_trust`
- provider receipt / draw ref は optional attachment に留める
- authority keeps commit / provider returns draw-not-state-transition を current helper-local cut に固定する
- final public provider receipt schema、delegated provider attestation、combined provider+witness public contract は still later に残す

### `distributed_randomness_provider`

#### 読み

- randomness 自体を multi-party / commit-reveal / threshold provider で作る

#### 利点

- authority trust 1 点への依存を弱めやすい
- stronger fairness claim の余地がある

#### 欠点

- membership churn / timeout / partial participation と強く結びつく
- activation rule、authority placement、late join / reconnect policy と同時に重くなる
- current repo phase では最も future-facing である

### room type ごとの current working line

#### authoritative game room

- current first choice は `authority_rng`
- next practical candidate は `delegated_rng_service`
- `distributed_randomness_provider` は future comparison に残す

#### append-friendly room

- shared RNG 自体が不要なことも多い
- 必要な場合でも `authority_rng` を room rule と密結合させるより、`delegated_rng_service` の方が切り分けやすい
- distributed randomness は still future comparison

### current working judgment

- **authoritative room の current first choice は `authority_rng`**
- **差し替え可能性 / HW 連携 / debug provider hook を意識するなら `delegated_rng_service` が next practical candidate**
- **`distributed_randomness_provider` は future research に残す**
- distributed randomness は default にせず、authority または delegated provider が必要に応じて明示的に接続する future option として残す

したがって current phase では、RNG / fairness source を tree topology や participant carrier に埋め込まず、

- authority placement
- resource owner slot
- provider placement

を別軸で保つのが最も自然である。

## fairness trust model をどこまで room profile に入れるか

RNG provider placement とは別に、`draw が fair だった` と誰がどこまで主張するか、という trust model も分けて考える必要がある。

### 比較したい 4 案

1. opaque authority trust
2. auditable authority witness
3. delegated provider attestation
4. distributed fairness protocol

### 案A — opaque authority trust

#### 読み

- authority が `draw` を決める
- audit には結果だけが残る
- fairness claim は authority trust に寄せる

#### 利点

- current authoritative room では最小
- rule / lock / move / reset を 1 本の transition history にまとめやすい
- implementation cost が最も低い

#### 欠点

- fairness を後から検証しづらい
- debug replay と real fairness claim が同じ authority trust に乗る
- provider 差し替えや proof 境界が見えにくい

### 案B — auditable authority witness

#### 読み

- `draw` 自体は authority が確定してよい
- ただし audit には result だけでなく、少なくとも `witness_ref` または replay に必要な witness を残す

概念上はたとえば次のように読む。

```text
draw <- authority_rng.d6()
witness <- authority_rng.make_witness(room_epoch, turn_epoch, actor_incarnation, draw)
commit move with draw + witness_ref
```

#### 利点

- provider placement を変えずに audit / replay を 1 段強化できる
- debug / deterministic replay hook を差し込みやすい
- fairness claim を「authority trusted」から「authority + witness trusted」へ少し分離できる

#### 欠点

- witness の最小 shape を later に決める必要がある
- cryptographic proof なのか replay token なのかを current phase では固定しないと明記し続ける必要がある

### 案C — delegated provider attestation

#### 読み

- authority は transition commit を行う
- randomness は delegated provider が返し、provider receipt / attestation を audit に残す

#### 利点

- authority placement と fairness source をより明確に分けられる
- HW entropy / external RNG / replayable debug RNG を provider family として差し替えやすい
- fairness claim を authority 1 点信頼から少し外せる

#### 欠点

- provider identity / receipt validation / failure policy が増える
- auth / identity layering と自然に接触しやすい
- current phase では operational realization 寄りになる

### 案D — distributed fairness protocol

#### 読み

- commit-reveal や threshold randomness で multi-party に draw を決める

#### 利点

- strongest fairness claim の余地がある
- authority 1 点の信頼を最も弱めやすい

#### 欠点

- membership churn / timeout / partial participation / reconnect と強く結びつく
- current room profile の 4 軸 bundle より一段重い control-plane を要する
- current repo phase では premature である

### authoritative game room の current working line

- **current minimal practical candidate は案A**
- **next narrow strengthening candidate は案B**
- **provider placement を分離する段階では案Cを組み合わせ候補に残す**
- **案Dは future research に残す**

ここで大事なのは、`delegated_rng_service` を採るかどうかと、fairness claim をどこまで audit witness 付きにするかを同じ軸に潰さないことである。

### append-friendly room での current line

- shared RNG 自体が不要なことが多い
- 必要な場合でも、fairness claim を room profile の必須核にせず、provider-side attestation を optional capability に留める方が自然である

### current working judgment

- **authoritative room の fairness trust model は、current phase では `opaque authority trust` を最小候補に置く**
- **`auditable_authority_witness` の minimal witness core は `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md` で切り出し済みであり、room profile claim と audit / receipt side の typed witness bundle を分ける cut まで current checkpoint に含めてよい**
- **`delegated_rng_service` は provider placement の next practical candidate であり、その authoritative-room practical cut は `specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md` までで current first choice を切ってよい**
- **その際も trust model 上は `auditable_authority_witness` と組み合わせて読めるが、provider placement と witness requirement 自体は同じ軸に潰さない**
- **distributed fairness protocol は current room-profile line に混ぜず future research に残す**
- delegated provider を採るかどうか、fairness claim をどこまで witness 付きにするか、provider placement を room topology / owner slot とどう接続するかは、引き続き別軸で比較する

したがって current repo では、fairness を詰めるときも

- randomness provider がどこにあるか
- fairness claim を誰が担保するか
- audit / replay witness をどこまで要求するか

を別軸で比較するのが最も自然である。

### `auditable_authority_witness` の minimal witness core

`auditable_authority_witness` の minimal witness core comparison は `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md` に切り出した。
current first choice は、room profile に

```text
fairness_claim = auditable_authority_witness
```

だけを残し、witness payload 自体は audit / receipt side に

```text
witness = {
  witness_kind,
  action_ref,
  draw_slot,
  draw_result
}
```

の 4 field を持つ typed bundle として置く cut である。

repo-level actualization としては、`specs/examples/476-current-l2-auditable-authority-witness-strengthening-actualization.md` がこの minimal witness core を helper-local strengthening manifest に ratchet した。current reading は次である。

- reached sample は `p07-dice-late-join-visible-history`
- `p08-dice-stale-reconnect-refresh` と `p05-dice-owner-guarded-chain` は guard-only に留める
- claim / payload split は保ち、room profile には `fairness_claim = auditable_authority_witness` だけを残す
- `draw_result` は final public scalar receipt ではなく symbolic binding ref として actualize する
- final public witness schema、final public provider receipt schema、delegated provider attestation は still later に残す

この judgment により、

- provider placement
- membership / causality carrier
- auth / identity layering

を minimal witness core に混ぜずに済む。
したがって current phase の次段は、provider placement を `delegated_rng_service` に差し替えた authoritative room candidate 自体は `specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md` までで current first choice を切り、その次に control-plane separated causal carrier を reopen する threshold を `specs/examples/125-shared-space-control-plane-carrier-threshold.md` で比べる line に移るのが自然である。

### control-plane separated carrier の threshold 読み

`specs/examples/125-shared-space-control-plane-carrier-threshold.md` の current judgment は次である。

- `membership_epoch + member_incarnation` は、authoritative room baseline、minimal witness core、delegated-provider practical cutを current package として維持する間は first practical candidate のままでよい
- authority handoff、provider binding rotation、activation ack frontier の compare need を room core / audit compare に上げた時点で、control-plane separated carrier の reopen threshold は満たす
- その場合でも first reopen cut は full control-plane log ではなく、`control_epoch` 相当の lightweight split に留めるのが自然である

したがって current repo では、causal metadata line をこれ以上 current package に押し広げず、Phase 4 の current package は authoritative baseline、working subset、minimal witness core、delegated-provider practical cut、control-plane threshold comparisonまでで checkpoint close とみなしてよい。

## repo-level docs-first re-entry の current reading

Phase 4 self-driven closeout のあとで shared-space line を repo-level current line へ戻すときも、
current repo では old `FutureWork` bucket を再導入しない方が自然である。

### current first choice

- Mirrorea / shared-space は **Macro 6 docs-first boundary track** として再入場させる
- Typed-Effect Wiring Platform と PrismCascade は **adjacent boundary track** に留める
- upper-layer application / domain target は **user-spec-required gate** に留める
- shared-space docs-first follow-up checkpoint は
  - identity / auth layering
  - admission policy / compile-time visibility
  - authority / resource ownership
  - room-profile / host-binding bridge-only note
  の 4 package までで fixed 済みと読む

### current next reserve line

- room-profile / host-binding bridge-only note までは fixed 済みである。
- 次に残る shared-space reserve line は fairness / replay strengthening であり、
  final shared-space catalog と operational realization は still later / user-spec-required gate に残す。

### まだ current re-entry bundle に入れないもの

- final shared-space catalog
- control-plane separated carrier actualization
- distributed fairness protocol
- Mirrorea operational runtime / path-proof / consensus realization
- upper-layer app finalization

### practical reading

この re-entry bundle が言いたいのは、
Phase 4 closeout package を再拡張することではなく、

- `mirrorea_fabric_boundary`
- `shared_space_practical_boundary`

を current repo-level boundary bundle として並べ直し、
そのうえで next shared-space reopen を identity/auth line へ narrow に handoff する、
ということである。

## identity / auth layering をどこで切るか

shared-space line では、participant carrier を考え始めるとすぐに

- stable principal identity
- transport/session auth
- service login / external subscription
- room-local permission
- display identity / avatar projection

が混ざりやすい。

blog 起点の直感としても、早すぎる interface concretization は避けたいし、platform / world / avatar の layered graph は分けて読みたい。これは [同期に特化した言語を考えてみる（１）](https://blog.yukatayu.tech/blog/sync_language_01/) の「実装前に node 間 interface を固定しすぎない」line と、[同期に特化した言語を考えてみる（２）](https://blog.yukatayu.tech/blog/sync_language_02/) の layered graph / platform→world→avatar line と整合する。

### 比較したい 3 案

1. membership carrier に identity / auth をまとめて埋め込む
2. membership registry に identity core を残し、auth stack / admission policy を分ける
3. room core には opaque actor handle しか持ち込まず、identity / auth を完全に外へ出す

### 案A — membership carrier に identity / auth をまとめて埋め込む

#### 読み

- participant entry が principal / credential / permission / display identity を全部持つ

概念上は次のようになる。

```text
MemberEntry {
  member_ref,
  principal_ref,
  auth_token_ref,
  permission_set,
  display_name,
  member_incarnation,
  activation_state,
}
```

#### 利点

- 1 carrier だけ見れば room 参加の可否を説明しやすい
- implementation 初期には分かりやすく見える

#### 欠点

- membership と auth refresh / transport session renewal / room permission を同時に更新しやすい
- auth stack の layering が language core に漏れやすい
- fairness witness / authority placement / admission policy と混線しやすい

### 案B — identity core と auth stack / admission policy を分ける

#### 読み

- membership registry は identity core だけを持つ
- auth stack と room admission policy は別 carrier に置く

概念上は次のように分ける。

```text
MemberCore {
  member_ref,
  principal_ref,
  member_incarnation,
  activation_state,
}

AdmissionContext {
  transport_auth_ref,
  service_auth_ref,
  room_permission_ref,
  policy_epoch,
}

ProjectionIdentity {
  display_projection_ref,
}
```

#### 利点

- membership churn と auth renewal を分けやすい
- principal identity と room-local membership slot を同一視しなくて済む
- authority / fairness / admission policy を別軸に保ちやすい
- auth layer を TLS / OAuth / subscription / room ACL などで後から差し替えやすい

#### 欠点

- carrier が 1 段増える
- room profile には admission rule と permission ref の接点だけを書き、raw auth protocol は外に残す discipline が要る

### 案C — room core には opaque actor handle しか持ち込まない

#### 読み

- room semantics は `actor_handle` と activation state だけを見る
- identity / auth / display identity は完全に外部 gateway / session service に置く

#### 利点

- language core から auth details を最もきれいに外せる
- append-friendly room や external auth-heavy deployment と相性が良い

#### 欠点

- audit / blame / fairness witness と principal identity の接続が弱くなりやすい
- authoritative room で `誰が turn を持っていたか` を説明するときに opaque handle だけでは足りない場面がある
- room-local capability と principal continuity の bridge が別に要る

### authoritative すごろく room での読み

authoritative room の join path を概念上で書くと、現在自然なのは次である。

```text
join_request(player_p, auth_stack)
  -> authority verifies admission policy
  -> registry creates MemberCore {
       member_ref = player_p#8,
       principal_ref = principal:user_p,
       member_incarnation = 8,
       activation_state = active,
       display_ref = avatar:user_p
     }
  -> room emits players_snapshot
```

room rule 側は raw token を見ず、たとえば次だけを見る。

```text
request_roll(by member_ref)
require member_is_active(member_ref)
require member_has_turn(member_ref)
```

ここで `member_is_active` や `member_has_turn` は room semantics に属するが、`principal:user_p` がどの TLS / OAuth / subscription stack で確認されたかは auth layer に残る。

### append-friendly room での読み

- `principal_ref` を持つ identity core だけあれば十分なことが多い
- append permission は room-local capability / ACL ref に留め、transport/service auth の更新を room log に混ぜない方が自然である

### current working judgment

- **current first practical candidate は案B**
- **membership registry には identity core を残し、auth stack / admission policy は別 carrier に置く**
- **案C は auth-heavy deployment では有力だが、current authoritative room では principal continuity と audit connection が薄くなりやすい**
- **案A は implementation 初期には単純に見えるが、layering conflict が起きやすいので current first choice にしない**

したがって current repo では、shared-space identity / auth line を詰めるときも

- membership core
- admission policy
- auth stack
- display / projection identity

を別軸に保つのが最も自然である。

`specs/examples/365...366` により、この docs-first reopen は current normative package として fixed 済みである。current minimum は `member_ref + principal_ref + member_incarnation + activation_state` を membership identity core に残し、auth/admission side と projection side を separate ref family に保つ cut に留める。

## admission policy と compile-time over-approximation の接点

identity / auth layering を分けた後に残る論点は、**compile-time が何を知ってよく、runtime control-plane に何を残すか** である。

shared-space authoritative room でも append-friendly room でも、compile-time が直接確定しにくいものは少なくとも次である。

- actual principal set
- late join / reconnect 後の active member set
- transport / service auth refresh の成否
- authority が最終的に受理した admission event

一方、compile-time で **over-approximation** してよいものはある。

- どの room role が activation visibility に関与しうるか
- どの action がどの room-local capability / permission ref を要求しうるか
- どの notify / publish path が必要になりうるか

### 比較したい 3 案

1. runtime-only admission
2. declared role / capability / visibility over-approx + runtime admission
3. closed-world exact admission / visibility set を compile-time へ上げる

### 案A — runtime-only admission

#### 読み

- room source は role / permission requirement をほとんど宣言しない
- authority / gateway / control-plane が runtime で全部判定する

#### 利点

- room syntax は最も軽い
- auth stack の variation を最も吸収しやすい

#### 欠点

- compile-time で room capability / visibility requirement を見抜きにくい
- docs / proof / audit で「この room が何を要求するか」が弱くなる
- detached validation loop に載せられる structural evidence が減る

### 案B — declared role / capability / visibility over-approx + runtime admission

#### 読み

- room source は role / capability / visibility requirement を declaration として持つ
- ただし actual principal satisfaction と active set は runtime control-plane に残す

概念上はたとえば次のように書ける。

```text
room sugoroku_room {
  visibility_roles = [authority, watcher, player]

  action request_roll by player
    requires capability(room.roll)
    notifies [watcher, player]
}
```

runtime 側では次のように読む。

```text
join_request(principal:user_p, auth_stack)
  -> authority checks admission_policy(role = player, capability = room.roll)
  -> activate member_ref = player_p#8
```

#### 利点

- compile-time で required role / capability / notify path の over-approximation ができる
- type / proof / model checking 側へ送る static floor を作りやすい
- actual admission / active member set は runtime に残せる
- current `authority-ack` と両立しやすい

#### 欠点

- declaration-side wording と runtime admission policy の bridge を later に詰める必要がある
- role / capability declaration をどこまで room syntax に載せるかは後続比較が要る

### 案C — closed-world exact admission / visibility set を compile-time へ上げる

#### 読み

- compile-time に room participant set や visibility committee をかなり精密に固定する
- active 化や notification の成立条件まで static に近づける

#### 利点

- fixed small room では strongest static explanation を作りやすい
- proof 上はきれいに見える

#### 欠点

- late join / reconnect / churn に弱い
- shared-space line を closed-world に寄せすぎる
- auth refresh / authority handoff / delegated provider failure まで compile-time 前提に引きずりやすい

### authoritative すごろく room での current line

current phase の practical line は、概念上は次で十分である。

```text
room sugoroku_room {
  visibility_roles = [authority, player, watcher]

  action request_roll by player
    requires capability(room.roll)
    notifies [authority, watcher, player]
}
```

この declaration は compile-time には

- `request_roll` が `player` role を要求しうる
- `room.roll` capability が必要になりうる
- move / winner / reset が watcher/player/authority へ notify されうる

ことだけを与える。

actual `principal:user_p` が本当に `player` として active 化されたかは runtime admission policy と control-plane に残る。

### append-friendly room での current line

- append action でも `append` capability や visibility role の over-approximationは有益である
- ただし append visibility 自体は activation ack と同じ rule に固定しない方が自然である

### current working judgment

- **current first practical candidate は案B**
- **compile-time には role / capability / visibility requirement の over-approximationだけを残す**
- **actual admission / activation / reconciliation / late join は runtime control-plane に残す**
- **案C は fixed small room proof には魅力があるが、shared-space mainline の first choice にしない**

したがって current repo では、shared-space の compile-time line を詰めるときも

- identity / auth layering
- admission policy
- room capability declaration
- visibility requirement declaration
- runtime control-plane

を別軸に保つのが最も自然である。

`specs/examples/373...374` により、
admission / compile-time visibility line 自体は current first practical cut を固定済みであり、

- compile-time には role / capability / visibility / notify path requirement の over-approximationだけを残す
- actual admission / activation / active member set / reconciliation は runtime control-plane に残す
- raw auth protocol と exact principal set は still later に残す

という split を採った。
したがって shared-space line の next docs-first reopen は、
**authority / resource ownership**
である。

## shared-space resource ownership / delegation の current working model

### 問い

user 直感として、

- 「すべての resource には owner がいる」
- 「必要に応じて owner が delegation / loan を出せる」

という model はかなり分かりやすい。

これは shared-space 側でも有力な working hypothesis である。
ただしここでの `owner` は、そのまま Mir core の value-level ownership と同一視せず、**shared-space resource に対する authoritative write authority slot** として読む方が安全である。

### まず分けるべきもの

最低でも次は分ける。

1. Mir core の ownership / lifetime
   - linearity
   - monotone degradation
   - duplication prohibition
2. shared-space resource owner
   - room resource を authoritative に mutate できる operational authority
3. delegated capability
   - resource 本体の owner を増やさずに、request / append / reserve / move の権限だけを限定的に与える carrier

### current working hypothesis

authoritative room では、**mutable shared resource ごとに current owner slot は 1 つ**、という仮説が最も扱いやすい。

概念上は次のように読む。

```text
ResourceAuthority<R> = {
  owner_ref,
  delegation_set,
  handoff_epoch
}
```

- `owner_ref`
  - 現在 authoritative に write / commit できる主体
- `delegation_set`
  - owner から限定 capability を受けた主体
- `handoff_epoch`
  - owner handoff を audit / replay で区別する世代

### 何が嬉しいか

この model だと、

- exclusive mutation
- lock ownership
- handoff audit
- fairness source の placement

を同じ語彙で比較しやすい。

### 何に注意が要るか

この `owner` は、

- 人間の利用者本人
- deployment 上の server
- replicated authority group
- delegated service

のどれでもありうる。

したがって「resource には owner がいる」は useful だが、「owner は常に participant である」とは言えない。

### minimal invariants

current working model として、少なくとも次の invariant を比較候補にしてよい。

1. **exclusive-authority invariant**
   - authoritative serial resource では、ある時点の authoritative owner slot は高々 1 つ
2. **delegation-is-not-coownership invariant**
   - delegated capability は co-owner を増やさない
3. **explicit-handoff invariant**
   - owner handoff / revocation は explicit event と audit を持つ
4. **mode-compatibility invariant**
   - append-friendly room では「resource owner = object 全体の唯一 owner」を常に要求しなくてよく、append capability の共有で十分な場合がある

### current working judgment

- **authoritative serial room**
  - 「resource ごとに 1 owner slot + delegated request capability」が第一候補
- **append-friendly room**
  - 「log 全体の authoritative owner」と「append capability を持つ participant 群」を分ける方が自然
- **relaxed / merge-friendly room**
  - local shard / contribution owner と global projection を分ける必要があり、current phase では future research に残す

要するに、「すべての resource に owner がいる」は useful だが、**owner = mutable room resource の authoritative authority slot** と読み替えると理論上きれいである。

## authority placement と resource ownership の関係

### authoritative game room

authoritative すごろく room では、少なくとも次の 3 層が見える。

1. room authority
2. resource owner slot
3. participant capability

概念上はたとえばこう切れる。

```text
resource board_state      owner = room_authority
resource roll_lock        owner = room_authority
resource roll_transition  owner = room_authority
provider dice_rng_source  selected_by = room_authority
  source = authority_rng | delegated_rng_service

capability request_roll   delegated_to = active_players
capability read_positions delegated_to = active_players_and_watchers
```

ここで player は `request_roll` capability を持てても、

- `board_state` を直接 mutate する owner
- `roll_transition` を commit する owner
- `dice_rng_source` をどこへ向けるかを選ぶ authority

とは限らない。

### append-friendly room

notice board のような room では、

```text
resource notes_log           owner = room_log_authority
capability append_note       delegated_to = active_participants
capability read_visible_tail delegated_to = active_participants_and_watchers
```

のように、

- log 本体の owner
- append capability

を分ける方が自然である。

### ここから言えること

participant carrier が同じでも、

- authority placement
- resource owner placement
- delegated capability shape

は room rule に依存して変わる。

したがって **membership carrier と resource ownership / delegation model も別軸で比較するべき**である。

## consistency mode と ownership model の相性

### authoritative serial transition

最も相性が良い。

- single owner slot
- explicit delegation
- explicit handoff
- authoritative log

を素直に置ける。

### append-friendly room

中程度に相性が良い。

- object 全体の唯一 owner を前面に出すより
- append right を delegated capability として分ける

方が自然である。

### relaxed / merge-friendly room

現時点では hardest case である。

- local owner
- merge authority
- conflict resolution policy

が別 carrier になりやすい。

current repo の phase では、ここを無理に一枚岩にせず future research とする方が自然である。

## authority placement の候補比較

### 比較したい 3 案

shared-space の authority placement として、current phase で比較対象にしてよいのは少なくとも次である。

1. `single room authority`
2. `replicated authority`
3. `relaxed projection authority`

ここでいう `authority` は、

- room rule を authoritative に commit する主体
- membership activation / deactivation を受理する主体
- resource owner slot の最終 handoff を承認する主体

を含む operational role である。
ただし current repo では、これを algorithm 名や deployment topology に直結させない。

### 案A — `single room authority`

#### 読み

- 1 room につき 1 つの authoritative commit point を持つ
- membership activation も resource mutation もそこを通す

概念上はたとえば次のように読む。

```text
room_authority(room_42)
  accepts join / leave / roll / reset
  emits authoritative transition log
```

#### 利点

- current phase では最も分かりやすい
- `authority-ack` activation と自然に組み合わさる
- owner slot / delegation / handoff / audit を 1 本の transition sequence で説明しやすい
- authoritative game room と相性が良い

#### 欠点

- failure point が 1 点に寄る
- availability / failover を別に考える必要がある
- scale-out では operational realization の工夫が要る

#### 向く room

- turn-based game room
- reset や exclusive lock を強く使う room

### 案B — `replicated authority`

#### 読み

- logical な authority role は 1 つに見せつつ、内部は replica group が authoritative order を共有する
- external interface は single authority に近く保ち、内部で replication / failover を吸収する

概念上はたとえば次のように読む。

```text
logical_room_authority(room_42)
  realized_by authority_group(replica_a, replica_b, replica_c)
  exports one authoritative transition stream
```

#### 利点

- single room authority の reasoning を大きく崩さず failover を持てる
- authoritative game room にも将来的に接続しやすい
- `quorum-like activation` を将来 option に上げる余地を残せる

#### 欠点

- replication family の operational complexity を背負う
- membership activation と authority replica membership を混同しやすい
- current phase では、carrier を急に重くしやすい

#### 向く room

- authoritative room を本番運用寄りに考え始めた段階
- failure tolerance が必要だが、room semantics 自体は authoritative serial のまま保ちたい場合

### 案C — `relaxed projection authority`

#### 読み

- local contribution / local owner は participant 側に寄せる
- global room view は merge / projection / moderation authority が後段でまとめる

概念上はたとえば次のように読む。

```text
local_append(user_i) -> local shard / contribution owner
global_projection(room_42) -> merge / moderation authority
```

#### 利点

- append-heavy room や contribution-heavy room と相性が良い
- local autonomy を高くできる
- global projection を later consistency policy に委ねやすい

#### 欠点

- owner slot / merge authority / visibility policy が別 carrier になりやすい
- authoritative reset / winner / exclusive lock には不向き
- current phase では strongest open problem を多く含む

#### 向く room

- notice board / shared memo / contribution aggregate
- future relaxed / merge-friendly room

### authority placement と room type の相性

#### authoritative game room

- current first choice は `single room authority`
- next operational upgrade path は `replicated authority`
- `relaxed projection authority` は winner / reset / exclusive lock と相性が悪い

#### append-friendly room

- current first choiceは、membership activation だけを見るなら `single room authority` でもよい
- ただし data-plane append visibility まで同じ authority discipline に固定する必要はない
- 将来的には `relaxed projection authority` が有力候補になる

### current working judgment

- **authoritative game room の current first choice は `single room authority`**
- **`replicated authority` は operational realization 側の next candidate**
- **`relaxed projection authority` は append-friendly / merge-friendly room の future comparison に残す**
- ここでいう `single room authority` は、まず **room-level authoritative owner slot / write authority slot** を 1 つ置く読みであり、すべての resource owner を人間 participant に単純還元する意味ではない
- read-mostly snapshot、fan-out state、delegated capability のような例外は、owner slot と delegated capability を分けたまま later option として扱う

つまり current phase では、

- membership registry
- activation rule
- resource owner slot
- authority placement

を 1 carrier に潰さず、`single room authority` を最小 operational candidate として置くのが最もきれいである。

## consistency mode catalog の候補比較

### 比較したい 3 案

current phase で catalog 候補として比較対象にしてよいのは、少なくとも次である。

1. `authoritative serial transition`
2. `append-friendly room`
3. `relaxed merge-friendly room`

ここでいう `consistency mode` は、

- room の shared object がどの順序 / commit point で見えるか
- concurrent action をどこで serialise するか
- local contribution と global projection をどう結ぶか

を決める operational mode である。

### 案A — `authoritative serial transition`

#### 読み

- room mutation は 1 本の authoritative transition sequence に載る
- exclusive lock / reset / winner / handoff をその sequence 上で説明する

概念上はたとえば次のように読む。

```text
join -> activate -> roll_lock -> dice -> move -> goal_notice -> reset
```

#### 利点

- authoritative game room に最も自然
- owner slot / handoff / audit と相性が良い
- `single room authority` と `replicated authority` の両方に接続しやすい
- turn / reset / global winner の reasoning を最も簡単に保てる

#### 欠点

- append-heavy room では重い
- local autonomy を高くしにくい
- throughput より global serial order を優先する

#### 向く room

- authoritative game room
- lock / reset / reservation を伴う room

### 案B — `append-friendly room`

#### 読み

- membership activation や room snapshot は authority 側に寄せてもよい
- ただし data-plane は append capability を共有し、concurrent append を許す
- append visibility は authoritative reset と同じ discipline に縛らない

概念上はたとえば次のように読む。

```text
activate(user)
append(note_1)
append(note_2)
publish tail(snapshot)
```

#### 利点

- notice board / presence room に自然
- append capability と read visibility を分けやすい
- object 全体の唯一 owner を強く前面に出さずに済む

#### 欠点

- global reset / winner / exclusive lock には不向き
- append order と global moderation policy を別に整理する必要がある
- `authoritative serial transition` と同じ audit contract にはしにくい

#### 向く room

- notice board
- shared memo
- append log ベースの collaboration

### 案C — `relaxed merge-friendly room`

#### 読み

- local contribution を participant / shard 側に持ち
- global room view は merge / projection / moderation rule で得る

概念上はたとえば次のように読む。

```text
local_edit(user_i)
local_edit(user_j)
merge / project / moderate
publish merged view
```

#### 利点

- local autonomy を高くできる
- disconnected / intermittent participation と相性がよい
- future shared creative space に向く

#### 欠点

- merge policy と authority placement の coupling が強い
- convergence / conflict handling / moderation を別に要する
- current repo phase では hardest open problem である

#### 向く room

- future relaxed collaboration room
- merge / projection を中心に据える upper-layer space

### consistency mode と authority placement / ownership model の相性

#### authoritative game room

- current first choice は `authoritative serial transition`
- authority placement は `single room authority`
- next operational candidate は `replicated authority`

#### append-friendly room

- current first choice は `append-friendly room`
- membership activation は `single room authority` でもよい
- data-plane authority は append capability と moderation / projection を分けて読む方が自然

#### relaxed / merge-friendly room

- current phase では future comparison
- `relaxed projection authority` と一緒に詰める必要がある

### current working judgment

- **authoritative room の current first choice は `authoritative serial transition`**
- **append-heavy room の current first choice は `append-friendly room`**
- **`relaxed merge-friendly room` は future comparison に残す**
- current small catalog は final catalog や exhaustive / MECE 完了形ではなく、表現力と proof burden を比較するための **working subset** として扱う

したがって current phase では、consistency mode catalog を広く固定するより、

- authoritative room
- append-friendly room

の 2 本を first practical catalog として置き、merge-friendly branch は later に残すのが最も自然である。

## authoritative game room の concrete profile candidates

ここでは、user から出てきた「途中参加可能なすごろく room」を想定し、

- join / leave がある
- サイコロを振ると数秒の pending があり、その間は誰も触れない
- ゴールを踏み越えたらゴール止まり
- 誰かがゴールしたら winner を全員へ通知してから全員を start へ戻す
- player list と positions はリアルタイムに見える

という room rule を、current phase の 4 軸

1. activation rule
2. authority placement
3. consistency mode
4. RNG / fairness source

でどう束ねるかを比較する。

### profile A — minimal trusted authoritative room

current phase の最小 bundle は次である。

- activation: `authority-ack`
- authority placement: `single room authority`
- consistency mode: `authoritative serial transition`
- fairness source: `authority_rng`

概念疑似コードはたとえば次のように読める。

```text
room sugoroku_room {
  membership_source  = session_membership_registry
  players_view       = derived_snapshot(active_members where role == player)
  watchers_view      = derived_snapshot(active_members where role == watcher)

  activation_rule    = authority_ack
  authority_model    = single_room_authority
  consistency_mode   = authoritative_serial_transition
  fairness_source    = authority_rng

  resource board_state      owner = room_authority
  resource roll_lock        owner = room_authority
  resource winner_notice    owner = room_authority

  capability request_roll   delegated_to = players_view
  capability read_positions delegated_to = players_view + watchers_view
}

request join(player p) {
  room_authority.activate_member(p)
  room_authority.initialize_position(p, start)
  room_authority.emit(players_snapshot)
  room_authority.emit(positions_snapshot)
}

request roll(player p) {
  require active_member(p)
  require capability(request_roll, p)
  require roll_lock == idle

  transition by room_authority {
    roll_lock <- pending
    n <- authority_rng.d6()
    board_state[p] <- min(goal, board_state[p] + n)
    emit positions_snapshot(board_state)

    if board_state[p] == goal {
      emit winner_notice(p)
      reset board_state[*] <- start
      emit positions_snapshot(board_state)
    }

    roll_lock <- idle
  }
}

request leave(player p) {
  room_authority.deactivate_member(p)
  room_authority.remove_position_if_present(p)
  room_authority.emit(players_snapshot)
  room_authority.emit(positions_snapshot)
}
```

#### この profile の利点

- current phase では最も単純で説明しやすい
- turn / pending / move / winner notify / reset を 1 本の authoritative transition sequence で扱える
- audit と replay を取りやすい
- compile-time では `request_roll` visibility と room authority requirement の over-approximation を比較しやすい

#### この profile の欠点

- fairness は authority trust に依存する
- authority failure point が 1 点に寄る
- HW entropy や debug RNG provider を差し替える時に authority 実装と結びつきやすい

### profile B — replaceable RNG provider room

次の実践候補は、activation / authority / consistency は profile A のまま維持し、RNG だけを provider 分離する形である。

- activation: `authority-ack`
- authority placement: `single room authority`
- consistency mode: `authoritative serial transition`
- fairness source: `delegated_rng_service`

概念疑似コードは次のように読む。

```text
transition by room_authority {
  roll_lock <- pending
  draw <- delegated_rng_service.d6(room_id, turn_epoch)
  emit draw_reference(draw.audit_ref)
  board_state[p] <- min(goal, board_state[p] + draw.value)
  ...
}
```

#### この profile の利点

- room rule と randomness provider を分けやすい
- HW entropy / external service / deterministic debug provider を差し替えやすい
- portability / observability hook を replaceable layer に残しやすい

#### この profile の欠点

- provider identity / audit reference / failure handling が追加で必要
- provider 障害時に room authority がどう fail-closed するかを決める必要がある
- fairness source が分離される分だけ trust graph が 1 段増える

### profile C — future high-availability / stronger-fairness candidate

future comparison に残す bundle は次である。

- activation: `authority-ack` or successor rule via current leader / authority view
- authority placement: `replicated authority`
- consistency mode: `authoritative serial transition`
- fairness source: `delegated_rng_service` or `distributed_randomness_provider`

#### この profile で追加される論点

- leader / authority handoff epoch
- in-flight roll の ownership
- late join / reconnect と fairness source の relation
- replicated authority と RNG proof / audit reference の同期

current repo phase では、この profile を first practical bundle に上げない。

### 何を bundle artifact / room profile として固定し、何を外に残すか

current phase で room profile に入れてよいのは、

- activation rule family
- authority placement family
- consistency mode family
- fairness source family

までである。

一方で、次は still outside に残す。

- actual consensus algorithm 名
- auth stack
- reconnect protocol
- fairness proof protocol
- deployment topology

### contrast — append-friendly notice / presence room

同じ participant carrier を使っても、append-friendly room では 4 軸 bundle が変わる。

- activation: `authority-ack` を membership control-plane に使ってもよい
- authority placement: membership は `single room authority`、data-plane append は delegated capability
- consistency mode: `append-friendly room`
- fairness source: 通常は不要。必要なら `delegated_rng_service`

この contrast により、authoritative game room の bundle を shared-space 全体の既定と誤読しないようにする。

### current working judgment

- **authoritative game room の current minimal bundle は**
  - `authority-ack`
  - `single room authority`
  - `authoritative serial transition`
  - `authority_rng`
  **である**
- **replaceable / portable な next practical bundle は**
  - RNG だけ `delegated_rng_service` に差し替える形
  **である**
- **replicated authority や distributed randomness は future comparison に残す**

したがって current phase では、shared-space の practical example を進めるときも、

- participant carrier
- activation rule
- authority placement
- consistency mode
- fairness source

を別軸で束ねた **room profile** として書くのが最も自然である。

### baseline consolidation note

この 4 軸 bundle と side condition を 1 本の current baseline judgment に畳んだ正本は `specs/examples/121-shared-space-authoritative-room-baseline.md` である。
`plan/16` は引き続き比較の repository memory と practical contrast を保持し、baseline 自体の current stop line は `specs/examples/121...` を優先して読む。

### catalog working subset note

authoritative room baseline を practical room catalog の first row として含めつつ、append-friendly room との contrast、`auditable_authority_witness`、`membership_epoch + member_incarnation` split、`delegated_rng_service` の optional capability row までを small working subset として切った current judgment の正本は `specs/examples/122-shared-space-catalog-working-subset-comparison.md` である。
ここで append-friendly optional capability row が固定するのは `delegated_rng_service` source までであり、`delegated_provider_attestation` は room-core claim としては凍結せず later candidate に残す。
`plan/16` は引き続き比較の repository memory を保持するが、catalog の current working subset と deferred finalization の線引きは `specs/examples/122...` を優先して読む。

## Raft / Paxos をどう位置づけるか

### current working answer

`Raft` / `Paxos` / `VRR` のような family は、

- authoritative room state の replication
- membership activation の operational agreement
- route rebinding 後の authority handoff

には自然に使える。

しかし、repo の current architectural line では

- **Mir language spec**
- **Mirrorea / shared-space operational realization**

を分けるので、これらを **language core の必須要件にしない**。

### すごろく room での自然な使い方

authoritative sugoroku room なら、implementation としては次が自然である。

1. 1 個の room authority を単純に置く
2. その authority を replicated state machine にする
3. replication family の候補として `Raft` 等を使う

ただし spec がまず要求するのは algorithm 名ではなく、少なくとも次である。

- turn / action の single authoritative order
- join / leave / rejoin の explicit activation rule
- winner 通知と reset の auditability

## reconnect / late leave / in-flight action policy をどこに置くか

authoritative room を practical に考え始めると、すぐ次の論点が出る。

- player が `roll_lock == pending` の途中で抜けたらどうするか
- reconnect した player は pending action を引き継げるのか
- notify 済みだが未受信の winner event は再送対象か

これらを全部 room profile に押し込むと分かりやすそうに見えるが、deployment / liveness / retry / auth と混線しやすい。

### 案A — room profile に全部入れる

#### 読み

- reconnect window
- late leave handling
- in-flight action ownership
- notification retry

を room profile の一部として宣言する。

#### 利点

- application behavior は一見分かりやすい
- example を読む人には親切に見える

#### 欠点

- timeout / retry / network liveness まで room rule に混ざる
- deployment や auth stack の差を吸収しにくい
- current phase では固定が早すぎる

### 案B — すべて external policy layer に残す

#### 読み

- room profile は activation / authority / consistency / fairness だけを持つ
- reconnect や in-flight action は全部 operational policy に任せる

#### 利点

- room profile 自体は軽い
- deployment 差を吸収しやすい

#### 欠点

- pending action の正当性が hidden になりやすい
- 「leave した actor の未 commit action を revive してよいか」が room rule から読めなくなる
- audit / proof の観点では弱い

### 案C — minimal room profile + external policy split

#### 読み

room profile 側には、少なくとも次だけを残す。

1. action ownership は `member_incarnation` 単位で読む
2. deactivation 後、old incarnation の **未 commit action は自動 revive しない**
3. rejoin は new incarnation を作る
4. すでに commit 済みの transition はそのまま authoritative history に残る

一方で external policy 側には、次を残す。

- reconnect window
- retry budget
- notification resend
- network liveness probe
- UI grace period

#### 利点

- room profile に必要な safety rule は残る
- timeout / retry / auth を spec 本体へ混ぜなくて済む
- membership epoch / incarnation とも整合する

#### 欠点

- 2 層を読む必要がある
- user-facing docs では summary が要る

### authoritative すごろく room での読み

current minimal line では、pending roll は actor incarnation に結び付けて読む。

概念上はたとえば次のように置ける。

```text
pending_action roll_42 {
  actor = player_p#7
  status = awaiting_rng
}

event leave(player_p#7) {
  deactivate member player_p#7
  cancel pending_action roll_42 if not committed
}

event rejoin(player_p) {
  activate member player_p#8
  // old pending action does not revive automatically
}
```

このとき保証したいのは、

- old incarnation の未 commit action を hidden に revive しない
- commit 済み transition は authoritative history として残る
- reconnect policy の秒数や retry 回数は external policy に残る

という cut である。

### current working judgment

- **current phase では案Cが最も自然**
- **room profile に残す最小核は**
  - `member_incarnation`
  - deactivation による uncommitted action invalidation
  - rejoin = new incarnation
  - committed transition non-revival
  **までである**
- **timeout / retry / resend / liveness probe は external policy layer に残す**

したがって current repo では、shared-space room profile を practical に書くときでも、

- safety / authority / history continuity に関わる rule は room profile 側
- network / retry / UX grace に関わる rule は external policy 側

に分けるのが最も自然である。

## blog 2 本との連続性

### blog 1 との接続

blog 1 では、「`A[ S[b.hp -= a.attack_power] ]` のように、評価位置を変えるだけで interface が大きく変わって見える」という問題意識がある。

current architecture では、これは

- Mir core が causal / contract / effect の意味を保ち
- Mirrorea が route / authority placement を担い
- shared-space / app layer が participant / board / room rule を担う

という分離に対応する。

### blog 2 との接続

blog 2 では、多重 network、DAG、participant locality、layered graph の発想が出てくる。

current architecture では、これは

- L1 の event DAG
- L2 の routing / audit fabric
- L3 の session / shared object / consistency mode

に分離されていると読むと、かなり整合的である。

## current repo でここまで自走してよいこと

1. shared-space example を docs-first に書く
2. participant churn の boundary を plan に整理する
3. authoritative room と relaxed room の対比例を置く
4. join / leave / rejoin と incarnation の必要性を comparison として整理する
5. authority placement / consistency mode / RNG provider placement を docs-first に比較する
6. compile-time over-approximation と runtime control-plane の境界を comparison として整理する

## ここから先は user 仕様確認が必要

1. active 化の最終 rule
   - authority 1 点承認か
   - full-coverage-like activation か
   - quorum-like activation か
2. identity / auth model
3. RNG の trust model
4. reset / notification の exact guarantee
5. shared-space ごとの consistency mode catalog
6. late leave / in-flight action / reconnect の final policy
7. 上位空間を game / collaborative editor / Reversed Library のどれから具体化するか

## 現時点のまとめ

- participant は **plain array として言語 core に焼き込まない**
- source of truth は **session membership registry**
- array-like 参加者一覧は **derived snapshot**
- vector clock deletion 問題は、**membership reconfiguration と causal metadata の分離**で扱う
- `Raft` / `Paxos` は **implementation choice** として自然だが、spec の必須語彙にはしない
- authoritative game room と relaxed append room を分けると、consistency mode の選択問題が見えやすい
