# Phase 0 要約 — repository memory と decision boundary

## この phase の役割

Phase 0 は、Mir の semantics を増やす phase ではなく、**長期研究を壊さずに積み上げるための repo 構造を固定する phase** である。

## 固まった current reading

- `specs/` を規範正本に置く。
- `plan/` を長期の repository memory に置く。
- `docs/reports/` を task 単位の証跡に置く。
- `progress.md` と `tasks.md` は thin snapshot に保つ。
- `L0/L1/L2/L3` の decision level を崩さない。
- open / comparison / future を既成事実にしない。

## source-backed evidence

- `AGENTS.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/12-decision-register.md`
- report numbering / traceability / snapshot update policy

## まだここで決めていないこと

- Mir の final syntax
- full type system
- shared-space final catalog
- public API / backend / upper-layer app target

## 次へ渡したもの

以後の phase はすべて、Phase 0 の boundary を前提にして進む。
この phase があるため、agent が context を失っても `specs/` / `plan/` / `reports/` を読み直せば研究状態を復元できる。
