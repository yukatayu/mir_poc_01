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

## vector clock から participant を消す問題

### 問題の芯

`leave` 後に actor key を単純削除すると、古い message を受け取った側が

- 「古い incarnation の event」
- 「新しい join」

を区別できない。

これは plain vector clock の問題というより、**membership change と causal metadata を同じ carrier に押し込んでいること**が問題である。

### current working answer

少なくとも conceptual には、次を分ける。

1. membership reconfiguration / activation event
2. causal metadata

そのうえで、message / event 側は少なくとも

- `membership_epoch`
- `logical_member_id`
- `incarnation`

のどれか、できれば全部に相当する情報と整合する形で読む。

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

### 何がまだ OPEN か

- causal metadata の final carrier
- membership epoch と route / patch epoch を共有するか
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
- compile-time が持つべきなのは
  - `activation_visibility(room)` に関与しうる role 宣言
  - required publish / notify path の over-approximation
  までであり、
  actual active set と actual acknowledgement frontier は runtime control-plane に残す

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

current phase では、

- authoritative game room なら `authority_rng` または `delegated_rng_service`
- distributed randomness は future research

が自然である。

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

したがって current phase では、consistency mode catalog を広く固定するより、

- authoritative room
- append-friendly room

の 2 本を first practical catalog として置き、merge-friendly branch は later に残すのが最も自然である。

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
