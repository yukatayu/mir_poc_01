# 132 — current L2 theorem line public checker migration threshold

## 目的

`specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md`
と
`specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md`
を前提に、

- theorem-side projection bundle を docs-only のまま維持するか
- public checker-facing contract へ narrow migration するか
- actual theorem handoff emitter へ進むか

を比較し、
**current phase で public checker migration に進んでよい threshold**
を整理する。

ここで固定するのは current Phase 5 later reopen の threshold judgment であり、

- final public checker API
- final theorem prover input schema
- actual theorem handoff emitter

は固定しない。

## scope

- current L2 の theorem-side projection bundle だけを扱う。
- mixed row default と typed symbolic `evidence_refs` family の current cutは維持する。
- detached static gate artifact や `checked_reason_codes` を public contract に昇格させない。
- shared-space protocol / runtime line は巻き込まない。

## current 前提

current repo では、少なくとも次が成立している。

1. `theorem_prover_boundary` は first practical candidate である。
2. theorem line の current first cut は docs-only projection bundleである。
3. theorem-side `evidence_refs` は typed symbolic ref family に整える current first choice まで固定済みである。
4. actual theorem consumer / theorem prover encoding は docs 外にあり、repo 内 actual emitter はまだ無い。

したがって current 問いは、
**theorem-side projection bundle をいつ public checker migration に寄せてよいか**
である。

## 比較観点

1. helper-local carrier と residual proof obligation row を混ぜないか
2. actual theorem consumer が無い段階で public-looking contract を既成事実化しないか
3. later actual emitter を still deferred に保てるか
4. typed symbolic `evidence_refs` family を premature に path / URI contract へ寄せないか

## 比較対象

### 案 1. current phase では docs-only projection bundle に留める

#### 読み

theorem-side projection bundle は docs-only の narrow sketch に留め、
public checker-facing contract にはまだ移さない。

#### 利点

- actual theorem consumer が無い current phase と整合する
- detached static gate artifact / `checked_reason_codes` の premature 昇格を防げる
- actual emitter も deferred に保ちやすい

#### 欠点

- theorem consumer が見えたときの bridge はまだ docs 側にしかない

### 案 2. theorem-side projection bundle を public checker-facing contract へ narrow migration する

#### 読み

projection bundle を、
public checker か stable checker output とみなせる contract へ narrow migration する。

#### 利点

- theorem consumer が現れたときの bridge が明確になる
- Phase 6 checker line との接続は見えやすい

#### 欠点

- actual theorem consumer が無い段階では public-looking carrier を早く固定しやすい
- core checker が discharge したものと theorem 側へ残る row の境界を helper surface へ漏らしやすい

### 案 3. actual theorem handoff emitter に進む

#### 読み

theorem-side projection から actual theorem handoff artifact を emit する。

#### 利点

- future tool line には最も直結しやすい

#### 欠点

- current phase では重すぎる
- path policy / retention / bless / actual consumer contract を同時に固定しやすい

## current judgment

current L2 で最も自然なのは、
**案 1. current phase では docs-only projection bundle に留める**
である。

理由は次の通り。

1. current repo には actual theorem consumer がまだ無い
2. typed symbolic `evidence_refs` family まで切れば、later migration の橋は十分に見える
3. public checker API を早く既成事実化すると、helper-local carrier や detached static gate surface が巻き込まれやすい

## public checker migration に進んでよい threshold

次の条件が揃ったときだけ、
theorem-side projection bundle を public checker migration 候補に上げてよい。

1. **concrete theorem consumer pressure**
   - proof assistant / theorem notebook / external checker line が、
     docs-only projection では足りず machine-readable contract を要求する
2. **stable subject family**
   - current theorem-side subject が
     `fixture_static_cluster` と `runtime_try_cut_cluster` で一定安定している
3. **stable symbolic ref family**
   - `evidence_refs` の `ref_kind` / `ref_id` family が drift せず、
     helper wording や volatile path を含まない
4. **core/proof boundary clarity**
   - core checker が already discharge した row と theorem 側 residual row を
     public surface で混同しない説明が書ける

この条件が揃わない限り、
current phase では docs-only projection bundle に留める方が自然である。

## practical examples

### example A — fallback chain はまだ docs-only bridge で足りる

```text
subject_kind = fixture_static_cluster
subject_ref  = e12-lineage-edge-mismatch
theorem_rows = [
  {
    obligation_kind = canonical_normalization_law,
    evidence_refs = [
      { ref_kind = fixture, ref_id = e12-lineage-edge-mismatch },
      { ref_kind = static_gate_artifact, ref_id = e12 }
    ]
  }
]
```

この case は theorem consumer が見えなくても docs 上で十分に説明できるので、
まだ public checker contract へ上げる必要は小さい。

### example B — `try` / `atomic_cut` も current では docs-only でよい

```text
subject_kind = runtime_try_cut_cluster
subject_ref  = e6-write-after-expiry
theorem_rows = [
  {
    obligation_kind = rollback_cut_non_interference,
    evidence_refs = [
      { ref_kind = fixture, ref_id = e6-write-after-expiry },
      { ref_kind = runtime_cluster, ref_id = e6 }
    ]
  }
]
```

ここでも current repo が必要としているのは theorem obligation の見える化であり、
public checker output contract ではない。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. theorem-side projection bundle は current phase では docs-only に留める
2. public checker migration は concrete theorem consumer pressure が出たときだけ reopen 候補にする
3. actual theorem handoff emitter はさらに後段に残す
4. next later reopen は concrete theorem consumer bridge に必要な minimum contract rows をどこまで docs-only で書くかの comparison に置く

この cut では次を行わない。

- public checker API の actual 導入
- theorem-side emitted artifact の actualization
- detached static gate artifact / `checked_reason_codes` の public-looking contract 化
