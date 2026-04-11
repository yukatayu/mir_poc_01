# Report 0578 — phase5 handoff low level memory order threshold

- Date: 2026-04-11 14:00 JST
- Author / agent: Codex
- Scope: `specs/examples/255`、`256` と snapshot / mirror / report closeout
- Decision levels touched: L2 / L3

## 1. Objective

Phase 5 の current promoted line として、

- `minimal-handoff-transport-channel-body-ready low-level-memory-order-family threshold`
- `small-decidable-core-ready checker-cluster-matrix comparison`

を docs-first package として閉じる。

今回の狙いは、

1. theorem-line retained bridge を `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body` で止める threshold を source-backed に固定すること
2. low-level memory-order family を still later に残したうえで、Phase 5 の next promoted line を small decidable core / checker side へ戻すこと

である。

## 2. Scope and assumptions

- current L2 / Phase 5 docs-only comparison に限定する。
- final memory model、final type system、actual public checker API、actual theorem prover / protocol verifier integration には進まない。
- current Phase 4 / Phase 3 / Phase 2 checkpoint の semantics は変えない。

## 3. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/218-current-l2-theorem-line-checker-verdict-transport-channel-body-ready-low-level-memory-order-family-threshold.md`
- `specs/examples/254-current-l2-theorem-line-handoff-transport-channel-body-ready-minimal-handoff-transport-channel-body-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## 4. Actions taken

1. `specs/examples/255` を追加し、handoff transport channel body の次段でも low-level memory-order family を retained bridge に入れず、theorem-line を channel body で止める current first choice を固定した。
2. `specs/examples/256` を追加し、Phase 5 の current promoted line を theorem-line の先ではなく `core_static_checker` 側の checker-cluster matrix comparison に戻した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、Phase 5 research abstract を current snapshot に更新した。
4. reviewer を 1 回だけ起動し、mirror drift / report hygiene を補正した。

## 5. Files changed

- `specs/examples/255-current-l2-theorem-line-minimal-handoff-transport-channel-body-ready-low-level-memory-order-family-threshold.md`
- `specs/examples/256-current-l2-small-decidable-core-ready-checker-cluster-matrix-comparison.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0578-phase5-handoff-low-level-memory-order-threshold.md`
- `docs/reports/0579-review-phase5-handoff-low-level-memory-order-threshold.md`

## 6. Commands run and exact outputs

```text
$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   93G  1.4G  99% /

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       794Mi        82Mi       1.2Mi       239Mi       165Mi
Swap:           19Gi       2.2Gi        17Gi

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 577 numbered report(s).

$ git diff --check
[no output]
```

## 7. Evidence / findings

- `specs/examples/255` により、handoff transport channel body の次段でも low-level memory-order family は theorem-line retained bridge に入れず、`retained_payload_body_materialization_theorem_export_handoff_transport_channel_body` を stop line にする current first choiceが source-backed になった。
- `specs/examples/256` により、Phase 5 の current promoted line は theorem-line の先ではなく、first checker cut の 6 cluster を docs-only matrix として整理する line に戻すのが自然だと整理できた。
- reviewer は semantic inconsistency を見つけず、mirror drift と report hygiene だけを指摘した。正本は `docs/reports/0579-review-phase5-handoff-low-level-memory-order-threshold.md` である。

## 8. Changes in understanding

- current handoff theorem-line は channel body で十分 narrow であり、ここから low-level memory-order family を marker としても足さない方が自然である。
- Phase 5 の次の self-driven line は theorem-line の先へ下ることではなく、small decidable core / `core_static_checker` inventory を user-facing に見える形へ整理することだと明確になった。

## 9. Open questions

- checker-cluster matrix の minimal row core に `fixture_evidence_refs` を入れるべきか
- `typed_reason_family_hint` を current line でまだ入れない方がよいか
- low-level memory-order family を将来 external verifier vocabulary としてだけ残すのか

## 10. Suggested next prompt

`checker-cluster-matrix-ready minimal-checker-cluster-row threshold` を Phase 5 の current promoted line として進め、first checker cut の 6 cluster を docs-only matrix row としてどこまで stable に見せるべきかを narrow に比較してください。
