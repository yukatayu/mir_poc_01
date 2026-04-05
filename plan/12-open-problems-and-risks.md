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
| dynamic membership / causal metadata | shared space / fabric | OPEN / FUTURE | participant churn を plain vector clock deletion だけで扱うと membership change と causal history が混線しやすい | shared-space / Mirrorea workstream 側で、membership reconfiguration と causal metadata を分離して設計する |
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
- さらに current docs-only refinement として、actual implementation へ入る直前 cut では input surface は test inline string、`decl_guard_slot` internal carrier は dedicated wrapper + owned `surface_text`、private helper family は `current_l2_stage1_parser_spike_support` を第一候補にするのが最小である。
- ただし dedicated text fixture file の path policy、span-aware carrier、final parser-side type 名、final parser API は引き続き OPEN である。
- request / admissibility cluster は semantic inventory 上は first parser cut 候補に入れてよいが、current phase では checker boundary への直結が弱いため actual parser spike の第 1 段には置かない。

### first checker cut / proof boundary

- current L2 では、first checker cut に local / structural / decidable 寄りの floor だけを入れ、global invariant proof は theorem prover / model checker 側へ残す方が自然である。
- first checker cut に候補として入れてよいのは、same-lineage static evidence floor、malformed / underdeclared rejection、minimal capability strengthening prohibition、request-local / option-local clause attachment、minimal predicate fragment、`try` / rollback locality の structural floor である。
- ただし canonical normalization の一般証明、no re-promotion、rollback / cut non-interference、multi-shot continuation / membership churn / scheduler のような global property は current L2 checker cut に入れない。

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
- current plan では、この問題を current L2 parser-free PoC に持ち込まず、
  - shared-space / session membership
  - membership reconfiguration / activation
  - causal metadata / version carrier
  を分けて設計する future problem として残す。

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

## update の見方

各 task でこれらを新たに決めた場合は、この文書を更新する。
決まっていない場合は、決まっていない理由を report に残し、この文書の status を維持する。
