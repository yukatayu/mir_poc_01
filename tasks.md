# tasks

最終更新: 2026-04-30 16:05 JST

## この文書について

- この文書は repo 全体の **current task map** です。
- 規範判断の正本は `specs/`、長期比較と source trace は `plan/`、runnable sample 状態は `samples_progress.md`、実行証跡は `docs/reports/` に置きます。
- append-only 履歴ではありません。current checkpoint、次に詰める gate、blocker を読める snapshot として保ちます。

## current task-level status

- active executable floor は維持されています:
  `samples/clean-near-end/`、`samples/current-l2/`、`samples/lean/`、Sugoroku world、avatar follow、typed external preview、network canary、projection/codegen bridge、viewer prototype inventory。
- `P0..P18`、`P19`、`P20`、`P21`、`R1..R7`、post-`P21` later-family docs-first trilogy は close 済みです。
- current promoted implementation line は存在しません。追加の self-driven post-`P21` docs-first family も残っていません。
- next reopen point は `U1` actual commitment です。
  Packaging / installed binary target、host integration target、first shipped public surface scope、final shared-space operational catalog breadth は user-facing decision を要します。
- self-driven に残るのは maintenance lane です:
  stale docs cleanup、validation rerun、report creation、formatting cleanup、guardrail maintenance、regression repair。
- 2026-04-29 の uncommitted Rust formatting cleanup は `b213721` `Apply Rust formatting cleanup` として push 済みです。
- 2026-04-30 の handoff path normalization / `sub-agent-pro` role sync / `plan/10` current-line cooling は maintenance-only closeout であり、新しい promoted implementation line は開いていません。
- 2026-04-30 の `plan/02` / `plan/07` / `plan/09` temperature audit も maintenance-only closeout であり、repository memory の stale current-line wording を冷やしただけで、new implementation queue は reopened していません。
- 2026-04-30 の full validation rerun / `samples/README.md` + `scripts/README.md` + `samples_progress.md` front-door parity sync も maintenance-only closeout であり、current validation floor と sample/script taxonomy の説明を揃えただけで、new implementation queue は reopened していません。
- 2026-04-30 の `plan/01` point-in-time disclaimer audit も maintenance-only closeout であり、repository memory の storage / queue wording を current snapshot authority に合わせて冷やしただけで、new implementation queue は reopened していません。
- 2026-04-30 の `plan/11` / roadmap mirror point-in-time audit も maintenance-only closeout であり、near-term roadmap memory / front-door summary / hands-on landing の stale date / queue wording を current snapshot authority に合わせて冷やしただけで、new implementation queue は reopened していません。
- 2026-04-30 の landing docs date-stamp audit も maintenance-only closeout であり、front-door current-line wording と `current_phase_closeout_01.md` 参照の stale queue wording を current snapshot authority に合わせて冷やしただけで、new implementation queue は reopened していません。
- 2026-04-30 の reader-facing detailed summary audit も maintenance-only closeout であり、`docs/research_abstract/` の residual `actual next open work` / stale date wording を current snapshot authority に合わせて冷やしただけで、new implementation queue は reopened していません。
- 2026-04-30 の post-sweep full validation rerun も maintenance-only closeout であり、docs-only refresh 後の sample/helper/cargo/formatter floor が still green であることを再確認しただけで、new implementation queue は reopened していません。
- 2026-04-30 の remaining-open-gate wording normalization も maintenance-only closeout であり、`specs/11` / `plan/` / `progress.md` / hands-on mirror の `next open work` lexical marker を current snapshot authority と dated snapshot wording に合わせて冷やしただけで、new implementation queue は reopened していません。
- 2026-04-30 の stale-wording lint feasibility audit では、allowlist なしの lexical lint は `docs/reports/` / `specs/examples/` / `sub-agent-pro/` / `progress.md` recent log に false positive を多発させるため不採用とし、将来やるなら active-current docs 限定の standalone allowlisted pass に留める boundary だけを確認しました。new implementation queue は reopened していません。
- 2026-04-30 の `VerificationLayer` law surface docs-first inventory では、current emitted floor が helper `verification_handoff_witness` と runtime `verification_model_check` の 2 row に留まり、shared emitted laws が `evidence_preservation` / `residual_obligations_are_explicit`、helper-only emitted law が `no_hidden_handoff_without_witness` であること、theorem bridge / runtime policy / visualization law families は widening candidate のままであることを `plan/29` / `plan/14` に固定しました。new implementation queue は reopened していません。
- 2026-04-30 の `VerificationLayer` public naming gate docs-first inventory では、active emitted names が helper `verification_handoff_witness` / runtime `verification_model_check` に留まり、`verification_preview` / `verification_summary` / `model_check_summary` / theorem bridge / runtime policy preview は evidence-carrier または downstream-consumer naming に残ること、public/shared naming と emitted-row-to-consumer naming relation は未決のままであることを `plan/29` / `plan/14` に固定しました。new implementation queue は reopened していません。

## executable floor

| lane | current floor | current command | not a claim of |
|---|---|---|---|
| Mir current-L2 | `samples/current-l2/` | `python3 scripts/current_l2_guided_samples.py closeout --format json` | final parser grammar / public API |
| clean near-end suite | `samples/clean-near-end/` | `python3 scripts/clean_near_end_samples.py closeout` | full language completion |
| Sugoroku world | `scripts/sugoroku_world_samples.py` | `python3 scripts/sugoroku_world_samples.py closeout --format json` | real network / durable distributed runtime |
| avatar follow | `scripts/avatar_follow_samples.py` | `python3 scripts/avatar_follow_samples.py closeout --format json` | `FAIRY-05` implementation / public avatar API |
| typed external | `scripts/typed_external_boundary_samples.py` | `python3 scripts/typed_external_boundary_samples.py closeout --format json` | final host schema / final adapter API |
| network transport | `scripts/network_transport_samples.py` | `python3 scripts/network_transport_samples.py closeout --format json` | production socket / durable replay |
| projection / placement | `scripts/projection_codegen_samples.py` + committed generated manifest | `python3 scripts/projection_codegen_samples.py closeout --format json` | final emitted executable family |
| viewer prototype | `scripts/visual_debugger_viewer_samples.py` | `python3 scripts/visual_debugger_viewer_samples.py closeout --format json` | final viewer API / telemetry service |
| hot-plug runtime | `crates/mirrorea-core`, `crates/mir-runtime` | `cargo test -p mir-runtime --test hotplug_runtime_skeleton` | rollback / durable migration / distributed ordering / final ABI |
| storage / backend | `/mnt/mirrorea-work`, `scripts/env/`, `scripts/storage/` | `bash scripts/storage/detach_prepare.sh` | actual LLVM build / backend choice |

## package map

| Package | Macro phase | Status | Current reading |
|---|---|---|---|
| `P0` / `P1` | `Macro 0` | closed | source hierarchy, repo layer map, dashboard stabilization |
| `P2` / `P12` | `Macro 6-7` | closed | typed external residual review and helper-local host-boundary inventory |
| `P3` / `P15` | `Macro 6-7` | closed | projection preview boundary and committed generated bridge evidence |
| `P4` / `P5` | `Macro 6` | closed | `TermSignature` and `LayerSignature` current lanes / scope split |
| `P6` / `P7` | `Macro 6-7` | closed | `MessageEnvelope` / `AuthEvidence` seam and typed visualization / telemetry security |
| `P8` / `P9` | `Macro 6` | closed | Sugoroku attach hardening and avatar follow representative slice hardening |
| `P10` / `P11` | `Macro 6-7` | closed | `mirrorea-core` carrier substrate and logical multi-place shell floor |
| `P13` / `P14` | `Macro 6-7` | closed | helper-local network canaries and hot-plug package-manager preview inventory |
| `P16` / `P17` | `Macro 7` | closed | typed viewer prototype inventory and storage / LLVM guardrail |
| `P18` | `Macro 7` mixed gate | closed repo-side | public-boundary inventory / mixed-gate split; final freeze still later |
| `U1` option matrix | `Macro 8` prep | closed docs-first, actual commitment open | option inventory exists; actual choice remains user-facing |
| `R1` / `R2` / `R3` | `Macro 8` prep | closed | verification widening threshold, AttachPoint minimal contract, `FAIRY-05` carrier recommendation |
| `R4` / `R5` / `R6` / `R7` | `Macro 8` prep | closed | hot-plug kept-later boundary, owner split, carrier admission, next-package inventory |
| `P19` | `Macro 6-7` | closed | engine-neutral hot-plug request/verdict carrier in `mirrorea-core` |
| `P20` | `Macro 6-7` | closed | thin runtime/report assembly in `mir-runtime` |
| `P21` | `Macro 6-7` | closed | runtime-side engine-state progression narrow floor |
| post-`P21` rollback / durable migration | `Macro 8` prep | closed docs-first | first recommendation boundary; no actual rollback / migration engine completion |
| post-`P21` distributed activation ordering | `Macro 8` prep | closed docs-first | second recommendation boundary; no actual distributed activation protocol |
| post-`P21` final public hot-plug ABI | `Macro 8` mixed gate | closed docs-first | third recommendation bridge: `freeze prerequisite fixed; public ABI still unfrozen` |

## ordered current work

| Order | Work item | Owner | Status | Completion condition |
|---:|---|---|---|---|
| 1 | Documentation freshness / stale-current audit | repo | active maintenance | stale wording removed, report created, validation run, commit/push |
| 2 | Regression / formatting cleanup | repo | active maintenance | focused tests pass, formatting clean, commit/push |
| 3 | `U1` actual commitment | user + repo | open | actual choices recorded for packaging, host target, first shipped public surface, final catalog breadth |
| 4 | Post-`U1` first implementation tranche | repo after user choice | blocked | chosen public / host / packaging surface has enough scope to implement without guessing |

## self-driven maintenance tasks

These are safe to do without new product decisions.

| Task | Objective | Validation | Report requirement | Stop line |
|---|---|---|---|---|
| docs freshness audit | keep `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, research summaries, and traceability current | `check_source_hierarchy.py`, `validate_docs.py`, `git diff --check` | new `docs/reports/NNNN-*.md` | do not create new normative decisions in snapshot docs; do not use naive banned-phrase scans over `docs/reports/`, `specs/examples/`, `sub-agent-pro/`, or `progress.md` recent log |
| runnable dashboard refresh | keep sample status, validation timestamps, and blockers evidence-backed | relevant helper closeout commands | report + `samples_progress.md` | do not mark conceptual rows over 25% or use 100% without current-scope commit/push |
| Rust formatting / regression repair | keep cargo formatting and focused test floors green | `cargo fmt --check`, affected `cargo test` commands | report if behavior or docs status changes | do not mix unrelated feature work into formatting cleanup |
| storage guardrail check | keep external workdir and cleanup scripts safe on small VPS | `df -h`, `free -h`, `findmnt`, storage scripts | report with resource output | no destructive cleanup / mount / format without explicit confirmation |

## user decision blockers

### Blocker 1. packaging shape / installed binary target

- overview:
  choose whether the first public shape is `CLI`, `library`, `engine-adapter`, or `hybrid`.
- affects:
  `P18`, `U1`, backend / storage / distribution, future public API shape.
- current recommendation:
  `library-first` remains the provisional recommendation. CLI or installed binary promotion should be a second gate.
- reason:
  current shell / helper actualization is not an installed binary adoption fact.

### Blocker 2. host integration target

- overview:
  choose `browser`, `native process`, `engine`, or `mixed`.
- affects:
  typed external boundary, network transport, viewer, engine ABI, packaging.
- current recommendation:
  `native process` remains the provisional recommendation because current evidence is strongest around same-process / process-boundary helpers.
- reason:
  browser / engine targets require exact host schema and adapter ABI decisions that are still user-facing.

### Blocker 3. first shipped public surface scope

- overview:
  choose `parser/checker/runtime/verifier first`, `adapter/viewer/projection/hot-plug first`, or `two-step split`.
- affects:
  final public parser/API, viewer, adapter, projection, hot-plug ABI.
- current recommendation:
  `two-step split`: first narrow core library surface, then reopen integration surfaces with host target.
- reason:
  public core and integration surfaces have different dependencies and validation floors.

### Blocker 4. final shared-space operational catalog breadth

- overview:
  choose `minimal subset`, `portal / multi-world expansion`, or `fairness / quorum / exhaustive catalog`.
- affects:
  Sugoroku, avatar, network transport, hot-plug migration / replay, application realization.
- current recommendation:
  keep `minimal subset` until packaging / host / shipped surface are fixed.
- reason:
  broader catalog choices can force durability, replay, fairness, and host integration commitments too early.

## research-discovery items

- exact public `VerificationLayer` law surface, public naming, and composition contract beyond the current emitted floor (`verification_handoff_witness`, `verification_model_check`).
- `FAIRY-05` final implementation carrier and negative companion shape.
- projection equivalence evidence beyond committed generated bridge manifest.
- rollback / durable migration engine state machine after `P21`.
- distributed activation ordering evidence once multi-place / multi-server pressure is available.
- production transport / replay design that preserves transport / auth / membership / capability / witness separation.

## validation floor for this snapshot

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 scripts/current_l2_guided_samples.py closeout --format json
python3 scripts/clean_near_end_samples.py closeout
python3 scripts/current_l2_lean_sample_sync.py
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/avatar_follow_samples.py closeout --format json
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 scripts/network_transport_samples.py closeout --format json
python3 scripts/projection_codegen_samples.py closeout --format json
python3 scripts/visual_debugger_viewer_samples.py closeout --format json
bash scripts/storage/detach_prepare.sh
cargo test -p mir-ast
cargo test -p mirrorea-core
cargo test -p mir-runtime
cargo test -p mir-semantics
cargo fmt --check
git diff --check
```

## reporting requirement

Every non-trivial change must add a new report under `docs/reports/`.
For the current docs freshness task, the report must include:

- dirty state and formatting cleanup commit / push status
- documents consulted
- files changed
- reviewer findings and follow-up
- validation commands and results
- skipped validations and reasons
- remaining user decision blockers
