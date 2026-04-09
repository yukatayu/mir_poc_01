# plan/13 — 重い将来 workstream

## 目的

この文書は、今すぐ着手しないが計画には明示的に入れておくべき重い workstream を整理する。
current L2 parser-free PoC の延長だけでは扱いきれない論点を、将来の独立した workstream として切り出す。

## なぜ今 plan に含めるのか

- 後で急に現れる論点ではないため
- 現在の semantics / helper / notation の設計が、将来の型・解析・証明可能性にどの程度影響するかを意識しておく必要があるため
- current L2 でまだ扱わない理由を明文化し、scope creep を防ぐため

## workstream 1. 型システムの強さ

### 主題

- ownership / lifetime / contract / effect / capability をどこまで型レベルへ持ち上げるか
- inference と annotation のバランス
- linearity / monotonicity をどこまで型規則で強制するか

### entry criteria

- current L2 semantics の failure space と fallback 読みが十分安定していること
- parser 境界の最小 shape が見えていること

### current inventory note

- `place` / `try-fallback` / `perform on` / `perform via` / statement-local `require` / `ensure` / option declaration core / option-local `admit` / explicit edge-row family までは first parser cut 候補として inventory 化してよい。
- exact lexical polish と richer predicate grammar はまだ companion / OPEN に残す。
- same-lineage static evidence floor、malformed / underdeclared rejection、minimal capability strengthening prohibition、request-local / option-local clause attachment、minimal predicate fragment、`try` / rollback locality の structural floor までは first checker cut 候補として inventory 化してよい。
- current static-only corpus baseline では、same-lineage floor `4`、capability floor `2`、missing-option structure floor `3` まで source-backed な regression baseline が見え始めているため、heavy workstream 前の checker-boundary整理は次段 mainline に戻してよい。
- same-lineage floor については helper-local first checker spike を入れてよいが、heavy workstream に入る前の current 段階では final checker API へはまだ固定しない。
- missing-option structure floor についても helper-local second checker spike を入れてよいが、same-lineage / missing-option を共通 helper に寄せるかどうかは heavy workstream 前にもう一段 narrow に比較する。
- capability strengthening floor についても helper-local third checker spike を入れてよいが、same-lineage / missing-option / capability を共通 helper に寄せるかどうかは heavy workstream 前にもう一段 narrow に比較する。

### 今すぐやらない理由

- 現在は syntax と semantics の companion 整理段階であり、型規則を先に固定すると全体が早期凍結しやすい

### 将来 deliverable 候補

- typing judgment の最小スケッチ
- representative programs に対する typing walk-through
- annotation burden / inference burden の比較表

## workstream 2. 静的解析可能性

### 主題

- lineage、fallback admissibility、capability mismatch、underdeclared case を静的にどこまで検出できるか
- parser-free fixture の expectation を、どこまで static analysis に移せるか

### entry criteria

- AST / syntax boundary が一定以上安定していること
- current helper stack の public behavior と thin delegation が整理されていること

### 今すぐやらない理由

- 解析対象の surface と IR がまだ companion notation 段階である

### 将来 deliverable 候補

- static checker の prototype
- representative fixture に対する static-only 判定の formalization
- false positive / false negative trade-off の整理

## workstream 3. 定理証明可能性

### 主題

- current L2 invariants を theorem prover に送れる形へ落とせるか
- canonical normalization、no re-promotion、rollback-cut non-interference などの性質をどう証明するか

### entry criteria

- semantics が current L2 で十分安定していること
- syntax ではなく semantic core を対象にできること

### 今すぐやらない理由

- まだ proof object を支える core formalization が薄い
- 先に parser-free PoC と docs mirror の drift を抑える方が費用対効果が高い

### 将来 deliverable 候補

- proof sketch
- theorem prover 向け core relation の encoding 案
- どの invariant が machine-checked proof に向くかの棚卸し

## workstream 4. 決定可能性

### 主題

- predicate sublanguage、fallback chain validation、contract check、effect wiring のどこが decidable か
- undecidable / semi-decidable な論点をどこで切るか

### entry criteria

- current L2 の範囲が一定程度閉じていること
- static analysis の土台が見えていること

### 今すぐやらない理由

- 現時点では semantics の companion 整理が優先であり、decision procedure 設計はまだ早い

### 将来 deliverable 候補

- decidability matrix
- complexity note
- language core と external verifier 境界の提案

## workstream 5. 実装可能性 / complexity

### 主題

- parser、checker、runtime、host interface、scheduler を実装したときの複雑さ
- current semantics が implementation complexity をどこまで押し上げるか

### entry criteria

- parser 前提の syntax / AST / runtime boundary がもう少し安定していること

### 今すぐやらない理由

- まだ production runtime を設計する段階ではない

### 将来 deliverable 候補

- subsystem ごとの complexity inventory
- prototype 実装比較
- benchmark の最小設計

## workstream 6. 非同期制御 / scheduler / memory-model boundary

### 主題

- `atomic_cut` のような local cut と、higher-level async-control / ordering / fairness semantics をどこで分けるか
- event-tree / authority-serial transition / witness-aware commit のような高位制御族を language / runtime / external verifier のどこに置くか
- low-level memory-order 語彙を採るべきか、それとも higher-level room / authority / consistency family に留めるべきか

### entry criteria

- shared-space / membership / authority / consistency / fairness の docs-first boundary がある程度揃っていること
- small decidable core inventory が見えており、local cut と global ordering の切り分けを語れること

### 今すぐやらない理由

- current repo で source-backed なのは `atomic_cut` の place-local cut と rollback frontier line までであり、scheduler / fairness / hardware-memory-like semantics まで immediate に固定する段階ではない
- low-level memory-order を早く入れると parser / checker / runtime / proof のすべてが同時に重くなりやすい

### current inventory note

- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md` では、current first package として
  - `core_static_checker`
  - `theorem_prover_boundary`
  - `protocol_verifier_boundary`
  - `runtime_policy_boundary`
  の 4-way split を整理済みである。
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md` と `specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md` により、proof-obligation matrix は docs 正本、external handoff artifact は mixed row bundle を current default に維持し、boundary-specific split と actual emitter は concrete consumer pressure が出たときだけ reopen する threshold まで切れている。
- `specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md` により、first concrete external consumer pressure の current first practical candidate は `theorem_prover_boundary` に置く。
- `specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md` により、theorem line の current first cut は actual emitter ではなく docs-only projection bundle に留める。
- `specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md` により、theorem-side `evidence_refs` は typed symbolic ref family を current first choice に置き、actual path / URI は later reopen に残す。
- `specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md` により、current phase では theorem-side projection bundle を docs-only bridge に留め、public checker migration は concrete theorem consumer pressure が出たときだけ reopen する threshold まで固定した。
- `specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md` により、minimum contract row core は `obligation_kind + evidence_refs` に留め、`goal_text` / `proof_hint` / `consumer_hint` は later consumer-specific attachment に残す current first choice まで固定した。
- `specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md` により、first practical consumer class は `proof_notebook` に置き、`proof_assistant_adapter` は second practical candidate、`theorem_export_checker` は later candidate に留める current first choice まで固定した。
- `specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md` により、`proof_notebook` first bridge の current attachment は `goal_text` に留め、`proof_hint` / `consumer_hint` は later reopen に残す current first choice まで固定した。
- `specs/examples/136-current-l2-theorem-line-notebook-bridge-artifact-threshold.md` により、current phase では `proof_notebook` first bridge を docs-only derived view に留め、named notebook bridge sketch と actual emitted notebook artifact は concrete notebook workflow pressure が出たときだけ reopen 候補に残す current first choice まで固定した。
- `specs/examples/137-current-l2-theorem-line-next-consumer-pressure-comparison.md` により、current next practical reopen は concrete notebook workflow pressure comparison を first choice、`proof_assistant_adapter` consumer pressure comparison を second practical candidate に置く current order まで固定した。
- `specs/examples/138-current-l2-theorem-line-concrete-notebook-workflow-pressure-comparison.md` により、concrete notebook workflow pressure の current first threshold は human review checklist / walkthrough pressure に置く current first choice まで固定した。
- `specs/examples/139-current-l2-theorem-line-notebook-review-unit-named-bundle-threshold.md` により、current first cut は review checklist / walkthrough unit を docs-only named bundle に寄せるところまでに留め、stronger notebook bridge sketch は later reopen に残す current first choice まで固定した。
- `specs/examples/140-current-l2-theorem-line-review-unit-to-bridge-sketch-comparison.md` により、current first cut は named review unit を docs-only notebook bridge sketch に寄せるところまでに留め、compare / bless-like review flow metadata は later reopen に残す current first choice まで固定した。
- `specs/examples/141-current-l2-theorem-line-bridge-sketch-compare-metadata-threshold.md` により、current first cut は bridge sketch に compare basis refs までを足し、bless decision state は later reopen に残す current first choice まで固定した。
- `specs/examples/142-current-l2-theorem-line-compare-ready-bridge-bless-decision-threshold.md` により、current first cut は compare-ready bridge sketch に bless decision state までを足し、review-session metadata は later reopen に残す current first choice まで固定した。
- `specs/examples/143-current-l2-theorem-line-bless-ready-bridge-review-session-threshold.md` により、current first cut は bless-ready bridge sketch に review-note refs までを足し、retained notebook path / review session lifecycle は later reopen に残す current first choice まで固定した。
- `specs/examples/144-current-l2-theorem-line-review-linked-bless-bridge-retained-notebook-threshold.md` により、current first cut は review-linked bless bridge に retained-notebook ref までを足し、actual retained path policy は later reopen に残す current first choice まで固定した。
- `specs/examples/145-current-l2-theorem-line-retained-bridge-review-session-link-threshold.md` により、current first cut は retained-ready bless bridge に review-session ref までを足し、actor / timestamp / lifecycle state は later reopen に残す current first choice まで固定した。
- `specs/examples/146-current-l2-theorem-line-session-linked-retained-bridge-review-observation-threshold.md` により、current first cut は session-linked retained bridge に `reviewed_by_ref + reviewed_at_ref` までを足し、session lifecycle state は later reopen に残す current first choice まで固定した。
- `specs/examples/147-current-l2-theorem-line-observed-session-lifecycle-threshold.md` により、current first cut は observed session-linked retained bridge に `review_session_state` までを足し、retention state / actual retained path policy は later reopen に残す current first choice まで固定した。
- `specs/examples/148-current-l2-theorem-line-lifecycle-ready-retention-state-threshold.md` により、current first cut は lifecycle-ready retained bridge に `retention_state` までを足し、actual retained path policy / emitted artifact は later reopen に残す current first choice まで固定した。
- `specs/examples/149-current-l2-theorem-line-retention-ready-path-policy-threshold.md` により、current first cut は retention-ready retained bridge に `retained_path_policy_ref` までを足し、actual emitted notebook artifact は later reopen に残す current first choice まで固定した。
- `specs/examples/150-current-l2-theorem-line-path-ready-emitted-artifact-threshold.md` により、current first cut は path-ready retained bridge に `emitted_artifact_ref` までを足し、actual handoff emitter / consumer adapter policy は later reopen に残す current first choice まで固定した。
- `specs/examples/151-current-l2-theorem-line-emitted-ready-handoff-emitter-threshold.md` により、current first cut は emitted-ready retained bridge に `handoff_emitter_ref` までを足し、actual consumer adapter / notebook exchange rule は later reopen に残す current first choice まで固定した。
- `specs/examples/152-current-l2-theorem-line-emitter-linked-consumer-adapter-threshold.md` により、current first cut は emitter-linked retained bridge に `consumer_adapter_ref` までを足し、actual notebook exchange rule / adapter-local validation は later reopen に残す current first choice まで固定した。
- `specs/examples/153-current-l2-theorem-line-adapter-linked-exchange-rule-threshold.md` により、current first cut は adapter-linked retained bridge に `exchange_rule_ref` までを足し、adapter-local validation / environment-specific invocation surface は later reopen に残す current first choice まで固定した。
- `specs/examples/154-current-l2-theorem-line-exchange-ready-adapter-validation-threshold.md` により、current first cut は exchange-ready retained bridge に `adapter_validation_ref` までを足し、actual notebook exchange rule body / environment-specific invocation surface は later reopen に残す current first choice まで固定した。
- `specs/examples/155-current-l2-theorem-line-validation-ready-invocation-surface-threshold.md` により、current first cut は validation-ready retained bridge に `consumer_invocation_surface_ref` までを足し、actual notebook exchange body / concrete runtime coupling は later reopen に残す current first choice まで固定した。
- `specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md` により、current first cut は invocation-ready retained bridge に `exchange_rule_body_ref` までを足し、concrete runtime coupling / transport protocol / failure body は later reopen に残す current first choice まで固定した。
- `specs/examples/157-current-l2-theorem-line-exchange-body-ready-runtime-coupling-threshold.md` により、current first cut は exchange-body-ready retained bridge に `runtime_coupling_ref` までを足し、concrete transport protocol / failure body は later reopen に残す current first choice まで固定した。
- `specs/examples/158-current-l2-theorem-line-runtime-coupling-ready-transport-protocol-threshold.md` により、current first cut は runtime-coupling-ready retained bridge に `transport_protocol_ref` までを足し、concrete failure body は later reopen に残す current first choice まで固定した。
- `specs/examples/159-current-l2-theorem-line-transport-ready-failure-body-threshold.md` により、current first cut は transport-ready retained bridge に `failure_body_ref` までを足し、actual runtime invocation protocol / host binding / failure wording は later reopen に残す current first choice まで固定した。
- したがって current next pressure は low-level memory-order の即時導入ではなく、actual-invocation-protocol threshold を narrow に比較することである。

### 将来 deliverable 候補

- async-control vocabulary comparison
- event-tree / authority-serial / witness-aware commit の formal sketch
- decidable core と external verifier 境界の比較表
- room profile 例に対する ordering / fairness / replay obligation walk-through

## language core と external verifier の境界

これは重い workstream の横断論点である。

### current で残す working question

- どこまでを言語 core に入れるか
- どこからを external verifier / theorem prover / out-of-band analyzer に送るか
- machine-check と human-facing explanation の境界をどこまで formalize するか

### current 方針

- いまは language core を過剰に肥大化させない
- current L2 helper stack に proof / analysis obligations を押し込まない
- ただし将来の移送先として external verifier / theorem prover を明示的に計画へ残す
- first parser cut と first checker cut は、heavy workstream の前に切る narrow gate として扱う

## この workstream に最低限必要なもの

本格着手の前に、少なくとも次が必要である。

- stable enough semantic core
- representative examples と fixture による regression baseline
- parser / AST / runtime の boundary inventory
- proof / analysis / complexity を議論するための最小 IR か relation 定義

## まとめ

これらの heavy workstream は、今すぐ実装しない。
しかし current L2 の decisions が将来それらに接続できるよう、**計画には明示的に残す**。
