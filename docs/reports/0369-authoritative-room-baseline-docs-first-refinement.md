# 0369 — authoritative room baseline docs-first refinement

## Objective

Phase 4 前半の `authoritative room baseline の docs-first 精密化` を current self-driven portion まで進め、existing shared-space comparison chain を 1 本の baseline judgment に畳む。

## Scope and assumptions

- shared-space / membership line のうち、authoritative room の current baseline だけを扱う
- final activation / authority / auth / consistency / fairness catalog は固定しない
- production runtime / actual consensus algorithm / auth stack 実装には進まない
- current checkpoint では baseline closeout と mainline shift だけを行う

## Documents consulted

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
- `docs/reports/0270-shared-space-activation-rule-comparison.md`
- `docs/reports/0278-shared-space-authoritative-game-room-profile-comparison.md`
- `docs/reports/0280-shared-space-reconnect-and-inflight-policy-cut.md`
- `docs/reports/0284-shared-space-fairness-trust-model-comparison.md`

## Actions taken

1. authoritative room line の既存 judgment を再読し、baseline に入るものと policy option に残すものを分離した。
2. `specs/examples/121-shared-space-authoritative-room-baseline.md` を追加し、current baseline を
   - `authority-ack`
   - `single room authority`
   - `authoritative serial transition`
   - `authority_rng`
   の 4 軸 bundle と、membership registry / fairness trust / reconnect kernel / compile-time over-approximation の side condition に畳んだ。
3. `Documentation.md` と `specs/00-document-map.md` に導線を追加した。
4. `plan/16` に baseline consolidation note を追加し、`plan/12` に authoritative room baseline は current baseline であって final catalog 固定ではないことを追記した。
5. `plan/11`、`plan/17`、`progress.md`、`tasks.md` を更新し、authoritative room baseline package を checkpoint close に移し、current mainline を consistency / fairness / causal metadata catalog comparison へ送った。
6. `plan/90-source-traceability.md` に今回の source addendum を追記した。

## Files changed

- `specs/examples/121-shared-space-authoritative-room-baseline.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`

## Evidence / outputs / test results

Resource check before substantial work:

```text
2026-04-09 15:12 JST
df -h .  -> /dev/vda2 99G total / 90G used / 4.7G avail / 96%
free -h  -> 960Mi total / 721Mi used / 239Mi available / swap 19Gi total
```

Commands run:

```text
git status --short --branch
date '+%Y-%m-%d %H:%M JST'
sed -n ... plan/16-shared-space-membership-and-example-boundary.md
sed -n ... tasks.md
sed -n ... plan/11-roadmap-near-term.md
sed -n ... plan/17-research-phases-and-autonomy-gates.md
sed -n ... progress.md
rg -n "authority-ack|single room authority|authoritative serial transition|authority_rng|delegated_rng_service|auditable authority witness|authoritative game room" ...
python3 scripts/validate_docs.py
git diff --check
```

Validation results:

```text
python3 scripts/validate_docs.py
-> Documentation scaffold looks complete.
-> Found 369 numbered report(s).

git diff --check
-> no output
```

## What changed in understanding

- authoritative room line は、比較 chain のまま保持するよりも、baseline judgment を 1 本に畳んだ方が current mainline を進めやすい
- current baseline は
  - `authority-ack`
  - `single room authority`
  - `authoritative serial transition`
  - `authority_rng`
  で十分 source-backed であり、ここから先の `delegated_rng_service` / `auditable authority witness` / epoch split は catalog comparison 側へ送ってよい
- したがって Phase 4 前半の `authoritative room baseline の docs-first 精密化` は current self-driven portion として checkpoint close に移し、次の主線は consistency / fairness / causal metadata catalog comparison に送るのが自然である

## Open questions

- final activation rule を overlay policy としてどこまで room profile から差し替えられるか
- `auditable authority witness` を room profile 側にどこまで書き、どこから audit serialization に残すか
- `delegated_rng_service` と fairness witness の接続をどこまで baseline 近傍で扱うか
- `relaxed merge-friendly room` を current working subset に上げる threshold

## Suggested next prompt

`consistency / fairness / causal metadata catalog comparison` を次の主線として進め、authoritative room baseline からどの strengthening candidate を先に reopen するかを narrow に比較してください。まずは `delegated_rng_service` / `auditable authority witness` / epoch-incarnation split の 3 軸を、authoritative room と append-friendly room の両方にまたがる working subset として整理してください。
