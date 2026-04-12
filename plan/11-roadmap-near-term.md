# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 task 先までの near-term roadmap を示す。
step 数や task 数は厳密な約束ではなく、**rough estimate** である。

## current reading

- legacy checkpoint としては、Phase 1〜5 closeout / freeze は fixed 済みである。
- legacy Phase 6 front-half compile-ready checkpoint も fixed 済みである。
- current macro phase mainline は `Macro 4` executable fixed-subset sample expansion と `Macro 5` theorem / verifier bridge である。

## immediate execution order

### 1. parser / checker / runtime public surface inventory

- macro phase
  - `Macro 3-7`
- rough weight
  - 中
- rough 所要
  - 2〜4 task / 1〜2週

## next reserve line

### Mirrorea / shared-space docs-first re-entry

- `Macro 6`
- old `FutureWork` bucket に戻さず、fabric / shared-space line を独立 track として再開する。

### model-check / public-checker second reserve inventory

- `Macro 5`
- proof notebook first concrete cut の後で machine-facing carrier をどこから reopen するかを整理する。

### stable-static edge-pair first reopen

- `Macro 4`
- stable reason-code / fixture-static cluster selection を保ったまま、`e4` / `e19` edge-pair sideの actual reopen を narrow に進める。

## later gates

- shared-space final catalog
- upper-layer application target
- backend / public operational surface success criteria

## current recommendation

- current near-term では、proof/model-check first concrete pilot、second source-sample cluster sequencing、actual `e22` contrast-row source actualization、stable static malformed post-contrast sequencingを fixed entry criteria に閉じたうえで、public surface inventory と shared-space docs-first re-entry を staged に進める line を優先する。
- backend や higher-level async-control family を current executable core に早く混ぜない。
