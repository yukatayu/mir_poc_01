# 0268 — shared-space authority / ownership / consistency comparison

## Objective

shared-space / membership line の self-driven research を 1 段進め、participant carrier とは別軸で

- authority placement
- resource ownership / delegated capability
- consistency mode
- RNG / fairness provider placement

をどう切るのが current repo の layer separation と整合するかを、日本語の docs / plan mirror として整理する。

## Scope and assumptions

- current repo の規範判断の正本は `specs/` に残す。
- 今回は `shared-space` 上位層の **docs-first boundary refinement** に留める。
- `Raft` / `Paxos` / `VRR` / CRDT などを final spec に固定しない。
- `participant` tree-like view は derived snapshot として認めるが、**current source-of-truth model は session-scoped membership registry** であり、final carrier naming / serialization detail はまだ未決である。
- `identity / auth`、active 化規則、consistency mode catalog、fairness trust model の finalization は引き続き user 仕様確認を要する。

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
- `specs/08-cross-system-relations.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/10-roadmap-overall.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `docs/reports/0264-shared-space-membership-boundary-and-example.md`
- `docs/reports/0266-shared-space-membership-research-line-refresh.md`

## Actions taken

1. shared-space line の open question を、participant carrier と separate axis に分解した。
2. `plan/16` に次を追加した。
   - shared-space resource ownership / delegation の current working model
   - `ResourceAuthority<R> = { owner_ref, delegation_set, handoff_epoch }` という概念 carrier
   - `exclusive-authority invariant` / `delegation-is-not-coownership invariant` / `explicit-handoff invariant` / `mode-compatibility invariant`
   - authoritative game room と append-friendly room の対比例
   - ownership model と consistency mode の相性
3. `plan/12` に `shared-space authority / resource ownership` risk を追加した。
4. `plan/10` に、shared-space self-driven scope として authority / delegated capability / RNG provider placement comparison を追記した。
5. `AGENTS.md` と `plan/91` に、long-running research で
   - PoC 検証と formal boundary / proof obligation を並走させること
   - portability / observability / graph export hook を replaceable layer として残すこと
   - subagent は latency だけで早切りせず completion まで待つこと
   - 不要になった subagent は close すること
   を明記した。
6. `progress.md` を更新し、shared-space line の current understanding と rough progress を補正した。
7. `plan/90` を更新し、`plan/10` / `plan/16` / `plan/91` の今回変更が `0268` / `0269` に依拠することを traceability に反映した。

## Files changed

- `docs/reports/0268-shared-space-authority-ownership-and-consistency-comparison.md`
- `docs/reports/0269-review-shared-space-authority-doc-change.md`
- `AGENTS.md`
- `plan/10-roadmap-overall.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `progress.md`

## Evidence / outputs / test results

Task start dirty state:

```text
## main...origin/main
```

Resource check:

```text
$ df -h .
/dev/vda2        99G   87G  7.4G  93% /

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       552Mi        98Mi       2.0Mi       309Mi       317Mi
Swap:           18Gi       224Ki        18Gi
```

Timestamp used for progress update:

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 10:54 JST
```

Validation after review fix:

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 269 numbered report(s).

$ git diff --check
[no output]
```

Reviewer:

- reviewer 1 回実施
- substantive finding 2 件
  - authoritative room 例で RNG provider placement が owner slot に再混線していた
  - report wording が membership registry の current working judgment を弱く書きすぎていた
- いずれも task 内で反映済み

## What changed in understanding

- `participant` carrier、resource owner、delegated capability、consistency mode、RNG / fairness source は、同じ room を扱っていても同一 carrier に潰さない方が理論上きれいだと再確認した。
- 「すべての resource に owner がいる」は useful な直感だが、その `owner` は人間 participant に限らず、**authoritative write authority slot** と読む方が shared-space と Mir core の境界を守りやすい。
- `delegation` は co-ownership ではなく、resource 本体の ownership を増やさず capability を限定的に配る carrier として読むのが current working model である。
- authoritative game room、append-friendly room、relaxed / merge-friendly room では ownership / delegation の自然な切り方が違うため、membership carrier と別軸 comparison に留めるのが current phase に合う。

## Open questions

- active 化規則を authority 1 点承認 / full-coverage-like / quorum-like のどこで最終化するか。
- identity / auth を shared-space control-plane とどう接続するか。
- fairness / RNG を `authority_rng` / `delegated_rng_service` / distributed provider のどこで切るか。
- replicated authority を spec にどう持ち込まず operational realization に残すか。
- relaxed / merge-friendly room で local owner / merge authority / conflict resolution policy をどこまで catalog 化するか。

## plan/progress update status

- `plan/` 更新あり
- `progress.md` 更新あり

## Suggested next prompt

`shared-space authoritative room の active 化規則について、authority-ack / full-coverage-like / quorum-like の 3 案を、authoritative game room と append-friendly room の対比つきで narrow に比較してください。`
