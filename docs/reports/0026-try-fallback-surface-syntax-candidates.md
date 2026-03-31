# 報告 0026 — `try` / `fallback` の current L2 surface syntax 候補

- 作成日時: 2026-03-31T11:22:30+09:00
- 作成者 / agent: Codex
- 対象範囲: representative Mir programs（current L2）で使う `try` / `fallback` の最小 surface syntax 候補、examples の書き直し、最小導線の追加
- 触れた decision level: 既存理論を変えない L2 の companion notation 整理

## 1. Objective

representative Mir programs でまだ説明用 shorthand のまま残っていた `try` / `fallback` の書き方を、`perform` と option chain 参照で前回そろえた companion notation と矛盾しない current L2 の最小 surface syntax 候補へ寄せる。parser 実装には進まず、examples 用の安定した block-form 候補だけを見える形にする。

## 2. Scope and assumptions

- Mir-0 / Mir-1 / Mirrorea の境界は変更しない。
- canonical normalization law、incompatible branch rejection phase、static evidence floor、underdeclared handling、`documented lineage annotation` の最小 surface form は変更しない。
- `perform` と option chain 参照には、前回整理した current L2 companion notation を前提として使う。
- `try` は local rollback semantics を持つ primitive として、current L2 examples では block form のみを候補に置く。
- final parser syntax、reserved keyword、式形式、追加 sugar は **未決定** のまま残す。

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
- `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
- `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`
- `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`
- `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`
- `docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md`
- `docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md`
- `docs/reports/0024-representative-mir-programs-current-l2.md`
- `docs/reports/0025-perform-and-option-chain-surface-syntax-candidates.md`

## 4. Actions taken

- `code_mapper` を使い、task-start の worktree が clean であること、今回の最小 blast radius が `specs/examples/01-current-l2-surface-syntax-candidates.md` と `specs/examples/00-representative-mir-programs.md` を中心にした小さな更新でよいことを確認した。
- representative examples と companion syntax 文書を読み、`try` / `fallback` に関わる現行 shorthand を棚卸しした。
- current L2 の最小候補として、`try` は block form の `try { ... } fallback { ... }` に限定して整理した。
- rollback scope は current `place` の入れ子が与えると読み、`try` 自体には追加の scope 指定句を入れない方針にそろえた。
- `fallback { ... }` を「直前の `try` に後置される explicit branch」と説明し、option chain の `fallback successor` とは構文形で区別することを明文化した。
- `specs/examples/01-current-l2-surface-syntax-candidates.md` に `try` / `fallback` 節を追加し、`atomic_cut` と並べたときの読みも短く補った。
- `specs/examples/00-representative-mir-programs.md` の E2 と総括を更新し、`try { ... } fallback { ... }` を current L2 companion syntax 候補として扱うことを明示した。
- `Documentation.md` と `specs/00-document-map.md` に、`try` / `fallback` 候補書式も companion syntax 文書に含まれることを追記した。
- `specs/10-open-questions.md` に「examples では block-form の `try { ... } fallback { ... }` を使ってよいが、final keyword / punctuation / 式形式は未決定」という最小記録を追加した。
- `specs/12-decision-register.md` に D-031 を追加し、representative examples 用の current L2 companion syntax 候補として位置づけた。
- `reviewer` を起動して spec commit を review させた。結果は後述する。

## 5. Files changed

今回実際に変更したファイル:

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `docs/reports/0026-try-fallback-surface-syntax-candidates.md`

確認したが今回変更しなかったファイル:

- `README.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
- `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`
- `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`
- `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`
- `docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md`
- `docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md`
- `docs/reports/0024-representative-mir-programs-current-l2.md`
- `docs/reports/0025-perform-and-option-chain-surface-syntax-candidates.md`

既存 dirty state と今回の変更の区別:

- 作業開始時点の `git status --short --branch` は `## main...origin/main` で、未コミットの tracked / untracked 変更はなかった。
- したがって今回の差分はすべてこの作業で追加したものである。

## 6. Commands run and exact outputs

1. 作業開始時点の `git status --short --branch`

```text
## main...origin/main
```

2. `python3 scripts/new_report.py --slug try-fallback-surface-syntax-candidates`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0026-try-fallback-surface-syntax-candidates.md
```

3. `git diff --check`

```text
<no output>
```

4. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 27 numbered report(s).
```

5. 仕様本文 commit

```text
[main 65b13d2] current L2 の try と fallback 候補書式を整理する
 6 files changed, 102 insertions(+), 35 deletions(-)
```

6. reviewer 結果

```text
no findings

Residual risk: `fallback` の二つの用法はこの commit では十分に切り分けられていますが、将来 `try` の式形式や追加 sugar を入れるときは、option-chain の canonicalization / lineage evidence rules に参加しないことを再度明記した方が安全です。
```

## 7. Evidence / findings

今回の整理で確認できたこと:

- current L2 の examples では、`try` を block form に限定するだけで local rollback の読みがかなり安定する。
- rollback scope を `place` の入れ子に任せると、`try` に scope 指定句を増やさずに済み、`atomic_cut` との並びも自然に読める。
- `fallback { ... }` を直前の `try` に後置される block branch と説明すれば、option chain の `fallback successor` と token を共有していても、構文形の違いだけで読み分けられる。
- 代表例の E2 は既存のコード形をほぼ維持したまま、説明上だけでなく current L2 companion syntax 候補として扱えるようになった。

current L2 で十分に書けるもの:

- local rollback を伴う `try` の block form
- explicit fallback branch
- `atomic_cut` を含む `try` body の読み
- `perform` / option chain / `try` / `fallback` を組み合わせた same-place representative example

まだ説明用のまま残るもの:

- `try` / `fallback` の最終 keyword
- punctuation の final 形
- `try` の式形式や value-returning form
- richer な fallback sugar
- cross-place 文脈で同じ形を共有するかどうか

## 8. What changed in understanding

- `try` / `fallback` の曖昧さは、意味論不足よりも「companion notation が未整列だったこと」による読みづらさが大きかった。
- option chain の `fallback successor` と local `try` の `fallback { ... }` は、理論上の新語彙を足さなくても、構文形の違いを明示するだけで十分に読み分けられる。
- `atomic_cut` を `try` 専用 syntax にせず ordinary statement として保つ方が、既存の cut semantics を壊さずに representative code を書ける。
- parser 実装前にもう一段詰めるべき surface syntax は、`try` よりむしろ clause punctuation と block nesting 規則の方に寄ってきている。
- reviewer 観点でも `fallback` の二重用法はこの commit では衝突していないと確認できた。残る注意点は、将来 `try` に式形式や sugar を足すときに、option-chain 側の canonicalization / lineage evidence 規則へ誤って巻き込まないことである。

## 9. Open questions

- `try` / `fallback` を最終 reserved keyword にするかどうか。
- `try { ... } fallback { ... }` を唯一の block form とするか、将来 sugar や式形式を許すか。
- `require` / `ensure` の final punctuation と改行規則。
- `atomic_cut` の前後で statement separator をどう見せるか。
- same-place / cross-place で `try` / `fallback` companion notation をどこまで共有するか。

## 10. Suggested next prompt

```text
あなたはこのリポジトリに初めて入る CodeX です。会話の過去文脈は信用せず、リポジトリ内の文書を正本として扱ってください。

今回は、current L2 の representative Mir programs で使っている `require` / `ensure` clause と statement separator の最小 surface syntax 候補を整理してください。
目的は、`perform`、option chain、`try` / `fallback` でそろえた companion notation の上で、clause の改行・区切り・block nesting の読みを安定させることです。

- Mir-0 / Mir-1 / Mirrorea の境界は変更しないでください。
- canonical normalization law、rejection phase、static evidence floor、underdeclared handling、lineage annotation surface form は変更しないでください。
- parser 実装には進まないでください。
- `specs/examples/00-representative-mir-programs.md` と `specs/examples/01-current-l2-surface-syntax-candidates.md` を主対象にしてください。
- 毎回新しい report を `docs/reports/` に追加してください。
```

## commit 記録

- 仕様本文 commit:
  - `65b13d2` `current L2 の try と fallback 候補書式を整理する`
- report commit:
  - この report 自身を含むため、self-reference の都合で本文には固定できない。最終 hash は `git log` と final response で補完する。
