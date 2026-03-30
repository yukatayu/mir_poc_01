# 報告 0024 — representative Mir programs（current L2）整備

- 作成日時: 2026-03-31T03:02:56+09:00
- 作成者 / agent: Codex
- 対象範囲: current L2 の Mir / Mir-0 読解に基づく representative program 集の追加、最小導線の追加、各例に対する static 判定 / runtime outcome / trace 説明の整理
- 触れた decision level: 既存の Mir-0 / current L2 の意味論を前提にした例示資産の追加であり、新しい理論判断は追加しない

## 1. 目的

current L2 の Mir で何が自然に書けるかを、representative program 集として可視化する。
今回は parser や interpreter を実装せず、説明用表記の representative code と、その期待される static 判定、runtime outcome、最小 trace を揃えることに集中する。

## 2. 範囲と前提

- `AGENTS.md` の指示に従い、`README.md`、`Documentation.md`、`specs/00..04`、`specs/09`、`specs/10`、`specs/11`、`specs/12`、`docs/reports/0018`、`docs/reports/0019`、`docs/reports/0020`、`docs/reports/0021`、`docs/reports/0022`、`docs/reports/0023` を指定順で読んだ。
- 作業開始時点の git dirty state は `## main...origin/main` だけで、tracked / untracked の作業中変更はなかった。
- 既存の Mir-0 / Mir-1 / Mirrorea 境界、fallback canonical normalization law、incompatible branch rejection phase、static evidence floor、underdeclared handling は変更しない。
- `place` をスコープ指定しながら入れ子で書く方式を基本スタイルとする。
- `perform`、`lineage(A -> B)`、option の `target=...` / `capability=...` / `lease=...` は説明用表記であり、最終 surface syntax を固定しない。
- `code_mapper` は最初に起動したが、このセッションでは明示的な応答を回収できなかった。代替として `git status --short --branch`、`find specs -maxdepth 2 -type d | sort`、`rg` によるローカル確認で dirty state と examples の置き場候補を確認した。
- `reviewer` を最後に使い、examples が current L2 を超えていないか、説明用記法が未決 syntax を決め打ちしていないかを点検した。

## 3. 参照文書

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
- `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
- `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`
- `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`
- `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`
- `docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md`
- `docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md`

## 4. 実施内容

- `specs/examples/00-representative-mir-programs.md` を新設し、representative program 集を 1 本に集約した。
- 例は次の 6 つに絞った。
  - `place` 入れ子 + authority update + `atomic_cut`
  - local `try` + `fallback`
  - valid な fallback / preference chain
  - malformed fallback branch
  - underdeclared fallback case
  - write-after-expiry が `Reject` になる例
- 各例について、コード、期待される static 判定、期待される runtime outcome、最小 trace 説明をそろえた。
- examples 文書の冒頭で、コードが current L2 の**説明用表記**であり、最終 parser grammar や予約語集合を固定しないことを明記した。
- `specs/00-document-map.md` に examples 文書の導線を追加し、役割を「補助正本」として記載した。
- `Documentation.md` に、current L2 の書き味を representative code で確認したい読者向け導線を追加した。
- `README.md`、`specs/04-mir-core.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` は今回は参照のみとし、理論本文は変更しなかった。
- 作業中に `docs/reports/0025-representative-mir-programs-current-l2.md` が一時的に見えたが、最終成果物には含めず、作業完了時点では report 一覧に残っていないことを確認した。
- commit は 2 本に分ける。
  - 1 本目は examples 文書と導線だけを記録する `representative Mir programs を追加する`
  - 2 本目は本 report を記録する `representative Mir programs の作業報告を追加する`
  - ただし 2 本目の commit はこの report 自身を含むため、self-reference の都合で本文中に最終 hash を固定値で埋め込めない。実 hash は final response と `git log` で補完する。

今回実際に変更したファイル:

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/00-representative-mir-programs.md`
- `docs/reports/0024-representative-mir-programs-current-l2.md`

確認したが今回変更しなかったファイル:

- `README.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
- `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`
- `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`
- `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`
- `docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md`
- `docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md`

## 5. コマンドと正確な出力

1. 作業開始時点の `git status --short --branch`

```text
## main...origin/main
```

2. `find specs -maxdepth 2 -type d | sort`

```text
specs
```

3. `python3 scripts/new_report.py --slug representative-mir-programs-current-l2`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0024-representative-mir-programs-current-l2.md
```

4. `date --iso-8601=seconds`

```text
2026-03-31T03:02:56+09:00
```

5. `git status --short --branch`（report scaffold 生成直後）

```text
## main...origin/main
 M Documentation.md
 M specs/00-document-map.md
?? docs/reports/0024-representative-mir-programs-current-l2.md
?? docs/reports/0025-representative-mir-programs-current-l2.md
?? specs/examples/
```

6. reviewer 結果

```text
no findings
```

7. 仕様本文 commit

```text
ddaeee8 representative Mir programs を追加する
 Documentation.md                                 |   3 +-
 specs/00-document-map.md                         |  15 +
 specs/examples/00-representative-mir-programs.md | 337 +++++++++++++++++++++++
 3 files changed, 354 insertions(+), 1 deletion(-)
```

8. `git diff --check`

```text
<no output>
```

9. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 25 numbered report(s).
```

10. `git status --short --branch`（report commit 前）

```text
## main...origin/main [ahead 1]
?? docs/reports/0024-representative-mir-programs-current-l2.md
```

## 6. 証拠 / 所見

今回追加した representative program が確認する理論:

- 例 1 は `place` 入れ子、authority update、`atomic_cut` 後の rollback 不可を確認する。
- 例 2 は local `try`、current `place` 内 rollback、fallback への接続を確認する。
- 例 3 は valid な fallback / preference chain、edge-local な `documented lineage annotation`、monotone degradation を確認する。
- 例 4 は malformed fallback branch が static rejection に寄ることを確認する。
- 例 5 は underdeclared fallback case が surface-level static error に寄ることを確認する。
- 例 6 は write-after-expiry が current L2 で runtime `Reject` に落ちることを確認する。

書きやすかった点:

- `place` 入れ子
- `atomic_cut` 前後の意味差
- valid / malformed / underdeclared の fallback chain 区別
- write-after-expiry の最小読解

書きづらかった点:

- `try` と `fallback` の最終表面構文
- option 宣言に必要な declared information をどの token で inline 化するか
- `perform` が option chain をどの surface syntax で参照するか
- trace / audit をコード内にどこまで書き込むべきか

## 7. 今回整理して分かったこと

- current L2 は、説明用表記であれば representative program を 6 例程度までは自然に書ける。
- 特に fallback / preference chain まわりは、理論自体よりも**surface syntax の不足**が書きづらさの主因である。
- `place` 入れ子と `atomic_cut`、local `try` は、意味論の説明としてはすでにかなり安定している。
- 一方で option chain の宣言と effect request の接続は、専用 sugar が未決のため、例示時にも説明文で補う必要がある。
- `code_mapper` の要約が取れなくても、task-start dirty state と置き場所確認はローカル inspection で十分に補えた。一方で reviewer の `no findings` が取れたことで、examples の説明用記法と static / dynamic 区別が existing L2 を壊していないことを最後に再確認できた。

## 8. 未解決事項

- `perform` と option chain 参照の最終 surface syntax
- `try` / `fallback` の最終表面構文
- `documented lineage annotation` の最終 keyword / punctuation / serialization
- `declared contract surface` / `declared capability surface` の inline 表記をどこまで最小 syntax に含めるか
- representative program 集を parser test case 集へ将来どう落とすか

## 9. Suggested next prompt

```text
あなたはこのリポジトリに初めて入る CodeX です。会話の過去文脈は信用せず、リポジトリ内の文書を正本として扱ってください。

今回は、representative Mir programs 文書で見えた surface syntax の穴のうち、
`perform` と option chain 参照の最小表面構文だけを整理してください。
目的は、valid な fallback / preference chain 例と write-after-expiry 例を、説明用ではなくより安定した current L2 の最小表記に近づけることです。

- Mir-0 / Mir-1 / Mirrorea の境界は変更しないでください。
- canonical law / rejection phase / static evidence floor / underdeclared handling は変更しないでください。
- 新しい capability theory や parser 実装には進まないでください。
- `specs/04-mir-core.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md`、`specs/examples/00-representative-mir-programs.md` を読んでから始めてください。
- 毎回新しい report を `docs/reports/` に追加してください。
```
