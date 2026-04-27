# samples_progress

Last updated: 2026-04-27 18:23 JST
Current repo-local focus: clean near-end current layer と Sugoroku world vertical slice を runnable floor として維持しつつ、Mirrorea future-axis を sample-first / docs-first に段階 actualize する
Current active packages: `0919` LayerSignature system close、`0920` repository layer map and staged restructuring close、`0921` MessageEnvelope / Auth seam close、`0922` VisualizationProtocol close、`0923` Typed external boundary / adapter close、`0924` Projection / placement close、`0925` HotPlug Patch / AttachPoint close、next `Network transport`, `Compiler/backend/LLVM prep`, `Hands-on closeout`

## Legend

Progress:
- 0%: not scheduled
- 1%: started; sample ID and goal exist
- 10%: spec/sample skeleton exists
- 25%: parser/loader/static carrier exists
- 50%: minimal implementation passes primary positive sample
- 65%: negative/rejection samples pass
- 75%: debug/visualization output exists
- 90%: E2E/regression/closeout validation passes
- 100%: complete for current scope: implementation + positive/negative samples + debug/visualization + docs + report + tests + progress update + git commit/push

## Summary

| Layer | Overall % | Status | Current focus | Next validation |
|---|---:|---|---|---|
| Mir core | 90 | active current layer | `samples/current-l2/` base corpus と current-L2 source execution を維持 | `python3 scripts/current_l2_guided_samples.py smoke-all --format json` |
| Typing / checker | 90 | active clean suite | finite-index first fragment と negative corpus を維持 | `python3 scripts/clean_near_end_samples.py run typing --format json` |
| Order / handoff | 90 | active clean suite | relation family / witness / authority handoff を維持 | `python3 scripts/clean_near_end_samples.py run order-handoff --format json` |
| Model-check | 90 | active clean suite | mutex / weak-memory second line と Sugoroku reset safety | `python3 scripts/clean_near_end_samples.py run model-check --format json` |
| Lean / theorem | 89 | active proof bridge | committed Lean bridge と repo-local stub alignment を維持 | `python3 scripts/current_l2_lean_sample_sync.py` |
| Sugoroku runtime | 90 | active vertical slice | attach / membership / handoff / reset model-check + debug lanes | `python3 scripts/sugoroku_world_samples.py closeout --format json` |
| Avatar follow | 10 | planned skeleton only | phase 8 helper contract と active promotion条件を切る | none yet |
| External adapters | 10 | planned skeleton | phase 9 `EXT-01..05` ladder を provider-boundary evidence に結び直す | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` |
| Visualization | 90 | helper-local + report-local first cut | `visualization_views` / `telemetry_rows` / redaction naming を維持しつつ package 3 へ渡す | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` |
| Projection / placement | 10 | docs-first plan | `plan/20` の validity checklist と stop line を維持しつつ hot-plug package へ渡す | `python3 scripts/validate_docs.py` |
| Hot-plug package | 10 | docs-first plan + TODO boundary | `plan/21` の compatibility/activation/migration stop line を維持しつつ transport widening へ渡す | `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --format json` |

## Active sample matrix

| Sample ID | Layer | Path | Kind | Progress | Positive/Negative | Last validation | Docs | Notes |
|---|---|---|---|---:|---|---|---|---|
| `PH0` | repository memory | `samples_progress.md`, `docs/reports/`, `scripts/check_source_hierarchy.py` | dashboard / hierarchy check | 90 | mixed | 2026-04-27 17:03 JST | `0913`, `0920` | source hierarchy と report discipline の current floor |
| `PH1` | Mir core | `samples/current-l2/` | base corpus | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | final parser / public API deferred |
| `PH2` | parser-free PoC | `samples/current-l2/`, `scripts/current_l2_detached_loop.py` | detached loop | 75 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | dedicated detached-loop CLI refresh remains open |
| `PH3` | parser / checker cut | `crates/mir-ast/tests/*stage*`, `samples/current-l2/` | crate test surface | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | parser grammar freeze deferred |
| `PH5` | theorem / model-check boundary | `samples/clean-near-end/{typing,model-check}`, `samples/lean/` | active bridge family | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | proof/model-check public contract absent |
| `PH6` | compile-ready minimal actualization | `samples/clean-near-end/` | active clean suite | 90 | positive + negative | 2026-04-27 15:59 JST | `0904`, `0913` | public shell / packaging deferred |
| `SUG-00` | shared-space membership | `samples/clean-near-end/sugoroku-world/00_world_bootstrap.mir` | active runnable | 90 | positive | 2026-04-27 15:21 JST | `0909`, `0916` | empty world baseline |
| `SUG-01` | Sugoroku runtime attach | `samples/clean-near-end/sugoroku-world/01_runtime_attach_game.mir` | active runnable | 90 | positive | 2026-04-27 15:21 JST | `0909`, `0916` | runtime attach floor |
| `SUG-02` | shared-space membership | `samples/clean-near-end/sugoroku-world/02_admin_start_reset.mir` | active runnable | 90 | positive | 2026-04-27 15:21 JST | `0909`, `0916` | admin-only start/reset |
| `SUG-03` | Sugoroku runtime | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | active runnable E2E | 90 | positive | 2026-04-27 15:59 JST | `0909`, `0916`, `0918`, `0919` | roll -> publish -> witness -> handoff |
| `SUG-04` | shared-space membership | `samples/clean-near-end/sugoroku-world/04_non_owner_roll_rejected.mir` | active runnable | 90 | negative | 2026-04-27 15:21 JST | `0909`, `0916` | wrong owner rejection |
| `SUG-05` | shared-space membership | `samples/clean-near-end/sugoroku-world/05_late_join_history_visible.mir` | active runnable E2E | 90 | positive | 2026-04-27 15:59 JST | `0909`, `0916`, `0919` | membership timeline anchor |
| `SUG-06` | shared-space membership | `samples/clean-near-end/sugoroku-world/06_leave_non_owner.mir` | active runnable | 90 | positive | 2026-04-27 15:21 JST | `0909`, `0916` | epoch invalidation |
| `SUG-07` | shared-space membership | `samples/clean-near-end/sugoroku-world/07_owner_leave_reassign.mir` | active runnable | 90 | positive | 2026-04-27 15:21 JST | `0909`, `0916` | owner reassignment |
| `SUG-08` | theorem / model-check boundary | `samples/clean-near-end/sugoroku-world/08_reset_interleaving_model_check.mir` | active runnable E2E | 90 | positive | 2026-04-27 15:21 JST | `0909`, `0916` | reset safety bridge |
| `SUG-09` | hot-plug preview | `samples/clean-near-end/sugoroku-world/09_detach_todo.mir` | active TODO boundary | 10 | explicit TODO | 2026-04-27 18:23 JST | `0909`, `0916`, `0925` | completion evidence ではなく stop line; `AttachPoint` package の current anchor |
| `LAY-01` | LayerSignature | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | helper-local inventory | 90 | positive | 2026-04-27 15:59 JST | `0919` | `verification` / `runtime_trace` layer inventory |
| `LAY-02` | LayerSignature | `samples/clean-near-end/sugoroku-world/05_late_join_history_visible.mir` | helper-local inventory | 90 | positive | 2026-04-27 15:59 JST | `0919` | `membership` layer inventory |
| `LAY-03` | LayerSignature | `samples/clean-near-end/order-handoff/05_delegated_rng_service.mir` | runtime report inventory | 90 | positive | 2026-04-27 15:59 JST | `0919` | `transport_provider_boundary` lane |
| `LAY-04` | LayerSignature | `samples/clean-near-end/order-handoff/06_auditable_authority_witness.mir` | runtime report inventory | 90 | positive | 2026-04-27 15:59 JST | `0919` | `auth_authority_witness` lane |
| `LAY-05` | LayerSignature | `samples/clean-near-end/model-check/01_peterson_sc_pass.mir` | runtime report inventory | 90 | positive | 2026-04-27 15:59 JST | `0919` | `verification_model_check` lane |
| `AUTH-01` | auth seam | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | helper-local envelope carrier | 90 | positive + negative | 2026-04-27 17:46 JST | `0921` | local queue baseline with `auth none`, membership freshness, capability refs, witness refs |
| `AUTH-02` | auth seam | `samples/clean-near-end/order-handoff/05_delegated_rng_service.mir` | runtime report-local envelope carrier | 90 | positive | 2026-04-27 17:46 JST | `0921` | provider boundary envelope keeps transport / witness / auth split explicit |
| `VIS-01` | visualization | `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir` | helper-local view/telemetry carrier | 90 | positive | 2026-04-27 18:04 JST | `0922` | `turn_timeline` / `message_route` / `verification_summary` with label-authority-redaction |
| `VIS-02` | visualization | `samples/clean-near-end/sugoroku-world/05_late_join_history_visible.mir` | helper-local membership view | 90 | positive | 2026-04-27 18:04 JST | `0922` | membership timeline stays redacted and published-history-only |
| `VIS-03` | visualization | `samples/clean-near-end/order-handoff/05_delegated_rng_service.mir` | runtime report-local view | 90 | positive | 2026-04-27 18:04 JST | `0922` | provider-boundary view stays downstream of layer/message inventory |
| `VIS-04` | visualization | `samples/clean-near-end/order-handoff/06_auditable_authority_witness.mir` | runtime report-local telemetry | 90 | positive | 2026-04-27 18:04 JST | `0922` | authority witness remains evidence, not auth |

## Base corpus / planned / spec-only matrix

| Sample ID | Layer | Path | Kind | Progress | Positive/Negative | Last validation | Docs | Notes |
|---|---|---|---|---:|---|---|---|---|
| `FAIRY-01..06` | avatar follow | `samples/not_implemented/avatar-fairy-follow/` | planned skeleton | 10 | not yet | 2026-04-27 15:21 JST | `0917` | skeleton exists; no helper / runner yet |
| `PH9` | typed external boundary | `samples/not_implemented/typed-external-boundary/`, `samples/clean-near-end/order-handoff/05_delegated_rng_service.mir` | planned docs-first row | 10 | planned | 2026-04-27 18:10 JST | `0913`, `0920`, `0923` | stdio を core primitive にしない boundary、provider boundary / local queue を evidence anchor に使う |
| `PH10` | MessageEnvelope / auth seam | `scripts/sugoroku_world_samples.py`, `crates/mir-runtime/src/clean_near_end.rs` | helper-local / report-local first cut | 90 | positive + negative | 2026-04-27 17:46 JST | `0912`, `0913`, `0920`, `0921` | `auth none` baseline、local queue / provider boundary、public auth contract deferred |
| `EXT-01` | typed external boundary | `samples/not_implemented/typed-external-boundary/README.md` | planned skeleton | 10 | not yet | 2026-04-27 18:10 JST | `0923` | `LogText` adapter local console; exact host schema open |
| `EXT-02` | typed external boundary | `samples/not_implemented/typed-external-boundary/README.md` | planned skeleton | 10 | not yet | 2026-04-27 18:10 JST | `0923` | `ShowFloatingText` world overlay; visualization label line remains typed |
| `EXT-03` | typed external boundary | `samples/not_implemented/typed-external-boundary/README.md` | planned skeleton | 10 | not yet | 2026-04-27 18:10 JST | `0923` | `SendRoomMessage` local queue; message route stays separate from auth |
| `EXT-04` | typed external boundary | `samples/not_implemented/typed-external-boundary/README.md` | planned skeleton | 10 | not yet | 2026-04-27 18:10 JST | `0923` | adapter failure typed result; transport failure remains explicit |
| `EXT-05` | typed external boundary | `samples/not_implemented/typed-external-boundary/README.md` | planned skeleton | 10 | not yet | 2026-04-27 18:10 JST | `0923` | debug visualization label restriction; no untyped leak |
| `PH12` | projection / placement | `plan/20-projection-and-placement-roadmap.md`, `docs/research_abstract/projection_placement_plan_01.md` | docs-first row | 10 | planned | 2026-04-27 18:15 JST | `0912`, `0913`, `0920`, `0924` | source-to-place validity checklist、place split、stop line を固定 |
| `PH13` | network transport | spec only | planned docs-first row | 1 | not yet | not yet | `0913`, `0920` | no separate-process sample exists |
| `PH15` | visualization / IDE | `scripts/sugoroku_world_samples.py`, `crates/mir-runtime/src/clean_near_end.rs`, `mir_hilight.html` | helper-local / report-local first cut | 90 | positive | 2026-04-27 18:04 JST | `0910`, `0911`, `0913`, `0918`, `0920`, `0922` | typed/redacted visualization actualized; final public viewer and retention remain deferred |
| `PH16` | compiler/backend/LLVM prep | `scripts/env/mirrorea_storage_env.sh`, `scripts/storage/*` | ops guardrail | 50 | partial | 2026-04-27 13:20 JST | `0913`, `0915`, `0920` | cargo registry cache / LLVM actual probe still open |

## E2E samples

| E2E ID | Scope | Path / command | Progress | What it proves | Last result |
|---|---|---|---:|---|---|
| `E2E-CLEAN-SMOKE` | clean near-end family run | `python3 scripts/clean_near_end_samples.py smoke-all --format json` | 90 | typing / order-handoff / model-check / modal の active clean suite floor | pass 2026-04-27 15:59 JST |
| `E2E-SUG-ATTACH` | membership -> attach -> start -> roll -> publish -> witness -> handoff | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary --format json` | 90 | logical multi-Place vertical slice の中心線 | pass 2026-04-27 15:59 JST |
| `E2E-SUG-HISTORY` | membership -> late join -> published history visibility | `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership` | 90 | epoch / active-pending / published-history visibility line | pass 2026-04-27 15:59 JST |
| `E2E-SUG-RESET` | runtime attach -> reset -> stale handoff invalidation | `python3 scripts/sugoroku_world_samples.py run 08_reset_interleaving_model_check --debug verification` | 90 | reset safety と model-check second line の bridge | pass 2026-04-27 15:21 JST |
| `E2E-TERM-LAYER-INV` | source sample -> helper/runtime inventory | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures` and `--debug layers` | 90 | helper-local `TermSignature` / `LayerSignature` inventory floor | pass 2026-04-27 15:59 JST |
| `E2E-AVATAR-FALLBACK` | attach -> follow -> fallback -> reject -> reacquire | planned | 10 | planned phase 8 route only | not yet |
| `E2E-ADAPTER-BOUNDARY` | source -> adapter -> typed effect receipt/failure | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` and `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json` | 10 | current evidence anchor for provider-boundary / local-queue phase 9 ladder | plan fixed 2026-04-27 18:10 JST |
| `E2E-MSG-AUTH` | envelope -> auth -> authz -> membership -> dispatch | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json` | 90 | helper-local envelope carrier keeps auth / membership / capability / witness separate | pass 2026-04-27 17:46 JST |
| `E2E-VIS-TRACE` | source -> runtime trace -> visualization view -> telemetry row | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` | 90 | helper-local typed/redacted visualization first cut | pass 2026-04-27 18:04 JST |
| `E2E-HOTPLUG-ATTACH` | patch -> compatibility -> attach -> activate | `plan/21-hotplug-attachpoint-roadmap.md` | 10 | docs-first compatibility / activation / migration stop line fixed; executable helper is not yet present | plan fixed 2026-04-27 18:23 JST |
| `E2E-PROJECTION` | system source -> place-specific projection -> equivalent trace | `plan/20-projection-and-placement-roadmap.md` | 10 | docs-first validity checklist fixed; executable projection helper is not yet present | plan fixed 2026-04-27 18:15 JST |

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

## Current blockers

| Blocker | Layer | Severity | Owner | Next action |
|---|---|---|---|---|
| detach lifecycle is still an explicit TODO boundary | hot-plug | medium | CodeX | keep `SUG-09` visible until executable AttachPoint / migration helper exists |
| phase 8 has skeleton files but still no active helper / runner | avatar follow | medium | CodeX | promote helper contract after `MessageEnvelope` / `Visualization` direction is clearer |
| final public visualization contract / retention / multi-tenant telemetry が未決 | visualization | medium | mixed gate | keep helper/report-local first cut only and reopen after adapter / projection packages |
| final public `AuthEvidence` kind と real transport widening が未決 | auth / transport | medium | mixed gate | keep helper-local `auth none` baseline and reopen with adapter / network packages |
| no separate-process / network transport sample exists | network transport | medium | mixed gate | wait for envelope/auth seam and projection line |
| mounted workdir exists but cargo registry cache / LLVM actual probe is not yet exercised | backend / storage | low | CodeX | run first backend / LLVM preparation package on `/mnt/mirrorea-work` |
| repository taxonomy drift can reintroduce active/planned/generated confusion | docs / structure | medium | CodeX | keep `plan/19`, `samples/README.md`, `scripts/README.md`, `0920` report in sync |

## Recent validation

| Date | Command | Result | Notes |
|---|---|---|---|
| 2026-04-27 18:23 JST | `python3 scripts/check_source_hierarchy.py` | pass | hot-plug docs-first package did not introduce hierarchy drift |
| 2026-04-27 18:23 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 923 numbered report(s).` |
| 2026-04-27 18:23 JST | `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --format json` | pass | attach request / runtime attach envelope remain current hot-plug anchor |
| 2026-04-27 18:23 JST | `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --format json` | pass | detach TODO boundary remains explicit stop line, not completion evidence |
| 2026-04-27 18:23 JST | `python3 scripts/sugoroku_world_samples.py check-all` | pass | all 10 Sugoroku samples remain green after hot-plug docs-first sync |
| 2026-04-27 18:15 JST | `python3 scripts/check_source_hierarchy.py` | pass | projection docs-first package did not introduce hierarchy drift |
| 2026-04-27 18:15 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 922 numbered report(s).` |
| 2026-04-27 18:15 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | provider boundary remains current projection evidence anchor |
| 2026-04-27 18:15 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` | pass | place split remains visible on helper-local visualization view |
| 2026-04-27 18:10 JST | `python3 scripts/check_source_hierarchy.py` | pass | planned phase 9 sample root added without hierarchy drift |
| 2026-04-27 18:10 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 921 numbered report(s).` |
| 2026-04-27 18:10 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | provider boundary remains current phase 9 evidence anchor |
| 2026-04-27 18:10 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json` | pass | local-queue envelope remains current phase 9 evidence anchor |
| 2026-04-27 18:04 JST | `python3 scripts/check_source_hierarchy.py` | pass | required `23`, present `23`, missing `0` |
| 2026-04-27 18:04 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 920 numbered report(s).` |
| 2026-04-27 18:04 JST | `git diff --check` | pass | whitespace-clean after visualization sync |
| 2026-04-27 18:04 JST | `cargo test -p mir-runtime` | pass | full runtime suite remained green after visualization carrier additions |
| 2026-04-27 18:04 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | runtime closeout emits `visualization_views` / `telemetry_rows` |
| 2026-04-27 18:04 JST | `cargo test -p mir-runtime --test clean_near_end_samples` | pass | runtime-side visualization/telemetry tests included |
| 2026-04-27 18:04 JST | `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | helper closeout emits visualization/telemetry inventory |
| 2026-04-27 18:04 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json` | pass | helper-local typed/redacted view and telemetry visible |
| 2026-04-27 18:04 JST | `python3 scripts/sugoroku_world_samples.py check-all` | pass | all 10 Sugoroku samples still pass with visualization carrier enabled |
| 2026-04-27 18:04 JST | `python3 -m unittest scripts.tests.test_sugoroku_world_samples` | pass | helper-side visualization tests included |
| 2026-04-27 17:46 JST | `python3 scripts/check_source_hierarchy.py` | pass | required `23`, present `23`, missing `0` |
| 2026-04-27 17:46 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 919 numbered report(s).` |
| 2026-04-27 17:46 JST | `git diff --check` | pass | whitespace-clean after `0921` report and snapshot sync |
| 2026-04-27 17:46 JST | `cargo test -p mir-runtime --test clean_near_end_samples` | pass | runtime-side MessageEnvelope inventory tests included |
| 2026-04-27 17:46 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json` | pass | runtime report-local `message_envelopes` visible on provider boundary |
| 2026-04-27 17:46 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | closeout emits `message_envelope_lanes`, `auth_evidence_kinds`, `transport_seams` |
| 2026-04-27 17:46 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json` | pass | helper-local `message_envelopes` visible on local-queue baseline |
| 2026-04-27 17:46 JST | `python3 scripts/sugoroku_world_samples.py check-all` | pass | 10 Sugoroku samples still pass with envelope carrier enabled |
| 2026-04-27 17:46 JST | `python3 -m unittest scripts.tests.test_sugoroku_world_samples` | pass | helper-side MessageEnvelope tests included |
| 2026-04-27 17:03 JST | `python3 scripts/check_source_hierarchy.py` | pass | required `23`, present `23`, missing `0` |
| 2026-04-27 17:03 JST | `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete.`, `Found 918 numbered report(s).` |
| 2026-04-27 17:03 JST | `git diff --check` | pass | report / snapshot sync after final reviewer fixes is whitespace-clean |
| 2026-04-27 13:20 JST | `findmnt /mnt/mirrorea-work` | pass | external workdir mount confirmed |
| 2026-04-27 13:20 JST | `cargo test -p mir-ast --no-run` | pass | externalized `target/` path usable |
| 2026-04-27 15:21 JST | `python3 scripts/sugoroku_world_samples.py check-all` | pass | 10 Sugoroku samples passed |
| 2026-04-27 15:21 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug turn-trace` | pass | publish -> handoff trace readable |
| 2026-04-27 15:21 JST | `python3 scripts/sugoroku_world_samples.py run 08_reset_interleaving_model_check --debug verification` | pass | reset safety bridge readable |
| 2026-04-27 15:59 JST | `python3 -m unittest scripts.tests.test_sugoroku_world_samples` | pass | helper-side Term/Layer inventory tests |
| 2026-04-27 15:59 JST | `cargo test -p mir-runtime --test clean_near_end_samples` | pass | runtime-side Term/Layer inventory tests |
| 2026-04-27 15:59 JST | `python3 scripts/current_l2_guided_samples.py smoke-all --format json` | pass | current-L2 smoke after Term/Layer work |
| 2026-04-27 15:59 JST | `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers` | pass | helper-local layer inventory view |
| 2026-04-27 15:59 JST | `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json` | pass | runtime closeout emits `layer_signatures` |
| 2026-04-27 15:59 JST | `python3 scripts/check_source_hierarchy.py` | pass | required `23`, missing `0` |
| 2026-04-27 15:59 JST | `python3 scripts/validate_docs.py` | pass | documentation scaffold complete |
| 2026-04-27 15:59 JST | `git diff --check` | pass | whitespace-clean at `0919` close |
| 2026-04-27 15:21 JST | `python3 scripts/current_l2_guided_samples.py run-source-sample samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.txt --host-plan samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.host-plan.json --format json` | expected fail | prototype is not active runnable surface |

## Historical / archived samples

| Old path | New path / status | Reason |
|---|---|---|
| `samples/old/2026-04-22-pre-clean-near-end/` | archived; active replacement is `samples/clean-near-end/` | historical pre-clean-near-end corpus; not active |
| `samples/lean/old/2026-04-22-pre-clean-near-end/` | archived; active replacement is `samples/lean/clean-near-end/` | historical Lean carry-over only |
| `samples/prototype/current-l2-dynamic-attach-detach/` | prototype reference only | useful anchor for avatar/hot-plug planning, but not active |
