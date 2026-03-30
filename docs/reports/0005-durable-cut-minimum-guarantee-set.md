# 報告 0005 — `durable_cut` の最小 guarantee set 整理

- 作成日時: 2026-03-27T05:58:14.893313Z
- 作成者 / agent: Codex
- 対象範囲: Mir-0 を広げずに、Mir-1 側の cut vocabulary 候補である `durable_cut` の最小 guarantee set を整理する
- 触れた decision level: L1

## 1. 目的

`durable_cut` が `atomic_cut` に対して何を追加する語彙なのかを、最小限の意味論として定める。
今回は新しい大きな抽象を足さず、Mir-1 に残す意味づけと Mirrorea に送る operational responsibility を切り分けることだけを目的にした。

## 2. スコープと前提

- 今回の対象は `durable_cut` の最小 guarantee set だけであり、Mir-0 の再定義は行わない。
- `barrier` は比較対象として触れるに留め、詳細設計には入らない。
- `persistence`、`durable commit`、`distributed finalization` の三語は、既存文書に現れている範囲を越えて拡張しない。
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
- `docs/reports/0003-mir-0-cut-and-vocabulary-clarification.md`
- `docs/reports/0004-cut-vocabulary-responsibility-split.md`

## 4. 実施内容

- `code_mapper` を起動し、`durable_cut` の現状位置づけと今回触るべき最小ファイル集合の確認を依頼した。
- 指定順で文書を読み、`atomic_cut` が Mir-0 の place-local rollback frontier に限定されていること、`durable_cut` は Mir-1 側の後段語彙としてのみ扱われていることを再確認した。
- `specs/04-mir-core.md` を更新し、`durable_cut` の最小意味を abstract persistence requirement を伴う durable commit guarantee とし、`distributed finalization` は operational realization 側に分けた。
- `specs/10-open-questions.md` を更新し、`durable_cut` に関する open question を「三択」から「最小意味は何か、どこからが未決定か」に書き換えた。
- `specs/12-decision-register.md` に D-014 を追加し、今回の判断を L1 の境界明確化として記録した。
- 本レポートを新規追加した。

変更したファイル:

- `specs/04-mir-core.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0005-durable-cut-minimum-guarantee-set.md`

## 5. 根拠・出力・テスト結果

1. `python3 scripts/new_report.py --slug durable-cut-minimum-guarantee-set`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0005-durable-cut-minimum-guarantee-set.md
```

2. `git status --short`

```text
 M specs/04-mir-core.md
 M specs/09-invariants-and-constraints.md
 M specs/10-open-questions.md
 M specs/12-decision-register.md
?? docs/reports/0001-mir-0-semantic-core-alignment.md
?? docs/reports/0002-mir-0-review-findings.md
?? docs/reports/0003-mir-0-cut-and-vocabulary-clarification.md
?? docs/reports/0004-cut-vocabulary-responsibility-split.md
?? docs/reports/0005-durable-cut-minimum-guarantee-set.md
```

3. `git diff --check`

```text
```

4. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 6 numbered report(s).
```

根拠と所見:

- `durable_cut` の最小意味に含めるべきなのは、`atomic_cut` の rollback frontier に加えて、pre-cut prefix が abstract persistence requirement を伴う durable commit 済みであり、local rollback / process restart / route rebinding の後も未確定へ戻らないことである。
- `persistence` は `durable_cut` 自体の別名ではなく、この durable commit guarantee を支えるための抽象要件として扱うのが最も狭い。
- `distributed finalization` は bare な `durable_cut` の最小意味には入れず、複数 `place` / 実ノード上でその guarantee を実現するときの Mirrorea 側責務へ送るのが自然である。
- これにより、Mir-1 は cut 語彙の意味づけを保ちつつ、Mirrorea に storage / replication / route-safe realization の具体機構を押し付ける構図を避けられる。

規範変更メモ:

- 今回は `durable_cut` の最小追加保証を abstract persistence requirement を伴う durable commit guarantee として固定した。
- `persistence` は抽象要件として前提するが、具体機構は Mir-1 で固定しないとした。
- `distributed finalization` は最小意味そのものではなく、Mirrorea の operational realization 側に属すると整理した。

## 6. 今回整理して変わった理解

- これまでの文書では `durable_cut` が persistence / durable commit / distributed finalization をまとめて抱えるようにも読めたが、今回の整理で最小意味は abstract persistence requirement を伴う durable commit guarantee に絞れると分かった。
- `persistence` は Mir-1 の意味づけに必要だが、あくまで abstract requirement として扱えば Mirrorea の実装責務と衝突しない。
- `distributed finalization` を Mirrorea 側へ送ることで、Mir-1 は cut の意味を定め、Mirrorea はその分散実現を担うという層分離が以前より明確になった。
- `atomic_cut` と `durable_cut` の差分も、rollback frontier と durable commit guarantee の差として説明できるようになった。

## 7. 未決定事項

- `persistence evidence` を何で観測し、どこまで contract / audit から参照可能にするかは **未決定**。
- `durable_cut` が複数 `place` にまたがるときの正確な scope rule は **未決定**。
- durability を確立できなかった場合に、それを `Reject` / `Compensate` / 別 failure class のどこへ落とすかは **未決定**。
- Mirrorea が `durable_cut` の実現で必要とする最小 protocol surface は **未決定**。

## 8. Suggested next prompt

Mir-0 を広げずに、`durable_cut` が失敗した場合の failure surface を整理してください。特に durable commit を確立できなかったとき、その失敗を `Reject`、`Compensate`、または別の明示的 failure としてどう扱うべきかを、日本語で `specs/10-open-questions.md` と必要最小限の関連文書に反映してください。
