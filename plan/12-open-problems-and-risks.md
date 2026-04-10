# plan/12 — 未解決問題とリスク

## 目的

この文書は、current repo の未解決問題と risk を 1 箇所で管理する。
未決のものは未決と書き、将来 workstream と current L2 settled judgment を混ぜない。

## risk register

| 項目 | 種別 | current 状態 | リスク | current 対応 |
|---|---|---|---|---|
| fallback intuition drift | semantics / notation | 継続中 | outer-longer-lifetime wrapper と誤読しやすい | prose、fixture、notation 比較で drift を明示 |
| notation の outer/inner 誘発 | notation | 継続中 | nested 直感が chain semantics を上書きする | explicit edge-row form を維持 |
| final parser grammar 未固定 | syntax | OPEN | syntax を早く決めすぎると semantics を拘束する | companion notation に留める |
| review infrastructure の返答遅延 | process | 継続中 | task close が reviewer 依存で滞る | 最後に 1 回長く待ち、必要なら retry 1 回、なお無理なら local evidence を report に残す |
| machine-readable catalog / manifest を今は入れないこと | architecture | current 方針 | hard-coded catalog と externalization 比較が再燃しやすい | current L2 では comparison 止まりと明記 |
| path canonicalization | helper / packaging | OPEN | selector / single-fixture / sidecar discovery の長期互換性に影響 | current L2 では minimal behavior のみ保持 |
| detached trace / audit serialization | runtime / tooling | OPEN / docs-only minimal boundary あり | trace / audit を helper 内表現に閉じ込めたままでは repo 外保存・再比較・後解析が重い | exact compare core / detached non-core / human-facing explanation を分けた docs-only minimal shape を先に切る |
| fixture authoring / elaboration bottleneck | authoring / workflow | 継続中 | new fixture ごとに AST / sidecar / expectation / profile 影響確認の人手コストが重い | hand-written fixture を正本に保ったまま、boilerplate だけを non-production scaffold helper へ切り出す |
| `expected_static.reasons` の二重用途 | checker / fixture schema | OPEN / narrow migration available | explanatory note と machine-check 候補が同じ field に混在しているため、そのまま harness machine-check core に昇格させると current fixture corpus と衝突する | current L2 では `verdict` だけを harness core に残し、actual `reasons` compare は detached static gate artifact に留める。future checker API への narrow migration として optional `checked_reasons` を additive に導入してよい |
| richer host interface | runtime boundary | OPEN / comparison 上の後続候補 | current host harness を production host に誤昇格しやすく、coverage analysis を先に肥大化させやすい | helper と production host を分離して記述し、detached artifact 境界の後で narrow に切る |
| constrained continuation / multi-shot | semantics / runtime boundary | OPEN / FUTURE | unrestricted multi-shot が linear resource、rollback frontier、lifetime crossing を壊しやすい | coroutine semantics を Mir-0 の外に残し、one-shot / multi-shot / capture restriction を将来 workstream で明示する |
| dynamic membership / causal metadata | shared space / fabric | OPEN / FUTURE | participant churn を plain vector clock deletion だけで扱うと membership change と causal history が混線しやすい。participant 集合を plain array の source of truth にすると、identity / activation / incarnation / ordering が混ざりやすい | shared-space / Mirrorea workstream 側で、membership reconfiguration と causal metadata を分離し、session-scoped membership registry + derived snapshot view を第一候補として比較する。activation rule は authoritative room では `authority-ack` を最小候補に置くが、final profile はまだ固定せず、full-coverage-like / quorum-like は overlay 可能な policy option として残す。compile-time には activation visibility の over-approximation だけを残し、actual activation / reconciliation は runtime control-plane に残す。causal carrier は plain vector deletion を避け、epoch / incarnation split を first practical candidate、control-plane separated carrier を next stronger candidate として比較する。current threshold judgment は `specs/examples/125-shared-space-control-plane-carrier-threshold.md` に整理済みであり、authority handoff / provider binding / activation frontier を room rule 側へ上げない限り current default に reopen しない。reopen する場合の first cut は full control-plane log ではなく `control_epoch` 相当の lightweight split に留める。 |
| shared-space identity / auth layering | shared space / fabric | OPEN / FUTURE | principal identity、transport auth、service login、room permission、display identity を membership carrier に潰すと、activation / authority / fairness / audit と同じ carrier に漏れやすい | current first practical candidate は identity core を membership registry に残し、auth stack / admission policy を別 carrier に置く cut である。raw auth protocol は room semantics に持ち込まず、principal continuity と room-local membership / capability の bridge だけを shared-space line に残す |
| shared-space admission policy / compile-time visibility | shared space / fabric | OPEN / FUTURE | actual principal set や activation 成立条件まで compile-time に上げると churn / reconnect / authority handoff に弱くなり、逆に runtime-only に寄せすぎると room capability / notify requirement の static floor が弱くなる | current first practical candidate は role / capability / visibility requirement の over-approximation を compile-time に残し、actual admission / activation / reconciliation は runtime control-plane に残す cut である |
| shared-space authority / resource ownership | shared space / fabric | OPEN / FUTURE | participant carrier、resource owner、delegated capability、consistency mode、fairness source を同じ carrier に潰すと、authoritative room と append-friendly room で必要な invariants が混線しやすい | shared-space workstream 側で、resource owner slot と delegated capability を分け、authoritative room / append-friendly room / relaxed room の相性を docs-first に比較する。authority placement は current phase では `single room authority` を room-level authoritative owner slot / write authority slot の最小候補として置き、`replicated authority` と `relaxed projection authority` は comparison option に残す。これにより、read-mostly resource、fan-out state、delegated capability を later option として残しつつ、人間 participant への単純還元を避ける。consistency mode は `authoritative serial transition` と `append-friendly room` を current working subset として置くが、これは final catalog の固定ではなく、`relaxed merge-friendly room` を future comparison に残したまま表現力 / proof burden の比較土台とする。RNG / fairness source は `authority_rng` を最小候補にし、`delegated_rng_service` は authoritative room 側でも provider-placement candidate として practical に読める current cut まで進めた。ここでは authority が request / lock / commit / publish の owner のまま、provider は draw result だけを返し、provider receipt / draw ref は audit / receipt side optional attachment に留める。`distributed_randomness_provider` は default にせず future comparison に残す。authoritative room baseline 自体は `specs/examples/121-shared-space-authoritative-room-baseline.md` に `authority-ack` + `single room authority` + `authoritative serial transition` + `authority_rng` として集約済みであり、その次段の small cross-room working subset は `specs/examples/122-shared-space-catalog-working-subset-comparison.md` に authoritative / append-friendly の row と stop line として整理済みである。ただしこれは current working subset であって final catalog 固定ではない。fairness trust model は `opaque authority trust` を current minimal candidate、`auditable authority witness` を next narrow strengthening candidate に置き、minimal witness core は `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md` に `witness_kind + action_ref + draw_slot + draw_result` として整理済みであり、provider practical cut は `specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md` に切り出し済みである。provider placement と witness requirement は引き続き別軸で比較する。reconnect / late leave / in-flight action は room profile に全部入れず、`member_incarnation` と uncommitted action invalidation だけを minimal room-profile rule とし、timeout / retry / resend は external policy layer に残す |
| async control / memory-model boundary | semantics / shared space / proof boundary | OPEN / FUTURE | `atomic_cut` のような local cut だけで上位の ordering / fairness / scheduler semantics を背負わせると意味が肥大化しやすく、逆に C++ 的 low-level memory-order 語彙を早く入れると language core / scheduler / proof burden が一気に膨らむ | current phase では `atomic_cut` を place-local finalizing cut に留め、higher-level async-control は event-tree / authority-serial transition / witness-aware commit / room policy family として Phase 4 / 5 で docs-first に比較する。low-level memory-order vocabulary は immediate candidate にせず、どの局所性までを decidable core に入れ、どこから先を theorem prover / model checker / runtime policy に残すかを inventory する |
| rollback restore scope / checker boundary | semantics / checker boundary | OPEN / current runtime reading あり | `AtomicCut` frontier update と restore scope を checker floor に混ぜると、current whole-store snapshot restore と place-local explanation がずれやすい | `TryFallback` / `AtomicCut` の structural floor は checker 候補に残しつつ、`place_anchor == current_place` gate と restore scope は runtime / proof boundary に残す |
| portability / observability hooks | implementation / tooling boundary | OPEN / FUTURE | CPU 固定や非切替デバッグ実装を早く焼き付けると、後で HW 拡張や graph 可視化 / step 実行の導入で手戻りが大きい | semantics core には入れず、detached artifact / step execution / graph export hook を replaceable layer として残す |
| multi-request scheduler | runtime | FUTURE | current direct-style interpreter と概念が混ざる | 現時点では未着手を明示 |
| `Approximate` / `Compensate` | semantics / runtime | FUTURE | failure space と rollback を広く再設計する必要がある | 今は plan に残すだけ |

## 各項目の補足

### fallback intuition drift

- current L2 では fallback は guarded option chain である
- しかし見た目や用語から outer wrapper 読みが起きやすい
- これは semantics を変えるより、prose / examples / regression fixture で drift を抑える方針を採っている

### notation の outer/inner 誘発

- line-leading `>` ladder は compact だが、operator / nested wrapper / expression 風の誤読を誘発しやすい
- explicit edge-row form はやや重いが、current semantics を誤読させにくい

### final parser grammar 未固定

- current companion notation は parser-ready final syntax ではない
- final punctuation、keyword、indentation discipline、serialization contract は未決である

### machine-readable catalog / manifest を今は入れない理由

- aliases がまだ 4 個規模であり、hard-coded table が最小
- asset 化すると loader / placement / validation / path policy が先に膨らむ
- current L2 では利点より固定圧の方が大きい

### path canonicalization

- `single-fixture` selector と sidecar discovery は現状の repo layout に依存している
- 長期固定するには canonical path policy が要るが、current L2 ではまだ決めない

### detached trace / audit serialization

- 現在は helper と tests の都合で in-memory compare が中心である
- `TraceAuditSink` / `RunReport` / bundle summary から detached artifact をどう切り出すかは未決である
- ただし detached trace / audit と richer host interface の 2 項目を比べると、前者の方が current parser-free PoC を「大量に回して比較する」段階へ進めやすい
- さらに exporter entry layer の比較では、
  - payload core は `RunReport` に最も近いが、
  - first exporter entry は `run_bundle` / `BundleRunReport` に置く方が helper boundary を壊しにくい
  - `BatchRunSummary` は後段 aggregate export に回す
  という narrow judgment を current understanding とする
- bundle-first payload/context split では、
  - `fixture_id` / `fixture_path` / `host_plan_path` / `runtime_requirement` は `bundle_context`
  - `steps_executed` は detached non-core
  - `host_plan_coverage_failure` は aggregate-only
  と切るのが current helper boundary に最も自然である
- docs-only minimal boundary としては、
  - exact-compare core
    - `static_verdict`
    - `entered_evaluation`
    - `terminal_outcome`
    - `event_kinds`
    - formal `non_admissible_metadata`
    - short `narrative_explanations`
    - batch-layer の `host_plan_coverage_failure`
  - detached non-core
    - `steps_executed`
    - coverage explanation
    - host-plan explanation
    - auxiliary counters / summaries
  - human-facing explanation
    - `must_explain`
    - long-form audit
  に分けるのが最小である
- production schema version、保存パス規約、typed coverage carrier は引き続き未決である
- actual exporter API と aggregate export の順序も引き続き未決である
- bundle-level failure artifact の actual exporter 導入時期と API は引き続き未決である
- ただし current comparison としては、`host_plan_coverage_failure` を将来 typed carrier に昇格させるなら aggregate-only を維持したまま bundle failure artifact 側に切るのが最も自然であり、payload core / `bundle_context` / detached non-core に薄く混ぜるのは避ける
- さらに docs-only schema refinement としては、bundle failure artifact 側の typed core は `failure_kind` discriminator だけに留めるのが最小であり、`bundle_context` 参照や short note は後段に回す
- aggregate export がその typed bundle failure を吸うとしても、current L2 の最小は `failure_kind` ごとの histogram / kind count までであり、bundle failure summary の再掲は避ける
- さらに docs-only refinement としては、aggregate typed field 名の最小候補は `bundle_failure_kind_counts` であり、current `host_plan_coverage_failures` list と `BatchBundleOutcome::Failed.host_plan_coverage_failure` bool を compatibility anchor として残した additive coexistence が最小 migration cut である
- current actual sketch では、`bundle_failure_kind_counts` を migrated kind only の partial histogram として扱い、`bundle_failure_kind_counts_scope = "migrated-kinds-only"` を併せて持たせて full histogram と誤読させない
- detached exporter consolidation sprint の current state としては、
  - `specs/examples/23-current-l2-detached-export-loop-consolidation.md` が docs-only judgment の集約文書になり、
  - `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs` が bundle-first の non-production emitter sketch、
  - `scripts/current_l2_diff_detached_artifacts.py` が payload core comparison helper、
  - `plan/15-current-l2-fixture-authoring-template.md` が fixture authoring / elaboration の実務テンプレート
  である
- detached validation-loop continuation の current state としては、
  - `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md` が aggregate export 接続と storage/path policy の docs-only 集約文書になり、
  - `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md` が aggregate emitter sketch の actual narrow cut を与え、
  - `specs/examples/37-current-l2-detached-bundle-transform-helper.md` が bundle emitter 内 private transform を shared support module へ落とす repo 内 callable boundary を与え、
  - `specs/examples/31-current-l2-detached-aggregate-transform-helper.md` が aggregate emitter 内 private transform を shared support module へ落とす repo 内 callable boundary を与え、
  - `specs/examples/32-current-l2-static-gate-artifact-loop.md` が static gate verdict / reasons を detached validation loop に接続する static gate artifact helper cut を与え、
  - `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md` が 1 fixture loop を 1 command で回す helper 境界を与え、
  - `scripts/current_l2_detached_loop.py` が bundle-first emitter、aggregate emitter、diff helper を束ねる non-production wrapper であり、
  - current non-production default candidate は `target/current-l2-detached/` である
- ただしこれは production exporter 完了を意味しない
- actual exporter API、artifact 保存先と path policy、aggregate typed field の actual implementation timing は引き続き OPEN である
- また current actualization は bundle / aggregate / static-gate の shared support module までであり、`lib.rs` / `harness.rs` の public exporter API finalization は依然として OPEN である
- static gate artifact helper も同様に `examples/support/` と script helper に留まり、future checker API finalization は依然として OPEN である
- `expected_static.reasons` は current fixture corpus では human-facing explanation と static machine-check 候補が混在しており、そのまま `run_bundle()` へ昇格させると valid fixture 群と衝突する
- したがって current actual machine-check は `expected_static.verdict` に留め、actual `reasons` compare は detached static gate artifact 側へ残す
- future checker API で static reason compare を core に上げたいなら、まず additive optional `checked_reasons` を dedicated carrier として別立てにするのが最小である
- その次段では、`checked_reasons` を長期維持するか、typed reason code へ進めるかが OPEN である
- current helper cut では、detached static gate artifact 側に `detached_noncore.reason_codes` を置いてよい
- ただしこれは `checker_core.reasons` の first-class typed replacement ではなく、stable cluster だけを best-effort で mirror する helper-local / reference-only carrier に留める
- current actualization では、first typed family は lineage edge pair family、second tranche は declared target edge pair family、その後 missing option family と capability family まで current stable cluster inventory を `expected_static.checked_reason_codes` に揃えた
- また first-class carrier placement は detached-side mirror 昇格ではなく、fixture-side additive optional `expected_static.checked_reason_codes` を stable cluster 8 kind に対して採るのが current cut である
- current corpus では stable cluster 8 kind を覆う 9 fixture の coexistence は `checked_reasons` / `checked_reason_codes` / actual suggestion の 3 者で揃っているが、これは immediate shrink を意味しない
- current corpus では、first checker cut 候補 cluster のうち same-lineage floor / capability floor / missing-option structure floor について最低限の regression baseline が見え始めている
- same-lineage floor については helper-local / non-production checker spike が入り、current sequence は same-lineage -> missing-option -> capability まで actualize 済みである
- missing-option structure floor についても helper-local / non-production second spike が入った
- capability strengthening floor についても `e13` / `e20` を根拠に helper-local / non-production third spike が入った
- 一方で `TryFallback` / `AtomicCut` の structural floor は、current representative runtime evidence と docs-only judgment は揃っているが、same reason-row family へ直ちに fourth spike actualization するのは premature である
- current cut では、`try` / rollback locality は docs/runtime representative に留め、parser boundary と first checker API cut がもう一段見えた時点で dedicated AST structural helper の要否を比較する
- current docs-only refinement では、future dedicated AST structural helper を切るなら、parser/loader malformed source の explicit 化、AST-only floor への限定、reason-row family と別 carrier、runtime gate / restore scope を non-goal にすること、複数の structural family を持つこと、を entry criteria とする
- current docs-only refinement では、`TryFallback` / `AtomicCut` の structural malformed source は parser-free current phase では static gate / dedicated AST structural helper 側へ置き、loader は carrier/schema malformed に留める
- current docs-only refinement では、runtime representative `E2` / `E21` / `E22` が structural floor / dynamic gate split の evidence として十分強いため、malformed static family はまだ actual corpus に増やさない
- current docs-only refinement では、future dedicated AST structural helper の compare contract は current phase では helper-local dedicated contract から始め、row-family 流用や detached artifact shared carrier 先行はしない
- current docs-only refinement では、future dedicated AST structural helper の optional expected field 名は `expected_static.checked_try_rollback_structural_findings` が最小候補であり、focused compare shape も `subject_kind` / `finding_kind` の helper-local row list に留める
- current docs-only refinement では、future dedicated AST structural helper を detached validation loop へ差し込むなら、bundle-first runtime path ではなく static gate artifact loop の helper-local smoke family に留める
- current docs-only refinement では、future dedicated AST structural helper の structural verdict は `expected_static.verdict` を流用せず、`expected_static.checked_try_rollback_structural_verdict` と helper-local string enum `no_findings` / `findings_present` に留める
- current docs-only refinement では、future dedicated AST structural helper を detached artifact shared carrier へ上げる閾値は、helper actualization、fixture-side field actualization、static corpus、loop stabilization、saved artifact compare need の 5 条件が揃った時点に置くのが最小であり、current state ではまだ未充足である
- current docs-only refinement では、future dedicated AST structural helper の wrapper family は family-specific に留め、exact subcommand 名は actual helper actualization task まで deferred にする
- current docs-only refinement では、future dedicated AST structural helper を generic structural checker family と合流させるのは later public checker API comparison と同時に扱い、actual helper actualization や wrapper naming cut と混ぜない
- current docs-only refinement では、later public checker API comparison に future dedicated AST structural helper family を載せるには、generic family 合流とは別に、actual helper / fixture contract / corpus / loop stabilization / public comparison pressure の entry criteria が要る
- current docs-only refinement では、future dedicated AST structural helper の malformed static family は current phase の今すぐではなく、dedicated helper actualization first tranche と同時に actual corpus へ足すのが自然である
- current docs-only refinement では、future dedicated AST structural helper の first tranche は helper code / fixture-side fields / minimal malformed static family / static gate smoke path を一体で切り、shared carrier / public checker API は外に残すのが自然である
- current docs-only refinement では、future dedicated AST structural helper の minimal malformed static family tranche は `TryFallback` 1 件 + `AtomicCut` 1 件の two-fixture pair を最小とするのが自然である
- current actualization では、この first tranche が helper-local dedicated family として actualize 済みであり、
  - `checked_try_rollback_structural_verdict`
  - `checked_try_rollback_structural_findings`
  - `e23`
  - `e24`
  - `smoke-try-rollback-structural-checker`
  まで current repo に入っている
- current comparison としては、second malformed static tranche の問いは先に閉じるが、actual tranche 追加はまだ行わないのが自然である。理由は、current source だけでは decode-valid な concrete second-tranche family がまだ不足しており、loader malformed と runtime-valid representative を除くと requirement invent に寄りやすいからである
- current comparison としては、first-tranche wording / finding family は少なくとも shared carrier threshold の再比較までは exact working set に留めるのが自然であり、helper-local `finding_kind` の rename や alias / synonym 導入は hidden acceptance と premature genericization の risk を増やす
- current recheck では、saved artifact compare need の narrow version は helper-local checker が `fixture_path` + saved `artifact_path` を直接読むことで満たせるため、shared detached carrier threshold はまだ未充足と読むのが自然である
- ただし shared detached carrier、generic structural checker family、public checker API にはまだ上げない
- current recheck では、generic structural checker family / public checker API comparison に進む concrete pressure もまだ source-backed でないため、current try/rollback checker line は pause とみなし、second concrete family / shared output contract / verifier handoff pressure のいずれかが見えるまで reopen しないのが自然である
- current remaining risk は、later generic structural checker family comparison で `disallowed_fallback_placement` をどう generalize するか、second concrete family や shared/public pressure を later にどう inventory 化するか、である
- current cut では、3 spike に shared support helper を導入して duplicated core contract だけを束ね、family facade script と detached loop wrapper command 名は維持してよい
- current docs-only judgment では、generic checker-side shared family compare entry はまだ切らず、family facade 維持で止める
- その next OPEN は、generic entry の比較を later public checker API comparison と同時に行うべきか、それともその前段で別 helper として試すべきかである
- typed reason code に進むとしても、duplicate reason のように helper 内部構造へ近い cluster は急いで code 化しない
- ただし `checked_reasons` と `checked_reason_codes` をいつ統合・縮退させるか、stable cluster と duplicate cluster の境界を later verifier / parser workstream でどう扱うかは引き続き OPEN であり、current judgment は additive coexistence 維持である
- ただし capability floor の corpus coverage は `2` まで厚くなったとはいえ、same-lineage / missing-option よりは依然として薄いので、public checker API cut の前に helper-local evidence をもう少し積む余地がある
- actual checker spike を public helper / API に昇格させるタイミングも引き続き OPEN であり、current same-lineage helper は detached validation loop の non-production convenience に留める
- current missing-option helper も同様に detached validation loop の non-production convenience に留める
- current capability helper も同様に detached validation loop の non-production convenience に留める
- ただし current list / bool shape をいつ置き換えるか、actual exporter API をどこで切るか、aggregate row を object map にするか array row にするかは引き続き OPEN である
- compare input discovery を explicit path 主体のまま保つか、run label / fixture stem からの convenience discovery をどこまで formalize するかも引き続き OPEN である
- `smoke-fixture` のような fixture-level convenience を入れても、それを production CLI や final retention policy と誤読しない boundary discipline が引き続き必要である
- current non-production aggregate emitter sketch と aggregate compare helper は入ってよいが、`run_directory` / `BatchRunSummary` の public behavior を置き換えず、final API finalization は後段に残す

### fixture authoring / elaboration bottleneck

- current repo では fixture authoring 自体はまだ hand-written を正本に保つ。
- ただし boilerplate 作成の反復は bottleneck なので、
  - `target/current-l2-fixture-scaffolds/` を default candidate にし、
  - runtime / static-only の skeleton と empty `.host-plan.json` sidecar だけを出す
  - non-production scaffold helper を置いてよい。
- ここで helper が semantics inference、expected outcome completion、profile 自動更新まで行うと hidden elaboration になりやすいので、それらは authoring / review task 側へ残す。

### parser boundary / first cut inventory

- current L2 では final grammar を先に決めず、first parser cut に入れてよい semantic cluster を narrow に棚卸しする方が自然である。
- `place` / `try-fallback` / `perform on` / `perform via` / statement-local `require` / `ensure` / option declaration core / option-local `admit` / explicit edge-row family は first parser cut 候補になりうる。
- ただし A2 と A1 の exact lexical choice、`contract` block sugar、richer predicate grammar、option-local outcome metadata は引き続き OPEN に残す。
- ただし actual parser spike を切る順序は別問題であり、current next narrow step では monolithic actualization を避け、
  1. chain / declaration structural floor
  2. `try` / rollback structural floor
  3. request / admissibility cluster
  の checker-led staged spike として進める方が自然である。
- さらに current docs-only refinement として、stage 1 の chain / declaration structural floor に含める declaration-side guard slot は predicate fragment parser の入口としてではなく、option declaration に attached した opaque slot として扱うのが最小である。
- したがって stage 1 では、slot の existence / attachment / boundary だけを parse 対象に残し、guard fragment の parse / well-formedness は stage 3 以降へ残す。
- さらに current docs-only refinement として、stage 1 の actual parser / checker handoff では parser-side opaque slot carrier と current parser-free AST fixture schema を直結させず、`OptionDecl.lease` への thin lowering bridge を compatibility anchor として使うのが最小である。
- さらに current docs-only refinement として、その parser-side opaque slot carrier 名は `decl_guard_slot` を第一候補にし、bridge は slot-only helper ではなく option declaration 単位の structural transfer として narrow に切るのが最小である。
- さらに current docs-only refinement として、stage 1 actual parser spike の smoke family は `e4-malformed-lineage` と `e7-write-fallback-after-expiry` の two-fixture pair を最小 working set にし、`e3-option-admit-chain` は request / admissibility cluster の later-stage contrast reference に残すのが自然である。
- さらに current docs-only refinement として、actual stage 1 parser spike は `crates/mir-ast/tests/support/` 配置の private helper として始め、compare surface は parser-side raw AST ではなく lowered fixture-subset compare に留めるのが最小である。
- current actualization 後の remaining risk は、first tranche helper が通ったことを理由に public parser API や final grammar を早く凍らせてしまうことである。current helper はあくまで private / test-only / non-production であり、`e4` / `e7` の structural floor を超える accepted cluster は still later stage に残す。
- malformed-source smoke を actualize した後も、helper-local wording fragment 2 件をそのまま public diagnostics surface や typed parser error carrier に昇格させないことが重要である。current wording は exact working set の smoke anchor であり、public contract ではない。
- stage 3 request / admissibility cluster では、declaration-side `admit` attached slot と `PerformVia` / request-local clause を一気に同時 actualize すると lexical freeze pressure が急に上がる。`e3` を full-program parse へ送る前に、stage 3 の最初の sub-cutとして attached-slot extension を切り、request cluster の残りと分け続けることが重要である。
- さらに current docs-only refinement として、stage 3 first tranche の parser-side carrier 名は `decl_admit_slot` を第一候補にし、fixture-side `OptionDecl.admit` predicate node へ direct lower しないのが最小である。current first tranche の compare surface は lowered fixture-subset compare と `decl_admit_slot.surface_text` retention smoke に分け、helper-local predicate canonicalization を導入しない。
- current actualization 後の remaining risk は、stage 3 first tranche が通ったことを理由に fixture-side `OptionDecl.admit` node compare、`PerformVia`、request-local `require` / `ensure` を同じ helper へ早く混ぜてしまうことである。current helper はあくまで private / test-only / non-production であり、request cluster と predicate parse は still later stage に残す。
- さらに current actualization として、helper-local malformed-source first tranche は declaration-side `admit` payload 欠落と `PerformVia` spillover の 2 件に留め、request-local clause malformed まではまだ拡げないのが最小である。これにより stage 3 helper の accepted cluster boundary と later-stage request boundary を fail-closed に示しつつ、request attachment rule や predicate parse を hidden に持ち込まない。
- その次段 sequencing としては、request-local clause malformed より先に fixture-side `OptionDecl.admit` handoff comparison を扱う方が stage 1 handoff line と対称である。ただし fixture-side `OptionDecl.admit` は already elaborated predicate node なので、current phase で direct lowering や helper-local canonical surface compare を actualize すると hidden predicate parse / normalization point を増やしやすい。したがって current handoff line は docs-only deferred に留め、predicate fragment boundary が見えるまで reopen しないのが最小である。
- さらに current actualization として、request-local clause spillover は bare `require` / `ensure` line の helper-local malformed-source pair まで actualize してよい。これは request head / clause attachment multiline shape を parse せずに later-stage clause boundary を fail-closed に示す cut であり、hidden request attachment rule や predicate parse を持ち込まない。
- さらに current comparison として、predicate fragment boundary 自体は opaque surface retention や declaration-side only compare に寄せず、declaration-side `admit` と request-local `require` / `ensure` が共有する isolated helper-local fragment parser から reopen するのが最小である。
- current actualization として、その first tranche は program parser の accepted cluster を広げず、isolated fragment string だけを input にして `e2` / `e3` / `e10` / `e11` の fixture-side predicate subset compare を通す line に留める。
- current remaining risk は、grouping / `and` の helper-local evidence が通ったことを理由に request head + clause attachment multiline shape や fixture-side full request contract compare を早く混ぜてしまうことである。current helper はあくまで private / test-only / non-production であり、request cluster の attachment rule と multiline suite は still later stage に残す。
- current sequencing judgment としては、fragment helper の malformed-source pair を先に増やすより、declaration-side `admit:` と request-local `require:` / `ensure:` の multiline attachment shape を shared structural floor として比較する方が staged line に合う。理由は、first checker cut で clause attachment と predicate fragment が別 cluster であり、bridge を先に切る方が diagnostics contract を unnecessary に太らせないためである。
- current comparison judgment としては、その shared structural floor は clause suite 全体や generic continuation ではなく、`<clause-head>:` + 直下 1 段深い predicate block という **single attachment frame** に留めるのが最小である。request-local clause suite owner / ordering / multiplicity と option-local metadata owner は still later stage に残し、fragment helper input formation だけを先に共有する。
- current actualization として、その shared single attachment frame は helper-local / test-only multiline block extraction bridge として actualize 済みであり、declaration-side `admit:` と request-local `require:` / `ensure:` の extracted fragment source を既存 isolated predicate fragment helper へ渡す line まで current repo で通してよい。current bridge は clause header search を head の immediate child attachment line に限定し、blank line は helper-local で fail-closed に reject する。
- current remaining risk は、multiline attachment bridge が通ったことを理由に request-local clause suite completion や attachment malformed diagnostics family を同時に太らせてしまうことである。current sequencing judgment としては malformed-source pair extension より先に request-local `require:` / `ensure:` の sibling clause suite structural floor を比べるのが自然であり、duplicate clause や clause-between blank line の malformed family は still later stage に残す。
- current comparison judgment としては、その request-local suite structural floor は `perform` owner の fixed two-slot suite に留めるのが最小である。ordered clause list や generic keyed map を先に入れると、current companion line より抽象 shape が先行しやすい。
- current next actualization judgment としては、その fixed two-slot suite floor を helper-local / test-only actual evidence に上げる first tranche は clause presence summary だけでは弱く、`require_fragment_text` / `ensure_fragment_text` を持つ suite bridge を切るのが最小である。これにより slot ごとの fragment compare を existing isolated predicate fragment helper へ委ねつつ、request head full parse や full request contract compare を still later stage に残せる。
- current actualization として、その suite bridge first tranche は helper-local / test-only actual evidence として actualize 済みであり、single-line / multiline の mixed clause payload を同じ slot carrier に載せ、`require` after `ensure`、duplicate `require`、clause-between blank line を minimal structural fail-closed として通してよい。
- current remaining risk は、suite bridge first tranche が通ったことを理由に duplicate `ensure` / unsupported child line / public diagnostics widening を同時に太らせることである。next narrow step では malformed/source family extension と fixture-side full request contract compare の sequencing を改めて比較するのが自然である。
- current sequencing judgment としては、その next narrow step は fixture-side full request contract compare より先に helper-local malformed/source family extension を比較するのが自然である。理由は、suite bridge first tranche がすでに fail-closed path を持っており、これを source-backed に可視化してから request head parse pressure を増やす方が staged line に合うためである。
- current malformed/source pair comparison としては、その first pair は `duplicate ensure` + unsupported direct child line を先に actualize するのが最小である。理由は、`duplicate require` と対になる at-most-one symmetry を補いつつ、suite helper が generic continuation parser ではないことを source-backed に固定できるためである。
- current actualization として、その first pair は helper-local / test-only focused smoke に actualize 済みであり、current tranche では helper code widening や public diagnostics carrier 導入は不要である。
- `missing multiline ensure block` は multiline attachment bridge family と近いため、current phase では still later に残してよい。
- current next-step sequencing としては、その first pair actualization の後も fixture-side full request contract compare より先に `missing multiline ensure block` family を surfaced するのが自然である。理由は、current helper family の hidden fail-closed path を narrow に閉じ切ってから request head / contract node compare へ進む方が staged line に合うためである。
- current actualization として、その `missing multiline ensure block` hidden path も helper-local / test-only focused smoke に actualize 済みであり、current phase では suite bridge family の existing hidden malformed path を narrow に surfaced する line までは閉じたと読んでよい。
- current remaining risk は、ここから bare clause line family / fixture-side full request contract compare / public diagnostics refinement を同時に開いて staged line を崩すことである。current reopen judgment としては、remaining suite malformed wording family の多くは shared attachment helper か clause-token family と重複しやすいため、suite helper 側で wording を still 追うより、full request compare を narrow 条件つきで reopen する方が自然である。
- その reopen 条件は、request head parse を still later に残し、source-side helper output を `Stage3RequestClauseSuite` の two-slot carrier に留め、fixture-side compare を `contract.require` / `contract.ensure` subset だけに限定することである。
- current next actualization cut としては、その fixture-side compare を ad-hoc per-slot wiring に留めず、`Stage3RequestContractSubset` 相当の dedicated helper-local carrier に切る方が自然である。理由は、fixed two-slot suite bridge と fixture-side contract subset compare の接点を explicit にでき、なお request head parse を still later に残せるためである。
- current actualization として、その helper-local carrier first tranche は source-backed に通ったが、remaining risk はここから request head metadata を同じ carrier に早く混ぜて compare surface を太らせることである。current next narrow step は、request head neutral carrier を保ったまま contract-only compare surface をどこまで widening してよいかを comparison として切ることである。
- current guard judgment としては、row-list widening も現時点では still early である。理由は、current fixture corpus に multi-row clause array anchor がなく、source-side suite bridge も still fixed two-slot / one-fragment-per-slot shape に留まっているためである。
- current sequencing judgment としては、その先の source-side suite bridge widening 条件を speculative に積むより、request contract subset family は current tranche で freeze し、Phase 3 全体の parser boundary / first checker cut connection へ戻る方が自然である。
- current side-selection judgment としては、その戻り先でも parser-side accepted cluster widening を先に再開するより、first checker cut connection 側から existing parser evidence の reconnect family を比べる方が自然である。理由は、syntax pressure を still 抑えつつ local / structural / decidable floor の整理を前に出せるためである。
- current reconnect-family judgment としては、その first reconnect family は stage 1 chain / declaration structural floor family を先に取る方が自然である。理由は、`specs/examples/45...` の same-lineage / missing-option / capability floor baseline と `specs/examples/73...` の checker-led staged spike が同じ direction を向いており、parser-side actual evidence も stage 1 family が最も厚いためである。
- current first-tranche actualization としては、existing `e4` / `e7` stage 1 working set を維持しつつ、`e13-malformed-capability-strengthening` と `e16-malformed-missing-chain-head-option` を representative fixture compare に追加し、`Stage1ReconnectClusters` summary を helper-local / test-only に置く cut が最小である。一方で `e17-malformed-missing-predecessor-option` は current stage 1 surface で structural predecessor が chain progression に固定されるため still later に残す。
- さらに current docs-only refinement として、actual implementation へ入る直前 cut では input surface は test inline string、`decl_guard_slot` internal carrier は dedicated wrapper + owned `surface_text`、private helper family は `current_l2_stage1_parser_spike_support` を第一候補にするのが最小である。
- current sequencing judgment としては、その後の next reconnect step は stage 1 `e18` / `e19` / `e20` widening より先に stage 2 `try` / rollback structural floor reconnect を取る方が自然である。理由は、stage 1 widening では `e19` が current reconnect summary contract を押し広げやすい一方、stage 2 側には `checked_try_rollback_structural_*` dedicated contract と `e23` / `e24` pair が already あるためである。
- current actualization として、その stage 2 first tranche は `current_l2_stage2_try_rollback_spike_support` private helper と focused tests で helper-local / test-only に actualize 済みである。
  - compare surface は fixture-side `checked_try_rollback_structural_verdict` / `checked_try_rollback_structural_findings`
  - first-tranche evidence は `e23` / `e24` malformed pair と valid one-shot `atomic_cut` in try body の `no_findings` smoke
  - nested `place`、`place_anchor == current_place` gate、restore scope は still later に残す
- current next widening judgment としては、stage 2 `E21` / `E22` runtime contrast を先に parser-side reconnect へ mirror するより、current summary contract を保ったまま stage 1 `e18` / `e20` widening を先に actualize する方が自然である。
- current actualization として、その stage 1 summary-preserving widening tranche は support helper widening なしに focused tests だけで actualize 済みである。
  - `e18` は `same_lineage_floor + missing_option_structure_floor`
  - `e20` は `same_lineage_floor + capability_strengthening_floor`
  - `e19` は `declared_target_mismatch_floor` の summary redesign pressure があるため still later に残す
- current threshold judgment としては、その後も `e19` を reconnect summary へ direct に混ぜたり、stage 2 `E21` / `E22` contrast を parser-side reconnect へ mirror したりするより、reconnect subline を current tranche で freeze する方が自然である。
  - `e19` は already declared target edge pair family の typed static reason anchor を持つ
  - `E21` / `E22` は runtime / proof boundary に近く、current `checked_try_rollback_structural_*` contract の widening なしには reconnect helper に自然に乗らない
  - current reconnect line は stage 1 representative widening と stage 2 malformed pair + valid one-shot `atomic_cut` smoke までで一区切りとみなしてよい
- ただし dedicated text fixture file の path policy、span-aware carrier、final parser-side type 名、final parser API は引き続き OPEN である。
- request / admissibility cluster は semantic inventory 上は first parser cut 候補に入れてよいが、current phase では checker boundary への直結が弱いため actual parser spike の第 1 段には置かない。

### first checker cut / proof boundary

- current L2 では、first checker cut に local / structural / decidable 寄りの floor だけを入れ、global invariant proof は theorem prover / model checker 側へ残す方が自然である。
- first checker cut に候補として入れてよいのは、same-lineage static evidence floor、malformed / underdeclared rejection、minimal capability strengthening prohibition、request-local / option-local clause attachment、minimal predicate fragment、`try` / rollback locality の structural floor である。
- ただし canonical normalization の一般証明、no re-promotion、rollback / cut non-interference、multi-shot continuation / membership churn / scheduler のような global property は current L2 checker cut に入れない。
- current first package は `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md` に集約済みであり、`core_static_checker` / `theorem_prover_boundary` / `protocol_verifier_boundary` / `runtime_policy_boundary` の 4-way split を current first choice に置く。
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md` までで、proof-obligation matrix を docs 正本に置き、source evidence を参照する mixed row bundle を later reopen 用 handoff sketch として切る current first choice までは整理済みである。
- `specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md` により、current docs-only default は mixed row bundle 維持であり、boundary-specific handoff artifact と actual handoff emitter は concrete consumer pressure が出たときだけ reopen 候補にする threshold まで固定済みである。
- `specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md` により、current first practical candidate は `theorem_prover_boundary`、`protocol_verifier_boundary` は second practical candidate、`runtime_policy_boundary` は later candidate に置く current first choice まで固定済みである。
- `specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md` により、mixed row default を壊さずに theorem-side projection bundle を docs-only first cut に置く current first choice まで固定済みである。
- `specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md` により、theorem-side projection bundle の `evidence_refs` は typed symbolic ref family を current first choice に置き、actual path / URI / emitted artifact id は later reopen に残す current cut まで固定済みである。
- `specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md` により、projection bundle を current phase では docs-only に留め、public checker migration は concrete theorem consumer pressure が出たときだけ reopen 候補にする threshold まで固定済みである。
- `specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md` により、concrete theorem consumer bridge の current minimum contract row core は `obligation_kind + evidence_refs` に留め、`goal_text` / `proof_hint` / `consumer_hint` は later consumer-specific attachment に残す threshold まで固定済みである。
- `specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md` により、concrete theorem consumer class の current first practical candidate は `proof_notebook`、`proof_assistant_adapter` は second practical candidate、`theorem_export_checker` は later candidate に固定済みである。
- `specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md` により、`proof_notebook` first bridge の current lightweight attachment は `goal_text` に留め、`proof_hint` / `consumer_hint` は later attachment family に残す threshold まで固定済みである。
- `specs/examples/136-current-l2-theorem-line-notebook-bridge-artifact-threshold.md` により、current phase では `proof_notebook` first bridge を named artifact family へ昇格させず、docs-only derived view に留める first choice まで固定済みである。named stable notebook bridge sketch は concrete notebook workflow pressure が出たときだけ reopen 候補にし、actual emitted notebook artifact はさらに強い consumer pressure が揃うまで later reopen に残す。
- `specs/examples/137-current-l2-theorem-line-next-consumer-pressure-comparison.md` により、next practical reopen は concrete notebook workflow pressure comparison を first choice にし、`proof_assistant_adapter` consumer pressure comparison は second practical candidate に留める current order まで固定済みである。
- `specs/examples/138-current-l2-theorem-line-concrete-notebook-workflow-pressure-comparison.md` により、concrete notebook workflow pressure の current first threshold は human review checklist / walkthrough pressure に置き、compare / bless-like flow は second step、actual file exchange はさらに後段に残す current first choice まで固定済みである。
- `specs/examples/139-current-l2-theorem-line-notebook-review-unit-named-bundle-threshold.md` により、current first cut は review checklist / walkthrough unit を docs-only named review unit bundle に寄せるところまでであり、stronger notebook bridge sketch は second step に残す current first choice まで固定済みである。
- `specs/examples/140-current-l2-theorem-line-review-unit-to-bridge-sketch-comparison.md` により、current first cut は named review unit を docs-only notebook bridge sketch (`bridge_subject_ref + review_units + bridge_goal_text`) へ寄せるところまでであり、compare / bless-like review flow metadata は second step に残す current first choice まで固定済みである。
- `specs/examples/141-current-l2-theorem-line-bridge-sketch-compare-metadata-threshold.md` により、current first cut は bridge sketch に compare basis refs までは足し、bless decision / reviewer notes / retained path は second step に残す current first choice まで固定済みである。
- `specs/examples/142-current-l2-theorem-line-compare-ready-bridge-bless-decision-threshold.md` により、current first cut は compare-ready bridge sketch に bless decision state までは足し、reviewer notes / retained path / review session metadata は second step に残す current first choice まで固定済みである。
- `specs/examples/143-current-l2-theorem-line-bless-ready-bridge-review-session-threshold.md` により、current first cut は bless-ready bridge sketch に review-note refs までは足し、retained notebook path / reviewer actor / timestamp / review session metadata は second step に残す current first choice まで固定済みである。
- `specs/examples/144-current-l2-theorem-line-review-linked-bless-bridge-retained-notebook-threshold.md` により、current first cut は review-linked bless bridge に retained-notebook ref までは足し、actual retained path / overwrite / retention policy は second step に残す current first choice まで固定済みである。
- `specs/examples/145-current-l2-theorem-line-retained-bridge-review-session-link-threshold.md` により、current first cut は retained-ready bless bridge に review-session ref までは足し、reviewer actor / timestamp / lifecycle state は second step に残す current first choice まで固定済みである。
- `specs/examples/146-current-l2-theorem-line-session-linked-retained-bridge-review-observation-threshold.md` により、current first cut は session-linked retained bridge に `reviewed_by_ref + reviewed_at_ref` までは足し、session lifecycle state と retention state は second step に残す current first choice まで固定済みである。
- `specs/examples/147-current-l2-theorem-line-observed-session-lifecycle-threshold.md` により、current first cut は observed session-linked retained bridge に `review_session_state` までは足し、retention state / actual retained path policy / emitted artifact pressure は second step に残す current first choice まで固定済みである。
- `specs/examples/148-current-l2-theorem-line-lifecycle-ready-retention-state-threshold.md` により、current first cut は lifecycle-ready retained bridge に `retention_state` までは足し、actual retained path policy / emitted artifact pressure は second step に残す current first choice まで固定済みである。
- `specs/examples/149-current-l2-theorem-line-retention-ready-path-policy-threshold.md` により、current first cut は retention-ready retained bridge に `retained_path_policy_ref` までは足し、actual emitted notebook artifact は second step に残す current first choice まで固定済みである。
- `specs/examples/150-current-l2-theorem-line-path-ready-emitted-artifact-threshold.md` により、current first cut は path-ready retained bridge に `emitted_artifact_ref` までは足し、actual handoff emitter / consumer adapter policy は second step に残す current first choice まで固定済みである。
- `specs/examples/151-current-l2-theorem-line-emitted-ready-handoff-emitter-threshold.md` により、current first cut は emitted-ready retained bridge に `handoff_emitter_ref` までは足し、actual consumer adapter / notebook exchange rule は second step に残す current first choice まで固定済みである。
- `specs/examples/152-current-l2-theorem-line-emitter-linked-consumer-adapter-threshold.md` により、current first cut は emitter-linked retained bridge に `consumer_adapter_ref` までは足し、actual notebook exchange rule / concrete file-blob exchange protocol は second step に残す current first choice まで固定済みである。
- `specs/examples/153-current-l2-theorem-line-adapter-linked-exchange-rule-threshold.md` により、current first cut は adapter-linked retained bridge に `exchange_rule_ref` までは足し、adapter-local validation / concrete file-blob exchange protocol は second step に残す current first choice まで固定済みである。
- `specs/examples/154-current-l2-theorem-line-exchange-ready-adapter-validation-threshold.md` により、current first cut は exchange-ready retained bridge に `adapter_validation_ref` までは足し、actual notebook exchange rule body / environment-specific invocation surface は second step に残す current first choice まで固定済みである。
- `specs/examples/155-current-l2-theorem-line-validation-ready-invocation-surface-threshold.md` により、current first cut は validation-ready retained bridge に `consumer_invocation_surface_ref` までは足し、actual notebook exchange rule body / concrete runtime coupling は second step に残す current first choice まで固定済みである。
- `specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md` により、current first cut は invocation-ready retained bridge に `exchange_rule_body_ref` までは足し、concrete runtime coupling / file-blob transport / failure body は second step に残す current first choice まで固定済みである。
- `specs/examples/157-current-l2-theorem-line-exchange-body-ready-runtime-coupling-threshold.md` により、current first cut は exchange-body-ready retained bridge に `runtime_coupling_ref` までは足し、concrete transport protocol / failure body は second step に残す current first choice まで固定済みである。
- `specs/examples/158-current-l2-theorem-line-runtime-coupling-ready-transport-protocol-threshold.md` により、current first cut は runtime-coupling-ready retained bridge に `transport_protocol_ref` までは足し、concrete failure body は second step に残す current first choice まで固定済みである。
- `specs/examples/159-current-l2-theorem-line-transport-ready-failure-body-threshold.md` により、current first cut は transport-ready retained bridge に `failure_body_ref` までは足し、actual runtime invocation protocol / host binding / failure wording は second step に残す current first choice まで固定済みである。
- `specs/examples/160-current-l2-theorem-line-failure-ready-actual-invocation-protocol-threshold.md`、`161-current-l2-theorem-line-invocation-ready-host-binding-threshold.md`、`162-current-l2-theorem-line-binding-ready-failure-wording-threshold.md`、`163-current-l2-theorem-line-wording-ready-runtime-handoff-threshold.md`、`164-current-l2-theorem-line-runtime-handoff-ready-invocation-receipt-threshold.md`、`165-current-l2-theorem-line-invocation-receipt-ready-runtime-transcript-threshold.md`、`166-current-l2-theorem-line-transcript-ready-materialized-runtime-handoff-threshold.md`、`167-current-l2-theorem-line-materialized-ready-concrete-payload-threshold.md`、`168-current-l2-theorem-line-payload-ready-concrete-transcript-threshold.md`、`169-current-l2-theorem-line-transcript-body-ready-serialized-channel-body-threshold.md`、`170-current-l2-theorem-line-serialized-ready-emitted-attachment-body-threshold.md`、`171-current-l2-theorem-line-attachment-body-ready-emitted-attachment-blob-threshold.md`、`172-current-l2-theorem-line-attachment-blob-ready-retained-file-body-threshold.md`、`173-current-l2-theorem-line-retained-file-body-ready-archive-materialization-threshold.md`、`174-current-l2-theorem-line-archive-materialization-ready-archive-body-bundle-threshold.md`、`175-current-l2-theorem-line-archive-body-ready-archive-bundle-threshold.md`、`176-current-l2-theorem-line-archive-bundle-ready-archive-manifest-threshold.md`、`177-current-l2-theorem-line-archive-manifest-ready-archive-member-family-threshold.md`、`178-current-l2-theorem-line-archive-member-family-ready-archive-member-body-compare-threshold.md`、`179-current-l2-theorem-line-archive-member-body-compare-ready-archive-bless-update-policy-threshold.md`、`180-current-l2-theorem-line-archive-bless-update-policy-ready-retained-archive-payload-threshold.md`、`181-current-l2-theorem-line-retained-archive-payload-ready-archive-retention-layout-threshold.md`、`182-current-l2-theorem-line-archive-retention-layout-ready-retained-archive-payload-body-family-threshold.md`、`183-current-l2-theorem-line-retained-archive-payload-body-family-ready-retained-payload-materialization-family-threshold.md`、`184-current-l2-theorem-line-retained-payload-materialization-family-ready-retained-payload-body-materialization-detail-threshold.md`、`185-current-l2-theorem-line-retained-payload-body-materialization-detail-ready-retained-payload-body-materialization-payload-threshold.md`、`186-current-l2-theorem-line-retained-payload-body-materialization-payload-ready-retained-payload-body-materialization-carrier-detail-threshold.md`、`187-current-l2-theorem-line-retained-payload-body-materialization-carrier-detail-ready-retained-payload-body-materialization-carrier-payload-threshold.md`、`188-current-l2-theorem-line-retained-payload-body-materialization-carrier-payload-ready-retained-payload-bless-update-split-threshold.md`、`189-current-l2-theorem-line-retained-payload-bless-update-split-ready-retained-payload-bless-update-row-pair-threshold.md`、`190-current-l2-theorem-line-retained-payload-bless-update-row-pair-ready-retained-payload-bless-update-row-ref-family-threshold.md`、`191-current-l2-theorem-line-retained-payload-bless-update-row-ref-family-ready-retained-payload-bless-update-dual-ref-bundle-threshold.md`、`192-current-l2-theorem-line-retained-payload-bless-update-dual-ref-bundle-ready-retained-payload-bless-update-strict-dual-field-threshold.md`、`193-current-l2-theorem-line-retained-payload-bless-update-strict-dual-field-ready-consumer-visible-pair-threshold.md` により、current first cut は `actual_runtime_handoff_ref`、`emitted_invocation_receipt_ref`、`runtime_transcript_ref`、`materialized_runtime_handoff_ref`、`concrete_payload_ref`、`concrete_transcript_body_ref`、`serialized_channel_body_ref`、`emitted_attachment_body_ref`、`emitted_attachment_blob_ref`、`retained_file_body_ref`、`archive_materialization_ref`、`archive_body_ref`、`archive_bundle_ref`、`archive_bundle_manifest_ref`、`archive_bundle_member_family_ref`、`archive_member_body_compare_ref`、`archive_bless_update_policy_ref`、`retained_archive_payload_ref`、`archive_retention_layout_ref`、`retained_archive_payload_body_family_ref`、`retained_payload_materialization_family_ref`、`retained_payload_body_materialization_detail_ref`、`retained_payload_body_materialization_payload_ref`、`retained_payload_body_materialization_carrier_detail_ref`、`retained_payload_body_materialization_carrier_payload_ref`、`retained_payload_body_materialization_bless_update_split_ref`、`retained_payload_body_materialization_bless_update_row_pair_ref`、`retained_payload_body_materialization_bless_update_row_ref_family_ref`、`retained_payload_body_materialization_bless_update_dual_ref_bundle_ref`、`retained_payload_body_materialization_bless_side_row_ref`、`retained_payload_body_materialization_update_side_row_ref`、`retained_payload_body_materialization_bless_update_pair` までは symbolic retained bridge に段階的に足せるが、boundary-specific handoff pair は second step に残す current first choice まで固定済みである。
- 引き続き OPEN なのは、notebook consumer pair を handoff-facing pair にどう昇格させるか、pair surface を symbolic retained bridge のまま維持するか boundary-specific artifact row へ actualize するか、pair surface を actual external contract へ actualize する concrete pressure を何とみなすか、`proof_assistant_adapter` consumer pressure を second practical candidate のまま維持する条件がいつ崩れるかである。

### richer host interface

- current host harness は current L2 verification basis
- production host interface として扱うと責務が膨らみすぎる
- uncovered call detection、coverage explanation、preflight の必要性は見えているが、先に host API を肥大化させるより detached artifact 境界を切った方が PoC 前進量が大きい
- current host coverage failure が batch summary で文字列検出に依存している点は drift source だが、これは richer host interface を直ちに入れる理由ではなく、後段で typed coverage field を検討する入口とみなす

### constrained continuation / multi-shot

- current repo では、無制限 coroutine model を採らない方向が decision register にある一方、one-shot / multi-shot の最終モデル、suspension restriction、lifetime crossing rule は未決である。
- current L2 / Mir-0 は linear resource、local rollback、monotone lifetime を強く保つので、state を持つ resource を unrestricted multi-shot continuation へ黙って閉じ込める設計は緊張が大きい。
- したがって current plan では、continuation 問題を current L2 helper task に押し込まず、将来 workstream で
  - one-shot と multi-shot の切り分け
  - capture restriction
  - route / patch interaction
  - lifetime crossing
  をまとめて扱う必要がある。

### dynamic membership / causal metadata

- current repo は synchronized shared-space の小例を将来 workstream に残しているが、participant churn を持つ causal metadata policy はまだ固定していない。
- plain vector clock に participant add / remove を直接重ねると、membership reconfiguration と causal history を同じ carrier へ押し込みやすく、leave 後の古い message を新規 join と誤読しない rule を別途必要とする。
- current working comparison では、participant 集合を plain array の source of truth にするより、
  - session-scoped membership registry
  - explicit activation / deactivation
  - rejoin を区別する incarnation
  - UI / app 用の derived snapshot view
  に分ける方が自然である。
- current plan では、この問題を current L2 parser-free PoC に持ち込まず、
  - shared-space / session membership
  - membership reconfiguration / activation
  - causal metadata / version carrier
  を分けて設計する future problem として残す。
- consensus family についても、`Raft` / `Paxos` / その近縁を implementation candidate として検討するのは自然だが、current repo の architectural line では language core や Mirrorea spec に単一 algorithm を焼き込まない。
- 上位 shared-space 例としては、authoritative lock と global reset を持つ room と、append-only に近い room とで consistency mode の自然さが異なる。したがって participant carrier と consistency mode catalog は分けて比較する必要がある。

### rollback restore scope / checker boundary

- current runtime evidence では、`e21` が `place_anchor == current_place` の cut を、`e22` が nested place mismatch cut を表す。
- これにより、`AtomicCut` event の存在と frontier update の成否は別であることが source-backed に確認できる。
- current code anchor は rollback frame が whole `place_store` snapshot を保持するので、restore scope を place-local checker floor として言い切ると drift source になる。
- したがって current cut では、
  - `TryFallback` / `AtomicCut` の structural floor
  - rollback / cut が chain order を変えないという structural boundary
 までは checker 候補に残し、
  - `place_anchor == current_place` gate
  - restore scope の exact shape
  は runtime / theorem prover boundary に残す。

### async control / memory-model boundary

- current repo で source-backed に固定されているのは、Mir-0 / current L2 における `atomic_cut` の **place-local finalizing cut** と、その rollback frontier への関与までである。
- したがって current line は「`atomic_cut` だけで全非同期制御を表す」ではなく、`atomic_cut` は local cut の最小核に留め、shared-space 側の authority / consistency / fairness / audit と結びつく高位 ordering は別 line として扱う。
- ここで C++ 的な low-level `memory_order` family を早く導入すると、scheduler・hardware-memory-like semantics・proof burden・user-facing syntax を同時に背負いやすい。
- current repo で比較価値が高いのは、むしろ
  - event-tree / derived execution view
  - authoritative serial transition
  - owner slot / delegated capability
  - auditable witness / explicit provider
  を組み合わせた higher-level async-control family であり、これは Phase 4 / 5 の docs-first comparison として進めるのが自然である。
- compile-time / checker 側に残せるのは、当面は local / structural / decidable 寄りの floor までであり、global ordering・fairness・scheduler interaction は theorem prover / model checker / runtime policy 側に残す。

### portability / observability hooks

- current repo の mainline は semantics core を先に安定させる段階であり、CPU 固定・OS 固定・特定 HW 固定の runtime assumptions を規範判断へ入れない。
- detached artifact、step execution、graph extraction / visualization は、current L2 の semantics そのものではなく、replaceable tooling / observability layer として扱う方が手戻りが小さい。
- したがって current PoC では、
  - non-production helper を repo 相対 path と plain JSON で narrow に保つ
  - debug / graph / trace hook を helper boundary の外から差し替えられる設計に寄せる
  - GPU / accelerator support や richer debugger は future implementation concern として残す
  という方針を採る。

## 何を未決のまま残すか

次は current L2 で無理に決めない。

- final parser syntax
- machine-readable catalog asset / manifest 採用
- selector grammar / alias grammar の長期固定
- path canonicalization policy
- detached trace / audit serialization
- detached validation loop の actual exporter API finalization
- portability / observability hook の concrete API
- richer host interface
- constrained continuation / multi-shot model
- shared-space membership / causal metadata policy
- multi-request scheduler
- `Approximate` / `Compensate`
- stage 3 parser boundary の request head + clause attachment multiline shape と predicate fragment boundary reopen の exact cut

## update の見方

各 task でこれらを新たに決めた場合は、この文書を更新する。
決まっていない場合は、決まっていない理由を report に残し、この文書の status を維持する。
