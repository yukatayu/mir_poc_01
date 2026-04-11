# Report 0588 — Phase5 Checker Payload Row Detail Package

## 1. Objective

Phase 5 checker-side current promoted line として、

- checker payload row family の次段に checker payload row detail を docs-first line として切るべきか
- checker payload row detail の minimal shape をどこで止めるのが自然か

を `specs/examples/267...268` で narrow に整理し、mirror / snapshot を current package close まで追随させる。

## 2. Scope and assumptions

- current L2 / Phase 5 checker-side line だけを扱う。
- theorem-line retained bridge stop line は変えない。
- actual checker implementation、public checker API、final type system には進まない。
- checker payload row family の current minimal shape は `payload_family_ref + row_family_kind` を前提にする。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/264-current-l2-actual-checker-payload-family-ready-minimal-checker-payload-family-threshold.md`
- `specs/examples/265-current-l2-minimal-checker-payload-family-ready-checker-payload-row-family-comparison.md`
- `specs/examples/266-current-l2-checker-payload-row-family-ready-minimal-checker-payload-row-family-threshold.md`
- `specs/examples/39-current-l2-static-reason-code-readiness-scan.md`
- `specs/examples/49-current-l2-shared-family-checker-support-helper.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`

## 4. Actions taken

1. `specs/examples/267...268` を追加し、checker payload row detail line を 2 段で整理した。
2. checker payload row family の次段として checker payload row detail を docs-first line に切る current first choice を固定した。
3. checker payload row detail の minimal shape は `payload_row_family_ref + row_source_ref + row_reason_kind` に留め、actual row body と supported kind detail を still later に残した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot に更新した。

## 5. Files changed

- `Documentation.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0588-phase5-checker-payload-row-detail-package.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/examples/267-current-l2-minimal-checker-payload-row-family-ready-checker-payload-row-detail-comparison.md`
- `specs/examples/268-current-l2-checker-payload-row-detail-ready-minimal-checker-payload-row-detail-threshold.md`
- `tasks.md`

## 6. Commands run

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-11 15:29 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 588 numbered report(s).

$ git diff --check
[no output]
```

## 7. Evidence / outputs / test results

### specs/examples/267

- checker payload row family の次段として checker payload row detail を docs-first line に切ってよい。
- current first choice は `payload_row_family_ref + row_source_ref + row_reason_kind` である。
- actual row body は still later に残す。

### specs/examples/268

- checker payload row detail の current minimal shape は `payload_row_family_ref + row_source_ref + row_reason_kind` である。
- row detail first cut には actual row body payload や supported kind summary をまだ入れない。

## 8. What changed in understanding

- checker payload row detail は、source array と reason-code variant の接点だけを持つ first cut に留めるのが自然である。
- actual `StaticReasonCodeRow` body は row detail と切り分けて次段に残した方が、Phase 5 の docs-first ratchet を保てる。
- supported kind summary は checker-cluster matrix、row family、row detailのどこにもまだ入れない方が current cut に合う。

## 9. Open questions

- checker payload row body の first cut をどこまでに留めるべきか。
- `row_source_ref` と `row_reason_kind` の naming を current phase でどこまで stable token として読めるか。
- actual row body を public checker payload schema と切り分けたまま narrow に出せるか。

## 10. Suggested next prompt

`minimal-checker-payload-row-detail-ready checker-payload-row-body comparison` を narrow に比較し、checker payload row detail の次段として actual row body をどこまで docs-first に切るべきかを整理してください。
