# 258 — current L2 minimal-checker-cluster-row-ready checker-cluster-fixture-evidence-attachment comparison

## 目的

`specs/examples/257-current-l2-checker-cluster-matrix-ready-minimal-checker-cluster-row-threshold.md`
で checker-cluster row core を
`cluster_kind + checker_subject + decidable_class + external_handoff`
に留める判断を前提に、

- row core を変えずに `fixture_evidence_refs` をどこまで attachment として足すのが自然か
- `fixture_evidence_refs` と `typed_reason_family_hint` のどちらを先に扱うべきか
- current static-only baseline、helper-local checker spike、detached static gate artifact との接点をどこまで docs-first に見せてよいか

を比較する。

ここで固定するのは **checker-cluster fixture evidence attachment の current comparison** であり、

- actual checker artifact payload
- typed reason family の final cluster-wide actualization
- public checker API

はまだ固定しない。

## scope

- current L2 first checker cut 候補 cluster の docs-only matrix を扱う。
- `fixture_evidence_refs` attachment の比較に限る。
- existing `checked_reason_codes`、detached static gate `reason_codes`、checker spike helpers は source evidence として参照する。
- actual helper / fixture / schema の変更は行わない。

## current 前提

current repo では次が成立している。

1. same-lineage / missing-option / capability は helper-local checker spike として actual evidence がある。
2. `try` / rollback locality structural floor は separate helper-local spike として current docs に接続済みである。
3. stable cluster inventory として、`checked_reason_codes`・detached static gate `reason_codes`・static-only corpus baseline の 3 者接点がある。
4. minimal checker-cluster row core は `cluster_kind + checker_subject + decidable_class + external_handoff` に留める cut が固定済みである。

したがって current 問いは、
**checker-cluster row core の次段として、fixture-side / helper-side evidence を row attachment としてどこまで docs-first に足すべきか**
である。

## 比較観点

1. row core と evidence attachment の責務を分けられるか
2. current checker spike / static-only baseline との接点を user-facing に示せるか
3. `typed_reason_family_hint` を premature に cluster core へ押し込まないか
4. next threshold として typed reason hint comparison に narrow に進めるか

## 比較対象

### 案 1. minimal row core のまま、evidence attachment をまだ足さない

#### 利点

- row core は最も軽いまま保てる
- current fixture corpus に依存しない

#### 欠点

- current checker spike / static-only baseline / detached static gate artifact との bridge が prose 依存のままになる
- checker-side line が docs-only inventory に留まりすぎ、existing source evidence が user-facing に見えにくい

### 案 2. `fixture_evidence_refs` を row attachment として足す

#### 読み

row core とは別に、次の attachment を許す。

```text
checker_cluster_row_attachment = {
  fixture_evidence_refs
}
```

`fixture_evidence_refs` は、たとえば

- static-only corpus baseline の fixture 群
- helper-local checker spike spec / report anchor
- detached static gate artifact family anchor

を symbolic ref として束ねる。

#### 利点

- row core と evidence attachment を分けたまま、existing source evidence との bridge を 1 段だけ足せる
- same-lineage / missing-option / capability / `try` structural floor の current coverage を user-facing に説明しやすい
- `typed_reason_family_hint` より先に evidence layer を切れるので、cluster ごとの readiness 差を吸収しやすい

#### 欠点

- attachment shape を docs-only でも 1 段増やすことになる
- concrete fixture 名をどこまで row attachment に含めるかの discipline が要る

### 案 3. `fixture_evidence_refs` を飛ばして `typed_reason_family_hint` を先に扱う

#### 利点

- checker-side typed carrier への道筋は早く見える
- `checked_reason_codes` と cluster matrix の接点が direct に見える

#### 欠点

- typed reason family を持つ cluster と持たない cluster の readiness 差を premature に潰しやすい
- `request_option_clause_attachment` / `minimal_predicate_fragment_well_formedness` / `try_rollback_locality_structural_floor` を同じ speed で typed reason hint に寄せにくい
- evidence layer を飛ばすと、why this cluster is source-backed かが逆に見えにくい

## current judgment

current L2 で最も自然なのは、
**案 2. `fixture_evidence_refs` を row attachment として足す**
ことである。

理由は次の通り。

1. row core と evidence attachment を分けたまま、current repo に already ある source evidence と接続できる
2. `typed_reason_family_hint` をまだ cluster-wide に要求せずに済む
3. checker-side line を theorem-line と違う形で具体化しつつ、actual checker artifact を premature に固定しない

## current attachment reading

current docs-only attachment の読みは次でよい。

```text
checker_cluster_row_attachment = {
  fixture_evidence_refs = [
    { ref_kind = static_baseline_fixture, ref_id = ... },
    { ref_kind = checker_spike_anchor,    ref_id = ... },
    { ref_kind = static_gate_anchor,      ref_id = ... }
  ]
}
```

### practical example A — same-lineage floor

```text
checker_cluster_row = {
  cluster_kind     = same_lineage_static_evidence_floor,
  checker_subject  = fallback_chain_edge,
  decidable_class  = local_structural,
  external_handoff = theorem_prover_boundary
}

checker_cluster_row_attachment = {
  fixture_evidence_refs = [
    { ref_kind = static_baseline_fixture, ref_id = e4_malformed_lineage },
    { ref_kind = static_baseline_fixture, ref_id = e12_underdeclared_target_missing },
    { ref_kind = checker_spike_anchor,    ref_id = same_lineage_first_checker_spike }
  ]
}
```

### practical example B — capability floor

```text
checker_cluster_row = {
  cluster_kind     = minimal_capability_strengthening_floor,
  checker_subject  = fallback_chain_capability_edge,
  decidable_class  = local_structural,
  external_handoff = theorem_prover_boundary
}

checker_cluster_row_attachment = {
  fixture_evidence_refs = [
    { ref_kind = static_baseline_fixture, ref_id = e13_malformed_capability_strengthening },
    { ref_kind = static_baseline_fixture, ref_id = e20_malformed_late_capability_strengthening },
    { ref_kind = checker_spike_anchor,    ref_id = capability_third_checker_spike }
  ]
}
```

## next promoted line

next promoted line は、
**checker-cluster-fixture-evidence-attachment-ready typed-reason-family-hint threshold**
に置く。

ここで比べるのは、
`typed_reason_family_hint` を current checker-cluster line に本当に足してよいか、
それとも cluster ごとの readiness 差を残したまま still later に置くべきかである。

## what is decided here

### decided

- minimal checker-cluster row core の次段としては `fixture_evidence_refs` attachment が自然である
- `fixture_evidence_refs` は row core ではなく attachment として切る
- `typed_reason_family_hint` は still next threshold に残す

### not decided

- `fixture_evidence_refs` attachment の final field set
- `typed_reason_family_hint` を optional attachment にするか
- actual checker-cluster artifact / public checker API

## open questions

- `fixture_evidence_refs` を row-local に置くか cluster-bundle-local に置くか
- `typed_reason_family_hint` を stable cluster だけ先行 actualize してよいか
- checker-cluster matrix から theorem prover / protocol verifier handoff row へ再合流させる必要があるか
