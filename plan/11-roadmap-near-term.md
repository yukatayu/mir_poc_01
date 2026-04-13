# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 task 先までの near-term roadmap を示す。
step 数や task 数は厳密な約束ではなく、**rough estimate** である。

## current reading

- legacy checkpoint としては、Phase 1〜5 closeout / freeze は fixed 済みである。
- legacy Phase 6 front-half compile-ready checkpoint も fixed 済みである。
- current macro phase mainline は、まず `Macro 6` shared-space follow-up の残り 1 package を閉じ、その後に `Macro 5` model-check concrete carrier actualization comparison を入口とする sample-visible theorem/model-check line（comparison + 3 package）を詰める line である。
- `Macro 7` は public-surface later ordering を already 固めており、その次段の host-facing integration は `docs-first I/O / host-facing port boundary`（working label）として後続に置くのが current reading である。

## immediate execution order

### 1. shared-space authority / resource ownership reopen

- macro phase
  - `Macro 6`
- rough weight
  - 中
- rough 所要
  - 1〜3 task / 数日〜1週

### 2. model-check concrete carrier actualization comparison

- macro phase
  - `Macro 5`
- rough weight
  - 中
- rough 所要
  - 1〜3 task / 数日〜1週

### 3. model-check concrete carrier first actualization

- macro phase
  - `Macro 5`
- rough weight
  - 中
- rough 所要
  - 1〜3 task / 数日〜1週

### 4. source-sample emitted verification artifact wiring

- macro phase
  - `Macro 5`
- rough weight
  - 中
- rough 所要
  - 1〜3 task / 数日〜1週

### 5. sample-facing theorem / model-check evidence summary and bless/review flow

- macro phase
  - `Macro 5`
- rough weight
  - 中
- rough 所要
  - 1〜3 task / 数日〜1週

## next reserve line

### docs-first I/O / host-facing port boundary comparison

- `Macro 7`
- capability-scoped port / adapter boundary、visualizer / host substrate、FFI、game engine adapter を docs-first に整理する。

### stable malformed missing-option first reopen actualization comparison

- `Macro 4`
- broader malformed inventory fixed 後の次段として、missing-option family first reopen を helper-local compare と source-backed wideningのどの順で actualize するかを整理する。

### final public parser / checker / runtime API first later gate actualization comparison

- `Macro 7`
- public-side later ordering fixed 後の次段として、library-side final public contract を symbol-level でどう narrow に actualize するかを整理する。

## later gates

- shared-space final catalog
- upper-layer application target
- backend / public operational surface success criteria
- visualizer / host substrate / FFI / game engine adapter の first target profile

## current recommendation

- current near-term では、public operational surface inventory と later orderingを既存固定点として尊重しつつ、shared-space docs-first follow-up の残り 1 package を先に閉じ、その後に theorem/model-check side を **sample-visible milestone** まで進める。
- sample-visible milestone の current package は
  - model-check concrete carrier first actualization
  - source-sample emitted verification artifact wiring
  - sample-facing theorem/model-check evidence summary and bless/review flow
  の 3 本に分けるのが最小である。
- `stdin/stdout` の privileged primitive 化や raw FFI actualization を mainline に直接混ぜず、host-facing integration は docs-first I/O / adapter boundary として別 gate に置く。
- backend や higher-level async-control family を current executable core に早く混ぜない。
