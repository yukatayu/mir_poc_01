# 0280 — shared-space reconnect / late leave / in-flight policy cut

## Objective

shared-space room profile の practicalization 次段として、authoritative room における

- reconnect
- late leave
- in-flight action

を room profile 本体へどこまで入れるべきかを比較し、最小の policy cut を docs-first に整理する。

## Scope and assumptions

- 今回は docs-only comparison に留める。
- example は authoritative すごろく room を使う。
- actual timeout 秒数、retry 回数、network liveness probe、auth stack は finalization しない。
- current bundle は `authority-ack` + `single room authority` + `authoritative serial transition` + `authority_rng` を前提にする。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `plan/00-index.md`
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
- `docs/reports/0278-shared-space-authoritative-game-room-profile-comparison.md`
- `docs/reports/0279-review-shared-space-authoritative-game-room-profile-comparison.md`

## Actions taken

1. `plan/16` に reconnect / late leave / in-flight action policy placement の候補比較を追加した。
2. room profile に全部入れる案、全部 external policy に残す案、minimal split 案を比較した。
3. authoritative すごろく room で `member_incarnation` と pending action invalidation をどう読むかの pseudocode を追加した。
4. `plan/12` と `progress.md` に minimal policy cut を mirror した。
5. `plan/90` に traceability mirror を追加した。

## Files changed

- `docs/reports/0280-shared-space-reconnect-and-inflight-policy-cut.md`
- `docs/reports/0281-review-shared-space-reconnect-and-inflight-policy-cut.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 12:04 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 281 numbered report(s).

$ git diff --check
[no output]
```

## Evidence / outputs / test results

Task start dirty state:

```text
## main...origin/main
```

Reviewer:

- reviewer 1 回実施
- substantive semantic finding なし
- hygiene finding 2 件
  - `plan/90` の traceability mirror に `0280` / `0281` を追加する必要
  - report の command trace / final validation evidence を close out する必要
- いずれも task 内で反映済み

## What changed in understanding

- reconnect / late leave / in-flight action を全部 room profile に押し込むより、`member_incarnation` と uncommitted action invalidation だけを room-profile 側に残し、timeout / retry / resend は external policy layer に残す方が自然である。
- これにより、authoritative history の safety rule は room profile に残しつつ、network / UX / deployment 差は外部 policy へ逃がせる。
- `member_incarnation` は reconnect と causal metadata の接点になりうるので、次段で membership epoch / incarnation の比較へ進む自然な足場になる。

## Open questions

- reconnect window を完全に external policy へ出してよいか、それとも policy family 名だけ room profile に残すべきか。
- notification resend を room profile の audit concern と見なすか、delivery policy と見なすか。
- committed 直後 / notify 未達の edge case をどの layer に置くか。

## plan/progress update status

- `plan/` 更新あり
- `progress.md` 更新あり

## Suggested next prompt

`shared-space membership epoch / incarnation と causal metadata の接続について、plain vector deletion / epoch-incarnation split / control-plane separated carrier を narrow に比較してください。`
