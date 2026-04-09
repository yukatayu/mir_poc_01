# tasks

最終更新: 2026-04-09 12:53 JST

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
- したがって、今すぐ自走で進める順番は
  1. detached validation loop の throughput を上げる
  2. shared-space の authoritative baseline を practical example で厚くする
  3. consistency / fairness / causal metadata を room profile として比べる
  4. static analysis / type / theorem prover / async-control boundary の inventory を整える
  5. checkpoint ごとに drift suppression と mirror sweep を入れる
  である。

## 設計上の基本線

- **throughput を先に上げる**: detached validation loop の friction を先に下げ、以後の研究で fixture / artifact / compare の反復コストを抑える。
- **practical bundle を先に固める**: shared-space は final catalog を急がず、まず authoritative room の最小 practical bundle を厚くする。
- **working subset を先に比べる**: consistency / fairness / causal metadata は exhaustive catalog より先に、room profile の working subset を source-backed に比較する。
- **docs-first inventory を先に作る**: static analysis / proof / async-control は actualization より先に、small decidable core と deferred global property の境界を明確にする。

## 次に自走で進める順番と rough estimate

| 順番 | task package | 主眼 | rough weight | rough 所要 | 自走可否 | 備考 |
|---|---|---|---|---|---|---|
| 1 | detached validation loop の運用摩擦低減 | fixture を足して export / compare / triage を反復しやすくする | 中 | 1〜3 task / 半日〜2日 | 自走可能 | 既存 helper と detached loop を使って friction point を実地観測する |
| 2 | authoritative room baseline の docs-first 精密化 | activation / authority / consistency / RNG の最小 practical bundle を厚くする | 中〜重 | 2〜4 task / 2〜5日 | 自走可能 | すごろく room などの practical example に直結しやすい |
| 3 | consistency / fairness / causal metadata catalog comparison | room mode catalog と membership / epoch / witness / RNG provider の切り分けを詰める | 重 | 3〜6 task / 4〜10日 | 一部自走可能 | final catalog の固定は避け、working subset と stop line を増やす |
| 4 | static analysis / type / theorem prover / async-control boundary inventory | local / decidable core と external verifier 側の境界を整理する | 重 | 3〜6 task / 3〜8日 | 自走可能 | `atomic_cut` を最小核に留め、上位 async-control family を docs-first 比較する |
| 5 | drift suppression / checkpoint sweep | docs / helper / report / progress / plan の整合を保つ | 低〜中 | 各 checkpoint ごとに 0.5〜1日 | 自走可能 | 独立 task というより closeout package |
| 6 | Phase 3 reserve path reopen | parser boundary / first checker cut を later pressure が出たときだけ再開する | 中〜重 | 0〜2 task | 後段依存 | 今は active package ではない |

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

#### rough 所要

- 1〜3 task
- 半日〜2日

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

#### rough 所要

- 2〜4 task で authoritative baseline を一段厚くする
- 4〜8 task で consistency / fairness / causal metadata の working subset comparison まで進める
- 実時間では 2〜10日程度を見込む

#### 現在の推奨度

- **高い**

---

### Task C. static analysis / type / theorem prover / async-control boundary の inventory

#### 目的

将来の型システムや証明器との接続のために、

- 何を local / structural / decidable core に入れるか
- 何を theorem prover / model checker 側へ残すか
- `atomic_cut` のような local cut と、高位の async-control / ordering / consistency rule をどこで分けるか

を small inventory として整理する。

#### この task でやること

- first checker cut に寄せやすい性質を narrow に列挙する
- shared-space / membership / continuation / fairness などの global property を external verifier 側へ残す根拠を明文化する
- parser boundary / detached validation loop と衝突しない bridge 条件を定義する
- C++ 的 low-level memory-order 語彙をそのまま入れずに、event-tree / authority-serial / witness-aware async control family をどこまで docs-first に比べられるか整理する

#### いま自走できる理由

- current repo はすでに hybrid staged approach を前提にしている
- final type system actualization に入らず、inventory だけを切るなら手戻りが小さい

#### 期待する成果物

- docs-first comparison
- small decidable core inventory
- future proof obligation の分類表
- async-control boundary の比較メモ

#### 重さ

- 重い

#### rough 所要

- 3〜6 task
- 3〜8日

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

#### rough 所要

- 各 checkpoint ごとに 0.5〜1日
- 単独主線にはしない

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
- **ただし final activation rule はまだ固定せず、`full-coverage-like` / `quorum-like` は overlay 可能な room policy option として残す**

#### 理由

- authoritative room との整合が最も良い
- implementation と audit の説明がしやすい
- compile-time には visibility over-approximation だけを残し、actual activation は runtime control-plane に逃がせる
- final profile を language core に早く焼き込むより、shared-space / runtime / compile option 側の policy layer に残す方が手戻りが小さい

#### 残っている悩み

- `authority-ack` を language core に見せるか、room policy に留めるか
- reconnect / in-flight action との接続をどこまで room profile に入れるか
- auth / admission layer を重ねたときに activation profile をどこまで独立に差し替えられるか

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
- **ここでいう `single room authority` は「room の authoritative owner slot / write authority slot を 1 つ置く」読みを第一候補にし、人間 participant そのものへの単純還元はしない**

#### 理由

- semantics が最も素直
- game room のような exclusive action と相性が良い
- `replicated authority` は failover に強いが、いまの phase では proof / protocol burden が重い
- read-only snapshot、fan-out、delegated capability などの例外を、owner slot / delegated capability の分離で後から足しやすい

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

#### 残っている悩み

- 「全 resource に owner がいる」という直感を、owner slot / delegated capability / replicated realization の 3 層でどう分けて説明するか
- read-mostly resource や大規模 fan-out state で `single room authority` をどこまで baseline にできるか

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

- **小 catalog を current working subset として使う**
  - `authoritative serial transition`
  - `append-friendly room`
- **ただしこれは final catalog でも MECE 完了形でもなく、今後の表現力 / 証明可能性 / 実装可能性の比較の土台に留める**

#### 理由

- room の違いを見せるには十分
- relaxed merge semantics を早く言語コアへ入れると、proof burden が大きくなる
- final catalog を拙速に固定すると、将来の拡張余地を不必要に狭める

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
- **`distributed_randomness_provider` は default にせず、authority または delegated provider が必要なときに明示的に接続する future option に残す**

#### 理由

- 最小で回すには authority が持つのが一番簡単
- ただし audit や trust を強くしたいなら delegated provider の余地を残したい
- distributed randomness は将来の重い workstream
- fairness / RNG source を participant tree topology や room ownership model に直結させない方が、provider 差し替えと audit model を分けやすい

#### 簡単な例

サイコロ object を room に持ち込めても、RNG だけは

```text
roll_dice uses rng_provider
```

のように explicit provider capability として切る方が、topology と randomness source を混ぜずに済む。

#### 残っている悩み

- delegated provider を room policy / topology / authority delegation のどこで自然に選ばせるか
- fairness claim、audit witness、provider placement をどこまで別軸のまま保てるか

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

---

### 6. 非同期制御 / memory-model boundary

#### 概要

`atomic_cut` のような local finalizing cut だけで十分か、それとも memory-order 的な並列制御語彙や、もっと高位の async-control family が要るかが未決である。

#### 何に影響するか

- room / authority / consistency の表現力
- scheduler / fairness / replay / debug hook の設計
- 型システム / 定理証明 / model checker へどこまで送るか
- user-facing syntax を low-level concurrency 語彙に寄せるかどうか

#### 選択肢

1. `atomic_cut` を最小 cut とし、上位 ordering は room policy / runtime / external verifier 側へ残す
2. C++ 的な `memory_order` に近い low-level ordering vocabulary を language 側へ入れる
3. event-tree / owner slot / authority-serial transition / witness-aware commit を使う higher-level async-control family を別 workstream として育てる

#### current recommendation

- **いまは 1 を core 側の baseline に置く**
- **ただし「`atomic_cut` だけで全部を背負う」とは読まず、3 を Phase 4 / 5 の docs-first comparison として進める**
- **2 は current phase では採らない**

#### 理由

- `atomic_cut` は current repo で place-local finalizing cut として source-backed だが、global ordering / fairness / scheduler まで直接表す primitive ではない
- C++ 的 low-level memory-order を早く入れると、language core・scheduler・proof burden が同時に膨らみやすい
- tree-like room view、authority placement、consistency mode、audit witness を活かすなら、より高位の async-control family として比べる方が理論上きれいで、user-facing でも分かりやすくなりやすい

#### 簡単な例

authoritative room での move request は、現段階では

```text
request move(player)
  -> authority validates request
  -> authority applies serial transition
  -> authority publishes committed state
```

のような high-level ordering として読む方が自然であり、ここを直接 low-level memory barrier 群へ落とす必要はまだない。

#### 残っている悩み

- event-tree / leaf-to-root event bubbling を source-of-truth ではなく execution / debug view としてどこまで formalize できるか
- higher-level async-control family を consistency mode / authority placement / fairness witness とどう直交化するか
- どの局所性までが decidable core に入り、どこから先を theorem prover / model checker 側へ送るべきか

## current recommendation summary

- **自走の順番**:
  1. detached validation loop friction reduction
  2. authoritative room baseline の docs-first 精密化
  3. consistency / fairness / causal metadata catalog comparison
  4. static analysis / proof / async-control boundary inventory
  5. checkpoint ごとの drift suppression / mirror sweep
- **今は止める / later pressure 待ち**:
  - Phase 3 reserve path の reopen
  - final parser grammar
  - public checker API
  - shared-space final catalog
- **user 仕様が強く要る**:
  - upper-layer application contract
  - final shared-space profile catalog
  - higher-layer integration details
