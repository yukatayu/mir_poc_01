# 0371 — shared-space catalog working subset comparison

- Date: 2026-04-09T06:59:04.189777Z
- Author / agent: Codex
- Scope: Phase 4 前半 closeout 後の current mainline として、consistency / fairness / causal metadata catalog の first cut を docs-first で整理する
- Decision levels touched: L2 / current working judgment

## 1. Objective

authoritative room baseline を checkpoint close に置いた後の次段として、

- consistency mode
- fairness source
- fairness claim
- causal membership carrier

を room profile catalog の **working subset** としてどこまで current phase に入れてよいかを整理する。

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
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `docs/reports/0278-shared-space-authoritative-game-room-profile-comparison.md`
- `docs/reports/0282-shared-space-causal-metadata-and-membership-epoch-comparison.md`
- `docs/reports/0284-shared-space-fairness-trust-model-comparison.md`
- blog `https://blog.yukatayu.tech/blog/sync_language_02/`

## 3. Actions taken

1. `specs/examples/122-shared-space-catalog-working-subset-comparison.md` を追加し、authoritative room と append-friendly room をまたぐ small cross-room working subset row と stop line を定義した。
2. row-level subset を
   - `authoritative_room_baseline`
   - `authoritative_room_witnessed_draw`
   - `append_friendly_room_baseline`
   - `append_friendly_room_with_rng_capability`
   の 4 本に絞った。
3. `delegated_rng_service` を provider placement の practical candidate、`auditable_authority_witness` を fairness claim strengthening の next narrow step として別軸に保つ wording を明示した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/16`、`plan/17`、`plan/90`、`progress.md`、`tasks.md` を更新し、catalog comparison の current first cut と次段の mainline を mirror した。

## 4. Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/122-shared-space-catalog-working-subset-comparison.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0371-shared-space-catalog-working-subset-comparison.md`
- `docs/reports/0372-review-shared-space-catalog-working-subset-comparison.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-09 15:58 JST

$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   90G  4.6G  96% /

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       784Mi        78Mi       1.2Mi       251Mi       175Mi
Swap:           19Gi       1.3Gi        18Gi

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 371 numbered report(s).

$ git diff --check
[no output]
```

## 6. Evidence / findings

- authoritative room baseline だけでは、append-friendly room を含む catalog working subset の row-level 比較単位が弱かった。
- row を 4 本に絞ることで、
  - current baseline
  - fairness claim の strengthening
  - append-friendly baseline
  - append-friendly 側の optional RNG capability
  を final catalog に見せずに source-backed に並べられるようになった。
- current next narrow step は provider placement をさらに広げることではなく、`auditable_authority_witness` の最小 witness shape comparison に置く方が自然だと整理できた。

## 7. Changes in understanding

- shared-space の current mainline は「catalog を final に広げること」ではなく、「working subset の row を切り、stop line を明示すること」である。
- `delegated_rng_service` と `auditable_authority_witness` は同じ strengthening ではなく、
  - provider placement
  - fairness claim
  の別軸として扱う方が一貫している。
- append-friendly room では fairness を room core の必須核にしない row を持つ方が、authoritative room との差が分かりやすい。

## 8. Open questions

- `auditable_authority_witness` の最小 witness shape を room profile にどこまで書くか。
- `delegated_rng_service` を authoritative room 側でも practical candidate としてどこまで row に昇格させるか。
- control-plane separated carrier を reopen する threshold をどこに置くか。

## 9. Suggested next prompt

`shared-space の consistency / fairness / causal metadata catalog comparison の次段として、auditable authority witness の最小 witness shape を docs-first に比較し、provider placement と fairness claim strengthening を別軸のまま保てる最小 carrier を整理してください。`
