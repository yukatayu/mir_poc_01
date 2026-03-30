# 報告 0014 — `all_of` missing coverage snapshot の基準時点と粒度整理

- 作成日時: 2026-03-27T12:45:17.580096Z
- 作成者 / agent: Codex
- 対象範囲: Mir-0 を広げずに、`all_of` profile における missing coverage snapshot の基準時点と最小粒度を整理する
- 触れた decision level: L1

## 1. 目的

cross-place `durable_cut` の `all_of` profile について、aggregate failure を監査で説明するために required member ごとの coverage state を「いつ」「どの粒度で」切り取れば十分かを最小限の意味論として定める。
今回は新しい大きな抽象を足さず、single snapshot の基準時点と per-required-member の最小状態区別だけを Mir-1 に残し、履歴管理や表現形式は Mirrorea に送ることを目的にした。

## 2. スコープと前提

- 対象は `all_of` profile における missing coverage snapshot の基準時点と抽象粒度だけであり、Mir-0 の再定義は行わない。
- `quorum-like`、`barrier`、coroutine、overlay、fallback 正規化には広げない。
- 既存の `durable_cut` guarantee / failure / observation / cross-place scope / scope rule family / aggregate evidence / aggregate failure justification / audit trace 最小要件の整理は維持する。
- event surface と audit surface を混同しない。
- snapshot の具体的保存形式、ID 形式、serialization、圧縮、複数世代管理は Mir-1 で固定しない。
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
- `docs/reports/0013-durable-cut-all-of-aggregate-failure-audit-trace.md`

## 4. 実施内容

- `using-superpowers` と `brainstorming` の手順を確認したうえで、`code_mapper` を先に使い、今回の論点が `specs/10-open-questions.md`、`specs/12-decision-register.md`、直近 report 群に集中していることを確認した。
- 指定順で文書を読み、既存仕様では missing coverage snapshot が「failure 観測時点または closure 時点」とだけ書かれており、どちらを最小基準時点として採るかがまだ曖昧だと確認した。
- `specs/10-open-questions.md` を更新し、same aggregate attempt が aggregate failure outcome として明示的に観測された決定点を minimum baseline と定めた。
- 同じ箇所で、explicit failed closure がそのまま failure outcome を明示する場合だけ closure 時点が baseline になること、required member local failure が justification source の場合は member-local failure 時刻ではなく aggregate failure 観測時点を使うことを明記した。
- coverage state の最小粒度を per-required-member 粒度に限定し、各 member について対応する local attempt / local prefix 参照に結びついた三状態の区別だけを要求した。
- `specs/12-decision-register.md` に D-022 を追加し、今回の snapshot semantics を L1 として記録した。
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
- `docs/reports/0013-durable-cut-all-of-aggregate-failure-audit-trace.md`

今回実際に変更したファイル:

- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0014-durable-cut-all-of-missing-coverage-snapshot.md`

## 6. コマンド実行と正確な出力

1. `python3 scripts/new_report.py --slug durable-cut-all-of-missing-coverage-snapshot`

```text
/home/yukatayu/dev/mir_poc_01/docs/reports/0014-durable-cut-all-of-missing-coverage-snapshot.md
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
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
?? docs/reports/0014-durable-cut-all-of-missing-coverage-snapshot.md
```

3. `code_mapper`

```text
1. Most relevant files / line areas

- [specs/10-open-questions.md:28](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md#L28) to [specs/10-open-questions.md:37](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md#L37) is the main target. This is where `all_of` audit requirements, full-coverage absence, aggregate failure justification, and the remaining open wording already sit.
- [specs/12-decision-register.md:25](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L25) to [specs/12-decision-register.md:26](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L26) is the normative summary layer. D-019 covers aggregate evidence coverage; D-020 covers aggregate failure justification.

2. Existing dirty state vs smallest likely files to touch

- Current dirty state from `git status --short`: `agents/implementer.toml`, `specs/04-mir-core.md`, `specs/09-invariants-and-constraints.md`, `specs/10-open-questions.md`, `specs/12-decision-register.md`, plus untracked reports through [0012](/home/yukatayu/dev/mir_poc_01/docs/reports/0012-durable-cut-all-of-aggregate-failure-justification.md).
- Smallest likely touch set for this task: [specs/10-open-questions.md](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md), [specs/12-decision-register.md](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md), and the next new report under `docs/reports/`.

3. Wording risks

- Snapshot semantics can become too implementation-specific if “missing coverage snapshot” starts implying a concrete storage form, timestamp model, checkpoint protocol, or transport-level state capture.
- There is a mixing risk between event surface and audit surface if the spec starts requiring the aggregate failure event itself to carry per-place missing-coverage detail inline.
```

4. `git diff --check`

```text
```

5. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 15 numbered report(s).
```

6. `reviewer`

```text
no findings
```

根拠と所見:

- missing coverage snapshot は aggregate failure を監査で説明するためのものなので、最小基準時点は outcome surface に failure が現れた決定点に合わせるのが最も狭い。
- ただし failed closure 自体がその決定点を構成する場合は、closure 時点を基準時点としてそのまま読める。これで closure-based path だけ別の抽象を足さずに済む。
- required member local failure が justification source であっても、その member-local failure 時刻を基準にしてしまうと aggregate failure 観測より前の途中状態を Mir-1 に持ち込むことになる。そのため aggregate failure 観測時点を基準に揃える方が境界を保てる。
- 粒度については per-required-member 粒度で十分であり、既存の local attempt / local prefix 対応を参照しつつ、「counted coverage あり」「local failure により不可能」「未充足」の三状態を区別できれば、failure explanation に必要な最小説明可能性は満たせる。
- event-by-event 履歴や複数 snapshot 比較を Mir-1 で要求すると audit semantics が重くなるため、今回は最小要件から外すのが自然である。

規範変更メモ:

- 今回は `all_of` missing coverage snapshot の minimum baseline を aggregate failure outcome の明示観測点に固定した。
- explicit failed closure がそのまま failure outcome を表す場合だけ closure 時点を baseline とする、と整理した。
- 最小粒度は per-required-member 粒度で、三状態の区別までに限定した。

## 7. 今回整理して変わった理解

- 0013 までで snapshot の必要性自体は整理されていたが、「いつの状態を撮るのか」が曖昧なままだと audit 要件の読み方が分岐しうると分かった。
- 今回、baseline を aggregate failure outcome の決定点に寄せることで、failure justification と snapshot semantics を同じ境界で説明できるようになった。
- また、粒度を per-required-member に留めることで、per-prefix 履歴や event-by-event 監査を Mir-1 に持ち込まずに済むことも明確になった。

## 8. 未決定事項

- snapshot の複数時点比較を将来 Mir-1 に持ち込む必要が本当にあるかは **未決定**。
- `reason_ref` 相当の参照語彙を将来 Mir-1 で標準化する必要があるかは **未決定**。
- per-required-member の三状態を将来 named vocabulary に引き上げる必要があるかは **未決定**。
- 将来 `quorum-like` などの profile が追加されたとき、今回の snapshot semantics をどこまで一般化して再利用するかは **未決定**。
- Mirrorea が複数 snapshot や中間状態保持を実装するための最小 protocol surface は **未決定**。

## 9. Suggested next prompt

Mir-0 を広げずに、`all_of` profile における required member coverage state の命名と contract との関係を整理してください。特に今回整理した三状態を Mir-1 の独立語彙に上げる必要があるか、それとも audit 説明上の抽象区別に留めるべきかを、日本語で `specs/10-open-questions.md` と必要最小限の関連文書に反映してください。
