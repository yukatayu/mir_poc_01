# 136 — current L2 theorem line notebook bridge artifact threshold

## 目的

`specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md`
から
`specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md`
までを前提に、

- `proof_notebook` first bridge を **named notebook bridge artifact** へ昇格させるべきか
- それとも current phase では **docs-only derived view** に留めるべきか
- actual emitted notebook bridge artifact に進んでよい threshold はどこか

を比較する。

ここで固定するのは **current Phase 5 later reopen の threshold judgment** であり、

- public checker API
- actual theorem handoff emitter
- final notebook artifact path policy
- proof assistant specific schema

は固定しない。

## scope

- theorem-side `proof_notebook` bridge だけを扱う。
- minimum contract row core は `obligation_kind + evidence_refs` のまま維持する。
- notebook attachment family は `goal_text` を current first choice とする。
- `proof_hint` / `consumer_hint` / solver-specific script には進まない。
- shared-space / protocol / runtime line は巻き込まない。

## current 前提

current repo では、少なくとも次が成立している。

1. theorem-side minimum contract row core は `obligation_kind + evidence_refs` に留める。
2. first practical consumer class は `proof_notebook` に置く。
3. `proof_notebook` first bridge の current lightweight attachment は `goal_text` に留める。
4. theorem-side projection bundle と minimum contract bridge は docs-only line に留める。
5. public checker migration と actual theorem handoff emitter は deferred に保つ。

したがって current 問いは、
**`proof_notebook` first bridge をそれ自体で stable artifact family と読むべきか、
それとも current phase では docs-only derived view に留めるべきか**
である。

## 比較観点

1. docs-only bridge と public-looking contract を混同しないか
2. `goal_text` attachment だけで current notebook consumer を十分に支えられるか
3. path / bless / retention / emitted artifact id を premature に固定しないか
4. later に `proof_assistant_adapter` や actual notebook workflow へ進むとき手戻りが小さいか
5. mixed row default と theorem-side projection bundle の current cut を壊さないか

## 比較対象

### 案 1. current phase では docs-only derived view に留める

#### 読み

`proof_notebook` first bridge は、

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

という **derived notebook view** に留め、
named artifact family や `bridge_kind` はまだ導入しない。

#### 利点

- current phase の docs-only discipline と最も整合する
- `goal_text` attachment があれば notebook consumer の説明力は十分に確保できる
- stable contract family や actual emitter を premature に既成事実化しない
- `proof_assistant_adapter` / `theorem_export_checker` の later pressure が出ても戻しやすい

#### 欠点

- notebook bridge を別 artifact family として参照したい場合の名前はまだ無い
- later reopen 時に stable sketch comparison をもう 1 度行う必要がある

### 案 2. docs-only の stable notebook bridge artifact sketch を切る

#### 読み

actual emitter はまだ導入しないが、
次のような named sketch を docs-only で導入する。

```text
{
  bridge_kind = proof_notebook_bridge,
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

#### 利点

- later actualization の対象となる consumer-local envelope が見えやすい
- `proof_notebook` を separate consumer class として名前付きで扱いやすい

#### 欠点

- current phase では `bridge_kind` が public-looking contract に見えやすい
- actual notebook workflow が無い段階では、current envelopeとの差分が小さい
- actual emitter や stable retention policy への pressure を不要に強めやすい

### 案 3. actual emitted notebook bridge artifact に進む

#### 読み

`proof_notebook` bridge を helper / support module から emit する actual artifact に進む。

#### 利点

- notebook workflow には最も直結しやすい

#### 欠点

- current phase では重すぎる
- path policy / bless / retention / stable id naming を同時に固定しやすい
- public checker migration defer threshold と衝突しやすい

## current judgment

current L2 で最も自然なのは、
**案 1. current phase では docs-only derived view に留める**
である。

理由は次の通り。

1. `proof_notebook` first bridge の current need は `goal_text` 付き row の human-readable bridge であり、named artifact family ではない
2. actual notebook workflow や retention policy がまだ repo 内に存在しない
3. `bridge_kind` のような consumer-local envelope を足しても、current phase では mixed row default / theorem-side projection bundle / minimum contract bridge と比べた利得が小さい
4. actual emitter と public-looking contract を後段へ残す current Phase 5 disciplineを守りやすい

## stable notebook bridge sketch に進んでよい threshold

次の条件が揃ったときだけ、
案 2 の docs-only stable notebook bridge sketch を reopen 候補に上げてよい。

1. **concrete notebook workflow pressure**
   - repo 内で notebook-oriented review / checklist / compare / bless-like flow が必要になり、
     mere docs view では不足する
2. **bridge envelope stability**
   - current theorem-side subject family、row core、`goal_text` attachment が一定安定し、
     notebook consumer-local envelope を separate に切る利点が明確である
3. **public checker との分離維持**
   - notebook bridge を named sketch にしても、
     public checker API や actual theorem handoff emitter と混同しない説明が書ける

この 3 条件が揃わない限り、
current phase では derived notebook view に留める方が自然である。

## actual emitted notebook artifact に進んでよい threshold

案 3 の actual emitted notebook artifact は、さらに強い条件が要る。

1. docs-only stable notebook bridge sketch が一段安定している
2. notebook consumer が actual file artifact を必要としている
3. path / retention / bless / overwrite policy が consumer-local に定まっている
4. emitted notebook artifact を detached validation loop や public checker line と混同しない境界が明記できる

したがって current phase では、
**actual emitted notebook artifact は later reopen に残す**
のが自然である。

## practical examples

### example A — fallback chain subject は current docs-only bridge で足りる

```text
proof_notebook view = {
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

この case では current docs-only bridge で十分に読め、
`bridge_kind = proof_notebook_bridge` を導入する利得はまだ小さい。

### example B — `try` / `atomic_cut` subject も current docs-only bridge で足りる

```text
proof_notebook view = {
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

この段階で必要なのは notebook reader が row と source evidence を追えることであり、
named artifact family や emitted file ではない。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. `proof_notebook` first bridge は current phase では docs-only derived view に留める
2. named notebook bridge artifact sketch は concrete notebook workflow pressure が出たときだけ reopen 候補にする
3. actual emitted notebook artifact はさらに後段に残す
4. next later reopen は、concrete notebook workflow pressure が出たときの stable notebook bridge sketch comparison、または `proof_assistant_adapter` 側の consumer pressure comparison に置く

この cut では次を行わない。

- `proof_notebook_bridge` の named contract 導入
- notebook bridge artifact emitter の actualization
- path / bless / retention policy の theorem-side固定
