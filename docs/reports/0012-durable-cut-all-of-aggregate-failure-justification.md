# 報告 0012 — `all_of` aggregate failure justification の整理

- 作成日時: 2026-03-27T10:34:15.122122Z
- 作成者 / agent: Codex
- 対象範囲: Mir-0 を広げずに、`all_of` profile における aggregate failure event の十分条件と Mir-1 / Mirrorea 境界を整理する
- 触れた decision level: L1

## 1. 目的

cross-place `durable_cut` の `all_of` profile について、full coverage が成立しない場合に、どの条件が満たされたとき aggregate failure event を Mir-1 の outcome surface に出してよいかを最小限の意味論として定める。
今回は新しい大きな抽象や新しい failure class を足さず、coverage 不足と failure 確定を分離したまま、failure justification だけを狭く固定することを目的にした。

## 2. スコープと前提

- 対象は `all_of` profile における aggregate failure justification だけであり、Mir-0 の再定義は行わない。
- `quorum-like`、`barrier`、coroutine、overlay、fallback 正規化には広げない。
- 既存の `durable_cut` guarantee / failure / observation / cross-place scope / scope rule family / aggregate evidence の整理は維持する。
- full coverage 不足と aggregate failure を機械的に同一視しない。
- local failure、timeout-like closure、policy cancellation の区別は、Mir-1 が意味として持つ最小条件と、Mirrorea の operational policy / audit に残す理由分類とを分けて扱う。
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
- `docs/reports/0011-durable-cut-all-of-aggregate-evidence.md`

## 4. 実施内容

- `using-superpowers` と `brainstorming` の手順を確認したうえで、`code_mapper` を先に使い、今回の論点が `specs/10-open-questions.md`、`specs/12-decision-register.md`、直前 report 群に集中していることを確認した。
- 指定順で文書を読み、`all_of` では full coverage 不成立が success 不成立を意味するだけで、failure は explicit aggregate failure event でのみ観測される、という直前整理を再確認した。
- `specs/10-open-questions.md` を更新し、aggregate failure event を正当化できる十分条件を二つに限定した。
- 同じ箇所で、required member の explicit local failure は sufficient justification だが aggregate failure event の代用ではないこと、timeout-like budget や policy cancellation は Mir-1 の独立語彙にせず Mirrorea の operational policy / audit に残すことを明記した。
- `specs/12-decision-register.md` に D-020 を追加し、今回の failure justification 判断を L1 として記録した。
- `reviewer` に最終確認を依頼し、coverage 不足と failure の混同や Mir-1 の過剰肥大が起きていないかを確認した。
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

今回実際に変更したファイル:

- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0012-durable-cut-all-of-aggregate-failure-justification.md`

## 6. コマンド実行と正確な出力

1. `python3 scripts/new_report.py --slug durable-cut-all-of-aggregate-failure-justification`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0012-durable-cut-all-of-aggregate-failure-justification.md
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
```

3. `code_mapper`

```text
触るべきファイルは [specs/10-open-questions.md](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md)、[specs/12-decision-register.md](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md)、[docs/reports/0011-durable-cut-all-of-aggregate-evidence.md](/home/yukatayu/dev/mir_poc_01/docs/reports/0011-durable-cut-all-of-aggregate-evidence.md) です。`all_of` profile の aggregate failure justification に直接つながる未決点は [specs/10-open-questions.md:29](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md#L29) と [specs/10-open-questions.md:34](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md#L34) にあり、決定済み境界は [specs/12-decision-register.md:24](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L24) と [specs/12-decision-register.md:25](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L25) に集まっています。
```

4. `git diff --check`

```text
```

5. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 13 numbered report(s).
```

6. `reviewer`

```text
no findings
```

根拠と所見:

- `all_of` では aggregate success に full coverage が必要なので、required member の一つでも対応する local attempt / local prefix に explicit local failure が立てば、その same aggregate attempt は success 不可能になる。したがってこれは aggregate failure event を正当化する十分条件になる。
- ただし local failure event だけでは aggregate failure を観測したことにはならない。failure は既存整理どおり explicit aggregate failure event でのみ outcome surface に現れる。
- full coverage 不足だけでは、単に evidence がまだ揃っていない状態と、aggregate attempt が失敗として打ち切られた状態を区別できないため、failure と同一視できない。
- local failure がまだない場合でも、same aggregate attempt が success 前に failed outcome として明示的に閉じられたなら aggregate failure event を出せると整理すれば、timeout-like budget や policy cancellation を Mir-1 の独立語彙にせずに済む。
- その理由 taxonomy を Mirrorea の operational policy / audit に残すことで、Mir-1 は failure justification の最小意味だけを持ち、Mirrorea は closure 理由と表現形式の管理だけを引き受ける構図を保てる。

規範変更メモ:

- 今回は `all_of` における aggregate failure event の十分条件を、「required member local failure」または「success 前の explicit failed closure」の二つに限定した。
- full coverage 不足だけでは aggregate failure を確定しないことを明文化した。
- timeout-like budget、policy cancellation、retry exhaustion などは Mir-1 の独立語彙にせず、Mirrorea の operational policy / audit に残すと整理した。

## 7. 今回整理して変わった理解

- 直前までの仕様では、`all_of` aggregate evidence は full coverage 条件として整理されていたが、aggregate failure を何が正当化するかはまだ分離しきれていなかった。
- 今回の整理で、本質は「coverage 不足」ではなく「same aggregate attempt の success 不可能性を何が明示するか」だと分かった。
- required member local failure を sufficient justification としつつ、それでも aggregate failure event を別に要求すると、single-place failure と cross-place outcome surface の役割を混同せずに済む。
- timeout-like closure や policy cancellation を Mir-1 の named vocabulary に入れないことで、Mir-1 は outcome の意味だけを保ち、Mirrorea に operational policy を押し戻せる。

## 8. 未決定事項

- timeout-like closure を将来 Mir-1 の独立語彙として標準化する必要が本当にあるかは **未決定**。
- policy-driven closure を Mir-1 の意味に引き上げるべきか、Mirrorea の operational policy に留めるべきかは **未決定**。
- local evidence 不足と failed closure の関係を audit surface でどこまで構造化して残すかは **未決定**。
- 将来 `quorum-like` などの profile が追加されたとき、aggregate failure justification の意味差分をどこまで共通化するかは **未決定**。
- `Approximate` を許す contract 条件と cross-place aggregate failure justification の関係は **未決定**。

## 9. Suggested next prompt

Mir-0 を広げずに、`all_of` profile における aggregate failure の audit trace 最小要件を整理してください。特に required member local failure と explicit failed closure を後から区別して説明するために、audit surface が最低限保持すべき local attempt / local prefix / missing coverage / reason reference を、日本語で `specs/10-open-questions.md` と必要最小限の関連文書に反映してください。
