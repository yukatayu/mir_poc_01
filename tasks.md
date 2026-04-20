# tasks

最終更新: 2026-04-20 16:44 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- 規範判断の正本は `specs/`、detail-side の repository memory は `plan/11`、`plan/12`、`plan/13`、`plan/16`、`plan/17`、`plan/18` に寄せる。
- `tasks.md` は append-only ではなく、毎回 snapshot に合わせて全体を書き直す。

## current status at task level

- current mapped corpus では、
  - authored sixteen
  - corrected prototype set `p01...p16`
  - runner / CLI / regression / helper-local compare floor
  が already runnable である。
- representative Lean sample set `e5 / p06 / p10 / p11 / p12 / p15 / p16 / p07 / p08 / p09 / p13 / p14` は actual Lean execution reached であり、`samples/lean/current-l2/` に committed corpus として保存済みである。
- source-side finite-index first layer は `p10 / p11 / p12 / p15 / p16` まで corrected prototype と helper-local checker summary に actualize 済みである。
- Lean-first formal skeleton hardening は close 済みであり、`samples/lean/foundations/` の actual small proof fragment と `samples/lean/current-l2/` generated stub corpus の役割差を `CurrentL2FiniteIndexFirstLayer.lean` と representative generated theorem stub corpus `p15 / p16` widening まで source-backed に同期した。
- `samples/lean/foundations/` には
  - `CurrentL2LabelModel.lean`
  - `CurrentL2IfcSecretExamples.lean`
  - `CurrentL2FiniteIndexFirstLayer.lean`
  - `CurrentL2ProofSkeleton.lean`
  の actual small proof fragment が入った。
- Problem 1 current first line は、
  full dependent core を first public core に入れず、
  finite decidable index fragment、`Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ C` conceptual spine、
  IFC / taint、capture / lifetime、simple cost を
  first strong typing layer の principal target に置く読みへ同期済みである。
- ここでいう principal target は checker-adjacent first layer を指し、
  stronger typed surface を early に source principal へ昇格する判断とは切り分けて読む。
- したがって、remaining work の主眼は
  **repo-local once-through near-end completion**
  として整理し直すのが自然である。
  current self-driven sequence は、
  - Package 119 Problem 1 model-check public-contract split
  - Package 120 Problem 2 source wording / emitted schema split
  - Package 121 Problem 2 witness-provider public-shape split
  に分けて追う。
- exact rough stimulus は `samples/not_implemented/` preservation bucket に残し、corrected runnable version と混同しない。

## current executable floor

- `samples/current-l2/`
  - authored sixteen は inventory / runner / verification ladder / emitted artifact wiring / regression helper に乗っている
- `samples/prototype/`
  - corrected prototype set `p01...p16` は explicit path + adjacent host-plan sidecar で runnable
  - helper-local `debug_outputs` / `verification_preview` / `artifact_preview` を見せる current cut がある
- `samples/lean/`
  - `foundations/` は actual small proof fragment
  - `current-l2/` は representative Lean sample set generated stub corpus
  - `manifest.json` は Lean verification summary
  - generated current-L2 theorem stubs は `sorry` を含むため、artifact well-formedness / bridge alignment evidence として読む
- theorem / model-check / order-handoff / shared-space current floor
  - theorem-side:
    theorem-first pilot、binding preflight、Lean-stub pilot、artifact-conformance bridge、representative trace alignment、public-seam compression、toolchain probe/reopen manifest、representative Lean sample set actual Lean execution、reopen-threshold helper mirror
  - model-check side:
    row-local property route、checker-artifact route、final public-contract reopen threshold、public-seam compression、reopen-threshold helper mirror
  - order-handoff / shared-space side:
    authoritative-room vertical slice、surface actual adoption、source-wording route actual adoption、source-surface / artifact route actual adoption、authoritative-room first scenario helper summary actual adoption、serial-scope reserve surface、witness/provider route/schema route actual adoption、emitted-contract trace alignment bridge、public-seam compression、public-seam helper mirror、CLI `surface_preview`、late-join negative static stop `p13 / p14`

## ordered self-driven packages

| package | question | package weight | macro phase | current recommendation | promotion criteria |
|---|---|---|---|---|---|
| `119` Problem 1 model-check public-contract split | Problem 1 model-check public-contract residual を typed / theorem residual から切り離して narrow にする | `S-M` | `Macro 5/7` | row-local property route first / checker-artifact route first を保ったまま model-check reopen point を独立 package に戻す | model-check public-contract residual が独立 package として読める |
| `120` Problem 2 source wording / emitted schema split | Problem 2 source wording / emitted schema residual を witness-provider public-shape residual から切り離して narrow にする | `S-M` | `Macro 5/6` | edge-row principal / stage-block secondary を保ったまま source wording reopen point を独立 package に戻す | source wording / emitted schema residual が独立 package として読める |
| `121` Problem 2 witness-provider public-shape split | Problem 2 witness/provider public-shape residual を source wording residual から切り離して narrow にする | `S-M` | `Macro 5/6` | claim/payload split first / route-schema split first を保ったまま witness-provider reopen point を独立 package に戻す | witness-provider public-shape residual が独立 package として読める |

## recently closed package note

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
  concrete production prover binding、final proof object public contract、final public verifier contract

### Package 56 — layered strong typing / IFC first-fragment

- current reading:
  first actual adoption package として close 済みである。
- close evidence:
  `specs/examples/522`
  `specs/examples/523`
  `specs/examples/524`
  `specs/examples/557`
  `samples/lean/foundations/CurrentL2LabelModel.lean`
  `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
  `samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt`
  `samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt`
  `samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt`
- kept later:
  stronger typed source principal、checker-hint / diagnostics widening beyond IFC trio、actual checker payload family、final typed calculus、final IFC syntax、final public verifier contract

### Package 91 — phase6-perform-head-request-clause-bundle-attachment comparison

- current reading:
  close 済み。Package 89 / 90 の separate carrier minimum を保ったまま、`Stage3RequestHeadClauseBundle { perform_head, clause_suite, attachment_frame_kind }` thin wrapper first を `CurrentL2RequestHeadClauseBundleManifest`、`Stage3RequestAttachmentFrameKind`、`parse_stage3_request_head_clause_bundle_text()` に actualize した。
- close evidence:
  `specs/examples/564`
  `specs/examples/565`
  `crates/mir-ast/src/current_l2.rs`
  `crates/mir-ast/tests/current_l2_request_head_clause_bundle_manifest.rs`
  `crates/mir-ast/tests/current_l2_stage3_request_head_clause_bundle_spike.rs`
- kept later:
  span-rich diagnostics
  final grammar
  final public parser / checker / runtime surface
  full `Program` lowering

### Package 92 — first strong typing finite-index layer

- current reading:
  close 済み。finite decidable index fragment の checker-adjacent first layer を `p10 / p11 / p12 / p15 / p16` source-side sample set まで widen し、capture / lifetime negative と simple cost negative を prototype / helper-local checker summary / docs に actualize した。
- close evidence:
  `specs/examples/557`
  `specs/examples/566`
  `samples/prototype/current-l2-typed-proof-model-check/p15-typed-capture-escape-rejected.txt`
  `samples/prototype/current-l2-typed-proof-model-check/p16-typed-remote-call-budget-exceeded.txt`
  `crates/mir-runtime/src/current_l2_cli.rs`
  `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
  `crates/mir-runtime/tests/current_l2_verifier_preview_alignment.rs`
  `crates/mir-runtime/tests/current_l2_model_check_projection_prefloor.rs`
  `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- kept later:
  stronger typed source principal
  final typed calculus
  final public verifier contract
  Lean generated corpus widening

### Package 93 — Lean-first formal skeleton hardening

- current reading:
  close 済み。`samples/lean/foundations/` の actual small proof fragment と `samples/lean/current-l2/` generated stub corpus の役割差を保ったまま、`CurrentL2FiniteIndexFirstLayer.lean` と representative generated theorem stub corpus `p15 / p16` widening を actualize し、Package 92 の finite-index first layer と theorem-side placeholder 群を drift なく日本語 explanation に同期した。
- close evidence:
  `specs/examples/567`
  `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`
  `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.md`
  `samples/lean/current-l2/p15-typed-capture-escape-rejected/`
  `samples/lean/current-l2/p16-typed-remote-call-budget-exceeded/`
  `scripts/current_l2_lean_sample_sync.py`
  `scripts/tests/test_current_l2_lean_sample_sync.py`
- kept later:
  production prover binding
  final proof object public contract
  final public verifier contract

### Package 94 — theorem-first and model-check second-line carrier

- current reading:
  close 済み。theorem public seam の representative quartet `e5 / p06 / p07 / p08` は維持したまま、source-side first strong typing sample set `p10 / p11 / p12 / p15 / p16` を theorem/model-check helper-local `bridge_floor_refs` と model-check second-line / row-local carrier widening に reconnect した。
- close evidence:
  `specs/examples/568`
  `docs/reports/0851`
  `crates/mir-runtime/src/current_l2_cli.rs`
  `crates/mir-runtime/tests/current_l2_operational_cli.rs`
  `crates/mir-runtime/tests/current_l2_model_check_second_line_concretization.rs`
  `crates/mir-runtime/tests/current_l2_model_check_row_local_property_actual_adoption.rs`
- kept later:
  final public theorem result object
  consumer-shaped theorem payload public contract
  first settled property language
  concrete theorem/model-check tool binding
  final public checker artifact
  final public verifier contract

## active package notes

### Package 95 — order/handoff source surface and artifacts

- current reading:
  close 済み。explicit edge-row principal、stage-block secondary、repo-local emitted artifact reading、`serial on ...` reserve、late-join negative pair `p13 / p14` を `run-source-sample` helper summary に揃えた。
- stop line:
  final source wording
  low-level exact source surface
  final emitted-handoff contract

### Package 96 — authoritative-room first scenario

- current reading:
  close 済み。authority-ack / single room authority / authoritative serial transition / authority_rng / late join visible past / stale reconnect fail-then-refresh / replay invalidation を first room default profile として representative run / artifact / docs に揃え、`authoritative_room_first_scenario_actual_adoption` helper summary に `p07 / p08` reached、`p09` reserve、`p13 / p14` negative pair を同期した。
- stop line:
  distributed fairness theorem
  exhaustive shared-space catalog
  final public witness/provider/artifact contract

### Package 97 — reserve strengthening

- current reading:
  close 済み。first completion line を壊さずに、`auditable_authority_witness`、`delegated_rng_service`、model-check second-line concretization を reserve strengthening lane として narrow に切り分け、`authoritative_room_reserve_strengthening_lane` helper summary に `p07` witness + model-check、`p08` model-check、`p09` delegated RNG + model-check、`p05` guard-only を同期した。
- stop line:
  final public witness/provider schema
  distributed randomness provider adoption
  final public checker artifact

### Package 98 — documentation/report closeout

- current reading:
  close 済み。`specs/` / `plan/` / `progress.md` / `tasks.md` / `Documentation.md` / `docs/reports/` / `plan/90` を stale wording なしで同期し、二大問題それぞれの current first line を `samples/prototype/.../README.md` と `scripts/current_l2_guided_samples.py` による簡潔な日本語 guide として actualize した。
- close evidence:
  `specs/examples/572`
  `docs/reports/0855`
  `samples/prototype/current-l2-typed-proof-model-check/README.md`
  `samples/prototype/current-l2-order-handoff/README.md`
  `scripts/current_l2_guided_samples.py`
  `scripts/tests/test_current_l2_guided_samples.py`
- stop line:
  final public language completion claim
  packaging / FFI / engine adapter adoption

### Package 99 — theorem/model-check public-seam residual bundle

- current reading:
  close 済み。Problem 1 current first line 自体は close 済みのまま、representative sample bundle `p06 / p10 / p11 / p12 / p15 / p16` と existing helper summary を使い、theorem result-object / model-check public-checker / final public-contract reopen threshold の residual mixed gate を `matrix problem1` helper と README で narrow に整理した。
- current recommendation:
  sample corpus や theorem/model-check compare floor を増やさず、guided sample `problem1`、existing `bridge_floor_refs` / `verification_preview` / `artifact_preview` / `typed_checker_hint_preview`、`matrix problem1` を入口にして、public seam 残件の reopen 順を readable に保つ。
- stop line:
  final public theorem result object
  consumer-shaped theorem payload public contract
  concrete theorem/model-check tool binding
  final public checker artifact
  final public verifier contract

### Package 100 — witness/provider/public-shape residual bundle

- current reading:
  close 済み。Problem 2 current first line と reserve strengthening lane 自体は close 済みのまま、representative sample bundle `p07 / p08 / p09 / p13 / p14` と existing helper summary を使い、final public witness/provider/artifact contract mixed gate を `matrix problem2` helper と README で narrow に整理した。
- current recommendation:
  sample corpus を増やさず、guided sample `problem2`、existing `surface_preview` / `authoritative_room_first_scenario_actual_adoption` / `authoritative_room_reserve_strengthening_lane`、`matrix problem2` を入口にして、claim/payload split first の current cutを保ったまま public-shape residual を圧縮する。
- stop line:
  final public witness schema
  final public provider receipt schema
  final emitted-handoff contract
  exhaustive shared-space catalog

### Package 101 — theorem-first pilot bundle

- current reading:
  close 済み。Problem 1 residual matrix まで close 済みの line を、`bundle problem1` helper、`samples/lean/current-l2/` representative artifact、anchor spec / report まで束ねる repo-local theorem-first pilot bundle として actualize した。
- current recommendation:
  final public theorem contract や concrete prover brand を採らず、repo-local emitted artifact / Lean sample corpus / command path の bundle 化に留める。
- stop line:
  final public theorem result object
  concrete theorem prover brand
  final public verifier contract

### Package 102 — authoritative-room scenario bundle

- current reading:
  close 済み。Problem 2 residual matrix まで close 済みの line を、`bundle problem2` helper、representative pair / reserve / negative pair の Lean artifact、anchor spec / report まで束ねる repo-local scenario bundle として actualize した。
- current recommendation:
  final public witness/provider/artifact contract を採らず、repo-local scenario reading / artifact route / reserve split を一本道で読める bundle 化に留める。
- stop line:
  final public witness schema
  final public provider receipt schema
  exhaustive shared-space catalog

### Package 103 — parser-side companion surface bundle

- current reading:
  close 済み。`bundle problem1 / problem2` まで close 済みの line から、`p06` と `p07 / p08` representative slice を thin experimental companion surface / non-production parser-side carrier へ戻し、`samples/prototype/current-l2-parser-companion/` と `mir-ast` parse test で machine-check した。
- current recommendation:
  final grammar や final public parser/checker/runtime API を採らず、existing parser-side non-production carrier と representative bundle docs を結ぶ minimum companion surface だけを進める。
- stop line:
  final grammar
  final public parser / checker / runtime API
  final public theorem / witness-provider contract

### Package 104 — parser-side bundle-to-helper bridge

- current reading:
  close 済み。parser-side carrier を current `bundle problem1 / problem2`、guided sample、helper summary へ thin bridge し、`parser_companion_path` を通じて representative slice が docs / helper / parser-side carrier の 3 点で一本道になる minimum を作った。
- current recommendation:
  full `Program` lowering や final public API を採らず、representative slice を helper-local summary / guide へ戻す thin bridge だけを進める。
- stop line:
  final public parser / checker / runtime API
  full `Program` lowering
  final public verifier contract

### Package 105 — parser companion inspector

- current reading:
  close 済み。`p06 / p07 / p08` companion sample の parse result を repo-local JSON / pretty summary で inspectable にし、parser-side carrier を docs-only で終わらせない current cut を `mir-ast` example と focused test に actualize した。
- current recommendation:
  final public parser API や full `Program` lowering を採らず、representative slice 専用の small inspector command / example / helper と focused test に留める。
- stop line:
  final public parser / checker / runtime API
  full `Program` lowering
  final diagnostics / span-rich contract

### Package 106 — parser companion mapping matrix

- current reading:
  close 済み。parser companion path は bundle helper へ入ったので、representative slice の original prototype / parser companion / guided bundle / Lean artifact / anchor spec-report 対応を `mapping` helper と sample README table に actualize した。
- current recommendation:
  representative slice だけを対象に matrix を actualize し、reserve / negative line の exhaustive widening は still later に残す。
- stop line:
  exhaustive sample catalog
  final public tutorial surface
  final public parser / checker / runtime API

### Package 107 — explained representative problem sample bundles

- current reading:
  close 済み。二大問題それぞれの representative sample を `samples/problem-bundles/` の簡潔な日本語 guide と `bundle problem1|problem2` の `sample_bundle_doc` へ actualize し、runner / Lean artifact / parser companion / guided helper の 4 本を README から辿れるようにした。
- current recommendation:
  Problem 1 は `p06`、Problem 2 は `p07 / p08` を中心にし、current first line の意味と stop line を過剰に広げずに explained bundle を actualize する。
- stop line:
  exhaustive tutorial expansion
  final public parser / checker / runtime API
  exhaustive shared-space catalog

### Package 108 — representative problem bundle smoke runner

- current reading:
  close 済み。Problem 1 / Problem 2 の representative sample bundle guide に書いた主要 command 群を `smoke problem1|problem2` helper で順に実行し、guide / helper / runnable evidence の 3 点が drift していないことを 1 本の command surface で確認できるようにした。
- current recommendation:
  representative sample と current helper-local cut に限定し、bundle guide 全文の自動化や exhaustive workflow 化までは広げない。
- stop line:
  exhaustive workflow automation
  final public CLI / tutorial surface
  final public parser / checker / runtime API

### Package 109 — representative problem bundle aggregate smoke summary

- current reading:
  close 済み。`smoke-all` helper を actualize し、representative 2 問題の smoke 成否と step inventory をまとめて compact に読める repo-local summary helper を追加した。
- current recommendation:
  long raw output の再整形ではなく、per-problem smoke route を壊さずに aggregate summary だけを追加する。
- stop line:
  exhaustive workflow automation
  aggregate CI contract
  final public CLI / tutorial surface

### Package 110 — representative problem bundle failure-focused smoke diagnostics

- current reading:
  close 済み。`smoke-all` failure 時に failed step / command / return code / output excerpt を compact に追える diagnostics を actualize し、aggregate summary が失敗時にも verification entrypoint として使えるようにした。
- current recommendation:
  success 側の compact summary は維持し、failure 時だけ failing step / command / return code / output excerpt を narrow に surfacing する。
- stop line:
  exhaustive workflow automation
  aggregate CI contract
  final public CLI / tutorial surface

### Package 111 — representative problem bundle quickstart walkthrough hardening

- current reading:
  close 済み。`samples/problem-bundles/problem1|problem2` に `最短 quickstart` と `見るべき結果` を actualize し、代表サンプル導線を doc 単体でも分かりやすくした。
- current recommendation:
  current helper と sample bundle doc をずらさず、最短 4 ステップで「何を実行し、何を見ればよいか」を示す。
- stop line:
  exhaustive tutorial surface
  exhaustive sample catalog
  final public CLI / tutorial surface

### Package 112 — representative problem quickstart CLI mirror

- current reading:
  close 済み。bundle doc 側に置いた 4 段 quickstart を `scripts/current_l2_guided_samples.py quickstart problem1|problem2` からも problem ごとに表示できるようにし、sample-side quickstart と helper-side summary を一致させた。
- current recommendation:
  doc を読まなくても first 4 steps を helper から見られるようにするが、final public tutorial surface には上げない。
- stop line:
  exhaustive tutorial surface
  final public CLI / tutorial surface
  final public parser / checker / runtime API

### Package 113 — representative problem quickstart parity checks

- current reading:
  close 済み。sample bundle doc と `quickstart problem1|problem2` helper が同じ 4-step 導線を保っていることを focused test / helper で確認し、doc-side と helper-side の quickstart drift を早めに拾えるようにした。
- current recommendation:
  representative 4-step quickstart だけを対象にした narrow parity check に留め、exhaustive tutorial validation には広げない。
- stop line:
  exhaustive tutorial surface
  final public CLI / tutorial surface
  final public parser / checker / runtime API

### Package 114 — representative problem mixed-gate reopen map refresh

- current reading:
  close 済み。representative entrypoint floor を踏まえて Problem 1 / Problem 2 の mixed-gate reopen point を quickstart / bundle / matrix / smoke 現況に合わせて短く再整理し、`reopen-map` helper と sample bundle doc 側の `現在の mixed gate 再開点` section で problem-local mixed gate と global true user-spec residual を分けて読めるようにした。
- current recommendation:
  current runnable floor と remaining mixed gate の境界を sample bundle / helper / snapshot / roadmap で同じ読みへ揃え、queue drift を起こしにくくする。
- stop line:
  final public theorem/model-check/witness-provider contract
  final public CLI / tutorial surface
  final public parser / checker / runtime API

### Package 115 — Problem 1 theorem/model-check mixed-gate split refresh

- current reading:
  close 済み。reopen-map で aggregate に見える Problem 1 remaining mixed gate を、typed source principal / theorem public-contract / model-check public-contract の separate package へ split した。
- current recommendation:
  Problem 1 の mixed gate を 1 枚の reopen map で止めず、次 package が typed / theorem / model-check のどこに掛かるかを narrow に戻す。
- stop line:
  final public theorem contract
  final public model-check contract
  final public verifier contract

### Package 116 — Problem 2 order-handoff/public-shape mixed-gate split refresh

- current reading:
  close 済み。reopen-map で aggregate に見える Problem 2 remaining mixed gate を、source wording / emitted schema と witness-provider public shape の separate package へ split した。
- current recommendation:
  Problem 2 の mixed gate を 1 枚の reopen map で止めず、source wording 側と shared-space public shape 側の reopen point を narrow に戻す。
- stop line:
  final public witness/provider/artifact contract
  exhaustive shared-space catalog
  final public parser / checker / runtime API

### Package 117 — Problem 1 typed source principal split

- current reading:
  close 済み。checker-adjacent principal / structural marker first / finite decidable index fragment first を保ったまま、typed source principal residual を theorem/model-check public-contract residual から切り離して narrow に読む helper/doc cut を actualize した。
- current recommendation:
  `p06` representative と `p10 / p11 / p12 / p15 / p16` 補助 sample の役割差を保ち、typed residual だけを `split problem1 typed-source-principal` から reopen できる current cut を保持する。
- stop line:
  final typed source principal
  final typed calculus
  final public verifier contract

### Package 118 — Problem 1 theorem public-contract split

- current reading:
  close 済み。review-unit transport first / notebook-consumer object first / theorem-first pilot keep を保ったまま、theorem public-contract residual を typed / model-check residual から切り離して narrow に読む helper/doc cut を actualize した。
- current recommendation:
  theorem public-contract residual は `split problem1 theorem-public-contract` と `bundle problem1` / Lean artifact 導線を主 anchor にし、typed source principal residual や model-check residual と混ぜない。
- stop line:
  final public theorem contract
  concrete theorem prover brand
  final public verifier contract

### Package 119 — Problem 1 model-check public-contract split

- current reading:
  next active bundle。row-local property route first / checker-artifact route first / reopen-threshold helper mirror keep を保ったまま、model-check public-contract residual を typed / theorem residual から切り離して narrow にする。
- current recommendation:
  model-check residual は `matrix problem1` と `bundle problem1` の row-local carrier 導線を主 anchor にし、property/tool seam の mixed gate を theorem residual と混ぜない。
- stop line:
  first settled property language
  final public checker artifact
  final public verifier contract

### Package 120 — Problem 2 source wording / emitted schema split

- current reading:
  next active bundle。edge-row principal / stage-block secondary / serial reserve keep を保ったまま、source wording / emitted schema residual を witness-provider public-shape residual から切り離して narrow にする。
- current recommendation:
  source wording residual は parser companion / bundle problem2 / surface preview を主 anchor にし、shared-space public-shape residual と混ぜない。
- stop line:
  final source-surface handoff wording
  final emitted-artifact schema
  final public parser / checker / runtime API

### Package 121 — Problem 2 witness-provider public-shape split

- current reading:
  next active bundle。claim/payload split first / route-schema split first / representative pair keep を保ったまま、witness-provider public-shape residual を source wording residual から切り離して narrow にする。
- current recommendation:
  witness-provider residual は `matrix problem2` / `bundle problem2` / reserve lane の current split を主 anchor にし、source wording residual と混ぜない。
- stop line:
  final public witness/provider/artifact contract
  stronger fairness / replay profile
  exhaustive shared-space catalog

### Package 58 — helper / CLI hardening and broader coverage

- current reading:
  actual Lean execution reached 後の helper/CLI hardening と broader theorem-side / diagnostics / order-handoff corpus widening は close 済みである。`p09-dice-delegated-rng-provider-placement` carry-over、order-handoff CLI `surface_preview`、`p13 / p14` late-join visibility static stop、negative pair theorem-side Lean carry-over、`p10 / p11 / p12` sample-local `typed_checker_hint_preview`、theorem result-object preview helper mirror、model-check public-checker preview helper mirror まで actualize 済みと読む。
- evidence:
  toolchain probe / reopen manifest、representative sample set actual Lean execution、`samples/lean/` committed corpus、source-side IFC trio `p10 / p11 / p12`、`specs/examples/525`、`specs/examples/526`、`specs/examples/527`、`specs/examples/528`、`specs/examples/529`、`specs/examples/530`、`docs/reports/0806`、`docs/reports/0807`、`docs/reports/0808`、`docs/reports/0809`、`docs/reports/0810`、`docs/reports/0811`、`p09-dice-delegated-rng-provider-placement`、`p13-dice-late-join-missing-publication-witness`、`p14-dice-late-join-handoff-before-publication`
- stop line:
  final public theorem contract / final parser grammar / packaging には上げない

### Package 59 — near-end closeout sync

- current reading:
  close 済み。Package 58 close 後の helper-local actualization / residual gate reading を snapshot / roadmap / traceability に同期し、queue を residual mixed-gate packages へ再構成した。
- evidence:
  `specs/examples/531`
  `docs/reports/0812`
  `progress.md`
  `plan/11`
  `plan/17`
  `plan/18`
  `plan/90`

### Package 60 — theorem/model-check residual mixed-gate compression

- current reading:
  close 済み。theorem/model-check final public-contract reopen threshold を `run-source-sample` helper summary に mirror し、`p08` theorem reached / model-check guarded と `p09` theorem guarded / model-check reached の非対称を helper-local operational summary に actualize した。
- evidence:
  `specs/examples/532`
  `docs/reports/0813`
  `crates/mir-runtime/src/current_l2_cli.rs`
  `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- stop line:
  final public theorem/model-check contract adoption、final public verifier contract adoption、concrete production prover/model-check binding には上げない

### Package 61 — order-handoff/shared-space residual mixed-gate compression

- current reading:
  close 済み。order-handoff source wording residual / emitted-artifact residual / witness-provider public-seam residual を `run-source-sample` helper summary に mirror し、`p07 / p08` reached・`p09` guard の current cut を helper-local operational summary に actualize した。
- evidence:
  `specs/examples/533`
  `docs/reports/0814`
  `crates/mir-runtime/src/current_l2_cli.rs`
  `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- stop line:
  final source wording adoption、final public witness/provider/artifact contract adoption、final parser/public API adoption には上げない

### Package 62 — typed/IFC helper-to-checker ratchet

- current reading:
  close 済み。`typed_checker_hint_preview` を final typed source principal や final public verifier contract に上げず、checker-adjacent payload threshold まで narrow に ratchet し、`actual_checker_payload_family_threshold` を `payload_family_kind + source_refs` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/522`
  `specs/examples/523`
  `specs/examples/524`
  `specs/examples/529`
  `specs/examples/534`
- stop line:
  final typed source principal、final IFC syntax、final public checker artifact、final public verifier contract には上げない

### Package 63 — checker payload row-family ratchet

- current reading:
  close 済み。`actual_checker_payload_family_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload row family まで narrow に ratchet し、`actual_checker_payload_row_family_threshold` を `payload_family_ref + row_family_kind` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/265`
  `specs/examples/266`
  `specs/examples/534`
  `specs/examples/535`
- stop line:
  supported kind detail、actual checker row payload、final public checker artifact、final public verifier contract には上げない

### Package 64 — checker payload row-detail ratchet

- current reading:
  close 済み。`actual_checker_payload_row_family_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload row detail まで narrow に ratchet し、`actual_checker_payload_row_detail_threshold` を `payload_row_family_ref + row_source_ref + row_reason_kind` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/267`
  `specs/examples/268`
  `specs/examples/535`
  `specs/examples/536`
- stop line:
  actual checker row body、final public checker artifact、final public verifier contract には上げない

### Package 65 — checker payload row-body ratchet

- current reading:
  close 済み。`actual_checker_payload_row_detail_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload row body まで narrow に ratchet し、`actual_checker_payload_row_body_threshold` を `payload_row_family_ref + row_source_ref + row_reason_kind + row_body` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/269`
  `specs/examples/270`
  `specs/examples/536`
  `specs/examples/537`
- stop line:
  supported kind detail
  final public checker artifact
  final public verifier contract

### Package 66 — checker payload supported-kind-summary ratchet

- current reading:
  close 済み。`actual_checker_payload_row_body_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload supported kind summary まで narrow に ratchet し、`actual_checker_payload_supported_kind_summary_threshold` を `payload_row_family_ref + supported_kind_scope + supported_kind_refs` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/271`
  `specs/examples/272`
  `specs/examples/537`
  `specs/examples/538`
- stop line:
  public checker payload schema
  final public checker artifact
  final public verifier contract

### Package 67 — checker payload public-schema sketch ratchet

- current reading:
  close 済み。`actual_checker_payload_supported_kind_summary_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload public schema sketch まで narrow に ratchet し、`actual_checker_payload_public_schema_sketch_threshold` を 5 ref wrapper current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/273`
  `specs/examples/274`
  `specs/examples/538`
  `specs/examples/539`
- stop line:
  public checker API
  final public checker artifact
  final public verifier contract

### Package 68 — checker payload public-checker-api sketch ratchet

- current reading:
  close 済み。`actual_checker_payload_public_schema_sketch_threshold` を actual command surface や final public verifier contract に上げず、public checker API sketch まで narrow に ratchet し、`actual_public_checker_api_sketch_threshold` を `checker_subject + public_checker_payload_schema_ref` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/275`
  `specs/examples/276`
  `specs/examples/539`
  `specs/examples/540`
- stop line:
  public checker entry criteria
  actual command surface
  shared output contract
  final public verifier contract

### Package 69 — public-checker entry-criteria ratchet

- current reading:
  close 済み。`actual_public_checker_api_sketch_threshold` を actual command surface や parser-front public checker boundary に上げず、public-checker comparison 専用の entry criteria まで narrow に ratchet し、`actual_public_checker_entry_criteria_threshold` を `public_checker_api_ref + entry_criteria_refs + family_facade_support_ref + family_facade_script_refs + smoke_command_refs + next_comparison_target_ref + deferred_boundary_refs` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/277`
  `specs/examples/278`
  `specs/examples/540`
  `specs/examples/541`
- stop line:
  actual command surface
  shared output contract
  parser-front public checker boundary
  emitted verifier handoff surface
  final public verifier contract

### Package 70 — public-checker command-surface ratchet

- current reading:
  close 済み。`actual_public_checker_entry_criteria_threshold` を shared output contract や parser-front public checker boundary に上げず、public-checker command surface ready sketch まで narrow に ratchet し、`actual_public_checker_command_surface_threshold` を `command_surface_kind + family_facade_command_refs + public_checker_api_ref` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/279`
  `specs/examples/280`
  `specs/examples/541`
  `specs/examples/542`
- stop line:
  detached-loop `smoke-*` wrapper の public surface 昇格
  generic shared public checker entry
  shared output contract
  parser-front public checker boundary
  emitted verifier handoff surface
  final public verifier contract

### Package 71 — shared-output-contract ratchet

- current reading:
  close 済み。`actual_public_checker_command_surface_threshold` を parser-front public checker boundary や emitted verifier handoff surface に上げず、shared-output-contract ready sketch まで narrow に ratchet し、`actual_shared_output_contract_threshold` を `output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/281`
  `specs/examples/282`
  `specs/examples/542`
  `specs/examples/543`
- stop line:
  generic shared public checker entry
  parser-front public checker boundary
  emitted verifier handoff surface
  final public verifier contract

### Package 72 — public-checker-boundary ratchet

- current reading:
  close 済み。`actual_shared_output_contract_threshold` を final parser grammar や emitted verifier handoff surface に上げず、public-checker-boundary ready sketch まで narrow に ratchet し、`actual_public_checker_boundary_threshold` を `boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/283`
  `specs/examples/284`
  `specs/examples/543`
  `specs/examples/544`
- stop line:
  final parser grammar
  generic shared public checker entry
  emitted verifier handoff surface
  final public verifier contract

### Package 73 — verifier-handoff-surface ratchet

- current reading:
  close 済み。`actual_public_checker_boundary_threshold` を actual emitted verifier handoff artifact や theorem / protocol / runtime-policy dedicated split に上げず、verifier-handoff-surface ready sketch まで narrow に ratchet し、`actual_verifier_handoff_surface_threshold` を `handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/285`
  `specs/examples/286`
  `specs/examples/544`
  `specs/examples/545`
- stop line:
  actual emitted verifier handoff artifact
  theorem / protocol / runtime-policy dedicated contract
  final parser grammar
  final public verifier contract

### Package 74 — minimal-parser-subset-freeze ratchet

- current reading:
  close 済み。`actual_verifier_handoff_surface_threshold` を final parser grammar や parser-to-checker reconnect freeze に上げず、minimal parser subset freeze ready sketch まで narrow に ratchet し、`actual_minimal_parser_subset_freeze_threshold` を `freeze_kind + accepted_cluster_refs + reject_cluster_refs + retention_floor_refs` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/287`
  `specs/examples/288`
  `specs/examples/545`
  `specs/examples/546`
- stop line:
  final parser grammar
  parser-to-checker reconnect freeze
  final public parser/checker API
  final public verifier contract

### Package 75 — parser-to-checker-reconnect-freeze ratchet

- current reading:
  close 済み。`actual_minimal_parser_subset_freeze_threshold` を final parser grammar や final public parser/checker API に上げず、parser-to-checker reconnect freeze ready sketch まで narrow に ratchet し、`actual_parser_to_checker_reconnect_freeze_threshold` を `reconnect_kind + parser_subset_freeze_ref + checker_floor_refs + retained_helper_refs` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/289`
  `specs/examples/290`
  `specs/examples/546`
  `specs/examples/547`
- stop line:
  final parser grammar
  final public parser/checker API
  final public verifier contract

### Package 76 — phase1-semantics-closeout ratchet

- current reading:
  close 済み。`actual_parser_to_checker_reconnect_freeze_threshold` を final parser grammar や final type system に上げず、phase1 semantics closeout ready sketch まで narrow に ratchet し、`actual_phase1_semantics_closeout_threshold` を `closeout_kind + core_semantics_refs + invariant_bridge_refs + notation_status_refs` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/291`
  `specs/examples/292`
  `specs/examples/547`
  `specs/examples/548`
- stop line:
  final parser grammar
  final type system
  actual external schema
  final public verifier contract

### Package 77 — phase2-parser-free-poc-closeout ratchet

- current reading:
  close 済み。`actual_phase1_semantics_closeout_threshold` を reference update / bless workflow や public exporter API に上げず、phase2 parser-free PoC closeout ready sketch まで narrow に ratchet し、`actual_phase2_parser_free_poc_closeout_threshold` を `closeout_kind + compile_gate_refs + helper_boundary_refs + detached_loop_policy_refs` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/293`
  `specs/examples/294`
  `specs/examples/548`
  `specs/examples/549`
- stop line:
  reference update / bless workflow
  final retention/path policy
  public exporter API
  production host interface

### Package 78 — phase4-shared-space-self-driven-closeout ratchet

- current reading:
  close 済み。`actual_phase2_parser_free_poc_closeout_threshold` を final activation / authority / auth / identity / admission / consistency / fairness catalog に上げず、phase4 shared-space self-driven closeout ready sketch まで narrow に ratchet し、`actual_phase4_shared_space_self_driven_closeout_threshold` を `closeout_kind + current_package_refs + user_spec_required_catalog_refs + retained_later_refs` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/295`
  `specs/examples/296`
  `specs/examples/549`
  `specs/examples/550`
- stop line:
  final activation overlay catalog
  final authority / auth / identity / admission catalog
  final consistency / fairness catalog
  final operational realization

### Package 79 — phase5-proof-protocol-runtime-policy-handoff-closeout ratchet

- current reading:
  close 済み。`actual_phase4_shared_space_self_driven_closeout_threshold` を actual subject row materialization や boundary-specific handoff artifact family に上げず、phase5 proof / protocol / runtime-policy handoff closeout ready sketch まで narrow に ratchet し、`actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold` を `closeout_kind + verifier_handoff_surface_ref + theorem_retained_bridge_stop_ref + boundary_inventory_ref + retained_later_refs` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/297`
  `specs/examples/298`
  `specs/examples/550`
  `specs/examples/551`
- stop line:
  actual subject row materialization
  boundary-specific handoff artifact family
  actual emitted verifier artifact
  concrete theorem / model-check tool binding

### Package 80 — phase6-actual-parser-ast-carrier-first-tranche ratchet

- current reading:
  close 済み。`actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold` を stage3 admit/request/predicate cluster や final parser grammar に上げず、`carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs` current cut の phase6 actual parser / AST carrier first tranche ready sketch を `mir-ast` manifest と `run-source-sample` helper summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/299`
  `specs/examples/300`
  `specs/examples/552`
- stop line:
  stage3 admit slot surface
  stage3 request clause suite
  stage3 predicate fragment
  perform head final public parser API
  span-rich diagnostics
  final grammar

### Package 81 — phase6-actual-checker-runtime-skeleton-first-tranche ratchet

- current reading:
  close 済み。`actual_phase6_actual_parser_ast_carrier_first_tranche_threshold` の次段として、`skeleton_kind + semantic_entry_refs + runtime_bridge_refs + parser_bridge_contract_refs + retained_later_refs` current cut の phase6 actual checker/runtime first tranche ready sketch を `mir-runtime` manifest と `run-source-sample` helper summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/301`
  `specs/examples/302`
  `specs/examples/553`
- stop line:
  parser_to_program_lowering
  stage3_request_predicate_reconnect
  richer_host_interface
  final_public_runtime_checker_api
  formal_hook_concrete_tool_binding

### Package 82 — phase6-compile-ready-verification-and-formal-hook ratchet

- current reading:
  close 済み。`actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold` の次段として、selected cargo / smoke gate と tool-neutral formal hook shape を current cut に留めた phase6 compile-ready verification / formal-hook ready sketch を helper-local summary と code anchor に actualize 済みと読む。
- evidence anchor:
  `specs/examples/303`
  `specs/examples/304`
  `specs/examples/554`
- stop line:
  concrete_theorem_tool_binding
  concrete_model_check_tool_binding
  parser_second_tranche_widen
  final_public_surface
  final_public_verifier_contract

### Package 83 — phase6-next-reopen-sequencing ratchet

- current reading:
  close 済み。`actual_phase6_compile_ready_verification_and_formal_hook_threshold` の次段として、parser second tranche first / theorem-first reserve / model-check second reserve の sequencing minimum を helper-local threshold に留めた phase6 next-reopen line を actualize 済みと読む。
- evidence anchor:
  `specs/examples/305`
  `specs/examples/306`
  `specs/examples/555`
- stop line:
  request_clause_suite_bulk_widen
  perform_head_final_public_api
  concrete_theorem_tool_binding
  concrete_model_check_tool_binding
  final_public_surface

### Package 84 — phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package ratchet

- current reading:
  close 済み。`actual_phase6_next_reopen_sequencing_threshold` の次段として、stage3 declaration-side admit attached slot と shared isolated predicate fragment の first package minimum を `CurrentL2SecondTrancheManifest` と stage3 spike regression 群へ narrow に actualize 済みと読む。
- evidence anchor:
  `specs/examples/307`
  `specs/examples/308`
  `specs/examples/556`
- stop line:
  shared_single_attachment_frame
  request_clause_suite_publicization
  perform_head_final_public_api
  span_rich_diagnostics
  final_grammar
  theorem_model_check_concrete_binding

### Package 85 — phase6-reserve-formal-tool-binding-inventory ratchet

- current reading:
  close 済み。`CurrentL2SecondTrancheManifest` の次段として、theorem-first reserve / model-check second reserve / tool-neutral formal-hook keep / parser-side mainline keep を helper-local manifest と snapshot docs へ narrow に actualize 済みと読む。
- evidence anchor:
  `specs/examples/309`
  `specs/examples/310`
  `specs/examples/556`
  `specs/examples/558`
- stop line:
  concrete_theorem_tool_name
  concrete_model_check_tool_name
  actual_ci_artifact_retention_policy
  parser_side_followup_package_selection
  final_public_parser_checker_runtime_surface

### Package 86 — phase6-parser-side-follow-up-package-sequencing ratchet

- current reading:
  close 済み。Package 85 close 後の次段として、shared single attachment frame を next parser-side package に固定し、request clause suite / perform head / source-sample path を deferred reopen として helper-local manifest と snapshot docs に narrow に actualize 済みと読む。
- evidence anchor:
  `specs/examples/311`
  `specs/examples/312`
  `specs/examples/559`
  `docs/reports/0841`
- stop line:
  shared single attachment frame actual code shape
  request clause suite publicization
  perform head final public parser API
  source-sample corpus scope / file layout
  final public parser / checker / runtime surface

### Package 87 — phase6-parser-second-tranche-shared-single-attachment-frame-first-package ratchet

- current reading:
  close 済み。shared single attachment frame の multiline extraction bridge minimum を `CurrentL2SharedSingleAttachmentFrameManifest` と stage3 multiline attachment spike 群へ actualize し、request clause suite / perform head / source-sample path は retained-later に残す。
- evidence anchor:
  `specs/examples/313`
  `specs/examples/314`
  `specs/examples/560`
  `docs/reports/0842`
- stop line:
  request clause suite publicization
  perform head final public parser API
  source-sample corpus scope / file layout
  final public parser / checker / runtime surface

### Package 88 — fixed-subset-source-sample-corpus-scope-and-file-layout ratchet

- current reading:
  close 済み。repo-root `samples/current-l2/` flat `.txt` layer の scope / directory / naming / non-goal minimum を `CurrentL2FixedSubsetSourceSampleCorpusScopeAndFileLayoutManifest` と source-corpus policy docs に actualize し、representative / fixture / source の第 3 層 minimum を source-backed に同期した。
- evidence anchor:
  `specs/examples/315`
  `specs/examples/316`
  `specs/examples/561`
- stop line:
  representative / fixture / source mapping matrix
  actual sample file content
  parser-to-`Program` lowering
  bless / regression policy

### Package 89 — phase6-request-clause-suite publicization comparison

- current reading:
  close 済み。shared single attachment frame と source-corpus scope/layout minimum を保ったまま、request-local `require` / `ensure` fixed two-slot suite bridge を `CurrentL2RequestClauseSuiteManifest`、`Stage3RequestClauseSuite`、`parse_stage3_request_clause_suite_text()` に actualize し、crate-local non-production parser carrier として inspectable にした。
- stop line:
  perform head final public parser API
  span-rich diagnostics
  final grammar
  final public parser / checker / runtime surface

### Package 90 — phase6-perform-head structural carrier actualization

- current reading:
  close 済み。request clause suite bridge を current entry criteria に保ったまま、`perform` head の owner / op / target-or-via shape を `CurrentL2PerformHeadManifest`、`Stage3PerformTargetRef`、`Stage3PerformHead`、`parse_stage3_perform_head_text()` に actualize し、crate-local non-production parser carrier として inspectable にした。
- stop line:
  request clause suite bundle attachment
  span-rich diagnostics
  final grammar
  final public parser / checker / runtime surface

## research-discovery items

| item | 何に影響するか | current recommendation |
|---|---|---|
| stronger typed-surface actual adoption | typed source principal | full dependent core は first public core に入れず、finite decidable index fragment + IFC / taint + capture / lifetime + simple cost を principal target に維持し、evidence pressure が出るまで experimental adoption を待つ |
| final modal foundation / final source marker | syntax / modality / proof spine | partial basis + stronger family keep を維持し、final adoption は mixed gate に残す |
| authoritative-room `serial` sugar admissibility | order-handoff source-facing reserve surface | room-specific reserve に留め、principal surface には上げないまま helper-local evidence を集める |
| formal skeleton artifact shape beyond first slice | proof plan / Rust-Lean alignment | public proof artifact contract へ上げず、mechanization-ready internal floor に留める |

## remaining mixed gates

| topic | impact | current recommendation |
|---|---|---|
| final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema / final public verifier contract | theorem-first pilot | review-unit transport first、notebook-consumer object first、Lean-stub bridge current floor と representative Lean sample set actual Lean execution floor を維持し、final public theorem contract 群には上げない |
| first settled property language / concrete model-check tool brand / final public checker artifact / actual public checker migration / actual emitted verifier handoff artifact / production checker-runtime-policy contract / final public verifier contract | model-check line | row-local property route first、checker-artifact route first、reopen-threshold helper mirror keep、public-seam compression keep を維持する |
| final source-surface handoff wording / final emitted-artifact schema | order-handoff public surface | edge-row principal、stage-block secondary keep、thread/node same causal language keep、serial sugar reserve を維持する |
| final public witness schema / provider receipt schema / combined public contract / emitted-handoff contract | shared-space stronger public shape | claim/payload split first、route/schema route actual adoption、trace-alignment reserve を維持し、final public contract 群には上げない |
| stronger typed-surface actual adoption | typed source principal | `Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ C` conceptual spine、finite decidable index fragment、IFC / taint、capture / lifetime、simple cost を先に actualize し、full dependent core と final public type syntax は still later に残す |
| final modal foundation / final source marker | syntax / modality | partial basis + stronger family keep を維持する |
| final parser grammar / final public parser-checker-runtime API | public surface | this line では凍らせない |

## true user-spec residuals

| item | impact | current recommendation |
|---|---|---|
| shared-space exhaustive final catalog beyond minimal working subset | shared-space / room-profile final shape | minimal working subset default を保持し、exhaustive catalog は user-spec residual に残す |
| installed-binary / packaging / FFI / engine adapter / host integration target | backend / distribution / external embedding | repo-local runnable CLI + tests + emitted artifacts + reproducible compare floor を near-end success とし、distribution / embedding target は later に残す |
| upper-layer application target beyond authoritative-room first scenario | broader app realization | authoritative turn-based room first を保持し、それ beyond は user goal に応じて reopen する |

## next reopen order

1. Package 119 で Problem 1 model-check public-contract residual を separate package として narrow にする。
2. Package 120 / 121 で Problem 2 source wording residual と witness-provider public-shape residual を separate package として narrow にする。
3. residual public-seam maintenance と later mixed/user-spec residual を Package 119...121 close 後に再同期する。
