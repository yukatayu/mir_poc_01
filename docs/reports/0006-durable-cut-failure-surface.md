# 報告 0006 — `durable_cut` の failure surface 整理

- 作成日時: 2026-03-27T06:48:07.997953Z
- 作成者 / agent: Codex
- 対象範囲: Mir-0 を広げずに、Mir-1 側の `durable_cut` failure を既存 failure lattice (`Reject` / `Approximate` / `Compensate`) にどう対応づけるかを整理する
- 触れた decision level: L1

## 1. 目的

`durable_cut` が失敗したときに、それを Mir の既存 failure lattice でどう表現するかを最小限の意味論として明確にする。
今回は新しい大きな抽象や新しい failure class を足さず、Mir-1 と Mirrorea の境界を崩さないことを優先した。

## 2. スコープと前提

- 対象は `durable_cut` の failure surface だけであり、Mir-0 の再定義は行わない。
- `barrier`、coroutine、overlay、fallback 正規化には広げない。
- 既存 failure lattice の再利用を優先し、新しい failure class が必要かどうかは最後まで保留して検討した。
- `specs/05-mirrorea-fabric.md` は境界確認のためだけに参照し、Mirrorea の実装詳細には踏み込まない。
- `Documentation.md` は今回の変更を反映しなくても全体要約として破綻しないため、更新しない。

## 3. 参照した文書

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `docs/reports/0003-mir-0-cut-and-vocabulary-clarification.md`
- `docs/reports/0004-cut-vocabulary-responsibility-split.md`
- `docs/reports/0005-durable-cut-minimum-guarantee-set.md`
- 境界確認のみ:
  - `specs/05-mirrorea-fabric.md`

## 4. 実施内容

- `code_mapper` を起動し、`durable_cut` failure surface を整理するのに必要な最小ファイル集合と、failure lattice への寄せ方の見立てを依頼した。
- 指定順で文書を読み、Mir-0 では `atomic_cut` だけが最小 cut primitive であり、`durable_cut` は Mir-1 側の後段語彙であることを再確認した。
- `specs/04-mir-core.md` に、`durable_cut` failure は durable guarantee を確立できなかった失敗試行であり、新しい failure class を導入しないことを注記した。
- 同じ箇所で、既定分類を `Reject`、cut 後の外部化された obligations を巻き戻す必要がある場合のみ `Compensate`、`Approximate` は contract が durability を弱めた代替結果を明示的に許す場合に限ると整理した。
- `specs/10-open-questions.md` を更新し、`durable_cut` failure surface を未決の大箱のまま残すのではなく、何が決まり、何がまだ未決定かを分けて記録した。
- `specs/12-decision-register.md` に D-015 を追加し、failure lattice 再利用の判断を L1 として記録した。
- 本レポートを新規追加した。

## 5. 変更したファイル

- `specs/04-mir-core.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0006-durable-cut-failure-surface.md`

## 6. 根拠・出力・テスト結果

1. `python3 scripts/new_report.py --slug durable-cut-failure-surface`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0006-durable-cut-failure-surface.md
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
?? docs/reports/0006-durable-cut-failure-surface.md
```

3. `rg -n "Reject|Approximate|Compensate|durable_cut|failure surface|failure" specs docs/reports | sed -n '1,220p'`

```text
`Reject` / `Approximate` / `Compensate` が規範的に固定されている箇所が `specs/04-mir-core.md` に限られ、先行レポートでは `durable_cut` failure surface が未解決として残されていたことを確認した。
```

根拠と所見:

- durable guarantee を確立できなかったとは、successful な `durable_cut` として pre-cut prefix を確定できなかったことを意味する。これは「成功した cut だが durability が弱い」という状態ではなく、失敗試行と読むのが最も狭い。
- そのため既定分類は `Reject` が自然である。成功条件を満たしていない以上、まず cut の成立拒否として扱う方が既存 lattice と整合する。
- `Compensate` が必要なのは、失敗が cut 後に表面化し、すでに外部化された obligations を明示的に巻き戻す必要がある場合に限られる。これは新しい failure class ではなく、既存の compensation path で表現できる。
- `Approximate` は durability を弱めた代替結果を contract が明示的に許すなら理論上は入りうるが、それを既定分類にすると `durable_cut` 自体の意味を曖昧にする。そのため今回は既定から外した。
- local durable failure と distributed durable failure は、現段階では別 failure class に分ける必要がない。どちらも Mir-1 では同じ durable guarantee failure であり、違いは Mirrorea が引き受ける実現側にある。
- 新しい failure class は現時点では不要である。

規範変更メモ:

- 今回は `durable_cut` failure を既存 failure lattice で再利用する方針に固定した。
- 既定分類は `Reject`、補償が必要な場合のみ `Compensate`、`Approximate` は contract が durability を弱めた代替結果を明示的に許す場合に限ると整理した。
- local / distributed の違いは実現上の差分として扱い、新しい failure class にはしなかった。

## 7. 今回整理して変わった理解

- `durable_cut` failure を未決のまま残していた主因は、durable guarantee failure と distributed realization failure を同じ場所で見ていたことだった。
- Mir-1 で必要なのは failure の意味づけであり、Mirrorea で必要なのはその実現上の差分をどう扱い、どう観測するかである。
- 既存の `Reject` / `Compensate` を使い分ければ、failure lattice を壊さずに `durable_cut` failure を表現できる。
- `Approximate` は完全に排除する必要はないが、contract による明示許可がない限り既定分類に入れるべきではないと整理できた。

## 8. 未決定事項

- `persistence evidence` を何で観測し、失敗観測点をどこに置くかは **未決定**。
- 複数 `place` にまたがる `durable_cut` の scope rule は **未決定**。
- contract が durability を弱めた代替結果を許す場合、`Approximate` をどこまで認めるかは **未決定**。
- Mirrorea が distributed durable failure を実現・観測するための最小 protocol surface は **未決定**。

## 9. Suggested next prompt

Mir-0 を広げずに、`durable_cut` の persistence evidence と failure observation point を整理してください。特に successful / failed `durable_cut` をどの event / audit surface で観測するのかを、日本語で `specs/10-open-questions.md` と必要最小限の関連文書に反映してください。
