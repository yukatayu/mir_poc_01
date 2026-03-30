# 報告 0011 — `all_of` aggregate evidence の最小要件整理

- 作成日時: 2026-03-27T09:51:55.873263Z
- 作成者 / agent: Codex
- 対象範囲: Mir-0 を広げずに、cross-place `durable_cut` の `all_of` profile における aggregate evidence の最小要件と、event / audit / Mirrorea 境界を整理する
- 触れた decision level: L1

## 1. 目的

cross-place `durable_cut` の `all_of` profile について、aggregate success を正当化する最小 evidence 条件を定める。
今回は新しい大きな抽象を増やさず、participating `place` ごとの local observation / persistence evidence を Mir-1 がどこまで意味語彙として要求し、どこからを Mirrorea の表現裁量に送るかだけを明確にする。

## 2. スコープと前提

- 対象は `all_of` profile における aggregate evidence だけであり、Mir-0 の再定義は行わない。
- `quorum-like`、`barrier`、coroutine、overlay、fallback 正規化には広げない。
- 既存の `durable_cut` guarantee / failure / observation / cross-place scope / scope rule family の整理は維持する。
- local と aggregate の関係は最小限だけ明示し、ack / signature / proof / log record などの具象形式は固定しない。
- `specs/04-mir-core.md` はこれ以上膨らませず、Mir-1 側の更新は `specs/10-open-questions.md` と `specs/12-decision-register.md` に留める。

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
- `docs/reports/0010-durable-cut-scope-rule-family-validation.md`

## 4. 実施内容

- `using-superpowers` と `brainstorming` の手順を確認したうえで、`code_mapper` を先に起動し、今回主に触るべき箇所が `specs/10-open-questions.md`、`specs/12-decision-register.md`、最新 report 群に収まることを確認した。
- 指定順で文書を読み、`all_of` が current minimum profile として固定済みであり、未決点が aggregate evidence の最小要件に絞られていることを確認した。
- `specs/10-open-questions.md` を更新し、`all_of` の full coverage 条件、event surface に残す最小情報、audit surface に残す per-place trace、full coverage 不成立と aggregate failure の関係を明文化した。
- 同じファイルの Mirrorea 節で、Mir-1 が固定するのは full coverage 条件だけであり、per-place evidence reference の表現・圧縮・共有形式は Mirrorea の裁量に残ると明記した。
- `specs/12-decision-register.md` に D-019 を追加し、`all_of` aggregate evidence を最小 coverage 条件に絞る判断を L1 として記録した。
- 本レポートを新規追加した。
- `reviewer` は最後に起動し、Mir-1 を肥大化させていないか、aggregate evidence が実装依存に寄りすぎていないかを確認する。

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
- `docs/reports/0009-durable-cut-scope-rule-family.md`
- `docs/reports/0010-durable-cut-scope-rule-family-validation.md`

今回実際に変更したファイル:

- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0011-durable-cut-all-of-aggregate-evidence.md`

## 6. 根拠・出力・テスト結果

1. `python3 scripts/new_report.py --slug durable-cut-all-of-aggregate-evidence`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0011-durable-cut-all-of-aggregate-evidence.md
```

2. `code_mapper`

```text
触るべきファイルは `specs/10-open-questions.md`、`specs/12-decision-register.md`、`docs/reports/0008`、`0009`、`0010` であり、今回の論点だけなら `specs/04-mir-core.md` と `specs/05-mirrorea-fabric.md` は参照用で十分だと確認した。
```

3. `git status --short`

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
?? docs/reports/0011-durable-cut-all-of-aggregate-evidence.md
```

4. `git diff --check`

```text
```

5. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 12 numbered report(s).
```

6. `reviewer`

```text
初回 review では、report の検証欄が `<pending>` のままだという一点だけが指摘された。
再 review では、reviewer 欄に最終 verdict が記録されていないという一点だけが指摘された。
現時点までに、specs 本文そのものへの追加 finding は報告されていない。
最終 review: no findings
```

根拠と所見:

- `all_of` では、aggregate success を言うために必要なのは participating `place` 全員に対する full coverage だけであり、per-place evidence payload 自体を event surface に持ち込む必要はない。
- そのため Mir-1 が固定すべき最小要件は、「各 `place` に対応する success 側 local observation と supporting persistence evidence が outcome 判定に数えられていること」であり、ack / signature / proof / log record などの具象形式は Mirrorea に残すのが最も狭い。
- full coverage の不成立は aggregate success の不成立を意味するが、それ自体を即 failure と読むと、既存の「failure は explicit outcome surface で観測する」という整理と衝突する。そのため failure は従来どおり aggregate failure event でだけ確定させる方が整合的である。
- audit surface は per-place 対応関係と counted / missing の説明責務を持ち、event surface は aggregate attempt と aggregate outcome に絞るのが、Mir-1 を軽く保つうえで自然である。

## 7. 今回整理して変わった理解

- これまで未決だったのは「どんな evidence を使うか」よりも、「`all_of` success を意味上どこまで説明しなければならないか」だった。
- `all_of` の aggregate evidence を full coverage 条件に絞ると、cross-place `durable_cut` は single-place observation の単純な conjunction として説明でき、`quorum-like` に備えた追加語彙を今持ち込まずに済む。
- event surface と audit surface の役割を分けることで、Mir-1 は meaning、Mirrorea は representation という境界が以前より明瞭になった。

## 8. 未決定事項

- evidence の具象形式を ack / signature / proof / log record のどれで表すかは **未決定**。
- `all_of` の per-place evidence 参照を event surface にどこまで露出するかは **未決定**。
- local evidence と distributed outcome の関係をどこまで Mir-1 で語るかは **未決定**。
- 将来 profile family が増えたときに aggregate evidence の意味差分をどこまで先に固定するかは **未決定**。
- Mirrorea が evidence 表現で必要とする最小 protocol surface は **未決定**。

## 9. Suggested next prompt

Mir-0 を広げずに、`all_of` profile における aggregate failure justification を整理してください。特に full coverage が成立しなかったとき、どの時点で aggregate failure event を正当化できるのか、何を Mir-1 の outcome surface に残し、何を Mirrorea の operational policy に送るべきかを、日本語で `specs/10-open-questions.md` と必要最小限の関連文書に反映してください。
