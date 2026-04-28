# progress

最終更新: 2026-04-28 13:37 JST

## この文書について

- この文書は repo 全体の **rough progress snapshot** です。
- 規範判断の正本は `specs/`、長期参照は `plan/`、runnable sample dashboard は `samples_progress.md`、詳細証跡は `docs/reports/` にあります。
- 進捗率は **repo-local alpha-ready current layer** と **Mirrorea future-axis docs-first / sample-first integration** に scoped した rough estimate であり、final public completion を意味しません。

## project axis

```text
正しい理論に基づき、
正しく hot-plug でき、
Place をまたいで実行・通信・検証・可視化できる
仮想空間システムを作る。
```

## current snapshot

- active floor:
  clean near-end suite、Sugoroku world vertical slice、avatar follow representative slice は runnable
- future-axis floor:
  `TermSignature`、`LayerSignature`、`MessageEnvelope / AuthEvidence` seam、`VisualizationProtocol`、typed external synthetic preview helper、projection preview、hot-plug helper-local lifecycle canary、network transport helper-local canary までは actualize 済み
- integration floor:
  `P0` current-state audit、`P1` repository layer map / `samples_progress.md` stabilization、`P2` typed external residual planned family review、`P3` projection / placement residual emitted-program gate、`P4` `TermSignature` registry hardening、`P5` `LayerSignature` system hardening、`P6` `MessageEnvelope / AuthEvidence` seam hardening、`P7` `VisualizationProtocol / VisualizationSecurity` hardening、`P8` Sugoroku runtime attach hardening、`P9` avatar fairy follow hardening を close し、handoff 由来の queue numbering と source hierarchy 読み、typed external residual reopen matrix、projection validity report minimum、generated artifact reserve policy、current `signature_lanes` / `signature_scope` / `signature_evidence_roles` rule、current `LayerSignature` row schema / scope split / representative vs canonical inventory rule、`MessageEnvelope` medium/seam split、shared `AuthEvidence` lane inventory、view / telemetry security envelope、fail-closed observer route trace、MembershipRegistry source-of-truth wording、world sugar boundary、hot-plug stop line、`FAIRY-05` reopen gate と planned path inventory を current repo へ mirror した
- current promoted next line:
  `P10` `mirrorea-core` first real implementation tranche
- next reopen point:
  `P11` logical multi-place runtime tranche
- still later:
  `P11-P17` implementation tranche、`P18` public API / parser grammar gate
- architectural caution:
  effect-based OS-like substrate は内側の解釈に留め、Mir core standard I/O primitive や subsystem collapse を既成事実化しない
- verification caution:
  `VerificationLayer` composition は helper `verification_handoff_witness` と runtime `verification_model_check` を current emitted floor とする typed layer composition の current reading に留め、hidden verifier builtin や final public verifier contract と混同しない

## 現在の一言での読み

2026-04-28 時点の repo は、**current-L2 / shared-space sample の runnable floor を維持したまま、Mirrorea future-axis の future-plan integration、next package queue stabilization、`P2` typed external residual planned family review、`P3` projection emitted-program gate、`P4` `TermSignature` registry hardening、`P5` `LayerSignature` system hardening、`P6` `MessageEnvelope / AuthEvidence` seam hardening、`P7` `VisualizationProtocol / VisualizationSecurity` hardening、`P8` Sugoroku runtime attach hardening、`P9` avatar fairy follow hardening を close し、次に `P10` `mirrorea-core` first real implementation tranche と `P11` logical multi-place runtime tranche を整理する段階**です。

## 3 軸 progress

| layer / track | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 現在の読み |
|---|---:|---:|---:|---|
| Mir core / current-L2 | 90% | 88% | 80% | finite-index current layer は強いが、final parser / public API は未完 |
| order / handoff / cut family | 90% | 90% | 80% | high-level relation line は runnable。final source wording と public artifact contract は残る |
| theorem / model-check boundary | 92% | 90% | 86% | repo-local bridge は強いが、concrete external binding は未完 |
| Lean foundations / proof spine | 86% | 88% | 68% | small proof fragment と generated stub はあるが、full discharge ではない |
| shared-space samples | 84% | 87% | 75% | Sugoroku / avatar の runnable floor はあるが、real transport / durable evidence は未着手 |
| docs / dashboard / repository structure | 93% | 94% | 86% | source hierarchy、queue numbering、taxonomy、dashboard semantics は current line に追随 |
| Mirrorea future axis | 82% | 91% | 70% | helper-local / report-local preview は揃ったが、public contract と emitted artifact family は後段 |
| storage / backend guardrail | 72% | 83% | 76% | external workdir と probe floor はあるが、actual LLVM build はまだ |

## macro phase map

| Macro phase | 主眼 | 現在位置 | rough progress % | 自走可否 |
|---|---|---:|---:|---|
| `Macro 0` | repository memory / docs / traceability | active | 99% | 着手可能 |
| `Macro 1` | semantic kernel / invariant stabilization | late | 87% | 着手可能 |
| `Macro 2` | parser-free validation substrate | late | 88% | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | late | 84% | public parser / API 以外は着手可能 |
| `Macro 4` | executable sample floor | active clean suite | 98% | 着手可能 |
| `Macro 5` | typed / theorem / model-check bridge | repo-local alpha-ready current layer | 96% | public seam 以外は自走可能 |
| `Macro 6` | shared-space / fabric boundary | `P10` promoted + `P11` reopen待ち | 95% | public contract / real transport 以外は自走可能 |
| `Macro 7` | toolchain / backend / host-facing integration | guardrail + implementation tranche待ち | 72% | installed binary / backend choice 以外は段階的に自走可能 |
| `Macro 8` | application realization | early | 22% | 要仕様確認 |

## feature family snapshot

| feature family | 現在地 | できていること | まだ残ること |
|---|---|---|---|
| executable sample corpus | `S6` | active clean suite、Sugoroku、avatar representative slice、dashboard | final public sample catalog |
| finite-index typing / order-handoff | `S6` | authority / label / capture / region / cost finite theory、high-level relation family | final typed source principal と public checker / artifact contract |
| theorem / model-check / Lean | `S5-S6` | model-check second line、small Lean proof fragment、generated stub corpus | full domain discharge と concrete external tool binding |
| shared-space runtime samples | `S6` | attach / membership / handoff / late join / follow / fallback / reset safety。P8 closeout で world sugar / MembershipRegistry / stop line wording、P9 closeout で `FAIRY-05` reopen gate / planned path inventory を明示 | detach lifecycle residual、real transport、durable evidence |
| typed external / projection / hot-plug / transport preview | `S5-S6` | typed external synthetic preview、typed external residual reopen matrix、projection preview、projection validity report minimum、generated artifact reserve policy、hot-plug lifecycle canary、`NET-01..05` helper-local canary | final host-facing contract、actual emitted program family、real migration / replay |
| verification / visualization composition | `S5` | `TermSignature` current registry hardening、`LayerSignature` row schema / scope split / obligations lane、`MessageEnvelope` medium/seam split、shared `AuthEvidence` lane inventory、report-local inventories、typed visualization / telemetry security envelope と fail-closed route trace | exact public `VerificationLayer` law surface、public viewer / verifier contract |
| repository structure / dashboard | `S6` | layer-aware repo map、sample/script taxonomy、front-door docs、snapshot docs | risky crate/path move をまだしていない |
| storage / backend guardrail | `S5` | external workdir、`target/` cutover、`CARGO_HOME` probe、LLVM path readiness | actual LLVM build、backend choice、packaging target |

## next package queue

| Package | Status | Objective | Stop line |
|---|---|---|---|
| `P0` current-state audit | closed | source hierarchy / stale reference / docs drift を補正する | risky move や public freeze を audit task に混ぜない |
| `P1` layer map / samples dashboard stabilization | closed | taxonomy と dashboard semantics を揃える | planned/helper/final-public の混同を再導入しない |
| `P2` typed external residual review | closed | `EXT-01/02/05` residual planned family の indirect anchor / reopen criterion / kept-later gate を固定した | final host schema / adapter API を固定しない |
| `P3` projection emitted-program gate | closed | preview floor と emitted program family の boundary、validity report minimum、generated artifact reserve policy を固定する | actual emitted place-specific program family を claim しない |
| `P4` TermSignature hardening | closed | current `signature_lanes` / `signature_scope` / `signature_evidence_roles` rule と reserved kind split を fixed した | final public signature schema を claim しない |
| `P5` LayerSignature hardening | closed | helper/runtime `LayerSignature` row schema、`obligations` lane、scope split、inventory naming を fixed した | final public layer law schema を claim しない |
| `P6` MessageEnvelope/AuthEvidence hardening | closed | helper/runtime `message_envelope_scope`、`transport_medium` / `transport_seam`、`emitter_principal`、`freshness_checks`、shared `auth_evidence_lanes` を fixed した | final public auth schema、final public transport ABI、`witness_refs` role taxonomy を claim しない |
| `P7` visualization security hardening | closed | view / telemetry security envelope、typed telemetry retention floor、fail-closed observer route trace を固定した | final public viewer contract、retention policy、multi-tenant telemetry service を固定しない |
| `P8` Sugoroku runtime attach hardening | closed | MembershipRegistry source-of-truth、world sugar boundary、late-join/handoff boundary、hot-plug stop line を固定した | real network、consensus、durable distributed commit、rollback、final public runtime / hot-plug ABI を固定しない |
| `P9` avatar fairy follow hardening | closed | helper closeout `FAIRY-05` reopen gate と planned path inventory を fixed した | `FAIRY-05` 自体は active 化しない |
| `P10` `mirrorea-core` first real implementation tranche | queued; next | placeholder / preview floor から first real core へ進める | subsystem collapse や premature freeze をしない |
| `P11-P17` implementation tranche | later | logical multi-place runtime / adapter / transport / hot-plug / projection / viewer / backend を段階的に widen する | `P10` ownership cut 前に責務を collapse しない |
| `P18` public API / parser grammar gate | final mixed gate | final freeze 条件を定義する | prior packages 未成熟のまま public freeze しない |

## 着手可能 / 要仕様確認 / 後段依存

| 項目 | 状態 | 理由 |
|---|---|---|
| `P2` Typed external boundary residual planned family review | close 済み | residual `EXT-01` / `EXT-02` / `EXT-05` の indirect anchor / reopen criterion / kept-later gate を docs-first に固定した |
| `P3` Projection / placement residual emitted-program gate | close 済み | preview floor と later emitted-program family の boundary、validity report minimum、generated artifact reserve policy を docs-first に固定した |
| `P4` `TermSignature` registry hardening | close 済み | current `signature_lanes` / `signature_scope` / `signature_evidence_roles` rule を fixed し、helper-local representative slice と clean near-end canonical inventory を区別した |
| `P5` `LayerSignature` system hardening | close 済み | helper/runtime `LayerSignature` row schema、`obligations` lane、scope split、representative/canonical inventory distinctionを fixed した |
| `P6` `MessageEnvelope / AuthEvidence` seam hardening | close 済み | helper/runtime `message_envelope_scope`、medium/seam split、shared `AuthEvidence` lane inventory、freshness lane、subject/emitter distinction を fixed した |
| `P7` visualization security hardening | close 済み | helper/runtime security envelope、typed telemetry、NET-05 fail-closed route trace を current line に固定した |
| `P8` Sugoroku runtime attach hardening | close 済み | helper closeout に world sugar / MembershipRegistry / stop line carrier を追加し、attach / handoff / late-join / detach evidence gate を current line に固定した |
| `P9` avatar fairy follow hardening | close 済み | helper closeout `FAIRY-05` reopen gate と planned path inventory を current line に固定した |
| `P10` `mirrorea-core` first real implementation tranche | 着手可能 | helper-local preview と docs-first queue が揃っており、crate responsibility boundary から始められる |
| `P11-P17` implementation tranche | 後段依存 | `P10` ownership cut と minimal core carrier が先に要る |
| `P18` public API / parser grammar gate | 要仕様確認 | prior tranche の成熟と user 側の公開範囲判断が必要 |

## 再現性アンカー

- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `python3 scripts/current_l2_guided_samples.py closeout --format json`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `python3 scripts/avatar_follow_samples.py closeout --format json`
- `python3 scripts/typed_external_boundary_samples.py closeout --format json`
- `python3 scripts/network_transport_samples.py closeout --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
- `cargo test -p mir-ast`
- `cargo test -p mir-runtime`
- `cargo test -p mir-semantics`
- `git diff --check`

## recent log

- 2026-04-28 13:37 JST — `P9` avatar fairy follow hardening を close し、avatar helper closeout に `planned_sample_paths` と `fairy05_reopen_gate = { sample_status = planned_only, required_evidence = [...], carrier_choice = UNRESOLVED, planning_only_candidate_labels = state_timeline / anchor_switch }` を追加した。avatar helper unittest、`check-all`、avatar closeout、`01` / `02` anchors、`03` membership、`06` verification、`check_source_hierarchy.py`、`validate_docs.py`、`git diff --check` を通し、report `0956` と snapshot docs を `P10` promoted / `P11` reopen next に同期した。
- 2026-04-28 13:09 JST — `P8` Sugoroku runtime attach hardening を close し、Sugoroku helper closeout に `world_surface = host_server_side_sugar`、`membership_model.source_of_truth = MembershipRegistry`、`membership_model.late_join_handoff_boundary`、`hotplug_stop_line` を追加した。`01_runtime_attach_game --debug hotplug`、`03_roll_publish_handoff --debug envelopes`、`05_late_join_history_visible --debug membership`、`09_detach_todo --debug hotplug`、Sugoroku closeout、Sugoroku unittest、`check_source_hierarchy.py`、`validate_docs.py`、`git diff --check` を通し、report `0955` と snapshot docs を `P9` promoted / `P10` reopen next に同期した。
- 2026-04-28 12:42 JST — `P7` `VisualizationProtocol / VisualizationSecurity` hardening を close し、Sugoroku helper と clean near-end runtime の view / telemetry security envelope に `label` / `authority` / `redaction` / `retention_scope` / `source_refs` を追加した。NET-05 observer route trace は fail-closed に固定し、typed telemetry を security boundary の内側に戻した。Sugoroku / network helper closeout、clean near-end focused run / closeout、`cargo test -p mir-runtime --test clean_near_end_samples`、`cargo test -p mir-runtime`、`check_source_hierarchy.py`、`validate_docs.py`、`git diff --check` を通し、report `0954` と snapshot docs を `P8` promoted / `P9` reopen next に同期した。
- 2026-04-28 12:18 JST — `P6` `MessageEnvelope / AuthEvidence` seam hardening を close し、helper/runtime `message_envelope_scope`、`transport_medium` / `transport_seam`、`emitter_principal`、`freshness_checks`、shared `auth_evidence_lanes` を固定した。review follow-up で legacy `transport` alias を helper/runtime ともに seam 意味へ正規化し、snapshot docs の next reopen point を `P8` へ揃え、report `0953` に full validation evidence を追記した。helper/network unittest、Sugoroku / network closeout、clean near-end focused run / closeout、`cargo test -p mir-runtime --test clean_near_end_samples`、`cargo test -p mir-runtime`、`check_source_hierarchy.py`、`validate_docs.py`、`git diff --check` を通した。
- 2026-04-28 11:19 JST — `P5` `LayerSignature` system hardening を close し、Sugoroku helper と clean near-end runtime の `LayerSignature` row key を `name` に揃え、current carrier を `name / requires / provides / transforms / checks / emits / obligations / laws` に widen した。helper closeout には representative-slice `layer_signatures` / `layer_signature_scope` / `layer_signature_names` / `reserved_layer_signature_names` を追加し、runtime closeout には `obligations` lane と `layer_signature_scope = clean_near_end_canonical_inventory` を追加した。helper representative inventory `verification_handoff_witness` / `runtime_turn_trace` / `membership_*` / `hotplug_*` と runtime canonical inventory `auth_authority_witness` / `transport_provider_boundary` / `verification_model_check` の scope split を front-door docs、`plan/09`、`plan/14`、`specs/10` / `specs/11`、report `0952` に同期し、Sugoroku helper focused run / closeout、helper unittest、runtime focused run / closeout、runtime sample test、`check_source_hierarchy.py`、`validate_docs.py`、`git diff --check` を通した。
- 2026-04-28 10:08 JST — `P4` `TermSignature` registry hardening を close し、Sugoroku helper closeout と clean near-end closeout に `signature_lanes = kind/name/evidence_role`、`signature_scope`、`signature_evidence_roles` を追加した。runtime 側では active `TermSignature` kind family を `effect / transition / witness / relation / property` に tighten し、`history` / `witness-field` / `proof-obligation` は dedicated field 側へ戻した上で provenance を `(kind, name, evidence_role)` で保持するようにした。front-door docs、snapshot docs、`plan/09`、`plan/14`、`specs/10` / `specs/11`、report `0950` に同期し、Sugoroku helper unittest、runtime sample test、Sugoroku / clean near-end closeout JSON、`check_source_hierarchy.py`、`validate_docs.py`、`git diff --check` を通した。
- 2026-04-28 09:57 JST — `P3` Projection / placement residual emitted-program gate を close し、projection validity report minimum、generated artifact reserve policy、`P15` handoff line、`P4` / `P5` queue promotionを `plan/20`、front-door docs、snapshot docs、`samples/generated/README.md`、reports `0947` / `0948` / `0949` に同期した。review follow-up で `P15` の `server/client` wording を `place-specific` に戻し、top-level closeout command set に generated reserve guard を追加した。projection preview / visualization、clean near-end `cross_place_projection`、Sugoroku / clean near-end closeout、helper unittest、runtime test、generated reserve inventory、`check_source_hierarchy.py`、`validate_docs.py`、`git diff --check` を通した。
- 2026-04-28 09:26 JST — `P2` Typed external boundary residual planned family review を close し、`scripts/typed_external_boundary_samples.py` の pretty `list` / `check-all` / `closeout` bug を修正した上で、helper closeout `residual_review_matrix`、`plan/25`、typed-external docs、`progress.md`、`tasks.md`、`samples_progress.md`、report `0946` を同期した。typed external helper/unit test、Sugoroku envelopes / visualization anchor、clean near-end `provider_boundary` anchor、`check_source_hierarchy.py`、`validate_docs.py`、`git diff --check` を通した。
- 2026-04-28 03:27 JST — `sub-agent-pro/mirrorea_next_stage_full_plan_handoff_2026-04-27.md` を repo current line に mirror する task を close し、front-door docs、`AGENTS.md`、`specs/10` / `specs/11`、relevant `plan/`、`progress.md`、`tasks.md`、`samples_progress.md`、report `0945` を再同期した。`check_source_hierarchy.py`、`validate_docs.py`、clean suite smoke/closeout、Sugoroku / avatar / typed external / transport closeout、`cargo test -p mir-{ast,runtime,semantics}`、`git diff --check` を通した。
- 2026-04-27 23:26 JST — `README.md` / `Documentation.md` / `progress.md` / `tasks.md` / `samples_progress.md` を snapshot として圧縮・同期し、current next queue、front-door docs、recent validation の読みを current line に揃えた。`0943` report を追加した上で `check_source_hierarchy.py`、`validate_docs.py`、typed external closeout、focused projection run、runtime sample run、`git diff --check` を通した。
- 2026-04-27 22:52 JST — `Projection / placement executable widening` として Sugoroku helper `projection_view` / `--debug projection` と clean near-end runtime report-local `cross_place_projection` を追加し、next promoted package を Typed external boundary residual planned family review に進めた。
- 2026-04-27 21:55 JST — `Typed external boundary executable widening` として `scripts/typed_external_boundary_samples.py`、synthetic preview subset `EXT-03` / `EXT-04`、residual planned family `EXT-01` / `EXT-02` / `EXT-05` を同期した。
