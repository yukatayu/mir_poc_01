# Report 0381 — phase5 proof obligation matrix and handoff artifact

- Date: 2026-04-09T10:21:58.131042Z
- Author / agent: Codex
- Scope: Phase 5 later package として、proof-obligation matrix と external handoff artifact の docs-only cut を整理し、Phase 5 checkpoint 読みを mirror に反映する
- Decision levels touched: L2 docs-first package refinement / repository memory update

## 1. Objective

`specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
で固定した 4-way split を前提に、

- proof obligation を docs 上でどう matrix 化するか
- external handoff artifact をどこまで narrow sketch に留めるか
- detached static gate artifact / `checked_reason_codes` / shared-space witness bundle を premature に public-looking contract にしない cut

を、current self-driven package として整理する。

## 2. Inputs consulted

- `AGENTS.md`
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
- `specs/examples/32-current-l2-static-gate-artifact-loop.md`
- `specs/examples/34-current-l2-static-reason-code-entry-criteria.md`
- `specs/examples/49-current-l2-shared-family-checker-support-helper.md`
- `specs/examples/121-current-l2-shared-space-authoritative-room-baseline.md`
- `specs/examples/122-current-l2-shared-space-catalog-working-subset-comparison.md`
- `specs/examples/123-current-l2-shared-space-auditable-authority-witness-minimal-shape.md`
- `specs/examples/124-current-l2-shared-space-authoritative-room-delegated-rng-provider-placement.md`
- `specs/examples/125-current-l2-shared-space-control-plane-carrier-threshold.md`
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `docs/research_abstract/README.md`

## 3. Actions taken

1. resource check と current top-level snapshot を確認し、disk / memory が tight なことを踏まえて docs-only package に留めた。
2. Phase 5 current evidence として、static gate artifact、`checked_reasons` / `checked_reason_codes`、shared-space witness core / provider receipt attachment の current cut を再確認した。
3. `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md` を追加し、matrix-only / mixed row sketch / direct promotion の 3 案を比較した。
4. current first choice を「matrix を docs 正本に置きつつ、source evidence 参照の mixed row bundle sketch に留める」として固定した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md` を現在地へ合わせて更新した。
6. condensed summary として `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を追加した。
7. local validation 後に reviewer を 1 回だけ回し、2 finding を受けて
   - shared-space example の receipt attachment wording
   - report / progress log / traceability evidence
   を補正した。

## 4. Files changed

- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0381-phase5-proof-obligation-matrix-and-handoff-artifact.md`
- `docs/reports/0382-review-phase5-proof-obligation-matrix-and-handoff-artifact-package.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z' && df -h . && free -h
2026-04-09 19:20 JST
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   90G  4.4G  96% /
               total        used        free      shared  buff/cache   available
Mem:           960Mi       743Mi        80Mi       1.2Mi       291Mi       217Mi
Swap:           19Gi       1.4Gi        18Gi

$ python3 scripts/new_report.py --slug phase5-proof-obligation-matrix-and-handoff-artifact
/home/yukatayu/dev/mir_poc_01/docs/reports/0381-phase5-proof-obligation-matrix-and-handoff-artifact.md

$ python3 scripts/validate_docs.py && git diff --check
Documentation scaffold looks complete.
Found 381 numbered report(s).

$ git status --short && git diff --stat
[review 前の changed file list と diffstat を確認]
```

review は reviewer subagent を 1 回だけ起動し、completion まで待った。

## 6. Evidence / findings

- `specs/examples/127...` で、proof obligation row と helper-local row を分ける current first choice は `126...` と整合した。
- current first choice は次である。
  - proof-obligation matrix は docs 正本
  - external handoff artifact は source evidence を参照する mixed row bundle sketch
  - detached static gate artifact / `checked_reason_codes` / shared-space witness bundle は handoff row そのものではなく source evidence 側
- reviewer finding は 2 件だった。
  1. shared-space example が provider receipt attachment を evidence 側へ十分に出していない
  2. second inventory package close を支える report / progress log / traceability evidence が未補完
- いずれも narrow に補正し、package close の mirror drift は解消した。

## 7. Changes in understanding

- Phase 5 は `126...` の 4-way split だけでなく、`127...` で proof-obligation matrix と mixed handoff sketch まで source-backed に切ったと読める。
- ただし handoff artifact 自体は still docs-only sketch であり、actual emitter / public checker API / boundary-specific artifact split は OPEN に残る。
- shared-space witness core と provider receipt attachment は、Phase 5 側でも混ぜずに `evidence_refs` でだけ接続する方が current Phase 4 cut と整合する。

## 8. Open questions

- mixed row bundle を維持するか、boundary-specific handoff artifact に割るか
- actual handoff emitter をどこで切るか
- `subject_ref` / `evidence_refs` の canonical serialization をどう narrow に決めるか
- low-level memory-order family を将来 external vocabulary としてだけ残すか

## 9. Suggested next prompt

Phase 5 later reopen 候補として、mixed row bundle を維持するか boundary-specific handoff artifact に割るかの threshold comparison を進めてください。existing detached/static/shared-space artifact を source evidence に留める current cutは維持してください。
