# tasks

最終更新: 2026-04-09

## この文書について

- この文書は repo 全体の **current task map** である。
- `progress.md` が rough progress snapshot なのに対し、ここでは
  - ある程度まとまった task として自走できるもの
  - 方針決定が必要で、いま研究の障害になっているもの
  を、もう少し具体的に整理する。
- 規範判断の正本は `specs/`、長期比較と repository memory は `plan/` である。
- ここは append で積み増さず、**毎回 current snapshot に合わせて全体を書き直す**。

## 現在の読み

- 主線は **Phase 2 maintenance tail + Phase 4 side line + Phase 5 inventory line** である。
- Phase 3 は current checkpoint では **reserve path** であり、later pressure が出たときだけ reopen 候補にする。
- したがって、今すぐ進めるべきものは
  - detached validation loop の運用摩擦低減
  - shared-space / membership の docs-first boundary 整理
  - static analysis / type / theorem prover boundary の inventory 整理
  の 3 本である。

## 自走で進められる task package

### Task A. detached validation loop の運用摩擦低減

#### 目的

- fixture を追加し、
- artifact を保存し、
- compare し、
- 差分を見て次の fixture を足す

という loop を、もっと機械的に回せるようにする。

#### この task でやること

- current tiny emitter / diff helper / wrapper を使って、追加 fixture を数本回す
- 実際に friction が残る箇所を観測する
  - export path
  - reference compare
  - aggregate compare
  - fixture scaffold
- friction を `scripts/` や non-production helper の薄い改善で下げる

#### いま自走できる理由

- detached validation loop の入口自体は成立済み
- payload core / bundle context / detached non-core / explanation の split も固まっている
- production API にしない限り、手戻りは比較的小さい

#### 成果物のイメージ

- helper の small improvement
- additional smoke evidence
- friction point comparison の report

#### 重さ

- 中

#### 現在の推奨度

- **最優先**

---

### Task B. shared-space / membership boundary の docs-first 研究

#### 目的

shared-space を language core に早く焼き込まずに、

- participant carrier
- activation rule
- authority placement
- consistency mode
- fairness / RNG provider
- reconnect / leave / rejoin
- causal metadata

の境界を、practical example で崩れない形に整理する。

#### この task でやること

- authoritative room と append-friendly room を対比する
- `authority-ack`、`single room authority`、`authoritative serial transition`、`authority_rng` の組み合わせを practical profile として精密化する
- `session-scoped membership registry + derived snapshot view`
  を source-of-truth model として保ちつつ、
  tree-like / JSON-like な derived view をどう見せるかを整理する
- membership epoch / incarnation と causal metadata の split を比較する

#### いま自走できる理由

- current repo には authoritative room の minimal candidate までの line が既にある
- final catalog を決めない範囲なら、docs-first comparison を積める
- 具体例を使った境界比較は、まだ user 仕様なしでも進められる

#### 実践例

たとえば authoritative game room なら、次のような profile が current first choice である。

```text
activation: authority-ack
authority: single room authority
consistency: authoritative serial transition
fairness source: authority_rng
```

一方、append-heavy room では

```text
consistency: append-friendly room
rng: delegated_rng_service
```

のように、room profile を変える必要がある。

#### 重さ

- 重い

#### 現在の推奨度

- **高い**

---

### Task C. static analysis / type / theorem prover boundary の inventory

#### 目的

将来の型システムや証明器との接続のために、

- 何を local / structural / decidable core に入れるか
- 何を theorem prover / model checker 側へ残すか

を small inventory として整理する。

#### この task でやること

- first checker cut に寄せやすい性質を narrow に列挙する
- shared-space / membership / continuation / fairness などの global property を external verifier 側へ残す根拠を明文化する
- parser boundary / detached validation loop と衝突しない bridge 条件を定義する

#### いま自走できる理由

- current repo はすでに hybrid staged approach を前提にしている
- final type system actualization に入らず、inventory だけを切るなら手戻りが小さい

#### 期待する成果物

- docs-first comparison
- small decidable core inventory
- future proof obligation の分類表

#### 重さ

- 重い

#### 現在の推奨度

- **高い**

---

### Task D. current L2 semantics / notation / examples の drift suppression

#### 目的

主線ではないが、他の研究を進める前提として、

- prose drift
- representative example drift
- helper / plan / progress mirror drift

を抑える。

#### この task でやること

- representative example prose の drift を narrow に点検する
- helper 実装や compare wording と docs を合わせる
- checkpoint ごとに mirror 群を見直す

#### 重さ

- 低〜中

#### 現在の推奨度

- **随時**

## 方針決定が必要で、いま研究の障害になっている悩み

### 1. shared-space の final activation rule

#### 概要

participant が join / leave / rejoin するとき、
「いつ active member とみなすか」を final に決め切れていない。

#### 何に影響するか

- room への publish / notify の成立条件
- late join / reconnect の扱い
- membership と causality の整合
- practical game room / collaboration room の semantics

#### 選択肢

1. `authority-ack`
2. `full-coverage-like`
3. `quorum-like`

#### current recommendation

- **現段階では `authority-ack` を最小 operational candidate に置く**

#### 理由

- authoritative room との整合が最も良い
- implementation と audit の説明がしやすい
- compile-time には visibility over-approximation だけを残し、actual activation は runtime control-plane に逃がせる

#### 残っている悩み

- `authority-ack` を language core に見せるか、room policy に留めるか
- reconnect / in-flight action との接続をどこまで room profile に入れるか

---

### 2. authority placement の final shape

#### 概要

排他制御の要である authority を、

- single に置くか
- replicated にするか
- room ごとに relaxed にするか

を final に決めていない。

#### 何に影響するか

- 排他 action
- reset / global notification
- fairness / RNG trust model
- failure handling

#### 選択肢

1. `single room authority`
2. `replicated authority`
3. `relaxed projection authority`

#### current recommendation

- **authoritative room では `single room authority` を first choice**
- **`replicated authority` は next candidate**

#### 理由

- semantics が最も素直
- game room のような exclusive action と相性が良い
- `replicated authority` は failover に強いが、いまの phase では proof / protocol burden が重い

#### 簡単な例

すごろく room で「サイコロを振ってコマを進める」は、single authority なら

```text
request move(player)
  -> authority checks lock
  -> authority draws RNG
  -> authority applies transition
  -> authority broadcasts state
```

で済む。  
replicated authority にすると、ここに replication / agreement / failover の protocol が増える。

---

### 3. consistency mode catalog をどこまで language 側に持つか

#### 概要

room の consistency mode を

- authoritative serial transition だけに絞るか
- append-friendly room などを catalog に入れるか
- merge-friendly room まで広げるか

で迷っている。

#### 何に影響するか

- shared-space の表現力
- proof / model checking の複雑さ
- examples の分かりやすさ

#### 選択肢

1. `authoritative serial transition` だけを強く押す
2. `authoritative serial transition` + `append-friendly room` の小 catalog
3. merge-friendly room まで catalog に入れる

#### current recommendation

- **小 catalog で始める**
  - `authoritative serial transition`
  - `append-friendly room`

#### 理由

- room の違いを見せるには十分
- relaxed merge semantics を早く言語コアへ入れると、proof burden が大きくなる

---

### 4. fairness / RNG trust model

#### 概要

RNG を

- authority に持たせるか
- delegated provider に出すか
- distributed protocol にするか

を final に決めていない。

#### 何に影響するか

- game room の fairness
- replay / audit
- performance / trust / implementation complexity

#### 選択肢

1. `authority_rng`
2. `delegated_rng_service`
3. `distributed_randomness_provider`

#### current recommendation

- **authoritative room の current minimal は `authority_rng`**
- **next practical candidate は `delegated_rng_service`**

#### 理由

- 最小で回すには authority が持つのが一番簡単
- ただし audit や trust を強くしたいなら delegated provider の余地を残したい
- distributed randomness は将来の重い workstream

#### 簡単な例

サイコロ object を room に持ち込めても、RNG だけは

```text
roll_dice uses rng_provider
```

のように explicit provider capability として切る方が、topology と randomness source を混ぜずに済む。

---

### 5. final parser grammar / public checker boundary

#### 概要

Phase 3 self-driven portion は reserve path に戻したが、
最終的には

- final parser grammar
- public checker API
- runtime / proof boundary

をどこかで決める必要がある。

#### 何に影響するか

- parser / checker 実装の actualization
- syntax sugar の整理
- theorem prover / model checker との接続

#### 選択肢

1. companion notation を長く維持する
2. minimal parser subset から narrow に public 化する
3. grammar を広く先に固定する

#### current recommendation

- **いまは 1 + 2 の間**
- companion notation を維持しつつ、public 化は later pressure が出たときだけ narrow に行う

#### 理由

- 早く public surface を決めると semantics より surface が先行しやすい
- 逆に inventory を作らないと Phase 5 に進みにくい

## current recommendation summary

- **すぐ進める**:
  - detached validation loop friction reduction
  - shared-space / membership docs-first boundary
  - static analysis / proof boundary inventory
- **今は止める / later pressure 待ち**:
  - Phase 3 reserve path の reopen
  - final parser grammar
  - public checker API
  - shared-space final catalog
- **user 仕様が強く要る**:
  - upper-layer application contract
  - final shared-space profile catalog
  - higher-layer integration details
