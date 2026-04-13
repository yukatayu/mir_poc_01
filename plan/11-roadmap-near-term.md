# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 task 先までの near-term roadmap を示す。
step 数や task 数は厳密な約束ではなく、**rough estimate** である。

## current reading

- legacy checkpoint としては、Phase 1〜5 closeout / freeze は fixed 済みである。
- legacy Phase 6 front-half compile-ready checkpoint も fixed 済みである。
- current macro phase mainline は、`Macro 6` shared-space docs-first follow-up checkpoint を `specs/examples/375...376` までで一旦閉じたうえで、`specs/examples/377...384` の sequencing judgment / first actual carrier / emitted route / sample-facing summary package に従い、sample-visible theorem/model-check milestone を閉じ、`specs/examples/385...386` で `Macro 7` の docs-first I/O / host-facing port boundary を固定した後、その次段として `Macro 4` missing-option actualization comparison を current line に置く line である。
- `Macro 7` は public-surface later ordering と docs-first host-facing boundary を already 固めており、その次段の public-side current line は final public parser / checker / runtime first later gate actualization comparison に残る。

## immediate execution order

### 1. stable malformed missing-option first reopen actualization comparison

- macro phase
  - `Macro 4`
- rough weight
  - 中
- rough 所要
  - 1〜3 task / 数日〜1週

### 2. final public parser/checker/runtime first later gate actualization comparison

- macro phase
  - `Macro 7`
- rough weight
  - 中
- rough 所要
  - 1〜3 task / 数日〜1週

## next reserve line

### final public parser/checker/runtime first later gate actualization comparison

- `Macro 7`
- public-side later ordering fixed 後の次段として、library-side final public contract を symbol-level でどう narrow に actualize するかを整理する。

### public operational CLI second later gate actualization comparison

- `Macro 7`
- library-side final public contract actualization の後段として、public operational CLI と repo-local helper / example surface の境界をどう narrow に actualize するかを整理する。

## later gates

- shared-space final catalog
- upper-layer application target
- backend / public operational surface success criteria
- visualizer / host substrate / FFI / game engine adapter の first target profile

## current recommendation

- current near-term では、public operational surface inventory と later orderingを既存固定点として尊重しつつ、shared-space docs-first follow-up checkpoint と sample-visible theorem/model-check milestone を保持したまま、`specs/examples/385...386` で docs-first host-facing boundary を close 済みと読む。current line は missing-option actualization comparison である。
- `stdin/stdout` の privileged primitive 化や raw FFI actualization を mainline に直接混ぜず、host-facing integration は `specs/examples/385...386` の docs-first I/O / adapter boundary として separate gate に置く。
- backend や higher-level async-control family を current executable core に早く混ぜない。
