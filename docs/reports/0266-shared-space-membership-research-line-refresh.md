# Report 0266 — shared space membership research line refresh

- Date: 2026-04-08T01:21:26.407040Z
- Author / agent: Codex
- Scope: shared-space / membership / authority / consistency / randomness の current working boundary を、user の追質問に応じて repository memory へ反映する。新しい規範仕様や final algorithm choice は追加しない。
- Decision levels touched: L3 open question / current working boundary の整理のみ。新しい settled decision は作らない。

## 1. Objective

次を source-backed に整理し、`plan/` と `progress.md` へ反映する。

1. participant を tree-like JSON view とみなす直感を認めつつ、なぜ source of truth は session-scoped membership registry の方が理論的に自然か
2. activation visibility を compile-time にどこまで見抜けるか
3. authority / consistency / RNG provider placement の次段 research line
4. この整理の結果、shared-space workstream がどこまで self-driven に進められ、どこから user 仕様確認が必要か

## 2. Inputs consulted

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
- `docs/reports/0111-faq-unresolved-issues-current-state.md`
- `docs/reports/0264-shared-space-membership-boundary-and-example.md`
- `docs/reports/0265-shared-space-open-questions-snapshot.md`

## 3. Actions taken

1. shared-space / membership / authority まわりの規範寄り文書と current repository memory を読み直した。
2. `plan/16-shared-space-membership-and-example-boundary.md` に、tree-like view と registry の役割分担、activation visibility の compile-time over-approximation、authority / consistency / RNG provider placement の research line を追記した。
3. `progress.md` の shared-space row と簡潔ログを更新し、現在地の rough snapshot を補正した。
4. `plan/90-source-traceability.md` に今回の根拠 report を追加した。
5. reviewer を 1 回だけ回し、activation policy wording と compile-time example に関する 2 finding を受けて boundary-safe に補正した。review record は `docs/reports/0267-review-shared-space-membership-boundary-refinement.md` に残した。

## 4. Files changed

- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0266-shared-space-membership-research-line-refresh.md`
- `docs/reports/0267-review-shared-space-membership-boundary-refinement.md`

## 5. Commands run and exact outputs

```text
$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   87G  7.4G  93% /
```

```text
$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       685Mi        77Mi       1.2Mi       353Mi       274Mi
Swap:           19Gi       1.0Gi        18Gi
```

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 10:18 JST
```

```text
$ git status --short --branch
## main...origin/main
 M plan/16-shared-space-membership-and-example-boundary.md
 M plan/90-source-traceability.md
 M progress.md
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 267 numbered report(s).
```

```text
$ git diff --check
<no output>
```

## 6. Evidence / findings

### tree-like participant view と registry source-of-truth は両立する

- tree-like JSON / snapshot view は user-facing には自然であり、否定しない。
- ただし source of truth を tree にすると、identity、activation、incarnation、causal actor key、routing / authority ref が structural position に混ざりやすい。
- よって current working line は、
  - source of truth = `SessionMembers<Role>` 的 registry
  - tree-like / array-like list = derived snapshot view
 である。

### activation visibility は compile-time で閉じ切れない

- authority、watcher role、publish / notify path のような declaration は compile-time に over-approximation できる。
- しかし open room、late join、reconnect、authority handoff は runtime control-plane でしか確定しない。
- よって compile-time にできるのは visibility policy の structural floor までであり、actual dissemination / reconciliation は runtime に残る。

### authority / consistency / RNG は participant carrier と別軸で扱う

- same membership carrier の上に、
  - authoritative sugoroku room
  - append-friendly notice room
  を載せられる形が理論的に自然である。
- したがって participant carrier を決めることは authority placement や consistency mode を一意には決めない。
- RNG も tree topology に埋め込むより、`perform via authority_rng` / `delegated_rng_service` / `distributed_randomness_provider` のような explicit provider capability として置く方が audit / fairness reasoning がしやすい。

### self-driven に進めてよい範囲

- docs-first の authority placement comparison
- small consistency mode catalog comparison
- RNG provider placement comparison
- activation visibility の static over-approximation comparison

は継続してよい。

一方で、次はまだ user 仕様確認が要る。

- activation rule の final profile
- identity / auth の最終強度
- fairness / RNG の最終 trust model
- reconnect / late leave / in-flight action policy
- shared-space ごとの final consistency catalog

## 7. Changes in understanding

- shared-space の participant 表現について、「tree-like に見たい」という直感は否定しなくてよいが、source of truth を tree にする必要はない、という整理が明確になった。
- activation visibility は compile-time に全部決める問題ではなく、static over-approximation と runtime control-plane を分ける問題だと明確化した。
- RNG / fairness は shared-space topology ではなく provider placement の問題として切る方が、authority と audit の境界が綺麗になる。
- reviewer 指摘により、Mir-1 `durable_cut` の `all_of` / quorum-like と shared-space activation policy を同じ語彙として再利用しないこと、compile-time 例から runtime active set を外すことを明示した。

## 8. Open questions

1. authoritative room の active 化を authority-ack / full-coverage-like activation / quorum-like activation のどこに置くか
2. authoritative room と append-friendly room を catalog 化するときの最小 mode set
3. `authority_rng` と `delegated_rng_service` のどちらを first practical candidate にするか
4. reconnect と late leave を activation visibility policy にどう入れるか
5. `membership_epoch` と route / patch epoch をどこまで共有するか

## 9. Suggested next prompt

`shared-space の authority placement を、authoritative sugoroku room と append-friendly notice room の 2 例で、single authority / replicated authority / relaxed merge room の 3 案比較として docs-first に整理し、activation visibility policy と RNG provider placement への影響まで含めて narrow に検討してください。`
