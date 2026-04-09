# 130 — current L2 theorem line narrow actualization comparison

## 目的

`specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
から
`specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md`
までを前提に、

- mixed row bundle を current docs-only default に維持したままにするか
- theorem line だけを docs-only の narrow projection として切るか
- theorem-side actual handoff emitter まで進むか

を比較し、
**`theorem_prover_boundary` を first practical candidate に置いた後の next narrow cut**
を整理する。

ここで固定するのは current Phase 5 later reopen の docs-first judgment であり、

- final theorem prover encoding
- public checker API
- actual theorem handoff emitter
- protocol / runtime line の split

は固定しない。

## scope

- current L2 の theorem-side handoff だけを扱う。
- mixed row bundle を current docs-only default に維持する前提は崩さない。
- shared-space side line は protocol / runtime candidate に留め、premature に巻き込まない。
- low-level memory-order family には進まない。

## current 前提

current repo では、少なくとも次が成立している。

1. `core_static_checker` / `theorem_prover_boundary` / `protocol_verifier_boundary` /
   `runtime_policy_boundary` の 4-way split は固定済みである。
2. proof-obligation matrix は docs 正本、external handoff artifact は mixed row bundle sketch に留める cut が固定済みである。
3. boundary-specific split と actual emitter は concrete consumer pressure が出たときだけ reopen する。
4. first concrete external consumer pressure の current first practical candidate は `theorem_prover_boundary` に置く。

したがって current 問いは、
**mixed row default を維持したまま theorem line だけをどこまで narrow actualization できるか**
である。

## 比較観点

1. mixed row default を壊さずに theorem line の subject / row family を見える化できるか
2. detached static gate artifact や fixture corpus の source evidence だけで始められるか
3. protocol / runtime line を premature に巻き込まないか
4. public-looking dedicated artifact や emitter を既成事実化しないか
5. stable `evidence_refs` family の次段 actualization へ自然につながるか

## 比較対象

### 案 1. mixed row bundle だけを維持し、theorem-side の追加 shape を作らない

#### 読み

`subject_kind + subject_ref + proof_obligation_rows[]` の mixed row bundle だけを残し、
theorem line だけの narrow view は docs 上でも導入しない。

#### 利点

- current default を最も強く維持できる
- dedicated theorem family を増やさずに済む

#### 欠点

- theorem line が first practical candidate になった後も、実際にどの row family を first cut にするかが見えにくい
- `fixture_static_cluster` と `runtime_try_cut_cluster` の theorem subject を current docs 上で集約しにくい

### 案 2. mixed row bundle を維持したまま、theorem-side projection bundle を docs-only で切る

#### 読み

mixed row bundle 自体は current default に保ったまま、
theorem row だけを対象にした **derived projection**
を docs-only で与える。

projection の最小 shape は次のように読む。

```text
{
  subject_kind,
  subject_ref,
  theorem_rows: [
    {
      obligation_kind,
      evidence_refs
    }
  ]
}
```

ここで `theorem_rows` は
mixed row bundle のうち `boundary = theorem_prover` の row だけを抽出した
derived view であり、
new default carrier ではない。

#### 利点

- theorem line の first actualization candidate を narrow に見える化できる
- mixed row default を壊さずに済む
- protocol / runtime line の row family をまだ split しなくてよい
- next reopen を `stable evidence_refs family` の比較へ自然につなげやすい

#### 欠点

- projection という補助概念が 1 つ増える
- actual emitter を持たないので discipline を docs で維持する必要がある

### 案 3. theorem-side dedicated handoff artifact / emitter に進む

#### 読み

`theorem_handoff_artifact` のような dedicated family を切り、
helper か support module で actual emitter へ寄せる。

#### 利点

- theorem consumer に直接つなげやすそうに見える
- future actual tool line を早く試せる

#### 欠点

- mixed row default を early に弱めやすい
- public-looking carrier を premature に増やしやすい
- actual consumer や stable `evidence_refs` family が未確定な段階では既成事実化が強すぎる

## current judgment

current L2 で最も自然なのは、
**案 2. mixed row bundle を維持したまま、theorem-side projection bundle を docs-only で切る**
である。

理由は次の通り。

1. `theorem_prover_boundary` を first practical candidate にした current readingと整合する
2. mixed row default を維持したまま theorem line の first row family を narrow に見える化できる
3. protocol / runtime line を premature に split しなくてよい
4. actual emitter を deferred に残したまま、next reopen を `evidence_refs` 安定化へ寄せられる

## theorem-side projection bundle の current minimal cut

current docs-only で切ってよい theorem-side projection は次である。

### `subject_kind`

current first candidate では、少なくとも次に限る。

- `fixture_static_cluster`
- `runtime_try_cut_cluster`

shared-space `room_profile` / `room_action` は current theorem-side first cut には入れない。

### `subject_ref`

current first cut では、

- fixture id
- representative runtime cluster ref

のような **既存 source evidence へ戻れる stable ref** だけを置く。

volatile file path や helper wording を subject identity に使わない。

### `theorem_rows`

current first cut で `theorem_rows` に置いてよいのは、
mixed row bundle の theorem row から projection できるものに限る。

代表的には次である。

- `canonical_normalization_law`
- `no_re_promotion`
- `rollback_cut_non_interference`

`evidence_refs` は current first cut では、

- `fixture:<id>`
- `static_gate_artifact:<id>`
- `runtime_cluster:<id>`

のような stable symbolic ref family に留める。

`checked_reason_codes` や helper wording 自体を row 本体に上げない。

## なぜ shared-space theorem row を first cut に入れないか

shared-space side lineでは、

- authoritative baseline
- witness core
- delegated provider practical cut
- control-plane threshold

まで source-backed に揃っている。

しかし theorem-side first cut に `room_profile` / `room_action` を入れると、

- protocol verifier row との境界
- fairness / replay / receipt consistency
- activation frontier / runtime policy

を theorem line の first cut で再度巻き込みやすい。

したがって current phase では、
theorem-side first cut は current L2 fixture / try-cut cluster へ限る方が自然である。

## practical examples

### example A — fallback chain subject の theorem projection

```text
subject_kind = fixture_static_cluster
subject_ref  = e12-lineage-edge-mismatch
theorem_rows = [
  {
    obligation_kind = canonical_normalization_law,
    evidence_refs = [
      fixture:e12-lineage-edge-mismatch,
      static_gate_artifact:e12
    ]
  },
  {
    obligation_kind = no_re_promotion,
    evidence_refs = [
      fixture:e12-lineage-edge-mismatch
    ]
  }
]
```

この projection は mixed row bundle から theorem row だけを抜いた view であり、
new default carrier ではない。

### example B — `try` / `atomic_cut` subject の theorem projection

```text
subject_kind = runtime_try_cut_cluster
subject_ref  = e6-write-after-expiry
theorem_rows = [
  {
    obligation_kind = rollback_cut_non_interference,
    evidence_refs = [
      fixture:e6-write-after-expiry,
      runtime_cluster:e6
    ]
  }
]
```

この case でも `atomic_cut` 自体の place-local structural floor は core 側に残り、
global theorem obligation だけが theorem projection に現れる。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. mixed row bundle は current docs-only default に維持する
2. theorem line の next narrow actualization は dedicated artifact / emitter ではなく、docs-only theorem-side projection bundle に留める
3. theorem-side first cut の subject は `fixture_static_cluster` と `runtime_try_cut_cluster` に限る
4. next later reopen は stable `evidence_refs` family をどこまで actual artifact ref に寄せるかの comparison に置く

この cut では次を行わない。

- theorem-side actual emitter の導入
- protocol / runtime line の同時 split
- shared-space theorem row の先行 actualization
