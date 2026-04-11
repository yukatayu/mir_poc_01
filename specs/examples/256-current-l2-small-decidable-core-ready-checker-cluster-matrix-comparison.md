# 256 — current L2 small-decidable-core-ready checker-cluster-matrix comparison

## 目的

`specs/examples/255-current-l2-theorem-line-minimal-handoff-transport-channel-body-ready-low-level-memory-order-family-threshold.md`
で theorem-line retained bridge を
`retained_payload_body_materialization_theorem_export_handoff_transport_channel_body`
で止め、next promoted line を small decidable core 側へ戻す判断を前提に、

- current L2 の first checker cut 候補 cluster を docs-first にどこまで matrix 化してよいか
- prose inventory のまま保つか、docs-only checker-cluster matrix を置くか、minimal typing judgment skeleton に進むか
- local / structural / decidable floor を user-facing に見せつつ、public checker API や final type system を premature に固定しない cut をどう取るか

を比較する。

ここで固定するのは **current Phase 5 の
small-decidable-core-ready checker-cluster-matrix comparison** であり、

- final type system strength
- actual public checker artifact
- theorem prover relation の最終形

はまだ固定しない。

## scope

- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
  と
  `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
  を前提にする。
- current L2 parser-free PoC と detached validation loop は変えない。
- docs-only matrix の比較に限る。
- actual checker implementation / public checker API / actual typing judgment rule actualization には進まない。

## current 前提

current repo では次が成立している。

1. first checker cut に入れてよい cluster は、
   - same-lineage static evidence floor
   - malformed / underdeclared rejection
   - minimal capability strengthening prohibition
   - request-local / option-local clause attachment
   - minimal predicate fragment well-formedness
   - `try` / rollback locality の structural floor
   に限る。
2. current first choice は
   `core_static_checker` / `theorem_prover_boundary` / `protocol_verifier_boundary` / `runtime_policy_boundary`
   の 4-way split である。
3. theorem-line retained bridge は current package で
   `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body`
   を stop line にし、low-level memory-order family は still later に残す cut が固定済みである。

したがって current 問いは、
**Phase 5 の next promoted line として、small decidable core の first checker cut 候補 cluster をどこまで docs-first に matrix 化すると、type/static checking の全体像を見せつつ premature actualization を避けられるか**
である。

## 比較観点

1. current first checker cut の 6 cluster を誤読しにくく見せられるか
2. `core_static_checker` と external boundary の split を保てるか
3. final type system や public checker API を premature に既成事実化しないか
4. `checked_reason_codes`、detached static gate artifact、stage 1 / 2 / 3 parser-side evidence との接点を説明しやすいか
5. 次段として minimal checker-cluster row threshold へ narrow に進めるか

## 比較対象

### 案 1. `30` と `126` の prose inventory のまま維持する

#### 利点

- 追加 row を作らずに済む
- public checker API 誤読を増やしにくい

#### 欠点

- 6 cluster の相対関係が prose を横断しないと見えにくい
- type/static checking の全体像を rough に掴みにくい
- next threshold line が prose 依存になりやすい

### 案 2. docs-only checker-cluster matrix を置く

#### 読み

first checker cut 候補 cluster を、たとえば次の row bundle としてまとめる。

```text
checker_cluster_matrix = [
  {
    cluster_kind = same_lineage_floor,
    checker_subject = fallback_chain_edge,
    decidable_class = local_structural,
    external_handoff = theorem_prover_boundary
  },
  {
    cluster_kind = malformed_underdeclared,
    checker_subject = malformed_surface_shape,
    decidable_class = local_structural,
    external_handoff = core_static_checker_terminal
  },
  ...
]
```

#### 利点

- current first checker cut の 6 cluster を 1 箇所で見せられる
- docs-only matrix なので public checker API をまだ固定しない
- `core_static_checker` と external boundary の split を表で見せやすい
- next threshold として minimal checker-cluster row core を狭く比較しやすい

#### 欠点

- matrix という語が actual emitted artifact と誤読される余地がある
- row field 名を docs-only に留める注意が要る

### 案 3. minimal typing judgment skeleton へすぐ進む

#### 読み

たとえば

```text
Γ ⊢ cluster_ok
```

のような typing / checking judgment skeleton を current next line にする。

#### 利点

- type system を考える入口が早く見える
- proof assistant 向け relation と接続しやすい

#### 欠点

- current phase では still 早い
- final type system / annotation burden / public checker API と結び付きやすい
- 6 cluster を docs-only inventory として整理する前に judgment 記法を固定しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. docs-only checker-cluster matrix を置く**
である。

理由は次の通り。

1. current Phase 5 の next step では、まず 6 cluster の関係と split を見える化する方が自然である
2. docs-only matrix なら final type system / public checker API を premature に固定せずに済む
3. その後で minimal checker-cluster row core や typing judgment skeleton へ narrow に進める余地を守れる

## current matrix candidates

current docs-only matrix に載せてよい row 候補は次の 6 cluster である。

1. `same_lineage_static_evidence_floor`
2. `malformed_underdeclared_rejection`
3. `minimal_capability_strengthening_floor`
4. `request_option_clause_attachment`
5. `minimal_predicate_fragment_well_formedness`
6. `try_rollback_locality_structural_floor`

### まだ入れないもの

- richer predicate grammar
- stronger capability / contract theory
- theorem-level invariant proof
- model-checking を要する global protocol property
- final typing judgment notation

## practical example

### example A — same-lineage floor

```text
chain doc_ref = writer
  fallback readonly
    @ lineage(writer -> readonly)
```

ここでは matrix row に今ほしいのは、

- `cluster_kind = same_lineage_static_evidence_floor`
- `checker_subject = fallback_chain_edge`
- `decidable_class = local_structural`

であって、
actual public checker payload ではない。

### example B — `try` / rollback locality

```text
try {
  perform on doc_ref
  atomic_cut
} fallback {
  perform on recovery_log
}
```

ここでは matrix row に今ほしいのは、

- `cluster_kind = try_rollback_locality_structural_floor`
- `checker_subject = try_region_shape`
- `external_handoff = theorem_prover_boundary`

であって、
rollback-cut non-interference の一般証明そのものではない。

## next promoted line

next promoted line は、
**checker-cluster-matrix-ready minimal-checker-cluster-row threshold**
に置く。

ここで比べる first candidates は、少なくとも次である。

1. `cluster_kind + checker_subject + decidable_class + external_handoff`
2. 上に `fixture_evidence_refs` を足す
3. 上に `typed_reason_family_hint` まで足す

## what is decided here

### decided

- current Phase 5 の next line は theorem-line ではなく small decidable core 側へ戻す
- first checker cut の 6 cluster を docs-only checker-cluster matrix として比較してよい
- minimal typing judgment skeleton や public checker API は still 後段に残す

### not decided

- minimal checker-cluster row の final field set
- final type system
- actual checker artifact / API
- theorem prover relation の最終 encoding

## open questions

- checker-cluster matrix row に `fixture_evidence_refs` を入れるべきか
- `typed_reason_family_hint` を current line でまだ入れない方がよいか
- minimal typing judgment skeleton をどの threshold で reopen するか
