# tasks

最終更新: 2026-04-11 15:29 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- `progress.md` が rough progress snapshot なのに対し、ここでは
  - ある程度まとまった task として自走できるもの
  - 方針決定が必要で、いま研究の障害になっている blocker
  を少し具体的に整理する。
- 規範判断の正本は `specs/`、長期比較と repository memory は `plan/`、作業履歴の正本は `docs/reports/` である。
- append で履歴を積まず、**毎回 current snapshot に合わせて全体を書き直す**。
- 冗長な列挙は避け、long chain は current checkpoint と reopen point が分かる粒度に圧縮する。

## 現在の読み

- **Phase 0 / 1 / 2** は maintenance tail である。current L2 semantics、parser-free PoC、detached validation loop の mainline は安定しており、drift suppression と residual maintenance が主眼である。
- **Phase 3** は reserve path である。private staged spike / reconnect freeze threshold までは source-backed に揃っているが、current promoted line ではなく、later pressure が出たときだけ reopen する。
- **Phase 4** は `specs/examples/121...125` までで current package close である。authoritative room baseline、working subset row、minimal witness core、delegated-provider practical cut、control-plane threshold comparison までは source-backed に固まっている。
- **Phase 5** は `specs/examples/126...268` までで current package close である。theorem-line retained bridge は `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body` を stop line にし、low-level memory-order family は still later に残す。checker-side では minimal checker-cluster row core、`fixture_evidence_refs` attachment、optional `typed_reason_family_hint` attachment、`family_refs[]` minimal bundle、lightweight `coverage_state` まで source-backed に切り、`supported kind` summary は current matrix に足さない。actual checker payload family は `payload_family_kind + source_refs`、checker payload row family は `payload_family_ref + row_family_kind`、checker payload row detail は `payload_row_family_ref + row_source_ref + row_reason_kind` minimal bundle まで docs-first に切れており、current promoted line は **`minimal-checker-payload-row-detail-ready checker-payload-row-body comparison`** に置いている。
- 現在の主線は、**Phase 5 later reopen の narrow comparison** と **cross-phase checkpoint maintenance** の 2 本である。

## 次に自走で進める順番と rough estimate

| 順番 | phase | task package | 主眼 | rough weight | rough 所要 | 自走可否 | current 読み |
|---|---|---|---|---|---|---|---|
| 1 | Phase 5 後半 | minimal-checker-payload-row-detail-ready checker-payload-row-body comparison | checker-cluster matrix line を `coverage_state` で止め、actual checker payload family を `payload_family_kind + source_refs`、checker payload row family を `payload_family_ref + row_family_kind`、checker payload row detail を `payload_row_family_ref + row_source_ref + row_reason_kind` まで切った current package を前提に、checker payload row body をどこまで docs-first に切るかを narrow に整理する | 中 | 1〜3 task / 2〜5日 | 自走可能 | **current promoted line** |
| 2 | cross-phase checkpoint | drift suppression / mirror sweep | `specs/` / `plan/` / `progress.md` / `tasks.md` / research abstract の drift を抑える | 低〜中 | 0.5〜1日 / checkpoint ごと | 自走可能 | closeout package |
| 3 | Phase 4 後半 reserve path | shared-space later reopen | `control_epoch` first reopen cut、catalog refinement、provider / witness / authority の later compare | 中〜重 | 0〜3 task | 一部自走可能 | concrete pressure が出たときだけ promoted |
| 4 | Phase 2 maintenance tail | detached validation loop residual | bless / update / retention policy の policy-dependent residual を narrow に扱う | 低 | 0〜1 task | 自走可能 | drift または practical need 時のみ |
| 5 | Phase 3 reserve path | parser boundary / public checker reopen | final grammar / public checker へ寄せずに reserve path を narrow に reopen する | 中〜重 | 0〜2 task | 後段依存 | later pressure 時のみ |

## 自走で進められる task package

### Task A. Phase 5 checker-cluster matrix / evidence attachment line

#### 目的

- small decidable core / proof boundary inventory の次段として、first checker cut の 6 cluster を docs-only matrix へ整理する。
- public checker API や final type system に早く落とさず、checker-side inventory を narrow に見える化する。

#### current checkpoint

- `specs/examples/126...127` で 4-way split と proof-obligation matrix を集約済み。
- `specs/examples/128...134` で mixed row default、consumer pressure order、minimum contract row、first consumer class を固定済み。
- `specs/examples/135...255` で theorem-line retained bridge を段階的に伸ばし、現時点では `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body` を stop line にし、low-level memory-order family は still later に残す current first choice を固定済み。
- `specs/examples/256` で Phase 5 の next line を theorem-line の先ではなく small decidable core / checker-cluster matrix side に戻した。
- `specs/examples/257` で minimal checker-cluster row core を `cluster_kind + checker_subject + decidable_class + external_handoff` に留める current first choice を固定した。
- `specs/examples/258` で `fixture_evidence_refs` を row core ではなく attachment として足す current first choice を固定した。
- `specs/examples/259` で `typed_reason_family_hint` を row core ではなく optional attachment に留める current first choice を固定した。
- `specs/examples/260` で `typed_reason_family_hint` payload を `family_refs[]` minimal bundle に留める current first choice を固定した。
- `specs/examples/261` で multi-family row の誤読を抑える lightweight `coverage_state` を current checker-side line に足す current first choice を固定した。
- `specs/examples/262` で `supported kind` summary を current checker-cluster matrix に足さず、`coverage_state` で止める current first choice を固定した。
- `specs/examples/263` で checker-cluster matrix line の次段として actual checker payload family を docs-first bridge として 1 段切る current first choice を固定した。
- `specs/examples/264` で actual checker payload family の minimal shape を `payload_family_kind + source_refs` に留める current first choice を固定した。
- `specs/examples/265` で actual checker payload family の次段として checker payload row family を docs-first line に切り、`payload_family_ref + row_family_kind` を current first choice に固定した。
- `specs/examples/266` で checker payload row family の minimal shape を `payload_family_ref + row_family_kind` に留める current first choice を固定した。
- `specs/examples/267` で checker payload row family の次段として checker payload row detail を docs-first line に切り、`payload_row_family_ref + row_source_ref + row_reason_kind` を current first choice に固定した。
- `specs/examples/268` で checker payload row detail の minimal shape を `payload_row_family_ref + row_source_ref + row_reason_kind` に留める current first choice を固定した。

#### 次に扱うべき narrow question

- checker payload row body を docs-first line としてどこまで切るべきか

#### いま自走できる理由

- current line は docs-only cluster matrix / row core / evidence attachment の threshold comparison に留まり、actual public checker API / final type system / actual theorem prover relation までは踏み込んでいない。
- `specs/09` の invariants と Phase 3 reserve path の freeze line を壊さずに進められる。

#### 重さ / rough 所要

- 重さ: 中
- rough 所要: 1〜3 task / 2〜5日

#### 現在の推奨度

- **current promoted line**

---

### Task B. cross-phase checkpoint maintenance

#### 目的

- current checkpoint close 済み package が、`Documentation.md`、`progress.md`、`tasks.md`、`plan/`、research abstract で drift しないように保つ。
- current promoted line を誤読しにくい snapshot を維持する。

#### current checkpoint

- Phase 2 detached validation loop は checkpoint close。
- Phase 4 shared-space package は `specs/examples/121...125` で checkpoint close。
- Phase 5 current package は `specs/examples/126...268` で current package close。

#### いま自走できる理由

- 規範判断の追加ではなく mirror suppression なので、手戻りが小さい。

#### 重さ / rough 所要

- 重さ: 低〜中
- rough 所要: 0.5〜1日 / checkpoint ごと

#### 現在の推奨度

- **常時 maintenance**

---

### Task C. Phase 4 later reopen candidate

#### 目的

- shared-space / membership の current working subset を壊さずに、later pressure が出たときだけ stronger split や finer catalog を比較する。

#### current checkpoint

- authoritative room baseline は `authority-ack + single room authority + authoritative serial transition + authority_rng` を baseline に固定済み。
- working subset row、minimal witness core、delegated-provider practical cut、control-plane threshold comparison まで source-backed に整理済み。

#### 次に扱うべき narrow question

- authority handoff / provider binding / activation frontier を room rule 側に上げる pressure が本当に出たか
- final catalog を working subset の外へ広げる必要があるか

#### いま自走できる理由

- docs-first comparison と practical example の範囲なら自走できる。
- final activation / final authority / final auth / final consistency catalog の固定はまだ避ける。

#### 重さ / rough 所要

- 重さ: 中〜重
- rough 所要: 0〜3 task

#### 現在の推奨度

- **later reopen candidate**

---

### Task D. Phase 2 residual maintenance

#### 目的

- detached validation loop の residual policy-dependent helper を、必要が出たときだけ narrow に扱う。

#### current checkpoint

- emit / save / compare / smoke / shallow triage / aggregate compare convenience は成立済み。
- `reference update / bless` は final path policy / retention policy と接続する later candidate に残している。

#### 重さ / rough 所要

- 重さ: 低
- rough 所要: 0〜1 task

#### 現在の推奨度

- **必要時のみ**

## 方針決定が必要で、いま研究の障害になっているもの

### Blocker 1. shared-space の final activation rule

#### 概要

authoritative room の current baseline では `authority-ack` を first choice にしているが、final activation rule を単一 profile として固定するか、overlay 可能な activation family として残すかが未決である。

#### 何に影響するか

- shared-space final catalog
- activation / reconciliation / admission policy の semantics
- auth layering と runtime control-plane の責務分担

#### 主要な選択肢

1. `authority-ack` を final default に寄せる
2. `authority-ack` を baseline にしつつ、`full-coverage-like` / `quorum-like` を overlay policy として残す
3. activation family を早く広い language catalog として固定する

#### current recommendation / 見解

- **2 を推奨**
- current baseline は `authority-ack` で十分だが、final family はまだ固定しない。
- 可能な限り overlay / runtime policy / auth layering と直交させ、language core へ早く焼き込まない方が理論上きれいである。

---

### Blocker 2. authority placement の final shape

#### 概要

current baseline では `single room authority` を room-level authoritative owner slot / write authority slot の first choice にしているが、replicated / relaxed authority をどこまで later option として残すかが未決である。

#### 何に影響するか

- exclusive action、global reset、lock / commit ownership
- fairness / RNG provider placement
- read-mostly resource、fan-out state、delegated capability

#### 主要な選択肢

1. `single room authority` を final default に寄せる
2. `single room authority` を baseline にしつつ、replicated / relaxed authority を later option に残す
3. authority placement を room profile catalog の同格 row として早く広げる

#### current recommendation / 見解

- **2 を推奨**
- いまは owner slot / delegated capability を分けたまま `single room authority` baseline で十分 practical である。
- replicated / relaxed authority は、read-mostly / fan-out / failover pressure が concrete になってから reopen する方が理論的にきれいである。

---

### Blocker 3. consistency mode catalog をどこまで language 側に持つか

#### 概要

current working subset として `authoritative room` と `append-friendly room` は切れているが、final catalog をどこまで MECE に寄せ、どこまで room library / policy family に残すかは未決である。

#### 何に影響するか

- shared-space の表現力
- proof burden と compare burden
- practical example の書き味

#### 主要な選択肢

1. small working subset を長く維持する
2. working subset を保ちながら final catalog へ narrow expansion する
3. early に broad catalog を language 側へ出す

#### current recommendation / 見解

- **1 から 2 へ narrow に進む**のを推奨
- 今は MECE な final catalog よりも、working subset と stop line を source-backed に保つ方が重要である。

---

### Blocker 4. fairness / RNG trust model

#### 概要

`authority_rng`、`delegated_rng_service`、`auditable_authority_witness`、provider attachment、distributed randomness provider をどの順で整理するかが未決である。

#### 何に影響するか

- shared-space fairness claim
- audit / replay
- delegated provider / trust boundary
- consistency mode catalog との接続

#### 主要な選択肢

1. `authority_rng` を default に固定し、残りは library / provider policy に残す
2. `authority_rng` baseline を維持しつつ、`delegated_rng_service` と `auditable_authority_witness` を別軸で narrow に進める
3. delegated provider / distributed randomness を早く room core に上げる

#### current recommendation / 見解

- **2 を推奨**
- current baseline は `authority_rng` で十分簡潔であり、provider placement と witness requirement を別軸に保つ方が理論的にきれいである。
- distributed randomness は default にしない。

---

### Blocker 5. control-plane separated causal carrier の final actualization

#### 概要

current threshold judgment では `membership_epoch + member_incarnation` を default にし、authority handoff / provider binding / activation frontier が room rule 側に上がるときだけ `control_epoch` 相当の split を reopen する line にしているが、final actualization は未決である。

#### 何に影響するか

- stale action rejection
- authority handoff / provider binding compare
- audit / replay
- exported artifact に載せる causal carrier の粒度

#### 主要な選択肢

1. membership-only carrier を長く維持する
2. `control_epoch` 相当の lightweight split を first reopen cut にする
3. full control-plane log / ref bundle を早く current core に入れる

#### current recommendation / 見解

- **1 を current default、2 を first reopen cut** とするのを推奨
- これは plain vector deletion を正本にするのではなく、membership change と causal metadata を分けるための current practical line である。
- 3 は current phase では重すぎる。

---

### Blocker 6. final parser grammar / public checker boundary

#### 概要

Phase 3 reserve path は freeze しているが、final parser grammar と public checker boundary は未固定である。

#### 何に影響するか

- syntax の public surface
- type / checker の entry criteria
- actual parser / checker / runtime commitment の timing

#### 主要な選択肢

1. companion notation を長く維持する
2. minimal parser subset だけ narrow に public 化する
3. broad grammar / public checker API を早く固定する

#### current recommendation / 見解

- **1 から 2 へ narrow に進む**のを推奨
- current promoted line ではなく、later pressure が出たときだけ reopen するのが自然である。
