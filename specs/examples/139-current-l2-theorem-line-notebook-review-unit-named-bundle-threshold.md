# 139 — current L2 theorem line notebook review unit named-bundle threshold

## 目的

`specs/examples/136-current-l2-theorem-line-notebook-bridge-artifact-threshold.md`、
`specs/examples/137-current-l2-theorem-line-next-consumer-pressure-comparison.md`、
`specs/examples/138-current-l2-theorem-line-concrete-notebook-workflow-pressure-comparison.md`
までを前提に、

- review checklist / walkthrough unit を stable notebook bridge sketch へどこまで寄せるか
- そのとき、named bundle を docs-only bridge として切ってよいか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の notebook review unit named-bundle threshold** であり、

- actual emitted notebook artifact
- compare / bless policy
- notebook file format / retained path / overwrite policy
- `proof_assistant_adapter` specific schema
- public checker API

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- theorem-side minimum contract row core は `obligation_kind + evidence_refs` のまま維持する。
- notebook attachment family は `goal_text` のまま維持する。
- concrete notebook workflow pressure の current first threshold は human review checklist / walkthrough pressure のまま維持する。
- shared-space / protocol / runtime line は巻き込まない。

## current 前提

current repo では次が成立している。

1. `proof_notebook` first bridge は current phase では docs-only derived view に留める。
2. concrete notebook workflow pressure の first threshold は human review checklist / walkthrough pressure に置く。
3. compare / bless-like flow は second step に残し、actual notebook file exchange はさらに後段に残す。
4. minimum contract row core は `obligation_kind + evidence_refs` に留め、`goal_text` は lightweight attachment に留める。

したがって current 問いは、
**human walkthrough pressure を受けて、review unit を docs prose のまま維持するか、docs-only named bundle へ寄せるか、それとも stronger notebook bridge sketch へ進めるか**
である。

## 比較観点

1. `goal_text` 付き docs-only derived view の current cut を壊さないか
2. human review workflow と machine-facing bridge contract を混ぜないか
3. compare / bless / retained file policy を premature に固定しないか
4. `proof_assistant_adapter` line や actual emitter line より先に進みすぎないか
5. later reopen が必要になったときに、named review unit から narrow に再開できるか

## 比較対象

### 案 1. review checklist / walkthrough unit は unnamed prose のまま維持する

#### 読み

`specs/examples/138...` の human walkthrough pressure は認めるが、
review unit は named bundle にせず prose / checklist 記述のまま運用する。

#### 利点

- current docs-only bridge を最も動かさない
- new bundle 名や pseudo-schema を増やさない
- actual artifact や public API との混同を最小にしやすい

#### 欠点

- reusable review unit を繰り返し参照しにくい
- later reopen の start point が prose 依存になりやすい
- `subject_ref + row + checklist` の最小反復単位が曖昧なまま残る

### 案 2. docs-only named review unit bundle を stable notebook bridge sketch の最小 cut にする

#### 読み

review checklist / walkthrough pressure を受けて、
current phase では still docs-only のまま
**named review unit bundle**
を切る。

最小 shape は次に留める。

```text
proof_notebook_review_unit = {
  subject_ref,
  row: {
    obligation_kind,
    evidence_refs,
    goal_text
  },
  checklist
}
```

#### 利点

- current human walkthrough pressure と最も連続的である
- actual file / bless / retention policy をまだ巻き込まない
- named review unit という再利用単位を作れる
- later reopen で compare / bless-like flow や emitted bridge sketch を narrow に比べやすい

#### 欠点

- docs-only とはいえ bundle 名が 1 つ増える
- stable notebook bridge sketch と public-looking schema を誤読させない discipline が要る

### 案 3. compare / bless-like flow を前提に stronger notebook bridge sketch へ進める

#### 読み

review unit を named bundle にするだけでなく、
compare / bless metadata も含めた stronger notebook bridge sketch へ進める。

```text
proof_notebook_bridge = {
  review_unit,
  previous_snapshot,
  reviewer_notes,
  bless_decision
}
```

#### 利点

- later compare / bless flow に直接つながる
- notebook bridge sketch を stronger contract として語りやすい

#### 欠点

- detached validation loop の bless / retention / overwrite policy を早く巻き込みやすい
- actual notebook bridge artifact や stable contract pressure と混ざりやすい
- current phase の docs-only bridge からの跳躍が大きい

## current judgment

current L2 で最も自然なのは、
**案 2. docs-only named review unit bundle を stable notebook bridge sketch の最小 cut にする**
である。

理由は次の通り。

1. `specs/examples/138...` で fixed した human review checklist / walkthrough pressure と最も連続的である
2. minimum contract row core と `goal_text` attachment をそのまま再利用できる
3. compare / bless / retained file pressure を premature に混ぜずに、named reuse unit だけを取り出せる
4. later reopen で actual bridge sketch や emitted artifact を比べるときも、start point を prose ではなく named review unit に置ける

## minimal named review unit sketch

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_review_unit = {
  subject_ref,
  row: {
    obligation_kind,
    evidence_refs,
    goal_text
  },
  checklist
}
```

### `subject_ref`

fixture id、cluster id など、
既存 source evidence に戻れる ref だけを置く。

### `row`

`row` は minimum contract row core に `goal_text` attachment を足したものに留める。

### `checklist`

`checklist` は reviewer が row を追うための human-facing 手順であり、
machine-check rule や bless policy を入れない。

## なぜ `proof_notebook_bridge` まで進めないか

current pressure は still human walkthrough であり、
compare / bless policy は second step に残っている。

したがって current phase で stronger bridge sketch まで進めると、

- detached loop bless policy
- retained notebook path
- overwrite / retention rule

が premature に混ざりやすい。

current named bundle は **review unit** に留め、
bridge sketch 本体は later reopen に残す。

## practical examples

### example A — fallback chain review unit

```text
proof_notebook_review_unit = {
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
    "obligation_kind を確認する",
    "fixture と static gate artifact を追う",
    "goal_text を prose と照合する"
  ]
}
```

ここで必要なのは named review unit であり、
actual notebook file や bless decision ではない。

### example B — `try` / `atomic_cut` review unit

```text
proof_notebook_review_unit = {
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

この段階でも row-local named review unit で十分であり、
stronger compare / bless flow はまだ要らない。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. notebook review checklist / walkthrough unit は docs-only named bundle に寄せてよい
2. named bundle の最小 shape は `subject_ref + row(obligation_kind + evidence_refs + goal_text) + checklist` に留める
3. compare / bless-like flow を含む stronger notebook bridge sketch は second step に残す
4. actual emitted notebook artifact はさらに後段に残す
5. next later reopen は、named review unit を stronger notebook bridge sketch へどこまで拡張するかの comparison に置く

## この段階でまだ固定しないもの

- compare / bless policy
- retained notebook path
- overwrite / retention / bless state policy
- emitted notebook artifact id
- `proof_assistant_adapter` specific schema
- public checker API
