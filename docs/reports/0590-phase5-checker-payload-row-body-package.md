# Report 0590 — Phase5 Checker Payload Row Body Package

## 1. Objective

Phase 5 checker-side current promoted line として、

- checker payload row detail の次段に checker payload row body を docs-first line として切るべきか
- checker payload row body の minimal shape をどこで止めるのが自然か

を `specs/examples/269...270` で narrow に整理し、mirror / snapshot を current package close まで追随させる。

## 2. Scope and assumptions

- current L2 / Phase 5 checker-side line だけを扱う。
- theorem-line retained bridge stop line は変えない。
- actual checker implementation、public checker API、final type system には進まない。
- checker payload row detail の current minimal shape は `payload_row_family_ref + row_source_ref + row_reason_kind` を前提にする。

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
- `specs/examples/34-current-l2-static-reason-code-entry-criteria.md`
- `specs/examples/40-current-l2-first-typed-static-reason-family-selection.md`
- `specs/examples/41-current-l2-first-typed-static-reason-family-carrier-cut.md`
- `specs/examples/43-current-l2-complete-stable-static-reason-tranche.md`
- `specs/examples/267-current-l2-minimal-checker-payload-row-family-ready-checker-payload-row-detail-comparison.md`
- `specs/examples/268-current-l2-checker-payload-row-detail-ready-minimal-checker-payload-row-detail-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
- `crates/mir-ast/tests/fixtures/current-l2/e13-malformed-capability-strengthening.json`

## 4. Actions taken

1. `specs/examples/269...270` を追加し、checker payload row body line を 2 段で整理した。
2. checker payload row detail の次段として checker payload row body を docs-first line に切る current first choice を固定した。
3. checker payload row body の minimal shape は `row_body` variant-local slot bundle に留め、`row_reason_kind` / `row_source_ref` は detail 側に残し、supported kind summary と public checker payload schema を still later に残した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot に更新した。

## 5. Files changed

- `Documentation.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0590-phase5-checker-payload-row-body-package.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/examples/269-current-l2-minimal-checker-payload-row-detail-ready-checker-payload-row-body-comparison.md`
- `specs/examples/270-current-l2-checker-payload-row-body-ready-minimal-checker-payload-row-body-threshold.md`
- `tasks.md`

## 6. Commands run

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-11 16:25 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 590 numbered report(s).

$ git diff --check
[no output]
```

## 7. Evidence / outputs / test results

### specs/examples/269

- checker payload row detail の次段として checker payload row body を docs-first line に切ってよい。
- current first choice は `row_body` variant-local slot bundle である。
- full `StaticReasonCodeRow` serializer shape や supported kind summary は still later に残す。

### specs/examples/270

- checker payload row body の current minimal shape は `row_body` に variant-local slot bundleだけを持たせる cut である。
- `row_body.kind` echo、supported kind summary、public checker payload row schema は current line に入れない。

## 8. What changed in understanding

- row detail は `source + kind` の接点に、row body は variant-local slot bundle に分ける方が、actual `StaticReasonCodeRow` payload と docs-first ratchet の両方に整合する。
- supported kind summary は row family / row detail と同様に row body にも immediate には混ぜず、別 comparison line として reopen した方が current checker-side cut を保てる。
- actual serializer shape を current row body line にそのまま写すと `row_reason_kind` を duplicate に読ませやすく、public payload schema 誤読を増やす。

## 9. Open questions

- supported kind summary の first reopen cut をどこまで narrow に取るべきか。
- `row_body` naming を current phase でどこまで stable token として読めるか。
- missing-option family の `head` / `option` split を current row body line でどこまで明示すべきか。

## 10. Suggested next prompt

`minimal-checker-payload-row-body-ready checker-payload-supported-kind-summary comparison` を narrow に比較し、supported kind summary を checker payload side のどの layer に戻すのが最小かを整理してください。
