# 133 — current L2 theorem line minimum contract row comparison

## 目的

`specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
から
`specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md`
までを前提に、

- theorem-side projection bundle を concrete theorem consumer bridge に渡すと仮定したとき、
  minimum contract row を docs-only でどこまで書くか
- どの field を row core に残し、どの field を later consumer-specific attachment に残すか

を比較する。

ここで固定するのは **current Phase 5 later reopen の docs-only minimum cut** であり、

- public checker API
- actual theorem handoff emitter
- final theorem prover input schema
- proof script / goal text / solver hint

は固定しない。

## scope

- theorem-side minimum contract row だけを扱う。
- mixed row bundle default、theorem-side projection bundle、typed symbolic `evidence_refs` family の current cut は維持する。
- protocol / runtime / shared-space row family は current first cut に含めない。
- detached artifact の path / bless policy や emitted artifact id には進まない。

## current 前提

current repo では、少なくとも次が成立している。

1. `core_static_checker` / `theorem_prover_boundary` / `protocol_verifier_boundary` /
   `runtime_policy_boundary` の 4-way split は固定済みである。
2. proof-obligation matrix は docs 正本、external handoff artifact は mixed row bundle sketch に留める cut が固定済みである。
3. first concrete external consumer pressure の current first practical candidate は `theorem_prover_boundary` に置く。
4. theorem line の first cut は、mixed row default を壊さない docs-only projection bundle に留める。
5. theorem-side `evidence_refs` は typed symbolic `ref_kind + ref_id` family に整える。
6. public checker migration は concrete theorem consumer pressure が出るまで deferred に保つ。

したがって current 問いは、
**theorem-side projection bundle を concrete theorem consumer bridge に渡すとしても、
minimum contract row をどこまで docs-only で固定してよいか**
である。

## 比較観点

1. theorem-side projection bundle の current cutを weaken せずに bridge を見える化できるか
2. helper wording / path policy / emitted artifact id を premature に contract 化しないか
3. obligation row の core と consumer-specific attachment を分けられるか
4. detached static gate artifact や fixture corpus の source evidence へ戻れるか
5. concrete theorem consumer class が未決でも later reopen を狭く保てるか

## 比較対象

### 案 1. theorem-side projection bundle の `theorem_rows` をそのまま bridge row とみなす

#### 読み

`subject_kind + subject_ref + theorem_rows[]` の derived projection をそのまま bridge とみなし、
minimum contract row という別概念を導入しない。

#### 利点

- 追加概念が増えない
- mixed row default と theorem-side projection bundle の current cutに最も近い

#### 欠点

- 「docs-only projection」と「consumer へ渡してよい minimum row」の違いが見えにくい
- later に consumer-specific attachment を足すとき、何が row core で何が attachment かが曖昧になりやすい

### 案 2. theorem-side projection bundle を維持したまま、minimum contract row core を docs-only で切る

#### 読み

projection bundle 自体は current default に留めつつ、
consumer bridge に出してよい最小 row core だけを derived contract として切る。

current minimal row core は次に限る。

```text
{
  obligation_kind,
  evidence_refs
}
```

このとき

- `subject_kind`
- `subject_ref`

は row 本体ではなく envelope 側に残す。

#### 利点

- theorem-side projection bundle の current cutを弱めずに、bridge 側の minimum row だけを見える化できる
- later に `goal_text` / `proof_hint` / `consumer_hint` を足す場合も attachment として分けやすい
- typed symbolic `evidence_refs` family をそのまま row core に流用できる

#### 欠点

- projection bundle と minimum contract row core の 2 段構成になる
- docs-only の derived concept をさらに 1 つ持つので discipline を文章で維持する必要がある

### 案 3. minimum contract row に consumer-specific field も先に含める

#### 読み

minimum contract row を

```text
{
  obligation_kind,
  evidence_refs,
  goal_text,
  proof_hint,
  consumer_hint
}
```

のように consumer-specific attachment まで含む row として切る。

#### 利点

- proof notebook / assistant への bridge を早く具体化できる

#### 欠点

- concrete theorem consumer class が未決な current phase では重すぎる
- row core と consumer-specific attachment を混ぜやすい
- public checker migration / actual emitter を premature に既成事実化しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. theorem-side projection bundle を維持したまま、minimum contract row core を docs-only で切る**
である。

理由は次の通り。

1. theorem-side projection bundle と typed symbolic `evidence_refs` family の current cut をそのまま利用できる
2. minimum row core を `obligation_kind + evidence_refs` に絞れば、consumer-specific attachment を later reopen に残せる
3. public checker migration や actual emitter を既成事実化せずに、concrete theorem consumer bridge の最小条件だけを見える化できる

## current minimum contract row cut

current docs-only で切ってよい minimum contract bridge は次である。

### envelope

bridge envelope に置いてよいのは次に限る。

- `subject_kind`
- `subject_ref`
- `contract_rows[]`

ここで `contract_rows[]` は theorem-side bridge envelope の local name であり、
current phase では cross-boundary 共通 row family ではない。

ここで `subject_kind` は current theorem-side first cut と同じく、

- `fixture_static_cluster`
- `runtime_try_cut_cluster`

に限る。

### row core

current minimum contract row core は次に限る。

```text
{
  obligation_kind,
  evidence_refs
}
```

`obligation_kind` は current theorem-side row family のうち、少なくとも次に限る。

- `canonical_normalization_law`
- `no_re_promotion`
- `rollback_cut_non_interference`

`evidence_refs` は current theorem-side typed symbolic ref family をそのまま使い、
各 element は

```text
{
  ref_kind,
  ref_id
}
```

に留める。

### current row core に入れないもの

current minimum contract row core には次を入れない。

- `boundary`
- `goal_text`
- `proof_hint`
- `consumer_hint`
- `checked_reason_codes`
- helper wording
- path / URI
- emitted artifact id

`boundary` は theorem-side bridge では自明なので row core に重ねない。
`goal_text` / `proof_hint` / `consumer_hint` は concrete theorem consumer class が見えた後の attachment に残す。
`checked_reason_codes` や helper wording は source evidence 側に残す。

## practical examples

### example A — fallback chain subject

```text
{
  subject_kind = fixture_static_cluster,
  subject_ref  = e12-lineage-edge-mismatch,
  contract_rows = [
    {
      obligation_kind = canonical_normalization_law,
      evidence_refs = [
        { ref_kind = fixture, ref_id = e12-lineage-edge-mismatch },
        { ref_kind = static_gate_artifact, ref_id = e12 }
      ]
    },
    {
      obligation_kind = no_re_promotion,
      evidence_refs = [
        { ref_kind = fixture, ref_id = e12-lineage-edge-mismatch }
      ]
    }
  ]
}
```

この shape なら、consumer-specific goal text が未決でも
「何を示したいか」と「どの source evidence へ戻るか」は十分に見える。

### example B — `try` / `atomic_cut` subject

```text
{
  subject_kind = runtime_try_cut_cluster,
  subject_ref  = e6-write-after-expiry,
  contract_rows = [
    {
      obligation_kind = rollback_cut_non_interference,
      evidence_refs = [
        { ref_kind = fixture, ref_id = e6-write-after-expiry },
        { ref_kind = runtime_cluster, ref_id = e6 }
      ]
    }
  ]
}
```

この case でも row core は `obligation_kind + evidence_refs` だけで足りる。
rollback frontier の narrative や proof sketch は consumer-specific attachment 側に残せる。

## なぜ `goal_text` や `proof_hint` をまだ入れないか

current repo には、actual theorem consumer として

- proof notebook
- proof assistant adapter
- theorem export checker

のどれを first bridge にするかがまだ無い。

この段階で `goal_text` や `proof_hint` を row core に入れると、
consumer class ごとの差を minimum core に押し込みやすい。

したがって current phase では、
**row core は `obligation_kind + evidence_refs` に留め、
goal / hint family は later reopen の attachment に残す**
方が自然である。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. theorem-side minimum contract bridge は docs-only derived view に留める
2. bridge envelope は `subject_kind + subject_ref + contract_rows[]` に留める
3. row core は `obligation_kind + evidence_refs` に限る
4. `goal_text` / `proof_hint` / `consumer_hint` は later consumer-specific attachment に残す
5. next later reopen は concrete theorem consumer class と attachment family の比較に置く

この cut では次を行わない。

- public checker API の actual 導入
- theorem handoff emitter の actualization
- path / URI based evidence contract の導入
- proof script / solver-specific hint の固定
