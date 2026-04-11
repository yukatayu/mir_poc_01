# 257 — current L2 checker-cluster-matrix-ready minimal-checker-cluster-row threshold

## 目的

`specs/examples/256-current-l2-small-decidable-core-ready-checker-cluster-matrix-comparison.md`
で current Phase 5 の next promoted line を
`checker-cluster-matrix-ready minimal-checker-cluster-row threshold`
に置く判断を前提に、

- docs-only checker-cluster matrix の 1 row をどの field で止めるのが最小か
- `fixture_evidence_refs` や `typed_reason_family_hint` を row core に混ぜるべきか
- first checker cut の 6 cluster を user-facing に見せつつ、actual checker artifact と誤読されない cut をどう取るか

を比較する。

ここで固定するのは **minimal checker-cluster row の current docs-first threshold** であり、

- actual checker artifact
- public checker API
- final typing judgment notation

はまだ固定しない。

## scope

- current L2 の first checker cut 候補 6 cluster を扱う。
- checker-cluster matrix は docs-only inventory として扱う。
- parser-free PoC、detached static gate artifact、shared-space line は変えない。
- actual helper / checker 実装の actualization には進まない。

## current 前提

current repo では次が成立している。

1. first checker cut 候補 cluster は、
   - `same_lineage_static_evidence_floor`
   - `malformed_underdeclared_rejection`
   - `minimal_capability_strengthening_floor`
   - `request_option_clause_attachment`
   - `minimal_predicate_fragment_well_formedness`
   - `try_rollback_locality_structural_floor`
   の 6 つに限る。
2. current first choice は `core_static_checker` / `theorem_prover_boundary` /
   `protocol_verifier_boundary` / `runtime_policy_boundary` の 4-way split である。
3. theorem-line retained bridge は `specs/examples/255-current-l2-theorem-line-minimal-handoff-transport-channel-body-ready-low-level-memory-order-family-threshold.md`
   で stop line を固定済みであり、current promoted line は checker-cluster 側へ戻している。

したがって current 問いは、
**checker-cluster matrix の 1 row core に何を残し、何を row attachment や later threshold へ回すべきか**
である。

## 比較観点

1. 6 cluster の相対関係と external boundary を誤読しにくく見せられるか
2. `core_static_checker` と source evidence を混ぜすぎないか
3. final type system / public checker API を premature に既成事実化しないか
4. 次段として evidence attachment や typed reason hint へ narrow に進めるか

## 比較対象

### 案 1. `cluster_kind + checker_subject + decidable_class + external_handoff`

#### 読み

row core を次の 4 field に留める。

```text
checker_cluster_row = {
  cluster_kind,
  checker_subject,
  decidable_class,
  external_handoff
}
```

#### 利点

- row core が cluster の意味に集中する
- `core_static_checker` と theorem / protocol / runtime boundary の split を 1 row で見せやすい
- source evidence や typed reason family を premature に縛らない
- final checker artifact と誤読されにくい

#### 欠点

- current fixture corpus や helper-local spike との接点は row 単体では見えない
- detached static gate artifact との bridge は次段に回る

### 案 2. 案 1 に `fixture_evidence_refs` を含める

#### 読み

row core を次に広げる。

```text
checker_cluster_row = {
  cluster_kind,
  checker_subject,
  decidable_class,
  external_handoff,
  fixture_evidence_refs
}
```

#### 利点

- row と current corpus evidence の対応が 1 箇所で読める
- helper-local checker spike や static-only baseline への bridge が見えやすい

#### 欠点

- cluster core と evidence attachment が同じ row core に混ざる
- current fixture corpus の状態を row definition そのものと誤読しやすい
- evidence family の変化で row core 自体が揺れやすい

### 案 3. 案 2 に `typed_reason_family_hint` まで含める

#### 読み

row core を次に広げる。

```text
checker_cluster_row = {
  cluster_kind,
  checker_subject,
  decidable_class,
  external_handoff,
  fixture_evidence_refs,
  typed_reason_family_hint
}
```

#### 利点

- `checked_reason_codes` や detached static gate `reason_codes` との接点が早く見える
- future checker artifact に近い見え方になる

#### 欠点

- stable typed reason family を持つ cluster とそうでない cluster を同じ pace で縛りやすい
- request clause / predicate fragment / `try` structural floor など、まだ typed reason family を immediate に要求したくない cluster を premature に拘束する
- docs-only matrix から actual checker artifact への誤読が強くなる

## current judgment

current L2 で最も自然なのは、
**案 1. `cluster_kind + checker_subject + decidable_class + external_handoff`**
に留めることである。

理由は次の通り。

1. current line の目的は、まず 6 cluster の **意味の骨格** を stable に見せることである
2. `fixture_evidence_refs` は cluster core ではなく evidence attachment として別 layer に切る方が自然である
3. `typed_reason_family_hint` は current phase では still early であり、cluster ごとの readiness 差を吸収できない
4. final type system / public checker API を premature に既成事実化しない

## current minimal row

current docs-only row core は次でよい。

```text
checker_cluster_row = {
  cluster_kind,
  checker_subject,
  decidable_class,
  external_handoff
}
```

### practical example A — same-lineage floor

```text
{
  cluster_kind     = same_lineage_static_evidence_floor,
  checker_subject  = fallback_chain_edge,
  decidable_class  = local_structural,
  external_handoff = theorem_prover_boundary
}
```

### practical example B — `try` / rollback locality

```text
{
  cluster_kind     = try_rollback_locality_structural_floor,
  checker_subject  = try_region_shape,
  decidable_class  = local_structural,
  external_handoff = theorem_prover_boundary
}
```

この row は cluster の意味を示すものであり、
fixture corpus、`checked_reason_codes`、detached static gate artifact の actual row そのものではない。

## next promoted line

next promoted line は、
**minimal-checker-cluster-row-ready checker-cluster-fixture-evidence-attachment comparison**
に置く。

ここで比べるのは、
row core を変えずに `fixture_evidence_refs` を row attachment としてどこまで足すかである。

## what is decided here

### decided

- checker-cluster matrix の row core は 4 field に留める
- `fixture_evidence_refs` は row core に混ぜない
- `typed_reason_family_hint` は still 後段に残す

### not decided

- `fixture_evidence_refs` attachment の final shape
- `typed_reason_family_hint` をどの threshold で reopen するか
- actual checker-cluster artifact / API
- final typing judgment notation

## open questions

- `fixture_evidence_refs` attachment を row-local list にするか、cluster-local bundle にするか
- `typed_reason_family_hint` を cluster ごとの optional attachment として扱うべきか
- minimal row core を future parser/public checker line がそのまま再利用できるか
