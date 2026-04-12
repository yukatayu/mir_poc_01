# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 task 先までの near-term roadmap を示す。
step 数や task 数は厳密な約束ではなく、**rough estimate** である。

## current reading

- legacy checkpoint としては、Phase 1〜5 closeout / freeze は fixed 済みである。
- legacy Phase 6 front-half compile-ready checkpoint も fixed 済みである。
- current macro phase mainline は `Macro 4` executable fixed-subset sample expansion と `Macro 5` theorem / verifier bridge である。

## immediate execution order

### 1. compare-ready bridge sketch second reopen

- macro phase
  - `Macro 5`
- rough weight
  - 中
- rough 所要
  - 1〜2 task / 数日

### 2. deferred `e3` actualization reopen timing

- macro phase
  - `Macro 4-5`
- rough weight
  - 中
- rough 所要
  - 1〜2 task / 数日

### 3. actual `e3` authored-row reopen

- macro phase
  - `Macro 4`
- rough weight
  - 中
- rough 所要
  - 1〜2 task / 数日

### 4. proof / model-check first concrete tool pilot

- macro phase
  - `Macro 5`
- rough weight
  - 中〜やや重い
- rough 所要
  - 1〜3 task / 1〜2週

## next reserve line

### second source-sample cluster sequencing

- `Macro 4`
- `e3` の後にどの sample family を widening するかを再整理する。

### parser / checker / runtime public surface inventory

- `Macro 3-7`
- current helper-local surface と later public surface の境界を inventory 化する。

## later gates

- shared-space final catalog
- upper-layer application target
- backend / public operational surface success criteria

## current recommendation

- current near-term では、settled subset を runnable / verifiable に厚くする line を優先する。
- backend や higher-level async-control family を current executable core に早く混ぜない。
