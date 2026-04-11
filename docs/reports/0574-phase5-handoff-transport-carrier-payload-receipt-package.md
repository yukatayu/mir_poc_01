# 0574 — Phase 5 handoff transport carrier / payload / receipt package

## Objective

Phase 5 theorem-line later reopen として、

- `minimal-handoff-transport-family-ready handoff-transport-carrier-detail comparison`
- `minimal-handoff-transport-carrier-detail-ready handoff-transport-payload comparison`
- `minimal-handoff-transport-payload-ready handoff-transport-receipt comparison`

を current retained bridge package として閉じ、
snapshot を `specs/examples/126...252` / next promoted line `minimal-handoff-transport-receipt-ready handoff-transport-channel-body comparison`
へ更新する。

## Scope and assumptions

- current L2 / Phase 5 theorem-line の docs-first comparison に限定する。
- actual public checker API、actual runtime handoff transport line、low-level memory-order family finalization には進まない。
- current retained bridge の ratchet を保ち、channel body / low-level memory-order family は still later に残す。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/243-current-l2-theorem-line-minimal-handoff-payload-ref-ready-handoff-carrier-detail-comparison.md`
- `specs/examples/244-current-l2-theorem-line-handoff-carrier-detail-ready-minimal-handoff-carrier-detail-threshold.md`
- `specs/examples/245-current-l2-theorem-line-minimal-handoff-carrier-detail-ready-handoff-transport-family-comparison.md`
- `specs/examples/246-current-l2-theorem-line-handoff-transport-family-ready-minimal-handoff-transport-family-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Actions taken

1. `specs/examples/247` と `248` を close し、handoff transport family の次段を
   `handoff_transport_family_ref + handoff_transport_carrier_detail`
   で切る current first choice を固定した。
2. `specs/examples/249` と `250` を追加し、handoff transport carrier detail の次段を
   `handoff_transport_carrier_detail_ref + handoff_transport_payload`
   で切る current first choice を固定した。
3. `specs/examples/251` と `252` を追加し、handoff transport payload の次段を
   `handoff_transport_payload_ref + handoff_transport_receipt_row`
   で切る current first choice を固定した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、Phase 5 research abstract を `126...252` snapshot に更新した。

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 574 numbered report(s).`
- `git diff --check`
  - 無出力
- review:
  - `docs/reports/0575-review-phase5-handoff-transport-carrier-payload-receipt-package.md`
  - snapshot drift 4 件を補正済み

## What changed in understanding

- `retained_payload_body_materialization_theorem_export_handoff_transport_family` の次段は、
  payload や receipt を飛ばさず、まず `retained_payload_body_materialization_theorem_export_handoff_transport_carrier_detail`
  を current first choice に置くのが自然である。
- その次段は、
  `retained_payload_body_materialization_theorem_export_handoff_transport_payload`
  を current first choice に置くのが自然である。
- さらにその次段は、
  `retained_payload_body_materialization_theorem_export_handoff_transport_receipt_row`
  を current first choice に置くのが自然であり、channel body は still later に残す方が symbolic-to-detail ratchet を保ちやすい。
- したがって current promoted line は、`minimal-handoff-transport-receipt-ready handoff-transport-channel-body comparison` に移る。

## Open questions

- `handoff_transport_channel_body` をどの row shape で current next step に切るのが最小か
- `handoff_transport_receipt_row` を retained bridge のまま維持するか、channel body 側へ actualize するか
- low-level memory-order family をどの threshold まで still later に残すか

## Suggested next prompt

`minimal-handoff-transport-receipt-ready handoff-transport-channel-body comparison` を Phase 5 theorem-line later reopen の current promoted line として進め、channel body を retained bridge にどこまで足してよいかを docs-first で比較してください。
