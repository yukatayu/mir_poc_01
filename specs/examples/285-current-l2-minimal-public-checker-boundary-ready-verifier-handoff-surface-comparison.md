# 285 — current L2 minimal-public-checker-boundary-ready verifier-handoff-surface comparison

## 目的

`specs/examples/283-current-l2-minimal-shared-output-contract-ready-public-checker-boundary-comparison.md`
と
`specs/examples/284-current-l2-public-checker-boundary-ready-minimal-public-checker-boundary-threshold.md`
で public checker boundary の first cut を
docs-only parser-front bundle に固定した次段として、

- verifier handoff surface をどの docs-first cut から始めるべきか
- public checker boundary と proof-obligation matrix / mixed handoff artifact policy をどう接続するべきか
- actual subject row、boundary-specific handoff artifact、actual emitter をどこまで still later に残すべきか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
minimal-public-checker-boundary-ready verifier-handoff-surface comparison** であり、

- final parser grammar
- actual theorem / protocol / runtime-policy consumer schema
- actual emitted verifier handoff artifact

はまだ固定しない。

## scope

- current checker-side public-looking line だけを扱う。
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
  の proof-obligation matrix と row-based sketch は維持する。
- `specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md`
  の mixed row bundle default も維持する。
- `specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md`
  の theorem-side docs-only cutを壊さない。
- concrete external tool binding と actual emitter policy には進まない。

## current 前提

current repo では次が成立している。

1. public checker boundary の current minimum は
   `boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref`
   docs-only parser-front bundle である。
2. proof-obligation matrix は docs 正本であり、
   external handoff artifact は mixed row bundle sketch が current default である。
3. theorem-side retained bridge は
   `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body`
   を current stop lineにし、actual checker-side public lineとは still 分けている。
4. current Phase 6 front-half の immediate concern は、
   actual verifier schema を書くことではなく、
   public checker boundary の次段で verifier-facing relation を narrow に凍結することである。

したがって current 問いは、
**public checker boundary の次段として、どこまでを verifier handoff surface と呼ぶと narrow で、しかも later reopen に耐えるか**
である。

## 比較観点

1. public checker boundary と actual handoff artifact を premature に同一視しないか
2. proof-obligation matrix と mixed row bundle default の current judgment を surface 側に反映できるか
3. actual subject row / boundary-specific split / emitted artifact を premature に minimum へ混ぜないか
4. next promoted line を Phase 3 minimal parser subset freeze に切り替えられる程度に gate を narrow に閉じられるか

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
- mixed row bundle default を current minimum に反映しにくい
- public checker boundary と matrix の単なる adjacency に見えやすい

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

- public checker boundary の次段が actual emitted artifact ではなく docs-only mixed-row bridge だと minimum に出せる
- `specs/examples/127...128` の current judgment を checker-side public line に接続できる
- theorem / protocol / runtime-policy split は matrix 側へ残しつつ、surface minimum は narrow に保てる
- actual subject row / boundary-specific split / emitted artifact を still later に残せる

#### 欠点

- 案 1 より 2 field 重い
- `handoff_surface_kind` / `handoff_artifact_mode` の later naming refinement は未決である

### 案 3. subject row や boundary-specific handoff ref まで minimum に入れる

#### 例

- `subject_kind`
- `subject_ref`
- `proof_obligation_rows`
- `theorem_handoff_artifact_ref`
- `protocol_handoff_artifact_ref`
- `runtime_policy_handoff_artifact_ref`

#### 利点

- public-looking verifier surface を一気に広く見せられる

#### 欠点

- actual row bundle / boundary split / emitter pressure を premature に minimum へ混ぜる
- `specs/examples/128...132` の docs-only cutを壊しやすい
- Phase 3 reopen 前に checker-side surface を schema 化しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode` を持つ**
である。

理由は次の通り。

1. public checker boundary の次段として必要なのは actual row artifact ではなく、matrix-backed mixed-row bridge だという current judgment である
2. `specs/examples/127...128` の proof-obligation matrix / mixed row bundle default を public checker side に narrow に接続できる
3. subject row、boundary-specific split、actual emitter は still later に残せる
4. Phase 5 の immediate gate をここで narrow に閉じれば、主線を Phase 3 minimal parser subset freeze へ移せる

## current first choice shape

```text
verifier_handoff_surface_ready_sketch = {
  handoff_surface_kind = docs_only_mixed_row_bundle_verifier_surface,
  public_checker_boundary_ref = public_checker_boundary_ready_sketch,
  proof_obligation_matrix_ref = current_l2_proof_obligation_matrix,
  handoff_artifact_mode = docs_only_mixed_row_bundle
}
```

## この shape でまだ入れないもの

- `subject_kind` / `subject_ref` / `proof_obligation_rows`
- theorem / protocol / runtime-policy dedicated handoff artifact ref
- actual emitted verifier handoff artifact
- concrete theorem prover / protocol verifier / runtime-policy tool schema
- final parser grammar / query token / shared generic entry

これらは still later である。

## practical reading

current minimal verifier handoff surface が示すのは、

- public checker boundary の次段は actual emitted artifact ではなく docs-only mixed-row bridge である
- proof-obligation matrix が正本であり、mixed row bundle default は still 維持される
- subject row / boundary-specific split / actual emitter は later pressure が出たときだけ reopen する

の 3 点だけである。

## next promoted line

next promoted line は、
**minimal-verifier-handoff-surface-ready minimal-parser-subset-freeze comparison**
に置く。

## open questions

- `handoff_surface_kind` / `handoff_artifact_mode` の later public naming をどう整えるか
- theorem / protocol / runtime-policy dedicated split をどの threshold で reopen するか
- actual emitted verifier handoff artifact をどの tool-neutral carrier から始めるべきか
