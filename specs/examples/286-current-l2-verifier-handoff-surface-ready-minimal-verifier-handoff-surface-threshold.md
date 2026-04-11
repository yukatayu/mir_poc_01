# 286 — current L2 verifier-handoff-surface-ready minimal-verifier-handoff-surface threshold

## 目的

`specs/examples/285-current-l2-minimal-public-checker-boundary-ready-verifier-handoff-surface-comparison.md`
で verifier handoff surface comparison の first candidate を
docs-only mixed-row bridge に置く判断を fixed した次段として、

- verifier handoff surface の minimum をどこまでに留めるか
- public checker boundary / proof-obligation matrix / mixed handoff artifact default の relation を minimum にどう反映するか
- actual subject row / boundary-specific split / actual emitter をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
verifier-handoff-surface-ready minimal-verifier-handoff-surface threshold** であり、

- actual emitted verifier handoff artifact
- theorem / protocol / runtime-policy dedicated contract
- final parser grammar
- concrete tool binding

はまだ固定しない。

## 比較観点

1. verifier handoff surface と public checker boundary minimum を distinguish できるか
2. proof-obligation matrix / mixed row default の current judgment を minimum に反映できるか
3. subject row / emitted artifact / tool binding を premature に minimum に混ぜないか
4. next promoted line を narrow に `minimal-parser-subset-freeze comparison` へ進められるか

## 比較対象

### 案 1. `public_checker_boundary_ref + proof_obligation_matrix_ref` だけを持つ

#### shape

```text
verifier_handoff_surface_ready_sketch = {
  public_checker_boundary_ref = public_checker_boundary_ready_sketch,
  proof_obligation_matrix_ref = current_l2_proof_obligation_matrix
}
```

#### 利点

- 軽い
- public checker boundary から proof-obligation matrix へ戻れる

#### 欠点

- verifier handoff surface 自体の kind が prose 依存になる
- mixed row bundle default が minimum に現れない

### 案 2. `handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode` を持つ

#### shape

```text
verifier_handoff_surface_ready_sketch = {
  handoff_surface_kind = docs_only_mixed_row_bundle_verifier_surface,
  public_checker_boundary_ref = public_checker_boundary_ready_sketch,
  proof_obligation_matrix_ref = current_l2_proof_obligation_matrix,
  handoff_artifact_mode = docs_only_mixed_row_bundle
}
```

#### 利点

- verifier handoff surface が actual emitter ではなく docs-only mixed-row bridge だと minimum に見える
- proof-obligation matrix と mixed row bundle default の relation を minimum に反映できる
- subject row / boundary-specific split / actual emitter を still later に残せる

#### 欠点

- 案 1 より少し重い
- symbolic mode field の naming refinement は later で必要かもしれない

### 案 3. `subject_kind` / `subject_ref` / `proof_obligation_rows` や dedicated boundary ref まで入れる

#### 利点

- public-looking verifier surface を広く一度に見せられる

#### 欠点

- actual handoff artifact や boundary-specific split を premature に minimum へ入れやすい
- checker-side docs-only gate と external consumer contract を混ぜやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode` を持つ**
である。

理由は次の通り。

1. current verifier handoff surface が actual emitted artifact ではなく docs-only mixed-row bridge だという判断を minimum に反映できる
2. proof-obligation matrix と mixed row default の current cut を checker-side public line に接続できる
3. subject row、boundary-specific split、actual emitter を still later に残しやすい

## current first choice shape

```text
verifier_handoff_surface_ready_sketch = {
  handoff_surface_kind = docs_only_mixed_row_bundle_verifier_surface,
  public_checker_boundary_ref = public_checker_boundary_ready_sketch,
  proof_obligation_matrix_ref = current_l2_proof_obligation_matrix,
  handoff_artifact_mode = docs_only_mixed_row_bundle
}
```

### この shape でまだ入れないもの

- `subject_kind` / `subject_ref` / `proof_obligation_rows`
- theorem / protocol / runtime-policy dedicated handoff artifact family
- actual emitted verifier handoff artifact
- tool-specific schema / actual emitter policy
- final parser grammar / query token / shared generic entry

これらは still later である。

## practical reading

current minimal verifier handoff surface が示すのは、

- public checker boundary の次段は matrix-backed mixed-row bridge まででよい
- verifier handoff surface 自体は docs-only であり、actual row carrier ではない
- actual external consumer contract は later threshold に残る

という最小 cut である。

## next promoted line

next promoted line は、
**minimal-verifier-handoff-surface-ready minimal-parser-subset-freeze comparison**
に置く。

## open questions

- mixed row default を boundary-specific split へ切り替える threshold をどこに置くか
- emitted verifier handoff artifact を tool-neutral relation export から始めるべきか
- proof / protocol / runtime-policy closeout をどの order で続けるか
