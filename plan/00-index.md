# plan/00 — plan インデックス

## 目的

`plan/` は、この repo の **long-lived repository memory** である。

- 規範判断の正本は `specs/`
- snapshot は `Documentation.md` / `progress.md` / `tasks.md`
- 詳細経緯は `docs/reports/`
- `plan/` は、その間をつなぐ長期参照を保つ

## 先に読む順序

1. `plan/00-index.md`
2. `plan/01-status-at-a-glance.md`
3. `plan/02-system-overview-and-positioning.md`
4. `plan/03-decision-strengths-and-boundaries.md`
5. current-L2 実装寄りなら `plan/04 ... plan/09`
6. 全体計画なら `plan/10-roadmap-overall.md`
7. 直近の research split は `plan/11-roadmap-near-term.md`
8. projection / placement と hot-plug / transport / backend guardrail / avatar slice / typed external executable widening / viewer prototype / public-freeze mixed gate / post-`P18` user-spec hold option inventory / `VerificationLayer` widening threshold の current roadmap は `plan/20`、`plan/21`、`plan/22`、`plan/23`、`plan/24`、`plan/25`、`plan/26`、`plan/27`、`plan/28`、`plan/29`
9. リスクと heavy line は `plan/12`, `plan/13`, `plan/18`

## current repo の短い要約

- current 主眼は Mir current-L2
- authored sixteen と corrected prototype set `p01 ... p16` は runnable
- Problem 1 は
  typed / IFC、theorem-first emitted artifact loop、model-check second-line reserve summary、
  Lean foundation / generated stub acceptance まで repo-local に actualize 済み
- Problem 2 は
  order / handoff / authoritative-room representative pair、reserve route、negative static-stop pair、
  witness / delegated RNG reserve summary まで repo-local に actualize 済み
- ただし final public theorem/model-check contract、final public verifier contract、
  low-level `memory_order` exact surface、final witness/provider public contract、
  packaging / FFI / engine adapter は still later

## current reading

- **repo-local near-end**:
  representative bundle と reserve summary index を辿れば、二大問題の current cut を実行付きで確認できる
- **not final public**:
  concrete tool brand、final shared contract、public API、exact low-level source surfaceはまだ採っていない

## この index からどこへ行くか

- concise status:
  `plan/01-status-at-a-glance.md`
- current-L2 実装面:
  `plan/07-parser-free-poc-stack.md`
  `plan/08-representative-programs-and-fixtures.md`
  `plan/09-helper-stack-and-responsibility-map.md`
- roadmap:
  `plan/10-roadmap-overall.md`
  `plan/11-roadmap-near-term.md`
- projection / placement / hot-plug / transport:
  `plan/20-projection-and-placement-roadmap.md`
  `plan/21-hotplug-attachpoint-roadmap.md`
  `plan/22-network-transport-roadmap.md`
  `plan/23-compiler-backend-llvm-guardrail-roadmap.md`
  `plan/24-avatar-follow-representative-slice-roadmap.md`
  `plan/25-typed-external-boundary-executable-roadmap.md`
  `plan/26-visual-debugger-viewer-roadmap.md`
  `plan/27-public-api-parser-gate-roadmap.md`
  `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
  `plan/29-verification-layer-widening-threshold.md`
- twin peaks の detailed memory:
  `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## maintenance rule

`plan/` は scratchpad ではない。
決定、未決、仮説、履歴 / comparison を混ぜずに書く。
