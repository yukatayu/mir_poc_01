# 0220 — try-rollback malformed pattern slot selection

## Objective

future dedicated AST structural helper の two-fixture first tranche に対して、`TryFallback` slot と `AtomicCut` slot に最初に入れる malformed pattern を docs-only で narrow に比較し、actual first tranche へ進める working assumption を固定する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- dedicated AST structural helper は future option であり、今回は docs-only comparison だけを扱う。
- actual helper 実装、fixture schema actualization、malformed fixture actual追加は行わない。
- `TryFallback` / `AtomicCut` runtime representative `E2` / `E21` / `E22` は current evidence として維持する。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/04-current-l2-step-semantics.md`
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`
- `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md`
- `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md`
- `specs/examples/57-current-l2-try-rollback-ast-helper-expected-field-name.md`
- `specs/examples/64-current-l2-try-rollback-malformed-static-family-timing.md`
- `specs/examples/65-current-l2-try-rollback-ast-helper-first-tranche-cut.md`
- `specs/examples/66-current-l2-try-rollback-malformed-static-tranche-size.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `scripts/current_l2_detached_loop.py`

## Actions taken

1. current typed fixture schema で decode 可能か、loader/source placement judgement と衝突しないか、runtime representative と競合しないかを軸に `TryFallback` / `AtomicCut` slot の候補を比較した。
2. `specs/examples/67-current-l2-try-rollback-malformed-pattern-slot-selection.md` を追加し、first tranche working pair を
   - empty `fallback_body` -> `missing_fallback_body`
   - `fallback_body` placement の `AtomicCut` -> `disallowed_fallback_placement`
   に絞った。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/15`、`plan/90`、`progress.md` を更新した。

## Files changed

- `specs/examples/67-current-l2-try-rollback-malformed-pattern-slot-selection.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0220-try-rollback-malformed-pattern-slot-selection.md`
- `docs/reports/0221-review-try-rollback-malformed-pattern-slot-selection.md`

## Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 221 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

- local comparison の要点
  - `TryFallback` slot では field absence は loader malformed へ落ちるため、semantic structural malformed としては empty `fallback_body` が最小である
  - `AtomicCut` slot では `disallowed_nesting` は `E22` runtime representative と衝突し、non-statement shape は loader malformed に寄りすぎる
  - `fallback_body` placement は current typed schema で decode 可能で、runtime representative と競合しない first-tranche working assumption として最も narrow である

## What changed in understanding

- dedicated AST structural helper の first tranche を actualize するなら、slot selection まで含めた最小 working pair を current docs-only で固定してよい段階まで来ている。
- ただし `AtomicCut` side の `disallowed_fallback_placement` は final taxonomy ではなく、first tranche actualization のための working assumption である。

## Open questions

- `missing_fallback_body` の exact prose wording を empty `fallback_body` case にどう揃えるか。
- `disallowed_fallback_placement` を later generic structural family でどう generalize するか。
- dedicated helper actualization task の exact wrapper / subcommand 名。

## Suggested next prompt

future dedicated AST structural helper の malformed pattern slot selection judgment を前提に、fixture-side expected wording / `finding_kind` をどこまで narrow に固定して actual helper first tranche へ進めてよいかを比較し、可能なら helper code / fixture fields / two malformed fixtures / static gate smoke path まで actualize してください。
