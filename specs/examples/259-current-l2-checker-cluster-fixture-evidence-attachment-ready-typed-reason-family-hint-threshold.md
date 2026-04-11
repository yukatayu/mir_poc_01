# 259 — current L2 checker-cluster-fixture-evidence-attachment-ready typed-reason-family-hint threshold

## 目的

`specs/examples/258-current-l2-minimal-checker-cluster-row-ready-checker-cluster-fixture-evidence-attachment-comparison.md`
で `fixture_evidence_refs` を row attachment として足す cut を固定した次段として、

- `typed_reason_family_hint` を current checker-cluster line に足してよいか
- 足すとしても row core ではなくどの層に留めるべきか
- stable typed static reason actualization を持つ cluster と、まだ持たない cluster をどう同じ matrix 上に並べるべきか

を比較する。

ここで固定するのは **typed_reason_family_hint の current docs-first threshold** であり、

- actual checker artifact
- final public checker API
- final type system judgment
- checker-cluster hint payload の final bundle shape

はまだ固定しない。

## scope

- current L2 first checker cut 候補 6 cluster の checker-cluster matrix を扱う。
- `typed_reason_family_hint` の導入可否と placement だけを比較する。
- `expected_static.checked_reason_codes` と detached static gate `reason_codes` の actual behavior は source evidence として参照する。
- actual helper / schema / checker 実装の actualization には進まない。

## current 前提

current repo では次が成立している。

1. minimal checker-cluster row core は
   `cluster_kind + checker_subject + decidable_class + external_handoff`
   に留める cut が固定済みである。
2. `fixture_evidence_refs` は row core ではなく attachment として足す cut が固定済みである。
3. `specs/examples/40...43` と `docs/reports/0168-first-typed-static-reason-family-selection.md`
   により、stable typed static reason actualization は
   - lineage edge pair family
   - declared target edge pair family
   - capability strengthening singleton
   - missing option family
   まで source-backed に揃っている。
4. request / predicate fragment / `try` structural floor は current docs では checker-cluster inventory に入っているが、
   stable typed static reason family の first-class actualization はまだ持たない。

したがって current 問いは、
**checker-cluster matrix の row attachment に typed static reason 側の接点を 1 段だけ見せるべきか、それとも still later に残すべきか**
である。

## 比較観点

1. checker-cluster row core と typed static reason actualization を混ぜすぎないか
2. stable typed family を持つ cluster と、まだ持たない cluster を同じ matrix 上で誤読なく扱えるか
3. `checked_reason_codes` を current public checker artifact と誤読させないか
4. next 段で hint payload shape だけを narrow に比較できるか

## 比較対象

### 案 1. `typed_reason_family_hint` はまだ足さない

#### 利点

- checker-cluster matrix は minimal row core + `fixture_evidence_refs` attachment のまま保てる
- `checked_reason_codes` line を Phase 5 checker-side line に近づけすぎずに済む

#### 欠点

- current repo に already ある stable typed family actualization との接点が prose 依存のまま残る
- 「どの cluster が typed family 側へ比較的近いか」が matrix 上では見えにくい

### 案 2. `typed_reason_family_hint` を **optional row attachment** として足す

#### 読み

row core は変えず、`fixture_evidence_refs` と並ぶ optional attachment として
`typed_reason_family_hint`
を許す。

ただし current task では、

- hint payload の final field set
- single family か family bundle か
- actual supported kind row を inline するか

はまだ決めない。

#### 利点

- current repo に already ある stable typed family actualization との接点を 1 段だけ見せられる
- request / predicate / `try` cluster のように typed family をまだ持たない row では、hint absent を自然に保てる
- row core、evidence attachment、typed-family proximity を別 layer に分けられる
- next 段で hint payload shape だけを narrow に比較しやすい

#### 欠点

- row attachment が 1 段増える
- family bundle shape を後段で明文化する discipline が要る

### 案 3. `typed_reason_family_hint` を row attachment の actual kind set まで含む形で immediate に入れる

#### 利点

- `checked_reason_codes` との接点を最も具体的に見せられる
- checker-side typed carrier へ近い見え方を先に出せる

#### 欠点

- stable family を持たない cluster にも同じ粒度を要求しやすい
- docs-only checker-cluster matrix を actual checker artifact と誤読させやすい
- hint payload shape の比較を飛ばして kind-level payload を既成事実化しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `typed_reason_family_hint` を optional row attachment として足す**
ことである。

理由は次の通り。

1. stable typed static reason actualization との接点は current repo に already あるので、completely absent のままにするより 1 段だけ見せた方が user-facing に自然である
2. ただし、その接点は row core ではなく optional attachment に留める方が、typed-family readiness 差を壊さない
3. actual kind row や public checker payload を immediate に持ち込むのは still early である

## current reading

current docs-only 読みでは、
`typed_reason_family_hint`
は次の性質だけを持つ optional attachment でよい。

- **stable typed family actualization が current repo にある row だけ present**
- **present でも symbolic family proximity を示すだけ**
- **actual kind row / transport / public payload は still later**

### practical example A — same-lineage floor

```text
checker_cluster_row = {
  cluster_kind     = same_lineage_static_evidence_floor,
  checker_subject  = fallback_chain_edge,
  decidable_class  = local_structural,
  external_handoff = theorem_prover_boundary
}

checker_cluster_row_attachment = {
  fixture_evidence_refs = [...],
  typed_reason_family_hint = present
}
```

ここで hint が示すのは、
current repo に lineage edge pair family の `checked_reason_codes` actualization がある、
という proximity だけである。

### practical example B — request-local clause attachment

```text
checker_cluster_row = {
  cluster_kind     = request_option_clause_attachment,
  checker_subject  = clause_attachment_shape,
  decidable_class  = local_structural,
  external_handoff = runtime_policy_boundary
}

checker_cluster_row_attachment = {
  fixture_evidence_refs = [...],
  typed_reason_family_hint = absent
}
```

current repo では request / predicate / `try` structural floor に対する stable typed static reason family actualization はまだ無いので、
hint absent のままにする方が自然である。

## next promoted line

next promoted line は、
**typed-reason-family-hint-ready checker-cluster-hint-bundle-shape comparison**
に置く。

ここで比べるのは、

1. single symbolic family ref
2. multiple family refs
3. family refs + carrier / supported-kind summary を持つ docs-only bundle

のどこまでを current checker-cluster line に許してよいかである。

## what is decided here

### decided

- `typed_reason_family_hint` は current checker-cluster line に入れてよい
- ただし row core ではなく optional attachment に留める
- actual kind row / payload までは immediate に入れない

### not decided

- `typed_reason_family_hint` payload の final field set
- single family ref か family bundle か
- `checked_reason_codes` との actual schema 共有
- final public checker artifact への ratchet

## open questions

- malformed / underdeclared cluster のように複数 stable family と接続しうる row で、hint を 1 field でどう持つべきか
- capability singleton を family hint と呼んでよいか、それとも別 hint class に分けるべきか
- request / predicate / `try` cluster に future typed family が出たとき、same attachment shape をそのまま使えるか
