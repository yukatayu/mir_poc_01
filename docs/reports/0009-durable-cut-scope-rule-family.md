# 報告 0009 — `durable_cut` の scope rule family 整理

- 作成日時: 2026-03-27T09:19:18.753370Z
- 作成者 / agent: Codex
- 対象範囲: Mir-0 を広げずに、cross-place `durable_cut` の scope rule family を最小化し、Mir-1 に必須で残す profile と Mirrorea に送る裁量境界を整理する
- 触れた decision level: L1

## 1. 目的

cross-place `durable_cut` の scope rule family について、Mir-1 がどこまで語るべきかを最小限に定める。
今回は profile の種類を増やすこと自体を目的にせず、single-place observation と既存の aggregate attempt / aggregate event 整理を壊さない範囲で、Mir-1 に必須で残す profile を一つに絞ることを優先した。

## 2. スコープと前提

- 対象は `durable_cut` の scope rule family だけであり、Mir-0 の再定義は行わない。
- `attempt`、`prefix`、aggregate success / failure event の意味は `docs/reports/0008` までの整理を維持する。
- `barrier`、coroutine、overlay、fallback 正規化、具体的な合意アルゴリズムには広げない。
- 可能な限り Mir-1 の理論を小さく保つ。
- 未決定の点は無理に確定せず、明示的に **未決定** と記す。

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
- `docs/reports/0008-durable-cut-cross-place-scope-rule.md`

## 4. 実施内容

- `code_mapper` を先に起動し、今回の task で主に触るべき箇所が `specs/10-open-questions.md`、`specs/12-decision-register.md`、新規 report に収まるかを確認した。最終判断は workspace 上の `git status --short` と指定文書の読解でも独立に確認した。
- 指定順で文書を読み、`durable_cut` の guarantee / failure / observation / cross-place scope がすでに狭く整理されていることを確認した。
- scope rule family を Mir-1 にどこまで残すべきかを比較し、`all_of` だけを current minimum profile とし、`quorum-like` は将来拡張候補、`implementation-defined` は Mir-1 語彙にしない方針に整理した。
- `specs/10-open-questions.md` を更新し、`all_of` の最小意味、`quorum-like` を現時点で採らない理由、Mirrorea が独自 profile を意味語彙として追加しないことを明文化した。
- `specs/12-decision-register.md` に D-018 を追加し、profile family の最小化判断を L1 として記録した。
- `specs/04-mir-core.md` はこれ以上膨らませない方針を維持し、今回は更新しなかった。
- `reviewer` の使用は指示されていたが、このターンの tool surface には subagent 呼び出し手段が露出していなかったため、同観点の自己レビューと検証コマンドで代替し、その制約を明示的に記録した。
- 本レポートを新規追加した。

作業開始時に確認した既存 dirty state:

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
- `docs/reports/0008-durable-cut-cross-place-scope-rule.md`

今回実際に変更したファイル:

- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0009-durable-cut-scope-rule-family.md`

## 5. コマンド実行と正確な出力

1. `python3 scripts/new_report.py --slug durable-cut-scope-rule-family`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0009-durable-cut-scope-rule-family.md
```

2. `git status --short`

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
?? docs/reports/0009-durable-cut-scope-rule-family.md
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
```

5. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 10 numbered report(s).
```

6. `reviewer`

```text
このターンの tool surface には reviewer subagent の呼び出し手段が露出していなかったため、実行できなかった。
```

## 6. 根拠 / 所見

- Mir-1 の profile family は、single-place observation をどの追加意味なしに aggregate success 条件へ持ち上げるか、という最小論点だけに留めるべきである。
- `all_of` は participating `place` 全員の required local observations / persistence evidence を conjunction として数えるだけなので、aggregate attempt / aggregate event に新しい threshold semantics や member-class semantics を導入しない。
- `quorum-like` を Mir-1 に含めると、eligible member set、threshold interpretation、監査時に何を十分条件とみなすか、という追加語彙が必要になる。そのため current minimum profile には入れない方が理論が軽い。
- `implementation-defined` を profile 語彙として Mir-1 に入れると、Mir-1 の success / failure meaning が実装依存のブラックボックスになりやすい。そのため Mirrorea の裁量は、Mir-1 が意味を与えた named profile を observationally equivalent に実現する範囲に制限するのが自然である。
- この基準により、Mir-1 は `all_of` を必須 profile として持つだけで足り、将来 `quorum-like` を追加する場合も拡張点を明示的に増やす形にできる。

規範変更メモ:

- 今回は Mir-1 の current minimum scope rule profile を `all_of` のみに固定した。
- `quorum-like` は将来拡張候補として残すが、現時点の Mir-1 には含めないとした。
- `implementation-defined` は Mir-1 の profile 語彙にせず、Mirrorea の実現裁量は named profile の observationally equivalent realization に限定すると整理した。

## 7. 今回整理して変わった理解

- これまでの open question は「何種類の profile を許すか」に見えていたが、実際には「Mir-1 が追加でどの意味語彙まで背負うべきか」が本質だった。
- `all_of` だけを current minimum profile にすると、cross-place `durable_cut` は single-place observation の束ねとして説明でき、既存の aggregate attempt / aggregate event の意味を重くしない。
- `quorum-like` は単なる profile バリエーションではなく、member eligibility、threshold semantics、audit/accountability を伴う別段階の拡張だと整理できた。
- Mirrorea に profile 自体の意味づけを渡さず、「既知 profile の実現裁量」だけを渡す方が、Mir-1 / Mirrorea 境界をきれいに保てる。

## 8. 未決定事項

- `quorum-like` を将来の Mir-1 拡張 profile として本当に採用するかは **未決定**。
- `all_of` に対する aggregate evidence の最小要件を、Mir-1 でどこまで固定するかは **未決定**。
- local evidence と distributed outcome の関係を event / audit surface にどこまで露出するかは **未決定**。
- 将来 profile 拡張を導入する場合の capability negotiation を Mir-1 で語るか Mirrorea で吸収するかは **未決定**。
- Mirrorea が profile 実現時に必要とする最小 protocol surface は **未決定**。

## 9. Suggested next prompt

Mir-0 を広げずに、`all_of` profile における aggregate evidence の最小要件を整理してください。特に participating `place` 全員の local observations / persistence evidence を、Mir-1 の event / audit surface ではどこまで明示すべきか、どこからを Mirrorea の representation 裁量に送るべきかを、日本語で `specs/10-open-questions.md` と必要最小限の関連文書に反映してください。
