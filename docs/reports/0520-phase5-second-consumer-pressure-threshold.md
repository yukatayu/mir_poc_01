# 0520 — phase5 second-consumer-pressure threshold

## Objective

`specs/examples/202-current-l2-theorem-line-consumer-hint-ready-second-consumer-pressure-threshold.md`
を追加し、Phase 5 theorem-line の current promoted line として

- `consumer_hint` enrichment の次段で second consumer pressure をどこまで retained bridge に近づけるか
- actual `proof_assistant_adapter` contract と `theorem_export_checker` pressure をどこまで still 後段に残すか

を narrow に比較して、current first choice を固定する。

## Scope and assumptions

- current `proof_notebook` first bridge を起点にする。
- second practical consumer candidate は `proof_assistant_adapter` に限る。
- actual adapter schema と `theorem_export_checker` line はまだ扱わない。

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
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md`
- `specs/examples/137-current-l2-theorem-line-next-consumer-pressure-comparison.md`
- `specs/examples/200-current-l2-theorem-line-external-contract-payload-ready-proof-hint-threshold.md`
- `specs/examples/201-current-l2-theorem-line-proof-hint-ready-consumer-hint-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Actions taken

1. `specs/examples/202-current-l2-theorem-line-consumer-hint-ready-second-consumer-pressure-threshold.md` を追加し、consumer-hint-ready retained bridge の次段として
   - terminal cut
   - symbolic second consumer pressure marker
   - actual adapter contract / checker pressure 同時導入
   の 3 案を比較した。
2. current first choice を、`retained_payload_body_materialization_external_contract_second_consumer_pressure` だけを足す retained bridge として固定した。
3. mirror を `126...202` snapshot と `second-consumer-pressure-ready proof-assistant-adapter contract comparison` に更新した。

## Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/202-current-l2-theorem-line-consumer-hint-ready-second-consumer-pressure-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0520-phase5-second-consumer-pressure-threshold.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M %Z'
df -h .
free -h
rg -n "126\\.\\.\\.202|second-consumer-pressure-ready proof-assistant-adapter contract comparison|retained_payload_body_materialization_external_contract_second_consumer_pressure" \
  progress.md tasks.md plan/11-roadmap-near-term.md plan/17-research-phases-and-autonomy-gates.md \
  plan/12-open-problems-and-risks.md plan/13-heavy-future-workstreams.md \
  docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- resource preflight
  - `2026-04-10 18:08 JST`
- `df -h .`
  - `/dev/vda2` on `/`: `99G` total / `92G` used / `2.7G` avail / `98%`
- `free -h`
  - `Mem`: `960Mi` total / `773Mi` used / `72Mi` free / `186Mi` available
- `rg -n "126\\.\\.\\.202|second-consumer-pressure-ready proof-assistant-adapter contract comparison|retained_payload_body_materialization_external_contract_second_consumer_pressure" ...`
  - `progress.md`、`tasks.md`、`plan/11`、`plan/17`、`plan/12`、`plan/13`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` で `126...202` close、next promoted line、retained field rollup が一致していることを確認した
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 520 numbered report(s).`
- `git diff --check`
  - no output

## What changed in understanding

- `consumer_hint` enrichment の次段では、second consumer pressure 自体は symbolic marker として retained bridge に narrow に actualize してよい。
- ただし actual `proof_assistant_adapter` contract と `theorem_export_checker` pressure は同じ reopen に混ぜず、still 後段に残す方が line を保ちやすい。
- next promoted line は `second-consumer-pressure-ready proof-assistant-adapter contract comparison` と読むのが自然になった。

## Open questions

- actual `proof_assistant_adapter` contract をどの field / row bundle で切るか
- `proof_assistant_adapter` pressure を retained bridge のまま維持するか adapter-facing contract へ actualize するか
- `theorem_export_checker` pressure を later candidate のまま維持する concrete threshold を何と読むか

## Suggested next prompt

`specs/examples/202-current-l2-theorem-line-consumer-hint-ready-second-consumer-pressure-threshold.md` を前提に、second-consumer-pressure-ready proof-assistant-adapter contract comparison を 3 案で比較し、actual adapter-facing contract を theorem-line retained bridge にどこまで近づけるかを docs-first で整理してください。
