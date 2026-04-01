# Report 0049 — current L2 step semantics

- Date: 2026-04-01
- Author / agent: Codex
- Scope: current L2 representative examples・AST fixture schema・evaluation state schema を前提にした parser-free 最小 interpreter 用 step semantics
- Decision levels touched: L2

## 1. Objective

current L2 の representative examples を parser なしで実行準備できるようにするため、Program / PlaceBlock / PerformOn / PerformVia / TryFallback / AtomicCut を 1-step 単位でどう進めるかを整理する。
今回の目的は full interpreter 実装ではなく、最小 state carrier をどう遷移させれば E1 / E2 / E3 variant / E6 を説明できるかを明文化することにある。

## 2. Scope and assumptions

- Mir-0 / Mir-1 / Mirrorea の境界は変更しない。
- canonical normalization law、rejection phase、static evidence floor、underdeclared handling、lineage annotation surface form、`perform` / option chain companion notation、`try { ... } fallback { ... }` block form、`contract` semantic-role-only policy、predicate sublanguage、option-local `admit` の runtime reading、trace / audit reading、capability mismatch taxonomy、AST fixture schema、evaluation state schemaは既存判断を維持する。
- parser 文字列 syntax は固定せず、AST fixture schema・evaluation state schema・step semantics の意味構造だけを扱う。
- Rust 側は full interpreter 実装に進まず、今回は spec 側の step semantics 文書化を優先した。`crates/mir-semantics` への型補助追加は現段階では不要と判断した。

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
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/03-current-l2-evaluation-state-schema.md`
- `docs/reports/0046-review-0045-short-rereview.md`
- `docs/reports/0047-current-l2-ast-fixture-schema-and-example-fixtures.md`
- `docs/reports/0048-current-l2-evaluation-state-schema.md`
- `crates/mir-semantics/src/lib.rs`

## 4. Actions taken

1. 指定順で正本を読み直し、current L2 の example / fixture / state schema の責務分離を再確認した。
2. step semantics の対象 node を抽出し、Program / PlaceBlock / PerformOn / PerformVia / TryFallback / AtomicCut を最小対象として整理した。
3. あわせて、E3 / E6 の fixture が `OptionDecl` / `ChainDecl` を含むため、これらを declaration-only runtime no-op node として扱い、`PerformVia` が immutable fixture carrier から canonical order を解決する reading を追加した。
4. 新規補助文書 `specs/examples/04-current-l2-step-semantics.md` を追加し、次をまとめた。
   - static gate と runtime state の分離
   - meta-level control result（`Continue` / `BubbleFailure(kind)` / `Halt`）
   - 各 state component の更新タイミング
   - node ごとの 1-step 規則
   - E1 / E2 / E3 variant / E6 の walkthrough
5. `Documentation.md`、`specs/00-document-map.md`、`specs/examples/02-current-l2-ast-fixture-schema.md`、`specs/examples/03-current-l2-evaluation-state-schema.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` に最小限の導線と mirror を追加した。
6. final reviewer に step semantics 全体を確認してもらい、返答に応じて必要なら最小修正する流れを取った。
7. 仕様本文側の変更を `708d79d current L2 の step semantics を整理する` として commit した。

## 5. Evidence / outputs / test results

### Files changed

- 新規: `specs/examples/04-current-l2-step-semantics.md`
- 更新: `Documentation.md`
- 更新: `specs/00-document-map.md`
- 更新: `specs/examples/02-current-l2-ast-fixture-schema.md`
- 更新: `specs/examples/03-current-l2-evaluation-state-schema.md`
- 更新: `specs/10-open-questions.md`
- 更新: `specs/12-decision-register.md`
- 新規: `docs/reports/0049-current-l2-step-semantics.md`

### Commands run and exact outputs

```bash
git status --short --branch
```

```text
## main...origin/main [ahead 5]
```

`code_mapper` orientation summary:

```text
1. 今回直接触るべき本命は specs/examples/02-current-l2-ast-fixture-schema.md、docs/reports/0047-current-l2-ast-fixture-schema-and-example-fixtures.md、文脈元として specs/examples/00-representative-mir-programs.md と specs/examples/01-current-l2-surface-syntax-candidates.md、fixture 実体として crates/mir-ast/tests/fixtures/current-l2/ 配下です。
2. 現在の git dirty state はなしです。git status --short は空で、worktree は clean です。
3. 新規に足すのが自然なパスは spec なら specs/examples/03-current-l2-step-semantics.md、report なら docs/reports/0048-current-l2-step-semantics.md です。
4. 関係は「representative examples が意味上の期待動作を定義し、AST fixture がそれを parser なしで machine-readable 化し、evaluation state schema が実行時に保持する状態の shape を定義し、step semantics がその state をどう遷移させるかを定義する」です。
```

final reviewer:

```text
completion は返らず、fresh reviewer を close した時点の previous_status は running だった。
```

仕様本文 commit:

```text
708d79d current L2 の step semantics を整理する
```

検証:

```bash
git diff --check
```

```text

```

```bash
python3 scripts/validate_docs.py
```

```text
Documentation scaffold looks complete.
Found 49 numbered report(s).
```

```bash
python3 - <<'PY'
import json
from pathlib import Path
paths = sorted(Path('crates/mir-ast/tests/fixtures/current-l2').glob('*.json'))
for path in paths:
    with path.open() as f:
        json.load(f)
print(f'Loaded {len(paths)} fixture(s).')
PY
```

```text
Loaded 6 fixture(s).
```

### Findings

- current L2 の parser-free 最小 interpreter では、failure 伝播を persisted state field にせず meta-level control result に留めると、evaluation state schema を増やさずに `TryFallback` の catch / rethrow を説明できる。
- `OptionDecl` / `ChainDecl` を runtime no-op に留め、`PerformVia` が immutable fixture carrier から canonical order を解決する reading にすると、evaluation state schema に declaration environment を追加せず E3 / E6 を動かせる。
- E1 では active rollback frame が無いため、`AtomicCut` は frontier state を増やさず event / audit 説明だけを残せば足りる。E2 では rollback frame が active なので、同じ `AtomicCut` 規則を入れれば `restore_snapshot_ref` 更新として frontier を表せる。
- E6 は `lease-expired` を formal subreason、capability mismatch を narrative explanation、最終結果を request-level `Reject` として分離したまま step で読める。
- reviewer completion は返らなかったため、最終確認は local diff inspection と verification 出力に基づいて行った。
- 仕様本文 commit hash は `708d79d` である。report 自身の commit hash は self-reference の都合で本文には固定しない。

## 6. What changed in understanding

- evaluation state schema を先に固定したあとで step semantics を書くと、state field を増やすべき箇所より、meta-level signal として十分な箇所がよく見えるようになった。
- `OptionDecl` / `ChainDecl` は最初、step semantics の対象外に見えたが、E3 / E6 を parser なしで読むには declaration-only node としての扱いを明示しないと `PerformVia` の初期化規則が宙に浮くことが分かった。
- parser-free 最小 interpreter へ進む上では、syntax の未決定よりも、predicate / effect oracle の最小 API と chain exhaustion 時の exact carrier detail の方が次のボトルネックに近い。

## 7. Open questions

- predicate evaluator / effect oracle が `success` / `explicit_failure` / `Reject` をどう返すかという API 形は **未決定**。
- `cursor_stack` / `chain_cursor` / `rollback_stack` / `trace_audit_sink` の final field naming は **未決定**。
- `place_store` snapshot ref の具体表現と serialization は **未決定**。
- detached trace / audit serialization、event id、multi-request scheduler は **未決定**。
- `Approximate` / `Compensate` を parser-free 最小 interpreter の step signal にどの時点で織り込むかは **未決定**。
- declaration environment を immutable fixture carrier lookup のまま保つか、将来独立 state component に切り出すかは **未決定**。

## 8. Suggested next prompt

`specs/examples/04-current-l2-step-semantics.md` を前提に、parser-free 最小 interpreter の predicate / effect oracle API を整理してください。特に `PerformOn` / `PerformVia` がどの入力を oracle に渡し、`success` / `explicit_failure` / `Reject` / non-admissible skip をどう受け取り、E1 / E2 / E3 variant / E6 の step semantics と矛盾しない最小 interface をどう切るかに絞ってください。
