# 2032 — P-A1-22 alpha09 session-bound devtools export

## Objective

`P-A1-22` として、bounded operational α-0.9 session-bound devtools export を actualizeする。
具体的には、`practical_alpha05_session` / `practical_alpha08_hotplug_session` の同一 session carrier から、event DAG / local route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / observer-safe redacted view / retention-on-demand trace を JSON export と non-final HTML viewer に接続する。

## Scope and assumptions

- 規範正本は `specs/`、repository memory は `plan/`、snapshot は `progress.md` / `tasks.md` / `samples_progress.md` として扱った。
- bounded scope は α-0.9 session-bound devtools export までとし、final public viewer / telemetry ABI、durable audit backend、remote retained-artifact retrieval、distributed durable save/load、product-ready α-1 には踏み込まない。
- rejected attach は active runtime state を mutate しないが、α-0.9 で観測可能にするため session-carried observation journal として `hotplug_lifecycle` に残す前提に整理した。

## Start state / dirty state

- start state:
  previous closeout `P-A1-21`; worktree clean。
- resource check:
  `df -h .` で `/dev/vda2` 99G total / 30G avail、`free -h` で 960Mi RAM / 19Gi swap を確認。
- dirty state at start:
  none。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `specs/22-observability-devtools-semantics.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `plan/44-practical-alpha1-roadmap.md`
- `plan/47-operational-alpha09-devtools-roadmap.md`
- `plan/49-host-io-and-session-runtime-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `scripts/README.md`
- `docs/reports/2031-p-a1-21-alpha08-same-session-hotplug-runtime.md`

## Actions taken

1. Re-read the operational α specs / plans / snapshot docs and confirmed that `P-A1-22` was the promoted reopen point after `P-A1-21`.
2. Ran a RED cycle for `crates/mir-runtime/tests/practical_alpha09_devtools.rs`; confirmed compile failure because `mir_runtime::practical_alpha09_devtools` did not exist.
3. Ran a RED cycle for `scripts/tests/test_practical_alpha09_devtools.py`; confirmed import failure because `scripts/practical_alpha09_devtools.py` did not exist.
4. Added `crates/mir-runtime::practical_alpha09_devtools` with typed panels, telemetry rows, export sections, source-session references, explicit redaction/retention, and kept-later admin/debug marker.
5. Extended example `mir_practical_alpha05_session` with `export-devtools <session-path>`.
6. Adjusted `practical_alpha08_hotplug_session` so rejected attach attempts leave active runtime state and event DAG unchanged while becoming session-carried hot-plug lifecycle observations for α-0.9 export.
7. Added `scripts/practical_alpha09_devtools.py` and `scripts/tests/test_practical_alpha09_devtools.py` to actualize `OA09-01..09` and render a non-final static HTML viewer over the same session-bound payload.
8. Synchronized `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `samples/README.md`, `samples/practical-alpha1/README.md`, `scripts/README.md`, `specs/22`, `specs/24`, `plan/44`, `plan/47`, and `plan/49` to the α-0.9 actualized reading and next practical α-1 integrated workflow reopen point.

## Files changed

- `crates/mir-runtime/src/practical_alpha09_devtools.rs`
- `crates/mir-runtime/src/practical_alpha08_hotplug_session.rs`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/examples/mir_practical_alpha05_session.rs`
- `crates/mir-runtime/tests/practical_alpha09_devtools.rs`
- `crates/mir-runtime/tests/practical_alpha08_session_hotplug.rs`
- `scripts/practical_alpha09_devtools.py`
- `scripts/practical_alpha08_session_hotplug.py`
- `scripts/tests/test_practical_alpha09_devtools.py`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `scripts/README.md`
- `specs/22-observability-devtools-semantics.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `plan/44-practical-alpha1-roadmap.md`
- `plan/47-operational-alpha09-devtools-roadmap.md`
- `plan/49-host-io-and-session-runtime-roadmap.md`
- `docs/reports/2032-p-a1-22-alpha09-session-bound-devtools-export.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
pwd
find . -maxdepth 2 -type d | sort
git status --short
df -h .
free -h
git log --oneline -5
cargo test -p mir-runtime --test practical_alpha09_devtools -- --nocapture
python3 -m unittest scripts.tests.test_practical_alpha09_devtools
cargo test -p mir-runtime --test practical_alpha08_session_hotplug -- --nocapture
python3 scripts/practical_alpha09_devtools.py check-all --format json
python3 scripts/practical_alpha08_session_hotplug.py check-all --format json
python3 scripts/practical_alpha09_devtools.py render-html --format json
cargo fmt
python3 -m unittest scripts.tests.test_practical_alpha09_devtools scripts.tests.test_practical_alpha08_session_hotplug
date '+%Y-%m-%d %H:%M %Z'
cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture
cargo test -p mir-runtime --test practical_alpha05_host_io -- --nocapture
python3 scripts/practical_alpha05_session.py check-all --format json
python3 -m unittest scripts.tests.test_practical_alpha09_devtools scripts.tests.test_practical_alpha08_session_hotplug scripts.tests.test_practical_alpha05_session
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- RED:
  `cargo test -p mir-runtime --test practical_alpha09_devtools -- --nocapture`
  initially failed with unresolved import `mir_runtime::practical_alpha09_devtools`.
- RED:
  `python3 -m unittest scripts.tests.test_practical_alpha09_devtools`
  initially failed with `ModuleNotFoundError: No module named 'practical_alpha09_devtools'`.
- `cargo test -p mir-runtime --test practical_alpha09_devtools -- --nocapture`
  pass; `3` tests passed.
- `cargo test -p mir-runtime --test practical_alpha08_session_hotplug -- --nocapture`
  pass; `3` tests passed after rejected attach observation semantics were updated.
- `python3 scripts/practical_alpha09_devtools.py check-all --format json`
  pass; `OA09-01..09` passed and `operational_alpha09_ready = true`.
- `python3 scripts/practical_alpha08_session_hotplug.py check-all --format json`
  pass; `OA08-01..10` remained pass and `operational_alpha08_ready = true`.
- `python3 scripts/practical_alpha09_devtools.py render-html --format json`
  pass; emitted a temporary non-final static HTML viewer containing all `OA09` panels.
- `python3 -m unittest scripts.tests.test_practical_alpha09_devtools scripts.tests.test_practical_alpha08_session_hotplug`
  pass; `7` tests passed.
- `cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture`
  pass; `3` tests passed.
- `cargo test -p mir-runtime --test practical_alpha05_host_io -- --nocapture`
  pass; `2` tests passed.
- `python3 scripts/practical_alpha05_session.py check-all --format json`
  pass; `OA05-01..07` passed and `operational_alpha05_ready = true`.
- `python3 -m unittest scripts.tests.test_practical_alpha09_devtools scripts.tests.test_practical_alpha08_session_hotplug scripts.tests.test_practical_alpha05_session`
  pass; `10` tests passed.
- `python3 -m unittest scripts.tests.test_validate_docs`
  pass; `11` tests passed.
- `python3 scripts/check_source_hierarchy.py`
  pass; `84/84` required paths present.
- `python3 scripts/validate_docs.py`
  pass; documentation scaffold complete, latest report headings accepted.
- `cargo fmt --check`
  pass.
- `git diff --check`
  pass.

## What changed in understanding

- The honest α-0.9 bridge is not another exact-report bundle; it must derive from the same session carrier and its session-carried observation journal.
- Rejected attach attempts should remain non-mutating for active runtime state, but α-0.9 needs them retained as typed observation entries so lifecycle and fallback panels can explain what was rejected.
- `OA09-*` closes bounded session-bound devtools readiness, but it still does not make the viewer ABI public, durable, remote, or product-ready.

## Open questions

- Whether the next practical α-1 workflow should be a script-only carrier first or a Rust report module plus script, mirroring `P-A1-22`.
- Whether admin/full debug view should remain `kept_later` until a policy catalog is widened, or get a narrow explicit admin-only export in a later package.
- Whether `P-A1-23` should include Docker/local TCP revalidation directly or keep transport as first-floor evidence consumed by an integrated workflow summary.

## Suggested next prompt

`P-A1-23` として、existing first-floor front-door / checker / session runtime / host-I/O / same-session hot-plug / local save-load / α-0.9 devtools export / product-preview evidence を、final public product-ready と混同しない 1 つの bounded practical α-1 integrated workflow carrier に束ねてください。

## Plan update status

- updated:
  `plan/44-practical-alpha1-roadmap.md`
  `plan/47-operational-alpha09-devtools-roadmap.md`
  `plan/49-host-io-and-session-runtime-roadmap.md`
- reason:
  α-0.9 actualization、`OA09-01..09`、next practical α-1 integrated workflow reopen point が変わったため。

## Documentation.md update status

- updated:
  latest closeout、already-have / still-missing reading を α-0.9 actualized stateへ同期。

## progress.md update status

- updated:
  latest closeout、current promoted reopen point、three-axis progress、line snapshot、feature maturity rows、validation floor、recent log を α-0.9 actualized stateへ同期。

## tasks.md update status

- updated:
  current task-level status、ordered self-driven packages、current recommendation を `P-A1-23` practical α-1 integrated workflow carrier へ切り替えた。

## samples_progress.md update status

- updated:
  α-0.9 row を `100` / bounded operational α-0.9 ready へ変更し、`OA09-01..09` と validation anchor を追加。

## Reviewer findings and follow-up

- sub-agent reviewers were not spawned in this turn because the current user request did not explicitly ask for sub-agents and the active tool policy only permits spawning when explicitly requested.
- local focused review:
  checked the Rust export shape against `specs/22` / `specs/24`, verified rejected attach is observation-only for active runtime state, and confirmed `OA09-01..09` map to the required α-0.9 operational sample matrix.
- follow-up:
  next package should integrate the bounded α-0.5 / α-0.8 / α-0.9 line into a practical α-1 developer workflow without claiming final public product readiness.

## Skipped validations and reasons

- full repo-wide Rust / Python suite:
  skipped; package scope is `mir-runtime` α-0.9 devtools, α-0.8 observation semantics, scripts, and snapshot docs. Focused affected validations plus required docs/format checks were run.
- Docker Compose / transport focused reruns:
  skipped; `practical_alpha1_transport` and Docker transport runtime code were not changed.

## Commit / push status

- package commit:
  pending at report creation; to be updated after commit / push.
- push:
  pending at report creation; to be updated after commit / push.

## Sub-agent session close status

- no sub-agent sessions were spawned for this package.
