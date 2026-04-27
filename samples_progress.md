# samples_progress

Last updated: 2026-04-27 10:14 JST
Current repo-local focus: clean near-end current layer と Sugoroku world vertical slice を runnable floor として維持しつつ、phase/sample/progress/storage foundation を追加する
Current active packages: phase-sample-progress-storage-foundation closeout; next `Sugoroku sample progress alignment`, `Avatar fairy follow sample plan`, `TermSignature registry / debug output`

## Summary

| Area | Current % | Positive samples | Negative samples | E2E | Visualization | Report |
|---|---:|---:|---:|---|---|---|
| Phase 0 Repository memory and decision boundary | 90 | 2 | 1 | pass | dashboard/table | `0913` |
| Phase 1 Mir current L2 semantics | 90 | 2 | 3 | pass | pretty/json trace | `0904`, `0913` |
| Phase 2 Parser-free PoC / detached validation loop | 75 | 2 | 2 | partial | detached artifact summary | `0904`, `0913` |
| Phase 3 Parser boundary and first checker cut | 90 | 3 | 2 | pass | AST/checker test surface | `0904`, `0913` |
| Phase 4 Shared-space membership / room boundary | 90 | 4 | 1 | pass | membership timeline | `0909`, `0913` |
| Phase 5 Small decidable core and proof boundary | 90 | 4 | 3 | pass | verification/Lean/model-check preview | `0904`, `0913` |
| Phase 6 Compile-ready minimal actualization | 90 | 5 | 0 | pass | json/debug summary | `0904`, `0913` |
| Phase 7 Sugoroku runtime attach | 90 | 8 | 2 | pass | summary/turn-trace/membership/verification | `0909`, `0913` |
| Phase 8 Avatar fairy follow / fallback anchor | 10 | 0 | 0 | not yet | not yet | `0913` |
| Phase 9 Typed external boundary / adapter | 1 | 0 | 0 | not yet | not yet | `0913` |
| Phase 10 MessageEnvelope / authentication seam | 1 | 0 | 0 | not yet | not yet | `0913` |
| Phase 11 TermSignature / LayerSignature | 1 | 0 | 0 | not yet | not yet | `0912`, `0913` |
| Phase 12 Projection / placement | 1 | 0 | 0 | not yet | not yet | `0912`, `0913` |
| Phase 13 Network transport | 1 | 0 | 0 | not yet | not yet | `0913` |
| Phase 14 Hot-plug package / AttachPoint | 10 | 0 | 1 | not yet | lifecycle TODO only | `0909`, `0912`, `0913` |
| Phase 15 Visualization / IDE | 50 | 1 | 0 | partial | `mir_hilight.html` + helper debug | `0910`, `0911`, `0913` |
| Phase 16 Compiler/backend/LLVM preparation | 10 | 0 | 0 | not yet | storage location report only | `0913` |

## Build/storage environment

| Item | Status | Path | Notes |
|---|---|---|---|
| External workdir | not mounted | `/mnt/mirrorea-work` | `vdb` device is visible but no filesystem/mountpoint is active |
| Cargo target | repo/root risk | `target/` | current `target/` uses about `5.2G`; external redirect script added in this task |
| Storage env script | yes | `scripts/env/mirrorea_storage_env.sh` | exports safe default paths and refuses `--ensure-dirs` on unmounted default unless overridden |
| LLVM build | not started | `/mnt/mirrorea-work/llvm/{src,build,install}` | policy/path only; no LLVM artifact committed |
| Generated artifacts | policy only | `/mnt/mirrorea-work/generated-artifacts` | repo-local root should not become default heavy sink |
| Detach prep script | yes | `scripts/storage/detach_prepare.sh` | non-destructive; prints status only |
| Cleanup script | yes | `scripts/storage/cleanup_disposable_artifacts.sh` | requires `--confirm`; refuses unsafe unmounted default |

## Active sample matrix

| ID | Layer | Sample path | Purpose | Positive/Negative | Unit | Integration | E2E | Visualization | Docs | % | Last validation | Blocker | Next step | Report |
|---|---|---|---|---|---|---|---|---|---|---:|---|---|---|---|
| PH0 | repository memory / decision boundary | `samples_progress.md`, `docs/reports/`, `scripts/check_source_hierarchy.py` | source hierarchy, report path, dashboard sync | `P0-01,P0-03 / P0-02` | `python3 scripts/check_source_hierarchy.py` | `python3 scripts/validate_docs.py` | specs -> plan -> snapshot -> report path | dashboard table | yes | 90 | 2026-04-27 10:14 JST | 100% needs commit/push | keep dashboard/report rows in sync | `0913` |
| PH1 | Mir current L2 semantics | `samples/current-l2/` | guarded option chain, fallback, lineage, `atomic_cut` | `MIR-01 / MIR-02..05` | `cargo test -p mir-ast`, `cargo test -p mir-runtime` | `python3 scripts/current_l2_guided_samples.py smoke-all --format json` | source -> runtime -> trace summary | pretty/json trace, detached artifacts | yes | 90 | 2026-04-27 10:14 JST | final public parser/API deferred | keep sample IDs and rejection reasons stable | `0904`, `0913` |
| PH2 | parser-free PoC / detached loop | `samples/current-l2/`, `scripts/current_l2_detached_loop.py` | fixture -> run bundle -> detached artifact -> compare | `P2-01,P2-03 / P2-02,P2-04` | `cargo test -p mir-semantics` | current source regression + detached support tests | fixture -> interpreter -> artifact -> compare | detached artifact summary | yes | 75 | 2026-04-27 10:14 JST (partial) | no fresh detached-loop CLI run in this task yet | run dedicated detached-loop smoke in follow-up | `0904`, `0913` |
| PH3 | parser boundary / first checker cut | `crates/mir-ast/tests/*stage*`, `samples/current-l2/` | parse option/chain/request clause bundle and reject malformed cases | `P3-01..04 / P3-05` | `cargo test -p mir-ast` | `cargo test -p mir-runtime` | source sample -> AST -> checker -> runtime preview | AST/test surface | yes | 90 | 2026-04-27 10:14 JST | final parser grammar deferred | keep narrow parser cut explicit | `0904`, `0913` |
| PH4 | shared-space membership / room boundary | `samples/clean-near-end/sugoroku-world/00...07` | membership epoch/incarnation, join/leave, stale action rejection | `P4-01,P4-02,P4-05 / P4-03,P4-04` | `python3 scripts/sugoroku_world_samples.py check-all` | `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership` | room change -> command -> history/reject trace | membership timeline | yes | 90 | 2026-04-27 10:14 JST | real network and reconnect transport absent | tighten row/report mapping in follow-up | `0909`, `0913` |
| PH5 | small decidable core / proof boundary | `samples/clean-near-end/typing`, `samples/clean-near-end/model-check`, `samples/lean/` | finite-index typing, theorem bridge, model-check second line | `P5-01,P5-06,P5-07 / P5-02..05` | `cargo test -p mir-semantics` | `python3 scripts/clean_near_end_samples.py smoke-all --format json` | source -> checker -> theorem/model-check artifact | verification preview, Lean stub, counterexample trace | yes | 90 | 2026-04-27 10:14 JST | production theorem/model-check binding absent | keep proof/model-check overclaim avoided | `0904`, `0913` |
| PH6 | compile-ready minimal actualization | `samples/clean-near-end/` | `mir-ast` / `mir-semantics` / `mir-runtime` narrow implementation path | `P6-01..05 / none` | `cargo test -p mir-ast`, `cargo test -p mir-runtime`, `cargo test -p mir-semantics` | `python3 scripts/clean_near_end_samples.py smoke-all --format json` | clean suite smoke-all | json/debug summary | yes | 90 | 2026-04-27 10:14 JST | public shell/API deferred | maintain active suite and archive boundary | `0904`, `0913` |
| PH7 | Sugoroku runtime attach | `samples/clean-near-end/sugoroku-world/00...09`, `scripts/sugoroku_world_samples.py` | bootstrap, attach, start/reset, roll/publish/handoff, join/leave, reset, detach TODO | `SUG-00..03,SUG-05..08 / SUG-04,SUG-09` | `python3 scripts/sugoroku_world_samples.py check-all` | `python3 scripts/sugoroku_world_samples.py closeout --format json` | bootstrap -> attach -> start -> turn -> join/leave -> reset | summary, turn-trace, membership, verification | yes | 90 | 2026-04-27 10:14 JST | detach lifecycle and real transport deferred | align sample rows with helper debug surfaces | `0909`, `0913` |
| PH8 | avatar fairy follow / fallback anchor | `samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.txt` | define current prototype anchor and future sample family | planned only | not yet | not yet | not yet | not yet | yes | 10 | not yet | no active sample/helper exists | write active sample plan and helper contract | `0913` |
| PH9 | typed external boundary / adapter | `spec only` | replace stdio with typed external effect adapters | planned only | not yet | not yet | not yet | not yet | yes | 1 | not yet | no sample/helper exists | define `EXT-01..05` sample skeleton | `0913` |
| PH10 | MessageEnvelope / authentication seam | `spec only` | auth insertion without semantic collapse into transport | planned only | not yet | not yet | not yet | not yet | yes | 1 | not yet | no envelope carrier exists | define `AUTH-01..05` sample skeleton | `0913` |
| PH11 | TermSignature / LayerSignature | `spec only`, future `scripts/sugoroku_world_samples.py --debug signatures` | typed signatures for terms/layers | planned only | not yet | not yet | not yet | not yet | yes | 1 | not yet | no signature dump exists yet | implement signature dump after sample alignment | `0912`, `0913` |
| PH12 | projection / placement | `spec only` | system source -> place-specific program projection | planned only | not yet | not yet | not yet | not yet | yes | 1 | not yet | no projection helper exists | define projection checklist and examples | `0912`, `0913` |
| PH13 | network transport | `spec only` | move logical Place runtime to separate process/transport | planned only | not yet | not yet | not yet | not yet | yes | 1 | not yet | no network transport sample exists | keep queue docs-first for now | `0913` |
| PH14 | hot-plug package / AttachPoint | `samples/clean-near-end/sugoroku-world/09_detach_todo.mir`, `samples/prototype/current-l2-dynamic-attach-detach/` | safe runtime attach/detach compatibility line | `HOT-01 prototype / HOT-03..06 planned` | `python3 scripts/sugoroku_world_samples.py closeout --format json` | current detach TODO only | not yet | lifecycle TODO note | yes | 10 | 2026-04-27 10:14 JST | no compatibility checker or lifecycle runtime yet | move from TODO boundary to first design cut | `0909`, `0912`, `0913` |
| PH15 | visualization / IDE | `mir_hilight.html`, `scripts/sugoroku_world_samples.py --debug *` | current viewer/debug surfaces before typed/redacted viewer | `VIS-01 current / VIS-02..08 planned` | browser/unit test not rerun in this task | helper debug output in Sugoroku runner | partial | highlighter + debug modes | yes | 50 | 2026-04-27 10:14 JST (helper debug only) | typed/redacted visualization absent | tie phase to TermSignature/VisualizationProtocol | `0910`, `0911`, `0913` |
| PH16 | compiler/backend/LLVM preparation | `scripts/env/mirrorea_storage_env.sh`, `scripts/storage/*` | keep LLVM/build/cache off root when possible | `LLVM-01 planned / LLVM-03,04 policy-only` | shell script dry-run | detach/storage audit only | not yet | storage location report | yes | 10 | 2026-04-27 10:14 JST | external storage not mounted; no backend probe sample yet | mount policy first, then backend probe | `0913` |

## Phase plan details

| Phase | Objective | Representative samples | Unit validation | E2E validation | Debug / visualization output | Completion criteria |
|---|---|---|---|---|---|---|
| 0 | reader/agent が specs -> plan -> snapshot -> report path を再構成できること | `P0-01..03` | `check_source_hierarchy.py`, `validate_docs.py` | docs -> report generation path | dashboard / report count | hierarchy check, dashboard, report, commit/push |
| 1 | guarded option chain / fallback / `atomic_cut` current semantics | `MIR-01..05` | cargo tests for AST/runtime | guided samples smoke | pretty/json trace | positive/negative + docs/report + validation |
| 2 | detached validation loop の artifact compare | `P2-01..04` | semantics tests | fixture -> artifact -> compare | detached artifact summary | detached-loop smoke + report |
| 3 | parser boundary と first checker cut | `P3-01..05` | `cargo test -p mir-ast` | source -> AST -> checker -> runtime preview | AST/test surface | parser/negative tests + docs/report |
| 4 | shared-space membership / authoritative room baseline | `P4-01..05` | Sugoroku membership checks | room change -> command -> history/reject | membership timeline | join/leave/reconnect sample family + report |
| 5 | finite-index typing + theorem/model-check boundary | `P5-01..07` | semantics/proof support tests | source -> checker -> theorem/model-check artifact | verification preview / proof graph | positive/negative + proof/model-check bridge + report |
| 6 | compile-ready minimal actualization | `P6-01..05` | cargo test trio | clean suite smoke-all | json/debug summary | closeout validation + docs/report |
| 7 | Sugoroku runtime attach vertical slice | `SUG-00..09` | `check-all` | bootstrap -> attach -> start -> roll -> handoff -> join/leave -> reset | summary/turn-trace/membership/verification | closeout pass, docs/report, detach still explicit TODO |
| 8 | avatar fairy follow / fallback anchor | `FAIRY-01..06` planned | future helper tests | attach -> fallback -> reacquire | anchor graph / fallback reason | active sample/helper + negative cases + report |
| 9 | typed external effect boundary / adapter | `EXT-01..05` planned | future adapter tests | request -> adapter -> typed receipt/failure | effect route graph | at least one adapter pass/fail sample + report |
| 10 | auth seam without transport collapse | `AUTH-01..05` planned | future envelope/auth tests | envelope -> auth -> authz -> membership -> dispatch | message/auth/membership flow | no hidden auth collapse + samples/report |
| 11 | shared term/layer signatures | `SIG-01..05` planned | future signature tests | source -> signatures -> layer composition | signature table / residual list | signature dump + docs/report |
| 12 | projection / placement | `PROJ-01..05` planned | future projection checks | system source -> projected stubs -> same trace | source/place mapping | projection checklist + examples + report |
| 13 | network transport | `NET-01..05` planned | future transport tests | server + participant process run Sugoroku turn | network timeline / telemetry | explicit typed envelope exchange + report |
| 14 | hot-plug package / `AttachPoint` | `HOT-01..06` planned plus current detach TODO boundary | future compatibility tests | manifest -> attach -> activate -> lifecycle trace | compatibility / activation cut view | compatibility/design cut + report |
| 15 | visualization / IDE | `VIS-01..08` planned plus current highlighter/debug modes | browser/helper tests | run sample -> open viewer -> inspect trace | viewer itself | typed/redacted viewer path + report |
| 16 | compiler/backend/LLVM prep | `LLVM-01..04` planned | storage/env shell checks | minimal compiled artifact path on external workdir | storage usage dashboard | backend probe + external cache path + cleanup evidence |

## Current blockers

| ID | Blocker | Affected samples | Owner | Next action |
|---|---|---|---|---|
| STOR-01 | extra storage `vdb` is attached but not mounted or formatted in a documented way | PH16, future heavy packages | repo maintainer + CodeX | keep policy/docs/scripts ready; do not cut over heavy artifacts yet |
| SUG-01 | Sugoroku row is broad but not yet split into tighter per-sample dashboard entries | PH4, PH7 | CodeX | run alignment package and tighten row/report mapping |
| FAIRY-01 | no active avatar fairy helper/sample family exists | PH8, PH14 | CodeX | promote a first active sample plan from current prototype anchor |
| SIG-01 | no `--debug signatures` output exists yet | PH11, PH15 | CodeX | implement `TermSignature registry / debug output` package |
| NET-01 | no separate-process/network transport sample exists | PH13 | mixed gate | keep docs-first until envelope/auth seam is ready |

## Recent validation

| Timestamp | Command | Result | Notes |
|---|---|---|---|
| 2026-04-27 09:13 JST | `git status --short` | pass | dirty baseline captured before work |
| 2026-04-27 09:13 JST | `df -h` | pass | root `/dev/vda2` has 27G free |
| 2026-04-27 09:13 JST | `lsblk -f` | pass | `vdb` visible, not mounted |
| 2026-04-27 09:13 JST | `findmnt` | pass | no external workdir mount present |
| 2026-04-27 09:13 JST | `du -sh .` | pass | repo about `5.3G` |
| 2026-04-27 09:13 JST | `du -sh target .git .cargo .lake` | partial | `target` `5.2G`, `.git` `66M`, `.cargo` and `.lake` missing |
| 2026-04-27 10:14 JST | `python3 scripts/check_source_hierarchy.py` | pass | hierarchy / report / dashboard path are present |
| 2026-04-27 10:14 JST | `python3 scripts/validate_docs.py` | pass | `Found 910 numbered report(s).` before this report was added |
| 2026-04-27 10:14 JST | `bash scripts/env/mirrorea_storage_env.sh` | pass | safe default paths exported; mounted flag is `no` |
| 2026-04-27 10:14 JST | `bash scripts/storage/detach_prepare.sh` | pass | non-destructive storage audit printed current root / mount / git state |
| 2026-04-27 10:14 JST | `bash scripts/storage/cleanup_disposable_artifacts.sh --list` | pass | candidate disposable paths listed only; no deletion |
| 2026-04-27 10:14 JST | `python3 scripts/current_l2_guided_samples.py smoke-all --format json` | pass | current-L2 active sample matrix succeeded |
| 2026-04-27 10:14 JST | `python3 scripts/current_l2_guided_samples.py closeout --format json` | pass | current-L2 closeout summary refreshed in this task |
| 2026-04-27 10:14 JST | `python3 scripts/clean_near_end_samples.py smoke-all --format json` | pass | clean near-end suite pass evidence was refreshed earlier in this task and summarized at closeout |
| 2026-04-27 10:14 JST | `python3 scripts/sugoroku_world_samples.py check-all` | pass | 10 Sugoroku samples passed unit/static guard checks |
| 2026-04-27 10:14 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | 10 Sugoroku samples passed closeout summary |
| 2026-04-27 10:14 JST | `cargo test -p mir-ast` | pass | crate tests and doctests passed earlier in this task; summarized at closeout |
| 2026-04-27 10:14 JST | `cargo test -p mir-runtime` | pass | crate tests and doctests passed earlier in this task; summarized at closeout |
| 2026-04-27 10:14 JST | `cargo test -p mir-semantics` | pass | semantics tests, support tests, and doctests passed earlier in this task; summarized at closeout |

## Historical / archived samples

| Old path | New status | Replacement | Notes |
|---|---|---|---|
| `samples/old/2026-04-22-pre-clean-near-end/` | archived | `samples/clean-near-end/` | historical pre-clean-near-end corpus; not active |
| `samples/lean/old/2026-04-22-pre-clean-near-end/` | archived | `samples/lean/clean-near-end/` | historical Lean carry-over only |
| `samples/prototype/current-l2-dynamic-attach-detach/` | prototype reference only | future phase 8 / 14 active samples | useful anchor for avatar/hot-plug planning, not active |
| `samples/not_implemented/` | conceptual / not active | phase rows at `1%` or `10%` in this dashboard | conceptual-only material must not be over-claimed |
