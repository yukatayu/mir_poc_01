# Report 0440 — phase5 actual materialized runtime handoff threshold

- Date: 2026-04-10 04:00 JST
- Author / agent: Codex
- Scope: Phase 5 theorem-line later package の次段として、transcript-ready retained bridge に `materialized_runtime_handoff_ref` を足してよいかを docs-first に比較し、current next reopen を concrete payload / transcript body comparison へ狭める
- Decision levels touched: L2 examples / non-normative roadmap and repository memory wording

## 1. Objective

`specs/examples/165...` で `runtime_transcript_ref` までを theorem-line retained bridge に寄せた後の次段として、

- `materialized_runtime_handoff_ref` だけを先に入れる
- concrete payload / transcript body とまとめて still 外に残す
- materialized handoff と concrete payload をまとめて入れる

の 3 案を比較し、current first choice を 1 本に固定する。

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
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `specs/examples/163-current-l2-theorem-line-wording-ready-runtime-handoff-threshold.md`
- `specs/examples/164-current-l2-theorem-line-runtime-handoff-ready-invocation-receipt-threshold.md`
- `specs/examples/165-current-l2-theorem-line-invocation-receipt-ready-runtime-transcript-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## 3. Actions taken

1. theorem-line retained bridge の current stop line を `165...` まで読み直し、`actual_runtime_handoff_ref`、`emitted_invocation_receipt_ref`、`runtime_transcript_ref` の連鎖の次に何を足すのが最小かを comparison 対象として切り出した。
2. `specs/examples/166-current-l2-theorem-line-transcript-ready-materialized-runtime-handoff-threshold.md` を追加し、`materialized_runtime_handoff_ref` だけを先に足す案を current first choice に固定した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot に揃えた。
4. review record 雛形を作り、closeout review の準備をした。

## 4. Files changed

- `specs/examples/166-current-l2-theorem-line-transcript-ready-materialized-runtime-handoff-threshold.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0440-phase5-actual-materialized-runtime-handoff-threshold.md`
- `docs/reports/0441-review-phase5-actual-materialized-runtime-handoff-threshold.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z%n%Y-%m-%dT%H:%M:%S%z'
2026-04-10 03:48 JST
2026-04-10T03:48:19+0900

$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   92G  2.8G  98% /

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       754Mi        81Mi       6.0Mi       124Mi       206Mi
Swap:           19Gi       1.1Gi        18Gi

$ rg -n "actual notebook runtime handoff materialization|notebook exchange rule threshold|concrete payload / transcript body|materialized_runtime_handoff_ref" progress.md plan/11-roadmap-near-term.md plan/17-research-phases-and-autonomy-gates.md tasks.md docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md specs/examples/166-current-l2-theorem-line-transcript-ready-materialized-runtime-handoff-threshold.md
[matches inspected locally]

$ date '+%Y-%m-%d %H:%M %Z'
2026-04-10 04:00 JST
```

## 6. Evidence / findings

- transcript-ready retained bridge の次段としては、concrete payload / transcript body を current theorem-line bridge に持ち込むより、`materialized_runtime_handoff_ref` だけを先に足す方が narrow である。
- `materialized_runtime_handoff_ref` は runtime transcript とは別に、「runtime から theorem-side consumer に handoff する materialized package が存在する」という fact だけを symbolically 保持できるため、bridge を太らせずに reopen を 1 段進められる。
- concrete payload / transcript body / actual serialized channel body を同時に入れると、proof boundary ではなく runtime transport / serialization policy まで current package に押し込むことになり、Phase 5 current split と衝突しやすい。
- current first choice を `materialized_runtime_handoff_ref` に留めることで、次の later reopen を **concrete payload / transcript body comparison** へ狭く残せる。
- `plan/ 更新済み`
- `progress.md 更新済み`
- `tasks.md 更新済み`

## 7. Changes in understanding

- theorem-line later package の symbolic retained bridge は、`runtime_transcript_ref` を超えても concrete payload を deferred に残したまま 1 段だけ伸ばせる。
- Phase 5 の reopen pressure は「actual materialized handoff を入れるかどうか」ではなく、「materialized handoff fact の次に concrete payload / transcript body を入れるかどうか」に移った。

## 8. Open questions

- concrete payload / transcript body を 1 field family として扱うか、payload body と transcript body を別 comparison に分けるか。
- materialized runtime handoff fact を docs-only retained bridge に留め続けるか、将来 mixed handoff artifact row に移す pressure があるか。
- proof-side consumer が `proof_notebook` 以外に増えたとき、`materialized_runtime_handoff_ref` が十分に generic か。

## 9. Suggested next prompt

Phase 5 theorem-line later package の次段として、`materialized_runtime_handoff_ref` までは retained bridge に入れる current first choiceを前提に、concrete payload / transcript body / actual serialized channel body をどこまで theorem-line bridge に寄せるかを narrow comparison してください。
