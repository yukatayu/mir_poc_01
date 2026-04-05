# Report 0253 — current L2 stage 3 admit-slot branch comparison

- Date: 2026-04-06
- Author / agent: Codex
- Decision levels touched: L2

## 1. Objective

stage 1 parser spike の success-side first trancheと malformed-source first tranche を前提に、
request / admissibility cluster を stage 3 として進めるなら、その最初の sub-cut を何にするべきかを比較する。

## 2. Scope and assumptions

- current L2 の core semantics、parser-free AST fixture schema、runtime semantics は変更しない。
- stage 1 parser spike actualization は維持する。
- この task では docs-only comparison に留め、code actualization は行わない。
- `e3-option-admit-chain` は full-program parse target ではなく contrast reference として読む。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/29-current-l2-first-parser-subset-inventory.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
- `specs/examples/80-current-l2-stage1-parser-spike-first-tranche-actualization.md`
- `specs/examples/82-current-l2-stage1-parser-spike-malformed-source-first-tranche-actualization.md`
- `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## 4. Actions taken

1. `e3-option-admit-chain` を current contrast anchor として再読し、option-local `admit` と `PerformVia` / `require` の同居点を整理した。
2. `specs/examples/83-current-l2-stage3-admit-slot-branch-comparison.md` を追加し、stage 3 branch compare を 3 案で整理した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/90`、`progress.md` を更新した。

## 5. Files changed

- Added `specs/examples/83-current-l2-stage3-admit-slot-branch-comparison.md`
- Updated `Documentation.md`
- Updated `specs/00-document-map.md`
- Updated `plan/11-roadmap-near-term.md`
- Updated `plan/12-open-problems-and-risks.md`
- Updated `plan/90-source-traceability.md`
- Updated `progress.md`
- Added `docs/reports/0253-current-l2-stage3-admit-slot-branch-comparison.md`

## 6. Evidence / outputs / test results

### Direct evidence

- `e3-option-admit-chain.json`
  - option declaration 側に `admit`
  - chain declaration
  - `PerformVia`
  - request-local `require`
  が同居している
- `specs/examples/29-current-l2-first-parser-subset-inventory.md`
  - option-local `admit` と request head / request-local clause は distinct parser cluster として列挙されている
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
  - stage 1 non-goal に option-local `admit` と `perform` cluster が明示されている

### Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
```

### Outputs

- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
- `git diff --check`
  - 無出力

## 7. What changed in understanding

- `e3` を丸ごと request / admissibility cluster へ送ると、option-local admissibility と request / execution head を同時に parser spike へ持ち込みやすい。
- stage 3 の最初の sub-cutとしては、declaration-side `admit` attached slot を `lease` guard slot と対称に切る方が自然である。
- したがって `PerformVia` / request-local `require` / `ensure` は still later stage に残す方が、lexical freeze pressure と role mixing を抑えられる。

## 8. Open questions

- declaration-side `admit` attached slot の carrier naming をどこまで narrow に決めるか。
- stage 3 admit-slot branch でも `admit` の内部を opaque slot に留めるか、minimal predicate fragment まで先行するか。
- `PerformVia` / request-local clause spillover を stage 1 helper に追加で持たせるか、それとも stage 3 compare の後段へ回すか。

plan/ 更新済み:
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`

## 9. Suggested next prompt

`specs/examples/83-current-l2-stage3-admit-slot-branch-comparison.md` を前提に、次は stage 3 admit-slot branch の declaration-side `admit` attached slot の carrier naming と compare surface を narrow に比較し、actual code へ入る直前 cut を source-backed に整理してください。
