# 128 — current L2 handoff artifact threshold comparison

## 目的

`specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
と
`specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
を前提に、

- current mixed row bundle をそのまま current docs-only default に維持するか
- `theorem_prover` / `protocol_verifier` / `runtime_policy` ごとの
  boundary-specific handoff artifact に割るか
- actual handoff emitter をどの threshold で切り始めるか

を narrow に比較する。

ここで固定するのは **Phase 5 later reopen candidate の threshold judgment** であり、

- final public checker API
- final theorem prover input schema
- final protocol verifier input schema
- actual handoff emitter

はまだ固定しない。

## scope

- current L2 / shared-space line の proof obligation handoff だけを扱う。
- `proof-obligation matrix` は docs 正本として維持する。
- detached static gate artifact、shared-space witness bundle、provider receipt attachment は
  source evidence として扱い続ける。
- low-level memory-order family の direct encoding には進まない。

## current 前提

current repo では、少なくとも次が成立している。

1. `specs/examples/126...` により 4-way split は current first choice として固定済みである。
2. `specs/examples/127...` により、external handoff artifact は
   `subject_kind + subject_ref + proof_obligation_rows[]` を持つ
   mixed row bundle sketch に留める current cut が固定済みである。
3. current row は `boundary` ごとに意味が違い、
   detached static gate artifact / witness bundle / provider receipt は
   いずれも **row 本体ではなく source evidence** に留めている。

したがって current 問いは、
**mixed row bundle をこのまま docs-only default に維持するか、
それとも boundary-specific artifact を先に切って later actualization へ備えるか**
である。

## 比較観点

1. current 4-way split を premature に public-looking carrier へ潰さないか
2. `subject_ref` と `evidence_refs` の重複を過度に増やさないか
3. theorem / protocol / runtime の row family 差を消さないか
4. actual external tool pressure が出たときに narrow reopen しやすいか
5. detached helper artifact や shared-space witness bundle を誤昇格させないか

## 比較対象

### 案 1. mixed row bundle を current default に維持する

#### 読み

`proof-obligation matrix` を docs 正本に置き続け、
external handoff artifact は current mixed row bundle sketch に留める。

#### 利点

- `subject_kind` / `subject_ref` を 1 箇所に保てる
- theorem / protocol / runtime の row が同じ subject に共存しても carrier を増やさない
- actual external tool pressure がまだ無い current phase と整合する
- detached helper artifact / witness bundle を evidence 側に留めやすい

#### 欠点

- boundary ごとの emitter 設計はまだ曖昧に残る
- later actualization 時には split threshold をもう一段明文化する必要がある

### 案 2. boundary-specific handoff artifact を docs-only で先に分ける

#### 読み

次のような 3 family を先に切る。

- `theorem_handoff_artifact`
- `protocol_handoff_artifact`
- `runtime_policy_handoff_artifact`

#### 利点

- future external tool ごとの shape は見えやすい
- boundary ごとの emitter を個別に考えやすい

#### 欠点

- current phase では `subject_ref` と `evidence_refs` の duplication が増えやすい
- 1 つの subject に複数 boundary row が残る case で split が premature になりやすい
- public-looking carrier family を docs-only で増やしすぎる

### 案 3. mixed row bundle を actual emitter へ寄せながら boundary split も前提にする

#### 読み

current mixed row bundle を shared support / helper へ actualize しつつ、
later split を見越した field family を early に作る。

#### 利点

- actual code evidence は増える
- future emitter へ直接つながるように見える

#### 欠点

- actual external consumer が無い段階で carrier を既成事実化しやすい
- docs-only threshold comparison を飛ばして helper 側の都合が前に出やすい
- current detached / witness / static gate evidence line と混ざりやすい

## current judgment

current L2 で最も自然なのは、
**案 1. mixed row bundle を current docs-only default に維持する**
である。

理由は次の通り。

1. current phase では external handoff の actual consumer がまだ無い
2. subject 単位で theorem / protocol / runtime の row が混在する case を素直に持てる
3. `checked_reason_codes`、detached static gate artifact、shared-space witness bundle を
   source evidence に留める current cut を壊さない
4. later pressure が出たときだけ boundary-specific split や actual emitter を narrow に reopen できる

## boundary-specific split を reopen してよい threshold

次の 3 条件が揃ったときだけ、
boundary-specific handoff artifact を reopen 候補に上げてよい。

1. **concrete consumer pressure**
   - theorem prover / protocol verifier / runtime policy tool のいずれかで
     row bundle ではなく boundary-specific carrier が必要になった
2. **subject concentration**
   - 特定 boundary の row が一定 family で安定し、mixed row の利点より split の利点が勝つ
3. **evidence stabilization**
   - `evidence_refs` が helper-local wording や volatile path ではなく
     stable ref family に寄っている

この 3 条件が揃わない限り、
current mixed row bundle を維持する方が自然である。

## actual handoff emitter を切ってよい threshold

actual handoff emitter は、boundary-specific split よりもさらに強い条件が要る。

current docs-only judgment では、少なくとも次が揃うまで actual emitter に進まない。

1. boundary-specific split か mixed row bundle かの choice が一段安定している
2. actual external consumer が 1 つ以上ある
3. emitted artifact を compare / bless / retain する policy が helper-local を超えて定まっている

したがって current phase では、
**actual handoff emitter は later pressure が出るまで deferred** に残す。

## practical examples

### example A — fallback chain subject は mixed row bundle のままでよい

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

この case は theorem row だけだが、
current phase ではこの 1 row のために dedicated `theorem_handoff_artifact`
family を増やす利点が小さい。

### example B — room action subject は mixed row の方が自然である

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
  },
  {
    boundary = runtime_policy,
    obligation_kind = activation_frontier_policy,
    evidence_refs = [
      room_profile:authoritative_room_baseline,
      room_policy:authority_ack
    ]
  }
]
```

同じ `room_action` に protocol row と runtime row が共存するので、
current phase では mixed row bundle の方が subject 単位の見通しを保ちやすい。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. proof-obligation matrix は docs 正本に置き続ける
2. external handoff artifact は mixed row bundle sketch を current default に維持する
3. boundary-specific handoff artifact は concrete consumer pressure が出たときだけ reopen 候補にする
4. actual handoff emitter はさらに後段へ残す

この cut では次を行わない。

- `theorem_handoff_artifact` などの dedicated family を actual docs contract として増やす
- mixed row bundle emitter を helper stack へ actualize する
- detached static gate artifact / witness bundle を public-looking handoff contract に昇格させる
