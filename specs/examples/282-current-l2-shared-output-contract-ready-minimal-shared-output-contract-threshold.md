# 282 — current L2 shared-output-contract-ready minimal-shared-output-contract threshold

## 目的

`specs/examples/281-current-l2-minimal-public-checker-command-surface-ready-shared-output-contract-comparison.md`
で shared output contract comparison の first candidate を
family checker shared helper の summary line に置く判断を fixed した次段として、

- shared output contract の minimum をどこまでに留めるか
- family checker shared summary と public checker payload schema の接点をどこまで explicit に持たせるか
- detached loop wrapper / parser boundary / verifier handoff をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
shared-output-contract-ready minimal-shared-output-contract threshold** であり、

- parser-front public checker boundary
- generic shared public checker entry
- emitted verifier handoff surface

はまだ固定しない。

## 比較観点

1. command surface minimum と distinguish できる emitted-output minimum になっているか
2. current shared helper の `cluster` / `status` summary を source-backed に反映しつつ still minimal に保てるか
3. detached loop wrapper line / row snippet text / parser boundary / verifier handoff を premature に minimum へ混ぜないか
4. next promoted line を narrow に `public-checker-boundary comparison` へ進められるか

## 比較対象

### 案 1. `checker_status + public_checker_payload_schema_ref` だけを持つ

#### shape

```text
shared_output_contract_ready_sketch = {
  checker_status,
  public_checker_payload_schema_ref = public_checker_payload_schema_ready_sketch
}
```

#### 利点

- 最も軽い
- payload schema との relation を残せる

#### 欠点

- current shared helper が actual に出している `cluster: ...` summary が minimum に現れない
- output contract kind が still prose 依存になりやすい

### 案 2. `output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref` を持つ

#### shape

```text
shared_output_contract_ready_sketch = {
  output_contract_kind = family_checker_row_compare_summary,
  checker_cluster_name,
  checker_status,
  public_checker_payload_schema_ref = public_checker_payload_schema_ready_sketch
}
```

#### 利点

- emitted output line が family checker shared summary に anchored していることを minimum に反映できる
- `cluster` / `status` summary と payload schema の接点を 1 段で読める
- detached loop wrapper / parser boundary / verifier handoff を still later に残せる

#### 欠点

- 案 1 より 1 段重い
- `checker_cluster_name` の later normalization は未決のまま残る

### 案 3. wrapper path line や row snippet text まで minimum に入れる

#### 利点

- current console output を広く 1 度に拾える

#### 欠点

- `static gate artifact:` line や `fixture rows:` / `actual rows:` text を premature に contract 化しやすい
- parser boundary / verifier handoff と presentation detail が混ざりやすい
- current threshold としては still heavy である

## current judgment

current L2 で最も自然なのは、
**案 2. `output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref` を持つ**
である。

理由は次の通り。

1. shared output contract として 1 段切る以上、current source が actual に出している `cluster` / `status` summary を minimum に見せる方が自然である
2. ただし wrapper path line や row snippet text まで minimum に入れるのは still early である
3. `public_checker_payload_schema_ref` を参照すれば、emitted output と payload-side docs-only bridge の接点を narrow に保てる

## current first choice shape

```text
shared_output_contract_ready_sketch = {
  output_contract_kind = family_checker_row_compare_summary,
  checker_cluster_name,
  checker_status,
  public_checker_payload_schema_ref = public_checker_payload_schema_ready_sketch
}
```

### `checker_status` の current reading

current source-backed minimum が示す `checker_status` は、

- shared status
  - `matched`
  - `mismatch`
  - `out_of_scope`
- family-local missing status
  - `fixture_same_lineage_rows_missing`
  - `fixture_missing_option_rows_missing`
  - `fixture_capability_rows_missing`

を含む string status である。

current task では、この status family を richer typed enum へは昇格させない。

### この shape でまだ入れないもの

- `static_gate_artifact_path`
- `fixture_path`
- `artifact_path`
- `fixture rows:` textual rendering
- `actual rows:` textual rendering
- parser-front public checker entry
- generic shared public checker entry
- emitted verifier handoff surface

これらは still later である。

## practical reading

current minimal shared output contract が示すのは、

- public checker family facade command は shared に読める emitted summary line を already 持っている
- その minimum は `cluster` / `status` summary と payload schema ref の接点までである
- wrapper path line や row snippet text は current source-backed evidence だが、minimal shared output contract の first-class field にはまだ上げない

の 3 点だけである。

## next promoted line

next promoted line は、
**minimal-shared-output-contract-ready public-checker-boundary comparison**
に置く。

## open questions

- `checker_cluster_name` を later で `checker_cluster_ref` に寄せるべきか
- row snippet textual rendering を later public output contract に含めるべきか
- parser-front public checker boundary を shared output contract の次段でどこまで narrow に切るべきか
