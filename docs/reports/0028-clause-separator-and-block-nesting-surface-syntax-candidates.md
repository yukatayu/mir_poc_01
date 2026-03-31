# 報告 0028 — clause / separator / block nesting の current L2 surface syntax 候補

- 作成日時: 2026-03-31T11:56:14+09:00
- 作成者 / agent: Codex
- 対象範囲: representative Mir programs（current L2）で使う `contract` / `require` / `ensure`、statement separator、block nesting の最小 companion notation の確認と最小整流
- 触れた decision level: 既存理論を変えない L2 の companion notation 整理

## 1. Objective

`perform`、option chain 参照、`try` / `fallback` でそろえた current L2 companion notation の上で、`contract` / `require` / `ensure`、statement separator、block nesting の読みを安定させる。今回は parser 実装には進まず、既存 repo がすでに持っている最小候補を確認し、その上で representative examples 側の説明を tightening する。

## 2. Scope and assumptions

- Mir-0 / Mir-1 / Mirrorea の境界は変更しない。
- canonical normalization law、rejection phase、static evidence floor、underdeclared handling、lineage annotation surface form、`perform` / option chain の companion notation、`try` / `fallback` の block formは変更しない。
- current L2 の clause syntax 候補は、repo 正本に従い、`contract` を独立 surface keyword にせず、`require` / `ensure` を indented な statement-local clause として直前の `perform` に付ける最小読解を採る。
- blank line は readability の補助であり、意味論 token とはしない。
- final parser syntax、reserved keyword、final punctuation、explicit separator token は **未決定** のまま残す。

## 3. Documents consulted

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
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `docs/reports/0024-representative-mir-programs-current-l2.md`
- `docs/reports/0025-perform-and-option-chain-surface-syntax-candidates.md`
- `docs/reports/0026-try-fallback-surface-syntax-candidates.md`

## 4. Actions taken

- task-start 状態を確認したところ、worktree には tracked な未コミット変更はなかったが、未記入の untracked report scaffold `docs/reports/0027-clause-separator-and-block-nesting-surface-syntax-candidates.md` が 1 本だけ存在していた。
- `code_mapper` を最初に起動したが、このセッションでは要約を回収できなかったため、指定順の文書読解と `git status` / `rg` / `nl` によるローカル inspection を代替手段として使った。
- repo 正本を読み直した結果、`specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` には、今回必要な clause / separator / block nesting の current L2 companion notation がすでに入っていることを確認した。
- そのため今回は理論や companion syntax の中心方針を追加変更せず、representative examples 側で次の tightening だけを入れた。
  - `require` / `ensure` は blank line を挟まない同一 clause suite として読むこと
  - clause suite は直前の `perform` にだけ属し、dedent や次の statement head で終わること
  - `try` / `fallback`、option chain request、write-after-expiry 例でも clause attachment の読みが同じであること
- `specs/examples/00-representative-mir-programs.md` の説明文を更新し、E1 / E2 / E3 / E6 の静的読解に clause attachment の説明を追加した。
- `docs/reports/0028-clause-separator-and-block-nesting-surface-syntax-candidates.md` を新規作成し、この作業の証跡をまとめた。
- `reviewer` を起動して spec commit を review させた。結果は後述する。

## 5. Files changed

今回実際に変更したファイル:

- `specs/examples/00-representative-mir-programs.md`
- `docs/reports/0028-clause-separator-and-block-nesting-surface-syntax-candidates.md`

確認したが今回変更しなかったファイル:

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
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `docs/reports/0024-representative-mir-programs-current-l2.md`
- `docs/reports/0025-perform-and-option-chain-surface-syntax-candidates.md`
- `docs/reports/0026-try-fallback-surface-syntax-candidates.md`

既存 dirty state と今回の変更の区別:

- 作業開始時点の `git status --short --branch` は、branch が `ahead 2` であることに加えて、未記入の untracked scaffold `docs/reports/0027-clause-separator-and-block-nesting-surface-syntax-candidates.md` を示していた。
- tracked な未コミット変更はなかった。
- その `0027` scaffold は空テンプレートだったため、既存 dirty state として記録しつつ、本作業の最終成果物には含めない。
- 以後の実差分は、`specs/examples/00-representative-mir-programs.md` の tightening と、この `0028` report だけである。

## 6. Commands run and exact outputs

1. task-start の `git status --short --branch`

```text
## main...origin/main [ahead 2]
?? docs/reports/0027-clause-separator-and-block-nesting-surface-syntax-candidates.md
```

2. task-start dirty state の内容確認

```text
# Report 0027 — clause separator and block nesting surface syntax candidates

- Date: 2026-03-31T02:49:35.761518Z
- Author / agent:
- Scope:
- Decision levels touched:
```

3. `python3 scripts/new_report.py --slug clause-separator-and-block-nesting-surface-syntax-candidates`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0028-clause-separator-and-block-nesting-surface-syntax-candidates.md
```

4. `git diff --check`

```text
<no output>
```

5. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 27 numbered report(s).
```

6. 仕様本文 commit

```text
[main 9596d97] clause と separator の読みを代表例で明確化する
 1 file changed, 5 insertions(+), 3 deletions(-)
```

7. reviewer 結果

```text
no findings

`9596d97` は、代表例の読みを `specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` に既にある current L2 companion notation へ寄せた変更に留まっており、`contract` の独立 keyword 化や未決 parser syntax の固定には踏み込んでいない。

Residual risk:
`specs/examples/00-representative-mir-programs.md` の説明密度が上がったぶん、今後 `contract { ... }` block form や explicit separator token を検討するときは、`specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` と同時更新しないと、代表例側だけが先に強く見える drift は起こりうる。
```

## 7. Evidence / findings

今回確認できた current L2 の最小候補:

- `contract` は current L2 companion notation では独立 surface keyword にしない。
- `require` / `ensure` は、直前の `perform` に付く indented な statement-local clause として読む。
- clause の既定順は `require` を先、`ensure` を後に置く。
- `perform`、`atomic_cut`、`option`、`chain` は ordinary statement line、`place` / `try` / `fallback` は brace-delimited block head として読む。
- statement separator に dedicated token は要求せず、block structure、dedent、行頭 keyword、blank line による視認補助で最小の読みを与える。

今回の tightening で補えたこと:

- E1 で、`update_authority` に付く clause suite が `atomic_cut` の前で終わることを説明できるようになった。
- E2 で、`try` body 内の各 `require` が fallback branch へ共有されないことを明示できた。
- E3 / E6 で、`via chain_ref` request に付く `require` も同じ statement-local clause として読めることを示せた。

current L2 で十分に書けるもの:

- `perform` + statement-local `require` / `ensure`
- `place` / `try` / `fallback` / `atomic_cut` を混ぜた same-place examples
- option chain request と request-local clause の読み

まだ未決で残るもの:

- `contract` を最終的に surface keyword にするかどうか
- `contract { require { ... } ensure { ... } }` block form を導入するかどうか
- `require` / `ensure` の final punctuation
- blank line 以外の explicit separator token を導入するかどうか
- richer な option-local contract surface

## 8. Changes in understanding

- 今回の主な発見は、新しい clause theory を足す必要はなく、repo 正本にはすでに current L2 の最小 clause / separator 読解が入っていたという点である。
- 不安定だったのは理論本体よりも、representative examples 側で clause attachment の説明がやや薄かったことだった。
- `contract` を semantic role に留め、surface syntax では `require` / `ensure` の inline clause に絞る方が、現時点の repo 読解と矛盾せず、`atomic_cut` や `try` / `fallback` の block 読みも壊さない。

## 9. Open questions

- `contract` を final reserved keyword にするかどうか。
- `contract { ... }` block を current inline clause から昇格させる必要が本当にあるか。
- `require` / `ensure` の punctuation と複数行 predicate をどう表すか。
- clause attachment を将来 option-local contract surface とどこまで共有させるか。
- explicit separator token を parser-ready syntax に導入する必要があるか。

## 10. Suggested next prompt

```text
あなたはこのリポジトリに初めて入る CodeX です。会話の過去文脈は信用せず、リポジトリ内の文書を正本として扱ってください。

今回は、current L2 の representative Mir programs で still open な `contract` の表記方針だけに絞って、
`contract` を semantic role のまま維持するか、それとも `contract { ... }` block form を companion notation に昇格させる価値があるかを整理してください。

- Mir-0 / Mir-1 / Mirrorea の境界は変更しないでください。
- parser 実装には進まないでください。
- `specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` を主対象にしてください。
- 毎回新しい report を `docs/reports/` に追加してください。
```

## commit 記録

- 仕様本文 commit:
  - `9596d97` `clause と separator の読みを代表例で明確化する`
- report commit:
  - この report 自身を含むため、self-reference の都合で本文には固定できない。最終 hash は `git log` と final response で補完する。
