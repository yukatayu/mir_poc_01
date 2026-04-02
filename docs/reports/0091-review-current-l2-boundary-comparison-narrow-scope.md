# Report 0091 — review current l2 boundary comparison narrow scope

- Date: 2026-04-02T17:05:42.794129Z
- Author / agent: Codex
- Scope: `plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`docs/reports/0090-current-l2-trace-audit-vs-host-interface-boundary.md` の narrow-scope review
- Decision levels touched: review only

## 1. Objective

current L2 parser-free PoC 基盤の boundary comparison task について、次の 4 点だけを narrow scope で確認する。

1. detached trace / audit serialization を next bottleneck とし、richer host interface を後続 bottleneck と置く判断が既存 docs / code / report と矛盾しないか。
2. `must_explain` を machine-check に上げていない current policy を壊していないか。
3. host harness / bundle / batch / selection / profile / catalog の thin delegation boundary を越えて、新しい semantics や production host commitment を既成事実化していないか。
4. plan wording が未決事項を決まったことのように見せていないか。

Scope and assumptions:

- `specs/` を規範正本、`plan/` を repository memory、`docs/reports/` を経緯根拠として扱う。
- 今回は review のみであり、対象差分自体は編集しない。
- `plan/ 更新不要`

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/07-current-l2-host-stub-harness.md`
- `specs/examples/08-current-l2-host-plan-schema.md`
- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `specs/examples/11-current-l2-selection-helper.md`
- `specs/examples/12-current-l2-selection-profiles.md`
- `specs/examples/13-current-l2-profile-catalog.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0056-current-l2-host-stub-harness.md`
- `docs/reports/0059-current-l2-host-plan-sidecar-loader.md`
- `docs/reports/0060-current-l2-bundle-loader.md`
- `docs/reports/0062-current-l2-batch-runner.md`
- `docs/reports/0077-current-l2-helper-layer-responsibility-alignment.md`
- `docs/reports/0089-review-followup-0088-and-poc-blockers.md`
- `docs/reports/0090-current-l2-trace-audit-vs-host-interface-boundary.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`

## 3. Actions taken

1. AGENTS の必読順に従って `README.md`、`Documentation.md`、`specs/00`〜`03`、`specs/09` を先に読んだ。
2. current L2 parser-free PoC task の追加読書ルールに従って `plan/00-index.md`、`plan/07`、`plan/09` を確認した。
3. 対象 4 ファイルの diff と行番号を確認した。
4. `must_explain`、detached trace / audit serialization、host-plan coverage、production host boundary に関する既存 spec / report / code anchor を横断検索した。
5. `harness.rs` と `lib.rs` で、`ExpectedTraceAudit`、`TraceAuditSink`、`RunReport`、`BundleRunReport`、`BatchRunSummary`、`FixtureHostStub` の現在実装を照合した。
6. `docs/reports/0089` と `0090` を比較し、bottleneck ordering の整合を確認した。

## 4. Files changed

- 新規: `docs/reports/0091-review-current-l2-boundary-comparison-narrow-scope.md`

## 5. Commands run and exact outputs

```bash
git status --short
git diff -- plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/90-source-traceability.md docs/reports/0090-current-l2-trace-audit-vs-host-interface-boundary.md
rg -n "must_explain|TraceAuditSink|RunReport|host plan coverage|coverage failure|production host|verification basis|detached|artifact|uncovered call|preflight|coverage explanation" specs/examples plan docs/reports crates/mir-semantics/src crates/mir-semantics/tests
git log --oneline -n 8
python3 scripts/new_report.py --slug review-current-l2-boundary-comparison-narrow-scope
```

### Exact outputs

- `git status --short`

```text
 M plan/11-roadmap-near-term.md
 M plan/12-open-problems-and-risks.md
 M plan/90-source-traceability.md
?? diff_investigation_01.md
?? docs/reports/0090-current-l2-trace-audit-vs-host-interface-boundary.md
?? "旧資料_参考_ChatGPT_01_69c5e3f6/"
?? "旧資料_参考_ChatGPT_02_Mirrorea_2025/"
```

- `git log --oneline -n 8`

```text
6034878 0088 の review follow-up を記録する
82bdff3 representative examples prose の drift を整流する
5b34698 A2 rendering の rollout 境界を整理する
1b421ec plan 外在化 report の検証結果を補記する
6a8044d plan 外在化の作業報告を追加する
8003e26 plan ディレクトリと維持ルールを追加する
1ca8d67 explicit edge-row notation polishing の作業報告を追加する
1a97f08 explicit edge-row notation の polishing を整理する
```

- `python3 scripts/new_report.py --slug review-current-l2-boundary-comparison-narrow-scope`

```text
/home/yukatayu/dev/mir_poc_01/docs/reports/0091-review-current-l2-boundary-comparison-narrow-scope.md
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
```

Tests:

- review task のため未実行

## 6. Evidence / findings

### Finding 1

`plan/11-roadmap-near-term.md:81-83`、`plan/11-roadmap-near-term.md:89-93`、`plan/12-open-problems-and-risks.md:18-19`、`plan/12-open-problems-and-risks.md:56-64`、`docs/reports/0090-current-l2-trace-audit-vs-host-interface-boundary.md:217-224` は、detached trace / audit serialization を「次の主 bottleneck」、richer host interface を「その次」と読める強さで書いている。  
しかし既存の `docs/reports/0089-review-followup-0088-and-poc-blockers.md:130-140` では、current L2 の PoC blocker 順として `fixture authoring / elaboration` が先に置かれ、その後に detached trace / audit serialization、richer host interface が続いている。`docs/reports/0089-review-followup-0088-and-poc-blockers.md:153-159` でも、主 bottleneck 群は serialization / host integration へ移ったと要約している一方で、authoring を明示的に取り下げてはいない。今回の更新差分には、その prior ordering を「この比較では scope 外に置く」のか「実際に優先度が下がった」のかを説明する文が無いため、未決の優先順位が repo 全体で確定したように見える。

修正するなら次のどちらかが必要である。

- claim を「detached vs richer host interface の比較に限れば detached を先に置く」へ弱める
- `fixture authoring / elaboration` がなぜ今回 near-term bottleneck の先頭から外れるのかを明記する

### Checked with no finding

- `must_explain` policy は、`plan/11-roadmap-near-term.md:25`、`plan/11-roadmap-near-term.md:37`、`plan/12-open-problems-and-risks.md:57`、`docs/reports/0090-current-l2-trace-audit-vs-host-interface-boundary.md:253-268` で human-facing obligation のまま維持されており、`specs/examples/07`〜`13` と `specs/12-decision-register.md:51-56` にある current policy を壊していない。
- thin delegation boundary は、`plan/11-roadmap-near-term.md:29-31`、`plan/12-open-problems-and-risks.md:19`、`plan/12-open-problems-and-risks.md:61-64`、`docs/reports/0090-current-l2-trace-audit-vs-host-interface-boundary.md:272-279` がいずれも current host harness を verification basis に留める前提を明示しており、`specs/examples/07`〜`13` と `crates/mir-semantics/src/harness.rs:16-31`、`crates/mir-semantics/src/harness.rs:793-851`、`crates/mir-semantics/src/harness.rs:1019-1029` にある helper boundary を越えて新しい semantics や production host commitment を既成事実化している箇所は見当たらなかった。
- `plan/90-source-traceability.md` の追記自体は、今回の変更理由として `0089` と `0090` を source chain に加える内容であり、この narrow scope では独立した矛盾は見当たらなかった。

## 7. Changes in understanding

- `must_explain` と thin delegation boundary については、今回の差分は既存 current L2 policy と概ね整合していた。
- 実質的な問題は boundary comparison の中身ではなく、その比較結果を `plan/11` と `plan/12` で repo 全体の near-term bottleneck ordering に見える強さで書いている点だった。
- したがって、必要なのは detached vs host の比較結果を取り消すことではなく、`0089` からの優先順位遷移を scope 付きで書き直すことである。

## 8. Open questions

- `fixture authoring / elaboration` は near-term roadmap 上で独立 bottleneck として残すのか。
- もし detached trace / audit serialization を先頭に上げるなら、その理由を `0089` との差分としてどこまで明示するか。
- `plan/11` と `plan/12` では、comparison result と repo-level sequencing judgment をどの程度はっきり区別して書くべきか。

## 9. Suggested next prompt

`plan/11-roadmap-near-term.md` と `plan/12-open-problems-and-risks.md` について、docs/reports/0089-review-followup-0088-and-poc-blockers.md にある fixture authoring / elaboration の prior ranking と矛盾しないよう、detached trace / audit serialization と richer host interface の優先順位表現を scope 付きで弱めるか、優先順位変更理由を明記する narrow-scope 修正をしてください。`
