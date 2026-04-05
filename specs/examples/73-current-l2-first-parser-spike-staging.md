# 73 — current L2 first parser spike staging

## 目的

この文書は、`specs/examples/29-current-l2-first-parser-subset-inventory.md` で棚卸しした
first parser cut 候補 cluster を前提に、
**actual parser spike を切るならどの cluster から staged に入れるべきか**
を narrow に比較する。

ここで固定するのは final parser grammar ではない。
固定するのは、

- full inventory は維持したまま
- actual parser spike は monolithic にせず
- current checker / validation loop に最も近い cluster から staged に入れる

という sequencing judgment だけである。

## 前提

- current L2 の core semantics は変更しない。
- `specs/examples/29-current-l2-first-parser-subset-inventory.md` の inventory 自体は維持する。
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` の first checker cut inventory も維持する。
- final token、final punctuation、A2 / A1 の exact grammar choice は引き続き OPEN である。

## 現在の 3 cluster

### 1. chain / declaration structural floor

この cluster は、少なくとも次を含む。

- option declaration core
- chain declaration の explicit edge-row family
- edge-local lineage metadata
- declaration-side guard slot

current source-backed evidence は次である。

- static gate anchor:
  - `crates/mir-semantics/src/lib.rs` の `static_gate_detailed`
- current fixture corpus:
  - `e4-malformed-lineage`
  - `e12-underdeclared-target-missing`
  - `e13-malformed-capability-strengthening`
  - `e16-malformed-missing-chain-head-option`
  - `e17-malformed-missing-predecessor-option`
  - `e20-malformed-late-capability-strengthening`
- helper:
  - `scripts/current_l2_same_lineage_checker.py`
  - `scripts/current_l2_missing_option_checker.py`
  - `scripts/current_l2_capability_checker.py`
  - `scripts/current_l2_detached_loop.py` の family-specific smoke command

### 2. `try` / rollback structural floor

この cluster は、少なくとも次を含む。

- `try { ... } fallback { ... }`
- `atomic_cut`

current source-backed evidence は次である。

- static structural anchor:
  - `crates/mir-semantics/src/lib.rs` の `collect_try_rollback_structural_reasons`
- malformed static fixture:
  - `e23-malformed-try-fallback-missing-fallback-body`
  - `e24-malformed-atomic-cut-fallback-placement`
- runtime representative:
  - `e21-try-atomic-cut-frontier`
  - `e22-try-atomic-cut-place-mismatch`
- helper:
  - `scripts/current_l2_try_rollback_structural_checker.py`
  - `scripts/current_l2_detached_loop.py` の `smoke-try-rollback-structural-checker`

### 3. request / admissibility cluster

この cluster は、少なくとも次を含む。

- `perform <op> on <target>`
- `perform <op> via <chain_ref>`
- statement-local `require` / `ensure`
- option-local `admit`
- request / admissibility 側で使う最小 predicate fragment

current source-backed evidence は次である。

- runtime fixture:
  - `e3-option-admit-chain`
  - `e10-perform-on-ensure-failure`
  - `e11-perform-via-ensure-then-success`
- runtime / bundle / batch anchor:
  - `crates/mir-semantics/src/harness.rs` の `run_bundle`
  - `crates/mir-semantics/src/harness.rs` の `batch_summary_from_discovery`

この cluster は first parser cut 候補としては自然だが、
current phase では first checker cut への直結が上の 2 cluster より弱い。

## 比較する 3 案

### 案 1. monolithic first parser spike

first parser cut inventory の cluster を 1 本の parser spike へまとめて入れる。

#### 利点

- parser inventory と actual parser spike の差が少ない。
- representative examples の広い面を一度に扱える。

#### 欠点

- lexical choice を早く固定しやすい。
- predicate fragment、request head、admissibility、rollback shape を一度に抱えるため、
  current repo の narrow progression と相性が悪い。
- checker / theorem prover / runtime boundary の handoff を観測しにくい。

### 案 2. checker-led staged spike

actual parser spike を次の順に切る。

1. chain / declaration structural floor
2. `try` / rollback structural floor
3. request / admissibility cluster

#### 利点

- current helper stack と static gate evidence に最も近い。
- first checker cut と theorem prover boundary への handoff を保ちやすい。
- request-side syntax や richer predicate grammar を早く凍らせずに済む。
- `TryFallback` / `AtomicCut` の structural floor を、generic/public checker pressure が出る前に parser boundary 側へ narrow に接続できる。

#### 欠点

- representative examples の user-facing surface を全部すぐには parse しない。
- runtime-heavy cluster は第 3 段に回るため、parser 実装の前進は控えめに見える。

### 案 3. runtime-led staged spike

actual parser spike を次の順に切る。

1. request / admissibility cluster
2. chain / declaration structural floor
3. `try` / rollback structural floor

#### 利点

- `perform` / `require` / `ensure` / `admit` という user-facing surfaceに早く触れられる。
- runtime fixture `e3` / `e10` / `e11` とのつながりは見せやすい。

#### 欠点

- first checker cut との handoff が弱い。
- predicate fragment と clause attachment の lexical pressureが先に高まる。
- option / chain / rollback を後回しにすると、
  current repo の「small decidable core を先に切る」順序から外れやすい。

## 比較

### current helper stack との整合

- 案 1:
  一度に多くを抱え込みすぎる。
- 案 2:
  same-lineage / missing-option / capability という current checker family と、
  `TryFallback` / `AtomicCut` helper-local first tranche に素直に接続できる。
- 案 3:
  runtime evidence には近いが、checker family と parser spike の handoff が遠い。

### first checker cut との handoff

- 案 1:
  可能だが過剰。
- 案 2:
  最も自然。
- 案 3:
  request/runtime evidence は強いが、checker-led ratchet には弱い。

### lexical freeze pressure

- 案 1:
  最も高い。
- 案 2:
  declaration / structural role から入れるので最も抑えやすい。
- 案 3:
  clause と predicate fragment を先に抱えるため高い。

### theorem prover / model checker 境界との整合

- 案 1:
  parser spike と heavy workstream の準備が混ざりやすい。
- 案 2:
  local / structural floor を先に parse 対象化しやすい。
- 案 3:
  request/runtime surface は見やすいが、proof boundary の narrow cut としては弱い。

## current judgment

current repo の next narrow step としては、**案 2. checker-led staged spike** が最も自然である。

つまり、

1. first parser cut inventory 自体は維持する
2. ただし actual parser spike を切るなら monolithic にはしない
3. current actual checker / validation loop に最も近い cluster から staged に進める

という順序を current judgment とする。

### stage 1 に送る cluster

- option declaration core
- explicit edge-row family
- edge-local lineage metadata
- declaration-side guard slot

この段では、
current same-lineage / missing-option / capability floor へ最も近い parser spike を優先する。
guard slot の存在までは stage 1 に含めてよいが、
predicate fragment 自体の parse / well-formedness は stage 3 以降へ残す。

### stage 2 に送る cluster

- `try { ... } fallback { ... }`
- `atomic_cut`

この段では、
`TryFallback` / `AtomicCut` の structural malformed source と runtime contrast evidence を parser boundary 側へ接続する。
ただし restore scope や dynamic gate は runtime / proof boundary に残す。

### stage 3 に送る cluster

- `perform on`
- `perform via`
- statement-local `require` / `ensure`
- option-local `admit`
- request / admissibility 側の最小 predicate fragment

この段では、
runtime surface への parser 接続を進めるが、richer predicate grammar や final lexical polish はまだ固定しない。

## 何をまだ決めないか

- final parser grammar
- accepted token set
- A2 / A1 の exact concrete accept set
- `perform` / `fallback` / `admit` / `require` / `ensure` の reserved keyword 最終確定
- predicate sublanguage の完成形
- parser cut と elaboration helper の actual API 結合

## current meaning

- `specs/examples/29-current-l2-first-parser-subset-inventory.md` の full inventory は変えない
- current next task で決めてよいのは inventory の staging 順だけである
- parser spike を始めるなら checker-led staged spike が自然であり、
  runtime-led or monolithic spike は current phase では採らない
- stage 1 の accepted parse cluster / non-goal 境界は、
  `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
  で declaration-side guard slot を opaque attached slot に留める cut として narrow に固定する
