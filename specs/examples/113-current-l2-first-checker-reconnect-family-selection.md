# 113 — current L2 first checker reconnect family selection

## 目的

この文書は、`specs/examples/112-current-l2-phase3-resume-side-selection.md` で
Phase 3 の主線を parser boundary staging 側ではなく
first checker cut connection 側へ戻すと整理したことを前提に、
**existing parser-boundary evidence family のうち、どれを first checker cut inventory へ最初に reconnect するのが最小か**
を比較する。

ここで固定するのは final checker API ではない。
固定するのは、

- どの parser-boundary family が current checker inventory に最も近いか
- syntax pressure を上げずに actual evidence を増やせるか
- next actualization を helper-local / test-only cut に留められるか

という sequencing judgment だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` の
  first checker cut inventory は維持する。
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md` の
  same-lineage / missing-option / capability floor baseline は維持する。
- `specs/examples/73-current-l2-first-parser-spike-staging.md` の
  checker-led staged spike judgmentも維持する。
- request contract subset family は current tranche で freeze したままにする。

## 比較する 3 案

### 案 1. stage 1 chain / declaration structural floor family

次を first reconnect family として扱う。

- option declaration core
- explicit edge-row family
- edge-local lineage metadata
- declaration-side guard slot

#### 利点

- `specs/examples/73...` の stage 1 cluster と一致する。
- `specs/examples/45...` の same-lineage / missing-option / capability floor baseline と自然に接続する。
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs` と
  `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
  に actual parser-side evidence が already ある。
- clause / predicate / request head をまだ持ち込まなくて済む。

#### 欠点

- `missing_predecessor_option` のように、current stage 1 surface ではまだ直接表しにくい malformed family が残る。
- reconnect first tranche でも family 全体を一度には覆い切らない可能性がある。

### 案 2. `try` / rollback structural floor family

次を first reconnect family として扱う。

- `try { ... } fallback { ... }`
- `atomic_cut`

#### 利点

- `specs/examples/30...` では first checker cut 候補 cluster に already 入っている。
- `specs/examples/73...` でも stage 2 cluster として独立している。

#### 欠点

- current parser-side actual evidence は stage 1 family より薄い。
- runtime contrast と structural malformed floor が近く、first reconnect としては proof / runtime boundary の説明が少し増える。

### 案 3. request-local clause attachment / minimal predicate family

次を first reconnect family として扱う。

- request-local `require` / `ensure`
- option-local `admit`
- minimal predicate fragment

#### 利点

- recent Phase 3 later branch で actual helper family が厚くなっている。
- user-facing surface には近い。

#### 欠点

- `specs/examples/73...` でも stage 3 cluster に留まっている。
- clause attachment、predicate fragment、request head metadata の reopen 条件が still later に残る。
- syntax pressure が strongest になりやすい。

## 比較

### first checker cut inventory との近さ

- 案 1 は `same-lineage static evidence floor`、`minimal capability strengthening prohibition`、
  `missing-option structure floor` に一度に近い。
- 案 2 は `try` / rollback locality の structural floor と対応するが、candidate cluster は 1 family に留まる。
- 案 3 は request-local / option-local clause attachment と minimal predicate fragment に触るが、current phase では parser-side reopen 条件がまだ多い。

### parser-side actual evidence の厚さ

- 案 1 は stage 1 parser spike が already actualize 済みである。
- 案 2 は docs / helper / runtime representative はあるが、parser-side reconnect の actual evidence はまだ薄い。
- 案 3 は later branch の helper-local actualization は厚いが、first checker cut への直結は stage 1 / stage 2 より弱い。

### syntax pressure

- 案 1 は declaration / chain structural floor に留まるため最も抑えやすい。
- 案 2 は中程度である。
- 案 3 は clause / predicate / request head reopen pressure を受けやすい。

## current judgment

current repo の next narrow step として最も自然なのは、
**案 1. stage 1 chain / declaration structural floor family を first reconnect family にする**
ことである。

理由は次の通り。

1. `specs/examples/73...` の checker-led staged spike と順序が一致する。
2. `specs/examples/45...` の first checker cut baseline が same-lineage / missing-option / capability の 3 cluster を already source-backed にしている。
3. current parser-side actual evidence も stage 1 family が最も厚い。
4. request / clause family を先に reconnect すると syntax pressure が強くなりやすい。

## current meaning

- Phase 3 の checker-side reconnect は stage 1 chain / declaration structural floor family から始める。
- ただし stage 1 family 全体を一度に public checker API へ送るわけではない。
- next task では、stage 1 family のうち **どの first actualization tranche が最小か** をさらに narrow に比較する。

## current stage でまだやらないこと

- `try` / rollback family の reconnect actualization
- request-local clause attachment / predicate family の reconnect actualization
- public checker API 化
- final parser grammar 固定

## next narrow step

次段では、
**stage 1 chain / declaration structural floor family のうち、どの reconnect first tranche を actualize するのが最小か**
を narrow に比較するのが自然である。
