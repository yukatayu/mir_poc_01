# Report 2029 — P-A1-19 alpha05 session runtime carrier

- Date: 2026-05-05
- Author / agent: Codex
- Scope: `P-A1-19` α-0.5 session runtime carrier
- Decision levels touched: `L1` operational category reading, `L2` session-carrier implementation memory / snapshot status

## Objective

`specs/24` と `plan/45/49` で fixed 済みの α-0.5 readiness 条件に対し、same-session carrier を actualize する。ここでは typed host-I/O direct execution はまだ行わず、checked package input -> runtime plan -> run-local -> observe -> save -> load を 1 carrier に束ね、membership / capability / witness negative rows と session-bound event DAG / observer-safe export を runtime surface に落とす。

## Scope and assumptions

- `P-A1-19` は bounded α-0.5 session carrier に限定する。
- stdio builtin を Mir core に入れない。
- local save/load は distributed durable save/load と混同しない。
- same-session hot-plug runtime と typed external host-I/O direct execution lane は後続 package に残す。

## Start state / dirty state

- Start state: `HEAD=2515fb3fe018df9f2e88736213b8bb3a6f7ce6c5`, worktree clean.
- Dirty state at start: none.
- Resource baseline:
  - `df -h .` => `/dev/vda2` 99G total / 30G avail / 69% used
  - `free -h` => 960Mi memory total / 360Mi available / swap 19Gi

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/19-verification-stratification.md`
- `specs/20-cut-save-load-semantics.md`
- `specs/22-observability-devtools-semantics.md`
- `specs/23-typed-external-host-boundary.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `plan/45-operational-alpha05-roadmap.md`
- `plan/49-host-io-and-session-runtime-roadmap.md`
- `samples/practical-alpha1/README.md`
- `scripts/README.md`
- `samples/README.md`

## Actions taken

- Added `required_witness_refs` to the practical local-runtime AST envelope so local witness negatives can stay distinct from offered witness refs.
- Widened `crates/mir-runtime::practical_alpha1_local_runtime` to evaluate capability sufficiency and required-witness presence in addition to membership freshness.
- Added new first-floor runtime fixtures and exact expected reports:
  - `RUN-03` missing capability reject
  - `RUN-04` missing witness reject
- Added `crates/mir-runtime::practical_alpha05_session` plus example `mir_practical_alpha05_session`.
- Implemented `start` / `observe` / `save` / `load` session-carrier flow over the existing local-runtime report.
- Added `scripts/practical_alpha05_session.py` as the alpha-local operational α-0.5 session surface.
- Added Rust / Python tests for widened local-runtime rows and the new session carrier.
- Synced repository memory and snapshot docs to move the reopen point from `P-A1-19` to `P-A1-20`.

## Files changed

- Runtime / AST:
  - `crates/mir-ast/src/practical_alpha1.rs`
  - `crates/mir-runtime/src/lib.rs`
  - `crates/mir-runtime/src/practical_alpha1_local_runtime.rs`
  - `crates/mir-runtime/src/practical_alpha05_session.rs`
  - `crates/mir-runtime/examples/mir_practical_alpha05_session.rs`
- Tests / script surfaces:
  - `crates/mir-runtime/tests/practical_alpha1_local_runtime.rs`
  - `crates/mir-runtime/tests/practical_alpha05_session.rs`
  - `scripts/practical_alpha1_run_local.py`
  - `scripts/practical_alpha05_session.py`
  - `scripts/tests/test_practical_alpha1_run_local.py`
  - `scripts/tests/test_practical_alpha05_session.py`
- Samples / expected evidence:
  - `samples/practical-alpha1/packages/run-03-missing-capability-rejected/package.mir.json`
  - `samples/practical-alpha1/packages/run-04-missing-witness-rejected/package.mir.json`
  - `samples/practical-alpha1/expected/run-01-local-sugoroku.expected.json`
  - `samples/practical-alpha1/expected/run-03-missing-capability-rejected.expected.json`
  - `samples/practical-alpha1/expected/run-04-missing-witness-rejected.expected.json`
- Docs / repository memory:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `samples/README.md`
  - `samples/practical-alpha1/README.md`
  - `scripts/README.md`
  - `plan/45-operational-alpha05-roadmap.md`
  - `plan/49-host-io-and-session-runtime-roadmap.md`
  - `docs/reports/2029-p-a1-19-alpha05-session-runtime-carrier.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
df -h .
free -h
python3 -m unittest scripts.tests.test_practical_alpha1_run_local
python3 -m unittest scripts.tests.test_practical_alpha05_session
cargo run -q -p mir-runtime --example mir_practical_alpha1_run_local -- samples/practical-alpha1/packages/run-03-missing-capability-rejected
cargo run -q -p mir-runtime --example mir_practical_alpha1_run_local -- samples/practical-alpha1/packages/run-04-missing-witness-rejected
cargo run -q -p mir-runtime --example mir_practical_alpha05_session -- start samples/practical-alpha1/packages/run-01-local-sugoroku
python3 scripts/practical_alpha05_session.py check-all --format json
cargo fmt
cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture
cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha05_session.py check-all --format json
python3 -m unittest scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha05_session
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- Initial failing tests:
  - `scripts.tests.test_practical_alpha1_run_local` failed because `RUN-03/04` were absent.
  - `scripts.tests.test_practical_alpha05_session` failed because `practical_alpha05_session` script/module did not exist.
- Runtime widening evidence:
  - `RUN-03` now yields `dispatch_outcome = rejected_missing_capability`, `reason_family = authorization`.
  - `RUN-04` now yields `dispatch_outcome = rejected_missing_witness`, `reason_family = witness`.
  - `RUN-01` exact report was updated to include explicit accepted `reason_refs` for capability and witness checks.
- Session-carrier evidence:
  - `mir_practical_alpha05_session start` emits `session_scope = practical-alpha05-session-floor`.
  - `observer_safe_export.redaction = observer_safe_session_summary`.
  - `savepoint#1` roundtrip stays equality-preserving and `load` restores the same event DAG frontier.
- Focused validation:
  - `cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture` pass (`6` tests)
  - `cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture` pass (`3` tests)
  - `python3 scripts/practical_alpha1_run_local.py check-all --format json` pass (`RUN-01..04`)
  - `python3 scripts/practical_alpha05_session.py check-all --format json` pass (`OA05-01..06`)
  - `python3 -m unittest scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha05_session` pass (`11` tests)
- Repository-wide validation:
  - `python3 -m unittest scripts.tests.test_validate_docs` pass (`11` tests)
  - `python3 scripts/check_source_hierarchy.py` pass (`84/84`)
  - `python3 scripts/validate_docs.py` pass
  - `cargo fmt --check` pass
  - `git diff --check` pass

## What changed in understanding

- Local capability negatives were already expressible with `claimed_capabilities` vs `capability_requirements`.
- Local witness negatives needed a distinct required/offered split; adding `required_witness_refs` at the AST/runtime-evaluation seam was enough and did not require widening the core `MessageEnvelope` carrier.
- α-0.5 session readiness could be advanced honestly without same-session hot-plug by binding local runtime, observer-safe export, and local save/load into one carrier and leaving typed host-I/O as the last explicit gap.

## Open questions

- `P-A1-20` should choose whether the minimal host-I/O family lives on a world package with `alpha_local_host_io_input` or a distinct adapter package consumed by the session carrier.
- The first host-I/O row should likely be `AddOne` for narrower payload semantics, but `EchoText` remains a viable alternative if observation-policy wording becomes clearer with string payloads.

## Suggested next prompt

Continue with `P-A1-20` typed external host-I/O direct execution lane over the new `practical_alpha05_session` carrier, preferably with a minimal `AddOne` adapter family and observer-safe request/receipt export.

## Plan update status

`plan/` 更新済み:
`plan/45-operational-alpha05-roadmap.md` と `plan/49-host-io-and-session-runtime-roadmap.md` を、session carrier actualized / next reopen point = `P-A1-20` へ更新した。

## Documentation.md update status

`Documentation.md` 更新済み:
`RUN-01..04` と `practical_alpha05_session` actualization を追加し、same-session carrier 欠如の記述を削除した。

## progress.md update status

`progress.md` 更新済み:
latest closeout package、reopen point、operational α-0.5 rough progress、validation floor、recent log を `P-A1-19` に同期した。

## tasks.md update status

`tasks.md` 更新済み:
`P-A1-19` を完了済みとして扱い、`P-A1-20` を current promoted reopen point に繰り上げた。

## samples_progress.md update status

`samples_progress.md` 更新済み:
α-0.5 row を `78%` に上げ、`RUN-01..04` と `OA05-01..06` を operational matrix evidence として反映した。

## Reviewer findings and follow-up

- Sub-agent reviewer は未使用。
- Local focused review を実施し、以下を確認した:
  - `RUN-01` accepted row は新 capability/witness checks を含めても exact parity に再固定できる。
  - witness negative は core carrier wideningなしで AST/runtime-evaluation seam に閉じ込められる。
  - session carrier は same-session hot-plug や host-I/O を claim せず、`typed_host_io_claimed = false` を保っている。

## Skipped validations and reasons

None.

## Commit / push status

- Primary package commit: `841773d` (`mirrorea: add alpha05 session runtime carrier`)
- Push status: pushed to `origin/main`

## Sub-agent session close status

No sub-agents were used in this package.
