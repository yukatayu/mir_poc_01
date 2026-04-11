# 275 — current L2 minimal-public-checker-payload-schema-ready public-checker-api comparison

## 目的

`specs/examples/274-current-l2-public-checker-payload-schema-ready-minimal-public-checker-payload-schema-threshold.md`
で public checker payload schema を
`actual_checker_payload_family_ref + checker_payload_row_family_ref + checker_payload_row_detail_ref + checker_payload_row_body_ref + checker_payload_supported_kind_summary_ref`
minimal wrapper に留める判断を fixed した次段として、

- public checker payload schema の後に public checker API を docs-first line として切るべきか
- public checker API を切るならどの layer が current first choice か
- actual command surface / shared output contract / parser boundary をどこまで still later に残すべきか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
minimal-public-checker-payload-schema-ready public-checker-api comparison** であり、

- actual command surface
- shared output contract
- final parser / checker boundary
- final checker implementation contract

はまだ固定しない。

## scope

- current L2 checker-side docs-first bridge だけを扱う。
- actual checker payload family / row family / row detail / row body / supported kind summary / public checker payload schema までを前提にする。
- actual CLI / library API / emitted transport surface には進まない。

## current 前提

current repo では次が成立している。

1. checker-cluster matrix row core は `cluster_kind + checker_subject + decidable_class + external_handoff` まで切れている。
2. checker-side payload line は actual checker payload family、row family、row detail、row body、supported kind summary、public checker payload schema まで docs-first に切れている。
3. actual source では、`crates/mir-semantics/src/harness.rs` が optional `checked_reason_codes` compare を bundle-local contract として持ち、`scripts/current_l2_family_checker_support.py` は family-local compare contract だけを shared support helper に寄せ、`scripts/current_l2_detached_loop.py` は `smoke-same-lineage-checker` / `smoke-missing-option-checker` / `smoke-capability-checker` の family facade を current command surface に保っている。
4. theorem-side でも `specs/examples/132`、`198`、`199` により、docs-only bridge と actual public-looking contract を同じ reopen で混ぜない current first choice が already 固まっている。

したがって current 問いは、
**public checker payload schema の次段として public checker API を 1 段切るべきか、切るならどの layer が current first choice か**
である。

## 比較観点

1. `checker_subject` と public checker payload schema の relation を 1 箇所に束ねられるか
2. helper-local family facade pattern を generic public entry と premature に混ぜないか
3. actual command surface / shared output contract / parser boundary を still later に残せるか
4. next 段の minimal public checker API threshold を narrow に保てるか

## 比較対象

### 案 1. public checker API は still prose に残す

#### 利点

- line は最も軽い
- actual command surface や parser boundary の premature actualization を避けやすい

#### 欠点

- `checker_subject` と public checker payload schema の接点が still prose 依存に残る
- later public checker entry criteria comparison の前提が docs 上で見えにくい

### 案 2. docs-only `public_checker_api_ready_sketch` read relation を 1 段切る

#### shape

```text
public_checker_api_ready_sketch = {
  checker_subject,
  public_checker_payload_schema_ref = public_checker_payload_schema_ready_sketch
}
```

#### 利点

- current checker-side package を、どの `checker_subject` に対してどの payload schema を返す line かとして束ねられる
- public checker payload schema と command surface actualization を分けて読める
- current family facade / helper-local smoke surfaceを public checker API 本体と誤読しにくい
- next 段の minimal public checker API threshold を narrow に比較しやすい

#### 欠点

- docs-only wrapper が 1 段増える
- `checker_subject` を current API line でどこまで stable token として読むかの説明が要る

### 案 3. generic command surface / shared output contract / parser boundary へ直接進む

#### 読み

public checker API を、
generic checker entry、shared output contract、parser / checker public boundary に近い
surface として同時に切る。

#### 利点

- public checker-facing surface を早く見せられる

#### 欠点

- current phase では still early である
- helper-local family facade と public checker API を同じ reopen で混ぜやすい
- parser boundary / verifier handoff / shared output contract を premature に既成事実化しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. docs-only `public_checker_api_ready_sketch` read relation を 1 段切る**
である。

理由は次の通り。

1. public checker payload schema の次段では、まず `checker_subject` と schema の relation を 1 箇所に束ねる方が自然である
2. current actual command surface は still family facade / helper-local smoke に留まっており、generic public entry をまだ選ぶ段階ではない
3. theorem-side の public-looking contract comparison と同様に、docs-only bridge と actual public API actualization を分けて ratchet する方がきれいである

## current reading

current docs-only では、
public checker payload schema の次段として
**`public_checker_api_ready_sketch`**
を 1 段切ってよい。

この line が示すのは、

- current checker-side package がどの `checker_subject` family に対する public checker reading か
- その subject に対して返す payload schema が `public_checker_payload_schema_ready_sketch` であること
- ただし API 自体は still docs-only であり、actual command surface ではないこと

の 3 点だけである。

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

この current reading で still later に残すものは、

- actual command surface / subcommand naming
- shared output contract
- parser / checker public boundary
- verifier handoff actualization

である。

## next promoted line

next promoted line は、
**public-checker-api-ready minimal-public-checker-api threshold**
に置く。

## what is decided here

### decided

- public checker payload schema の次段として public checker API を docs-first line に切ってよい
- current first choice は `public_checker_api_ready_sketch` docs-only read relation である
- actual command surface / shared output contract / parser boundary は still later に残す

### not decided

- minimal public checker API の final field set
- `checker_subject` を current phase でどこまで stable token として読むか
- actual command surface / shared output contract / parser boundary / verifier handoff

## open questions

- minimal public checker API を `checker_subject + public_checker_payload_schema_ref` に留めるべきか
- `checker_subject` を later で dedicated query token family に寄せるべきか
- public checker entry criteria comparison を API minimal threshold の次段にどこまで narrow に切るべきか
