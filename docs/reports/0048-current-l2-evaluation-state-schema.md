# Report 0048 — current L2 evaluation state schema

- Date: 2026-04-01
- Author / agent: Codex
- Scope: current L2 representative examples と AST fixture schema を前提にした parser なし最小 interpreter 用 evaluation state schema
- Decision levels touched: L2

## 1. Objective

current L2 の representative examples を parser なしで実行準備できるようにするため、最小 interpreter が保持すべき evaluation state schema を整理する。
今回の目的は full interpreter 実装ではなく、AST fixture から runtime carrier へ渡す最小 state component と、E1 / E2 / E3 variant / E6 を説明できる state 粒度を定めることにある。

## 2. Scope and assumptions

- Mir-0 / Mir-1 / Mirrorea の境界は変更しない。
- canonical normalization law、rejection phase、static evidence floor、underdeclared handling、lineage annotation surface form、`perform` / option chain companion notation、`try { ... } fallback { ... }` block form、`contract` semantic-role-only policy、predicate sublanguage、option-local `admit` の runtime reading、trace / audit reading、capability mismatch taxonomy は既存判断を維持する。
- parser 文字列 syntax は固定せず、AST fixture schema と evaluation state の意味構造だけを扱う。
- 前回 `0047` では最終 reviewer completion が返っていなかったため、本作業の冒頭で短い re-review をやり直し、no findings を確認してから本題へ進んだ。

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
- `docs/reports/0046-review-0045-short-rereview.md`
- `docs/reports/0047-current-l2-ast-fixture-schema-and-example-fixtures.md`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-semantics/src/lib.rs`

## 4. Actions taken

1. `0047` の差分に対する短い re-review を実施し、local 確認と reviewer の返答の両方で no findings を確認した。
2. parser なし最小 interpreter に必要な evaluation state component を整理し、新規補助文書 `specs/examples/03-current-l2-evaluation-state-schema.md` を追加した。
3. static gate と runtime state の責務を分離し、`expected_static` は pre-execution gate、`expected_runtime` / `expected_trace_audit` は post-execution oracle であることを明記した。
4. 最小 state component を `cursor_stack`、`place_stack`、`place_store`、`current_request`、`chain_cursor`、`rollback_stack`、`trace_audit_sink`、`terminal_outcome` に絞った。
5. `atomic_cut` frontier は global cut state にせず、current L2 では rollback frame の `restore_snapshot_ref` 更新として表せることを整理した。
6. E1 / E2 / E3 variant / E6 を動かすのに必要な state 粒度を、example ごとに補助文書へ書き下した。
7. 導線と mirror として `Documentation.md`、`specs/00-document-map.md`、`specs/examples/02-current-l2-ast-fixture-schema.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` を最小限更新した。
8. 最終 reviewer から受けた 2 件の指摘に対し、`atomic_cut` frontier は active rollback frame がある場合だけ `rollback_stack` で表すこと、`not_evaluated` は runtime state の terminal value ではなく static gate 側の結果であることを明文化して整合修正した。
9. 仕様本文側の変更を `0bd1c11 current L2 の evaluation state schema を整理する` として commit した。

## 5. Evidence / outputs / test results

### Files changed

- 新規: `specs/examples/03-current-l2-evaluation-state-schema.md`
- 更新: `Documentation.md`
- 更新: `specs/00-document-map.md`
- 更新: `specs/examples/02-current-l2-ast-fixture-schema.md`
- 更新: `specs/10-open-questions.md`
- 更新: `specs/12-decision-register.md`
- 新規: `docs/reports/0048-current-l2-evaluation-state-schema.md`

### Commands run and exact outputs

```bash
git status --short --branch
```

```text
## main...origin/main [ahead 3]
 M Documentation.md
 M specs/00-document-map.md
 M specs/10-open-questions.md
 M specs/12-decision-register.md
 M specs/examples/02-current-l2-ast-fixture-schema.md
?? docs/reports/0048-current-l2-evaluation-state-schema.md
?? specs/examples/03-current-l2-evaluation-state-schema.md
```

`0047` short re-review:

Reviewer completion:

```text
no findings
```

仕様本文 commit:

```text
0bd1c11 current L2 の evaluation state schema を整理する
```

最終 schema review:

```text
- Medium: [specs/examples/03-current-l2-evaluation-state-schema.md#L126] says `atomic_cut` frontier is represented by updating `rollback_stack.restore_snapshot_ref`, but E1 originally did not require `rollback_stack`.
- Low: `not_evaluated` was listed as a `terminal_outcome` value even though malformed / underdeclared fixtures stop at the static gate.
```

上記 2 点は修正後に再 review を行い、次の completion を得た。

```text
no findings
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
Found 48 numbered report(s).
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

- current L2 の representative examples を parser なしで評価するには、surface punctuation を state に持ち込む必要はなく、AST fixture schema と runtime carrier の間を繋ぐ最小 state component だけで足りることを確認した。
- `expected_static` を runtime state から切り離すことで、`malformed` / `underdeclared` fixture を full runtime state へ落とさずに扱える。
- E2 の local rollback には `rollback_stack` と snapshot-capable `place_store` が必須であり、E3 variant / E6 の chain evaluation には `chain_cursor` と request-local `trace_audit_sink` が必須である。
- `atomic_cut` frontier は current L2 では独立 global state にしなくても、rollback frame の restore snapshot 更新として表現できる。
- capability mismatch、`admit-miss`、`lease-expired` の trace / audit 読みは、既存の non-admissible metadata 方針と矛盾せず evaluation state schema に収まる。
- 仕様本文 commit hash は `0bd1c11` である。report 自身の commit hash は self-reference の都合で本文には固定しない。

## 6. What changed in understanding

- AST fixture schema を作った段階では「fixture が machine-readable であること」と「実行時に何を持つべきか」がまだ分離しきれていなかったが、今回の整理で static gate / runtime state / oracle の 3 層に分けると current L2 の責務がかなり明瞭になると分かった。
- `atomic_cut` を evaluation state の独立 top-level field にしなくても、current L2 の representative examples を読むには rollback frame 更新で十分であり、これにより最小 interpreter の state が軽く保てる。
- request-local trace / audit sink は event surface を増やさずに representative examples の説明責務を担えるため、最小 interpreter の初期段階では sink を独立 logger subsystem に分離する必要はない。

## 7. Open questions

- evaluation state の final field naming は **未決定**。
- `place_store` と snapshot ref の具体 layout は **未決定**。
- ambient place frontier を `rollback_stack` と別 field に切り出すかは **未決定**。
- trace / audit sink の detached serialization、event id、multi-request scheduler を current L2 の最小 state に入れるかは **未決定**。
- parser なし最小 interpreter に進む前提は整ったが、step semantics の粒度と predicate evaluator の最小 API は次段で詰める必要がある。

## 8. Suggested next prompt

`specs/examples/03-current-l2-evaluation-state-schema.md` と `crates/mir-ast/tests/fixtures/current-l2/` を前提に、current L2 parser-free 最小 interpreter の step semantics を整理してください。特に、Program / PlaceBlock / PerformOn / PerformVia / TryFallback / AtomicCut を 1-step 単位でどう進めるか、rollback frame と chain cursor と trace_audit_sink をいつ更新するかを、E1 / E2 / E3 variant / E6 を使って最小限に明文化してください。
