# 報告 0016 — 仕様文書群の日本語正本化と semantic drift 検証

- 作成日時: 2026-03-30T20:09:41+09:00
- 作成者 / agent: Codex
- 対象範囲: `README.md`、`Documentation.md`、`specs/00..12` の日本語正本化と、翻訳前後で意味が変わっていないことの検証
- 触れた decision level: L0 / L1 / L2 / L3 の既存記述を保存対象として扱い、新規判断は追加しない

## 1. 目的

仕様文書群を日本語で一貫した人間向け正本へ整流し、semantic drift が起きていないことを確認する。
今回は翻訳と表現統一だけを行い、新しい理論、境界変更、cut vocabulary の追加判断は行わない。

## 2. 範囲と前提

- `AGENTS.md` の指示に従い、`README.md`、`Documentation.md`、`specs/00..12` を指定順で読んだ。
- `specs/10-open-questions.md` と `specs/12-decision-register.md` は作業開始時点ですでに dirty state だったため、`HEAD` ではなく作業開始時の workspace 内容を正本として扱った。
- formal token、identifier、decision ID、L0〜L3 ラベル、failure lattice の名前、`atomic_cut` / `durable_cut` / `place` などの形式語彙は保持し、説明文だけを日本語化した。
- 空の report template がこのセッション中に `0017` と `0018` として生成されたため、最終成果物を 1 本に保つため削除した。
- この report は 0016 時点の翻訳 pass を記録する。後続の用語方針追加と cross-reference 整流は `0017` の task として別扱いにする。

## 3. 参照文書

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/06-prismcascade-positioning.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
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
- `docs/reports/0011-durable-cut-all-of-aggregate-evidence.md`
- `docs/reports/0012-durable-cut-all-of-aggregate-failure-justification.md`
- `docs/reports/0013-durable-cut-all-of-aggregate-failure-audit-trace.md`
- `docs/reports/0014-durable-cut-all-of-missing-coverage-snapshot.md`
- `docs/reports/0015-durable-cut-all-of-coverage-state-positioning.md`

## 4. 実施内容

- `code_mapper` を最初に起動した。初回待機では結果を回収できなかったが、後続で read-only のコードマップが返り、対象ファイル群、既存 dirty state、drift リスクが高い節の切り分けを裏付けた。
- `README.md`、`Documentation.md`、`specs/00..12` の人間向け説明を日本語へ統一した。
- 見出し順、箇条書き順、番号順、表構造、decision ID を保ちながら、英語の説明文だけを日本語へ置換した。
- `specs/07-typed-effects-wiring-platform.md` は一度 prose と箇条書きの構造差が出たため、機械比較の結果に基づいて元の構造へ戻した。
- `specs/10-open-questions.md` と `specs/12-decision-register.md` は作業開始時の workspace 内容と現在の内容を手照合し、番号、D-ID、未決定事項の配列、説明順を保った。
- 空の report template `0017` / `0018` はこの作業の途中生成物だったため削除し、`0016` に集約した。
- 最後に `reviewer` を実行し、翻訳に伴う意味変化や構造破壊がないかを確認した。

作業開始時に確認した既存 dirty state:

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
- `docs/reports/0011-durable-cut-all-of-aggregate-evidence.md`
- `docs/reports/0012-durable-cut-all-of-aggregate-failure-justification.md`
- `docs/reports/0013-durable-cut-all-of-aggregate-failure-audit-trace.md`
- `docs/reports/0014-durable-cut-all-of-missing-coverage-snapshot.md`
- `docs/reports/0015-durable-cut-all-of-coverage-state-positioning.md`

今回実際に変更したファイル:

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/06-prismcascade-positioning.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `docs/reports/0016-japanese-canonicalization-and-semantic-drift-check.md`

確認したが今回変更しなかったファイル:

- `AGENTS.md`
- `docs/reports/0005-durable-cut-minimum-guarantee-set.md`
- `docs/reports/0006-durable-cut-failure-surface.md`
- `docs/reports/0007-durable-cut-observation-surface.md`
- `docs/reports/0008-durable-cut-cross-place-scope-rule.md`
- `docs/reports/0009-durable-cut-scope-rule-family.md`
- `docs/reports/0010-durable-cut-scope-rule-family-validation.md`
- `docs/reports/0011-durable-cut-all-of-aggregate-evidence.md`
- `docs/reports/0012-durable-cut-all-of-aggregate-failure-justification.md`
- `docs/reports/0013-durable-cut-all-of-aggregate-failure-audit-trace.md`
- `docs/reports/0014-durable-cut-all-of-missing-coverage-snapshot.md`
- `docs/reports/0015-durable-cut-all-of-coverage-state-positioning.md`

## 5. 証拠 / 出力 / 検証結果

実行した主なコマンドと出力:

1. `python3 scripts/new_report.py --slug japanese-canonicalization-and-semantic-drift-check`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0016-japanese-canonicalization-and-semantic-drift-check.md
```

2. 作業開始時の `git status --short`

```text
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
?? docs/reports/0012-durable-cut-all-of-aggregate-failure-justification.md
?? docs/reports/0013-durable-cut-all-of-aggregate-failure-audit-trace.md
?? docs/reports/0014-durable-cut-all-of-missing-coverage-snapshot.md
?? docs/reports/0015-durable-cut-all-of-coverage-state-positioning.md
```

3. fresh 構造比較

```text
README.md: OK {'h1': 1, 'h2': 7, 'h3': 0, 'bullet': 13, 'num': 12, 'table': 0, 'fence': 0} -> {'h1': 1, 'h2': 7, 'h3': 0, 'bullet': 13, 'num': 12, 'table': 0, 'fence': 0}
Documentation.md: OK {'h1': 1, 'h2': 7, 'h3': 0, 'bullet': 11, 'num': 4, 'table': 0, 'fence': 4} -> {'h1': 1, 'h2': 7, 'h3': 0, 'bullet': 11, 'num': 4, 'table': 0, 'fence': 4}
specs/00-document-map.md: OK {'h1': 1, 'h2': 3, 'h3': 0, 'bullet': 14, 'num': 8, 'table': 0, 'fence': 0} -> {'h1': 1, 'h2': 3, 'h3': 0, 'bullet': 14, 'num': 8, 'table': 0, 'fence': 0}
specs/01-charter-and-decision-levels.md: OK {'h1': 1, 'h2': 4, 'h3': 4, 'bullet': 19, 'num': 5, 'table': 0, 'fence': 0} -> {'h1': 1, 'h2': 4, 'h3': 4, 'bullet': 19, 'num': 5, 'table': 0, 'fence': 0}
specs/02-system-overview.md: OK {'h1': 1, 'h2': 5, 'h3': 0, 'bullet': 18, 'num': 4, 'table': 0, 'fence': 0} -> {'h1': 1, 'h2': 5, 'h3': 0, 'bullet': 18, 'num': 4, 'table': 0, 'fence': 0}
specs/03-layer-model.md: OK {'h1': 1, 'h2': 3, 'h3': 6, 'bullet': 33, 'num': 0, 'table': 0, 'fence': 0} -> {'h1': 1, 'h2': 3, 'h3': 6, 'bullet': 33, 'num': 0, 'table': 0, 'fence': 0}
specs/04-mir-core.md: OK {'h1': 1, 'h2': 5, 'h3': 12, 'bullet': 70, 'num': 0, 'table': 0, 'fence': 0} -> {'h1': 1, 'h2': 5, 'h3': 12, 'bullet': 70, 'num': 0, 'table': 0, 'fence': 0}
specs/05-mirrorea-fabric.md: OK {'h1': 1, 'h2': 4, 'h3': 3, 'bullet': 7, 'num': 5, 'table': 0, 'fence': 0} -> {'h1': 1, 'h2': 4, 'h3': 3, 'bullet': 7, 'num': 5, 'table': 0, 'fence': 0}
specs/06-prismcascade-positioning.md: OK {'h1': 1, 'h2': 5, 'h3': 2, 'bullet': 15, 'num': 0, 'table': 0, 'fence': 0} -> {'h1': 1, 'h2': 5, 'h3': 2, 'bullet': 15, 'num': 0, 'table': 0, 'fence': 0}
specs/07-typed-effects-wiring-platform.md: OK {'h1': 1, 'h2': 6, 'h3': 0, 'bullet': 15, 'num': 0, 'table': 0, 'fence': 0} -> {'h1': 1, 'h2': 6, 'h3': 0, 'bullet': 15, 'num': 0, 'table': 0, 'fence': 0}
specs/08-cross-system-relations.md: OK {'h1': 1, 'h2': 4, 'h3': 3, 'bullet': 17, 'num': 3, 'table': 0, 'fence': 0} -> {'h1': 1, 'h2': 4, 'h3': 3, 'bullet': 17, 'num': 3, 'table': 0, 'fence': 0}
specs/09-invariants-and-constraints.md: OK {'h1': 1, 'h2': 6, 'h3': 0, 'bullet': 0, 'num': 18, 'table': 0, 'fence': 0} -> {'h1': 1, 'h2': 6, 'h3': 0, 'bullet': 0, 'num': 18, 'table': 0, 'fence': 0}
specs/10-open-questions.md: OK {'h1': 1, 'h2': 5, 'h3': 0, 'bullet': 0, 'num': 29, 'table': 0, 'fence': 0} -> {'h1': 1, 'h2': 5, 'h3': 0, 'bullet': 0, 'num': 29, 'table': 0, 'fence': 0}
specs/11-roadmap-and-workstreams.md: OK {'h1': 1, 'h2': 4, 'h3': 7, 'bullet': 31, 'num': 7, 'table': 0, 'fence': 0} -> {'h1': 1, 'h2': 4, 'h3': 7, 'bullet': 31, 'num': 7, 'table': 0, 'fence': 0}
specs/12-decision-register.md: DIFF {'h1': 1, 'h2': 1, 'h3': 0, 'bullet': 0, 'num': 0, 'table': 24, 'fence': 0} -> {'h1': 1, 'h2': 1, 'h3': 0, 'bullet': 0, 'num': 0, 'table': 25, 'fence': 0}
```

4. formal token の whole-word 比較

```text
specs/00-document-map.md: overlay:0->1
specs/01-charter-and-decision-levels.md: overlay:0->2
specs/04-mir-core.md: fallback:6->8; patch:0->2; overlay:2->4
specs/05-mirrorea-fabric.md: patch:1->2; overlay:1->4
specs/07-typed-effects-wiring-platform.md: perform:1->0
specs/08-cross-system-relations.md: overlay:0->1
specs/09-invariants-and-constraints.md: overlay:0->1
specs/10-open-questions.md: Mir-1:26->29; overlay:1->2
specs/12-decision-register.md: Mir-1:22->25; overlay:1->2
```

5. `git diff --check`

```text
```

6. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 17 numbered report(s).
```

7. `reviewer`

```text
{"status":{"completed":"no findings"}}
```

意味不変性の判断基準:

- 章立てと節構造:
  - `README.md`、`Documentation.md`、`specs/00..11` は見出し数、箇条書き数、番号付き項目数、表構造、コードフェンス数が一致した。
  - `specs/12-decision-register.md` だけは作業開始前から dirty state で D-023 が追加済みだったため、`HEAD` との差ではなく作業開始時の workspace 内容との手照合を採用した。
- 主要語彙の一対一対応:
  - `Mir-0`, `Mir-1`, `atomic_cut`, `durable_cut`, `place`, `fallback`, `overlay alias`, `Reject`, `Approximate`, `Compensate`, `require`, `ensure`, `invariant` は保持した。
  - formal token の count 比較は補助指標としてだけ使い、差分が出た箇所は手で読み直した。
- 契約・不変条件・決定レベル:
  - `specs/09-invariants-and-constraints.md` の 18 項目、`specs/10-open-questions.md` の 29 項目、`specs/12-decision-register.md` の D-001〜D-023 は配列と識別子を維持した。

補助指標で差分が出た箇所の扱い:

- `specs/07-typed-effects-wiring-platform.md` の `perform:1->0` は、旧文で動詞として一度だけ現れていた `perform` が日本語化で「実行しうる」に置き換わったもので、formal token としての `perform` を削除したものではない。
- `overlay`, `patch`, `fallback` の増加は、既存に分散していた説明文を formal token に寄せて表現統一したことによる表記揺れの収束であり、仕様内容を増やしたものではない。
- `specs/10-open-questions.md` と `specs/12-decision-register.md` の `Mir-1` count 差分は、作業開始前から存在した dirty state を `HEAD` と比較した影響が混ざるため、semantic drift の根拠には使っていない。

semantic drift の危険が高かった箇所:

- `specs/04-mir-core.md`
  - Mir 全体と Mir-0 の区別、`perform` の地位、`atomic_cut` / `durable_cut` の境界は意味変更に直結するため、formal token を保持したまま説明だけを日本語化した。
- `specs/05-mirrorea-fabric.md`
  - `wrap`, `overlay`, `shadow`, `contract space` は完全訳で語感が変わりやすいため、訳し切らずに原語を保持した。
- `specs/10-open-questions.md`
  - durable-cut 系の open question は後続 report 群と用語整合している必要があるため、番号順と formal token を固定したまま英語残存部だけを日本語化した。
- `specs/12-decision-register.md`
  - 既存 dirty state を含むため、翻訳差分と既存の設計差分を混同しないよう、task-start workspace を正本として読んだ。
- `code_mapper` の read-only 結果
  - 対象ファイルの責務分担、`specs/04-mir-core.md` と `specs/10-open-questions.md` を中心とする drift リスク箇所、`safe evolution` / `downstream addition` / `fabric` などの用語統一注意点について、手元確認と同じ整理を返した。

**AMBIGUOUS**:

- `safe evolution`
  - 今回は「安全な進化」で統一したが、互換性保存、graph discipline、downstream addition をどこまで含むかは原文側でも広い。
- `downstream addition`
  - 完全に日本語化するより原語保持のほうが既存 report との対応が安定すると判断した。
- `compatibility-preserving overlay`
  - 型互換、contract 互換、時間資源上の互換のどこまでを含むかが文書横断で分散している。
- `wrap`
  - 日本語説明では「包む」を使うが、boundary operation を指す formal token として `wrap` を残す箇所もあり、完全な一語統一にはしていない。
- `fabric`
  - 「基盤」と訳すと implementation substrate に寄りすぎ、「fabric」を残すと抽象度が高い。今回は subsystem 名に近い箇所では原語を維持した。

## 6. 今回整理して分かったこと

- durable-cut 周辺の recent report 群によって、`specs/10-open-questions.md` と `specs/12-decision-register.md` はすでに日本語化がかなり進んでおり、今回の中心作業は「翻訳」よりも「全体語調の統一」と「意味保存検証」だった。
- semantic drift を避けるうえでは、全面的な直訳よりも、formal token を固定しながら説明文だけを日本語化する方針がこの repo に適していることがはっきりした。
- 既存 dirty state が混ざる文書では、`HEAD` 差分だけでは検証にならず、task-start workspace を正本として扱う必要があることを確認した。

## 7. 未解決事項

- **AMBIGUOUS** として挙げた語のうち、今後 subsystem ごとに日本語訳を固定するか、formal token として英語のまま運用するかは未決定である。
- `specs/10-open-questions.md` と `specs/12-decision-register.md` のような継続 dirty state 文書について、task-start snapshot を自動保存する補助手順が必要かどうかは未決定である。

## 8. Suggested next prompt

`specs/` の日本語正本化が完了した前提で、今度は「日本語正本間で同一概念が同じ言い回しで現れているか」を対象に、語彙表と cross-reference の整合監査を行ってください。特に `safe evolution`、`downstream addition`、`overlay`、`wrap`、`fabric` の訳語方針を文書横断で揃え、必要なら用語集の最小骨格を追加してください。ただし新しい理論判断は追加せず、表現統一に限定してください。
