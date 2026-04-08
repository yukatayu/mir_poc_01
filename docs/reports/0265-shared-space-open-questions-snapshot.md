# Report 0265 — shared-space 側の open questions snapshot

- Date: 2026-04-08T00:46:43.081386Z
- Author / agent: Codex
- Scope: current repo の source を基に、shared-space / membership / authority / consistency まわりで「いま何に迷っているか」を整理する。新しい規範判断は追加しない。
- Decision levels touched: OPEN question の再整理のみ。新規の settled decision は作らない。

## 1. Objective

user からの

- 「今困っている論点は何か」
- 「何と何で迷っているか」
- 「各案のメリデメは何か」
- 「理論の観点ではどれがよいか」

という質問に対し、shared-space / membership 側の current open questions を source-backed に簡潔整理する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/03-layer-model.md`
- `specs/05-mirrorea-fabric.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/10-roadmap-overall.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `docs/reports/0111-faq-unresolved-issues-current-state.md`
- `docs/reports/0264-shared-space-membership-boundary-and-example.md`

## 3. Actions taken

1. current repo で shared-space / membership がどこまで整理済みかを再確認した。
2. いま本当に open な論点を、implementation convenience と理論上の問題を混ぜずに分解した。
3. 各論点について、
   - 何と何で迷っているか
   - それぞれの利点 / 欠点
   - 理論上の推奨
   を短くまとめた。

## 4. Files changed

- `docs/reports/0265-shared-space-open-questions-snapshot.md`

`plan/` 更新不要。  
`progress.md` 更新不要。

## 5. Commands run and exact outputs

```text
$ git status --short --branch
## main...origin/main
```

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 09:46 JST
```

## 6. Evidence / findings

### 論点 1. participant 集合を何で表すか

#### 迷っている案

1. plain array / list を source of truth にする
2. session-scoped membership registry を source of truth にし、array-like view は derived にする
3. consensus-coupled replicated membership object を最初から source of truth にする

#### 利点 / 欠点

**1. plain array**
- 利点: 見た目が単純、UI には素直
- 欠点: join / leave / rejoin、identity、ordering、activation、causal actor key が混ざる

**2. membership registry**
- 利点: identity / incarnation / activation / routing を分けやすい
- 欠点: carrier 設計が少し重い

**3. consensus-coupled object**
- 利点: authority を強くしたい room には自然
- 欠点: consensus family 依存が早すぎる

#### 理論の観点での推奨

**2 が最も自然**。  
理由は、membership change と causal metadata を切り分けやすく、`specs/03` の L3 shared-space 責務とも整合するからである。

### 論点 2. join / leave / rejoin の成立条件

#### 迷っている案

1. authority-ack で active 化
2. `all_of` 的な全員観測を要求して active 化
3. quorum-like activation

#### 利点 / 欠点

**1. authority-ack**
- 利点: 最小で実装しやすい
- 欠点: authority の trust model を決める必要がある

**2. all_of**
- 利点: 「全員が知った」強い整合がある
- 欠点: churn や offline member に弱い

**3. quorum-like**
- 利点: 可用性が高い
- 欠点: language spec に持ち込むと consensus 色が強すぎる

#### 理論の観点での推奨

**いまは 1 を最小 operational candidate として比較し、2 と 3 は policy option に残す**のが自然。  
`all_of` や quorum は Mir-1 / Mirrorea 的 aggregate rule と似るが、shared-space の activation まで直ちに同一視しない方が境界が綺麗。

### 論点 3. authority をどこに置くか

#### 迷っている案

1. room ごと single authority
2. replicated authority
3. merge-friendly decentralized room

#### 利点 / 欠点

**1. single authority**
- 利点: rule が明快、sugoroku のような exclusive action と相性がよい
- 欠点: failover を別で考える必要がある

**2. replicated authority**
- 利点:可用性を上げやすい
- 欠点: operational complexity が上がる

**3. decentralized merge**
- 利点: append-only board などには自然
- 欠点: exclusive lock / global reset とは相性が悪い

#### 理論の観点での推奨

**application ごとに分けるべき**。  
authoritative すごろく room は 1 または 2 が自然で、notice board は 3 に寄せてもよい。  
つまり authority placement は participant carrier とは別軸で選ぶべきである。

### 論点 4. consistency mode をどう catalog 化するか

#### 迷っている案

1. shared-space の既定を authoritative serial transition にする
2. room ごとに consistency mode を選べるようにする
3. CRDT / append-only / lock-step などを最初から広く language 化する

#### 利点 / 欠点

**1. authoritative serial**
- 利点: reasoning が最も簡単
- 欠点: append-heavy collaborative case には過剰

**2. room-selectable mode**
- 利点: application rule に合わせやすい
- 欠点: mode catalog の設計が必要

**3.広い language 化**
- 利点: 表現力は高い
- 欠点: current phase では早すぎる

#### 理論の観点での推奨

**2 が自然**。  
ただし最初の catalog は狭く、たとえば
- authoritative room
- append-only room
程度から始めるのがよい。

### 論点 5. causal metadata と membership change をどう分けるか

#### 迷っている案

1. plain vector clock に add / remove を直接混ぜる
2. membership epoch / incarnation と causal carrier を分離する
3. membership を control-plane event、causality を data-plane event としてさらに分離する

#### 利点 / 欠点

**1. direct mix**
- 利点: 一見簡単
- 欠点: old message と rejoin を誤読しやすい

**2. epoch / incarnation 分離**
- 利点: join / leave / rejoin を明示しやすい
- 欠点: carrier が少し増える

**3. control-plane / data-plane 分離**
- 利点: 境界が最も明快
- 欠点: 実装は少し複雑

#### 理論の観点での推奨

**最低でも 2、できれば 3**。  
current repo の explicit boundary 原則から見ると、1 はかなり不自然である。

### 論点 6. identity / auth をいつ入れるか

#### 迷っている案

1. participant identity を local opaque ID に留める
2. auth / principal / capability を初期から shared-space に入れる
3. identity core と auth policy を分ける

#### 利点 / 欠点

**1. opaque ID**
- 利点: 最小
- 欠点: real system に降ろすと auth 不足

**2. auth を最初から統合**
- 利点: 実運用には近い
- 欠点: current phase では重い

**3. identity core と auth policy 分離**
- 利点: language / space / deployment を分けやすい
- 欠点: 設計は 2 段になる

#### 理論の観点での推奨

**3 が最も自然**。  
identity の最小 core と auth / principal policy は分けるべきである。

### 論点 7. fairness / RNG をどう扱うか

#### 迷っている案

1. trusted authority RNG
2. commit-reveal 的 distributed randomness
3. deterministic seed + audit replay

#### 利点 / 欠点

**1. trusted authority**
- 利点:最小で分かりやすい
- 欠点: trust model が要る

**2. distributed randomness**
- 利点: trust を分散できる
- 欠点: 今は重すぎる

**3. deterministic replay**
- 利点: audit しやすい
- 欠点: seed governance が別問題になる

#### 理論の観点での推奨

**最初は 1、その後 3 を検討、2 は後段**が自然。  
current phase で 2 まで入れると shared-space の本質より protocol complexity が前面に出る。

## 7. Changes in understanding

- current open questions は、単に「shared-space をどう実装するか」ではなく、
  - participant carrier
  - activation rule
  - authority placement
  - consistency mode
  - causal metadata
  - identity / auth
  - fairness / RNG
  という複数軸に分かれていると見た方が分かりやすい。
- 理論的には、どの論点でも共通して
  - identity / causality / membership / policy / realization
  を混ぜない方向が有利である。

## 8. Open questions

1. active 化の final rule をどの family で比較するか
2. authority placement を room ごとに選ばせるか
3. consistency mode の最小 catalog をいくつから始めるか
4. identity core と auth policy の cut をどこに置くか
5. fairness / RNG を shared-space spec にどこまで入れるか

## 9. Suggested next prompt

`plan/16-shared-space-membership-and-example-boundary.md` と `docs/reports/0265-shared-space-open-questions-snapshot.md` を前提に、shared game room の active 化規則を authority-ack / all_of / quorum-like の 3 案で narrow に比較してください。特に、どの保証が language core ではなく shared-space operational policy に属するかを practical example つきで整理してください。
