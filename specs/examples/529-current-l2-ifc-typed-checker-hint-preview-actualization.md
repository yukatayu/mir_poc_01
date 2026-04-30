# 529 — current L2 IFC typed checker-hint preview actualization

## 目的

`specs/examples/259`、`260`、`261` では、

- `typed_reason_family_hint` を optional attachment として許すこと
- payload を `family_refs[]` minimal bundle に留めること
- multi-family / partial row の over-read を避けるため `coverage_state` を 1 段だけ足すこと

までは docs-first に整理済みである。

この文書の目的は、その compare-floor をいきなり final public checker payload に上げることではなく、
**active clean-near-end typing trio `01 / 02 / 03` に対して sample-local helper preview として actualize しつつ、archived `p10 / p11 / p12` compare anchor を historical memory に留める current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- final public checker artifact
- final public verifier contract
- final typed source principal
- final IFC syntax
- actual checker payload family

は still later に残す。

## 1. current question

current clean-near-end typing trio

- `01_authorized_declassification`
- `02_unauthorized_declassification_rejected`
- `03_label_flow_rejected`

を active compare floor に置いた後、
checker-side の `typed_reason_family_hint` line を
docs-only comparison のまま残すのではなく、
**sample-local helper preview に限って operational CLI へ mirror してよいか**。

## 2. current first line

current recommendation は次である。

1. helper-local preview に留める
2. global checker-cluster matrix や final public payload には上げない
3. `family_refs[] + coverage_state` shape を使う
4. `01 / 02` は authority-release cluster の partial coverage として扱う
5. `03` は label-flow cluster の first actual negative として full coverage に置く
6. archived `p10 / p11 / p12` は compare anchor に留め、current active entrypoint には戻さない
7. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized preview reading

| sample | cluster_kind | case_label | family_refs | coverage_state | current reading |
|---|---|---|---|---|---|
| `01_authorized_declassification` | `ifc_authority_release_cluster` | `authority_sensitive_success` | `ifc_label_model_family`, `explicit_authority_declassification_family` | `partial_cluster` | authority がある release success side |
| `02_unauthorized_declassification_rejected` | `ifc_authority_release_cluster` | `authority_miss_negative` | `ifc_label_model_family`, `explicit_authority_declassification_family` | `partial_cluster` | authority が無い release rejection side |
| `03_label_flow_rejected` | `ifc_label_flow_cluster` | `classified_to_public_negative` | `ifc_label_model_family`, `classified_public_label_flow_family` | `full_cluster` | classified payload を public に流す negative |

この cut では、
`01` と `02` の pair で authority-release cluster を読み、
各 sample 単体では `partial_cluster` とする。

一方 `03` は current repo の source-side label-flow negative first line を単独で担うため、
`ifc_label_flow_cluster` に対して `full_cluster` としてよい。

## 4. preview shape

current helper-local summary では、次の shape まで actualize してよい。

```text
typed_checker_hint_preview = {
  status = reached | guarded_not_reached,
  preview_kind = sample_local_helper_preview,
  cluster_kind = ...,
  case_label = ...,
  typed_reason_family_hint = {
    family_refs = [...],
    coverage_state = full_cluster | partial_cluster
  },
  evidence_refs = [...],
  compare_floor_refs = [...],
  guard_refs = [...],
  kept_later_refs = [...],
  guard_reason = ...
}
```

重要なのは次の 3 点である。

1. これは `verification_preview` や `artifact_preview` と同様の helper-local summary である
2. current repo が actualize した IFC trio だけを mirror する narrow cut である
3. `supported_kind_refs[]` や final payload schema には進まない

## 5. why this is enough

- current clean-near-end typing family `01 / 02 / 03` により、authority-release pair と label-flow negative は active compare floor に actualize 済みである
- `specs/examples/523` と `524` により、archived `p10 / p11 / p12` compare anchor memory も retained されている
- `specs/examples/259`、`260`、`261` により、`typed_reason_family_hint` / `family_refs[]` / `coverage_state` の docs-first reading も already available である

したがって current repo は、
global checker payload family を still later に残したまま、
active clean-near-end typing trio の current checker-adjacent reading を
helper-local operational CLI へ narrow に mirror してよい。

## 6. evidence

- current clean-near-end typing trio
  - `samples/clean-near-end/typing/01_authorized_declassification.mir`
  - `samples/clean-near-end/typing/02_unauthorized_declassification_rejected.mir`
  - `samples/clean-near-end/typing/03_label_flow_rejected.mir`
- archived compare anchors
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt`
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt`
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt`
  - `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/p10-typed-authorized-fingerprint-declassification/`
  - `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/p11-typed-unauthorized-fingerprint-release/`
  - `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/p12-typed-classified-fingerprint-publication-block/`
- operational CLI actualization
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- IFC foundation / explanation side
  - `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
  - `specs/examples/522`
  - `specs/examples/523`
  - `specs/examples/524`

## 7. stop line

この package の stop line は次である。

- `typed_checker_hint_preview` は helper-local / sample-local に留める
- `family_refs[] + coverage_state` で止める
- current clean-near-end IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- global checker-cluster matrix finalization
- actual checker payload family adoption
- `supported_kind_refs[]` adoption
- final public checker artifact
- final public verifier contract
- stronger typed source principal

## 8. retained later

- checker-hint / diagnostics widening beyond IFC trio
- `supported_kind_refs[]` を current checker-side line に足すべきかどうかの mixed gate
- actual checker payload family
- final typed source principal
- final IFC syntax
- final public verifier contract
