# 135 — current L2 theorem line notebook attachment family comparison

## 目的

`specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md`
と
`specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md`
を前提に、

- `proof_notebook` を first practical consumer class に置くなら
  どの attachment family を current docs-only first cut に置くか

を比較する。

ここで固定するのは **notebook first bridge の current lightweight attachment** であり、

- proof assistant specific hint
- solver script
- actual notebook artifact schema
- emitted theorem bridge file

は固定しない。

## scope

- `proof_notebook` 向け attachment family だけを扱う。
- minimum contract row core は `obligation_kind + evidence_refs` のまま維持する。
- proof assistant adapter / theorem export checker の attachment family は current first cut に含めない。
- actual handoff emitter には進まない。

## current 前提

current repo では、少なくとも次が成立している。

1. theorem-side minimum contract row core は `obligation_kind + evidence_refs` に留める。
2. first practical consumer class は `proof_notebook` に置く。
3. `proof_assistant_adapter` は second practical candidate、`theorem_export_checker` は later candidate である。

したがって current 問いは、
**`proof_notebook` に対して row core の上へ何を最初の attachment として足すのが最小か**
である。

## 比較観点

1. row core と attachment を混ぜずに済むか
2. human-readable bridge として十分な説明力があるか
3. solver-specific syntax を premature に要求しないか
4. later に proof assistant adapter へ進むとき手戻りが小さいか

## 比較対象

### 案 1. attachment を持たず、row core だけに留める

#### 利点

- 最も小さい
- current row core を一切増やさない

#### 欠点

- notebook view としては「何を示したいか」の prose が弱い
- `obligation_kind` だけでは human-oriented bridge として少し硬すぎる

### 案 2. `goal_text` だけを最小 attachment にする

#### 読み

row core はそのままにして、
各 row に **`goal_text`** だけを lightweight attachment として足す。

#### 利点

- human-oriented notebook bridge として十分読みやすい
- solver-specific script / hint を要求しない
- later に `proof_hint` や `consumer_hint` を増やしても row core を変えずに済む

#### 欠点

- prose quality の discipline が必要になる

### 案 3. `goal_text + proof_hint` を first cut にする

#### 利点

- theorem worker にとっては少し便利になる

#### 欠点

- `proof_hint` の粒度が consumer / solver 依存に寄りやすい
- docs-only first cut としてはやや早い

### 案 4. `goal_text + proof_hint + consumer_hint` を first cut にする

#### 利点

- later consumer bridge の shape を先に見せやすい

#### 欠点

- attachment family が重すぎる
- consumer class ごとの差を current first cut へ押し込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `goal_text` だけを最小 attachment にする**
である。

理由は次の通り。

1. row core を変えずに notebook bridge の説明力を上げられる
2. `proof_hint` や `consumer_hint` を later reopen に残せる
3. `proof_notebook` first choice と最も整合する

## current notebook attachment cut

current docs-only で切ってよい notebook bridge は次である。

```text
{
  subject_kind,
  subject_ref,
  contract_rows = [
    {
      obligation_kind,
      evidence_refs,
      goal_text
    }
  ]
}
```

ここで `goal_text` は、

- theorem statement の完全 formal syntax
- solver script
- machine-checker specific hint

ではなく、
**human-readable な short goal description**
に留める。

## current first cut に入れないもの

current notebook attachment first cut には次を入れない。

- `proof_hint`
- `consumer_hint`
- proof script ref
- solver-specific binder / tactic
- emitted artifact id

これらは `proof_notebook` first cut ではなく、
stronger consumer pressure が出た後の attachment family に残す。

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
      ],
      goal_text = "canonical normalization 後も edge mismatch が復元されないことを示す"
    }
  ]
}
```

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
      ],
      goal_text = "expired write path が rollback cut の外側状態を壊さないことを示す"
    }
  ]
}
```

## current docs-only cut

current task で fixed にしてよいのは次である。

1. `proof_notebook` first bridge の minimal attachment は `goal_text` に留める
2. `proof_hint` / `consumer_hint` は later attachment family に残す
3. row core 自体は `obligation_kind + evidence_refs` のまま変えない
4. next later reopen は notebook bridge artifact / stable contract threshold の comparison に置く

この cut では次を行わない。

- proof assistant hint family の actualization
- theorem export checker contract の actualization
- notebook bridge artifact の actual emitter 導入
