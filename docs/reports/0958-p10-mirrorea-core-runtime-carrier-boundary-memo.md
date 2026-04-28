# Report 0958 — P10 mirrorea-core runtime carrier boundary memo

- Date: 2026-04-28 13:47:46 +0900
- Author / agent: Codex (GPT-5)
- Scope: `crates/mir-runtime/src`、`crates/mir-runtime/tests`、および Mirrorea future-axis 関連 docs を読み、P10 `mirrorea-core` の minimal core carrier に今すぐ寄せてよい runtime-side concept と、helper-local / report-local に留めるべき concept を仕分ける。
- Decision levels touched: 既存 L1/L2 boundary の読解のみ。規範判断の変更はなし。

## 1. Objective

`place`、`membership`、`handoff`、`patch`、`AttachPoint`、`MessageEnvelope`、`auth`、`visualization`、`projection`、`hotplug` について、現行 `mir-runtime` がすでに stable-to-extract now と読める最小 carrier を持っているか、あるいは helper-local / report-local evidence floor に留まっているかを source-backed に整理する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/05-mirrorea-fabric.md`
- `plan/00-index.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/20-projection-and-placement-roadmap.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `docs/research_abstract/hands_on_sugoroku_02_runtime_attach.md`
- `docs/research_abstract/hands_on_sugoroku_07_message_envelope_auth.md`
- `docs/research_abstract/hands_on_sugoroku_08_visualization_protocol.md`
- `docs/hands_on/projection_placement_views_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `samples/clean-near-end/README.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/src/clean_near_end.rs`
- `crates/mir-runtime/src/bin/mir-current-l2.rs`
- `crates/mir-runtime/src/bin/mir-clean-near-end.rs`
- `crates/mir-runtime/tests/current_l2_runtime_skeleton.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `crates/mir-runtime/tests/clean_near_end_samples.rs`

## 3. Actions taken

1. AGENTS.md 指示どおり、repo の必須読書順と relevant plan/docs を先に確認した。
2. `mir-runtime` crate の top-level positioning を確認し、crate 全体が non-production thin skeleton だと再確認した。
3. `clean_near_end.rs` の exported report carriers、closeout inventory、sample-specific envelope / visualization / telemetry constructors を精読した。
4. `current_l2.rs` の `place` / `atomic_cut` / support-only runtime skeleton 側を確認した。
5. integration tests を読み、どの lane / name / scope が machine-check されているかを抽出した。
6. targeted tests を実行し、読んだ boundary が現行 test oracle と一致することを確認した。

## 4. Files changed

- 追加: `docs/reports/0958-p10-mirrorea-core-runtime-carrier-boundary-memo.md`
- `plan/` 更新不要: repository memory 自体は変わっていない。
- `progress.md` 更新不要: current status / roadmap / rough progress は変わっていない。
- `tasks.md` 更新不要: current task map は変わっていない。
- `samples_progress.md` 更新不要: runnable sample status は変わっていない。
- 規範文書 (`specs/`) 変更なし。

## 5. Commands run and exact outputs

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
```

Exact output:

```text
Task baseline recorded.
```

```bash
cargo test -p mir-runtime --test clean_near_end_samples --test current_l2_runtime_skeleton --test current_l2_source_lowering --test current_l2_operational_cli
```

Exact output summary:

```text
test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

## 6. Evidence / findings

### A. Stable-to-extract now: narrow runtime carrier only

1. `Place` as **opaque execution-locus carrier** is already stable enough.
   - Docs repeat the same meaning: execution locus, not participant/principal (`Documentation.md:53-58`, `specs/05-mirrorea-fabric.md:30-34`, `plan/14-glossary-and-boundary-rules.md:41-45`, `plan/16-shared-space-membership-and-example-boundary.md:7-10`, `plan/20-projection-and-placement-roadmap.md:19-30`).
   - Code already carries place identity without overcommitting to network/runtime realization: `MessageEnvelope.from_place/to_place`, `PrincipalClaim.participant_place`, projection focus subjects like `authority_place:...` / `place:...` (`crates/mir-runtime/src/clean_near_end.rs:700-737`, `1459-1515`).
   - `current_l2` lowering also treats nested `place { ... }` as first-class structural carrier and keeps `atomic_cut` place-local (`crates/mir-runtime/src/current_l2.rs:675-723`; tests `crates/mir-runtime/tests/current_l2_source_lowering.rs:25-44`, `382-463`).
   - Risk note: extract only opaque place identifiers + place-local boundary semantics. Do not pull in `world` sugar, server/client split, or emitted place-program logic.

2. `MessageEnvelope` / `PrincipalClaim` / optional `AuthEvidence` row shape is stable enough as a **minimal transport-adjacent carrier**.
   - Struct lanes are explicit in Rust (`crates/mir-runtime/src/clean_near_end.rs:700-737`).
   - Docs fixed the shared row vocabulary: `transport_medium / transport_seam / emitter_principal / freshness_checks / capability_requirements / authorization_checks / witness_refs`, plus `AuthEvidence kind / subject / issuer / bindings / notes` (`Documentation.md:63-68`, `plan/14-glossary-and-boundary-rules.md:70-80`, `docs/hands_on/current_phase_closeout_01.md:55-56`).
   - Tests machine-check the active runtime canonical seams and lane split (`crates/mir-runtime/tests/clean_near_end_samples.rs:260-318`, `550-603`).
   - Risk note: the stable part is the split, not the current payload catalog. Extract the carrier shape; do not freeze current sample IDs, payload names, or imply a final public transport ABI.

3. `membership` is stable only as a **freshness lane**, not as a registry/runtime subsystem.
   - Runtime envelope rows already pin `membership_epoch`, `member_incarnation`, and `freshness_checks` (`crates/mir-runtime/src/clean_near_end.rs:729-736`, `1371-1376`, `1403-1408`, `1430-1435`).
   - Docs explicitly allow `auth none` baseline while keeping membership freshness and witness refs separate (`Documentation.md:68`, `plan/16-shared-space-membership-and-example-boundary.md:32-37`).
   - Tests assert the freshness rows literally (`crates/mir-runtime/tests/clean_near_end_samples.rs:269-289`).
   - Risk note: do not extract `MembershipRegistry`, late-join insertion rules, or pending/active membership state from this crate. Those remain helper-side.

4. `handoff` is stable as **explicit witness/order discipline**, not as a dedicated emitted contract family.
   - Runtime sample specs encode handoff through `requires witness(...)`, `publication_order` / `witness_order` / `scoped_happens_before`, visible history, and proof obligations rather than a dedicated handoff artifact struct (`crates/mir-runtime/src/clean_near_end.rs:2391-2469` and nearby order-handoff sample specs).
   - Tests enforce the witness requirement and order constraints (`crates/mir-runtime/tests/clean_near_end_samples.rs:68-88`, `202-229`, `233-257`).
   - Docs still keep final witness/provider/public-contract and emitted handoff contract later (`plan/16-shared-space-membership-and-example-boundary.md:45-55`, `docs/hands_on/current_phase_closeout_01.md:73-75`).
   - Risk note: extract only the invariant that handoff is justified by explicit witness/order lanes. Do not invent a final handoff artifact schema from current sample evidence.

5. `LayerSignature` row schema is stable enough as the **cross-cutting layer inventory carrier** backing auth/transport/verification boundaries.
   - Row schema is explicit in code (`crates/mir-runtime/src/clean_near_end.rs:688-697`).
   - Runtime canonical inventory is fixed to `auth_authority_witness`, `transport_provider_boundary`, `verification_model_check`, while helper representative names remain separate (`Documentation.md:62-66`, `plan/14-glossary-and-boundary-rules.md:55-68`, `docs/hands_on/current_phase_closeout_01.md:55`).
   - Tests lock both the row lanes and the runtime canonical names (`crates/mir-runtime/tests/clean_near_end_samples.rs:202-257`, `428-447`, `500-547`).
   - Risk note: extract the row schema and canonical/runtime-vs-helper scope distinction, but do not treat current layer-name catalog as exhaustive.

6. `visualization` / `telemetry` have a stable **security envelope shape**, but only at carrier level.
   - Rust structs already fix `label / authority / redaction / retention_scope / source_refs` plus message/layer refs (`crates/mir-runtime/src/clean_near_end.rs:740-769`).
   - Runtime constructors mark the current rows as `report_local_inventory` and keep redaction explicit (`crates/mir-runtime/src/clean_near_end.rs:1465-1540`, `1555-1594`).
   - Tests assert those lanes and current report-local names (`crates/mir-runtime/tests/clean_near_end_samples.rs:321-419`, `606-670`).
   - Docs say the security envelope is fixed, but final viewer/telemetry service is not (`Documentation.md:64-65`, `plan/14-glossary-and-boundary-rules.md:82-86`, `docs/hands_on/current_phase_closeout_01.md:65-66`).
   - Risk note: extract only the typed evidence-egress envelope. Do not extract current view names, telemetry channels, or retention policies as stable protocol.

### B. Defer: helper-local or report-local only

1. `Patch` / `AttachPoint` / `hotplug` are not yet crate-stable carriers.
   - By inspection, `crates/mir-runtime/src` has no `Patch` or `AttachPoint` Rust type; the only crate-side anchors are reserved seam strings such as `attach_point_boundary` (`crates/mir-runtime/src/clean_near_end.rs:2993-3001`).
   - The docs place actual lifecycle evidence in Sugoroku helper/debug surfaces, not runtime canonical rows (`plan/21-hotplug-attachpoint-roadmap.md:12-18`, `28-68`; `samples/clean-near-end/sugoroku-world/README.md:66-68`, `103-108`).
   - Risk note: extracting a core hot-plug type now would freeze helper vocabulary as ABI before rollback / migration / activation law are decided.

2. Full `membership` subsystem semantics remain helper-local.
   - `MembershipRegistry` source-of-truth, late-join handoff boundary, epoch/incarnation invalidation flows, and attach/detach continuity are documented on the Sugoroku helper side (`Documentation.md:66-68`, `plan/16-shared-space-membership-and-example-boundary.md:17-25`, `samples/clean-near-end/sugoroku-world/README.md:49-58`, `80-81`).
   - The runtime crate only carries freshness stamps inside envelopes; it does not encode the registry or join/leave algorithm.
   - Risk note: pushing registry semantics into P10 core would collapse the helper vertical slice into a premature shared-space runtime model.

3. Concrete `auth` semantics beyond the empty baseline are deferred.
   - Closeout fixes only `auth_evidence_kinds = ["none"]` and reserves `session_token` / `signature` (`crates/mir-runtime/src/clean_near_end.rs:2982-2991`; tests `crates/mir-runtime/tests/clean_near_end_samples.rs:576-603`).
   - Docs explicitly keep session/signature/federation/multi-server identity later (`Documentation.md:68`, `docs/research_abstract/hands_on_sugoroku_07_message_envelope_auth.md`, `docs/hands_on/current_phase_closeout_01.md:73-75`).
   - Risk note: authority witness is not authentication; treating it as auth would violate the current non-collapse rule.

4. `projection` as a dedicated runtime object model is deferred.
   - Runtime-side `cross_place_projection` exists only as a `VisualizationView` with `report_local_inventory` retention (`crates/mir-runtime/src/clean_near_end.rs:1489-1516`).
   - The plan fixes only preview-floor validity categories and explicitly says this is not final emitted place program / projection IR / optimizer (`plan/20-projection-and-placement-roadmap.md:51-114`, `136-148`).
   - Tests only assert the preview view contents, not a standalone projection carrier (`crates/mir-runtime/tests/clean_near_end_samples.rs:359-390`).
   - Risk note: extracting `ProjectionView` as a core API now would harden a report-local preview into a false runtime contract.

5. Concrete `visualization` catalog and telemetry catalog are deferred.
   - `provider_boundary_redacted_flow`, `cross_place_projection`, and `authority_trace_redacted_view` are report-local inventory names, not canonical protocol families (`crates/mir-runtime/src/clean_near_end.rs:1465-1540`).
   - Helper docs are even more explicit that `visualization`, `projection`, and `hotplug` debug outputs are evidence-oriented helper-local surfaces, not final contracts (`samples/clean-near-end/sugoroku-world/README.md:84-108`).
   - Risk note: the stable part is the envelope, not the catalog.

6. Transport medium realization is deferred.
   - Runtime canonical closeout intentionally exposes seams but keeps `transport_mediums` empty and reserves `local_queue`, `loopback_socket`, `network_link` (`crates/mir-runtime/src/clean_near_end.rs:2987-3001`; tests `crates/mir-runtime/tests/clean_near_end_samples.rs:592-603`).
   - Docs say helper active mediums and runtime canonical seams are distinct (`Documentation.md:68`, `docs/hands_on/current_phase_closeout_01.md:56`).
   - Risk note: P10 core should carry seam-level distinctions first; medium taxonomy and real transport should remain later.

7. `mir_runtime::current_l2` pub surface is not itself a P10 extraction target.
   - The crate declares itself non-production and thin (`crates/mir-runtime/src/lib.rs:4-9`; `crates/mir-runtime/src/current_l2.rs:1-6`).
   - `plan/09` explicitly keeps most compile-ready `mir_runtime::current_l2` API in support-only / excluded buckets, with only `run_current_l2_source_sample` as the narrow first candidate (`plan/09-helper-stack-and-responsibility-map.md:913-923`, `944-1028`).
   - Risk note: using existing `pub` functions as mirrorea-core boundary would silently promote support helpers into architectural contract.

## 7. Changes in understanding

- `mir-runtime` の current stable surface は「Mirrorea object model」そのものより、**typed evidence carrier / canonical inventory carrier** の方が先に固まっている。
- stable と言えるのは、`place` / `envelope` / `auth lane` / `membership freshness` / `visualization security envelope` の **lane split** であって、`Patch` / `AttachPoint` / `projection IR` / final viewer contract / final auth protocol ではない。
- P10 でそのまま利用できるのは `clean_near_end` 側の row schema と non-collapse invariants であり、Sugoroku helper の lifecycle/debug catalog を core へ昇格させるのはまだ早い。

## 8. Open questions

- P10 minimal core carrier に `LayerSignature` を含めるか、`MessageEnvelope` / `VisualizationView` / `TelemetryRow` から参照される adjunct carrier として別置きにするか。
- `handoff` を core carrier に含める場合、current repo reading に忠実なのは dedicated struct ではなく witness/order/ref bundle だが、その bundle をどこまで generic にするか。
- `projection` について、P10 では invariant row / ref family だけを置くのか、それとも report-local `focus_subjects` 相当を generic ref array として先に切るのか。

## 9. Suggested next prompt

`docs/reports/0958-p10-mirrorea-core-runtime-carrier-boundary-memo.md` を前提に、P10 用に「今すぐ抽出してよい minimal Rust carrier set だけ」を 1 crate sketch として列挙してください。helper-local / report-local catalog は入れず、field 名の候補と kept-later notes だけ返してください。
