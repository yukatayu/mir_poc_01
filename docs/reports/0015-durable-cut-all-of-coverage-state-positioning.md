# 報告 0015 — `all_of` required member coverage state 三状態の位置づけ整理

- 作成日時: 2026-03-30T19:20:20+09:00
- 作成者 / agent: Codex
- 対象範囲: Mir-0 を広げずに、`all_of` profile における required member coverage state 三状態を Mir-1 の独立語彙に上げるべきか、それとも audit 説明上の抽象区別に留めるべきかを整理する
- 触れた decision level: L1

## 1. Objective

`all_of` profile の missing coverage snapshot で使っている required member coverage state 三状態

- counted success-side coverage あり
- local failure により coverage 不可能
- 未充足

が、Mir-1 の独立した意味語彙として必要かどうかを判断する。目的は新しい profile や failure class を足すことではなく、aggregate success / failure、contract、audit の境界を壊さずに、この三状態の位置づけだけを最小限で固定することである。

## 2. Scope and assumptions

- 今回は `all_of` profile の required member coverage state 三状態の位置づけだけを対象とし、Mir-0 の再定義は行わない。
- `quorum-like`、`barrier`、coroutine、overlay、fallback 正規化には広げない。
- 既存の `durable_cut` guarantee / failure / observation / cross-place scope / scope rule family / aggregate evidence / aggregate failure justification / audit trace / snapshot semantics の整理は維持する。
- contract と audit の責務を混同しない。
- `reason_ref` のような field 名、ID 形式、serialization は Mir-1 で固定しない。
- `code_mapper` はユーザー指定どおり先に起動したが、この turn では `wait_agent` の待機時間内に応答が返らなかったため、ローカル読解と差分確認を併用して進めた。
- `reviewer` も最後に起動したが、このセッションでは結果を回収できなかったため、review finding は report できていない。

## 3. Documents consulted

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
- `docs/reports/0014-durable-cut-all-of-missing-coverage-snapshot.md`

## 4. Actions taken

- 指定順で文書を再読し、三状態が現行 specs では missing coverage snapshot と aggregate failure audit の説明にしか現れておらず、aggregate success / failure や contract の定義には組み込まれていないことを確認した。
- `all_of` の aggregate success / failure、full coverage、failure justification を読み直し、これらの意味はすでに counted success-side coverage の充足と explicit failure justification で閉じており、三状態名を Mir-1 の first-class 語彙にしなくても成立することを確認した。
- `specs/10-open-questions.md` を更新し、三状態は `all_of` aggregate failure audit と missing coverage snapshot を説明するための最小抽象区別に留め、Mir-1 の独立した outcome / contract 語彙には上げないことを明記した。
- 同じ箇所で、aggregate success / failure、full coverage、failure justification は従来どおり coverage 充足と explicit failure justification で定まり、contract が三状態名を直接参照する必要はないと追記した。
- `specs/12-decision-register.md` に D-023 を追加し、三状態を audit 説明可能性のための最小抽象区別に留める判断を L1 として記録した。
- `docs/reports/0015-durable-cut-all-of-coverage-state-positioning.md` を新規追加した。

作業開始時に確認した既存 dirty state:

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

今回実際に変更したファイル:

- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0015-durable-cut-all-of-coverage-state-positioning.md`

## 5. Evidence / outputs / test results

1. `python3 scripts/new_report.py --slug durable-cut-all-of-coverage-state-positioning`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0015-durable-cut-all-of-coverage-state-positioning.md
```

2. 作業開始時の `git status --short`

```text
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
2 回起動したが、どちらも wait window 内では完了せず、`timed_out: true` だった。
```

4. `git diff --check`

```text
```

5. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 16 numbered report(s).
```

6. `reviewer`

```text
起動はしたが、このセッションでは agent id / review result を回収できなかった。
```

7. 規範変更の要点

- `specs/10-open-questions.md` では、三状態は missing coverage snapshot の audit 説明のための最小抽象区別であり、Mir-1 の独立した outcome / contract 語彙ではないと明記した。
- `specs/12-decision-register.md` では D-023 を追加し、aggregate success / failure、full coverage、failure justification の定義は従来どおり三状態名を必要としないことを記録した。

## 6. What changed in understanding

- 0014 では三状態を minimum snapshot granularity として固定していたが、その時点では「三状態そのものが Mir-1 の意味語彙かどうか」は未整理だった。
- 今回、aggregate success / failure の定義は full coverage と explicit failure justification で十分に閉じており、三状態は outcome semantics ではなく audit explanation に必要な抽象区別だと明確になった。
- これにより contract、outcome surface、audit surface の境界も整理され、Mir-1 が固定するべきなのは「三状態を名前付きで持つこと」ではなく「required member ごとにその区別を説明可能であること」だと分かった。

## 7. Open questions

- `quorum-like` など別 profile が将来入ったときにも、coverage state の区別を名前付き語彙へ引き上げずに済むかは **未決定**。
- contract language が将来、coverage state 相当の区別を明示的に参照したくなるかは **未決定**。
- `reason_ref` 相当の参照語彙を Mir-1 で標準化する必要があるかは **未決定**。
- snapshot の複数時点比較や event-by-event 履歴を Mir-1 で語る必要が将来あるかは **未決定**。
- subagent review が今回の待機窓では返っていないため、追加の文言検査を別 turn で再実行する必要があるかは **未決定**。

## 8. Suggested next prompt

Mir-0 を広げずに、`all_of` profile における aggregate failure audit が参照する justification source の最小正規形を整理してください。特に required member local failure と explicit failed closure を、Mir-1 の意味語彙としてどこまで共通化し、どこから先を Mirrorea の audit representation に残すべきかを、日本語で `specs/10-open-questions.md` と必要最小限の関連文書に反映してください。
