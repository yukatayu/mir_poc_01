# Report 0592 — Phase5 Checker Payload Supported Kind Summary Package

## 1. Objective

Phase 5 checker-side current promoted line として、

- checker payload row body の次段に checker payload supported kind summary を docs-first line として切るべきか
- checker payload supported kind summary の minimal shape をどこで止めるのが自然か

を `specs/examples/271...272` で narrow に整理し、mirror / snapshot を current package close まで追随させる。

## 2. Scope and assumptions

- current L2 / Phase 5 checker-side line だけを扱う。
- theorem-line retained bridge stop line は変えない。
- actual checker implementation、public checker API、final type system には進まない。
- checker payload row body の current minimal shape は `row_body` variant-local slot bundle を前提にする。
- `is_supported_checked_reason_code` と detached `reason_codes_scope` は current stable subset inventory の source evidence として参照する。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/262-current-l2-typed-family-coverage-state-ready-supported-kind-summary-threshold.md`
- `specs/examples/264-current-l2-actual-checker-payload-family-ready-minimal-checker-payload-family-threshold.md`
- `specs/examples/265-current-l2-minimal-checker-payload-family-ready-checker-payload-row-family-comparison.md`
- `specs/examples/266-current-l2-checker-payload-row-family-ready-minimal-checker-payload-row-family-threshold.md`
- `specs/examples/267-current-l2-minimal-checker-payload-row-family-ready-checker-payload-row-detail-comparison.md`
- `specs/examples/268-current-l2-checker-payload-row-detail-ready-minimal-checker-payload-row-detail-threshold.md`
- `specs/examples/269-current-l2-minimal-checker-payload-row-detail-ready-checker-payload-row-body-comparison.md`
- `specs/examples/270-current-l2-checker-payload-row-body-ready-minimal-checker-payload-row-body-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
- `crates/mir-semantics/tests/current_l2_static_gate_support.rs`

## 4. Actions taken

1. `specs/examples/271...272` を追加し、checker payload supported kind summary line を 2 段で整理した。
2. checker payload row body の次段として checker payload supported kind summary を docs-first line に切る current first choice を固定した。
3. checker payload supported kind summary は `payload_row_family_ref` keyed な独立 summary line とし、minimal shape を `payload_row_family_ref + supported_kind_scope + supported_kind_refs` に留めた。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot に更新した。

## 5. Files changed

- `Documentation.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0592-phase5-checker-payload-supported-kind-summary-package.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/examples/271-current-l2-minimal-checker-payload-row-body-ready-checker-payload-supported-kind-summary-comparison.md`
- `specs/examples/272-current-l2-checker-payload-supported-kind-summary-ready-minimal-checker-payload-supported-kind-summary-threshold.md`
- `tasks.md`

## 6. Commands run

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-11 16:36 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 592 numbered report(s).

$ git diff --check
[no output]
```

## 7. Evidence / outputs / test results

### specs/examples/271

- checker payload row body の次段として checker payload supported kind summary を docs-first line に切ってよい。
- current first choice は `payload_row_family_ref` keyed な独立 summary line である。
- row family / row detail / row body と summary の役割を分けたまま、public checker payload schema を still later に残せる。

### specs/examples/272

- checker payload supported kind summary の current minimal shape は `payload_row_family_ref + supported_kind_scope + supported_kind_refs` である。
- `supported_kind_scope = stable_clusters_only` を current line に残すことで、detached `reason_codes_scope` と support filter の stable subset 境界を explicit に読める。
- `source_support_map` と public checker payload schema wrapper は current line に入れない。

## 8. What changed in understanding

- supported kind summary は row family に畳み込むより、`payload_row_family_ref` keyed な独立 summary line にした方が payload family / row family / row detail / row body の役割分担を崩さない。
- current stable subset inventory は `supported_kind_scope + supported_kind_refs` までで十分であり、source_support_map を足すのは premature である。
- public checker payload schema は summary line の後段 comparison として残した方が current checker-side ratchet に合う。

## 9. Open questions

- `supported_kind_scope` と detached `reason_codes_scope` の naming を later でどこまで揃えるべきか。
- `supported_kind_refs` の token order を current phase でどこまで stable にしてよいか。
- public checker payload schema の first reopen cut をどの粒度に置くべきか。

## 10. Suggested next prompt

`minimal-checker-payload-supported-kind-summary-ready public-checker-payload-schema comparison` を narrow に比較し、current checker payload side の docs-first package の後に public checker payload schema をどこで開くのが最小かを整理してください。
