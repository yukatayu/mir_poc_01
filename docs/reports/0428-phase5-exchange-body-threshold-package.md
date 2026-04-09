# Report 0428 — phase5 exchange-body threshold package

- Date: 2026-04-10 01:56 JST
- Author / agent: Codex GPT-5
- Scope: Phase 5 theorem-line later reopen package for the exchange-body threshold and its mirror updates
- Decision levels touched: L2 docs-first threshold judgment and repository-memory mirror maintenance

## 1. Objective

Phase 5 theorem-line later reopen の current next package として、
`consumer_invocation_surface_ref` まで届いた retained bridge に
`exchange_rule_body_ref` をどこまで足してよいかを docs-first で比較し、
runtime coupling / transport / failure body は still later に残す current threshold を固定する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/155-current-l2-theorem-line-validation-ready-invocation-surface-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## 3. Actions taken

1. `exchange-body threshold` を
   - invocation-ready retained bridge を terminal cut にする案
   - `exchange_rule_body_ref` だけを足す案
   - `exchange_rule_body_ref + runtime_coupling_ref` をまとめて足す案
   の 3 案で比較した。
2. [156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md](/home/yukatayu/dev/mir_poc_01/specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md) を追加し、current first choice を **`exchange_rule_body_ref` のみ追加** に固定した。
3. [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)、[00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)、[11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)、[12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)、[13-heavy-future-workstreams.md](/home/yukatayu/dev/mir_poc_01/plan/13-heavy-future-workstreams.md)、[17-research-phases-and-autonomy-gates.md](/home/yukatayu/dev/mir_poc_01/plan/17-research-phases-and-autonomy-gates.md)、[90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)、[progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)、[tasks.md](/home/yukatayu/dev/mir_poc_01/tasks.md)、[phase5-small-decidable-core-and-proof-boundary.md](/home/yukatayu/dev/mir_poc_01/docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md) を 156 状態へ更新した。
4. local docs validation と diff check を通した後、reviewer を 1 回だけ起動し、mirror drift と report hygiene を補正した。

## 4. Files changed

- [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
- [00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
- [156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md](/home/yukatayu/dev/mir_poc_01/specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md)
- [11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)
- [12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)
- [13-heavy-future-workstreams.md](/home/yukatayu/dev/mir_poc_01/plan/13-heavy-future-workstreams.md)
- [17-research-phases-and-autonomy-gates.md](/home/yukatayu/dev/mir_poc_01/plan/17-research-phases-and-autonomy-gates.md)
- [90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
- [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)
- [tasks.md](/home/yukatayu/dev/mir_poc_01/tasks.md)
- [phase5-small-decidable-core-and-proof-boundary.md](/home/yukatayu/dev/mir_poc_01/docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md)
- [0428-phase5-exchange-body-threshold-package.md](/home/yukatayu/dev/mir_poc_01/docs/reports/0428-phase5-exchange-body-threshold-package.md)
- [0429-review-phase5-exchange-body-threshold-package.md](/home/yukatayu/dev/mir_poc_01/docs/reports/0429-review-phase5-exchange-body-threshold-package.md)

## 5. Commands run and exact outputs

```text
$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   92G  2.8G  98% /
```

```text
$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       829Mi        66Mi       1.1Mi       219Mi       130Mi
Swap:           19Gi       1.6Gi        18Gi
```

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-10 01:56 JST
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 429 numbered report(s).
```

```text
$ git diff --check
[no output]
```

```text
$ git status --short --branch
## main...origin/main
 M Documentation.md
 M docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md
 M plan/11-roadmap-near-term.md
 M plan/12-open-problems-and-risks.md
 M plan/13-heavy-future-workstreams.md
 M plan/17-research-phases-and-autonomy-gates.md
 M plan/90-source-traceability.md
 M progress.md
 M specs/00-document-map.md
 M tasks.md
?? docs/reports/0428-phase5-exchange-body-threshold-package.md
?? docs/reports/0429-review-phase5-exchange-body-threshold-package.md
?? specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md
```

## 6. Evidence / findings

- theorem-line retained bridge は、`consumer_invocation_surface_ref` の次段として `exchange_rule_body_ref` までは symbolic ref のまま narrow に吸収してよい。
- ただし concrete runtime coupling / transport / failure body はまだ theorem-line bridge に入れない方が自然である。
- したがって current next later reopen は `exchange-body threshold` ではなく **`runtime-coupling threshold`** に移った。
- reviewer は semantic defect なしと判断し、mirror drift と report hygiene のみ指摘した。修正内容は
  - `progress.md` の Phase 5 stale snapshot 補正
  - failure body defer の mirror wording 追加
  - `0428` の consulted / files changed / commands run / docs count 補正
  - `progress.md` の作業ログ追加
  である。

## 7. Changes in understanding

- theorem-line retained bridge は、`consumer_invocation_surface_ref` の次段として `exchange_rule_body_ref` までは symbolic ref のまま narrow に吸収してよい。
- concrete runtime coupling / transport / failure body を同時に足すのは premature であり、まだ later reopen に残す方が current docs-first discipline と整合する。
- next later reopen candidate は `exchange-body threshold` から `runtime-coupling threshold` に進んだ。

## 8. Open questions

- `runtime_coupling_ref` をどこまで theorem-line retained bridge に寄せてよいか。
- concrete file / blob transport と runtime-specific failure family を symbolic bridge とどう切り分けるか。
- `proof_assistant_adapter` pressure を second practical candidate のまま維持できる条件は何か。

## 9. Suggested next prompt

`runtime-coupling threshold` を docs-first で比較し、`exchange_rule_body_ref` の次段として `runtime_coupling_ref` をどこまで retained bridge に寄せてよいかを source-backed に整理してください。
