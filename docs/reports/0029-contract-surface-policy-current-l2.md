# 報告 0029 — `contract` の current L2 surface policy

- 作成日時: 2026-03-31T13:04:59+09:00
- 作成者 / agent: Codex
- 対象範囲: representative Mir programs と current L2 companion notation における `contract` の表記方針比較、最小の examples / 規範文書更新、report 追加
- 触れた decision level: 既存理論を変えない L2 の companion notation 整理

## 1. Objective

current L2 の representative examples で、`contract` を semantic role のまま維持するか、それとも `contract { ... }` block form を optional sugar として許すかを比較し、examples 用の最小で安定した表記方針を定める。
今回は parser 実装や新しい理論語彙の追加には進まず、surface policy と examples への反映だけを行う。

## 2. Inputs consulted

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
- `docs/reports/0028-clause-separator-and-block-nesting-surface-syntax-candidates.md`

## 3. Actions taken

- `code_mapper` を最初に起動し、task-start の worktree が clean であること、`contract` surface policy の主な記述箇所が `specs/examples/01`、`specs/10`、`specs/12` に集中していることを確認した。
- 指定順に文書を読み、repo 正本ではすでに次の current L2 方針が強く出ていることを確認した。
  - `contract` は semantic role の名前である。
  - examples の surface では statement-local `require` / `ensure` を使う。
  - `contract { ... }` block form は future syntax 候補としては残るが、未決定である。
- 比較対象として次の二案を明示的に置いた。
  1. `contract` は semantic role のまま維持し、surface では statement-local `require` / `ensure` のみを使う。
  2. `contract { ... }` block form を companion notation の optional sugar として許す。
- 代表例 E1、E2、E3 を基準に比較した。
  - E1 では `perform` と `atomic_cut` の間で clause suite の終端を読み分ける必要があり、statement-local clause の方が cut 境界を誤読しにくい。
  - E2 では `try { ... } fallback { ... }` がすでに block nesting を使っているため、`contract` まで block 化すると branch block と clause block が視覚的に競合しやすい。
  - E3 では option chain / `perform ... via ...` の request-local clause を読みたいだけなので、`contract` block を入れても理論上の利益がなく、記法だけが重くなる。
- 比較の結果、current L2 では案 1 を採り、案 2 は future syntax 候補としてのみ残す方が最小で安定だと整理した。
- `specs/examples/01-current-l2-surface-syntax-candidates.md` を更新し、`contract` section に比較結果と current L2 policy を明記した。
- `specs/examples/00-representative-mir-programs.md` を更新し、representative examples では `contract { ... }` block sugar を使わないことを明記した。
- `specs/10-open-questions.md` を更新し、current L2 companion notation では block form を optional sugar としても採用しないことを反映した。
- `specs/12-decision-register.md` に D-033 を追加し、current L2 の `contract` surface policy を decision register に記録した。
- `reviewer` を最後に使い、今回の方針が既存理論を壊していないかを確認した。結果は後述する。

## 4. Files changed

今回実際に変更したファイル:

- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0029-contract-surface-policy-current-l2.md`

確認したが今回変更しなかったファイル:

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0024-representative-mir-programs-current-l2.md`
- `docs/reports/0025-perform-and-option-chain-surface-syntax-candidates.md`
- `docs/reports/0026-try-fallback-surface-syntax-candidates.md`
- `docs/reports/0028-clause-separator-and-block-nesting-surface-syntax-candidates.md`

既存 dirty state と今回の変更の区別:

- 作業開始時点の `git status --short --branch` は `## main...origin/main` で、未コミットの tracked / untracked 変更はなかった。
- したがって今回の差分はすべてこの作業で追加したものである。

## 5. Commands run and exact outputs

1. 作業開始時点の `git status --short --branch`

```text
## main...origin/main
```

2. `python3 scripts/new_report.py --slug contract-surface-policy-current-l2`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0029-contract-surface-policy-current-l2.md
```

3. 仕様本文 commit 前の `git diff --check`

```text
<no output>
```

4. 仕様本文 commit 前の `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 29 numbered report(s).
```

5. 仕様本文 commit

```text
[main 382d566] contract の current L2 表記方針を整理する
 4 files changed, 9 insertions(+), 5 deletions(-)
```

6. reviewer 結果

```text
no findings

Residual risk: this was a read-only document review, so there were no executable tests to run; the only remaining risk is future wording drift if `specs/04-mir-core.md` and the example-side companion docs are edited independently.
```

## 6. Evidence / findings

- current L2 の repo 正本では、比較対象の二案のうち案 1 がすでに自然な既定として使われていた。今回の作業は、その implicit default を明示的な surface policy にしたものと理解するのが正確である。
- `contract` を semantic role に留める案は、次の点で安定している。
  - `require` / `ensure` を直前の `perform` にだけ属する clause suite として一貫して読める。
  - `place` / `try` / `fallback` の brace-delimited block と役割が競合しない。
  - E1 の `atomic_cut`、E2 の `try` / `fallback`、E3 の `via chain_ref` request で同じ読みを維持できる。
- `contract { ... }` block form を optional sugar として許す案は、将来の sugar として検討余地はあるが、current L2 では利得よりも曖昧さが先に立つ。
  - clause attachment を block nesting の見た目で誤読しやすくなる。
  - representative examples では、block を増やしても新しい意味は増えず、companion notation だけが重くなる。
  - parser 実装前の current L2 には、block sugar を許すための十分な punctuation / separator 規則がまだない。
- よって current L2 の方針は次の通りとする。
  - `contract` は semantic role に留める。
  - surface では statement-local `require` / `ensure` のみを使う。
  - `contract { ... }` block form は optional sugar としても現時点では admitted path に入れない。
  - 将来構文候補として残すこと自体は許す。
- reviewer は `no findings` で、この整理が example-side companion notation の範囲に留まり、`contract` を surface keyword へ昇格させていないことを確認した。

## 7. Changes in understanding

- `contract` の論点は、新しい contract 理論の不足ではなく、「examples で block をどこまで増やすべきか」という companion notation の安定性問題だった。
- 既存 repo はすでに D-032 と examples/01 で strong default を持っていたが、「optional sugar としても今は入れない」とまではやや散発的だった。今回それを揃えられた。
- representative examples の読みやすさでは、block を増やすより clause attachment を一本化する方が効くことが確認できた。

## 8. Open questions

- `contract` を将来 final reserved keyword にするかどうか。
- `contract { require { ... } ensure { ... } }` block form を future syntax 候補としてどの段階で再評価するか。
- `require` / `ensure` の final punctuation と explicit separator token をどう設計するか。
- richer な option-local contract surface を current examples に持ち込む場合、statement-local clause とどう切り分けるか。

## 9. Suggested next prompt

```text
あなたはこのリポジトリに初めて入る CodeX です。会話の過去文脈は信用せず、リポジトリ内の文書を正本として扱ってください。

今回は、current L2 の representative Mir programs と companion notation における
`require` / `ensure` の final punctuation と multi-line predicate の最小候補だけを整理してください。

目的は、`contract` を semantic role に留める current L2 policy を前提に、statement-local clause の読みをもう一段安定させることです。

- Mir-0 / Mir-1 / Mirrorea の境界は変更しないでください。
- `contract { ... }` block form は導入しないでください。
- parser 実装には進まないでください。
- `specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/examples/00-representative-mir-programs.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` を主対象にしてください。
- 毎回新しい report を `docs/reports/` に追加してください。
```

## commit 記録

- 仕様本文 commit:
  - `382d566` `contract の current L2 表記方針を整理する`
- report commit:
  - この report 自身を含むため、self-reference の都合で本文には固定できない。最終 hash は `git log` と final response で補完する。
