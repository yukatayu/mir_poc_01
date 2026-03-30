# 報告 0013 — `all_of` aggregate failure の audit trace 最小要件整理

- 作成日時: 2026-03-27T11:52:17.273785Z
- 作成者 / agent: Codex
- 対象範囲: Mir-0 を広げずに、`all_of` profile における aggregate failure を後から説明するための audit trace 最小要件を整理する
- 触れた decision level: L1

## 1. 目的

cross-place `durable_cut` の `all_of` profile について、aggregate failure を後から説明するために audit surface が最低限保持すべき情報を定める。
今回は新しい大きな抽象や新しい failure class を足さず、required member local failure、explicit failed closure、missing coverage を区別して説明するための最小情報だけを Mir-1 に残し、表現形式は Mirrorea に送ることを目的にした。

## 2. スコープと前提

- 対象は `all_of` profile における aggregate failure の audit trace 最小要件だけであり、Mir-0 の再定義は行わない。
- `quorum-like`、`barrier`、coroutine、overlay、fallback 正規化には広げない。
- 既存の `durable_cut` guarantee / failure / observation / cross-place scope / scope rule family / aggregate evidence / aggregate failure justification の整理は維持する。
- event surface と audit surface を混同しない。
- `reason_ref`、ID 形式、serialization、ack 形式、署名形式、保存バックエンドなどの具象表現は Mir-1 で固定しない。
- `specs/04-mir-core.md` はこれ以上膨らませず、今回の更新は `specs/10-open-questions.md` と `specs/12-decision-register.md` に留める。

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
- `docs/reports/0011-durable-cut-all-of-aggregate-evidence.md`
- `docs/reports/0012-durable-cut-all-of-aggregate-failure-justification.md`

## 4. 実施内容

- `using-superpowers` と `brainstorming` の手順を確認したうえで、`code_mapper` を先に使い、今回の論点が `specs/10-open-questions.md`、`specs/12-decision-register.md`、直近 report 群に集中していることを確認した。
- 指定順で文書を読み、`all_of` では event surface に aggregate failure outcome だけを残し、failure の説明可能性は audit surface に押し下げる既存方針を再確認した。
- `specs/10-open-questions.md` を更新し、aggregate failure audit に必要な最小要件を、aggregate attempt、participating / required member 集合、local attempt / local prefix 対応、justification source、missing coverage 状態の説明可能性として明文化した。
- 同じ箇所で、justification source が local failure の場合と failed closure の場合を分けて、どの参照可能性が必要かを明記した。
- `reason_ref` のような field 名や独立語彙は Mir-1 で固定しないこと、ID 形式や audit layout は Mirrorea の裁量に残すことを明記した。
- Mirrorea 節にも一文追記し、aggregate failure audit の参照表現をどう ID 化・直列化・配置するかは Mirrorea の実現責務に残ると整理した。
- `specs/12-decision-register.md` に D-021 を追加し、今回の audit trace 最小要件を L1 として記録した。
- 本レポートを新規追加した。

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
- `docs/reports/0011-durable-cut-all-of-aggregate-evidence.md`
- `docs/reports/0012-durable-cut-all-of-aggregate-failure-justification.md`

今回実際に変更したファイル:

- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0013-durable-cut-all-of-aggregate-failure-audit-trace.md`

## 6. コマンド実行と正確な出力

1. `python3 scripts/new_report.py --slug durable-cut-all-of-aggregate-failure-audit-trace`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0013-durable-cut-all-of-aggregate-failure-audit-trace.md
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
?? docs/reports/0011-durable-cut-all-of-aggregate-evidence.md
?? docs/reports/0012-durable-cut-all-of-aggregate-failure-justification.md
?? docs/reports/0013-durable-cut-all-of-aggregate-failure-audit-trace.md
```

3. `code_mapper`

```text
1. Relevant files and line areas

- [specs/10-open-questions.md:22](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md#L22) to [specs/10-open-questions.md:37](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md#L37) is the main target. This is where cross-place failure observation, `all_of` coverage, aggregate failure justification, audit trace requirements, and the remaining open questions now live.
- [specs/12-decision-register.md:25](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L25) to [specs/12-decision-register.md:26](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L26) is the normative summary layer. D-019 fixes `all_of` aggregate evidence; D-020 fixes minimum aggregate failure justification.
- [specs/05-mirrorea-fabric.md:21](/home/yukatayu/dev/mir_poc_01/specs/05-mirrorea-fabric.md#L21) to [specs/05-mirrorea-fabric.md:23](/home/yukatayu/dev/mir_poc_01/specs/05-mirrorea-fabric.md#L23) is secondary boundary context only. It matters if audit wording starts drifting into representation or path-proof details.

2. Existing dirty state vs likely files to touch

- Current dirty state from `git status --short`: `agents/implementer.toml`, `specs/04-mir-core.md`, `specs/09-invariants-and-constraints.md`, `specs/10-open-questions.md`, `specs/12-decision-register.md`, and untracked reports [0001](/home/yukatayu/dev/mir_poc_01/docs/reports/0001-mir-0-semantic-core-alignment.md) through [0012](/home/yukatayu/dev/mir_poc_01/docs/reports/0012-durable-cut-all-of-aggregate-failure-justification.md).
- For this task, the smallest likely touch set is [specs/10-open-questions.md](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md), [specs/12-decision-register.md](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md), and the new report file.

3. Wording risk where audit surface could become too implementation-specific

- The safe line is: Mir-1 says what must be explainable in audit; Mirrorea decides how that trace is represented.
```

4. `git diff --check`

```text
```

5. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 14 numbered report(s).
```

6. `reviewer`

```text
no findings
```

根拠と所見:

- event surface に必要なのは従来どおり aggregate failure outcome だけであり、どの local failure や closure が原因だったかは audit surface に残せば十分である。
- aggregate failure を後から説明するには、aggregate attempt と member 対応だけでは足りず、justification source と missing coverage 状態の両方が参照可能である必要がある。
- local failure 起点の失敗と failed closure 起点の失敗を区別できれば、full coverage 不足だけでは failure を確定しないという既存方針を壊さずに済む。
- ただし `reason_ref`、ID 形式、field 名、layout を Mir-1 で固定すると audit surface が実装依存に寄りすぎるため、Mir-1 では「参照可能であること」だけを要求するのが最も狭い。
- missing coverage についても、required member ごとの状態区別を要求すれば十分であり、その snapshot の具体表現や圧縮方式は Mirrorea の audit representation に残せる。

規範変更メモ:

- 今回は `all_of` aggregate failure audit の最小要件を、aggregate attempt、member 集合、local attempt / local prefix 対応、justification source、missing coverage 状態の説明可能性に限定した。
- `reason_ref` のような field 名や独立語彙は Mir-1 で固定しないと整理した。
- aggregate failure audit の ID 化・直列化・layout は Mirrorea の実現責務に残すと明記した。

## 7. 今回整理して変わった理解

- 直前の 0012 で failure justification は整理されていたが、それを監査上どう説明するかはまだ一段抽象度が混ざっていた。
- 今回の整理で、Mir-1 が本当に必要とするのは `reason_ref` という語そのものではなく、justification source と missing coverage 状態を追跡できることだと分かった。
- audit surface の最小要件を「説明可能性」に寄せると、event surface を増やさずに required member local failure と explicit failed closure を区別できる。
- その結果、Mir-1 は traceability requirement だけを持ち、Mirrorea はその参照表現を実装するという境界がさらに明瞭になった。

## 8. 未決定事項

- `reason_ref` 相当の参照語彙を将来 Mir-1 で標準化する必要が本当にあるかは **未決定**。
- audit の粒度を per-place / per-attempt / per-prefix のどこまで Mir-1 で固定するかは **未決定**。
- missing coverage snapshot を単一時点で十分とするか、複数観測点を要求するかは **未決定**。
- 将来 `quorum-like` などの profile が追加されたとき、今回の audit 最小要件をどこまで一般化して再利用するかは **未決定**。
- Mirrorea が aggregate failure audit representation で必要とする最小 protocol surface は **未決定**。

## 9. Suggested next prompt

Mir-0 を広げずに、`all_of` profile における aggregate failure の missing coverage snapshot をさらに詰めてください。特に failure 観測時点と closure 時点のどちらを Mir-1 の基準時点として採るべきか、また required member ごとの coverage state をどこまで抽象的に保てば十分かを、日本語で `specs/10-open-questions.md` と必要最小限の関連文書に反映してください。
