# 129 — current L2 first external consumer pressure comparison

## 目的

`specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
から
`specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md`
までを前提に、

- `theorem_prover_boundary`
- `protocol_verifier_boundary`
- `runtime_policy_boundary`

のどの系統が **first concrete external consumer pressure** として最も自然かを比較する。

ここで固定するのは **Phase 5 later reopen の current first practical candidate** であり、

- final theorem prover encoding
- final protocol verifier encoding
- final runtime policy engine
- actual handoff emitter

は固定しない。

## scope

- current L2 / shared-space line にある proof obligation handoff だけを扱う。
- mixed row bundle を current docs-only default に維持する前提を崩さない。
- boundary-specific split は concrete consumer pressure が出たときだけ reopen 候補にする。
- low-level memory-order family の導入には進まない。

## current 前提

current repo では、少なくとも次が成立している。

1. `core_static_checker` / `theorem_prover_boundary` / `protocol_verifier_boundary` /
   `runtime_policy_boundary` の 4-way split は固定済みである。
2. proof-obligation matrix は docs 正本、external handoff artifact は mixed row bundle sketch に留める cut が固定済みである。
3. boundary-specific split と actual emitter は concrete consumer pressure が出たときだけ reopen する。

したがって current 問いは、
**その first concrete consumer pressure がどの boundary から現れるか**
である。

## 比較観点

1. current source evidence だけで narrow actualization に入れるか
2. unresolved policy / catalog / authority 問題を過度に巻き込まないか
3. subject 粒度と obligation row が current docs に素直に乗るか
4. public-looking API を premature に作らずに済むか
5. later boundary-specific split を選ぶ場合でも手戻りが小さいか

## 比較対象

### 案 1. `theorem_prover_boundary` を first concrete consumer pressure にする

#### 読み

first concrete consumer は theorem prover / proof assistant / proof notebook 系であり、
対象は主に

- `fixture_static_cluster`
- `runtime_try_cut_cluster`

のような current L2 側 subject に置く。

#### 利点

- current fixture / static gate / proof-obligation matrix の anchor だけで比較を始めやすい
- shared-space final catalog や runtime retry policy を先に固定しなくてよい
- same-lineage、canonical normalization、rollback-cut non-interference のような
  existing proof obligation と相性が良い

#### 欠点

- theorem prover 実装系の actual consumer はまだ docs 外にある
- proof notebook / assistant への handoff row 粒度は後段で要整理である

### 案 2. `protocol_verifier_boundary` を first concrete consumer pressure にする

#### 読み

first concrete consumer は protocol verifier / model checker 系であり、
対象は主に

- `room_profile`
- `room_action`

に置く。

#### 利点

- shared-space authoritative room / witness / provider receipt の protocol lineと直接つながる
- fairness / replay / stale witness reject など protocol property を集中的に扱える

#### 欠点

- authority placement / consistency catalog / activation frontier の later reopen と衝突しやすい
- shared-space side line の unresolved policy を先に巻き込みやすい
- actual protocol carrier を切ると、mixed row default より早く split へ寄りやすい

### 案 3. `runtime_policy_boundary` を first concrete consumer pressure にする

#### 読み

first concrete consumer は runtime policy checker / operational gate 系であり、
対象は主に

- activation frontier
- retry / resend / reconcile policy
- retention / bless / artifact policy

に置く。

#### 利点

- actual operator workflow に近い
- detached loop や shared-space operation の friction と接続しやすい

#### 欠点

- auth / retry / retention / bless の unresolved policy を先に固定しやすい
- language / proof boundary より運用都合が前に出やすい
- current Phase 5 の small decidable core inventory より外側へ早く寄りすぎる

## current judgment

current L2 で最も自然なのは、
**案 1. `theorem_prover_boundary` を first concrete consumer pressure にする**
である。

理由は次の通り。

1. current source evidence は fixture / static gate / proof-obligation matrix 側に最も揃っている
2. shared-space side line の unresolved activation / authority / consistency / fairness policy を巻き込みにくい
3. runtime policy のような operational policy finalization を早く既成事実化しないで済む
4. mixed row default を維持したまま、theorem row family だけ narrow に reopen できる

## なぜ `protocol_verifier_boundary` を first にしないか

current shared-space line では、

- authoritative baseline
- working subset
- minimal witness core
- delegated-provider practical cut
- control-plane threshold

まで source-backed に揃っている。

しかし protocol verifier を first consumer にすると、

- authority handoff
- final consistency catalog
- activation frontier policy
- replay / fairness protocol の final cut

を theorem line より早く固定しやすい。

したがって current phase では、
protocol verifier は **second practical candidate** に留める方が自然である。

## なぜ `runtime_policy_boundary` を first にしないか

runtime policy line は、

- auth layering
- retry / resend / timeout
- retention / bless
- operator workflow

の unresolved policy を多く含む。

これを first consumer にすると、
small decidable core / proof boundary の inventory より先に
operational policy が前に出やすい。

したがって current phase では、
runtime policy は **later candidate** に留める方が自然である。

## practical examples

### example A — theorem line は fixture subject から始めやすい

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

この subject は current fixture corpus と static gate artifact に戻れるので、
first concrete consumer を theorem line に置いても
shared-space finalization を先取りしない。

### example B — protocol line はまだ second candidate に留める方がよい

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

この row 自体は current docs に載せられるが、
first consumer に上げると authority / catalog / activation の reopen pressure も強くなる。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. first concrete external consumer pressure の current first practical candidate は `theorem_prover_boundary` に置く
2. `protocol_verifier_boundary` は second practical candidate に留める
3. `runtime_policy_boundary` は later candidate に留める
4. mixed row default は維持し、next later reopen は theorem line の narrow actualization comparison に留める

この cut では次を行わない。

- theorem-specific handoff artifact の actual emitter 導入
- protocol verifier carrier の先行 actualization
- runtime policy engine / bless gate の finalization
