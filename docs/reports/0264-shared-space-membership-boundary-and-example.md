# Report 0264 — shared-space / membership boundary と practical example の整理

- Date: 2026-04-08T00:25:24.400308Z
- Author / agent: Codex
- Scope: shared-space / participant churn / practical example の docs-first boundary 整理。current L2 core semantics や parser-free PoC mainline は変更しない。
- Decision levels touched: L2 / L3 comparison、OPEN question refinement、plan / progress mirror

## 1. Objective

user から出た次の論点について、current repo の source と blog 2 本を突き合わせ、**どこまで self-driven に詰めてよいか**を practical example 付きで整理する。

- participant を array-like に持つべきか
- join / leave / rejoin と causal metadata をどう分けるか
- `Raft` / `Paxos` をどの layer に置くのが自然か
- 「誰でも途中参加してコマを動かせる shared game」のような例を、current architecture でどう切るか

また、current repo の mainline と upper-layer exploration の境界が見えやすいように `plan/` と `progress.md` を更新する。

## 2. Scope and assumptions

- 規範判断の正本は `specs/` に残る。
- 今回追加するのは docs-first の boundary comparison と practical example であり、shared-space protocol の final specification ではない。
- current repo の architectural line に従い、単一の consensus algorithm には commit しない。
- participant churn は current L2 mainline に入れず、shared-space / Mirrorea future work として扱う。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/08-cross-system-relations.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `docs/reports/0111-faq-unresolved-issues-current-state.md`
- blog `https://blog.yukatayu.tech/blog/sync_language_01/`
- blog `https://blog.yukatayu.tech/blog/sync_language_02/`

## 4. Actions taken

1. current repo の規範 source と existing plan / report chain を読み直し、shared-space / membership 論点がどの layer に属するかを再確認した。
2. blog 2 本を参照し、初期アイデアにあった
   - evaluator placement
   - participant locality
   - layered graph
   - vector-clock 的 causal concern
   が current architecture のどこに対応するかを整理した。
3. `plan/16-shared-space-membership-and-example-boundary.md` を追加し、
   - participant plain array を core に焼き込まない
   - session-scoped membership registry を第一候補にする
   - authoritative game room と relaxed append room を対比例として置く
   - `Raft` / `Paxos` を implementation choice として位置づける
   という current working boundary を practical example 付きで整理した。
4. `plan/12-open-problems-and-risks.md` に dynamic membership / causal metadata の具体化メモを追加した。
5. `plan/10-roadmap-overall.md`、`plan/00-index.md`、`plan/90-source-traceability.md`、`progress.md` を mirror 更新した。

6. 変更ファイルは次の通りである。
   - `plan/00-index.md`
   - `plan/10-roadmap-overall.md`
   - `plan/12-open-problems-and-risks.md`
   - `plan/16-shared-space-membership-and-example-boundary.md`
   - `plan/90-source-traceability.md`
   - `progress.md`
   - `docs/reports/0264-shared-space-membership-boundary-and-example.md`

## 5. Evidence / outputs / test results

### dirty state / resource check

```text
$ git status --short --branch
## main...origin/main
```

```text
$ df -h . && free -h
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   87G  7.5G  93% /
               total        used        free      shared  buff/cache   available
Mem:           960Mi       768Mi        85Mi       1.1Mi       263Mi       192Mi
Swap:           19Gi       1.4Gi        18Gi
```

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 09:24 JST
```

### validation

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 264 numbered report(s).
```

```text
$ git diff --check
<no output>
```

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 09:30 JST
```

### review path

- reviewer subagent を 1 回だけ起動し、semantic over-commitment と boundary drift を確認した。
- completion では substantive finding は返らなかった。

### findings

#### current repo で source-backed に言えること

- `specs/03-layer-model.md` は L3 shared space の責務を `session / space abstraction`、`shared object` 同期、consistency mode selection としている。
- `specs/05-mirrorea-fabric.md` と `specs/01-charter-and-decision-levels.md` は、単一の consensus algorithm に commit しない方針を明示している。
- `plan/12-open-problems-and-risks.md` と `docs/reports/0111` から、dynamic membership / causal metadata は current L2 mainline の外にある serious OPEN problem と読める。

#### practical example から見える current working judgment

今回の docs-first comparison では、次を current working boundary として採った。

1. participant 集合は **plain array を source of truth にしない**。
2. source of truth は **session-scoped membership registry** を第一候補にする。
3. array-like participant list は **derived snapshot view** に留める。
4. join / leave / rejoin は causal metadata と混ぜず、少なくとも `logical_member_id` / `incarnation` / `activation_state` を分ける方向が自然である。
5. `Raft` / `Paxos` は、authoritative room の replication や activation agreement の **implementation family** としては自然だが、language core / Mirrorea spec の必須語彙にはしない。

#### example comparison から見えること

- 「authoritative すごろく room」は
  - shared mutable state
  - exclusive action
  - global reset
  - participant churn
  を同時に持つため、authoritative transition log が自然である。
- 「shared notice board / presence room」は
  - append-only に近く
  - simultaneous append を許せる
  ため、same membership carrier のままでも consistency mode はより緩くできる。

したがって、**participant carrier と consistency mode catalog は分けて設計すべき**という理解が強化された。

## 6. What changed in understanding

- shared-space / membership は依然として `要仕様確認` 領域だが、docs-first boundary と practical example の整理までは self-driven に進められると確認できた。
- participant を array-like にするかという問いに対しては、current repo の architecture に照らすと、
  - source of truth = membership registry
  - array-like list = derived snapshot
  の切り分けが最も自然だと整理できた。
- vector-clock deletion 問題については、plain causal carrier の工夫だけでなく、**membership reconfiguration を別 layer に切り出す**ことが先である、という理解がより明確になった。

## 7. Open questions

1. active 化の final rule
   - authority 1 点承認
   - `all_of` 的 activation
   - quorum-like activation
2. identity / authentication model
3. RNG / fairness model
4. reset / notification の exact guarantee
5. shared-space ごとの consistency mode catalog
6. reconnect / late leave / in-flight action の final policy
7. 上位空間を game / collaborative editor / Reversed Library のどれから具体化するか

## 8. Suggested next prompt

`plan/16-shared-space-membership-and-example-boundary.md` を前提に、authoritative shared game room の最小 consistency contract を docs-only で narrow に比較してください。特に join / leave / rejoin の activation rule を、authority-ack / all_of / quorum-like の 3 案で比べ、何を language core に入れず、何を shared-space operational policy に残すべきかを practical example つきで整理してください。
