# Report 2030 — P-A1-20 typed host-I/O direct execution lane

- Date: 2026-05-05
- Author / agent: Codex
- Scope: `P-A1-20` typed external host-I/O direct execution lane
- Decision levels touched: `L1` operational α-0.5 readiness reading, `L2` bounded session/runtime host-I/O implementation memory / snapshot status

## Objective

`specs/23` と `specs/24` で fixed 済みの α-0.5 readiness 条件に対し、typed external host-I/O direct execution lane を same-session carrier に actualize する。ここでは stdio builtin を Mir core に入れず、checked package input -> runtime plan -> run-local -> observe -> host-I/O -> observe を bounded α-0.5 surface に落とし、event DAG / observer-safe export / save-load frontier と接続したまま `AddOne` の最小 request/receipt demo を確認する。

## Scope and assumptions

- `P-A1-20` は bounded operational α-0.5 line に限定する。
- stdio builtin を Mir core に入れない。
- typed external host-I/O は one minimal `AddOne` adapter family に限定する。
- local save/load は distributed durable save/load と混同しない。
- same-session hot-plug runtime と α-0.9 live/session-bound full devtools export は後続 package に残す。

## Start state / dirty state

- Start state: `HEAD=5e61a7de33a01f5370065087ea542bef8a5caf7e` (`docs: update p-a1-19 report metadata`)
- Dirty state at start:
  - modified: `crates/mir-ast/src/practical_alpha1.rs`
  - modified: `crates/mir-runtime/src/practical_alpha05_session.rs`
  - modified: `scripts/tests/test_practical_alpha05_session.py`
  - untracked: `crates/mir-runtime/src/practical_alpha05_host_io.rs`
- Interpretation:
  these were partial `P-A1-20` in-progress edits left in the same workspace after `P-A1-19` closeout; they were treated as this package’s starting state and completed rather than reverted.
- Resource baseline:
  reused the same-request baseline already taken before `P-A1-19` (`df -h .`, `free -h`).

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/19-verification-stratification.md`
- `specs/20-cut-save-load-semantics.md`
- `specs/23-typed-external-host-boundary.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `plan/45-operational-alpha05-roadmap.md`
- `plan/49-host-io-and-session-runtime-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `scripts/README.md`

## Actions taken

- Completed the practical AST widening for `alpha_local_host_io_input`:
  - host adapter kind
  - typed request / response payload
  - schema / effect row / failure row / authority / observation policy
- Completed `crates/mir-runtime::practical_alpha05_host_io` as a bounded session-side host-I/O lane.
- Exported the new runtime module from `crates/mir-runtime::lib`.
- Widened `mir_practical_alpha05_session` example with `host-io <session-path> <package-path> [session-out]`.
- Widened `practical_alpha05_session` observer-safe export and save/load frontier to retain host-I/O history across save / load.
- Added `OA05-07` sample fixture as one minimal typed external `AddOne` direct execution row.
- Widened `scripts/practical_alpha05_session.py` to run `OA05-07`, assert typed request/response receipts, and lift α-0.5 readiness to `true`.
- Added Rust host-I/O tests and widened Python session tests.
- Synced snapshot docs and repository memory from “α-0.5 host-I/O pending” to “bounded operational α-0.5 actualized; next reopen point = `P-A1-21`”.
- Fixed reviewer-found snapshot/spec drift by aligning `progress.md`, `plan/49`, and `specs/24`.

## Files changed

- Runtime / AST:
  - `crates/mir-ast/src/practical_alpha1.rs`
  - `crates/mir-runtime/src/lib.rs`
  - `crates/mir-runtime/src/practical_alpha05_session.rs`
  - `crates/mir-runtime/src/practical_alpha05_host_io.rs`
  - `crates/mir-runtime/examples/mir_practical_alpha05_session.rs`
- Tests / script surfaces:
  - `crates/mir-runtime/tests/practical_alpha05_host_io.rs`
  - `scripts/practical_alpha05_session.py`
  - `scripts/tests/test_practical_alpha05_session.py`
- Samples:
  - `samples/practical-alpha1/packages/oa05-07-add-one-host-io/package.mir.json`
- Docs / repository memory / normative sync:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `samples/README.md`
  - `samples/practical-alpha1/README.md`
  - `scripts/README.md`
  - `specs/24-operational-alpha05-alpha08-readiness.md`
  - `plan/45-operational-alpha05-roadmap.md`
  - `plan/49-host-io-and-session-runtime-roadmap.md`
  - `docs/reports/2030-p-a1-20-typed-host-io-direct-execution-lane.md`

## Commands run

```bash
git status --short
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/brainstorming/SKILL.md
sed -n '1,220p' /home/yukatayu/dev/mir_poc_01/.agents/skills/discord-report/SKILL.md
python3 -m unittest scripts.tests.test_practical_alpha05_session
cargo fmt
cargo test -p mir-runtime --test practical_alpha05_host_io -- --nocapture
cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha05_session.py check-all --format json
python3 scripts/practical_alpha05_session.py closeout --format json
python3 -m unittest scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha05_session
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
date '+%Y-%m-%d %H:%M:%S %Z'
git rev-parse HEAD
git diff --name-only
git ls-files --others --exclude-standard
```

## Evidence / outputs / test results

- Initial failing test:
  - `python3 -m unittest scripts.tests.test_practical_alpha05_session` failed before implementation because `OA05-07`, `typed_host_io_demo_present`, and `operational_alpha05_ready` were absent.
- Host-I/O runtime evidence:
  - `OA05-07` now yields `adapter_kind = add_one`
  - `request_payload = {"kind":"int","value":41}`
  - `response_payload = {"kind":"int","value":42}`
  - `terminal_outcome = accepted`
  - `session_event_ids_after` includes `host_request#1` and `host_response#1`
  - `observer_safe_export_after_host_io.host_io_events` includes `host_io:AddOne(41)->42`
- Session/save-load evidence:
  - host-I/O history is serialized into the α-0.5 savepoint frontier and restored on load
  - observer-safe export keeps host-I/O receipt summaries after restore
- Operational closeout evidence:
  - `python3 scripts/practical_alpha05_session.py closeout --format json` reports:
    - `sample_count = 7`
    - `passed = OA05-01..07`
    - `typed_host_io_demo_present = true`
    - `operational_alpha05_ready = true`
- Focused validation:
  - `cargo test -p mir-runtime --test practical_alpha05_host_io -- --nocapture` pass (`2` tests)
  - `cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture` pass (`3` tests)
  - `cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture` pass (`6` tests)
  - `python3 scripts/practical_alpha1_run_local.py check-all --format json` pass (`RUN-01..04`)
  - `python3 scripts/practical_alpha05_session.py check-all --format json` pass (`OA05-01..07`)
  - `python3 -m unittest scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha05_session` pass (`12` tests)
- Repository-wide validation:
  - `python3 -m unittest scripts.tests.test_validate_docs` pass (`11` tests)
  - `python3 scripts/check_source_hierarchy.py` pass (`84/84`)
  - `python3 scripts/validate_docs.py` pass
  - `cargo fmt --check` pass
  - `git diff --check` pass

## What changed in understanding

- Once the same-session α-0.5 carrier existed, the minimal host-I/O lane did not need a new top-level runtime family; it could stay as a bounded session-side transform that appends request/response nodes to the existing event DAG.
- For this line, the observer-safe host receipt summary belongs in the same session export surface, not in a separate preview bundle.
- Snapshot docs reached α-0.5 completion faster than the cited spec text; keeping `specs/24` in sync is necessary for source-hierarchy correctness once operational status changes.

## Open questions

- No blocking open question remains for bounded α-0.5 closeout.
- Optional later widening:
  `EchoText` can still be added as a broader host family row, but it is not required for this package.

## Suggested next prompt

Continue with `P-A1-21` α-0.8 same-session hot-plug runtime: attach debug / auth / rate-limit / object / avatar-placeholder packages onto the existing α-0.5 session carrier and observe accepted / rejected / deferred / activation-cut / lifecycle changes in the same session.

## Plan update status

`plan/` 更新済み:
`plan/45-operational-alpha05-roadmap.md` と `plan/49-host-io-and-session-runtime-roadmap.md` を、`P-A1-20` actualized / next reopen point = `P-A1-21` / α-0.8 gap = same-session attach mutation へ更新した。

## Documentation.md update status

`Documentation.md` 更新済み:
latest closeout を `P-A1-20` に進め、bounded α-0.5 line に minimal typed external `AddOne` lane が入ったことを反映した。

## progress.md update status

`progress.md` 更新済み:
latest closeout package、current promoted reopen point、operational α-0.5 `100%`、Macro / blockers / validation floor / recent log を `P-A1-20` に同期した。

## tasks.md update status

`tasks.md` 更新済み:
`P-A1-20` を完了済みとして扱い、current recommended reopen point を `P-A1-21` に繰り上げた。

## samples_progress.md update status

`samples_progress.md` 更新済み:
α-0.5 row を `100%` に上げ、`OA05-07` と `practical_alpha05_host_io` を typed host-I/O minimal demo evidence として反映した。

## Reviewer findings and follow-up

- Reviewer `Archimedes` returned 3 actionable findings:
  - `progress.md` Macro 7 wording drifted away from the promoted reopen point
  - `plan/49` still described α-0.8 as lacking an external execution seam
  - `specs/24` still claimed the repo lacked the same-session carrier and typed host-I/O lane
- Follow-up taken:
  - fixed `progress.md` Macro 7 to keep `P-A1-21` as the next promoted line
  - fixed `plan/49` to say the α-0.8 gap is same-session attach mutation / lifecycle observation
  - fixed `specs/24` current repo reading and progress-percentage wording to reflect post-`P-A1-20` α-0.5 state
- Reviewer `Laplace` and `Dirac` timed out across two waits and were shut down without findings.
- Local focused review covered the gaps left by timed-out reviewers:
  - Rust/runtime diff inspection confirmed host-I/O stays outside Mir core and only widens the bounded session carrier
  - save/load diff inspection confirmed host-I/O history roundtrips through the local frontier without claiming distributed durability
  - sample/dashboard diff inspection confirmed first-floor vs operational α-0.5 vs α-0.8 / α-0.9 distinctions stay explicit

## Skipped validations and reasons

None.

## Commit / push status

- Pending at report creation time.
- Primary package commit and push will be added after git closeout.

## Sub-agent session close status

- `Archimedes` completed and was closed after findings were applied.
- `Laplace` timed out twice and was closed without a returned finding.
- `Dirac` timed out twice and was closed without a returned finding.
