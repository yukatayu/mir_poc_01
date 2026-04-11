# 276 — current L2 public-checker-api-ready minimal-public-checker-api threshold

## 目的

`specs/examples/275-current-l2-minimal-public-checker-payload-schema-ready-public-checker-api-comparison.md`
で public checker API を
docs-only `public_checker_api_ready_sketch`
read relation として 1 段切る判断を fixed した次段として、

- public checker API の minimal field core をどこまでに留めるか
- `checker_subject` と public checker payload schema の relation をどこまで explicit に持たせるか
- actual command surface / shared output contract / parser boundary をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
public-checker-api-ready minimal-public-checker-api threshold** であり、

- actual command surface
- shared output contract
- final parser / checker boundary
- emitted public checker surface

はまだ固定しない。

## 比較観点

1. public checker payload schema と distinguish できる最小 API になっているか
2. `checker_subject` と schema の relation を明示しつつ still minimal に保てるか
3. actual command surface や shared output contract を premature に含めないか
4. next 段の public checker entry criteria comparison を narrow に保てるか

## 比較対象

### 案 1. `public_checker_payload_schema_ref` だけを持つ light API

#### shape

```text
public_checker_api_ready_sketch = {
  public_checker_payload_schema_ref = public_checker_payload_schema_ready_sketch
}
```

#### 利点

- 最も軽い
- schema wrapper 自体は public checker reading に渡せる

#### 欠点

- public checker payload schema と public checker API の区別が still 弱い
- どの `checker_subject` に対する API かが見えない

### 案 2. `checker_subject + public_checker_payload_schema_ref` を持つ minimal API

#### shape

```text
public_checker_api_ready_sketch = {
  checker_subject,
  public_checker_payload_schema_ref = public_checker_payload_schema_ready_sketch
}
```

#### 利点

- current public checker API line が、どの `checker_subject` に対してどの schema を返すかを最小限で示せる
- public checker payload schema と API line を分けて読める
- actual command surface / shared output contract / parser boundary は still later に残せる

#### 欠点

- `checker_subject` token を 1 field 持つぶん、light API よりは 1 段重い
- `checker_subject` naming discipline が要る

### 案 3. minimal API に `command_surface_kind` / shared output contract / verifier handoff 近傍 field まで同時に入れる

#### 利点

- public checker-facing surface に一気に近づける

#### 欠点

- current phase では still heavy である
- minimal API と actual command surface を同じ reopen で混ぜやすい
- parser / checker boundary と verifier handoff を premature に既成事実化しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `checker_subject + public_checker_payload_schema_ref` を持つ minimal API**
である。

理由は次の通り。

1. public checker API を切る以上、schema wrapper だけではなく subject 側の relation も見える方が自然である
2. ただし actual command surface / shared output contract / parser boundary まで持ち込むのは still early である
3. `checker_subject + public_checker_payload_schema_ref` なら minimal read relation として十分であり、次段の entry criteria comparison を narrow に保てる

## current first choice shape

```text
public_checker_api_ready_sketch = {
  checker_subject,
  public_checker_payload_schema_ref = public_checker_payload_schema_ready_sketch
}
```

### この shape でまだ入れないもの

- actual command surface / subcommand naming
- shared output contract
- parser / checker public boundary
- verifier handoff surface

これらは still later である。

## practical reading

current minimal API が示すのは、

- `checker_subject` がどの first checker cut subject familyを public reading に載せるか
- `public_checker_payload_schema_ref` がどの docs-only payload schema を返すか

の 2 点だけである。

actual command 名、generic shared entry、detached loop wrapper、library call surface はまだ minimal API に入れない。

### practical example

```text
public_checker_api_ready_sketch = {
  checker_subject = fallback_chain_edge,
  public_checker_payload_schema_ref = public_checker_payload_schema_ready_sketch
}
```

```text
public_checker_api_ready_sketch = {
  checker_subject = try_region_shape,
  public_checker_payload_schema_ref = public_checker_payload_schema_ready_sketch
}
```

## next promoted line

next promoted line は、
**minimal-public-checker-api-ready public-checker-entry-criteria comparison**
に置く。

## open questions

- `checker_subject` を later で dedicated query token / `*_ref` family に寄せるべきか
- actual command surface / shared output contract / verifier handoff をどの entry criteria で reopen するべきか
- family facade pattern と future public checker API comparison をどこで接続するべきか
