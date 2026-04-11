# 0611 — phase6 actual parser / AST carrier first tranche package

## Objective

Phase 6 front-half の first actualization として、
`mir-ast` public crate を placeholder から一段進め、stage 1 / stage 2 structural floor だけを
non-production parser carrier として compile-ready にする。

## Scope and assumptions

- accepted surface は Phase 3 freeze で fixed 済みの stage 1 option/chain と stage 2 try/fallback structural floor に留める。
- stage 3 admit / request clause / predicate fragment、perform head final public API、span-rich diagnostics、final grammar は fixed しない。
- `plan/13-heavy-future-workstreams.md` は今回の root source ではなく、**plan/13 更新不要** と判断した。
- current next mainline は Phase 6 actual checker / runtime skeleton first tranche に送る。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/79-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md`
- `specs/examples/287-current-l2-minimal-verifier-handoff-surface-ready-minimal-parser-subset-freeze-comparison.md`
- `specs/examples/288-current-l2-minimal-parser-subset-freeze-ready-minimal-parser-subset-freeze-threshold.md`
- `specs/examples/289-current-l2-minimal-parser-subset-freeze-ready-parser-to-checker-reconnect-freeze-comparison.md`
- `specs/examples/290-current-l2-parser-to-checker-reconnect-freeze-ready-minimal-parser-to-checker-reconnect-freeze-threshold.md`
- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
- `crates/mir-ast/tests/support/current_l2_stage2_try_rollback_spike_support.rs`

## Actions taken

1. Task 1 の cut を再確認し、Phase 6 first tranche では full parser ではなく stage 1 / stage 2 structural floor だけを `mir-ast` crate 本体へ昇格するのが current minimum だと整理した。
2. TDD の red として、stage 1 / stage 2 spike tests と support helper を `mir_ast::current_l2` import へ切り替えたうえで `cargo test -p mir-ast --test current_l2_stage1_parser_spike --test current_l2_stage2_try_rollback_spike` を実行し、`unresolved import mir_ast::current_l2` compile failure を確認した。
3. `crates/mir-ast/src/current_l2.rs` を追加し、stage 1 option/chain parser carrier と stage 2 try/fallback structural carrier を non-production module として actualize した。
4. `crates/mir-ast/src/lib.rs` の crate docs を更新し、`current_l2` module を export した。
5. stage 1 / stage 2 support helper は fixture loader / summary helper に narrow 化し、parser 本体は `src/current_l2.rs` を正本にした。
6. reviewer 指摘を受け、stage 2 parser が fallback close 後の余計な `}` を受理しないよう fail-fast を追加し、stage 1 carrier の `lineage_assertion` を required slot に tighten した。あわせて回帰 test を追加した。
7. `specs/examples/299...300`、`Documentation.md`、`progress.md`、`tasks.md`、`plan/09`、`plan/10`、`plan/11`、`plan/12`、`plan/17`、`plan/90`、`specs/00-document-map.md`、`docs/research_abstract/README.md`、`docs/research_abstract/phase6-compile-ready-minimal-actualization.md` を current mainline へ合わせて更新した。

## Files changed

- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
- `crates/mir-ast/tests/support/current_l2_stage2_try_rollback_spike_support.rs`
- `specs/examples/299-current-l2-phase5-proof-protocol-runtime-policy-handoff-closeout-ready-phase6-actual-parser-ast-carrier-first-tranche-comparison.md`
- `specs/examples/300-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-minimal-phase6-actual-parser-ast-carrier-first-tranche-threshold.md`
- `specs/00-document-map.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `docs/reports/0611-phase6-actual-parser-ast-carrier-first-tranche-package.md`

## Commands run and exact outputs

```bash
cargo test -p mir-ast --test current_l2_stage1_parser_spike --test current_l2_stage2_try_rollback_spike
cargo test -p mir-ast
python3 scripts/validate_docs.py
git diff --check
git status --short
```

- `cargo test -p mir-ast --test current_l2_stage1_parser_spike --test current_l2_stage2_try_rollback_spike`
  - red phase では `error[E0432]: unresolved import mir_ast::current_l2`
- `cargo test -p mir-ast`
  - `51 passed; 0 failed`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 610 numbered report(s).`
- `git diff --check`
  - no output
- `git status --short`
  - task close 直前は Task 1 関連の edited/new file が並ぶ uncommitted 状態

## Evidence / findings

- red verification:
  - `cargo test -p mir-ast --test current_l2_stage1_parser_spike --test current_l2_stage2_try_rollback_spike`
  - `error[E0432]: unresolved import mir_ast::current_l2`
- green verification:
  - `cargo test -p mir-ast`
  - `51 passed; 0 failed`
- docs verification:
  - `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 610 numbered report(s).`

## What changed in understanding

- Phase 6 parser first tranche で必要なのは stage 3 widen ではなく、Phase 3 freeze で fixed 済みの stage 1 / stage 2 floor を code anchor 化することだった。
- stage 1 / stage 2 support helper は parser 本体ではなく fixture compare / summary helper に narrow 化した方が、Phase 6 code path と retained-later evidence の境界が明確になる。
- current `mir-ast` first tranche は final parser API ではなく、checker/runtime bridge へ渡す non-production carrier として読むのが自然である。

## Open questions

- perform head の first actual parse を Task 2 に含めるか
- stage 3 request / predicate reconnect を compile-ready checkpoint 後の second tranche に切り出すか
- theorem / model-check formal hook を tool-neutral export で閉じるか、concrete tool first cut を入れるか

## Suggested next prompt

```text
Phase 6 front-half actual checker / runtime skeleton first tranche を進め、mir-semantics の program-level entry と mir-runtime thin orchestrator を actualize してください。
```
