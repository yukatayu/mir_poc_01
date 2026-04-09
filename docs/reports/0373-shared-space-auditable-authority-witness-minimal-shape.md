# Report 0373 — shared space auditable authority witness minimal shape

- Date: 2026-04-09T07:21:16.103533Z
- Author / agent: Codex
- Scope: Phase 4 shared-space / membership side line の fairness trust model comparison の次段として、`auditable_authority_witness` の minimal witness shape を docs-first で整理する
- Decision levels touched: L2 / L3

## 1. Objective

`specs/examples/122-shared-space-catalog-working-subset-comparison.md` で current working subset row を切った後の next narrow step として、

- `auditable_authority_witness`

を room profile と audit / receipt side にどう分けるのが自然か、
authoritative room と append-friendly room optional capability の両方に接続できる **minimal typed witness core** をどこで止めるかを比較する。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `tasks.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `specs/examples/121-shared-space-authoritative-room-baseline.md`
- `specs/examples/122-shared-space-catalog-working-subset-comparison.md`
- `docs/reports/0284-shared-space-fairness-trust-model-comparison.md`
- `docs/reports/0371-shared-space-catalog-working-subset-comparison.md`

## 3. Actions taken

1. `note-only witness`、`minimal typed witness bundle`、`expanded attested receipt` の 3 案を比較した。
2. `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md` を追加し、minimal witness core の current first choice を整理した。
3. `plan/16` に minimal witness core の summary を追加した。
4. `plan/11`、`plan/12`、`plan/17`、`progress.md`、`tasks.md` を更新し、next narrow step を provider-placement comparison に移した。
5. `Documentation.md` と `specs/00-document-map.md` に導線を追加した。

## 4. Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0373-shared-space-auditable-authority-witness-minimal-shape.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z' && df -h . && free -h
2026-04-09 16:19 JST
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   90G  4.5G  96% /
               total        used        free      shared  buff/cache   available
Mem:           960Mi       772Mi        75Mi       1.2Mi       266Mi       187Mi
Swap:           19Gi       1.3Gi        18Gi

$ python3 scripts/new_report.py --slug shared-space-auditable-authority-witness-minimal-shape
/home/yukatayu/dev/mir_poc_01/docs/reports/0373-shared-space-auditable-authority-witness-minimal-shape.md

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 374 numbered report(s).

$ git diff --check
[no output]
```

## 6. Evidence / findings

- `auditable_authority_witness` を `opaque authority trust` より 1 段だけ強い claim として切るには、room profile に payload を持ち込まず、`fairness_claim` だけを残す方が自然である。
- current first choice の minimal witness core は
  - `witness_kind`
  - `action_ref`
  - `draw_slot`
  - `draw_result`
  の 4 field で十分である。
- `action_ref` を room-local stable ref にしておけば、authoritative room の turn transition と append-friendly room の optional draw append の両方に接続できる。
- provider receipt / signature / auth evidence を入れ始めると `delegated_provider_attestation` を先取りしすぎる。
- `python3 scripts/validate_docs.py` は成功し、`git diff --check` は無出力だった。

## 7. Changes in understanding

- `auditable_authority_witness` は provider placement の comparison とは別に、audit / receipt side の small typed bridge として先に切れる。
- `action_ref + draw_slot + draw_result` までに留めると、membership / causality / auth layering を同じ carrier に潰さずに済む。
- Phase 4 current mainline の next narrow step は witness shape 自体ではなく、`delegated_rng_service` を authoritative room 側でもどこまで practical candidate に読むかへ移してよい。

## 8. Open questions

- `action_ref` の final room-local stable ref policy をどの layer で固定するか。
- `draw_slot` を final exporter / parser / debug UI でどう naming するか。
- `delegated_rng_service` を authoritative room 側でも provider-placement candidate として入れたとき、minimal witness core をそのまま維持できるか。
- control-plane separated causal carrier を reopen する threshold。

## 9. Suggested next prompt

`shared-space の authoritative room について、delegated_rng_service を provider-placement candidate としてどこまで practical に読めるかを docs-first に比較し、authority_rng baseline と minimal auditable authority witness core を壊さない最小 cut を整理してください。`
