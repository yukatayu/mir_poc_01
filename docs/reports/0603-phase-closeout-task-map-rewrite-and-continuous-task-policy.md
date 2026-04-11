# 0603 — phase closeout task-map rewrite and continuous-task policy

## Objective

`tasks.md` を一旦全て書き直し、Phase 1〜5 の self-driven / current-recommendation scope を閉じて Phase 6 前半の compile-ready minimal PoC へ入るまでの task package を、着手順・完了条件・rough estimate つきで再整理する。

あわせて、`progress.md` / `Documentation.md` / relevant `plan/` を mirror 更新し、連続 task 依頼時の intermediate report / Discord `progress` 運用を `AGENTS.md` と repo-local `.docs` に明記する。

## Scope and assumptions

- 規範判断の正本は `specs/` とし、この task では normative spec statement 自体は変更しない。
- 「Phase 完了」は、現 phase の self-driven / current-recommendation scope を閉じ、残件を `user spec required` または heavy future workstream として切り分け終えた状態として読む。
- compile-ready PoC は **current L2 subset の non-production minimal PoC** を指し、production parser / runtime / rich host / final formal toolchain を意味しない。
- `.docs/` は task 開始時点では存在せず、repo-local 運用メモを新設する前提で進めた。
- `plan/90-source-traceability.md` は今回の task では更新不要と判断した。roadmap reading は変えたが、traceability mapping 自体は変えていない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `tasks.md`
- `AGENTS.md`
- `specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md`
- `docs/reports/0347-phase3-self-driven-reopen-threshold-and-research-abstracts.md`
- `Cargo.toml`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-lsp/src/lib.rs`

## Actions taken

1. repository rule に従い、`df -h .` と `free -h` で resource 状況を確認した。
2. Discord skill 運用に従い、`python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .` を実行して task baseline を記録した。
3. required docs / plan / current snapshot を読み直し、Phase 1〜5 closeout と Phase 6 前半 compile-ready gate を一続きの execution map として再設計した。
4. Cargo workspace と key crates を確認し、parser-free PoC compile evidence と public crate placeholder 状態を切り分けた。
5. `tasks.md` を全面書き直し、Phase 1〜5 closeout + Phase 6 front-half compile-ready minimal PoC までの ordered package map、完了条件、rough estimate、late blocker を整理した。
6. `progress.md` を更新し、phase table、current mainline、compile-ready PoC estimate、recent log を新しい snapshot に合わせた。
7. `Documentation.md`、`plan/10`、`plan/11`、`plan/12`、`plan/17` を更新し、Phase 6 前半への橋と late blocker を mirror した。
8. `AGENTS.md` に「連続 task package 依頼では package close ごとに user 中間報告 + Discord `progress` を送る」運用を追加した。
9. repo-local `.docs/continuous-task-policy.md` を新設し、連続 task の checkpoint policy を短く固定した。

### Files changed

- `AGENTS.md`
- `.docs/continuous-task-policy.md`
- `Documentation.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0603-phase-closeout-task-map-rewrite-and-continuous-task-policy.md`

### Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
df -h .
free -h
sed -n '1,220p' README.md
sed -n '1,220p' Documentation.md
sed -n '1,260p' progress.md
sed -n '1,260p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,220p' specs/02-system-overview.md
sed -n '1,220p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,260p' specs/11-roadmap-and-workstreams.md
sed -n '1,260p' plan/00-index.md
sed -n '1,260p' plan/07-parser-free-poc-stack.md
sed -n '1,260p' plan/09-helper-stack-and-responsibility-map.md
sed -n '1,260p' plan/10-roadmap-overall.md
sed -n '1,260p' plan/11-roadmap-near-term.md
sed -n '1,260p' plan/12-open-problems-and-risks.md
sed -n '1,260p' plan/13-heavy-future-workstreams.md
sed -n '1,260p' plan/17-research-phases-and-autonomy-gates.md
sed -n '1,260p' tasks.md
sed -n '1,260p' AGENTS.md
sed -n '70,130p' specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md
sed -n '80,130p' docs/reports/0347-phase3-self-driven-reopen-threshold-and-research-abstracts.md
sed -n '1,220p' Cargo.toml
rg --files -g 'Cargo.toml' crates
ls -1 crates
sed -n '1,220p' crates/mir-semantics/src/lib.rs
sed -n '1,220p' crates/mir-ast/src/lib.rs
sed -n '1,220p' crates/mir-runtime/src/lib.rs
sed -n '1,220p' crates/mir-lsp/src/lib.rs
cargo test -p mir-semantics --test current_l2_minimal_interpreter -- --list
cargo test -p mir-ast --test current_l2_stage1_parser_spike -- --list
cargo test -p mir-ast --test current_l2_stage2_try_rollback_spike -- --list
date '+%Y-%m-%d %H:%M %Z'
```

## Evidence / outputs / test results

- resource snapshot
  - `df -h .`
    - `/dev/vda2 99G 94G 1006M 99% /`
  - `free -h`
    - `Mem: 960Mi total / 715Mi used / 80Mi free / 245Mi available`
- compile / test evidence
  - `cargo test -p mir-semantics --test current_l2_minimal_interpreter -- --list`
    - 46 tests listed
  - `cargo test -p mir-ast --test current_l2_stage1_parser_spike -- --list`
    - 14 tests listed
  - `cargo test -p mir-ast --test current_l2_stage2_try_rollback_spike -- --list`
    - 3 tests listed
- public crate status evidence
  - `crates/mir-semantics/src/lib.rs` は parser-free current L2 minimal interpreter skeleton を保持
  - `crates/mir-ast/src/lib.rs` / `crates/mir-runtime/src/lib.rs` / `crates/mir-lsp/src/lib.rs` は placeholder skeleton

## What changed in understanding

- Phase 3 は current repo の long-term reading では reserve path だったが、**Phase 6 front-half compile-ready PoC を本気で取りに行くなら reopen 必須** と読む方が自然である。
- Phase 4 は current recommendation で self-driven closeout できる範囲と、user-specific final catalog を必要とする範囲を分けて扱うべきであり、後者まで immediate blocker にする必要はない。
- Phase 5 は `126...284` package close で「ほぼ終わり」と見えるが、Phase 6 への橋としては verifier handoff surface と proof / model-check handoff closeout が still necessary である。
- parser-free PoC はすでに compile / test evidence を持つ一方、actual public parser / runtime crates は薄く、**Phase 6 front-half compile-ready PoC は rough 25% 前後** と読むのが妥当である。
- `.docs/` が未作成だったため、連続 task package の checkpoint policy は repo-local note を新設して明文化するのが自然だった。

## Open questions

- actual parser subset を stage 1 / 2 / 3 selected subset に留めるか、request head / richer diagnostics まで同時 actualize するか。
- theorem prover / model-check line の formal tool binding を tool-neutral export で一旦閉じるか、concrete tool first cut を Phase 6 front-half に含めるか。
- Phase 4 完了判定を self-driven current recommendation close で十分とみなすか、final activation / authority / fairness catalog まで要求するか。

## Suggested next prompt

```text
tasks.md の新しい順番に従って、Task 1 の verifier-handoff-surface comparison から自走で進めてください。
連続 task package ごとの close 時点では、AGENTS.md と .docs の新ルールに従って、短い中間報告と Discord progress を入れてください。
```
