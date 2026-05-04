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
8. projection / placement と hot-plug / transport / backend guardrail / avatar slice / typed external executable widening / viewer prototype / public-freeze mixed gate / post-`P18` user-spec hold option inventory / `VerificationLayer` widening threshold / `AttachPoint` minimal contract / `FAIRY-05` carrier bundling / hot-plug real migration rollback boundary / runtime-crate hot-plug engine ownership cut / runtime-crate hot-plug carrier admission cut / post-`P20` historical bridge / post-`P21` rollback-durable-migration boundary family / post-`P21` distributed-activation-ordering boundary family / post-`P21` final-public-hotplug-ABI boundary family / alpha-local type-system freeze / layer compatibility freeze / cut-save-load checkpoint / runtime package avatar policy / Mirrorea Spaces alpha E2E / practical alpha-1 / operational α-0.5 / α-0.8 / α-0.9 / host-I/O and session runtime の repository-memory roadmap は `plan/20`、`plan/21`、`plan/22`、`plan/23`、`plan/24`、`plan/25`、`plan/26`、`plan/27`、`plan/28`、`plan/29`、`plan/30`、`plan/31`、`plan/32`、`plan/33`、`plan/34`、`plan/35`、`plan/36`、`plan/37`、`plan/38`、`plan/39`、`plan/40`、`plan/41`、`plan/42`、`plan/43`、`plan/44`、`plan/45`、`plan/46`、`plan/47`、`plan/48`、`plan/49`
9. リスクと heavy line は `plan/12`, `plan/13`, `plan/18`

## current repo の短い要約

- current 主眼は Mir current-L2
- active base corpus は `samples/current-l2/`、active canonical executable suite は `samples/clean-near-end/` に置く
- pre-clean-near-end の authored / corrected prototype set `p01 ... p16` は historical comparison memory であり、current active runner floor ではない
- Problem 1 は
  typed / IFC、theorem-first emitted artifact loop、model-check second-line reserve summary、
  Lean foundation / generated stub acceptance まで repo-local に actualize 済み
- Problem 2 は
  order / handoff / authoritative-room representative pair、reserve route、negative static-stop pair、
  witness / delegated RNG reserve summary まで repo-local に actualize 済み
- ただし final public theorem/model-check contract、final public verifier contract、
  low-level `memory_order` exact surface、final witness/provider public contract、
  packaging / FFI / engine adapter は still later

## repository snapshot reading

- **repo-local near-end**:
  `samples/current-l2/` の base corpus と、clean near-end active suite へ forward する `current_l2_guided_samples.py` の `list / smoke-all / closeout` compatibility front door / clean near-end closeout を分けて辿れば、二大問題の current cut を確認できる。pre-clean-near-end の representative bundle / reserve summary index は historical memory として読む
- **alpha-local Mirrorea Spaces scaffold**:
  `samples/alpha/` は `specs/13..17` / `plan/39..43` と結びつく phase-indexed scaffold であり、theory-freeze から checker/runtime skeleton へ移る current planning lane を読む。current cut では expected-verdict sidecar 付き skeleton に留め、active runnable root としては扱わない
- **practical alpha-1 line**:
  `specs/18-practical-alpha1-scope.md` と `plan/44-practical-alpha1-roadmap.md` は、current-scope evidence closeout と分離された practical first-floor toolchain line を置く
- **operational alpha line**:
  `specs/19..24` と `plan/45..49` は、α-0.5 / α-0.8 / α-0.9 の operational readiness 条件、proof obligations、session runtime / host-I/O の reopen order を置く
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
  `plan/30-attachpoint-detach-minimal-contract.md`
  `plan/31-fairy05-visibility-return-carrier-bundling.md`
  `plan/32-hotplug-real-migration-rollback-boundary.md`
  `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
  `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
  `plan/35-post-p20-hotplug-next-package-inventory.md`
  `plan/36-post-p21-rollback-durable-migration-family.md`
  `plan/37-post-p21-distributed-activation-ordering-family.md`
  `plan/38-post-p21-final-public-hotplug-abi-family.md`
  `plan/39-type-system-freeze-roadmap.md`
  `plan/40-layer-compatibility-freeze-roadmap.md`
  `plan/41-save-load-checkpoint-roadmap.md`
  `plan/42-runtime-package-avatar-roadmap.md`
  `plan/43-alpha-e2e-roadmap.md`
  `plan/44-practical-alpha1-roadmap.md`
  `plan/45-operational-alpha05-roadmap.md`
  `plan/46-operational-alpha08-roadmap.md`
  `plan/47-operational-alpha09-devtools-roadmap.md`
  `plan/48-theory-freeze-proof-obligations.md`
  `plan/49-host-io-and-session-runtime-roadmap.md`
- twin peaks の detailed memory:
  `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## maintenance rule

`plan/` は scratchpad ではない。
決定、未決、仮説、履歴 / comparison を混ぜずに書く。
