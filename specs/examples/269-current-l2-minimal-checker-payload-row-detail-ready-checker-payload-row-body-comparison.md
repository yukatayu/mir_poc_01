# 269 — current L2 minimal-checker-payload-row-detail-ready checker-payload-row-body comparison

## 目的

`specs/examples/268-current-l2-checker-payload-row-detail-ready-minimal-checker-payload-row-detail-threshold.md`
で checker payload row detail を
`payload_row_family_ref + row_source_ref + row_reason_kind`
まで切る判断を fixed した次段として、

- checker payload row detail の次に checker payload row body を docs-first line として切るべきか
- row body を切るならどこまでが current first choice か
- actual `StaticReasonCodeRow` variant payload をどこまで still later に残すべきか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
minimal-checker-payload-row-detail-ready checker-payload-row-body comparison** であり、

- supported kind summary
- public checker payload schema
- public checker API

はまだ固定しない。

## scope

- current L2 checker-side docs-first bridge だけを扱う。
- `actual_checker_payload_row` の次段としてだけ row body を比較する。
- actual checker implementation / public checker API には進まない。

## current 前提

current repo では次が成立している。

1. checker-cluster matrix line は `coverage_state` で package close にしてよい。
2. actual checker payload family は `payload_family_kind + source_refs` minimal bundle まで切れている。
3. checker payload row family は `payload_family_ref + row_family_kind` minimal bundle まで切れている。
4. checker payload row detail は `payload_row_family_ref + row_source_ref + row_reason_kind` minimal bundle まで切れている。
5. actual machine-check source では、fixture-side `expected_static.checked_reason_codes` と detached static gate `reason_codes` が already `StaticReasonCodeRow` variant payload を source-backed に持っている。

したがって current 問いは、
**checker payload row detail の次段として row body を 1 段切るべきか、切るならどの field までが current first choice か**
である。

## 比較観点

1. row detail と row body の役割分担が明瞭か
2. actual `StaticReasonCodeRow` variant payload slot を narrow に見せられるか
3. `row_reason_kind` と actual payload serializer を duplicate に読ませないか
4. next 段で supported kind summary comparison を narrow に開けるか

## 比較対象

### 案 1. row detail で止め、row body は still prose に残す

#### 利点

- line は最も軽い
- actual serializer 誤読を増やしにくい

#### 欠点

- `StaticReasonCodeRow` variant payload slot との接点が still prose 依存である
- next 段の supported kind summary comparison へ進む narrow bridge が弱い

### 案 2. `row_body` に variant-local slot bundle を足す

#### shape

```text
actual_checker_payload_row = {
  payload_row_family_ref = actual_checker_payload_row_family,
  row_source_ref = fixture_checked_reason_codes,
  row_reason_kind = missing_lineage_assertion,
  row_body = {
    predecessor = primary,
    successor = mirror
  }
}
```

#### 利点

- row detail が `source + kind`、row body が variant-local slot bundle、という役割分担が明瞭である
- actual `StaticReasonCodeRow` payload slot を narrow に見せられる
- `row_reason_kind` を detail 側に残したまま body を足せる
- supported kind summary と public checker payload schema を still later に残せる

#### 欠点

- variant-local slot shape が family ごとに分かれる
- `row_body` naming discipline が要る

### 案 3. full `StaticReasonCodeRow` 風 payload を current row にそのまま重ねる

#### shape

```text
actual_checker_payload_row = {
  payload_row_family_ref = actual_checker_payload_row_family,
  row_source_ref = fixture_checked_reason_codes,
  row_reason_kind = missing_lineage_assertion,
  row_body = {
    kind = missing_lineage_assertion,
    predecessor = primary,
    successor = mirror
  }
}
```

#### 利点

- existing serializer shape との接続は最も直接である

#### 欠点

- `row_reason_kind` を duplicate に読ませやすい
- row detail と row body の境界が曖昧になる
- public checker payload schema / serializer を premature に docs-first line へ焼き込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `row_body` に variant-local slot bundle を足す**
である。

理由は次の通り。

1. row detail は source array と reason kind の接点に留め、row body は actual payload slot に限定できる
2. current repo の source-backed `StaticReasonCodeRow` variant payload を narrow に見せられる
3. serializer-like full row へ寄せずに、supported kind summary と public checker payload schema を still later に残せる

## current reading

current docs-only では、

```text
actual_checker_payload_row = {
  payload_row_family_ref = actual_checker_payload_row_family,
  row_source_ref = fixture_checked_reason_codes,
  row_reason_kind = missing_lineage_assertion,
  row_body = {
    predecessor = primary,
    successor = mirror
  }
}
```

まででよい。

### practical example

```text
actual_checker_payload_row = {
  payload_row_family_ref = actual_checker_payload_row_family,
  row_source_ref = detached_static_gate_reason_codes,
  row_reason_kind = capability_strengthens,
  row_body = {
    from_capability = read,
    to_capability = write
  }
}
```

この current reading が示すのは、

- row がどの source array から来たか
- row がどの reason kind か
- row body の variant-local slot が何か

の 3 点だけであり、
supported kind summary や public checker payload schema はまだ current task で扱わない。

## next promoted line

next promoted line は、
**checker-payload-row-body-ready minimal-checker-payload-row-body threshold**
に置く。

## what is decided here

### decided

- checker payload row detail の次段として checker payload row body を docs-first line に切ってよい
- current first choice は `row_body` に variant-local slot bundle を足す形である
- supported kind summary と public checker payload schema は still later に残す

### not decided

- `row_body` の final naming
- actual row body serializer と docs-first shape をどこまで寄せるか
- supported kind summary をどの layer へ戻すか

## open questions

- edge-pair / capability / missing-option family の variant-local slot shape を current phase でどこまで stable token として読めるか
- `row_body` field 名を later checker payload side でも維持できるか
- supported kind summary の first reopen cut を row body の後にどこへ置くべきか
