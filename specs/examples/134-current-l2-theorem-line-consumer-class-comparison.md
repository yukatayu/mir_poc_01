# 134 — current L2 theorem line consumer class comparison

## 目的

`specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
から
`specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md`
までを前提に、

- theorem-side minimum contract row core を最初に受ける concrete consumer class を何に置くか
- どの consumer class なら docs-only bridge を保ったまま narrow に進めるか

を比較する。

ここで固定するのは **current Phase 5 later reopen の first practical consumer class** であり、

- final theorem prover integration
- actual notebook artifact schema
- public checker API
- actual theorem handoff emitter

は固定しない。

## scope

- theorem-side concrete consumer class だけを扱う。
- minimum contract row core は `obligation_kind + evidence_refs` の current cutを維持する。
- protocol / runtime / shared-space line の consumer class は扱わない。
- solver-specific script format や actual proof object schema には進まない。

## current 前提

current repo では、少なくとも次が成立している。

1. first concrete external consumer pressure の current first practical candidate は `theorem_prover_boundary` に置く。
2. theorem line の first cutは docs-only projection bundle に留める。
3. typed symbolic `evidence_refs` family は current first choice に固定済みである。
4. public checker migration は concrete theorem consumer pressure が出るまで deferred に保つ。
5. minimum contract row core は `obligation_kind + evidence_refs` に留め、consumer-specific attachment は later reopen に残す。

したがって current 問いは、
**その minimum contract row core を最初に受ける concrete consumer class を何に置くのが最も自然か**
である。

## 比較観点

1. minimum contract row core だけで有効に読めるか
2. solver-specific syntax や proof object schema を premature に要求しないか
3. docs-only bridge を維持しやすいか
4. later に stronger consumer へ進むとき手戻りが小さいか
5. theorem-side row core と consumer-specific attachment をきれいに分けられるか

## 比較対象

### 案 1. `proof_notebook` を first practical consumer class に置く

#### 読み

最初の consumer は、
proof obligation row を人間 / theorem worker / external notebook が読む
**proof notebook view**
に置く。

この consumer は、minimum contract row core と最小の prose attachment を読むが、
solver-specific script や machine-checker API はまだ要求しない。

#### 利点

- `obligation_kind + evidence_refs` の row core をそのまま使いやすい
- docs-only bridge を維持しやすい
- later に proof assistant adapter や theorem export checker へ進む前の staging として自然である

#### 欠点

- actual theorem discharge は notebook 自体では行わない
- machine-readable contract を早く欲しい場合には弱い

### 案 2. `proof_assistant_adapter` を first practical consumer class に置く

#### 読み

最初の consumer を、
Lean / Coq 風の proof assistant adapter のようなものに置く。

#### 利点

- theorem discharge に最も近い consumer class である
- later proof automation と直接つながりやすい

#### 欠点

- `goal_text` 以上に structured theorem statement や solver-specific hint を早く要求しやすい
- docs-only bridge を維持しにくい
- current repo に actual adapter anchor が無い段階では重すぎる

### 案 3. `theorem_export_checker` を first practical consumer class に置く

#### 読み

最初の consumer を、
public checker か theorem export checker のような stable machine contract に置く。

#### 利点

- stable contract と emitted artifact の方向へ最も直線的である

#### 欠点

- public checker migration defer threshold と衝突しやすい
- actual artifact / path / bless policy を premature に要求しやすい
- current phase の docs-only disciplineを壊しやすい

## current judgment

current L2 で最も自然なのは、
**案 1. `proof_notebook` を first practical consumer class に置く**
である。

理由は次の通り。

1. minimum contract row core をそのまま読める
2. lightweight attachment family だけで有効な bridge を作りやすい
3. proof assistant adapter や public checker へ早く commit せずに済む
4. docs-first research line と最も整合する

## なぜ `proof_assistant_adapter` を first にしないか

`proof_assistant_adapter` は将来的には有力だが、
current phase では少なくとも

- structured theorem statement
- solver-specific binder / script hint
- actual exported contract

の pressure を早く呼びやすい。

したがって current phase では、
`proof_assistant_adapter` は **second practical candidate** に留める方が自然である。

## なぜ `theorem_export_checker` を first にしないか

`theorem_export_checker` を first にすると、

- public checker migration
- actual emitted artifact
- stable contract / retention policy

を current phase で先取りしやすい。

したがって current phase では、
`theorem_export_checker` は **later candidate** に留める方が自然である。

## practical examples

### example A — fallback chain subject を notebook で受ける

```text
consumer_class = proof_notebook
bridge = {
  subject_kind = fixture_static_cluster,
  subject_ref  = e12-lineage-edge-mismatch,
  contract_rows = [
    {
      obligation_kind = canonical_normalization_law,
      evidence_refs = [
        { ref_kind = fixture, ref_id = e12-lineage-edge-mismatch },
        { ref_kind = static_gate_artifact, ref_id = e12 }
      ]
    }
  ]
}
```

この段階では notebook 側が source evidence を辿れるだけで十分であり、
proof assistant script まではまだ要求しない。

### example B — `try` / `atomic_cut` subject も notebook first でよい

```text
consumer_class = proof_notebook
bridge = {
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

この case でも first bridge に必要なのは row core と source evidence であり、
solver-specific adapter ではない。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. theorem-side first practical consumer class は `proof_notebook` に置く
2. `proof_assistant_adapter` は second practical candidate に留める
3. `theorem_export_checker` は later candidate に留める
4. next later reopen は `proof_notebook` 向け attachment family comparison に置く

この cut では次を行わない。

- proof assistant adapter schema の固定
- theorem export checker contract の actualization
- emitted theorem artifact の導入
