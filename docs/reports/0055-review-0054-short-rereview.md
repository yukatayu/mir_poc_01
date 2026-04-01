# Report 0055 — review 0054 short re-review

- Date: 2026-04-01
- Author / agent: Codex
- Scope: `0054-current-l2-parser-free-interpreter-skeleton.md` と、それに対応する current L2 parser-free interpreter skeleton 導入差分の短い re-review
- Decision levels touched: L2

## 1. Objective

`0054` に対応する parser-free interpreter skeleton 導入部分が、既存の current L2 理論、とくに `current place` 境界と silent shadowing 禁止を壊していないかを短く確認する。

## 2. Scope and assumptions

- review 対象は `0054` が扱う parser-free interpreter skeleton 導入部分に限定する。
- full diff review は行わない。
- 既存の current L2 fixture は通る前提でも、理論境界を壊しうる実装上の読み替えは finding とする。

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
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/03-current-l2-evaluation-state-schema.md`
- `specs/examples/04-current-l2-step-semantics.md`
- `specs/examples/05-current-l2-oracle-api.md`
- `specs/examples/06-current-l2-interpreter-skeleton.md`
- `docs/reports/0054-current-l2-parser-free-interpreter-skeleton.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 4. Actions taken

1. 読了順の基礎文書と current L2 の補助仕様を確認した。
2. `0054` report と対応 commit 範囲を確認した。
3. `crates/mir-semantics/src/lib.rs` の declaration 解決、`PerformVia`、static gate の実装を重点確認した。

## 5. Evidence / outputs / test results

### Files changed

- 新規: `docs/reports/0055-review-0054-short-rereview.md`

### Commands run

```bash
git log --oneline --decorate -n 8
git show --stat --name-only --format=fuller ef797c3
git show --stat --name-only --format=fuller 83ce1c1
sed -n '1,260p' docs/reports/0054-current-l2-parser-free-interpreter-skeleton.md
sed -n '1,260p' specs/examples/04-current-l2-step-semantics.md
sed -n '1,260p' specs/examples/05-current-l2-oracle-api.md
sed -n '1,260p' specs/examples/06-current-l2-interpreter-skeleton.md
sed -n '1,1040p' crates/mir-semantics/src/lib.rs
nl -ba crates/mir-semantics/src/lib.rs | sed -n '696,760p'
nl -ba crates/mir-semantics/src/lib.rs | sed -n '941,1047p'
```

### Findings

- High: `crates/mir-semantics/src/lib.rs:941` と `crates/mir-semantics/src/lib.rs:998` 以降では、`PerformVia` の chain / option 解決が `current place` 可視性ではなくグローバル `HashMap<String, ...>` に落とされ、`insert` の last-write-wins で重複名を静かに上書きします。これは `specs/examples/04-current-l2-step-semantics.md` と `specs/examples/06-current-l2-interpreter-skeleton.md` が要求する「current place から見える immutable fixture carrier から解決する」という読みとずれ、さらに `specs/09-invariants-and-constraints.md` の silent shadowing 禁止にも反します。少なくとも static gate で重複宣言を rejection にし、runtime 解決も place-scoped にしてください。

## 6. What changed in understanding

- current fixture coverage が通っていても、declaration indexing を global name table に潰すと current L2 の `current place` 境界が mirror されないことが明確になった。

## 7. Open questions

- None.

## 8. Suggested next prompt

`crates/mir-semantics/src/lib.rs` の declaration 解決を short fix してください。特に `OptionDecl` / `ChainDecl` の収集と参照を place-scoped にし、少なくとも duplicate name の silent overwrite を static gate で弾くようにしてください。
