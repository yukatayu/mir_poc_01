# Report 0031 — review uncommitted l2 clause punctuation changes

- Date: 2026-03-31T04:57:53.280572Z
- Author / agent: Codex
- Scope: 指定 4 ファイルの uncommitted changes に限定した review
- Decision levels touched: L2 review only

## 1. Objective

`specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/examples/00-representative-mir-programs.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` の未コミット差分だけを対象に、current L2 の既存 theory を壊していないか、`require` / `ensure` punctuation と multi-line predicate の候補が final decision に見えすぎないか、`contract` surface policy・perform attachment・separator / block nesting・`try` / `fallback` policy と矛盾しないか、representative examples が未決事項を決め打ちしていないかを確認する。

## 2. Scope and assumptions

- `AGENTS.md` の指示に従い、`README.md`、`Documentation.md`、`specs/00` から `03`、`specs/09`、`specs/04-mir-core.md` を先に読んだ。
- review 対象は user 指定の 4 ファイルの current uncommitted diff のみとし、既存コミット全体の再レビューは行わない。
- 観点は user 指定の 4 項目に限定し、style-only comment は finding に含めない。
- 規範文書は変更せず、report だけを追加する。

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
- `specs/12-decision-register.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `docs/reports/0030-require-ensure-punctuation-and-multiline-current-l2.md`

## 4. Actions taken

1. 必読順に基礎文書と Mir Core の current L2 解釈を確認した。
2. 指定 4 ファイルの `git diff` を取り、今回の uncommitted changes だけを抽出した。
3. 変更箇所の前後文脈を行番号付きで読み、`contract` surface policy、clause attachment、separator / block nesting、`try` / `fallback`、fallback chain の既存記述と照合した。
4. finding の有無を severity 順で整理し、report に記録した。

## 5. Files changed

- 変更したファイル:
  - `docs/reports/0031-review-uncommitted-l2-clause-punctuation-changes.md`
- 確認したが変更しなかったファイル:
  - `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - `specs/examples/00-representative-mir-programs.md`
  - `specs/10-open-questions.md`
  - `specs/12-decision-register.md`

## 6. Commands run and exact outputs

- `git status --short -- specs/examples/01-current-l2-surface-syntax-candidates.md specs/examples/00-representative-mir-programs.md specs/10-open-questions.md specs/12-decision-register.md`
  - ` M specs/10-open-questions.md`
  - ` M specs/12-decision-register.md`
  - ` M specs/examples/00-representative-mir-programs.md`
  - ` M specs/examples/01-current-l2-surface-syntax-candidates.md`
- `git diff -- specs/examples/01-current-l2-surface-syntax-candidates.md specs/examples/00-representative-mir-programs.md specs/10-open-questions.md specs/12-decision-register.md`
  - diff を取得し、追加点は `require:` / `ensure:` block 候補、関連説明、E1/E2/E3 の example 更新、open question / decision register の mirror 追加であることを確認した。
- `nl -ba specs/examples/01-current-l2-surface-syntax-candidates.md | sed -n '28,150p'`
  - clause form、separator / block nesting、`try` / `fallback` 節の行番号を確認した。
- `nl -ba specs/examples/00-representative-mir-programs.md | sed -n '10,210p'`
  - E1、E2、E3 の変更箇所と説明文の行番号を確認した。
- `nl -ba specs/10-open-questions.md | sed -n '64,92p'`
  - current L2 companion notation の未決扱いを確認した。
- `nl -ba specs/12-decision-register.md | sed -n '30,48p'`
  - D-032 から D-034 の wording と未決の残し方を確認した。

## 7. Evidence / outputs / test results

### Finding 1

- Severity: medium
- File: `specs/examples/00-representative-mir-programs.md`
- Line: 168
- 内容:
  - E3 が `require:` に続けて単一行の `read` だけを置いており、同じ change set で導入した「multi-line predicate が必要な場合だけ `require:` / `ensure:` を使う」という companion rule と食い違っている。
  - 根拠:
    - `specs/examples/01-current-l2-surface-syntax-candidates.md:45` は、colon-headed form を multi-line predicate が必要な場合だけ使うと述べている。
    - `specs/10-open-questions.md:75` と `specs/12-decision-register.md:40` も同じ整理を mirror している。
  - 影響:
    - representative example 側で colon-headed form が bare form の単なる別書き方に見え、`require` / `ensure` punctuation 候補が必要以上に固定されたように読まれる。
    - user の観点 2 と 4 に直接触れており、「multi-line 用だけ」という制約を example が弱めてしまう。
  - 推奨:
    - E3 は `require read` に戻すか、colon-headed form を残すなら predicate を実際に複数行へ折り返した例にする。

### 追加確認

- `contract` semantic-role-only policy、perform attachment、separator / block nesting、`try` / `fallback` policy との整合性は、上の E3 を除けば今回の差分範囲では保たれている。
- `specs/10-open-questions.md:76` と `specs/12-decision-register.md:40` は final parser punctuation、predicate block 内の boolean expression grammar、blank line 許可を未決として残しており、decision text 自体は final parser decision には踏み込んでいない。

## 8. What changed in understanding

今回の change set は理論を拡張したというより、examples 用 companion notation をもう一段細かくしたものだった。問題は theory 破壊ではなく、example の 1 箇所が新ルールより緩く見え、editorial に「colon-headed form も常用可」と誤読されうる点に集約される。

## 9. Open questions

- E3 で colon-headed form を残したい意図が、「single-line でも許す別案の併記」なのか、「multi-line only rule の例外」なのか、「単なる見落とし」なのかは、この diff だけでは判定できない。
- E1 の説明文にある「dedent と blank line でその suite が終わる」は従前からの wording だが、separator policy の「blank line は意味論 token ではない」と並べると誤読余地がある。今回は新規 finding には上げなかったが、将来 wording 整理の余地はある。

## 10. Suggested next prompt

`specs/examples/00-representative-mir-programs.md` の E3 を見直し、`require:` を multi-line predicate 専用に保つなら `require read` に戻すか、実際に複数行 predicate の例へ書き換えてください。あわせて E1 の説明文の「blank line で suite が終わる」という表現を、separator policy と誤読しない wording に揃えるべきか確認してください。
