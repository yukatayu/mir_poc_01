# Report 0259 — current L2 stage 3 admit sequencing and handoff

- Date: 2026-04-06
- Author / agent: Codex
- Decision levels touched: L2

## 1. Objective

stage 3 admit-slot malformed-source first tranche の次段として、

1. request-local clause spillover
2. fixture-side `OptionDecl.admit` handoff

のどちらを先に扱うべきかを比較し、そのうえで
fixture-side `OptionDecl.admit` handoff を current phase でどこまで docs-only に留めるのが最小かを整理する。

## 2. Scope and assumptions

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 first tranche / malformed-source first tranche は維持する。
- current phase では request-local clause parse も predicate fragment parse も still later stage に残す。
- plan/ 更新済み。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
- `specs/examples/83-current-l2-stage3-admit-slot-branch-comparison.md`
- `specs/examples/84-current-l2-stage3-admit-slot-carrier-and-compare-surface.md`
- `specs/examples/85-current-l2-stage3-admit-slot-first-tranche-actualization.md`
- `specs/examples/86-current-l2-stage3-admit-slot-malformed-source-comparison.md`
- `specs/examples/87-current-l2-stage3-admit-slot-malformed-source-first-tranche-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`

## 4. Actions taken

1. `specs/examples/88-current-l2-stage3-admit-next-step-sequencing.md` を追加し、request-local clause spillover と fixture-side `OptionDecl.admit` handoff の sequencing を比較した。
2. stage 3 の次段は request-local clause より先に handoff comparison を扱うのが自然だと整理した。
3. `specs/examples/89-current-l2-stage3-admit-node-handoff-comparison.md` を追加し、direct lowering / helper-local canonicalization / docs-only deferred の 3 案を比較した。
4. current phase の handoff line は docs-only deferred に留める judgment を固定した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/11`、`plan/12`、`plan/90`、`progress.md` を更新した。

## 5. Files changed

- Added `specs/examples/88-current-l2-stage3-admit-next-step-sequencing.md`
- Added `specs/examples/89-current-l2-stage3-admit-node-handoff-comparison.md`
- Updated `Documentation.md`
- Updated `specs/00-document-map.md`
- Updated `plan/07-parser-free-poc-stack.md`
- Updated `plan/11-roadmap-near-term.md`
- Updated `plan/12-open-problems-and-risks.md`
- Updated `plan/90-source-traceability.md`
- Updated `progress.md`
- Added `docs/reports/0259-current-l2-stage3-admit-sequencing-and-handoff.md`

## 6. Evidence / outputs / test results

### Commands run

```bash
git status --short --branch
sed -n '1,240p' specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md
sed -n '1,220p' crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json
rg -n "stage 3|admit-slot|request-local|OptionDecl.admit|handoff|PerformVia" ...
```

### Source-backed observations

- stage 1 handoff line は parser-side opaque slot carrier と fixture-side compatibility anchor を thin lowering で分ける。
- stage 3 first tranche は fixture-side `OptionDecl.admit` への direct lowering を current non-goal にしている。
- stage 3 malformed-source first tranche で request head spillover までは fail-closed に示せている。
- fixture-side `OptionDecl.admit` は already elaborated predicate node であり、stage 1 の `lease` のような string-like compatibility anchor ではない。

## 7. What changed in understanding

- stage 3 の次段 sequencing は request-local clause spillover より先に fixture-side `OptionDecl.admit` handoff line を比較する方が自然である。
- ただし、その handoff line 自体も current phase では actual compare に上げる圧が弱く、predicate fragment boundary が見えるまで docs-only deferred に留めるのが最小である。
- これにより request-local clause line と declaration-side admit line を混ぜずに維持できる。

## 8. Open questions

- request-local `require` / `ensure` spillover を stage 3 later branch の docs-only comparison にどこまで持たせるか。
- predicate fragment boundary の first cut をどの条件で reopen するか。
- stage 3 private helper を public parser API へ昇格させる entry criteria をどこまで narrow に切るか。

## 9. Suggested next prompt

`specs/examples/89-current-l2-stage3-admit-node-handoff-comparison.md` を前提に、次は stage 3 later branch として request-local `require` / `ensure` spillover をどこまで docs-only comparison に持たせるべきかを narrow に比較してください。
