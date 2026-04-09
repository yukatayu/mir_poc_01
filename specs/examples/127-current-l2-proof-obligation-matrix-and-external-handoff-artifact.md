# 127 — current L2 proof-obligation matrix and external handoff artifact

## 目的

この文書は、`specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
で固定した

- `core_static_checker`
- `theorem_prover_boundary`
- `protocol_verifier_boundary`
- `runtime_policy_boundary`

の 4-way split を前提に、
**proof obligation をどこまで matrix として明示し、external handoff artifact をどこまで narrow に切るか**
を整理する。

ここで固定するのは final public checker API ではない。
固定するのは、

- core checker が何を discharge し、何を external boundary へ残すか
- external boundary へ渡すとき、既存 detached artifact をそのまま public-looking contract にしない cut
- later pressure が出たときに reopen する最小 row shape

だけである。

## current 前提

current repo では次が成立している。

1. `core_static_checker` は local / structural / decidable floor に留まる
2. detached static gate artifact は non-production helper であり、final checker API ではない
3. `expected_static.checked_reasons` / `checked_reason_codes` は current machine-check bridge であり、
   theorem prover / protocol verifier 向け final handoff carrier ではない
4. shared-space line では、authoritative room baseline / minimal witness core / delegated-provider practical cut /
   control-plane threshold comparison まで current package が揃っている

したがって current 問いは、
**proof obligation の残りを docs 上でどう見える化し、必要になったときにどの artifact shape へ narrow reopening するか**
である。

## practical matrix

current repo の representative family を matrix にすると、最低限次の 4 row がある。

| family | representative example | core checker が見るもの | external boundary に残るもの |
|---|---|---|---|
| fallback chain | `writer -> readonly` | same-lineage floor / capability strengthening prohibition / missing option structure | canonical normalization law / no re-promotion |
| `try` / `atomic_cut` | local rollback frontier | structural shape / statement presence | rollback-cut non-interference / hidden rollback absence |
| authoritative witnessed draw | room authority + draw result | room profile の static shape だけ | witness validity / provider receipt consistency / replay fairness |
| activation visibility | join / role / visibility requirement | compile-time over-approximation | activation frontier / retry / resend / reconcile policy |

この表から分かるのは、
**core checker の compare row と external handoff row は同一ではない**
ということである。

## 比較観点

1. current `checked_reasons` / `checked_reason_codes` / `reason_codes` mirror を premature に final contract 化しないか
2. theorem / protocol / runtime の boundary 差を消さないか
3. existing detached artifact をそのまま public-looking handoff artifact に誤読させないか
4. later actualization が必要になったときに、row family を narrow に reopen できるか

## 比較対象

### 案 1. matrix だけを持ち、artifact は増やさない

proof obligation は docs table にだけ残し、repo 内 carrier は増やさない。

#### 利点

- current helper stack をまったく増やさない
- public API や serialization shape を誤って固定しない

#### 欠点

- theorem / protocol / runtime handoff の subject 単位が見えにくい
- later pressure が出たときに、matrix から actual row shape への橋が弱い

### 案 2. matrix を正本にしつつ、row-based external handoff artifact を narrow sketch として切る

docs 上の matrix を正本にしつつ、later reopen 用に
**source artifact を参照するだけの row bundle**
を sketch として与える。

#### 利点

- current detached artifact や static gate artifact を置き換えない
- theorem / protocol / runtime へ残る obligation を subject / row 単位で見える化できる
- later actualization が必要なときも row family から narrow に始められる

#### 欠点

- artifact sketch という補助概念が 1 つ増える
- actual code をまだ持たないため、docs-only で discipline を保つ必要がある

### 案 3. existing detached artifact / static gate artifact をそのまま handoff artifact に昇格させる

`checker_core.reasons`、`checked_reason_codes`、shared-space witness bundle などを、
直接 external verifier contract に読む。

#### 利点

- reuse が多く、一見シンプルである

#### 欠点

- current helper-local carrier を public-looking contract に誤昇格させる
- theorem / protocol / runtime の boundary 差が潰れる
- wording-derived / helper-local row と proof obligation row が混ざる

## current judgment

current L2 で最も自然なのは、
**案 2. matrix を正本にしつつ、row-based external handoff artifact を narrow sketch として切る**
である。

理由は次の通り。

1. current 4-way split を崩さない
2. existing detached artifact / static gate artifact / witness bundle を source evidence として再利用できる
3. handoff artifact 自体は source evidence を参照するだけなので、helper-local carrier を premature に固定しない
4. later reopen が必要になったときも row family 単位で narrow actualization できる

## minimal handoff artifact sketch

current docs-only で sketch してよい最小 shape は次である。

```text
{
  subject_kind,
  subject_ref,
  proof_obligation_rows: [
    {
      boundary,
      obligation_kind,
      evidence_refs
    }
  ]
}
```

### `subject_kind`

- `fixture_static_cluster`
- `runtime_try_cut_cluster`
- `room_profile`
- `room_action`

のような subject の粒度を示す。

### `subject_ref`

fixture id、example id、room profile id、action ref など、
**既存 source artifact へ戻れる stable ref**
だけを置く。

### `proof_obligation_rows`

各 row は少なくとも次を持つ。

- `boundary`
  - `theorem_prover`
  - `protocol_verifier`
  - `runtime_policy`
- `obligation_kind`
  - 例: `canonical_normalization_law`
  - 例: `rollback_cut_non_interference`
  - 例: `witness_receipt_consistency`
  - 例: `activation_frontier_policy`
- `evidence_refs`
  - static gate artifact path
  - detached bundle / aggregate artifact path
  - fixture id
  - room profile id
  - witness ref
  - provider receipt ref
  などの source evidence への参照

## なぜ `checked_reason_codes` をそのまま handoff row にしないか

`checked_reason_codes` は、

- current fixture machine-check
- detached static gate artifact の helper-local compare

に向いた row family である。

しかし proof obligation row は、

- core checker が already discharged したもの
- theorem / protocol / runtime に still 残るもの

を分けて持つ必要がある。

したがって、
`checked_reason_codes` は handoff artifact の `evidence_refs` 側には置けても、
`proof_obligation_rows` 自体とは同一視しない。

## なぜ detached static gate artifact をそのまま handoff artifact にしないか

detached static gate artifact は current helper stack では、

- `fixture_context`
- `checker_core`
- helper-local `detached_noncore.reason_codes`

を持つ。

これは
**static-only / malformed / underdeclared fixture を detached loop で回す**
ための non-production helper であり、

- theorem prover boundary
- protocol verifier boundary
- runtime policy boundary

を一緒に渡す contract ではない。

したがって current cut では、
detached static gate artifact は handoff artifact の source evidence に留める。

## practical examples

### example A — fallback chain

```text
subject_kind = fixture_static_cluster
subject_ref  = e12-lineage-edge-mismatch
proof_obligation_rows = [
  {
    boundary = theorem_prover,
    obligation_kind = canonical_normalization_law,
    evidence_refs = [
      static_gate_artifact:e12,
      fixture:e12-lineage-edge-mismatch
    ]
  }
]
```

ここでは `checked_reason_codes` が same-lineage family を machine-check していても、
canonical normalization 自体は theorem prover 側へ残る。

### example B — authoritative witnessed draw

```text
subject_kind = room_action
subject_ref  = roll#42
proof_obligation_rows = [
  {
    boundary = protocol_verifier,
    obligation_kind = witness_receipt_consistency,
    evidence_refs = [
      room_profile:authoritative_room_baseline,
      witness:roll#42,
      provider_receipt:provider_1#roll42
    ]
  }
]
```

ここでも witness core 自体は `specs/examples/123...` の minimal bundle に留まり、
`provider_receipt:provider_1#roll42` は `specs/examples/124...` が残している optional audit / receipt-side attachment として
`evidence_refs` 側にだけ現れる。

つまり witness bundle 自体が row ではなく、
proof obligation row の evidence 側に入る。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. proof-obligation matrix を docs 正本として持つ
2. external handoff artifact は row-based sketch までに留める
3. existing detached artifact / static gate artifact / witness bundle は source evidence に留める
4. `checked_reason_codes` / `reason_codes` mirror を handoff row へ昇格させない

この cut では次を行わない。

- actual handoff artifact emitter の導入
- public checker API の追加
- theorem prover / protocol verifier input schema の finalization
- low-level memory-order family の導入

## OPEN に残すもの

- `proof_obligation_rows` を mixed row artifact にするか、boundary-specific artifact に割るか
- `subject_ref` の canonical serialization
- `evidence_refs` を path list にするか logical ref list にするか
- later actualization が必要になったとき、どの row family から first prototype を切るか

## current judgment

- current 4-way split の次段として、proof-obligation matrix は docs 正本に置いてよい
- later reopen 用の external handoff artifact は、source evidence を参照する row-based sketch に留めるのが最小である
- `checked_reason_codes`、detached static gate artifact、shared-space witness bundleは handoff row そのものではなく evidence 側に残す
- next later candidate は、mixed row artifact を維持するか、boundary-specific handoff artifact に割るかの threshold comparison である
