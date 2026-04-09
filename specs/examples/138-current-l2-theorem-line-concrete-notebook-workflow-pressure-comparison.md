# 138 — current L2 theorem line concrete notebook workflow pressure comparison

## 目的

`specs/examples/136-current-l2-theorem-line-notebook-bridge-artifact-threshold.md`
と
`specs/examples/137-current-l2-theorem-line-next-consumer-pressure-comparison.md`
までを前提に、

- concrete notebook workflow pressure を何とみなすのが最小か
- stable notebook bridge sketch を reopen 候補へ上げる最初の workflow をどこに置くか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の first notebook-workflow pressure threshold** であり、

- named notebook bridge contract
- actual emitted notebook artifact
- notebook file format / path / bless / retention policy
- `proof_assistant_adapter` schema

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- theorem-side minimum contract row core は `obligation_kind + evidence_refs` のまま維持する。
- notebook attachment family は `goal_text` のまま維持する。
- `proof_assistant_adapter` は second practical candidate のまま残す。
- shared-space / protocol / runtime line は巻き込まない。

## current 前提

current repo では、少なくとも次が成立している。

1. theorem-side minimum contract row core は `obligation_kind + evidence_refs` に留める。
2. first practical consumer class は `proof_notebook` に置く。
3. `proof_notebook` first bridge の current lightweight attachment は `goal_text` に留める。
4. current phase では notebook bridge artifact を named family に昇格させず、docs-only derived view に留める。
5. next practical reopen は concrete notebook workflow pressure comparison を first choice に置く。

したがって current 問いは、
**どの workflow requirement が出たときに、mere docs prose を超えて stable notebook bridge sketch の comparison を始めてよいか**
である。

## 比較観点

1. `goal_text` 付き docs-only derived view の current cut を壊さないか
2. stable notebook bridge sketch を reopen する理由として十分に concrete か
3. path / bless / retention / emitted artifact id を premature に固定しないか
4. `proof_assistant_adapter` line より先に扱う理由が明確か
5. human-facing review workflow と machine-facing schema pressure を混ぜないか

## 比較対象

### 案 1. human review checklist / walkthrough pressure を最小条件にする

#### 読み

次のような **人間 reviewer 向け checklist / walkthrough** が、
docs prose だけでは足りず、row-local bundle として繰り返し参照したい段階を
first concrete notebook workflow pressure とみなす。

```text
review_checklist = [
  "obligation_kind を確認する",
  "evidence_refs から fixture / static gate / runtime cluster を開く",
  "goal_text を prose と照合する"
]
```

#### 利点

- `proof_notebook` first consumer と最も連続的である
- `goal_text` attachment だけで十分に支えやすい
- machine-facing stable contract や emitted file へ pressure を広げにくい
- notebook bridge sketch の reopen 理由を human workflow として説明しやすい

#### 欠点

- compare / bless のような stronger flow まではまだ直接は支えない
- workflow が prose / checklist に近く、artifact pressure が弱く見える

### 案 2. compare / bless-like notebook review flow を最小条件にする

#### 読み

次のような **compare / bless-like review loop** が必要になった段階を、
first concrete notebook workflow pressure とみなす。

```text
notebook_review_flow = {
  previous_snapshot,
  current_snapshot,
  reviewer_notes,
  bless_decision
}
```

#### 利点

- stable notebook bridge sketch を切る理由が強い
- reproducible review flow や reusable compare input に接続しやすい

#### 欠点

- detached validation loop の bless / retention / overwrite policy を巻き込みやすい
- notebook bridge sketch comparison より先に policy pressure が強くなる
- current docs-only bridge からの跳躍が案 1 より大きい

### 案 3. actual notebook file exchange / retained notebook artifact を最小条件にする

#### 読み

次のような **actual notebook file** や retained artifact が必要になった段階を、
first concrete notebook workflow pressure とみなす。

```text
notebook_file = {
  rows,
  reviewer_session,
  retained_path
}
```

#### 利点

- actual emitted notebook artifact への道筋が明確である

#### 欠点

- current phase では重すぎる
- path / bless / retention / overwrite policy を同時に固定しやすい
- stable notebook bridge sketch の前に actual artifact pressure を入れてしまう

## current judgment

current L2 で最も自然なのは、
**案 1. human review checklist / walkthrough pressure を最小条件にする**
である。

理由は次の通り。

1. `proof_notebook` first consumer と current `goal_text` attachment の延長として最も小さい
2. docs-only derived view から stable notebook bridge sketch へ進む理由を、人間 review workflow の不足として説明できる
3. compare / bless-like flow や actual retained artifact を最初の pressure にすると、policy / storage / emitter pressure が早く混ざりやすい
4. `proof_assistant_adapter` line を second practical candidate に残した current orderとも整合する

## minimal notebook workflow pressure とみなしてよい条件

次の条件が揃ったときだけ、
案 1 を **concrete notebook workflow pressure** とみなしてよい。

1. obligation row の human walkthrough を、prose の一部ではなく reusable checklist / review unit として繰り返し参照したい
2. `obligation_kind + evidence_refs + goal_text` の 1 row bundle を、review 手順の独立 input として扱いたい
3. named stable notebook bridge sketch を導入しても、public checker API や actual emitted artifact と混同しない説明が書ける

## compare / bless-like flow を first pressure にしない理由

compare / bless-like flow は有力な second step だが、
first pressure としてはまだ強すぎる。

理由は、

- detached loop の bless / retention / overwrite policy を巻き込みやすい
- notebook bridge sketch 自体より先に review policy を固定しやすい
- current phase の docs-only discipline を崩しやすい

からである。

## practical examples

### example A — fallback chain review checklist

```text
notebook_review_unit = {
  subject_ref = e12-lineage-edge-mismatch,
  row = {
    obligation_kind = canonical_normalization_law,
    evidence_refs = [
      { ref_kind = fixture, ref_id = e12-lineage-edge-mismatch },
      { ref_kind = static_gate_artifact, ref_id = e12 }
    ],
    goal_text = "canonical normalization 後も edge mismatch が復元されないことを示す"
  },
  checklist = [
    "row の obligation_kind を確認する",
    "fixture と static gate artifact を追う",
    "goal_text を prose と照合する"
  ]
}
```

この case で必要なのは reusable review unit であり、
compare / bless policy や retained file ではない。

### example B — `try` / `atomic_cut` walkthrough

```text
notebook_review_unit = {
  subject_ref = e6-write-after-expiry,
  row = {
    obligation_kind = rollback_cut_non_interference,
    evidence_refs = [
      { ref_kind = fixture, ref_id = e6-write-after-expiry },
      { ref_kind = runtime_cluster, ref_id = e6 }
    ],
    goal_text = "expired write path が rollback cut の外側状態を壊さないことを示す"
  },
  checklist = [
    "runtime cluster を確認する",
    "rollback frontier と cut boundary を prose と照合する"
  ]
}
```

ここでも first pressure は human walkthrough で十分であり、
actual file exchange はまだ要らない。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. concrete notebook workflow pressure の first choice は human review checklist / walkthrough pressure に置く
2. compare / bless-like review flow は second step に残す
3. actual notebook file exchange / retained notebook artifact はさらに後段に残す
4. next later reopen は、review checklist / walkthrough unit を stable notebook bridge sketch としてどこまで named bundle に寄せるかの comparison に置く

## この段階でまだ固定しないもの

- compare / bless policy
- path / retention / overwrite policy
- emitted notebook artifact id
- `proof_assistant_adapter` specific schema
- public checker API
