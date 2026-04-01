# Report 0052 — review 0051 short re-review

- Date: 2026-04-01
- Author / agent: Codex
- Scope: `0051-current-l2-oracle-api.md` とその mirror (`specs/examples/04-current-l2-step-semantics.md`, `specs/examples/05-current-l2-oracle-api.md`, `specs/10-open-questions.md`, `specs/12-decision-register.md`) に対する短い re-review
- Decision levels touched: L2

## 1. Objective

`0051` で整理した current L2 oracle API の差分に対し、最終 reviewer completion が返っていなかったため、parser-free minimal interpreter 実装に進む前に短い re-review を行い、重大な齟齬があれば最小修正で解消する。

## 2. Scope and assumptions

- Mir-0 / Mir-1 / Mirrorea の境界は変更しない。
- current L2 oracle API の大枠、predicate / effect oracle の分割、`admit-miss` / `lease-expired` / capability mismatch の読みは維持する。
- 今回の re-review は `0051` 由来の via-chain `Reject` 境界だけを確認対象とし、新しい理論語彙は増やさない。
- worktree は review 開始時点で clean だった。

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
- `specs/examples/04-current-l2-step-semantics.md`
- `specs/examples/05-current-l2-oracle-api.md`
- `docs/reports/0051-current-l2-oracle-api.md`

## 4. Actions taken

1. `0051` の short re-review を reviewer に依頼した。
2. reviewer finding を確認し、via-chain の `Reject` 境界が narrow すぎる点を抽出した。
3. 次の mirror を最小修正した。
   - `specs/examples/04-current-l2-step-semantics.md`
   - `specs/examples/05-current-l2-oracle-api.md`
   - `specs/10-open-questions.md`
   - `specs/12-decision-register.md`
4. 修正内容を、`PerformVia` における final-candidate `request-require` failure / admitted-option `explicit_failure` / final-candidate `request-ensure` failure が、well-formed chain exhaustion と同様に request-level `Reject` へ畳まれる、という current L2 reading に揃えた。

## 5. Evidence / outputs / test results

### Files changed

- 更新: `specs/examples/04-current-l2-step-semantics.md`
- 更新: `specs/examples/05-current-l2-oracle-api.md`
- 更新: `specs/10-open-questions.md`
- 更新: `specs/12-decision-register.md`
- 新規: `docs/reports/0052-review-0051-short-rereview.md`

### Commands run and exact outputs

```bash
git status --short --branch
```

```text
## main...origin/main [ahead 9]
```

re-review finding:

```text
High: via-chain Reject boundary is too narrow. A well-formed chain that exhausts without success should end in request-level Reject even when the last admitted candidate fails by request-local require / explicit failure / ensure, not only when exhaustion comes from non-admissible skip.
```

### Findings

- `0051` 本体の分割方針自体は妥当だったが、via-chain `Reject` の説明が non-admissible skip 側に寄りすぎていた。
- current L2 では、well-formed chain が success を返さずに尽きたときの最終 outcome は request-level `Reject` であり、その理由 family は non-admissible skip だけに限定されない。
- この修正は oracle boundary の再設計ではなく、既存 step semantics / oracle API の mirror tightening である。
- この re-review finding を反映した仕様本文 / 実装本文の commit hash は `ef797c3` である。report 自身の commit hash は self-reference の都合で本文に固定しない。

## 6. What changed in understanding

- `Reject` を request-level outcome に留める current L2 方針は維持したままでも、via-chain final failure の folding rule は non-admissible skip 以外まで含めて明記しないと docs 間 drift が起きやすいことが分かった。
- parser-free minimal interpreter に進む前に、この境界を揃えておくことが runtime skeleton の outcome mapping を単純にする。

## 7. Open questions

- via-chain exhaustion の trace / audit explanation を、final `Reject` event 以外にどこまで narrative で細分化するかは **未決定**。
- full scheduler や detached trace serialization に進む段階で、request-level `Reject` folding を event id とどう結びつけるかは **未決定**。

## 8. Suggested next prompt

`specs/examples/04-current-l2-step-semantics.md` と `specs/examples/05-current-l2-oracle-api.md` の via-chain `Reject` 読みを前提に、`crates/mir-semantics` に parser-free minimal interpreter skeleton を実装してください。特に static gate、`EvaluationState`、`step_once` / `run_to_completion`、fixture ベースの最小テストを current L2 representative examples に揃えてください。
