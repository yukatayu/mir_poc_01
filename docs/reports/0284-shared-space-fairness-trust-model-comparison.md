# 0284 — shared-space fairness trust model comparison

## Objective

shared-space authoritative room の残課題として、RNG provider placement とは別軸に fairness trust model を比較し、current phase の minimal candidate と next narrow strengthening candidate を docs-first に整理する。

## Scope and assumptions

- 今回は docs-only comparison に留める。
- `authority_rng` / `delegated_rng_service` / `distributed_randomness_provider` という provider placement line は既存 judgmentとして前提にする。
- 今回の焦点は provider の置き場所ではなく、`draw が fair だった` という claim を誰がどこまで担保するかである。
- final cryptographic proof format、receipt serialization、identity / auth stack は固定しない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `docs/reports/0276-shared-space-rng-fairness-source-placement.md`
- `docs/reports/0278-shared-space-authoritative-game-room-profile-comparison.md`
- `docs/reports/0282-shared-space-causal-metadata-and-membership-epoch-comparison.md`

## Actions taken

1. `plan/16` に fairness trust model の比較 section を追加した。
2. `opaque authority trust` / `auditable authority witness` / `delegated provider attestation` / `distributed fairness protocol` の 4 案を整理した。
3. authoritative room では provider placement と fairness witness requirement を別軸に保つ current line を明示した。
4. `plan/12` と `progress.md` に current working judgment を mirror した。
5. `plan/90` に traceability mirror を追加した。

## Files changed

- `docs/reports/0284-shared-space-fairness-trust-model-comparison.md`
- `docs/reports/0285-review-shared-space-fairness-trust-model-comparison.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 12:25 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 285 numbered report(s).

$ git diff --check
[no output]
```

## Evidence / outputs / test results

Task start dirty state:

```text
## main...origin/main
```

Reviewer:

- reviewer completion をこの task の wait window 内では取得できなかったため、local evidence fallback で close out した
- local review では provider placement と witness requirement が別軸に保たれていること、`opaque authority trust` が current minimal candidate に留まっていること、`distributed fairness protocol` が future comparison に残っていることを確認した

## What changed in understanding

- `authority_rng` と `delegated_rng_service` の違いだけでは fairness claim の強さを十分に表現できないため、provider placement と witness requirement を別軸に保つ必要がある。
- authoritative room の current minimal candidate は引き続き `opaque authority trust` だが、next narrow strengthening candidate は `delegated provider attestation` より先に `auditable authority witness` を置く方が、proof / replay / debug hook の境界をきれいに切れる。
- distributed fairness protocol は current room-profile line に混ぜず future research に残すのが妥当である。

## Open questions

- `auditable authority witness` の最小 witness shape をどこまで room profile に書き、どこから audit serialization へ送るか。
- provider attestation を auth / identity layering とどの段で接続するか。
- append-friendly room に fairness claim を optional capability としてどう載せるか。

## plan/progress update status

- `plan/` 更新あり
- `progress.md` 更新あり

## Suggested next prompt

`shared-space identity / auth layering を participant carrier / authority placement / fairness trust model と混ぜずにどこで切るべきか、current shared-space line の残課題として narrow に比較してください。`
