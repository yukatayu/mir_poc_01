# Report 0035 — current L2 option-local declared contract surface review

- Date: 2026-03-31T06:19:19.419572Z
- Author / agent: Codex
- Scope: 未コミット差分のうち、`specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/examples/00-representative-mir-programs.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` に限った review
- Decision levels touched: L2

## 1. Objective

current L2 の option-local declared contract surface 候補について、未コミット変更だけを対象に spec review を行う。
確認観点は次の 4 点に限定した。

- canonical normalization law / rejection phase / static evidence floor / underdeclared handling / predicate sublanguage / contract semantic-role-only policy を壊していないか
- `admit` が option-local admission-side metadata 以上の新理論に見えていないか
- statement-local `require` / `ensure` の clause attachment policy と衝突していないか
- option-local outcome guarantee や final parser syntax を決め打ちしていないか

## 2. Scope and assumptions

- AGENTS.md の必読順に従い、`README.md`、`Documentation.md`、`specs/00` から `03`、`specs/09`、必要な subsystem 文書として `specs/04-mir-core.md` を先に確認した。
- review 対象は working tree 上の未コミット差分だけとし、上記 4 ファイル以外の変更内容は finding 対象に含めない。
- line 参照は current working tree の行番号を使う。

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

## 4. Actions taken

1. 必読文書を順に読み、L2 surface syntax と fallback invariants の既存制約を確認した。
2. 指定 4 ファイルの `git diff` を取り、未コミット変更だけを抽出した。
3. `canonical normalization law`、`rejection phase`、`static evidence floor`、`underdeclared`、`predicate sublanguage`、`require` / `ensure`、`admit` などの関連語を横断検索した。
4. 差分行の実番号を確認し、既存の D-025 から D-035 と今回の D-036 候補の整合性を突き合わせた。

## 5. Evidence / outputs / test results

- review は文書差分レビューであり、実装テストは実行していない。
- `git diff -- <4 files>` で確認できた未コミット変更は、`admit` を option-local admission-side metadata として追加する説明、E3 下の比較用 variant、`specs/10` mirror 更新、`specs/12` の D-036 追加に限られていた。
- `specs/12-decision-register.md:39` の D-033 はなお「surface では statement-local `require` / `ensure` だけを使う」と述べている一方、`specs/12-decision-register.md:42` の D-036 は option-local `admit` を surface 候補として追加している。
- `specs/12-decision-register.md:34` の D-028 は、declared contract surface / capability surface が不足して successor 判定できない case を surface-level static error としている。
- それに対し `specs/10-open-questions.md:77`、`specs/examples/01-current-l2-surface-syntax-candidates.md:87`、`specs/examples/00-representative-mir-programs.md:21` は、必要時に `admit` を「追加してよい」とだけ書いており、未記述時の underdeclared static error への接続をその場では明示していない。

## 6. What changed in understanding

`admit` 自体は、admission-side metadata に限定し、option-local outcome guarantee や final parser syntax を未決定に残しているため、理論拡張としてはかなり狭く抑えられている。
一方で、既存の D-033 と D-028 との接続は、今回の差分だけではまだ言い切りが揃っていない。

## 7. Open questions

- D-033 の「surface では statement-local `require` / `ensure` だけを使う」を、request-local clause policy の話に狭め直すべきか。
- `admit` が必要な case で省略された場合は underdeclared static error になる、と mirror 文書側でも明示するべきか。

## 8. Suggested next prompt

current L2 の option-local `admit` 候補について、D-033 と D-036 の言い回しを衝突しない形に直しつつ、`admit` 省略時に D-028 の underdeclared static error へ落ちることを `specs/10` と examples mirror にも明示してください。

## 9. Findings

### Medium

- `specs/12-decision-register.md:39` の D-033 は、representative examples の surface を statement-local `require` / `ensure` だけに限定していますが、今回追加の `specs/12-decision-register.md:42` の D-036 は option-local `admit` を同じ companion surface の一部として導入しています。`contract` keyword を立てないという semantic-role-only policy 自体は維持されていますが、「surface では `require` / `ensure` だけ」という既存の L2 判断とは未整理のまま競合しています。D-033 を request-local clause policy に狭めるか、option-local admission metadata を明示的な例外として追記しないと、maintainer 視点では decision register が二重読みに見えます。

### Low

- `specs/10-open-questions.md:77`, `specs/examples/01-current-l2-surface-syntax-candidates.md:87`, `specs/examples/00-representative-mir-programs.md:21` は、capability と `lease` だけで admissibility 差を書けない場合に `admit` を「追加してよい」と記しています。ですが既存の `specs/12-decision-register.md:34` では、declared contract surface / capability surface が不足して successor 判定できない case は underdeclared であり surface-level static error です。今回の差分は underdeclared policy を明示的には覆していませんが、局所文面だけ読むと `admit` が補助的 sugar に見え、必要 case でも省略可能に読めます。必要 case では omission が underdeclared static error になることを、その場でも一言つないだ方が rejection phase と static evidence floor の読みがぶれません。
