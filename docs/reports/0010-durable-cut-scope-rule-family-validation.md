# 報告 0010 — `durable_cut` の scope rule family 検証と表現整理

- 作成日時: 2026-03-27T09:24:41.669884Z
- 作成者 / agent: Codex
- 対象範囲: 既存の cross-place `durable_cut` scope rule family 整理を repo 正本として再確認し、Mir-1 / Mirrorea 境界の表現を最小限だけ明確化する
- 触れた decision level: L1

## 1. 目的

cross-place `durable_cut` の scope rule family について、repo 上にすでに入っていた判断が今回の依頼に対して十分かを検証し、必要なら表現だけを最小限修正する。
今回は profile family を作り直すのではなく、Mir-1 に残す profile を `all_of` だけに絞る既存方針を確認し、Mirrorea 側の裁量境界を新しい未定義語に依存せず書き直すことを目的にした。

## 2. スコープと前提

- Mir-0 は再定義しない。
- `attempt`、`prefix`、aggregate success / failure event の意味は `docs/reports/0008-durable-cut-cross-place-scope-rule.md` までの整理を維持する。
- `docs/reports/0009-durable-cut-scope-rule-family.md` はこのターン開始時点で repo 上に存在する prior report として扱い、上書きしない。
- 今回の規範変更は、`implementation-defined` を Mir-1 profile 語彙にしないという既存判断を保ったまま、Mirrorea の裁量範囲を「同じ aggregate success / failure 観測を保つ実現」に言い換えることに限定する。
- `barrier`、coroutine、overlay、fallback 正規化、具体的な合意アルゴリズムには広げない。

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
- `docs/reports/0009-durable-cut-scope-rule-family.md`

## 4. 実施内容

- `using-superpowers` と `brainstorming` の手順を確認したうえで、`code_mapper` を先に起動し、今回主に確認すべき箇所が `specs/10-open-questions.md`、`specs/12-decision-register.md`、最新 report 群に収まることを確認した。
- 指定順で文書を読み、repo 上ではすでに `all_of` を current minimum profile とする整理が入り、`quorum-like` は将来拡張候補、`implementation-defined` は Mir-1 profile 語彙にしない方針が記述済みであることを確認した。
- 既存の `specs/10-open-questions.md` と `specs/12-decision-register.md` のうち、`observationally equivalent` という新しい定義語に見えやすい表現だけを、同じ aggregate success / failure 観測を保つ実現という記述に置き換えた。
- 初回の `reviewer` で、report の証跡不足と `quorum-like` 周辺の補助語の増やしすぎが指摘されたため、report の出力欄を実値で埋め、spec 上の説明を「participating \`place\` の一部だけで十分とする条件」という最小表現まで削った。
- `specs/04-mir-core.md` は増やさず、scope rule family の規範追記は今回も `specs/10-open-questions.md` と `specs/12-decision-register.md` に留めた。
- 本レポートを新規追加した。
- `reviewer` は編集ごとに起動し、結果を §6 に記録した。

## 5. 既存 dirty state と今回実際に変更したファイル

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

途中で repo 上の prior report として確認し、未編集のまま扱ったファイル:

- `docs/reports/0009-durable-cut-scope-rule-family.md`

今回実際に変更したファイル:

- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0010-durable-cut-scope-rule-family-validation.md`

## 6. 根拠・出力・テスト結果

1. `python3 scripts/new_report.py --slug durable-cut-scope-rule-family-validation`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0010-durable-cut-scope-rule-family-validation.md
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
?? docs/reports/0010-durable-cut-scope-rule-family-validation.md
```

3. `code_mapper`

```text
Mir-1 の scope rule family は、`all_of` だけを current minimum profile として残す形に整理しました。
```

4. `git diff -- specs/10-open-questions.md specs/12-decision-register.md`

```text
`observationally equivalent` という表現を、Mirrorea は同じ aggregate success / failure 観測を保つ形で既知 profile を実現する、という日本語の明示記述に置き換えた。
```

5. `git diff --check`

```text
```

6. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 11 numbered report(s).
```

7. `reviewer`

```text
初回 review では、report の証跡不足と `quorum-like` 周辺の補助語の増やしすぎが指摘された。
再 review では、reviewer 証跡欄が未更新のままだという一点だけが残件として指摘された。
最終 review では、残件は reviewer 証跡欄の未記入だけだと指摘された。
この記録追記により、reviewer 証跡欄の未記入は解消した。
```

根拠と所見:

- repo 上の現行仕様は、scope rule family を増やしすぎないという今回の目的に対してすでに正しい方向を向いていた。
- `all_of` を current minimum profile に固定する判断自体は妥当であり、今回必要だったのは Mirrorea の実現裁量を undefined な専門語に頼らず説明し直すことだった。
- `quorum-like` を将来拡張候補のままに留めることで、Mir-1 に「participating `place` の一部だけで十分とする条件」を持ち込まずに済む。
- `implementation-defined` を profile 語彙にしないまま、Mirrorea には既知 profile の同一観測を保つ実現だけを許すと書けば、Mir-1 と Mirrorea の責務境界がより読みやすくなる。

## 7. 今回整理して変わった理解

- 0009 の判断で本質的な profile 境界はすでに固定されており、このターンで必要だったのは決定内容の再発明ではなく、用語の曖昧さを減らすことだった。
- `observationally equivalent` のような新しい定義語は、理論的には自然でも、この repo の現段階では追加の定義負債になりやすい。
- そのため「同じ aggregate success / failure 観測を保つ」という event / audit surface 由来の言い方に寄せた方が、既存仕様との接続がよい。

## 8. 未決定事項

- `quorum-like` を将来の Mir-1 拡張 profile として本当に採用するかは **未決定**。
- `all_of` に対する aggregate evidence の最小要件を Mir-1 でどこまで固定するかは **未決定**。
- local evidence と distributed outcome の関係を event / audit surface にどこまで露出するかは **未決定**。
- 将来 profile 拡張を導入する場合の相互運用条件を Mir-1 で語るか Mirrorea で吸収するかは **未決定**。
- Mirrorea が profile 実現時に必要とする最小 protocol surface は **未決定**。

## 9. Suggested next prompt

Mir-0 を広げずに、`all_of` profile における aggregate evidence の最小要件を整理してください。特に participating `place` 全員の local observations / persistence evidence を、Mir-1 の event / audit surface ではどこまで明示すべきか、どこからを Mirrorea の表現裁量に送るべきかを、日本語で `specs/10-open-questions.md` と必要最小限の関連文書に反映してください。
