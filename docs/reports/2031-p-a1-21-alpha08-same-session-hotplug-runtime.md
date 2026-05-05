# 2031 — P-A1-21 alpha08 same-session hotplug runtime

## Objective

`P-A1-21` として、bounded operational α-0.8 same-session hot-plug runtime を actualizeする。
具体的には、`practical_alpha05_session` carrier 上で debug / auth / rate-limit / object preview / deferred detach の attach accepted / rejected / deferred / activation cut / observer-safe lifecycle summary を同じ session workflow に接続し、repo snapshot と roadmap を α-0.8 actualized reading へ同期する。

## Scope and assumptions

- 規範正本は `specs/`、repository memory は `plan/`、snapshot は `progress.md` / `tasks.md` / `samples_progress.md` として扱った。
- bounded scope は α-0.8 same-session hot-plug runtime までとし、α-0.9 live/session-bound devtools export、accepted detach execution、distributed durable save/load、final public runtime/devtools/hot-plug ABI には踏み込まない。
- unsupported runtime fallback は same-session rejected attach に加え、既存 `AV-A1-03` companion preview evidence を source として再利用し、native execution や unsupported-runtime execution success へ overclaim しない前提で扱った。

## Start state / dirty state

- start state:
  `origin/main` at previous closeout `P-A1-20`; worktree clean。
- resource check:
  `df -h .` で `/dev/vda2` 99G total / 30G avail、`free -h` で 960Mi RAM / swap 18Gi free を確認。
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
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/18-practical-alpha1-scope.md`
- `specs/19-verification-stratification.md`
- `specs/20-cut-save-load-semantics.md`
- `specs/21-auth-layer-algebra.md`
- `specs/22-observability-devtools-semantics.md`
- `specs/23-typed-external-host-boundary.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `plan/44-practical-alpha1-roadmap.md`
- `plan/46-operational-alpha08-roadmap.md`
- `plan/47-operational-alpha09-devtools-roadmap.md`
- `plan/48-theory-freeze-proof-obligations.md`
- `plan/49-host-io-and-session-runtime-roadmap.md`
- `samples/practical-alpha1/README.md`
- `scripts/README.md`
- `docs/reports/2030-p-a1-20-typed-host-io-direct-execution-lane.md`
- sub-agent findings:
  existing explorers `019df5a9-44b0-7940-9e4d-b4bfcb262e09` and `019df5a9-454a-7712-b9a2-0d4a9274c52e`

## Actions taken

1. Re-read the operational α specs / plans / snapshot docs and confirmed that `P-A1-21` was the promoted reopen point after `P-A1-20`.
2. Ran a RED cycle for `crates/mir-runtime/tests/practical_alpha08_session_hotplug.rs`; confirmed compile failure because `mir_runtime::practical_alpha08_hotplug_session` did not exist.
3. Extended `practical_alpha05_session` state to persist `active_layers`, object preview inventory, hot-plug lifecycle entries, runtime behavior markers, and observer-safe summaries for those lanes.
4. Added `crates/mir-runtime::practical_alpha08_hotplug_session` to project exact `PracticalAlpha1HotPlugReport` results into the live session carrier with:
   accepted attach mutation,
   rejected attach no-mutation,
   deferred detach boundary visibility,
   event DAG growth,
   observer-safe lifecycle summaries,
   save/load persistence of hot-plug state.
5. Extended example `mir_practical_alpha05_session` with an `attach` subcommand that writes the updated session back when mutation is allowed and still returns explicit report-only rejections when mutation is denied.
6. Added Rust tests for same-session attach mutation, rejected no-mutation, object preview visibility, deferred detach boundary, and hot-plug save/load persistence.
7. Added `scripts/practical_alpha08_session_hotplug.py` and `scripts/tests/test_practical_alpha08_session_hotplug.py` to actualize the operational α-0.8 matrix `OA08-01..10` over the existing `HP-A1-*` / `AV-A1-03` evidence and the live session carrier.
8. Synchronized `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `samples/README.md`, `samples/practical-alpha1/README.md`, `scripts/README.md`, `plan/46`, `plan/47`, `plan/49`, and `specs/24` to the new α-0.8 reading and next reopen point.
9. Ran focused runtime/script validations, then reran docs scaffold / source hierarchy / formatting / diff floor after the doc sync.

## Files changed

- `crates/mir-runtime/src/practical_alpha05_session.rs`
- `crates/mir-runtime/src/practical_alpha08_hotplug_session.rs`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/examples/mir_practical_alpha05_session.rs`
- `crates/mir-runtime/tests/practical_alpha08_session_hotplug.rs`
- `scripts/practical_alpha08_session_hotplug.py`
- `scripts/tests/test_practical_alpha08_session_hotplug.py`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `scripts/README.md`
- `plan/46-operational-alpha08-roadmap.md`
- `plan/47-operational-alpha09-devtools-roadmap.md`
- `plan/49-host-io-and-session-runtime-roadmap.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `docs/reports/2031-p-a1-21-alpha08-same-session-hotplug-runtime.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
pwd
find . -maxdepth 2 -type d | sort
git status --short
df -h .
free -h
sed -n '1,260p' crates/mir-runtime/src/practical_alpha1_hotplug.rs
sed -n '1,260p' crates/mir-runtime/src/practical_alpha05_session.rs
sed -n '1,280p' scripts/practical_alpha05_session.py
sed -n '1,260p' scripts/practical_alpha1_export_devtools.py
sed -n '1,260p' crates/mir-runtime/src/practical_alpha05_host_io.rs
cargo test -p mir-runtime --test practical_alpha08_session_hotplug -- --nocapture
python3 -m unittest scripts.tests.test_practical_alpha08_session_hotplug
python3 scripts/practical_alpha08_session_hotplug.py check-all --format json
cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture
cargo test -p mir-runtime --test practical_alpha05_host_io -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture
python3 scripts/practical_alpha05_session.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_avatar.py check-all --format json
python3 -m unittest scripts.tests.test_practical_alpha05_session scripts.tests.test_practical_alpha08_session_hotplug
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt
cargo fmt --check
git diff --check
date '+%Y-%m-%d %H:%M %Z'
git diff --stat
```

## Evidence / outputs / test results

- `cargo test -p mir-runtime --test practical_alpha08_session_hotplug -- --nocapture`
  pass; `3` tests passed.
- `cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture`
  pass; `3` tests passed.
- `cargo test -p mir-runtime --test practical_alpha05_host_io -- --nocapture`
  pass; `2` tests passed.
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
  pass; `17` tests passed.
- `python3 scripts/practical_alpha05_session.py check-all --format json`
  pass; `OA05-01..07` passed and `operational_alpha05_ready = true`.
- `python3 scripts/practical_alpha08_session_hotplug.py check-all --format json`
  pass; `OA08-01..10` passed and `operational_alpha08_ready = true`.
- `python3 scripts/practical_alpha1_attach.py check-all --format json`
  pass; `HP-A1-01..07` row family preserved.
- `python3 scripts/practical_alpha1_avatar.py check-all --format json`
  pass; `AV-A1-01/02/03` row family preserved.
- `python3 -m unittest scripts.tests.test_practical_alpha05_session scripts.tests.test_practical_alpha08_session_hotplug`
  pass; `6` tests passed.
- `python3 -m unittest scripts.tests.test_validate_docs`
  pass; `11` tests passed.
- `python3 scripts/check_source_hierarchy.py`
  pass; `84/84` required paths present.
- `python3 scripts/validate_docs.py`
  pass.
- `cargo fmt --check`
  pass after one `cargo fmt` normalization step.
- `git diff --check`
  pass.

## What changed in understanding

- The smallest honest α-0.8 bridge was not to widen exact-report devtools first, but to reuse `practical_alpha05_session` as the live state substrate and project exact hot-plug reports into that carrier.
- Rejected attach can stay same-session observable without mutating the persisted session frontier by returning an explicit attach report and keeping observer export unchanged.
- Unsupported runtime fallback still needs the distinct avatar companion preview source for its visible degradation claim; that is compatible with α-0.8 as long as the same-session lane stays explicit about rejecting the live attach.

## Open questions

- `P-A1-22` should decide whether α-0.9 route trace / membership timeline / save-load timeline should be extended directly inside `PracticalAlpha05ObserverSafeExport` or emitted as a sibling session-bound devtools bundle.
- The current α-0.8 line keeps rate-limit as a declared-failure preview marker, not a later in-session replay of a new runtime action. If a broader semantic replay lane is needed, it should be promoted explicitly rather than implied from the marker.
- accepted detach execution remains later; `HP-A1-07` / `OA08-10` still expose only the deferred minimal contract boundary.

## Suggested next prompt

`P-A1-22` として、same-session α-0.5 / α-0.8 carrier から α-0.9 session-bound devtools export を actualizeし、event DAG / route trace / membership timeline / witness relation / hot-plug lifecycle / save-load timeline / observer-safe redacted view を live export と non-final viewer surface へ接続してください。exact `VIS-A1-*` bundles は source vocabulary として reuse してよいですが、same-session runtime source を first-class にしてください。

## Plan update status

- updated:
  `plan/46-operational-alpha08-roadmap.md`
  `plan/47-operational-alpha09-devtools-roadmap.md`
  `plan/49-host-io-and-session-runtime-roadmap.md`
- reason:
  α-0.8 actualization、α-0.9 next reopen point、session/runtime sequencing が変わったため。

## Documentation.md update status

- updated:
  latest closeout、already-have / still-missing reading を α-0.8 actualized stateへ同期。

## progress.md update status

- updated:
  latest closeout、current promoted reopen point、three-axis progress、line snapshot、feature maturity rows、validation floor、recent log を α-0.8 actualized stateへ同期。

## tasks.md update status

- updated:
  current task-level status、ordered self-driven packages、current recommendation を `P-A1-22` α-0.9 session-bound devtools export へ切り替えた。

## samples_progress.md update status

- updated:
  α-0.8 row を `100` / bounded operational α-0.8 ready へ変更し、`OA08-01..10` と validation anchor を追加。

## Reviewer findings and follow-up

- sub-agent `019df5a9-454a-7712-b9a2-0d4a9274c52e`
  exact `VIS-A1-04` bundle vocabularyを live session observer-safe export へ薄く再利用する方針が最小-churnであると確認。
- sub-agent `019df5a9-44b0-7940-9e4d-b4bfcb262e09`
  current seam が exact report assembly であり live session mutation ではないこと、session save/load と observer export に hot-plug state carrier が必要なことを再確認。
- local focused review:
  rejected attach no-mutation、object preview visibility、deferred detach boundary persistence、save/load roundtrip を Rust tests で固定し、helper matrix 側では `OA08-01..10` closeout で α-0.8 bounded readiness condition を確認した。
- follow-up:
  α-0.9 では exact `VIS-A1-*` vocabulary と live session source の対応関係を更に整理する必要がある。

## Skipped validations and reasons

- full repo-wide Rust / Python suite:
  skipped; package scope は same-session hot-plug carrier、α-0.5 carrier persistence、existing hot-plug/avatar source rows、snapshot docs に限定されているため、affected validations を優先した。
- transport / Docker Compose / product-preview focused reruns:
  skipped; `practical_alpha1_transport`、Docker transport runtime、product-preview bundle code は今回未変更のため。

## Commit / push status

- commit:
  pending
- push:
  pending

## Sub-agent session close status

- `019df5a9-44b0-7940-9e4d-b4bfcb262e09`:
  completed, not yet closed at report write time
- `019df5a9-454a-7712-b9a2-0d4a9274c52e`:
  completed, not yet closed at report write time
