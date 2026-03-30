# 報告 0007 — `durable_cut` の observation surface 整理

- 作成日時: 2026-03-27T08:12:17.586220Z
- 作成者 / agent: Codex
- 対象範囲: Mir-0 を広げずに、`durable_cut` の successful / failed 観測と `persistence evidence` の最小要求を event / audit surface で整理する
- 触れた decision level: L1

## 1. 目的

`durable_cut` の成功・失敗がどの event / audit surface で観測されるのかを、最小限の意味論として定める。
今回は新しい大きな抽象を増やさず、Mir-1 が意味として要求する最小情報と、Mirrorea に残す具象実装の自由度を切り分けることだけを目的にした。

## 2. スコープと前提

- 対象は `durable_cut` の observation surface と `persistence evidence` の最小要求だけであり、Mir-0 の再定義は行わない。
- `barrier`、coroutine、overlay、fallback 正規化には広げない。
- 既存の failure lattice は維持し、今回の整理は successful / failed 観測条件の明文化に限定する。
- `specs/05-mirrorea-fabric.md` は Mirrorea 境界の確認のためだけに参照し、実装詳細には踏み込まない。
- 具象的なログ形式、署名形式、ack 形式、特定の storage protocol は今回固定しない。

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
- `docs/reports/0004-cut-vocabulary-responsibility-split.md`
- `docs/reports/0005-durable-cut-minimum-guarantee-set.md`
- `docs/reports/0006-durable-cut-failure-surface.md`

## 4. 実施内容

- `code_mapper` を先に起動し、対象ファイルと既存 dirty state の確認を依頼した。今回の最終判断は、workspace 上の `git status --short` と指定文書の読解でも独立に確認した。
- 指定順で文書を読み、`durable_cut` が Mir-1 側の語彙であり、failure lattice と Mirrorea 境界はすでに前段で狭く固定されていることを再確認した。
- `specs/10-open-questions.md` を更新し、successful / failed `durable_cut` を event surface でどう観測するか、audit surface に最低何を残すか、`persistence evidence` の最小要求を何に留めるかを明文化した。
- 同じ箇所で、failure observation point は durable side の内側ではなく cut attempt の outcome surface にあることを明示し、local / distributed の違いは実現上の差分として残した。
- `specs/12-decision-register.md` に D-016 を追加し、今回の観測面の判断を L1 として記録した。
- 本レポートを新規追加した。

## 5. 既存 dirty state と今回実際に変更したファイル

作業開始時に確認した既存 dirty state:

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

作業途中で追加検出した今回未編集の dirty state:

- `agents/implementer.toml`

今回実際に変更したファイル:

- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0007-durable-cut-observation-surface.md`

## 6. 根拠・出力・テスト結果

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
```

2. `python3 scripts/new_report.py --slug durable-cut-observation-surface`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0007-durable-cut-observation-surface.md
```

3. `rg -n "event|audit|persistence evidence|failure observation|durable_cut|path proof|trace|observ" specs docs/reports`

```text
`durable_cut` については `specs/04-mir-core.md`, `specs/10-open-questions.md`, `specs/12-decision-register.md`, `docs/reports/0005`, `docs/reports/0006` に集中しており、event / audit surface の最小意味はまだ明示的に固定されていなかった。
```

4. `git diff -- agents/implementer.toml`

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

5. `git diff --check`

```text
```

6. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 8 numbered report(s).
```

- successful `durable_cut` は、ある cut attempt / pre-cut prefix が persistence evidence に支持されたことを示す明示的 event が立ったときにだけ観測されたものとみなす、という整理が最も狭い。
- failed `durable_cut` は、同じ attempt に対して successful event が立つ前に failure outcome が明示されたときに観測される、と定めれば、failure observation point を cut の内側へ持ち込まずに済む。
- audit surface に最低限必要なのは、cut attempt、対象 prefix、success / failure outcome、success を支えた evidence の観測有無を説明できる trace までである。
- `persistence evidence` は Mir-1 では抽象的証拠として扱い、どの cut attempt / prefix を支持するか、success 観測を正当化するか、という最小情報だけを要求するのが自然である。
- ログ、署名、ack、proof などの具象形式をここで固定すると Mirrorea の実現責務へ踏み込みすぎるため、今回は固定しない。
- local / distributed observation の違いは、現段階では event / audit surface 上の別語彙にせず、Mirrorea 側の実現上の差分として残す方が境界を保てる。

規範変更メモ:

- 今回は `durable_cut` の successful / failed 観測条件を event surface 上で最小限定義した。
- audit surface が保持すべき最小 trace 要件を、attempt / prefix / outcome / evidence observation までに限定した。
- `persistence evidence` の具象形式は Mir-1 では固定せず、Mirrorea の実現責務に残した。

## 7. 今回整理して変わった理解

- これまで未決だったのは `persistence evidence` そのものよりも、それをどの surface で「観測した」とみなすかだった。
- successful / failed `durable_cut` を event surface で明示的に分けると、failure lattice の整理とも衝突せず、Mir-1 の意味づけだけで閉じられる。
- audit surface は event surface の重複ではなく、その観測をあとから説明できる trace を残す面として切り分けるのが自然である。
- その結果、Mir-1 は観測条件だけを持ち、Mirrorea は evidence / audit の具象表現だけを引き受ける、という分離が以前より明確になった。

## 8. 未決定事項

- 複数 `place` にまたがる `durable_cut` の scope rule は **未決定**。
- local / distributed observation の差分を、将来 event / audit surface にどこまで露出するかは **未決定**。
- `persistence evidence` の具象形式をログ、署名、ack、proof のどれで表すかは **未決定**。
- `Approximate` を許す contract 条件と、この observation surface の関係は **未決定**。
- Mirrorea が必要とする最小 protocol surface は **未決定**。

## 9. Suggested next prompt

Mir-0 を広げずに、複数 `place` にまたがる `durable_cut` の scope rule を整理してください。特に single-place な successful / failed 観測条件を壊さずに、cross-place な `durable_cut` を event / audit surface でどう束ねるかを、日本語で `specs/10-open-questions.md` と必要最小限の関連文書に反映してください。
