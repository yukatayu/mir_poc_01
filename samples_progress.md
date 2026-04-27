# samples_progress

Last updated: 2026-04-27 15:23 JST
Current repo-local focus: clean near-end current layer と Sugoroku world vertical slice を runnable floor として維持しつつ、Mirrorea future-axis を sample-first / docs-first に段階 actualize する
Current active packages: `0916` Sugoroku sample progress alignment close、`0917` Avatar fairy follow sample plan close、next `TermSignature registry / debug output`, `LayerSignature system`, `MessageEnvelope / Auth seam`

## Summary

| Area | Current % | Positive samples | Negative samples | E2E | Visualization | Report |
|---|---:|---:|---:|---|---|---|
| Phase 0 Repository memory and decision boundary | 90 | 2 | 1 | pass | dashboard/table | `0913` |
| Phase 1 Mir current L2 semantics | 90 | 2 | 3 | pass | pretty/json trace | `0904`, `0913` |
| Phase 2 Parser-free PoC / detached validation loop | 75 | 2 | 2 | partial | detached artifact summary | `0904`, `0913` |
| Phase 3 Parser boundary and first checker cut | 90 | 3 | 2 | pass | AST/checker test surface | `0904`, `0913` |
| Phase 4 Shared-space membership / room boundary | 90 | 6 | 1 | pass | membership timeline | `0909`, `0916` |
| Phase 5 Small decidable core and proof boundary | 90 | 4 | 3 | pass | verification/Lean/model-check preview | `0904`, `0913` |
| Phase 6 Compile-ready minimal actualization | 90 | 5 | 0 | pass | json/debug summary | `0904`, `0913` |
| Phase 7 Sugoroku runtime attach | 90 | 8 | 2 | pass | summary/turn-trace/membership/verification | `0909`, `0916` |
| Phase 8 Avatar fairy follow / fallback anchor | 10 | 0 | 0 | not yet | planned only | `0917` |
| Phase 9 Typed external boundary / adapter | 1 | 0 | 0 | not yet | not yet | `0913` |
| Phase 10 MessageEnvelope / authentication seam | 1 | 0 | 0 | not yet | not yet | `0912`, `0913` |
| Phase 11 TermSignature / LayerSignature | 1 | 0 | 0 | not yet | not yet | `0912`, `0913` |
| Phase 12 Projection / placement | 1 | 0 | 0 | not yet | not yet | `0912`, `0913` |
| Phase 13 Network transport | 1 | 0 | 0 | not yet | not yet | `0913` |
| Phase 14 Hot-plug package / AttachPoint | 10 | 0 | 1 | not yet | lifecycle TODO only | `0909`, `0912`, `0913` |
| Phase 15 Visualization / IDE | 50 | 1 | 0 | partial | `mir_hilight.html` + helper debug | `0910`, `0911`, `0913` |
| Phase 16 Compiler/backend/LLVM preparation | 50 | 1 | 0 | partial | storage location report + mounted target path | `0913`, `0915` |

## Build/storage environment

| Item | Status | Path | Notes |
|---|---|---|---|
| External workdir | mounted | `/mnt/mirrorea-work` | `/dev/vdb1` ext4 `mirrorea-work`, UUID `a87650a8-e3e9-4977-8940-6c293a0ee23c`, backed by `fstab` |
| Root setup helper | verified | `scripts/storage/setup_mirrorea_workdisk_root.sh` | GPT + ext4 + `/mnt/mirrorea-work` + UUID `fstab`; current session で mount 済み |
| Cargo target | externalized | `target/` -> `/mnt/mirrorea-work/cargo-target` | existing build artifact は SSD 側へ移送済み |
| Storage env script | yes | `scripts/env/mirrorea_storage_env.sh` | mounted default を確認しつつ safe path を export |
| LLVM build | path ready | `/mnt/mirrorea-work/llvm/{src,build,install}` | actual LLVM artifact はまだない |
| Generated artifacts | policy only | `/mnt/mirrorea-work/generated-artifacts` | heavy disposable artifact は root よりこちらを優先 |
| Detach prep script | yes | `scripts/storage/detach_prepare.sh` | non-destructive; status print only |
| Cleanup script | yes | `scripts/storage/cleanup_disposable_artifacts.sh` | requires `--confirm`; current default では safe guard 付き |

## Active sample matrix

| ID | Phase | Sample path | Purpose | Primary command | Preferred debug | Expected outcome | % | Last validation | Report | Notes |
|---|---|---|---|---|---|---|---:|---|---|---|
| `PH0` | 0 | `samples_progress.md`, `docs/reports/`, `scripts/check_source_hierarchy.py` | source hierarchy, report discipline, dashboard sync | `python3 scripts/check_source_hierarchy.py` | dashboard/table | success | 90 | 2026-04-27 10:19 JST | `0913` | `100%` には docs/report/commit/push close が要る |
| `PH1` | 1 | `samples/current-l2/` | guarded option chain / fallback / lineage / `atomic_cut` | `python3 scripts/current_l2_guided_samples.py smoke-all --format json` | pretty/json trace | success | 90 | 2026-04-27 10:14 JST | `0904`, `0913` | final public parser/API deferred |
| `PH2` | 2 | `samples/current-l2/`, `scripts/current_l2_detached_loop.py` | detached artifact compare loop | `cargo test -p mir-semantics` | detached artifact summary | partial | 75 | 2026-04-27 10:14 JST | `0904`, `0913` | dedicated detached-loop CLI refresh is still missing |
| `PH3` | 3 | `crates/mir-ast/tests/*stage*`, `samples/current-l2/` | parser boundary and first checker cut | `cargo test -p mir-ast` | AST/checker test surface | success | 90 | 2026-04-27 10:14 JST | `0904`, `0913` | final parser grammar deferred |
| `SUG-00` | 4 | `samples/clean-near-end/sugoroku-world/00_world_bootstrap.mir` | empty world and baseline membership | `python3 scripts/sugoroku_world_samples.py run 00_world_bootstrap --debug summary` | `summary` | success | 90 | 2026-04-27 15:21 JST via `check-all` | `0909`, `0916` | PH4 baseline |
| `SUG-01` | 7 | `samples/clean-near-end/sugoroku-world/01_runtime_attach_game.mir` | runtime attach of `SugorokuGame#1` | `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug summary` | `summary` | success | 90 | 2026-04-27 15:21 JST via `check-all` | `0909`, `0916` | PH7 attach floor |
| `SUG-02` | 4/7 | `samples/clean-near-end/sugoroku-world/02_admin_start_reset.mir` | admin-only start/reset control | `python3 scripts/sugoroku_world_samples.py run 02_admin_start_reset --debug summary` | `summary` | success | 90 | 2026-04-27 15:21 JST via `check-all` | `0909`, `0916` | control boundary |
| `SUG-03` | 7 | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | roll -> publish -> witness -> handoff | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary --format json` | `summary`, `turn-trace` | success | 90 | 2026-04-27 15:21 JST via focused run | `0909`, `0916` | core E2E row |
| `SUG-04` | 4 | `samples/clean-near-end/sugoroku-world/04_non_owner_roll_rejected.mir` | reject wrong-owner roll | `python3 scripts/sugoroku_world_samples.py run 04_non_owner_roll_rejected --debug summary` | `summary` | rejection | 90 | 2026-04-27 15:21 JST via `check-all` | `0909`, `0916` | negative guard |
| `SUG-05` | 4 | `samples/clean-near-end/sugoroku-world/05_late_join_history_visible.mir` | late join sees published history | `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership` | `membership` | success | 90 | 2026-04-27 15:21 JST via focused run | `0909`, `0916` | membership timeline anchor |
| `SUG-06` | 4 | `samples/clean-near-end/sugoroku-world/06_leave_non_owner.mir` | leave increments epoch and invalidates stale actions | `python3 scripts/sugoroku_world_samples.py run 06_leave_non_owner --debug membership` | `membership` | success | 90 | 2026-04-27 15:21 JST via `check-all` | `0909`, `0916` | leave invalidation |
| `SUG-07` | 4/7 | `samples/clean-near-end/sugoroku-world/07_owner_leave_reassign.mir` | owner leave triggers reassignment | `python3 scripts/sugoroku_world_samples.py run 07_owner_leave_reassign --debug membership` | `membership`, `summary` | success | 90 | 2026-04-27 15:21 JST via `check-all` | `0909`, `0916` | continuity under membership change |
| `SUG-08` | 5/7 | `samples/clean-near-end/sugoroku-world/08_reset_interleaving_model_check.mir` | reset invalidates old-epoch handoff | `python3 scripts/sugoroku_world_samples.py run 08_reset_interleaving_model_check --debug verification` | `verification` | success | 90 | 2026-04-27 15:21 JST via focused run | `0909`, `0916` | proof/model-check bridge |
| `SUG-09` | 14 | `samples/clean-near-end/sugoroku-world/09_detach_todo.mir` | explicit detach TODO boundary | `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug summary` | `summary` | explicit TODO | 10 | 2026-04-27 15:21 JST via `check-all` | `0909`, `0916` | stop line, not completion evidence |
| `PH5` | 5 | `samples/clean-near-end/typing`, `samples/clean-near-end/model-check`, `samples/lean/` | finite-index typing, theorem bridge, model-check second line | `python3 scripts/clean_near_end_samples.py smoke-all --format json` | verification preview / Lean stub | success | 90 | 2026-04-27 10:14 JST | `0904`, `0913` | proof/model-check binding absent |
| `PH6` | 6 | `samples/clean-near-end/` | compile-ready minimal actualization | `python3 scripts/clean_near_end_samples.py smoke-all --format json` | json/debug summary | success | 90 | 2026-04-27 10:14 JST | `0904`, `0913` | public shell/API deferred |
| `FAIRY-01..06` | 8 | `samples/not_implemented/avatar-fairy-follow/` | planned remote-head follow / fallback / rejection / model-check family | none yet | planned only | not yet | 10 | 2026-04-27 15:21 JST | `0917` | skeleton exists; no active helper/runner |
| `PH9` | 9 | `spec only` | typed external boundary / adapter | none yet | not yet | not yet | 1 | not yet | `0913` | no sample/helper exists |
| `PH10` | 10 | `spec only` | auth seam without transport collapse | none yet | not yet | not yet | 1 | not yet | `0912`, `0913` | no envelope carrier exists |
| `PH11` | 11 | `spec only` | shared term/layer signatures | none yet | not yet | not yet | 1 | not yet | `0912`, `0913` | next promoted package |
| `PH12` | 12 | `spec only` | system source -> place projection | none yet | not yet | not yet | 1 | not yet | `0912`, `0913` | no helper exists |
| `PH13` | 13 | `spec only` | separate-process / network transport | none yet | not yet | not yet | 1 | not yet | `0913` | no network transport sample exists |
| `PH15` | 15 | `mir_hilight.html`, helper debug modes | viewer + helper debug preview | browser open or sample helper runs | helper debug output | partial | 50 | 2026-04-27 10:14 JST | `0910`, `0911`, `0913` | typed/redacted visualization absent |
| `PH16` | 16 | `scripts/env/mirrorea_storage_env.sh`, `scripts/storage/*`, repo `target/` symlink | keep heavy artifacts off root | `bash scripts/env/mirrorea_storage_env.sh` | storage location report | partial | 50 | 2026-04-27 13:20 JST | `0913`, `0915` | cargo registry cache / LLVM actual probe still open |

## Phase plan details

| Phase | Objective | Representative samples | Unit validation | E2E validation | Debug / visualization output | Completion criteria |
|---|---|---|---|---|---|---|
| 0 | specs -> plan -> snapshot -> report path を再構成できること | `P0-01..03` | `check_source_hierarchy.py`, `validate_docs.py` | docs -> report generation path | dashboard / report count | hierarchy + dashboard + report + commit/push |
| 1 | guarded option chain / fallback / `atomic_cut` current semantics | `MIR-01..05` | cargo AST/runtime tests | guided samples smoke | pretty/json trace | positive/negative + docs/report |
| 2 | detached validation loop の artifact compare | `P2-01..04` | semantics tests | fixture -> artifact -> compare | detached artifact summary | detached-loop smoke + report |
| 3 | parser boundary と first checker cut | `P3-01..05` | `cargo test -p mir-ast` | source -> AST -> checker -> runtime preview | AST/test surface | parser/negative tests + docs/report |
| 4 | shared-space membership / authoritative room baseline | `SUG-00`, `SUG-02`, `SUG-04`, `SUG-05`, `SUG-06`, `SUG-07` | Sugoroku membership checks | room change -> command -> history/reject trace | membership timeline | join/leave/reject family + report |
| 5 | finite-index typing + theorem/model-check boundary | `P5-01..07`, `SUG-08` | semantics/proof support tests | source -> checker -> theorem/model-check artifact | verification preview / proof graph | positive/negative + proof/model-check bridge + report |
| 6 | compile-ready minimal actualization | `P6-01..05` | cargo test trio | clean suite smoke-all | json/debug summary | closeout validation + docs/report |
| 7 | Sugoroku runtime attach vertical slice | `SUG-01`, `SUG-03`, `SUG-07`, `SUG-08` | `check-all` | bootstrap -> attach -> start -> roll -> handoff -> join/leave -> reset | summary / turn-trace / membership / verification | closeout pass, docs/report, detach still explicit TODO |
| 8 | avatar fairy follow / fallback anchor | `FAIRY-01..06` planned | future helper tests | attach -> follow -> fallback -> reject -> reacquire | anchor graph / fallback reason / verification | active helper + positive/negative + report |
| 9 | typed external effect boundary / adapter | `EXT-01..05` planned | future adapter tests | request -> adapter -> typed receipt/failure | effect route graph | at least one adapter pass/fail sample + report |
| 10 | auth seam without transport collapse | `AUTH-01..05` planned | future envelope/auth tests | envelope -> auth -> authz -> membership -> dispatch | message/auth/membership flow | no hidden auth collapse + samples/report |
| 11 | shared term/layer signatures | `SIG-01..05` planned | future signature tests | source -> signatures -> layer composition | signature table / residual list | signature dump + docs/report |
| 12 | projection / placement | `PROJ-01..05` planned | future projection checks | system source -> projected stubs -> same trace | source/place mapping | projection checklist + examples + report |
| 13 | network transport | `NET-01..05` planned | future transport tests | server + participant process run a turn | network timeline / telemetry | explicit typed envelope exchange + report |
| 14 | hot-plug package / `AttachPoint` | `HOT-01..06` planned plus `SUG-09` | future compatibility tests | manifest -> attach -> activate -> lifecycle trace | compatibility / activation view | compatibility/design cut + report |
| 15 | visualization / IDE | `VIS-01..08` planned plus current highlighter/debug modes | browser/helper tests | run sample -> inspect trace | viewer itself | typed/redacted viewer path + report |
| 16 | compiler/backend/LLVM prep | `LLVM-01..04` planned | storage/env shell checks | mount -> target -> backend probe | storage usage dashboard | backend probe + external cache path + cleanup evidence |

## Current blockers

| ID | Blocker | Affected samples | Owner | Next action |
|---|---|---|---|---|
| `DETACH-01` | detach lifecycle is still an explicit TODO boundary | `SUG-09`, future `HOT-*` | CodeX | keep it visible until `AttachPoint` package exists |
| `FAIRY-01` | phase 8 now has skeleton files but still no active helper / runner | `FAIRY-01..06` | CodeX | promote helper contract after `TermSignature` / `LayerSignature` direction is clearer |
| `SIG-01` | no `--debug signatures` surface exists yet | `PH11`, `PH15` | CodeX | implement `TermSignature registry / debug output` |
| `NET-01` | no separate-process/network transport sample exists | `PH13` | mixed gate | wait for envelope/auth seam and projection line |
| `STOR-01` | mounted workdir exists but cargo registry cache / LLVM actual probe is not yet exercised | `PH16` | CodeX | run first backend / LLVM preparation package on `/mnt/mirrorea-work` |

## Recent validation

| Timestamp | Command | Result | Notes |
|---|---|---|---|
| 2026-04-27 13:20 JST | `findmnt /mnt/mirrorea-work` | pass | external workdir mount confirmed |
| 2026-04-27 13:20 JST | `grep '/mnt/mirrorea-work' /etc/fstab` | pass | UUID-based `defaults,nofail` entry exists |
| 2026-04-27 13:20 JST | `cargo test -p mir-ast --no-run` | pass | externalized `target/` path is usable |
| 2026-04-27 15:21 JST | `python3 scripts/sugoroku_world_samples.py check-all` | pass | 10 Sugoroku samples passed |
| 2026-04-27 15:21 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | closeout summary refreshed |
| 2026-04-27 15:21 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary --format json` | pass | `draw = 4`, witness `draw_pub#1`, handoff to Bob |
| 2026-04-27 15:21 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug turn-trace` | pass | publish -> handoff trace readable |
| 2026-04-27 15:21 JST | `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership` | pass | membership epoch and active/pending distinction visible |
| 2026-04-27 15:21 JST | `python3 scripts/sugoroku_world_samples.py run 08_reset_interleaving_model_check --debug verification` | pass | static/runtime/model-check lines readable |
| 2026-04-27 15:21 JST | `python3 scripts/current_l2_guided_samples.py run-source-sample samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.txt --host-plan samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.host-plan.json --format json` | expected fail | retired compatibility path; confirms prototype is not active runnable surface |
| 2026-04-27 15:23 JST | `python3 scripts/check_source_hierarchy.py` | pass | required `23`, missing `0` |
| 2026-04-27 15:23 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 915 numbered report(s).` |
| 2026-04-27 15:23 JST | `git diff --check` | pass | clean |

## Historical / archived samples

| Old path | New status | Replacement | Notes |
|---|---|---|---|
| `samples/old/2026-04-22-pre-clean-near-end/` | archived | `samples/clean-near-end/` | historical pre-clean-near-end corpus; not active |
| `samples/lean/old/2026-04-22-pre-clean-near-end/` | archived | `samples/lean/clean-near-end/` | historical Lean carry-over only |
| `samples/prototype/current-l2-dynamic-attach-detach/` | prototype reference only | `samples/not_implemented/avatar-fairy-follow/` then future phase 8 active helper | useful anchor for avatar/hot-plug planning, not active |
| `samples/not_implemented/avatar-fairy-follow/` | planned skeleton only | future phase 8 active sample family | phase 8 `%` stays `10%` until helper/validation exist |
