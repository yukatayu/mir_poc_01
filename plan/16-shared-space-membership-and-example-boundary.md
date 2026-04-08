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

## ここから先は user 仕様確認が必要

1. active 化の最終 rule
   - authority 1 点承認か
   - `all_of` 的 activation か
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

