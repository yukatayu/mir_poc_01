# tasks

最終更新: 2026-04-10 03:48 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- `progress.md` が rough progress snapshot なのに対し、ここでは
  - ある程度まとまった task として自走できるもの
  - 方針決定が必要で、いま研究の障害になっているもの
  をもう少し具体的に整理する。
- 規範判断の正本は `specs/`、長期比較と repository memory は `plan/` である。
- append で履歴を積まず、**毎回 current snapshot に合わせて全体を書き直す**。

## 現在の読み

- 主線は **Phase 0 / 1 / 2 maintenance tail + cross-phase checkpoint maintenance** である。
- Phase 4 は、authoritative room baseline / working subset / minimal witness core / delegated-provider practical cut / control-plane threshold comparison まで current package を切り終え、**checkpoint maintenance と later reopen candidate** に移った。
- Phase 5 は、`specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`、`specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`、`specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md`、`specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md`、`specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md`、`specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md`、`specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md`、`specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md`、`specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md`、`specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md`、`specs/examples/136-current-l2-theorem-line-notebook-bridge-artifact-threshold.md`、`specs/examples/137-current-l2-theorem-line-next-consumer-pressure-comparison.md`、`specs/examples/138-current-l2-theorem-line-concrete-notebook-workflow-pressure-comparison.md`、`specs/examples/139-current-l2-theorem-line-notebook-review-unit-named-bundle-threshold.md`、`specs/examples/140-current-l2-theorem-line-review-unit-to-bridge-sketch-comparison.md`、`specs/examples/141-current-l2-theorem-line-bridge-sketch-compare-metadata-threshold.md`、`specs/examples/142-current-l2-theorem-line-compare-ready-bridge-bless-decision-threshold.md`、`specs/examples/143-current-l2-theorem-line-bless-ready-bridge-review-session-threshold.md`、`specs/examples/144-current-l2-theorem-line-review-linked-bless-bridge-retained-notebook-threshold.md`、`specs/examples/145-current-l2-theorem-line-retained-bridge-review-session-link-threshold.md`、`specs/examples/146-current-l2-theorem-line-session-linked-retained-bridge-review-observation-threshold.md`、`specs/examples/147-current-l2-theorem-line-observed-session-lifecycle-threshold.md`、`specs/examples/148-current-l2-theorem-line-lifecycle-ready-retention-state-threshold.md`、`specs/examples/149-current-l2-theorem-line-retention-ready-path-policy-threshold.md`、`specs/examples/150-current-l2-theorem-line-path-ready-emitted-artifact-threshold.md`、`specs/examples/151-current-l2-theorem-line-emitted-ready-handoff-emitter-threshold.md`、`specs/examples/152-current-l2-theorem-line-emitter-linked-consumer-adapter-threshold.md`、`specs/examples/153-current-l2-theorem-line-adapter-linked-exchange-rule-threshold.md`、`specs/examples/154-current-l2-theorem-line-exchange-ready-adapter-validation-threshold.md`、`specs/examples/155-current-l2-theorem-line-validation-ready-invocation-surface-threshold.md`、`specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md`、`specs/examples/157-current-l2-theorem-line-exchange-body-ready-runtime-coupling-threshold.md`、`specs/examples/158-current-l2-theorem-line-runtime-coupling-ready-transport-protocol-threshold.md`、`specs/examples/159-current-l2-theorem-line-transport-ready-failure-body-threshold.md`、`specs/examples/160-current-l2-theorem-line-failure-ready-actual-invocation-protocol-threshold.md`、`specs/examples/161-current-l2-theorem-line-invocation-ready-host-binding-threshold.md`、`specs/examples/162-current-l2-theorem-line-binding-ready-failure-wording-threshold.md`、`specs/examples/163-current-l2-theorem-line-wording-ready-runtime-handoff-threshold.md`、`specs/examples/164-current-l2-theorem-line-runtime-handoff-ready-invocation-receipt-threshold.md`、`specs/examples/165-current-l2-theorem-line-invocation-receipt-ready-runtime-transcript-threshold.md`、`specs/examples/166-current-l2-theorem-line-transcript-ready-materialized-runtime-handoff-threshold.md` までで **small decidable core / proof / async-control boundary の theorem-line later package** を一段延ばし、checkpoint maintenance と concrete payload / transcript body comparison という later reopen candidate に移った。
- Phase 3 は current checkpoint では **reserve path** であり、later pressure が出たときだけ reopen 候補にする。

## 次に自走で進める順番と rough estimate

| 順番 | phase | task package | 主眼 | rough weight | rough 所要 | 自走可否 | 備考 |
|---|---|---|---|---|---|---|---|
| 1 | cross-phase checkpoint | drift suppression / mirror sweep | Phase 4 / 5 package close 後の mirror drift を抑える | 低〜中 | 各 checkpoint ごとに 0.5〜1日 | 自走可能 | closeout package。Phase 5 theorem-line package close 後の first maintenance line |
| 2 | Phase 5 checkpoint 後半 / later reopen 候補 | concrete payload / transcript body comparison | `proof_notebook` first bridge に `materialized_runtime_handoff_ref` までを足したうえで、concrete payload / transcript body / actual serialized channel body をどこまで theorem-side bridge に近づけるかを比べる | 中〜重 | 0〜2 task | 一部自走可能 | `specs/examples/126...` から `166...` までで current theorem-line package は close。`proof_assistant_adapter` pressure は second practical candidate に残す |
| 3 | Phase 4 checkpoint 後半 / later reopen 候補 | shared-space control-plane / catalog later reopen | `control_epoch` 相当の split や final catalog を later pressure 時だけ再開する | 中〜重 | 0〜3 task | 一部自走可能 | `specs/examples/121...` から `125...` までで current package は close。reopen は authority handoff / provider binding / activation frontier compare need が出たとき |
| 4 | Phase 2 maintenance tail | detached validation loop residual | drift suppression と policy-dependent residual の切り分け | 低 | 0〜1 task / 必要時のみ | 自走可能 | `reference update / bless` は retention / path policy 依存なので later candidate |
| 5 | Phase 4 checkpoint maintenance | authoritative room baseline / working subset の drift 抑制 | baseline judgment と practical contrast の drift を抑える | 低 | 0〜1 task / drift 時のみ | 自走可能 | baseline 自体は `specs/examples/121...`、working subset は `122...`、threshold は `125...` までで checkpoint close |
| 6 | Phase 3 reserve path | parser boundary / first checker cut reopen | parser boundary / first checker cut を later pressure が出たときだけ再開する | 中〜重 | 0〜2 task | 後段依存 | 今は active package ではない |

## 自走で進められる task package

### Task A. static analysis / type / theorem prover / async-control boundary の checkpoint maintenance と later reopen 候補整理

#### 目的

- 何を local / structural / decidable core に入れるか
- 何を theorem prover / model checker / external verifier 側へ残すか
- `atomic_cut` と higher-level async-control / authority-serial / witness-aware commit family をどこで分けるか

を docs-first に inventory 化する。

#### current checkpoint

- hybrid staged approach 自体は direction として固まっている。
- parser boundary / detached validation loop / shared-space working subset も、inventory を支える前提として揃っている。
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md` で
  - `core_static_checker`
  - `theorem_prover_boundary`
  - `protocol_verifier_boundary`
  - `runtime_policy_boundary`
  の 4-way split を current first choice として集約済みである。
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md` で、
  - proof-obligation matrix を docs 正本に置く
  - external handoff artifact は source evidence を参照する mixed row bundle sketch に留める
  current first choice まで集約済みである。
- `specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md` で、
  - mixed row bundle を current docs-only default に維持する
  - boundary-specific handoff artifact と actual handoff emitter は concrete consumer pressure が出たときだけ reopen する
  threshold まで固定済みである。
- `specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md` で、
  - first concrete external consumer pressure は `theorem_prover_boundary` に置く
  - `protocol_verifier_boundary` は second practical candidate に留める
  - `runtime_policy_boundary` は later candidate に留める
  current first choice まで固定済みである。
- `specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md` で、
  - mixed row default を維持したまま
  - theorem-side projection bundle を docs-only first cut に置く
  current first choice まで固定済みである。
- `specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md` で、
  - theorem-side `evidence_refs` は typed symbolic ref family に整える
  - actual path / URI / emitted artifact id は later reopen に残す
  current first choice まで固定済みである。
- `specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md` で、
  - theorem-side projection bundle は current phase では docs-only bridge に留める
  - public checker migration は concrete theorem consumer pressure が出たときだけ reopen 候補にする
  current first choice まで固定済みである。
- `specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md` で、
  - theorem-side minimum contract row core は `obligation_kind + evidence_refs` に留める
  - `goal_text` / `proof_hint` / `consumer_hint` は later consumer-specific attachment に残す
  current first choice まで固定済みである。
- `specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md` で、
  - theorem-side first practical consumer class は `proof_notebook` に置く
  - `proof_assistant_adapter` は second practical candidate に留める
  - `theorem_export_checker` は later candidate に留める
  current first choice まで固定済みである。
- `specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md` で、
  - `proof_notebook` first bridge の current lightweight attachment は `goal_text` に留める
  - `proof_hint` / `consumer_hint` は later attachment family に残す
  current first choice まで固定済みである。
- `specs/examples/136-current-l2-theorem-line-notebook-bridge-artifact-threshold.md` で、
  - current phase では `proof_notebook` first bridge を named artifact family に昇格させず docs-only derived view に留める
  - named stable notebook bridge sketch は concrete notebook workflow pressure が出たときだけ reopen 候補にする
  - actual emitted notebook artifact はさらに後段に残す
  current first choice まで固定済みである。
- `specs/examples/137-current-l2-theorem-line-next-consumer-pressure-comparison.md` で、
  - next practical reopen は concrete notebook workflow pressure comparison を first choice にする
  - `proof_assistant_adapter` consumer pressure comparison は second practical candidate に留める
  current order まで固定済みである。
- `specs/examples/138-current-l2-theorem-line-concrete-notebook-workflow-pressure-comparison.md` で、
  - concrete notebook workflow pressure の current first threshold は human review checklist / walkthrough pressure に置く
  - compare / bless-like flow は second step に残す
  - actual notebook file exchange はさらに後段に残す
  current first choice まで固定済みである。
- `specs/examples/139-current-l2-theorem-line-notebook-review-unit-named-bundle-threshold.md` で、
  - review checklist / walkthrough unit は docs-only named review unit bundle に寄せる
  - stronger notebook bridge sketch は second step に残す
  current first choice まで固定済みである。
- `specs/examples/140-current-l2-theorem-line-review-unit-to-bridge-sketch-comparison.md` で、
  - current named review unit の次段は docs-only notebook bridge sketch に寄せる
  - stronger compare / bless-like review flow metadata は second step に残す
  current first choice まで固定済みである。
- `specs/examples/141-current-l2-theorem-line-bridge-sketch-compare-metadata-threshold.md` で、
  - bridge sketch の次段では compare basis refs までは足してよい
  - bless decision / reviewer notes / retained path は second step に残す
  current first choice まで固定済みである。
- `specs/examples/142-current-l2-theorem-line-compare-ready-bridge-bless-decision-threshold.md` で、
  - compare-ready bridge sketch の次段では bless decision state までは足してよい
  - reviewer notes / retained path / review session metadata は second step に残す
  current first choice まで固定済みである。
- `specs/examples/143-current-l2-theorem-line-bless-ready-bridge-review-session-threshold.md` で、
  - bless-ready bridge sketch の次段では review-note refs までは足してよい
  - retained notebook path / reviewer actor / timestamp / review session metadata は second step に残す
  current first choice まで固定済みである。
- `specs/examples/144-current-l2-theorem-line-review-linked-bless-bridge-retained-notebook-threshold.md` で、
  - review-linked bless bridge の次段では retained-notebook ref までは足してよい
  - actual retained path / overwrite / retention policy は second step に残す
  current first choice まで固定済みである。
- `specs/examples/145-current-l2-theorem-line-retained-bridge-review-session-link-threshold.md` で、
  - retained-ready bless bridge の次段では review-session ref までは足してよい
  - reviewer actor / timestamp / lifecycle state は second step に残す
  current first choice まで固定済みである。
- `specs/examples/146-current-l2-theorem-line-session-linked-retained-bridge-review-observation-threshold.md` で、
  - session-linked retained bridge の次段では `reviewed_by_ref + reviewed_at_ref` までは足してよい
  - session lifecycle state / retention state は second step に残す
  current first choice まで固定済みである。
- `specs/examples/147-current-l2-theorem-line-observed-session-lifecycle-threshold.md` で、
  - observed session-linked retained bridge の次段では `review_session_state` までは足してよい
  - retention state / actual retained path policy / emitted artifact pressure は second step に残す
  current first choice まで固定済みである。
- `specs/examples/148-current-l2-theorem-line-lifecycle-ready-retention-state-threshold.md` で、
  - lifecycle-ready retained bridge の次段では `retention_state` までは足してよい
  - actual retained path policy / emitted artifact pressure は second step に残す
  current first choice まで固定済みである。
- `specs/examples/149-current-l2-theorem-line-retention-ready-path-policy-threshold.md` で、
  - retention-ready retained bridge の次段では `retained_path_policy_ref` までは足してよい
  - actual emitted notebook artifact は second step に残す
  current first choice まで固定済みである。
- `specs/examples/150-current-l2-theorem-line-path-ready-emitted-artifact-threshold.md` で、
  - path-ready retained bridge の次段では `emitted_artifact_ref` までは足してよい
  - actual handoff emitter / consumer adapter policy は second step に残す
  current first choice まで固定済みである。
- `specs/examples/151-current-l2-theorem-line-emitted-ready-handoff-emitter-threshold.md` で、
  - emitted-ready retained bridge の次段では `handoff_emitter_ref` までは足してよい
  - actual consumer adapter / notebook exchange rule は second step に残す
  current first choice まで固定済みである。
- `specs/examples/152-current-l2-theorem-line-emitter-linked-consumer-adapter-threshold.md` で、
  - emitter-linked retained bridge の次段では `consumer_adapter_ref` までは足してよい
  - actual notebook exchange rule / adapter-local validation は second step に残す
  current first choice まで固定済みである。
- `specs/examples/153-current-l2-theorem-line-adapter-linked-exchange-rule-threshold.md` で、
  - adapter-linked retained bridge の次段では `exchange_rule_ref` までは足してよい
  - adapter-local validation / environment-specific invocation surface は second step に残す
  current first choice まで固定済みである。
- `specs/examples/154-current-l2-theorem-line-exchange-ready-adapter-validation-threshold.md` で、
  - exchange-ready retained bridge の次段では `adapter_validation_ref` までは足してよい
  - environment-specific invocation surface / concrete runtime coupling は second step に残す
  current first choice まで固定済みである。
- `specs/examples/155-current-l2-theorem-line-validation-ready-invocation-surface-threshold.md` で、
  - validation-ready retained bridge の次段では `consumer_invocation_surface_ref` までは足してよい
  - actual notebook exchange body / concrete runtime coupling は second step に残す
  current first choice まで固定済みである。
- `specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md` で、
  - invocation-ready retained bridge の次段では `exchange_rule_body_ref` までは足してよい
  - concrete runtime coupling / file-blob transport / failure body は second step に残す
  current first choice まで固定済みである。
- `specs/examples/157-current-l2-theorem-line-exchange-body-ready-runtime-coupling-threshold.md` で、
  - exchange-body-ready retained bridge の次段では `runtime_coupling_ref` までは足してよい
  - concrete transport protocol / failure body は second step に残す
  current first choice まで固定済みである。
- `specs/examples/158-current-l2-theorem-line-runtime-coupling-ready-transport-protocol-threshold.md` で、
  - runtime-coupling-ready retained bridge の次段では `transport_protocol_ref` までは足してよい
  - concrete failure body は second step に残す
  current first choice まで固定済みである。
- `specs/examples/159-current-l2-theorem-line-transport-ready-failure-body-threshold.md` で、
  - transport-ready retained bridge の次段では `failure_body_ref` までは足してよい
  - actual runtime invocation protocol / host binding / failure wording は second step に残す
  current first choice まで固定済みである。
- `specs/examples/160-current-l2-theorem-line-failure-ready-actual-invocation-protocol-threshold.md` で、
  - failure-ready retained bridge の次段では `actual_invocation_protocol_ref` までは足してよい
  - consumer-host-specific binding / failure wording は second step に残す
  current first choice まで固定済みである。
- `specs/examples/161-current-l2-theorem-line-invocation-ready-host-binding-threshold.md` で、
  - invocation-ready retained bridge の次段では `consumer_host_binding_ref` までは足してよい
  - failure wording と actual notebook runtime handoff actualization は second step に残す
  current first choice まで固定済みである。
- `specs/examples/162-current-l2-theorem-line-binding-ready-failure-wording-threshold.md` で、
  - binding-ready retained bridge の次段では `failure_wording_ref` までは足してよい
  - actual notebook runtime handoff actualization / emitted invocation receipt / runtime transcript family は second step に残す
  current first choice まで固定済みである。
- `specs/examples/163-current-l2-theorem-line-wording-ready-runtime-handoff-threshold.md` で、
  - wording-ready retained bridge の次段では `actual_runtime_handoff_ref` までは足してよい
  - emitted invocation receipt / runtime transcript family は second step に残す
  current first choice まで固定済みである。
- `specs/examples/164-current-l2-theorem-line-runtime-handoff-ready-invocation-receipt-threshold.md` で、
  - handoff-ready retained bridge の次段では `emitted_invocation_receipt_ref` までは足してよい
  - runtime transcript family は second step に残す
  current first choice まで固定済みである。
- `specs/examples/165-current-l2-theorem-line-invocation-receipt-ready-runtime-transcript-threshold.md` で、
  - receipt-ready retained bridge の次段では `runtime_transcript_ref` までは足してよい
  - concrete payload / transcript body / actual materialized handoff artifact は second step に残す
  current first choice まで固定済みである。
- `specs/examples/166-current-l2-theorem-line-transcript-ready-materialized-runtime-handoff-threshold.md` で、
  - transcript-ready retained bridge の次段では `materialized_runtime_handoff_ref` までは足してよい
  - concrete payload / transcript body / actual serialized channel body は second step に残す
  current first choice まで固定済みである。

#### この task で残ること

- concrete payload / transcript body と actual serialized channel body をどこまで足すか
- typed symbolic `evidence_refs` family を theorem-side bridge の stable contract にいつ昇格させるか
- actual handoff emitter を helper-local 既成事実にせず later reopen に保てるか
- low-level memory-order family を将来 external vocabulary としてだけ残すか

#### いま自走できる理由

- final type system actualization に入らず、inventory comparison と handoff comparison に留める限り手戻りが比較的小さい
- `specs/09` の invariants、Phase 3 reserve path、Phase 4 working subset の境界がすでにある

#### 重さ

- 中〜重

#### rough 所要

- 0〜2 task / concrete consumer pressure 時のみ

#### 現在の推奨度

- **checkpoint maintenance / later reopen candidate**

---

### Task B. shared-space / membership line の checkpoint maintenance と later reopen 候補整理

#### 目的

- `specs/examples/121...` から `125...` までで切った current package を drift させない
- stronger control-plane split や final catalog を、later pressure が出たときだけ reopen できる形で残す

#### current checkpoint

- authoritative room baseline は `121...`
- working subset row は `122...`
- minimal witness core は `123...`
- authoritative delegated-provider practical cut は `124...`
- control-plane separated carrier threshold は `125...`

まで source-backed に揃った。

#### この task で残ること

- authority handoff / provider binding / activation frontier を room rule 側へ上げる必要が実際に出た時だけ、`control_epoch` 相当の first reopen cut を比較する
- final consistency catalog を MECE に寄せる判断はまだ保留のまま残す

#### いま自走できる理由

- current package 自体は close したので、maintenance と later comparison だけなら手戻りが小さい
- final activation / authority / fairness catalog は user 仕様確認前に固定しない運用が成立している

#### 重さ

- 中

#### rough 所要

- 0〜3 task / later pressure 時のみ

#### 現在の推奨度

- **checkpoint maintenance / later reopen candidate**

---

### Task C. detached validation loop の maintenance residual

#### 目的

- bundle / aggregate / static gate の emit / compare / smoke を維持する
- policy-dependent residual と self-driven helper 改善を分ける

#### current checkpoint

- current self-driven friction reduction は checkpoint close 済み
- residual は `reference update / bless` と retention / path policy 接続だけである

#### 残すもの

- `reference update / bless` を helper に入れるか
- retention / overwrite / path policy とどう接続するか

#### 重さ

- 低

#### rough 所要

- 0〜1 task / 必要時のみ

#### 現在の推奨度

- **maintenance mode**

---

### Task D. cross-phase drift suppression / checkpoint sweep

#### 目的

- docs / helper / plan / progress / tasks / reports の mirror drift を抑える

#### いま自走できる理由

- checkpoint close ごとの closeout package として運用が安定している

#### 重さ

- 低〜中

#### rough 所要

- 各 checkpoint ごとに 0.5〜1日

#### 現在の推奨度

- **常時必要**

## 方針決定が必要で、いま研究の障害になっているもの

### Blocker 1. shared-space の final activation rule

#### 概要

authoritative room の current baseline では `authority-ack` を first choice にしているが、final activation rule をそれで固定するか、overlay 可能な policy family をどう残すかが未決である。

#### 何に影響するか

- shared-space final catalog
- admission / activation / reconciliation の semantics
- authority handoff や auth layering との接続

#### 主要な選択肢

1. `authority-ack` を final default に寄せる
2. `authority-ack` を baseline にしつつ、`full-coverage-like` / `quorum-like` を overlay policy として残す
3. 最初から activation family を広い catalog として language 側に出す

#### current recommendation / 見解

- **2 を推奨**
- current baseline は `authority-ack` で進めてよいが、final activation family はまだ固定しない
- control-plane / auth / operational realization の研究が進んでから narrow に再比較する方が自然である

---

### Blocker 2. authority placement の final shape

#### 概要

current baseline では `single room authority` を room-level authoritative owner slot / write authority slot の first choice にしているが、final shape を single-authority に固定するか、replicated / relaxed projection authority をどこまで language 側へ出すかが未決である。

#### 何に影響するか

- exclusive action や global reset の semantics
- fairness / RNG provider placement
- read-mostly / fan-out state の model

#### 主要な選択肢

1. `single room authority` を final default に寄せる
2. `single room authority` を baseline にしつつ、replicated / relaxed authority を later option に残す
3. authority placement を room profile catalog の同格 row として早く広げる

#### current recommendation / 見解

- **2 を推奨**
- current package は `single room authority` baseline で十分 practical であり、replicated / relaxed authority は later pressure が出たときに reopen する方が理論上きれいである

---

### Blocker 3. consistency mode catalog をどこまで language 側に持つか

#### 概要

working subset として `authoritative_serial_transition` / `append_friendly_room` までは切ったが、final catalog をどこまで MECE に寄せ、どこまで room library / policy family に残すかが未決である。

#### 何に影響するか

- shared-space の表現力
- proof burden
- practical example の書き味

#### 主要な選択肢

1. small catalog を長く維持する
2. working subset を保ちつつ final catalog へ narrow expansion する
3. early に broad catalog を language 側へ出す

#### current recommendation / 見解

- **1 から 2 へ narrow に進む**のを推奨
- 今は working subset と stop line の cut を守り、MECE な final catalog は later research に残す方が自然である

---

### Blocker 4. fairness / RNG trust model

#### 概要

`authority_rng`、`delegated_rng_service`、`auditable_authority_witness`、provider attachment、distributed randomness provider をどの順で catalog に入れるかが未決である。

#### 何に影響するか

- shared-space fairness claim
- audit / replay
- delegated provider / auth layering
- control-plane carrier reopen 条件

#### 主要な選択肢

1. `authority_rng` を default に固定し、残りは library / provider policy に残す
2. `authority_rng` baseline を維持しつつ、`delegated_rng_service` と `auditable_authority_witness` を別軸で narrow に進める
3. delegated provider / distributed randomness を早く room core へ上げる

#### current recommendation / 見解

- **2 を推奨**
- provider placement と witness requirement を別軸に保つ current line が最も理論的にきれいである
- distributed randomness は default にしない

---

### Blocker 5. control-plane separated causal carrier の final actualization

#### 概要

current threshold judgment では、`membership_epoch + member_incarnation` を維持し、authority handoff / provider binding / activation frontier が room rule 側へ入る時だけ `control_epoch` 相当の split へ reopen する line にしているが、その final shape はまだ未決である。

#### 何に影響するか

- stale action rejection
- authority handoff / provider binding compare
- audit / replay
- exported artifact に載せる carrier の粒度

#### 主要な選択肢

1. membership-only carrier を長く維持する
2. `control_epoch` 相当の lightweight split を first reopen cut にする
3. full control-plane log / ref bundle を早く current core に入れる

#### current recommendation / 見解

- **1 を current default、2 を first reopen cut** とするのを推奨
- 3 は current phase では重すぎる

---

### Blocker 6. final parser grammar / public checker boundary

#### 概要

Phase 3 reserve path は freeze しているが、final parser grammar と public checker boundary は未固定である。

#### 何に影響するか

- syntax の public surface
- type / checker の entry criteria
- implementation actualization の timing

#### 主要な選択肢

1. companion notation を長く維持する
2. minimal parser subset だけ narrow に public 化する
3. broad grammar / public checker API を早く固定する

#### current recommendation / 見解

- **1 から 2 へ narrow に進む**のを推奨
- current promoted line ではなく、later pressure が出たときだけ reopen するのが自然である
