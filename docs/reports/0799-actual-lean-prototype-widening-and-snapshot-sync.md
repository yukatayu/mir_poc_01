# 0799 — actual Lean prototype widening and snapshot sync

## Objective

Lean global toolchain install 後の theorem-side actual execution line を

- `e5-underdeclared-lineage` only
- toolchain-ready but prototype side not yet actualized

という読みから進め、

- `p06-typed-proof-owner-handoff`
- `p07-dice-late-join-visible-history`
- `p08-dice-stale-reconnect-refresh`

まで actual Lean execution を widen した上で、
`plan/` / `specs/` / `Documentation.md` / `progress.md` / `tasks.md` の current reading を同期する。

## Scope and assumptions

- `specs/` を規範正本、`plan/` を repository memory、`docs/reports/` を時系列証跡として扱う。
- final public theorem contract、proof object public schema、final public verifier contract は今回も adoption しない。
- current package は helper-local / repo-local actual execution hardening に留める。
- Lean toolchain は user-authorized default に従い global `elan` stable を用いる。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/examples/516-current-l2-theorem-actual-lean-execution-toolchain-probe-and-reopen-manifest.md`
- `specs/examples/517-current-l2-model-check-public-seam-compression-after-threshold-and-probe.md`
- `specs/examples/518-current-l2-theorem-actual-lean-execution-narrow-probe-after-global-toolchain-install.md`

## Actions taken

1. current theorem-side actual execution widening candidate を manual probe した。
   - 既存 `scripts/current_l2_theorem_lean_stub_pipeline.py` は `p06/p07/p08` を inventory として扱えず、prototype widening helper としては未整備だと確認した。
2. existing runtime support が持つ theorem Lean-stub pilot actualization route を使い、
   `p06/p07/p08` を actual `.lean` file に materialize して `lean` へ渡す focused runtime regression を追加した。
3. representative theorem quartet
   - `e5-underdeclared-lineage`
   - `p06-typed-proof-owner-handoff`
   - `p07-dice-late-join-visible-history`
   - `p08-dice-stale-reconnect-refresh`
   が current actual execution floor に乗ったという reading へ `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、`specs/` を同期した。
4. theorem-side current queue を、
   - environment-conditional reserve
   ではなく
   - actual Lean execution hardening / helper orchestration and broader coverage
   + later mixed gate
   として読み替えた。

## Files changed

- Added: `crates/mir-runtime/tests/current_l2_theorem_actual_lean_execution_prototype_widening.rs`
- Added: `specs/examples/519-current-l2-theorem-actual-lean-execution-representative-prototype-widening.md`
- Added: `docs/reports/0799-actual-lean-prototype-widening-and-snapshot-sync.md`
- Updated:
  - `Documentation.md`
  - `docs/research_abstract/README.md`
  - `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
  - `progress.md`
  - `tasks.md`
  - `plan/00-index.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/10-roadmap-overall.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `plan/90-source-traceability.md`
  - `specs/00-document-map.md`
  - `specs/10-open-questions.md`
  - `specs/11-roadmap-and-workstreams.md`
  - `specs/12-decision-register.md`

## Commands run

```bash
df -h .
free -h
git status --short
source "$HOME/.elan/env" && lean --version && lake --version && elan --version
source "$HOME/.elan/env" && python3 scripts/current_l2_theorem_lean_stub_pipeline.py p06-typed-proof-owner-handoff --artifact-root target/current-l2-theorem-lean-widen --run-label p06-probe
source "$HOME/.elan/env" && python3 scripts/current_l2_theorem_lean_stub_pipeline.py p07-dice-late-join-visible-history --artifact-root target/current-l2-theorem-lean-widen --run-label p07-probe
source "$HOME/.elan/env" && python3 scripts/current_l2_theorem_lean_stub_pipeline.py p08-dice-stale-reconnect-refresh --artifact-root target/current-l2-theorem-lean-widen --run-label p08-probe
source "$HOME/.elan/env" && cargo test -p mir-runtime --test current_l2_theorem_actual_lean_execution_prototype_widening
```

## Evidence / outputs / test results

- Lean toolchain observed on `2026-04-19`:
  - `Lean 4.29.1`
  - `Lake 5.0.0-src+f72c35b`
  - `elan 4.2.1`
- negative evidence:
  - `scripts/current_l2_theorem_lean_stub_pipeline.py` currently rejects `p06/p07/p08` as unknown sample stems
  - therefore the prototype widening helper path is still not unified under that Python pipeline
- positive evidence:
  - `cargo test -p mir-runtime --test current_l2_theorem_actual_lean_execution_prototype_widening`
  - result: `3 passed; 0 failed`
  - covered:
    - `p06-typed-proof-owner-handoff`
    - `p07-dice-late-join-visible-history`
    - `p08-dice-stale-reconnect-refresh`

## What changed in understanding

- theorem-side actual Lean execution is no longer merely a static representative-sample result.
- current actual execution floor now reaches the representative theorem quartet `e5/p06/p07/p08`.
- the next self-driven gap is not tool availability but helper/CLI orchestration and broader coverage policy.
- `current_l2_theorem_lean_stub_pipeline.py` remains fixture-inventory-oriented, so prototype widening currently lives in runtime support/test rather than a unified Python helper path.

## Open questions

- prototype/runtime actual Lean execution を Python helper / CLI まで揃えるか、それとも runtime test を principal regression surface に保つか。
- `p09` や runtime-side current-L2 representative sample を theorem actual execution widening に含める pressure があるか。
- final public theorem contract、proof object public schema、final public verifier contract をどの mixed gate 順で reopen するか。

## Suggested next prompt

`current_l2_theorem_lean_stub_pipeline.py` の prototype widening / helper orchestration を actual package として進め、representative theorem quartet actual execution を Python helper 側でも再現できるようにしつつ、progress/tasks/plan/spec snapshot を再同期してください。
