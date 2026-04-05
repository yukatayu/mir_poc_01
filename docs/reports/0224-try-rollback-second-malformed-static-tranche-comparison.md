# 0224 — try-rollback second malformed static tranche comparison

## Objective

`TryFallback` / `AtomicCut` dedicated AST structural helper first tranche actualization 後の next narrow step として、
second malformed static tranche を今すぐ actualize するべきか、それとも comparison だけを先に閉じて
wording / finding family stability comparison へ進むべきかを source-backed に整理する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- first tranche actualization (`e23` / `e24` / helper-local compare / static gate smoke path) は済んでいるものとする。
- current task は docs-only comparison と mirror 更新だけを扱う。
- new fixture / new `finding_kind` / static gate wording 拡張は行わない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md`
- `specs/examples/60-current-l2-try-rollback-ast-helper-shared-carrier-threshold.md`
- `specs/examples/63-current-l2-try-rollback-ast-helper-public-checker-entry-criteria.md`
- `specs/examples/67-current-l2-try-rollback-malformed-pattern-slot-selection.md`
- `specs/examples/68-current-l2-try-rollback-ast-helper-first-tranche-actualization.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0222-try-rollback-ast-helper-first-tranche-actualization.md`
- `crates/mir-semantics/src/lib.rs`
- `scripts/current_l2_try_rollback_structural_checker.py`

## Actions taken

1. first tranche 後の next question を source-backed に棚卸しした。
2. second tranche candidate として扱えるもの / 扱えないものを current docs chain と code anchor から切り分けた。
3. `specs/examples/69-current-l2-try-rollback-second-malformed-static-tranche-comparison.md` を追加し、second tranche comparison は先に閉じるが actual tranche 追加はまだ行わない judgment を固定した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`progress.md` を更新した。

## Files changed

- `specs/examples/69-current-l2-try-rollback-second-malformed-static-tranche-comparison.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
git status --short --branch
```

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
```

- `git diff --check`

```text
<no output>
```

- local inspection で確認したこと
  - `field absence` と raw schema / non-statement malformed は current malformed source placement では loader / decode 側に残る
  - nested place `AtomicCut` mismatch は `E22` runtime-valid contrast として残る
  - current docs chain 上、first tranche を越える concrete decode-valid second-tranche family はまだ source-backed に具体化されていない

## What changed in understanding

- second malformed static tranche comparison 自体は先に閉じる価値がある。
- ただし current source だけでは actualize-ready な second-tranche family がまだ不足しているため、いま行うべき次段は actual tranche 追加ではなく wording / finding family stability comparison である。

## Open questions

- second tranche の concrete decode-valid family を later にどう inventory 化するか。
- `missing_fallback_body` / `disallowed_fallback_placement` の長期 wording をどの反復で固定するか。

## Suggested next prompt

current first tranche actualization と second malformed static tranche comparison close を前提に、`missing_fallback_body` / `disallowed_fallback_placement` の wording と helper-local row family が数回の反復を経ても drift しにくいかを narrow に比較してください。
