# 284 — current L2 public-checker-boundary-ready minimal-public-checker-boundary threshold

## 目的

`specs/examples/283-current-l2-minimal-shared-output-contract-ready-public-checker-boundary-comparison.md`
で public checker boundary comparison の first candidate を
docs-only parser-front boundary に置く判断を fixed した次段として、

- public checker boundary の minimum をどこまでに留めるか
- command surface と shared output contract の接点をどこまで explicit に持たせるか
- final parser grammar / generic shared entry / verifier handoff をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
public-checker-boundary-ready minimal-public-checker-boundary threshold** であり、

- final parser grammar
- generic shared public checker entry
- emitted verifier handoff surface

はまだ固定しない。

## 比較観点

1. shared output contract minimum と distinguish できる boundary minimum になっているか
2. command surface と shared output contract の接点を minimum に反映しつつ still minimal に保てるか
3. final parser grammar / query token / verifier handoff を premature に minimum へ混ぜないか
4. next promoted line を narrow に `verifier-handoff-surface comparison` へ進められるか

## 比較対象

### 案 1. `public_checker_command_surface_ref + shared_output_contract_ref` だけを持つ

#### shape

```text
public_checker_boundary_ready_sketch = {
  public_checker_command_surface_ref = public_checker_command_surface_ready_sketch,
  shared_output_contract_ref = shared_output_contract_ready_sketch
}
```

#### 利点

- 軽い
- command side / output side の relation は見える

#### 欠点

- boundary 自体の kind が still prose 依存になる
- parser-front docs-only boundary だという current judgment が minimum に現れない

### 案 2. `boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref` を持つ

#### shape

```text
public_checker_boundary_ready_sketch = {
  boundary_kind = docs_only_parser_front_checker_boundary,
  public_checker_command_surface_ref = public_checker_command_surface_ready_sketch,
  shared_output_contract_ref = shared_output_contract_ready_sketch
}
```

#### 利点

- current public checker boundary が docs-only parser-front boundary だという judgment を minimum に反映できる
- command surface と shared output contract の接点を明示できる
- final parser grammar / generic shared entry / verifier handoff を still later に残せる

#### 欠点

- 案 1 より 1 段重い
- `boundary_kind` の later naming refinement は未決である

### 案 3. query token / parser input / verifier handoff 近傍 field まで minimum に入れる

#### 利点

- public-looking boundary を広く一度に表せる

#### 欠点

- final parser grammar / input surface / verifier handoff を premature に既成事実化しやすい
- current threshold としては still heavy である

## current judgment

current L2 で最も自然なのは、
**案 2. `boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref` を持つ**
である。

理由は次の通り。

1. boundary として 1 段切る以上、docs-only parser-front boundary だという current judgment が minimum に見える方が自然である
2. ただし query token / parser input / verifier handoff まで minimum に含めるのは still early である
3. command surface と shared output contract の relation だけを first minimum にすれば、Phase 3 reserve path を壊さず narrow に進められる

## current first choice shape

```text
public_checker_boundary_ready_sketch = {
  boundary_kind = docs_only_parser_front_checker_boundary,
  public_checker_command_surface_ref = public_checker_command_surface_ready_sketch,
  shared_output_contract_ref = shared_output_contract_ready_sketch
}
```

### この shape でまだ入れないもの

- final parser grammar / token surface
- dedicated query token / `checker_subject` public naming
- generic shared public checker entry
- detached loop wrapper path line
- emitted verifier handoff surface

これらは still later である。

## practical reading

current minimal public checker boundary が示すのは、

- current public-looking checker line は command side と output side の接点まで docs-only に切れている
- その boundary は actual parser grammar ではなく docs-only parser-front relation に留まる
- verifier handoff はこの次段で narrow に比較すればよい

の 3 点だけである。

## next promoted line

next promoted line は、
**minimal-public-checker-boundary-ready verifier-handoff-surface comparison**
に置く。

## open questions

- `boundary_kind` を later でより具体的な public naming に寄せるべきか
- query token / subject ref family を later public checker boundary にどう接続するか
- verifier handoff surface の minimum を何 field までに留めるべきか
