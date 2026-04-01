# examples/14 — current L2 profile catalog externalization

この文書は、current L2 の named profile catalog を **hard-coded table のまま維持するか**、それとも **machine-readable catalog asset / preset manifest へ外出しするか**を比較する補助文書である。ここで扱うのは production manifest ではなく、PoC 実験ループの回しやすさに限った current L2 の最小比較である。

## この文書の位置づけ

- 比較対象は `smoke-runtime`、`smoke-static`、`runtime-e3`、`static-e4` を持つ current L2 named profile catalog である。
- selection / profile / batch / bundle の既存責務境界は変えない。
- `must_explain` は引き続き human-facing explanation obligation に残し、catalog 比較でも machine-check に上げない。
- current L2 では production catalog manifest を導入した事実にはしない。

## 比較する 2 案

### 1. hard-coded table を維持する

- `crates/mir-semantics/src/harness.rs` の `ProfileCatalog::aliases()` と `ProfileCatalog::resolve()` に alias 対応表を保持する。
- alias は既存 `SelectionRequest` / `SelectionProfile` に直接解決する。
- tests と docs はこの resolver の振る舞いを mirror する。

### 2. 小さな machine-readable preset table を導入する

- たとえば `crates/mir-semantics/tests/fixtures/current-l2/named-profile-catalog.json` のような小さな asset に alias と request 対応表を置く。
- `harness.rs` 側はその asset を読み、既存 `SelectionRequest` / `SelectionProfile` に解決する loader を持つ。
- ここでの asset は comparison 用の検証補助であり、production manifest には昇格させない。

## 比較観点

### PoC 実験ループの回しやすさ

- hard-coded table は alias 数が小さい間は最短である。
- Rust 実装と tests が同じ場所にまとまり、変更点が追いやすい。
- machine-readable asset は、preset の差し替えや比較実験をコード変更なしで回しやすくする余地がある。
- ただし current L2 の 4 alias 規模では、その利点より loader / validation の追加コストの方が大きい。

### 既存 helper の責務境界

- hard-coded table は、catalog を alias resolver に留めやすい。
- machine-readable asset を導入しても理屈上は境界を保てるが、asset loader、schema validation、missing asset handling が新たな責務として増える。
- その結果、catalog layer が selection / bundle / batch に近づきやすくなる。

### alias grammar / selector grammar の固定圧

- hard-coded table は、current alias 名だけを局所的に持てばよく、grammar を強く固定しない。
- machine-readable asset は field naming、selector encoding、将来 alias 拡張時の schema 互換を意識させる。
- これは current L2 ではまだ早い固定圧になりやすい。

### path canonicalization / manifest 導入へのにじみ

- hard-coded table は path canonicalization policy をほぼ要求しない。
- machine-readable asset は file placement、relative path、directory discovery の取り扱いを先に考えさせる。
- そのまま preset manifest や bundle manifest へ話題がにじみやすい。

### tests / review のしやすさ

- hard-coded table は Rust code review と targeted test で十分追える。
- machine-readable asset は asset review 自体はしやすいが、loader と asset の二重整合が必要になる。
- current L2 では alias 数が少ないため、review 面でも hard-coded table の不利はまだ小さい。

## current L2 の結論

- current L2 では、`smoke-runtime`、`smoke-static`、`runtime-e3`、`static-e4` の規模なら hard-coded table のままで十分である。
- machine-readable catalog asset は comparison 対象として意味はあるが、current L2 の最小 PoC にはまだ昇格させない。
- したがって current L2 の暫定運用選択は、**hard-coded named profile catalog を維持し、externalization は未決定のまま残す** である。

## machine-readable asset に出す価値が出る条件

少なくとも次の条件が重なるなら、externalization を再検討する価値が出る。

- alias 数が「small named catalog」と言いにくい程度まで増える。
- preset を Rust 実装の変更なしで頻繁に差し替えたい。
- Rust code review と別に、preset table 自体を machine-readable asset として比較したい。
- 複数の preset set を実験的に切り替える要求が増える。
- catalog の drift を code より asset validation で抑えたい。

## machine-check と human-facing explanation の境界

### exact compare してよいもの

- alias から得られる `resolved_request`
- selected bundle counts
- static verdict
- runtime outcome
- formal trace / audit expectation
- host plan coverage

### narrative に残すもの

- `must_explain`
- alias をなぜその組み合わせにしたかという運用上の説明
- machine-readable asset を採らなかった理由の prose
- long-form audit explanation

## current L2 でまだ決めないこと

- bundle manifest を導入するかどうか
- alias grammar / selector grammar を長期固定するかどうか
- path canonicalization policy
- catalog asset の final file format を JSON / YAML のどちらにするか
- detached trace / audit serialization
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate`

これらは **未決定** とする。current L2 で固定するのは、「現状の 4 alias 規模では hard-coded table が最小であり、machine-readable asset は比較対象に留める」という判断だけである。
