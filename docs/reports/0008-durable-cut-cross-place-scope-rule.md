# 報告 0008 — `durable_cut` の cross-place scope rule 整理

- 作成日時: 2026-03-27T08:45:28.462320Z
- 作成者 / agent: Codex
- 対象範囲: Mir-0 を広げずに、cross-place `durable_cut` が single-place な successful / failed observation を壊さないように、`attempt` / `prefix` / aggregate event の最小 scope rule だけを整理する
- 触れた decision level: L1

## 1. 目的

複数 `place` にまたがる `durable_cut` を、event / audit surface 上でどう束ねるかを最小限の意味論として定める。
今回は guarantee / failure / observation の既存整理を壊さず、cross-place な束ね方だけを Mir-1 に残し、それ以上の protocol や合意アルゴリズムは Mirrorea 側へ送る。

## 2. スコープと前提

- 対象は cross-place `durable_cut` の scope rule だけであり、Mir-0 の再定義は行わない。
- `barrier`、coroutine、overlay、fallback 正規化には広げない。
- 既存の failure lattice は維持し、新しい failure class は追加しない。
- local / distributed の違いは、event / audit surface に必要最小限だけ反映し、実装や protocol の詳細には踏み込まない。
- `specs/04-mir-core.md` はこれ以上膨らませず、Mir-1 側の整理は `specs/10-open-questions.md` と `specs/12-decision-register.md` に留める。

## 3. 参照した文書

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `docs/reports/0005-durable-cut-minimum-guarantee-set.md`
- `docs/reports/0006-durable-cut-failure-surface.md`
- `docs/reports/0007-durable-cut-observation-surface.md`

## 4. 実施内容

- `code_mapper` を先に起動し、対象ファイルと既存 dirty state の確認を依頼した。最終判断は workspace 上の `git status --short` と指定文書の読解でも独立に確認した。
- 指定順で文書を読み、single-place な `durable_cut` の guarantee / failure / observation 条件と、Mirrorea 側へ送られている責務境界を再確認した。
- `specs/10-open-questions.md` を更新し、cross-place 文脈では各 `place` の local cut attempt を束ねる aggregate attempt が観測単位であること、cross-place の `prefix` は local prefixes の有限対応として扱うこと、single-place event を壊さずに aggregate success / failure event を追加することを明文化した。
- 同じ箇所で、cross-place successful / failed `durable_cut` は scope rule に従う aggregate success / failure event で観測されると整理し、scope rule の具体 family は未決定のまま残した。
- `specs/12-decision-register.md` に D-017 を追加し、cross-place scope rule の最小判断を L1 として記録した。
- `reviewer` に最終確認を依頼し、Mir-0 を広げていないこと、Mirrorea の protocol 詳細に踏み込みすぎていないことについて `no findings` を得た。
- 本レポートを新規追加した。
- 作業開始時に確認した既存 dirty state:
- `agents/implementer.toml`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0001-mir-0-semantic-core-alignment.md`
- `docs/reports/0002-mir-0-review-findings.md`
- `docs/reports/0003-mir-0-cut-and-vocabulary-clarification.md`
- `docs/reports/0004-cut-vocabulary-responsibility-split.md`
- `docs/reports/0005-durable-cut-minimum-guarantee-set.md`
- `docs/reports/0006-durable-cut-failure-surface.md`
- `docs/reports/0007-durable-cut-observation-surface.md`
- 今回実際に変更したファイル:
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0008-durable-cut-cross-place-scope-rule.md`

## 5. 根拠・出力・テスト結果

1. `git status --short`

```text
 M agents/implementer.toml
 M specs/04-mir-core.md
 M specs/09-invariants-and-constraints.md
 M specs/10-open-questions.md
 M specs/12-decision-register.md
?? docs/reports/0001-mir-0-semantic-core-alignment.md
?? docs/reports/0002-mir-0-review-findings.md
?? docs/reports/0003-mir-0-cut-and-vocabulary-clarification.md
?? docs/reports/0004-cut-vocabulary-responsibility-split.md
?? docs/reports/0005-durable-cut-minimum-guarantee-set.md
?? docs/reports/0006-durable-cut-failure-surface.md
?? docs/reports/0007-durable-cut-observation-surface.md
?? docs/reports/0008-durable-cut-cross-place-scope-rule.md
```

2. `python3 scripts/new_report.py --slug durable-cut-cross-place-scope-rule`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0008-durable-cut-cross-place-scope-rule.md
```

3. `git diff -- agents/implementer.toml`

```text
diff --git a/agents/implementer.toml b/agents/implementer.toml
index d736870..e28acff 100644
--- a/agents/implementer.toml
+++ b/agents/implementer.toml
@@ -1,7 +1,7 @@
 name = "implementer"
 description = "Single-writer implementation agent for non-test source changes."
 model = "gpt-5.4"
-model_reasoning_effort = "high"
+model_reasoning_effort = "xhigh"
 sandbox_mode = "workspace-write"
 
 developer_instructions = """
```

4. `git diff --check`

```text
<no output>
```

5. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 9 numbered report(s).
```

6. `reviewer`

```text
no findings
```

根拠と所見:

- cross-place `durable_cut` を single-place event だけで観測させると、どの local attempts がひとつの cross-place cut を構成していたのかが意味上不明になる。そのため aggregate attempt が必要である。
- ただし global prefix を単一の merged prefix として導入すると、既存の `place` ごとの event DAG を不必要に潰す。そのため cross-place の `prefix` は local prefixes の有限対応として扱うのが最も狭い。
- single-place な success / failure event はそのまま member-local observation として保持し、cross-place `durable_cut` 自体は aggregate success / failure event で観測する構成にすると、既存整理と衝突しない。
- aggregate event は Mir-1 の意味語彙として必要だが、member observation の収集・相関や aggregate event の実現方法は Mirrorea 側に残せる。
- scope rule の具体 family を all-of, quorum-like, implementation-defined のどこまで許すかは、現段階では固定しない方が境界を保てる。

規範変更メモ:

- 今回は cross-place `durable_cut` の観測単位を aggregate attempt とした。
- cross-place の `prefix` は merged global prefix ではなく local prefixes の有限対応として扱うと整理した。
- cross-place `durable_cut` 自体の観測には aggregate success / failure event が必要であり、これは Mir-1 の意味語彙に属するとした。
- aggregate event の実現方法、scope rule の具体 family、evidence の具象形式は Mir-1 では固定しないとした。

## 6. 今回整理して変わった理解

- これまで未決だったのは、cross-place `durable_cut` に一つの global prefix を作るべきかどうかよりも、single-place observation を壊さずに何を aggregate 単位として追加するべきかだった。
- `attempt` を local / aggregate に分け、`prefix` を cross-place 文脈では local prefixes の有限対応として読むと、既存の event DAG と観測面をそのまま保てる。
- cross-place `durable_cut` の success / failure は aggregate event でしか観測できないが、その実現方法まで Mir-1 に持ち込む必要はない。
- その結果、Mir-1 は束ねの意味だけを持ち、Mirrorea は収集・相関・aggregate representation の実現だけを担う、という分離が以前より明確になった。

## 7. 未決定事項

- scope rule を all-of に制限するか、quorum-like profile を許すか、さらに広い implementation-defined profile を許すかは **未決定**。
- local evidence と aggregate outcome の関係を、event / audit surface にどこまで露出するかは **未決定**。
- `persistence evidence` の具象形式をログ、署名、ack、proof のどれで表すかは **未決定**。
- `Approximate` を許す contract 条件と cross-place scope rule の関係は **未決定**。
- Mirrorea が必要とする最小 protocol surface は **未決定**。

## 8. Suggested next prompt

Mir-0 を広げずに、cross-place `durable_cut` の scope rule family を整理してください。特に all-of、quorum-like、implementation-defined profile のどこまでを Mir-1 で許し、どこからを Mirrorea の実現裁量に送るべきかを、日本語で `specs/10-open-questions.md` と必要最小限の関連文書に反映してください。
