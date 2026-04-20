# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 package 先までの near-term execution order を示す。
step 数や task 数は厳密な約束ではなく、**rough estimate** である。

execution lane、theory-lab lane、reserve integration lane は分けて書く。

## current reading

- current execution line は `Macro 4 active on fixed authored/prototype floor` である。
- current theory-lab line は `Macro 5 final-layer closeout packages active` である。
- current reserve integration line は `Macro 6 minimal working subset actual default / Macro 7 mixed` である。
- corrected runnable floor は current mapped corpus で already reached しており、next packages はその floor を作るためではなく closeout / mixed-gate narrowing のために置いている。
- current defaults:
  - theorem-first external integration target
  - first strong typing layer = finite decidable index fragment + IFC / taint + capture / lifetime + simple cost
  - repo-local runnable CLI + tests + emitted artifacts + reproducible compare floor = near-end success
  - first application target = authoritative shared-space turn-based room
  - shared-space scope = minimal working subset, not exhaustive final catalog
  - first room default = authority-ack / single room authority / authoritative serial transition / authority_rng / late join visible past / stale reconnect fail-then-refresh / replay invalidation

## recently closed packages

| package band | close evidence | current reading |
|---|---|---|
| `0...10` | `docs/reports/0740...0751` + `specs/examples/466...475` | actual adoption package、syntax/modality convergence、theory spine / Lean-first proof roadmap |
| `11...19` | `docs/reports/0752...0761` + `specs/examples/476...484` | reserve strengthening / practical actualization、model-check second line、mixed-gate threshold defaults |
| `20...35` | `docs/reports/0762...0778` + `specs/examples/485...500` | theorem/model-check/order-handoff/shared-space coupled-later / actual-adoption packages |
| `36...42` | `docs/reports/0779...0785` + `specs/examples/501...507` | checker-artifact / witness-provider / theorem/model-check final-public-contract reopen thresholds |
| `43...50` | `docs/reports/0786...0795` + `specs/examples/508...515` | Lean-stub pilot、artifact-conformance、trace alignment、public-seam compression、serial reserve surface |
| `51...54` | `docs/reports/0796...0799` + `specs/examples/516...519` | toolchain probe / reopen manifest、model-check public-seam compression、representative Lean sample set actual Lean execution |
| `55` | `docs/reports/0801` + `specs/examples/520` | final-layer closeout defaults and reopened self-driven queue |
| `57 first slice` | `docs/reports/0802` + `specs/examples/521` | committed Lean sample corpus、IFC first fragment、proof-skeleton / proof-obligation first foundations |
| `56 concrete slice` | `docs/reports/0803` + `specs/examples/522` | IFC secret valid/invalid concrete example、`samples/lean/` 日本語 explanation sync、Package 56 narrowed queue |
| `56 source pair` | `docs/reports/0804` + `specs/examples/523` | source-side IFC explicit authority pair、representative Lean sample set widening、Package 56 source-side evidence close |
| `56 label-flow close` | `docs/reports/0805` + `specs/examples/524` | source-side IFC label-flow negative、representative Lean sample set widening、Package 56 actual package close |
| `58 first widening slice` | `docs/reports/0806` + `specs/examples/525` | delegated RNG provider placement carry-over、representative Lean sample set widening、Package 58 started |
| `58 helper/CLI surface` | `docs/reports/0807` + `specs/examples/526` | order-handoff helper CLI `surface_preview`、`p07 / p08 / p09` reached/guarded actualization、Package 58 helper hardening first slice |
| `58 negative static stop` | `docs/reports/0808` + `specs/examples/527` | order-handoff late-join visibility negative pair `p13 / p14`、helper-local static stop actualization、Package 58 negative corpus tightening |
| `58 theorem-side negative carry-over` | `docs/reports/0809` + `specs/examples/528` | order-handoff late-join visibility negative pair `p13 / p14` を representative Lean sample set / committed Lean corpus へ carry over、Package 58 broader theorem-side widening |
| `58 IFC checker-hint preview` | `docs/reports/0810` + `specs/examples/529` | source-side IFC trio `p10 / p11 / p12` を `typed_checker_hint_preview` として actualize、`family_refs[] + coverage_state` mirror、Package 58 IFC helper widening |
| `58 theorem/model-check helper preview` | `docs/reports/0811` + `specs/examples/530` | theorem result-object preview と model-check public-checker preview を `run-source-sample` helper summary へ widen し、`p08` / `p09` の reached/guard 非対称を固定、Package 58 diagnostics widening close |
| `59 near-end closeout sync` | `docs/reports/0812` + `specs/examples/531` | Package 58 close 後の queue / mixed gate / user-spec residual を再圧縮し、next self-driven queue を residual mixed-gate packages `60 / 61` へ再構成 |
| `60 theorem/model-check residual mixed-gate compression` | `docs/reports/0813` + `specs/examples/532` | theorem/model-check final public-contract reopen threshold を `run-source-sample` helper summary に mirror し、`p08` / `p09` の非対称を helper-local operational summary に固定、Package 60 close |
| `61 order-handoff/shared-space residual mixed-gate compression` | `docs/reports/0814` + `specs/examples/533` | order-handoff / witness-provider public seam compression を `run-source-sample` helper summary に mirror し、`p07 / p08` reached・`p09` guard の current cut を helper-local operational summary に固定、Package 61 close |
| `62 typed/IFC helper-to-checker ratchet` | `docs/reports/0815` + `specs/examples/534` | `actual_checker_payload_family_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current checker-adjacent payload bridge を `payload_family_kind + source_refs` で固定、Package 62 close |
| `63 checker payload row-family ratchet` | `docs/reports/0816` + `specs/examples/535` | `actual_checker_payload_row_family_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current row grouping bridge を `payload_family_ref + row_family_kind` で固定、Package 63 close |
| `64 checker payload row-detail ratchet` | `docs/reports/0817` + `specs/examples/536` | `actual_checker_payload_row_detail_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current row-detail bridge を `payload_row_family_ref + row_source_ref + row_reason_kind` で固定、Package 64 close |
| `65 checker payload row-body ratchet` | `docs/reports/0818` + `specs/examples/537` | `actual_checker_payload_row_body_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current row-body bridge を `row_body = { selected_option_ref, visibility_target_ref }` で固定、Package 65 close |
| `66 checker payload supported-kind-summary ratchet` | `docs/reports/0819` + `specs/examples/538` | `actual_checker_payload_supported_kind_summary_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current row-family keyed summary bridge を `payload_row_family_ref + supported_kind_scope + supported_kind_refs` で固定、Package 66 close |
| `67 checker payload public-schema sketch ratchet` | `docs/reports/0820` + `specs/examples/539` | `actual_checker_payload_public_schema_sketch_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current checker-side package bundle を 5 ref wrapper で固定、Package 67 close |
| `68 public-checker-api sketch ratchet` | `docs/reports/0821` + `specs/examples/540` | `actual_public_checker_api_sketch_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current checker-side API read relation を `checker_subject + public_checker_payload_schema_ref` で固定、Package 68 close |
| `69 public-checker entry-criteria ratchet` | `docs/reports/0822` + `specs/examples/541` | `actual_public_checker_entry_criteria_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current public-looking reopen minimum を `public_checker_api_ref + entry_criteria_refs + next_comparison_target_ref + deferred_boundary_refs` で固定、Package 69 close |
| `70 public-checker command-surface ratchet` | `docs/reports/0823` + `specs/examples/542` | `actual_public_checker_command_surface_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current command-surface minimum を `command_surface_kind + family_facade_command_refs + public_checker_api_ref` で固定、Package 70 close |
| `71 shared-output-contract ratchet` | `docs/reports/0824` + `specs/examples/543` | `actual_shared_output_contract_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current shared-output-contract minimum を `output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref` で固定、Package 71 close |
| `72 public-checker-boundary ratchet` | `docs/reports/0825` + `specs/examples/544` | `actual_public_checker_boundary_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current public-checker-boundary minimum を `boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref` で固定、Package 72 close |
| `73 verifier-handoff-surface ratchet` | `docs/reports/0826` + `specs/examples/545` | `actual_verifier_handoff_surface_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current verifier-handoff-surface minimum を `handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode` で固定、Package 73 close |
| `74 minimal-parser-subset-freeze ratchet` | `docs/reports/0827` + `specs/examples/546` | `actual_minimal_parser_subset_freeze_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current parser first-tranche minimum を `freeze_kind + accepted_cluster_refs + reject_cluster_refs + retention_floor_refs` で固定、Package 74 close |
| `75 parser-to-checker-reconnect-freeze ratchet` | `docs/reports/0828` + `specs/examples/547` | `actual_parser_to_checker_reconnect_freeze_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current reconnect minimum を `reconnect_kind + parser_subset_freeze_ref + checker_floor_refs + retained_helper_refs` で固定、Package 75 close |
| `76 phase1-semantics-closeout ratchet` | `docs/reports/0829` + `specs/examples/548` | `actual_phase1_semantics_closeout_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current semantics closeout minimum を `closeout_kind + core_semantics_refs + invariant_bridge_refs + notation_status_refs` で固定、Package 76 close |
| `77 phase2-parser-free-poc-closeout ratchet` | `docs/reports/0830` + `specs/examples/549` | `actual_phase2_parser_free_poc_closeout_threshold` を `run-source-sample` helper summary に mirror し、`p10 / p11 / p12` の current parser-free closeout minimum を `closeout_kind + compile_gate_refs + helper_boundary_refs + detached_loop_policy_refs` で固定、Package 77 close |
| `78 phase4-shared-space-self-driven-closeout ratchet` | `docs/reports/0831` + `specs/examples/550` | `actual_phase4_shared_space_self_driven_closeout_threshold` を `run-source-sample` helper summary に mirror し、`p07 / p08 / p09` の current shared-space closeout minimum を `closeout_kind + current_package_refs + user_spec_required_catalog_refs + retained_later_refs` で固定、Package 78 close |
| `79 phase5-proof-protocol-runtime-policy-handoff-closeout ratchet` | `docs/reports/0832` + `specs/examples/551` | `actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold` を `run-source-sample` helper summary に mirror し、`p07 / p08 / p09` の current proof/protocol/runtime-policy handoff closeout minimum を `closeout_kind + verifier_handoff_surface_ref + theorem_retained_bridge_stop_ref + boundary_inventory_ref + retained_later_refs` で固定、Package 79 close |
| `80 phase6-actual-parser-ast-carrier-first-tranche ratchet` | `docs/reports/0833` + `specs/examples/552` | `actual_phase6_actual_parser_ast_carrier_first_tranche_threshold` を `run-source-sample` helper summary に mirror し、`p07 / p08 / p09` の current parser first tranche minimum を `carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs` で固定、Package 80 close |
| `92 first strong typing finite-index layer` | `docs/reports/0849` + `specs/examples/566` | `p15` capture/lifetime negative、`p16` simple cost negative、source-side first strong typing sample set `p10 / p11 / p12 / p15 / p16` を helper-local checker summary / verifier preview alignment / model-check projection pre-floor に widen し、Package 92 close |
| `93 Lean-first formal skeleton hardening` | `docs/reports/0850` + `specs/examples/567` | `CurrentL2FiniteIndexFirstLayer.lean` と representative generated theorem stub corpus `p15 / p16` widening を actualize し、foundation / generated stub の役割差を日本語 explanation と manifest に同期して Package 93 close |
| `94 theorem/model-check bridge carrier reconnect` | `docs/reports/0851` + `specs/examples/568` | theorem public seam quartet keep、model-check second-line / row-local carrier widening、helper-local `bridge_floor_refs` reconnect を actualize し、Package 94 close |
| `95 order-handoff source surface / artifact tightening` | `docs/reports/0852` + `specs/examples/569` | edge-row principal / stage-block secondary / repo-local emitted artifact refs first / `p09` reserve / `p13 / p14` negative pair を `run-source-sample` helper summary に actualize し、Package 95 close |
| `96 authoritative-room first scenario tightening` | `docs/reports/0853` + `specs/examples/570` | authoritative-room first default profile の representative reached pair `p07 / p08`、delegated RNG reserve `p09`、late-join negative static-stop pair `p13 / p14` を `run-source-sample` helper summary に actualize し、Package 96 close |
| `97 authoritative-room reserve strengthening lane tightening` | `docs/reports/0854` + `specs/examples/571` | `auditable_authority_witness`、`delegated_rng_service`、model-check second line を `run-source-sample` helper summary の reserve lane に separate status のまま actualize し、Package 97 close |
| `98 guided problem sample entrypoints and runner` | `docs/reports/0855` + `specs/examples/572` | Problem 1 / Problem 2 の representative sample entrypoint を `samples/` README と `scripts/current_l2_guided_samples.py` に actualize し、Package 98 close |
| `99/100 residual bundle matrices` | `docs/reports/0856` + `specs/examples/573...574` | Problem 1 / Problem 2 の residual mixed gate を `matrix problem1 / problem2` helper と sample README に actualize し、current live queue を theorem-first pilot bundle / authoritative-room scenario bundle へ進めた |
| `101/102 problem bundle actualization` | `docs/reports/0857` + `specs/examples/575...576` | Problem 1 / Problem 2 を `bundle problem1 / problem2` helper、Lean artifact、anchor spec / report まで一本道で辿れる bundle として actualize し、current live queue を parser-side companion surface bundle / parser-side bundle-to-helper bridge へ進めた |
| `103/104 parser-side companion and bridge` | `docs/reports/0858` + `specs/examples/577...578` | `p06 / p07 / p08` representative slice を parser-side companion sample と `Stage3RequestHeadClauseBundle` parse test へ actualize し、`bundle problem1 / problem2` helper に parser companion path を追加して docs / helper / parser-side carrier の 3 点を一本道にした。current live queue は Package 105 parser companion inspector / Package 106 parser companion mapping matrix へ進んだ |
| `105 parser companion inspector` | `docs/reports/0859` + `specs/examples/579` | `Stage3RequestHeadClauseBundle` の representative parse result を repo-local JSON / pretty inspector command と focused test に actualize し、current live queue を Package 106 parser companion mapping matrix へ進めた |
| `106 parser companion mapping matrix` | `docs/reports/0860` + `specs/examples/580` | `p06 / p07 / p08` representative slice の original prototype / parser companion / guided bundle / Lean artifact / anchor spec-report 対応を `mapping` helper と sample README table に actualize し、current live queue を Package 107 explained representative problem sample bundles へ進めた |
| `107 explained representative problem sample bundles` | `docs/reports/0861` + `specs/examples/581` | `samples/problem-bundles/` と `bundle problem1 / problem2` の `sample_bundle_doc` を actualize し、Problem 1 / Problem 2 の representative sample guide を `samples/` 側の簡潔な日本語 bundle として固定した。current live queue は Package 108 representative problem bundle smoke runner へ進んだ |
| `108 representative problem bundle smoke runner` | `docs/reports/0862` + `specs/examples/582` | `smoke problem1 / smoke problem2` helper を actualize し、representative sample bundle guide に書いた主要 command 群を repo-local helper から順に再現できるようにした。current live queue は Package 109 representative problem bundle aggregate smoke summary へ進んだ |
| `109 representative problem bundle aggregate smoke summary` | `docs/reports/0863` + `specs/examples/583` | `smoke-all` helper を actualize し、Problem 1 / Problem 2 の representative smoke 成否と step inventory を 1 コマンドで compact に俯瞰できるようにした。current live queue は Package 110 representative problem bundle failure-focused smoke diagnostics へ進んだ |
| `110 representative problem bundle failure-focused smoke diagnostics` | `docs/reports/0864` + `specs/examples/584` | `smoke-all` helper に failed step / command / return code / output excerpt の compact diagnostics と non-zero exit を actualize し、aggregate summary が failure 時にも regression entrypoint として使えるようにした。current live queue は Package 111 representative problem bundle quickstart walkthrough hardening へ進んだ |
| `111 representative problem bundle quickstart walkthrough hardening` | `docs/reports/0865` + `specs/examples/585` | `samples/problem-bundles/problem1|problem2` に `最短 quickstart` と `見るべき結果` を actualize し、representative sample guide を doc 単体でも読める quickstart bundle に harden した。current live queue は Package 112 representative problem quickstart CLI mirror へ進んだ |
| `112 representative problem quickstart CLI mirror` | `docs/reports/0866` + `specs/examples/586` | `quickstart problem1|problem2` helper を actualize し、bundle doc 側の representative 4-step quickstart を helper-side summary にも mirror した。current live queue は Package 113 representative problem quickstart parity checks へ進んだ |
| `113 representative problem quickstart parity checks` | `docs/reports/0867` + `specs/examples/587` | `quickstart-parity` helper を actualize し、sample bundle doc と helper-side quickstart mirror の 4-step parity を focused に確認できるようにした。current live queue は Package 114 representative problem mixed-gate reopen map refresh へ進んだ |
| `114 representative problem mixed-gate reopen map refresh` | `docs/reports/0868` + `specs/examples/588` | `reopen-map` helper と sample bundle doc 側の `現在の mixed gate 再開点` section を actualize し、Problem 1 / Problem 2 の remaining mixed gate と global true user-spec residual を entry command 付きで読み直せるようにした。current live queue は Package 115 / 116 split refresh へ進んだ |
| `115/116 representative problem split-package map refresh` | `docs/reports/0869` + `specs/examples/589` | `reopen-map problem1|problem2` と sample bundle doc 側の `次の split package` section を actualize し、Problem 1 / Problem 2 の remaining mixed gate を next separate package 名まで narrow に戻した。current live queue は Package 117...121 へ進んだ |
| `117 Problem 1 typed source principal split helper` | `docs/reports/0870` + `specs/examples/590` | `split problem1 typed-source-principal` helper と sample bundle doc 側の typed split entrypoint を actualize し、Problem 1 typed residual を representative/supporting sample set・kept-separate residual・stop line 付きで独立 package として読めるようにした。current live queue は Package 118...121 へ進んだ |
| `118 Problem 1 theorem public-contract split helper` | `docs/reports/0871` + `specs/examples/591` | `split problem1 theorem-public-contract` helper と sample bundle doc 側の theorem split entrypoint を actualize し、Problem 1 theorem public-contract residual を representative `p06`・theorem-first pilot / Lean artifact 導線・kept-separate residual・stop line 付きで独立 package として読めるようにした。current live queue は Package 119...121 へ進んだ |
| `119 Problem 1 model-check public-contract split helper` | `docs/reports/0872` + `specs/examples/592` | `split problem1 model-check-public-contract` helper と sample bundle doc 側の model-check split entrypoint を actualize し、Problem 1 model-check public-contract residual を representative/supporting sample set・kept-separate residual・stop line 付きで独立 package として読めるようにした。current live queue は Package 120...121 へ進んだ |
| `120 Problem 2 source wording / emitted schema split helper` | `docs/reports/0873` + `specs/examples/593` | `split problem2 source-wording-emitted-schema` helper と sample bundle doc 側の source wording split entrypoint を actualize し、Problem 2 source wording / emitted schema residual を representative/supporting sample set・kept-separate residual・stop line 付きで独立 package として読めるようにした。current live queue は Package 121 へ進んだ |
| `121 Problem 2 witness-provider public-shape split helper` | `docs/reports/0874` + `specs/examples/594` | `split problem2 witness-provider-public-shape` helper と sample bundle doc 側の witness-provider split entrypoint を actualize し、Problem 2 witness/provider public-shape residual を representative / reserve / negative supporting set・kept-separate residual・stop line 付きで独立 package として読めるようにした。current live queue は Package 122 / 123 へ進んだ |
| `122 representative problem reopen-map split closeout sync` | `docs/reports/0875` + `specs/examples/595` | `reopen-map problem1|problem2` helper public surface から stale な `next split packages` 表示を外し、`closed_split_packages` / `split package closeout` を actualize して residual public-seam maintenance 読みへ同期した。current live queue は Package 123 へ進んだ |
| `123 remaining residual lane summary` | `docs/reports/0876` + `specs/examples/596` | `residuals` helper と sample bundle doc 側の residual lane entrypoint を actualize し、remaining mixed gate と true user-spec residual を `problem1-final-public-seams` / `problem2-final-public-seams` / `syntax-modality-final-marker` lane へ圧縮した。current live queue は Package 124...126 へ進んだ |
| `124 Problem 1 final-public-seam lane helper` | `docs/reports/0877` + `specs/examples/597` | `lane problem1-final-public-seams` helper と Problem 1 bundle doc 側の lane entrypoint を actualize し、Problem 1 mixed gate を typed / theorem / model-check reopen order と stop line 付きの独立 lane として読めるようにした。current live queue は Package 125 / 126 へ進んだ |
| `125 Problem 2 final-public-seam lane helper` | `docs/reports/0878` + `specs/examples/598` | `lane problem2-final-public-seams` helper と Problem 2 bundle doc 側の lane entrypoint を actualize し、Problem 2 mixed gate を source wording / witness-provider reopen order と stop line 付きの独立 lane として読めるようにした。current live queue は Package 126 へ進んだ |
| `126 syntax-modality final-marker lane helper` | `docs/reports/0879` + `specs/examples/599` | `lane syntax-modality-final-marker` helper と sample bundle index / syntax-modality docs を actualize し、syntax / modality mixed gate を current recommendation / retained families / separation boundary 付きの独立 lane として読めるようにした。current live queue は Package 127...129 へ進んだ |
| `127 typed-checker first executable slice` | `docs/reports/0880` + `specs/examples/600` | `check-source-sample` focused checker command と Problem 1 bundle doc 側の quickstart 導線を actualize し、first strong typing sample set を checker-adjacent executable slice として読めるようにした。current live queue は Package 128 / 129 へ進んだ |
| `128 theorem-first emitted-artifact loop hardening` | `docs/reports/0881` + `specs/examples/601` | `emit-theorem problem1` helper と Problem 1 bundle doc 側の emitted-artifact step を actualize し、representative theorem line `p06 / p07 / p08` を repo-local output dir に materialize できる emitted-artifact loop として読めるようにした。current live queue は Package 129 へ進んだ |
| `129 authoritative-room runnable scenario hardening` | `docs/reports/0882` + `specs/examples/602` | `emit-scenario problem2` helper と Problem 2 bundle doc 側の runnable-scenario step を actualize し、representative pair `p07 / p08`、reserve route `p09`、negative pair `p13 / p14` を repo-local output dir に materialize できる scenario loop として読めるようにした。current live queue は Package 130...132 へ進んだ |
| `130 Problem 1 executable residual reopen sync` | `docs/reports/0883` + `specs/examples/603` | `reopen-map problem1`、`lane problem1-final-public-seams`、Problem 1 sample bundle doc を executable evidence 側へ再同期し、`check-source-sample` → `emit-theorem problem1` → lane summary の reopen order を actualize した。current live queue は Package 131...132 へ進んだ |
| `131 Problem 2 executable residual reopen sync` | `docs/reports/0884` + `specs/examples/604` | `reopen-map problem2`、`lane problem2-final-public-seams`、`residuals`、Problem 2 sample bundle doc を executable scenario loop 側へ再同期し、`emit-scenario problem2` → lane summary → residual summary の reopen order を actualize した。current live queue は Package 132 へ進んだ |
| `132 once-through closeout summary sync` | `docs/reports/0885` + `specs/examples/605` | `closeout` helper を actualize し、Package 127...131 executable loop 後の current first line / mixed-gate lane / true user-spec residual / next self-driven queue を 1 枚の helper-local summary に再圧縮した。current live queue は Package 133...135 へ進んだ |
| `133 reserve integration entrypoint sync` | `docs/reports/0886` + `specs/examples/606` | `reserve` helper を actualize し、theorem-first external pilot / `auditable_authority_witness` / `delegated_rng_service` / model-check second-line reserve を once-through closeout summary 後の next reopen line として helper / docs / snapshot に同期した。current live queue は Package 134 / 135 と later mixed/user-spec residual へ進んだ |
| `134 parser-side residual closeout sync` | `docs/reports/0887` + `specs/examples/607` | `lane parser-side-residual` を actualize し、parser companion surface / parser-side tranche / final parser-checker-runtime API residual を `residuals` / `closeout` / `reserve` と切り分けた mixed gate lane として helper / docs / snapshot に同期した。current live queue は Package 135 と later mixed/user-spec residual へ進んだ |
| `135 true user-spec residual freeze sync` | `docs/reports/0888` + `specs/examples/608` | `hold-line` helper を actualize し、true user-spec residual を explicit hold line として `closeout` / `reserve` / snapshot docs から切り離した。closeout 用 numbered queue は closed 状態になり、next reopen line は reserve integration / later mixed gate / user-spec hold line に移った |
| `136 theorem-first external pilot summary index actualization` | `docs/reports/0889` + `specs/examples/609` | `emit-theorem problem1` output dir に `pilot-summary.md` / `pilot-summary.json` を actualize し、theorem-first external pilot を notebook-first summary index まで進めた。next reopen line は `auditable_authority_witness` / `delegated_rng_service` / model-check second-line と later mixed gate へ移った |
| `137 auditable-authority-witness reserve package summary index actualization` | `docs/reports/0890` + `specs/examples/610` | `emit-reserve auditable-authority-witness` と `target/current-l2-guided/reserve-packages/auditable-authority-witness/package-summary.md|json` を actualize し、`p07` reached / `p08` non-witness-bearing contrast / `p05` pre-default comparison を reserve package 単独の repo-local summary index に圧縮した。next reopen line は `delegated_rng_service` / model-check second-line と later mixed gate へ移った |
| `138 delegated-rng-service reserve package summary index actualization` | `docs/reports/0891` + `specs/examples/611` | `emit-reserve delegated-rng-service` と `target/current-l2-guided/reserve-packages/delegated-rng-service/package-summary.md|json` を actualize し、`p09` reached / `p07` authority-rng baseline contrast / `p08` reconnect contrast を reserve package 単独の repo-local summary index に圧縮した。next reopen line は model-check second-line と later mixed gate へ移った |

## active self-driven packages

| 順番 | package | macro | question | rough weight | current exit signal |
|---|---|---|---|---|---|
| 1 | `model-check-second-line` | `Macro 5/6` | row-local property carrier second-line を public checker finalization と切り離したまま narrow actualization する | `M` | property carrier / tool seam / public seam residual が problem1 lane と干渉せず追える |

## package detail

### Package 56 — layered strong typing / IFC first-fragment

- current source:
  `specs/examples/475`
  `specs/examples/520`
  `specs/examples/521`
  `specs/examples/522`
  `specs/examples/523`
  `specs/examples/524`
  `specs/examples/557`
- current recommendation:
  close 済み。checker-adjacent principal + layered stack を維持し、full dependent core を first public core に入れず、finite decidable index fragment と `Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ C` conceptual spine、IFC / taint、capture / lifetime、simple cost を first strong typing layer の principal target に置く。stronger typed surface early principal promotion はしない。`CurrentL2LabelModel.lean` と `CurrentL2IfcSecretExamples.lean` により Lean-side first fragment と secret valid/invalid concrete example は actualize 済みであり、`p10 / p11 / p12` により source-side authority success / authority-miss negative / label-flow negative trio も corrected prototype として actualize 済みと読む。
- target corpus:
  `p06-typed-proof-owner-handoff`
  `p10-typed-authorized-fingerprint-declassification`
  `p11-typed-unauthorized-fingerprint-release`
  `p12-typed-classified-fingerprint-publication-block`
- stop line:
  final typed source principal
  final IFC syntax
  final public label API

### Package 57 — Lean formal skeleton / proof obligations

- current reading:
  first slice は close 済みである。
- close evidence:
  `specs/examples/521`
  `samples/lean/foundations/CurrentL2ProofSkeleton.lean`
  `samples/lean/current-l2/`
  `crates/mir-runtime/examples/current_l2_emit_theorem_lean_bundle.rs`
  `scripts/current_l2_lean_sample_sync.py`
- kept later:
  concrete production prover binding
  final proof object public contract
  final public verifier contract

### Package 58 — helper / CLI hardening and broader coverage

- current source:
  `specs/examples/516...530`
- current recommendation:
  close 済み。actual Lean execution floor と committed `samples/lean/` corpus を helper / CLI hardening、broader theorem-side / diagnostics widening に使い、`p09` carry-over、`surface_preview`、`p13 / p14` static stop と theorem-side carry-over、`typed_checker_hint_preview`、theorem result-object preview helper mirror、model-check public-checker preview helper mirror まで actualize 済みと読む。
- promotion rule:
  widened corpus は mixed gate を 1 つ閉じるか、formal skeleton package を concretize する場合だけ足す。
- stop line:
  final public theorem/model-check contract
  packaging / host-facing integration

### Package 59 — near-end closeout sync

- current source:
  `specs/examples/469`
  `specs/examples/520`
  `specs/examples/530`
  `specs/examples/531`
- current recommendation:
  close 済み。Package 58 close 後の queue / mixed gate / true user-spec residual を stale wording なしで同期し、next self-driven queue を residual mixed-gate packages へ再構成した。
- required sync targets:
  `progress.md`
  `tasks.md`
  `plan/01`
  `plan/17`
  `plan/18`
  `plan/90`

### Package 60 — theorem/model-check residual mixed-gate compression

- current source:
  `specs/examples/506`
  `specs/examples/507`
  `specs/examples/530`
  `specs/examples/532`
- current recommendation:
  close 済み。theorem/model-check final public-contract reopen threshold を `run-source-sample` helper summary に narrow に mirror し、`p08` theorem reached / model-check guarded と `p09` theorem guarded / model-check reached の非対称を helper-local operational summary に actualize した。
- stop line:
  final public theorem/model-check contract adoption
  final public verifier contract adoption
  concrete production prover / checker binding

### Package 61 — order-handoff/shared-space residual mixed-gate compression

- current source:
  `specs/examples/505`
  `specs/examples/515`
  `specs/examples/526`
  `specs/examples/533`
- current recommendation:
  close 済み。order-handoff source wording residual / emitted-artifact residual / witness-provider public-seam residual を `run-source-sample` helper summary に narrow に mirror し、`p07 / p08` reached・`p09` guard の current cut を helper-local operational summary に actualize した。
- stop line:
  final source wording adoption
  final public witness/provider/artifact contract adoption
  final parser/public API adoption

### Package 62 — typed/IFC helper-to-checker ratchet

- current source:
  `specs/examples/522`
  `specs/examples/523`
  `specs/examples/524`
  `specs/examples/529`
  `specs/examples/534`
- current recommendation:
  close 済み。`typed_checker_hint_preview` を final typed source principal や final public verifier contract に上げず、checker-adjacent payload threshold まで narrow に ratchet し、`actual_checker_payload_family_threshold` を `payload_family_kind + source_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  final typed source principal
  final IFC syntax
  final public checker artifact
  final public verifier contract

### Package 63 — checker payload row-family ratchet

- current source:
  `specs/examples/265`
  `specs/examples/266`
  `specs/examples/534`
  `specs/examples/535`
- current recommendation:
  close 済み。`actual_checker_payload_family_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload row family まで narrow に ratchet し、`actual_checker_payload_row_family_threshold` を `payload_family_ref + row_family_kind` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  supported kind detail
  actual checker row payload
  final public checker artifact
  final public verifier contract

### Package 64 — checker payload row-detail ratchet

- current source:
  `specs/examples/267`
  `specs/examples/268`
  `specs/examples/535`
  `specs/examples/536`
- current recommendation:
  close 済み。`actual_checker_payload_row_family_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload row detail まで narrow に ratchet し、`actual_checker_payload_row_detail_threshold` を `payload_row_family_ref + row_source_ref + row_reason_kind` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  actual checker row body
  supported kind detail
  final public checker artifact
  final public verifier contract

### Package 65 — checker payload row-body ratchet

- current source:
  `specs/examples/269`
  `specs/examples/270`
  `specs/examples/536`
  `specs/examples/537`
- current recommendation:
  close 済み。`actual_checker_payload_row_detail_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload row body まで narrow に ratchet し、`actual_checker_payload_row_body_threshold` を `payload_row_family_ref + row_source_ref + row_reason_kind + row_body` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  supported kind detail
  final public checker artifact
  final public verifier contract

### Package 66 — checker payload supported-kind-summary ratchet

- current source:
  `specs/examples/271`
  `specs/examples/272`
  `specs/examples/537`
  `specs/examples/538`
- current recommendation:
  close 済み。`actual_checker_payload_row_body_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload supported kind summary まで narrow に ratchet し、`actual_checker_payload_supported_kind_summary_threshold` を `payload_row_family_ref + supported_kind_scope + supported_kind_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  public checker payload schema
  final public checker artifact
  final public verifier contract

### Package 67 — checker payload public-schema sketch ratchet

- current source:
  `specs/examples/273`
  `specs/examples/274`
  `specs/examples/538`
  `specs/examples/539`
- current recommendation:
  close 済み。`actual_checker_payload_supported_kind_summary_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload public schema sketch まで narrow に ratchet し、`actual_checker_payload_public_schema_sketch_threshold` を 5 ref wrapper current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  public checker API
  final public checker artifact
  final public verifier contract

### Package 68 — checker payload public-checker-api sketch ratchet

- current source:
  `specs/examples/275`
  `specs/examples/276`
  `specs/examples/539`
  `specs/examples/540`
- current recommendation:
  close 済み。`actual_checker_payload_public_schema_sketch_threshold` を actual command surface や final public verifier contract に上げず、public checker API sketch まで narrow に ratchet し、`actual_public_checker_api_sketch_threshold` を `checker_subject + public_checker_payload_schema_ref` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  public checker entry criteria
  actual command surface
  shared output contract
  final public verifier contract

### Package 69 — public-checker entry-criteria ratchet

- current source:
  `specs/examples/277`
  `specs/examples/278`
  `specs/examples/540`
  `specs/examples/541`
- current recommendation:
  close 済み。`actual_public_checker_api_sketch_threshold` を actual command surface や parser-front public checker boundary に上げず、public-checker comparison 専用の entry criteria まで narrow に ratchet し、`actual_public_checker_entry_criteria_threshold` を `public_checker_api_ref + entry_criteria_refs + family_facade_support_ref + family_facade_script_refs + smoke_command_refs + next_comparison_target_ref + deferred_boundary_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  actual command surface
  shared output contract
  parser-front public checker boundary
  emitted verifier handoff surface
  final public verifier contract

### Package 70 — public-checker command-surface ratchet

- current source:
  `specs/examples/279`
  `specs/examples/280`
  `specs/examples/541`
  `specs/examples/542`
- current recommendation:
  close 済み。`actual_public_checker_entry_criteria_threshold` を shared output contract や parser-front public checker boundary に上げず、public-checker command surface ready sketch まで narrow に ratchet し、`actual_public_checker_command_surface_threshold` を `command_surface_kind + family_facade_command_refs + public_checker_api_ref` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  detached-loop `smoke-*` wrapper の public surface 昇格
  generic shared public checker entry
  shared output contract
  parser-front public checker boundary
  emitted verifier handoff surface
  final public verifier contract

### Package 71 — shared-output-contract ratchet

- current source:
  `specs/examples/281`
  `specs/examples/282`
  `specs/examples/542`
  `specs/examples/543`
- current recommendation:
  close 済み。`actual_public_checker_command_surface_threshold` を parser-front public checker boundary や emitted verifier handoff surface に上げず、shared-output-contract ready sketch まで narrow に ratchet し、`actual_shared_output_contract_threshold` を `output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  generic shared public checker entry
  parser-front public checker boundary
  emitted verifier handoff surface
  final public verifier contract

### Package 72 — public-checker-boundary ratchet

- current source:
  `specs/examples/283`
  `specs/examples/284`
  `specs/examples/543`
  `specs/examples/544`
- current recommendation:
  close 済み。`actual_shared_output_contract_threshold` を final parser grammar や emitted verifier handoff surface に上げず、public checker boundary ready sketch まで narrow に ratchet し、`actual_public_checker_boundary_threshold` を `boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  final parser grammar
  generic shared public checker entry
  emitted verifier handoff surface
  final public verifier contract

### Package 73 — verifier-handoff-surface ratchet

- current source:
  `specs/examples/285`
  `specs/examples/286`
  `specs/examples/544`
  `specs/examples/545`
- current recommendation:
  close 済み。`actual_public_checker_boundary_threshold` を actual emitted verifier handoff artifact や theorem / protocol / runtime-policy dedicated split に上げず、verifier handoff surface ready sketch まで narrow に ratchet し、`actual_verifier_handoff_surface_threshold` を `handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  actual emitted verifier handoff artifact
  theorem / protocol / runtime-policy dedicated contract
  final parser grammar
  final public verifier contract

### Package 74 — minimal-parser-subset-freeze ratchet

- current source:
  `specs/examples/287`
  `specs/examples/288`
  `specs/examples/545`
  `specs/examples/546`
- current recommendation:
  close 済み。`actual_verifier_handoff_surface_threshold` を final parser grammar や parser-to-checker reconnect freeze に上げず、minimal parser subset freeze ready sketch まで narrow に ratchet し、`actual_minimal_parser_subset_freeze_threshold` を `freeze_kind + accepted_cluster_refs + reject_cluster_refs + retention_floor_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  final parser grammar
  parser-to-checker reconnect freeze
  final public parser/checker API
  final public verifier contract

### Package 75 — parser-to-checker-reconnect-freeze ratchet

- current source:
  `specs/examples/289`
  `specs/examples/290`
  `specs/examples/546`
  `specs/examples/547`
- current recommendation:
  close 済み。`actual_minimal_parser_subset_freeze_threshold` を final parser grammar や final public parser/checker API に上げず、parser-to-checker reconnect freeze ready sketch まで narrow に ratchet し、`actual_parser_to_checker_reconnect_freeze_threshold` を `reconnect_kind + parser_subset_freeze_ref + checker_floor_refs + retained_helper_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  final parser grammar
  final public parser/checker API
  final public verifier contract

### Package 76 — phase1-semantics-closeout ratchet

- current source:
  `specs/examples/291`
  `specs/examples/292`
  `specs/examples/547`
- current recommendation:
  close 済み。`actual_parser_to_checker_reconnect_freeze_threshold` を final parser grammar や final type system に上げず、phase1 semantics closeout ready sketch まで narrow に ratchet し、`actual_phase1_semantics_closeout_threshold` を `closeout_kind + core_semantics_refs + invariant_bridge_refs + notation_status_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  final parser grammar
  final type system
  actual external schema
  final public verifier contract

### Package 77 — phase2-parser-free-poc-closeout ratchet

- current source:
  `specs/examples/293`
  `specs/examples/294`
  `specs/examples/548`
  `specs/examples/549`
- current recommendation:
  close 済み。`actual_phase1_semantics_closeout_threshold` を reference update / bless workflow や public exporter API に上げず、phase2 parser-free PoC closeout ready sketch まで narrow に ratchet し、`actual_phase2_parser_free_poc_closeout_threshold` を `closeout_kind + compile_gate_refs + helper_boundary_refs + detached_loop_policy_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  reference update / bless workflow
  final retention/path policy
  public exporter API
  production host interface

### Package 78 — phase4-shared-space-self-driven-closeout ratchet

- current source:
  `specs/examples/295`
  `specs/examples/296`
  `specs/examples/549`
  `specs/examples/550`
- current recommendation:
  close 済み。`actual_phase2_parser_free_poc_closeout_threshold` を final activation / authority / auth / identity / admission / consistency / fairness catalog に上げず、phase4 shared-space self-driven closeout ready sketch まで narrow に ratchet し、`actual_phase4_shared_space_self_driven_closeout_threshold` を `closeout_kind + current_package_refs + user_spec_required_catalog_refs + retained_later_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  exhaustive shared-space final catalog
  stronger fairness / replay profile
  final public witness/provider/artifact contract
  packaging / host-facing realization

### Package 79 — phase5-proof-protocol-runtime-policy-handoff-closeout ratchet

- current source:
  `specs/examples/297`
  `specs/examples/298`
  `specs/examples/550`
  `specs/examples/551`
- current recommendation:
  close 済み。`actual_phase4_shared_space_self_driven_closeout_threshold` を actual subject row materialization や boundary-specific handoff artifact family に上げず、phase5 proof / protocol / runtime-policy handoff closeout ready sketch まで narrow に ratchet し、`actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold` を `closeout_kind + verifier_handoff_surface_ref + theorem_retained_bridge_stop_ref + boundary_inventory_ref + retained_later_refs` current cut で helper-local operational summary に actualize 済みと読む。
- stop line:
  actual subject row materialization
  boundary-specific handoff artifact family
  actual emitted verifier artifact
  concrete theorem/model-check tool binding
  actual public checker migration
  final public verifier contract

### Package 80 — phase6-actual-parser-ast-carrier-first-tranche ratchet

- current source:
  `specs/examples/299`
  `specs/examples/300`
  `specs/examples/552`
- current recommendation:
  close 済み。`actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold` を stage3 admit/request/predicate cluster や final parser grammar に上げず、`carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs` current cut の phase6 actual parser / AST carrier first tranche ready sketch を `mir-ast` manifest と `run-source-sample` helper summary に actualize 済みと読む。
- stop line:
  stage3 admit slot surface
  stage3 request clause suite
  stage3 predicate fragment
  perform head final public parser API
  span-rich diagnostics
  final grammar

### Package 81 — phase6-actual-checker-runtime-skeleton-first-tranche ratchet

- current source:
  `specs/examples/301`
  `specs/examples/302`
  `specs/examples/553`
- current recommendation:
  close 済み。`actual_phase6_actual_parser_ast_carrier_first_tranche_threshold` の次段として、`skeleton_kind + semantic_entry_refs + runtime_bridge_refs + parser_bridge_contract_refs + retained_later_refs` current cut の phase6 actual checker/runtime first tranche ready sketch を helper-local summary と code anchor に actualize 済みと読む。
- stop line:
  parser_to_program_lowering
  stage3_request_predicate_reconnect
  richer_host_interface
  final_public_runtime_checker_api
  formal_hook_concrete_tool_binding

### Package 82 — phase6-compile-ready-verification-and-formal-hook ratchet

- current source:
  `specs/examples/303`
  `specs/examples/304`
  `specs/examples/554`
- current recommendation:
  close 済み。selected cargo / smoke gate と tool-neutral formal hook shape を current cut に留めた phase6 compile-ready verification / formal-hook ready sketch を helper-local summary と code anchor に actualize 済みと読む。
- stop line:
  concrete_theorem_tool_binding
  concrete_model_check_tool_binding
  parser_second_tranche_widen
  final_public_surface
  final_public_verifier_contract

### Package 83 — phase6-next-reopen-sequencing ratchet

- current source:
  `specs/examples/305`
  `specs/examples/306`
  `specs/examples/555`
- current recommendation:
  close 済み。tool-neutral formal hook を entry criteria に維持したまま、parser second tranche attached-slot / predicate fragment first、theorem-first concrete tool binding reserve、model-check second reserve の sequencing minimum を helper-local summary と snapshot docs に actualize 済みと読む。
- stop line:
  request_clause_suite_bulk_widen
  perform_head_final_public_api
  concrete_theorem_tool_binding
  concrete_model_check_tool_binding
  final_public_surface

### Package 84 — phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package ratchet

- current source:
  `specs/examples/307`
  `specs/examples/308`
  `specs/examples/556`
- current recommendation:
  close 済み。stage3 declaration-side admit attached slot と shared isolated predicate fragment の first package minimum を `CurrentL2SecondTrancheManifest` と stage3 spike regression 群へ actualize 済みと読む。
- stop line:
  shared_single_attachment_frame
  request_clause_suite_publicization
  perform_head_final_public_api
  span_rich_diagnostics
  final_grammar
  theorem_model_check_concrete_binding

### Package 85 — phase6-reserve-formal-tool-binding-inventory ratchet

- current source:
  `specs/examples/309`
  `specs/examples/310`
  `specs/examples/556`
  `specs/examples/558`
- current recommendation:
  close 済み。theorem-first reserve / model-check second reserve / tool-neutral formal-hook keep / parser-side mainline keep を helper-local manifest と snapshot docs に narrow に actualize 済みと読む。
- stop line:
  concrete_theorem_tool_name
  concrete_model_check_tool_name
  actual_ci_artifact_retention_policy
  parser_side_followup_package_selection
  final_public_surface

### Package 86 — phase6-parser-side-follow-up-package-sequencing ratchet

- current source:
  `specs/examples/311`
  `specs/examples/312`
  `specs/examples/559`
- current recommendation:
  close 済み。shared single attachment frame を next parser-side package に固定し、request clause suite / perform head / source-sample path を deferred reopen として helper-local manifest と snapshot docs に narrow に actualize 済みと読む。
- stop line:
  shared single attachment frame actual code shape
  request clause suite publicization
  perform head final public parser API
  source-sample corpus scope / file layout
  final public parser / checker / runtime surface

### Package 87 — phase6-parser-second-tranche-shared-single-attachment-frame-first-package ratchet

- current source:
  `specs/examples/313`
  `specs/examples/314`
  `specs/examples/560`
- current recommendation:
  close 済み。shared single attachment frame の multiline extraction bridge minimum を `CurrentL2SharedSingleAttachmentFrameManifest` と stage3 multiline attachment spike 群へ actualize し、request clause suite / perform head / source-sample path を retained-later に残す。
- stop line:
  request clause suite publicization
  perform head final public parser API
  source-sample corpus scope / file layout
  final public parser / checker / runtime surface

### Package 88 — fixed-subset-source-sample-corpus-scope-and-file-layout ratchet

- current source:
  `specs/examples/315`
  `specs/examples/316`
  `specs/examples/561`
- current recommendation:
  close 済み。repo-root `samples/current-l2/` flat `.txt` layer を第 3 層 source corpus として保つ minimum を `CurrentL2FixedSubsetSourceSampleCorpusScopeAndFileLayoutManifest` へ actualize し、initial cluster / directory / extension / naming / non-goal を narrow に固定した。
- stop line:
  representative / fixture / source mapping matrix
  actual sample file content
  parser-to-`Program` lowering
  bless / regression policy

### Package 89 — phase6-request-clause-suite-publicization comparison

- current recommendation:
  close 済み。shared single attachment frame と source-corpus scope/layout minimum を保ったまま、request-local `require` / `ensure` fixed two-slot suite bridge を `CurrentL2RequestClauseSuiteManifest`、`Stage3RequestClauseSuite`、`parse_stage3_request_clause_suite_text()` に actualize し、crate-local non-production parser carrier として inspectable にした。
- stop line:
  perform head final public parser API
  span-rich diagnostics
  final grammar
  final public parser / checker / runtime surface

### Package 90 — phase6-perform-head-final-public-parser-api comparison

- current recommendation:
  close 済み。request clause suite bridge を current entry criteria に保ったまま、`perform` head の owner / op / target-or-via shape を `CurrentL2PerformHeadManifest`、`Stage3PerformTargetRef`、`Stage3PerformHead`、`parse_stage3_perform_head_text()` に actualize し、crate-local non-production parser carrier として inspectable にした。
- stop line:
  request clause suite bundle attachment
  span-rich diagnostics
  final grammar
  final public parser / checker / runtime surface

### Package 91 — phase6-perform-head-request-clause-bundle-attachment comparison

- current recommendation:
  close 済み。perform head structural carrier と request clause suite carrier を current entry criteria に保ったまま、`Stage3RequestHeadClauseBundle { perform_head, clause_suite, attachment_frame_kind }` thin wrapper first を `CurrentL2RequestHeadClauseBundleManifest`、`Stage3RequestAttachmentFrameKind`、`parse_stage3_request_head_clause_bundle_text()` に actualize し、helper-local non-production parser carrier として inspectable にした。
- current source:
  `specs/examples/562`
  `specs/examples/563`
  `specs/examples/564`
  `specs/examples/565`
- stop line:
  span-rich diagnostics
  final grammar
  final public parser / checker / runtime surface

### Package 92 — first strong typing finite-index layer

- current source:
  `specs/examples/557`
  `specs/examples/566`
  `faq_010.md`
- current recommendation:
  close 済み。finite decidable index fragment を checker-adjacent first layer として actualize し、IFC / taint、capture / lifetime、simple cost を current sample-visible static carrier に落とし、source-side first strong typing sample set `p10 / p11 / p12 / p15 / p16` を helper-local checker summary / verifier preview alignment / model-check projection pre-floor に繋いだ。stronger typed surface は still non-principal に保つ。
- stop line:
  stronger typed source principal
  final typed calculus
  final public verifier contract

### Package 93 — Lean-first formal skeleton hardening

- current source:
  `specs/examples/521`
  `specs/examples/567`
  `samples/lean/foundations/CurrentL2LabelModel.lean`
  `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
  `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`
  `samples/lean/foundations/CurrentL2ProofSkeleton.lean`
- current recommendation:
  close 済み。actual small proof fragment と generated stub corpus の役割差を保ったまま、`CurrentL2FiniteIndexFirstLayer.lean` と representative generated theorem stub corpus `p15 / p16` widening を actualize し、Package 92 の first layer と first theorem placeholder 群を drift なく日本語 explanation に同期した。
- stop line:
  production prover binding
  final proof object public contract
  final public verifier contract

### Package 94 — theorem-first and model-check second-line carrier

- current source:
  `specs/examples/470`
  `specs/examples/478`
  `specs/examples/532`
  `specs/examples/568`
- current recommendation:
  close 済み。notebook-first theorem line、Lean-first experimental binding、row-local model-check carrier、brand-neutral reserve を維持したまま、theorem public seam quartet keep、model-check second-line / row-local carrier widening、guarded helper summary の `bridge_floor_refs` reconnect を actualize した。
- stop line:
  final public theorem/model-check contract
  concrete theorem/model-check production binding
  final public verifier contract

### Package 95 — order/handoff source surface and artifacts

- current source:
  `specs/examples/472`
  `specs/examples/473`
  `specs/examples/527`
  `specs/examples/533`
  `specs/examples/569`
- current recommendation:
  close 済み。explicit edge-row principal、stage-block secondary、repo-local emitted artifact reading、delegated RNG reserve、late-join negative pair を `run-source-sample` helper summary に揃え、low-level exact source surface へは戻さない。
- stop line:
  final source wording
  final emitted-artifact schema
  low-level exact source surface

### Package 96 — authoritative-room first scenario

- current source:
  `specs/examples/467`
  `specs/examples/471`
  `specs/examples/570`
  `plan/16-shared-space-membership-and-example-boundary.md`
- current recommendation:
  close 済み。authority-ack / single room authority / authoritative serial transition / authority_rng / late join visible past / stale reconnect fail-then-refresh / replay invalidation を first room default profile として representative run / CLI / artifact / docs に揃え、`authoritative_room_first_scenario_actual_adoption` helper summary に `p07 / p08` reached、`p09` reserve、`p13 / p14` negative pair を同期した。
- stop line:
  distributed fairness theorem
  exhaustive shared-space catalog
  final public witness/provider/artifact contract

### Package 97 — reserve strengthening

- current source:
  `specs/examples/476`
  `specs/examples/477`
  `specs/examples/478`
  `specs/examples/571`
- current recommendation:
  close 済み。first completion line を壊さず、`auditable_authority_witness`、`delegated_rng_service`、model-check second-line concretization を reserve strengthening lane として narrow に切り分け、`run-source-sample` helper summary の `authoritative_room_reserve_strengthening_lane` に actualize した。
- stop line:
  final public witness/provider schema
  distributed randomness provider adoption
  final public checker artifact

### Package 98 — documentation/report closeout

- current source:
  `progress.md`
  `tasks.md`
  `plan/90-source-traceability.md`
  `specs/examples/572`
- current recommendation:
  close 済み。once-through sequence を stale wording なしで同期し、二大問題それぞれの簡潔な日本語解説付き sample guide と repo-local helper runner を `samples/` / `scripts/` に actualize した。
- stop line:
  final public language completion claim
  packaging / FFI / engine adapter adoption

### Package 99 — theorem/model-check public-seam residual bundle

- current source:
  `specs/examples/568`
  `specs/examples/506`
  `specs/examples/507`
  `specs/examples/573`
- current recommendation:
  close 済み。`p06 / p10 / p11 / p12 / p15 / p16` を representative sample bundle に保ち、`matrix problem1` helper と sample README で theorem result-object / model-check public-checker / final public-contract reopen threshold の residual mixed gate を readable に actualize した。
- stop line:
  final public theorem result object
  final public checker artifact
  final public verifier contract

### Package 100 — witness/provider/public-shape residual bundle

- current source:
  `specs/examples/571`
  `specs/examples/505`
  `specs/examples/574`
- current recommendation:
  close 済み。`p07 / p08 / p09 / p13 / p14` を representative sample bundle に保ち、`matrix problem2` helper と sample README で first-line representative / reserve practical route / negative static-stop pair / public-shape residual を readable に actualize した。
- stop line:
  final public witness schema
  final public provider receipt schema
  combined public contract

### Package 101 — theorem-first pilot bundle

- current source:
  `specs/examples/470`
  `specs/examples/508`
  `specs/examples/509`
  `specs/examples/573`
  `specs/examples/575`
- current recommendation:
  close 済み。guided sample `problem1`、`matrix problem1`、`samples/lean/current-l2/`、repo-local emitted theorem artifact refs を `bundle problem1` helper と sample README に束ね、Problem 1 representative sample から theorem-first pilot artifact 群まで一本道で辿れる repo-local bundle に actualize した。
- stop line:
  final public theorem contract
  concrete theorem prover brand adoption
  final public verifier contract

### Package 102 — authoritative-room scenario bundle

- current source:
  `specs/examples/570`
  `specs/examples/571`
  `specs/examples/574`
  `specs/examples/576`
- current recommendation:
  close 済み。guided sample `problem2`、`matrix problem2`、authoritative-room first scenario helper summary、reserve lane helper summary、negative static-stop pair を `bundle problem2` helper と sample README に束ね、Problem 2 representative pair / reserve / negative / artifact route まで一本道で辿れる repo-local scenario bundle に actualize した。
- stop line:
  final public witness/provider/artifact contract
  exhaustive shared-space catalog
  installed-binary / host integration target

### Package 103 — parser-side companion surface bundle

- current source:
  `specs/examples/468`
  `specs/examples/472`
  `specs/examples/564`
  `specs/examples/565`
  `specs/examples/575`
  `specs/examples/576`
  `specs/examples/577`
- current recommendation:
  close 済み。`p06` と `p07 / p08` representative slice を、final grammar へ上げずに `samples/prototype/current-l2-parser-companion/` と `Stage3RequestHeadClauseBundle` parse test へ narrow に actualize し、thin experimental companion surface / non-production parser-side carrier の current cut を sample-visible に戻した。
- stop line:
  final grammar
  final public parser / checker / runtime surface
  final public theorem / witness-provider contract

### Package 104 — parser-side bundle-to-helper bridge

- current source:
  `specs/examples/552`
  `specs/examples/565`
  `specs/examples/575`
  `specs/examples/576`
  `specs/examples/577`
  `specs/examples/578`
- current recommendation:
  close 済み。parser-side carrier を current `bundle problem1 / problem2`、guided sample、helper summary へ thin bridge し、`parser_companion_path` を通じて representative slice が docs / helper / parser-side carrier の 3 点で一本道になる minimum を actualize した。
- stop line:
  final public parser / checker / runtime API
  full `Program` lowering
  final public verifier contract

### Package 105 — parser companion inspector

- current source:
  `specs/examples/577`
  `specs/examples/578`
  `specs/examples/579`
- current recommendation:
  close 済み。`p06 / p07 / p08` companion sample の parse result を final parser/public API に上げず、repo-local JSON / pretty inspector command と focused test で inspectable にした。
- stop line:
  final public parser / checker / runtime API
  full `Program` lowering
  final diagnostics / span-rich contract

### Package 106 — parser companion mapping matrix

- current source:
  `specs/examples/577`
  `specs/examples/578`
  `specs/examples/579`
  `specs/examples/580`
- current recommendation:
  close 済み。representative slice について original prototype / parser companion / guided bundle / Lean artifact / anchor spec-report の mapping matrix を `mapping` helper と sample README table に actualize し、bundle helper だけに依存しない readable cut を作った。
- stop line:
  exhaustive sample catalog
  final public tutorial surface
  final public parser / checker / runtime API

### Package 107 — explained representative problem sample bundles

- current source:
  `specs/examples/575`
  `specs/examples/576`
  `specs/examples/580`
  `specs/examples/581`
- current recommendation:
  close 済み。Problem 1 / Problem 2 の representative sample を `samples/problem-bundles/` の簡潔な日本語 guide と `bundle problem1|problem2` の `sample_bundle_doc` へ actualize し、runner / Lean artifact / parser companion / guided helper の 4 本を bundle README から辿れるようにした。
- stop line:
  exhaustive tutorial expansion
  final public parser / checker / runtime API
  exhaustive shared-space catalog

### Package 108 — representative problem bundle smoke runner

- current source:
  `specs/examples/580`
  `specs/examples/581`
  `specs/examples/582`
- current recommendation:
  close 済み。Problem 1 / Problem 2 の representative sample bundle guide に書いた主要 command 群を `smoke problem1|problem2` helper で順に実行し、guide / helper / runnable evidence の 3 点が drift していないことを 1 コマンドずつで確認できるようにした。
- stop line:
  exhaustive workflow automation
  final public CLI / tutorial surface
  final public parser / checker / runtime API

### Package 109 — representative problem bundle aggregate smoke summary

- current source:
  `specs/examples/582`
  `specs/examples/583`
- current recommendation:
  close 済み。`smoke-all` helper を actualize し、representative 2 問題の smoke 成否と step inventory を 1 コマンドで compact に読める repo-local summary helper を追加した。
- stop line:
  exhaustive workflow automation
  aggregate CI / installed-binary contract
  final public CLI / tutorial surface

### Package 110 — representative problem bundle failure-focused smoke diagnostics

- current source:
  `specs/examples/583`
  `specs/examples/584`
- current recommendation:
  close 済み。`smoke-all` を compact summary のまま保ちつつ、failure 時に failed step / command / return code / output excerpt を narrow に surfacing し、aggregate failure 自体も non-zero exit へ戻した。
- stop line:
  exhaustive workflow automation
  aggregate CI / installed-binary contract
  final public CLI / tutorial surface

### Package 111 — representative problem bundle quickstart walkthrough hardening

- current source:
  `specs/examples/581`
  `specs/examples/582`
  `specs/examples/583`
  `specs/examples/584`
- current recommendation:
  close 済み。`samples/problem-bundles/problem1|problem2` に smoke / matrix / bundle / parser companion inspector の mini walkthrough と `見るべき結果` を揃え、representative sample guide から「まず何を実行し、何を確認するか」を 1 画面で読めるようにした。
- stop line:
  exhaustive tutorial surface
  exhaustive sample catalog
  final public CLI / tutorial surface

### Package 112 — representative problem quickstart CLI mirror

- current source:
  `specs/examples/585`
- current recommendation:
  close 済み。bundle doc 側の 4 段 quickstart を `scripts/current_l2_guided_samples.py quickstart problem1|problem2` からも problem ごとに表示できるようにし、doc-only ではなく helper-side summary にも mirrored quickstart を持たせた。
- stop line:
  exhaustive tutorial surface
  final public CLI / tutorial surface
  final public parser / checker / runtime API

### Package 113 — representative problem quickstart parity checks

- current source:
  `specs/examples/585`
  `specs/examples/586`
- current recommendation:
  close 済み。sample bundle doc と `quickstart problem1|problem2` helper が同じ 4-step 導線を保っていることを focused test / helper で確認し、doc-side と helper-side の quickstart drift を早めに拾えるようにした。
- stop line:
  exhaustive tutorial surface
  final public CLI / tutorial surface
  final public parser / checker / runtime API

### Package 114 — representative problem mixed-gate reopen map refresh

- current source:
  `specs/examples/573`
  `specs/examples/574`
  `specs/examples/587`
  `specs/examples/588`
- current recommendation:
  close 済み。representative entrypoint floor が固まったので、Problem 1 / Problem 2 の mixed-gate reopen point を quickstart / bundle / matrix / smoke 現況に合わせて短く再整理し、`reopen-map` helper と sample bundle doc 側の `現在の mixed gate 再開点` section で Problem-local mixed gate と global true user-spec residual を切り分けて読めるようにした。
- stop line:
  final public theorem/model-check/witness-provider contract
  final public CLI / tutorial surface
  final public parser / checker / runtime API

### Package 115 — Problem 1 theorem/model-check mixed-gate split refresh

- current source:
  `specs/examples/588`
  `specs/examples/589`
- current recommendation:
  close 済み。Problem 1 remaining mixed gate を、typed source principal / theorem public-contract / model-check public-contract の separate package へ split し、aggregate reopen map から次 package 名まで narrow に戻した。
- stop line:
  final public theorem contract
  final public model-check contract
  final public verifier contract

### Package 116 — Problem 2 order-handoff/public-shape mixed-gate split refresh

- current source:
  `specs/examples/588`
  `specs/examples/589`
- current recommendation:
  close 済み。Problem 2 remaining mixed gate を、source wording / emitted schema と witness-provider public shape の separate package へ split し、aggregate reopen map から次 package 名まで narrow に戻した。
- stop line:
  final public witness/provider/artifact contract
  exhaustive shared-space catalog
  final public parser / checker / runtime API

### Package 117 — Problem 1 typed source principal split

- current source:
  `specs/examples/590`
- current recommendation:
  close 済み。checker-adjacent principal / structural marker first / finite decidable index fragment first を保ったまま、typed source principal residual を theorem/model-check public-contract residual から切り離して narrow に読む helper/doc cut を actualize した。
- stop line:
  final typed source principal
  final typed calculus
  final public verifier contract

### Package 118 — Problem 1 theorem public-contract split

- current source:
  `specs/examples/591`
- current recommendation:
  close 済み。review-unit transport first / notebook-consumer object first / theorem-first pilot keep を保ったまま、theorem public-contract residual を typed / model-check residual から切り離して narrow に読む helper/doc cut を actualize した。
- stop line:
  final public theorem contract
  concrete theorem prover brand
  final public verifier contract

### Package 119 — Problem 1 model-check public-contract split

- current source:
  `specs/examples/592`
- current recommendation:
  close 済み。row-local property route first / checker-artifact route first / reopen-threshold helper mirror keep を保ったまま、model-check public-contract residual を typed / theorem residual から切り離して narrow に読む helper/doc cut を actualize した。
- stop line:
  first settled property language
  final public checker artifact
  final public verifier contract

### Package 120 — Problem 2 source wording / emitted schema split

- current source:
  `specs/examples/593`
- current recommendation:
  close 済み。edge-row principal / stage-block secondary / serial reserve keep を保ったまま、source wording / emitted schema residual を witness-provider public-shape residual から切り離して narrow に読む helper/doc cut を actualize した。
- stop line:
  final source-surface handoff wording
  final emitted-artifact schema
  final public parser / checker / runtime API

### Package 121 — Problem 2 witness-provider public-shape split

- current source:
  `specs/examples/594`
- current recommendation:
  close 済み。claim/payload split first / route-schema split first / authoritative-room representative pair keep を保ったまま、witness-provider public-shape residual を source wording residual から切り離して narrow に読む helper/doc cut を actualize した。
- stop line:
  final public witness/provider/artifact contract
  stronger fairness / replay profile
  exhaustive shared-space catalog

### Package 122 — residual public-seam maintenance sync

- current source:
  `specs/examples/595`
- current recommendation:
  close 済み。`reopen-map problem1|problem2` helper public surface から stale な `next split packages` 表示を外し、`split package closeout` と remaining mixed gate を helper public surface に再同期した。
- stop line:
  final public wording / contract adoption
  final public parser / checker / runtime API
  exhaustive shared-space catalog

### Package 123 — remaining mixed-gate compression after split closeout

- current source:
  `specs/examples/588`
  `specs/examples/589`
  `specs/examples/595`
  `specs/examples/596`
- current recommendation:
  close 済み。`residuals` helper と sample bundle doc 側の residual lane entrypoint を actualize し、remaining mixed gate を `problem1-final-public-seams` / `problem2-final-public-seams` / `syntax-modality-final-marker` lane へ圧縮した。
- stop line:
  final public theorem/model-check/witness-provider contract
  packaging / FFI / engine adapter
  exhaustive shared-space catalog

### Package 124 — Problem 1 final-public-seam lane follow-up

- current source:
  `specs/examples/596`
  `specs/examples/590`
  `specs/examples/591`
  `specs/examples/592`
  `specs/examples/597`
- current recommendation:
  close 済み。`reopen-map problem1`、`matrix problem1`、`bundle problem1`、`residuals` に加えて `lane problem1-final-public-seams` を actualize し、Problem 1 final public seam lane を typed / theorem / model-check reopen order と stop line 付きの独立 lane として further narrow に扱えるようにした。
- stop line:
  final public theorem contract
  final public checker artifact
  final public verifier contract

### Package 125 — Problem 2 final-public-seam lane follow-up

- current source:
  `specs/examples/596`
  `specs/examples/593`
  `specs/examples/594`
  `specs/examples/598`
- current recommendation:
  close 済み。`reopen-map problem2`、`matrix problem2`、`bundle problem2`、`residuals` に加えて `lane problem2-final-public-seams` を actualize し、Problem 2 final public seam lane を final wording / witness-provider public-shape reopen order と stop line 付きの独立 lane として further narrow に扱えるようにした。
- stop line:
  final source-surface handoff wording
  final public witness/provider/artifact contract
  exhaustive shared-space catalog

### Package 126 — syntax-modality final-marker lane follow-up

- current source:
  `specs/examples/596`
  `specs/10-open-questions.md`
  `plan/06-surface-notation-status.md`
  `plan/13-heavy-future-workstreams.md`
- current recommendation:
  close 済み。`lane syntax-modality-final-marker` helper を主 anchor に、final modal foundation / source marker を Problem-local seam と true user-spec residual から切り分けたまま later mixed gate として圧縮した。
- close evidence:
  `docs/reports/0879`
  `specs/examples/599`
  `samples/problem-bundles/README.md`
  `scripts/current_l2_guided_samples.py`
- stop line:
  final modal foundation adoption
  final source marker adoption
  final parser grammar

### Package 127 — typed-checker first executable slice

- current source:
  `specs/examples/557`
  `samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt`
  `samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt`
  `samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt`
  `samples/prototype/current-l2-typed-proof-model-check/p15-typed-capture-escape-rejected.txt`
  `samples/prototype/current-l2-typed-proof-model-check/p16-typed-remote-call-budget-exceeded.txt`
- current recommendation:
  close 済み。`check-source-sample` focused checker command と Problem 1 bundle doc 側の quickstart 導線を actualize し、finite decidable index fragment / IFC / capture / lifetime / simple cost の current first line を checker-adjacent executable slice に ratchet した。
- close evidence:
  `docs/reports/0880`
  `specs/examples/600`
  `crates/mir-runtime/src/current_l2_cli.rs`
  `crates/mir-runtime/tests/current_l2_operational_cli.rs`
  `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- stop line:
  final typed source principal
  final IFC syntax
  final public verifier contract

### Package 128 — theorem-first emitted-artifact hardening

- current source:
  `specs/examples/470`
  `specs/examples/474`
  `specs/examples/601`
  `samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt`
  `samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt`
  `samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt`
- current recommendation:
  close 済み。notebook-first / brand-neutral のまま、`emit-theorem problem1` helper により theorem-first pilot を emitted artifact / Lean bundle / representative sample loop として harden した。
- close evidence:
  `docs/reports/0881`
  `samples/problem-bundles/problem1-typed-theorem-model-check.md`
  `scripts/current_l2_guided_samples.py`
  `scripts/tests/test_current_l2_guided_samples.py`
- stop line:
  final public theorem contract
  concrete theorem prover brand
  final public verifier contract

### Package 129 — authoritative-room runnable scenario hardening

- current source:
  `specs/examples/467`
  `specs/examples/471`
  `samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt`
  `samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt`
  `samples/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.txt`
  `samples/prototype/current-l2-order-handoff/p13-dice-late-join-missing-publication-witness.txt`
  `samples/prototype/current-l2-order-handoff/p14-dice-late-join-handoff-before-publication.txt`
- current recommendation:
  close 済み。representative / reserve / negative pair をまとめた runnable scenario bundle を harden し、authoritative-room first scenario current default を smoke / emitted artifact / refresh behavior で再確認できる。
- stop line:
  final source-surface handoff wording
  final public witness/provider/artifact contract
  exhaustive shared-space catalog

### Package 130 — Problem 1 executable residual reopen sync

- current source:
  `specs/examples/597`
  `specs/examples/600`
  `specs/examples/601`
  `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- current recommendation:
  `check-source-sample` executable slice、`emit-theorem problem1` emitted-artifact loop、`lane problem1-final-public-seams` reopen lane を 1 本の current executable reopen order として docs / helper へ再同期する。
- stop line:
  final typed source principal
  final public theorem contract
  final public checker artifact
  final public verifier contract

### Package 131 — Problem 2 executable residual reopen sync

- current source:
  `specs/examples/598`
  `specs/examples/602`
  `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- current recommendation:
  `emit-scenario problem2` repo-local loop、`lane problem2-final-public-seams` reopen lane、`residuals` global residual summary を 1 本の current executable reopen order として docs / helper へ再同期する。
- stop line:
  final source-surface handoff wording
  final public witness/provider/artifact contract
  exhaustive shared-space catalog

### Package 132 — repo-local once-through closeout sync after executable loops

- current source:
  `specs/examples/596`
  `specs/examples/601`
  `specs/examples/602`
  `specs/examples/603`
  `specs/examples/604`
- current recommendation:
  close 済み。Package 127...131 で揃った executable loop 群を踏まえ、`closeout` helper と snapshot docs に current first line / mixed-gate lane / true user-spec residual / next self-driven queue を再圧縮した。
- stop line:
  final public language completion claim
  packaging / installed binary / FFI / engine adapter adoption
  exhaustive shared-space catalog adoption

### Package 133 — reserve integration entrypoint sync

- current source:
  `specs/examples/470`
  `specs/examples/476`
  `specs/examples/477`
  `specs/examples/478`
  `specs/examples/571`
  `specs/examples/606`
- current recommendation:
  close 済み。theorem-first external pilot、`auditable_authority_witness`、`delegated_rng_service`、model-check second-line reserve を、closeout helper 後の next reopen order として再圧縮した。
- stop line:
  final public theorem contract
  final public witness/provider/artifact contract
  concrete theorem/model-check production binding

### Package 134 — parser-side residual closeout sync

- current source:
  `specs/examples/564`
  `specs/examples/565`
  `specs/examples/577`
  `specs/examples/578`
  `specs/examples/579`
  `specs/examples/580`
  `specs/examples/607`
- current recommendation:
  close 済み。parser companion surface、parser-side tranche、final parser/checker/runtime API residual を `lane parser-side-residual` と `residuals` / `closeout` / `reserve` に再同期し、repo-local once-through closeout line と混ぜない mixed gate lane として固定した。
- stop line:
  final parser grammar
  final public parser / checker / runtime API
  public tutorial surface adoption

### Package 135 — true user-spec residual freeze sync

- current source:
  `specs/examples/431`
  `specs/examples/432`
  `specs/examples/469`
  `specs/examples/520`
  `specs/examples/608`
- current recommendation:
  close 済み。packaging / FFI / engine adapter / exhaustive shared-space catalog / upper-layer application target を explicit hold line に固定し、self-driven queue と混ぜない。
- close evidence:
  `docs/reports/0888`
  `python3 scripts/current_l2_guided_samples.py hold-line`
- stop line:
  installed binary / packaging adoption
  concrete host / engine target adoption
  exhaustive shared-space catalog adoption

## later mixed-gate topics

| 順番 | lane | macro | topic | next gate |
|---|---|---|---|---|
| 5 | theory-lab | `Macro 5/7` | final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema | mixed gate |
| 6 | theory-lab | `Macro 5/7` | first settled property language / concrete model-check tool brand / final public checker artifact / actual public checker migration / actual emitted verifier handoff artifact | mixed gate |
| 7 | theory-lab | `Macro 5/6` | final source-surface handoff wording / emitted-artifact schema | mixed gate |
| 8 | theory-lab | `Macro 5/6` | final public witness schema / provider receipt schema / combined public contract / emitted-handoff contract | mixed gate |
| 9 | theory-lab | `Macro 5` | final modal foundation / source marker | mixed gate |
| 10 | reserve integration | `Macro 6/7` | exhaustive shared-space catalog / packaging / FFI targets | mixed + user-spec residual |

## current recommendation

- comparison debt より adoption debt を優先する。
- `specs/examples/458...465` は integrator / compare floor として保持するが、current line の principal anchor は `466...469`、`475`、`520` に移った。
- representative Lean sample set actual Lean execution reached は current queue zero を意味しない。
- closeout 用 numbered queue は closed 状態であり、next reopen line は reserve integration / later mixed gate / true user-spec hold line である。reserve integration lane、parser-side residual lane、hold-line helper は Package 133 / 134 / 135 まで actualize 済みである。
- authoritative-room default profile と append-friendly contrast room を shared-space current working subset に置く。
- `auditable_authority_witness` と `delegated_rng_service` は close 済み strengthening / practical actualization に移し、final public provider receipt / witness schema は mixed gate に残す。
- final parser grammar、final public API、final public verifier contract、installed binary、exhaustive catalog は near-term line に入れない。
