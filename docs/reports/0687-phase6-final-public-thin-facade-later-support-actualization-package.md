# Report 0687 — phase6 final public thin facade later support actualization package

- Date: 2026-04-13T11:18:01Z
- Author / agent: Codex
- Scope: `specs/examples/395...396` による final public parser/checker/runtime thin-facade later support actualization close と、その mirror update / focused runtime ratchet
- Decision levels touched: L2

## 1. Objective

runtime-led thin facade first cut と Rust-side operational wrapper second gate を巻き戻さず、standalone parser/checker/runtime support entry をどこまで later-support cut として actualize するかを固定する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/355...356`
- `specs/examples/363...364`
- `specs/examples/371...372`
- `specs/examples/389...390`
- `specs/examples/393...394`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`

## 3. Actions taken

1. thin facade first cut / CLI second gate / support-only tranche / excluded bucket の current anchor を整理した。
2. `run_current_l2_runtime_skeleton` + `CurrentL2RuntimeSkeletonReport` を chosen support cut に置く comparison / threshold pair `395...396` を追加した。
3. `crates/mir-runtime/tests/current_l2_runtime_skeleton.rs` に direct runtime path と static-stop path の focused ratchet を追加した。
4. snapshot / plan / abstract / FAQ / sample docs を current line `stable malformed capability second reopen actualization comparison` に合わせて更新した。

## 4. Files changed

- `crates/mir-runtime/tests/current_l2_runtime_skeleton.rs`
- `specs/examples/395-current-l2-public-operational-cli-second-later-gate-actualization-comparison-ready-final-public-parser-checker-runtime-thin-facade-later-support-actualization-comparison.md`
- `specs/examples/396-current-l2-final-public-parser-checker-runtime-thin-facade-later-support-actualization-ready-minimal-final-public-parser-checker-runtime-thin-facade-later-support-threshold.md`
- `specs/00-document-map.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `faq_003.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `docs/reports/0687-phase6-final-public-thin-facade-later-support-actualization-package.md`

## 5. Commands run and exact outputs

- `df -h . && free -h`
  - `/dev/vda2 99G 76G 19G 81% /`
  - `Mem: 960Mi total / 786Mi used / 67Mi free / 173Mi available`
- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  - `Task baseline recorded.`
- `cargo test -p mir-runtime --test current_l2_runtime_skeleton`
  - `running 6 tests`
  - `test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `cargo test -p mir-runtime --test current_l2_source_sample_runner`
  - `running 12 tests`
  - `test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 686 numbered report(s).`
- `git diff --check`
  - no output

## 6. Evidence / findings

- current later-support cut は `run_current_l2_runtime_skeleton` と `CurrentL2RuntimeSkeletonReport` に置くのが最小である。
- explicit support input は `Program` / `FixtureHostPlan` / optional `CurrentL2ParserBridgeInput` に留めるのが current cut である。
- `lower_current_l2_fixed_source_text`、semantic/checker core、parser carrier floor は deeper-support bucket に残し、`run_current_l2_source_sample` first public cut と CLI second gate は巻き戻さない。
- focused ratchet では、direct runtime fixture path と static-stop path の両方で parser bridge absent case を固定できた。
- `plan/90-source-traceability.md` は今回の package では更新不要と判断した。

## 7. Changes in understanding

- thin facade first public cut の次段は「parser text を直接 public に見せること」ではなく、「semantic `Program` を明示入力に取る support entry を narrow に actualize すること」である。
- `run_current_l2_runtime_skeleton` は source-sample runner を巻き戻さずに support-only visibility を与えられる最小 cut であり、`lower_current_l2_fixed_source_text` を同時昇格させるより drift が少ない。

## 8. Open questions

- `lower_current_l2_fixed_source_text` を later support へ昇格させる必要があるか
- explicit `FixtureHostPlan` coupling を later support でどこまで保つか
- public operational CLI concrete shell naming と host-plan shell concern をどこで接続するか

## 9. Suggested next prompt

`tasks.md` 先頭の `stable malformed capability second reopen actualization comparison` を、そのまま次の package として自走してください。
