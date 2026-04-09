# tasks

最終更新: 2026-04-09 20:53 JST

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
- Phase 5 は、`specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`、`specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`、`specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md` までで **small decidable core / proof / async-control boundary の third inventory package** を切り終え、checkpoint maintenance と concrete consumer pressure 待ちの later reopen candidate に移った。
- Phase 3 は current checkpoint では **reserve path** であり、later pressure が出たときだけ reopen 候補にする。

## 次に自走で進める順番と rough estimate

| 順番 | phase | task package | 主眼 | rough weight | rough 所要 | 自走可否 | 備考 |
|---|---|---|---|---|---|---|---|
| 1 | cross-phase checkpoint | drift suppression / mirror sweep | Phase 4 / 5 package close 後の mirror drift を抑える | 低〜中 | 各 checkpoint ごとに 0.5〜1日 | 自走可能 | closeout package。Phase 5 third package close 後の first maintenance line |
| 2 | Phase 5 checkpoint 後半 / later reopen 候補 | concrete external consumer pressure comparison | mixed row default を維持したまま、boundary-specific handoff artifact や actual emitter が本当に要る条件を narrow に比べる | 中〜重 | 0〜2 task | 一部自走可能 | `specs/examples/126...` から `128...` までで current package は close。reopen は actual checker / proof / protocol consumer pressure が出たとき |
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

#### この task で残ること

- first concrete external consumer がどの boundary から現れるか
- stable `evidence_refs` family をどこまで actual artifact ref に寄せるか
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
