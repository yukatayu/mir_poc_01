# 131 — current L2 theorem line evidence ref family comparison

## 目的

`specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md`
を前提に、
theorem-side projection bundle の `evidence_refs` を

- free-form symbolic string に留めるか
- typed symbolic ref family に整えるか
- actual path / emitted artifact ref に寄せるか

で比較し、
**current theorem-side first cut に置いてよい stable `evidence_refs` family**
を整理する。

ここで固定するのは docs-first の narrow actualization であり、

- final theorem prover input schema
- actual theorem handoff emitter
- path / URI based final retention policy

は固定しない。

## scope

- theorem-side projection bundle の `evidence_refs` だけを扱う。
- mixed row default と theorem-side projection bundle の current cutは維持する。
- detached path policy や bless flow を先に固定しない。
- protocol / runtime / shared-space line の ref family は current first cut に入れない。

## current 前提

current repo では、少なくとも次が成立している。

1. proof-obligation matrix は docs 正本に置く。
2. external handoff artifact の current default は mixed row bundle である。
3. `theorem_prover_boundary` は first concrete consumer pressure の current first practical candidate である。
4. theorem line の current first cut は docs-only projection bundle に留める。

したがって current 問いは、
**theorem-side projection bundle に置く `evidence_refs` をどの粒度で stable に読むか**
である。

## 比較観点

1. detached helper path や volatile wording を public-looking identity にしないか
2. fixture / static gate / runtime cluster へ戻れる stable ref になっているか
3. actual emitter を deferred に残したまま later migration できるか
4. theorem-side first cut を shared-space line へ広げすぎないか

## 比較対象

### 案 1. free-form symbolic string を維持する

#### 読み

`evidence_refs` を

- `fixture:e12-lineage-edge-mismatch`
- `static_gate_artifact:e12`

のような plain string list に留める。

#### 利点

- docs は最も短く書ける
- 現行の examples とは近い

#### 欠点

- ref family の allowed set が曖昧に残る
- later migration 時に parser / tool 側の解釈差が入りやすい
- helper wording と stable ref family の境界が弱い

### 案 2. typed symbolic ref family に整える

#### 読み

`evidence_refs` の各 row を

```text
{
  ref_kind,
  ref_id
}
```

の typed pair に整える。

current first cut で許す `ref_kind` は次に限る。

- `fixture`
- `static_gate_artifact`
- `runtime_cluster`

#### 利点

- stable ref family の allowed set を narrow に固定できる
- actual path / URI へ行かずに structure を持てる
- later actual emitter や public checker migration が必要になっても bridge を作りやすい

#### 欠点

- docs-only shape が 1 段増える
- actual consumer が無い段階では多少先取りに見える

### 案 3. actual path / emitted artifact ref に寄せる

#### 読み

`evidence_refs` を

- actual file path
- artifact id
- URI-like ref

へ寄せる。

#### 利点

- future tool には直結しやすそうに見える

#### 欠点

- detached path policy / retention / bless flow を premature に固定しやすい
- current docs-only theorem cut に対して重すぎる
- helper-local path drift を contract にしやすい

## current judgment

current L2 で最も自然なのは、
**案 2. typed symbolic ref family に整える**
である。

理由は次の通り。

1. theorem-side first cut は docs-only projection bundle に留めたまま、stable ref family の allowed set を narrow に見える化できる
2. actual path / URI を使わずに済むので、detached path policy と bless flow を巻き込まない
3. free-form string より later migration の橋が作りやすい

## theorem-side `evidence_refs` の current minimal shape

current docs-only で切ってよい最小 shape は次である。

```text
evidence_refs = [
  {
    ref_kind,
    ref_id
  }
]
```

### current first-cut `ref_kind`

- `fixture`
- `static_gate_artifact`
- `runtime_cluster`

### current first-cut `ref_id`

`ref_id` は、

- fixture id
- static gate artifact id
- runtime cluster id

のような **repo 内 source evidence へ戻れる stable symbolic identifier**
に限る。

path や helper wording を `ref_id` にしない。

## なぜ actual path / URI に寄せないか

current theorem line の narrow actualization は、

- first actual consumer pressure の current first choice
- mixed row default を壊さない docs-only cut

までである。

ここで actual path / URI を入れると、

- detached path policy
- bless / retention
- emitted artifact naming

を theorem line の narrow cut と同時に固定しやすい。

したがって current phase では、
actual path / URI ref は later reopen に残す方が自然である。

## practical examples

### example A — fallback chain theorem row

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

### example B — `try` / `atomic_cut` theorem row

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

## current docs-only cut

current task で fixed にしてよいのは次である。

1. theorem-side projection bundle の `evidence_refs` は typed symbolic ref family に整える
2. current first-cut `ref_kind` は `fixture` / `static_gate_artifact` / `runtime_cluster` に限る
3. actual path / URI / emitted artifact id は later reopen に残す
4. next later reopen は theorem-side projection bundle を public checker migration に寄せてよい threshold comparison に置く

この cut では次を行わない。

- theorem-side actual emitter の導入
- detached path policy の finalization
- shared-space witness / provider receipt ref を theorem-side first cut に混ぜる
