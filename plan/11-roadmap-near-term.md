# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 task 先までの near-term roadmap を示す。
step 数や task 数は厳密な約束ではなく、**rough estimate** である。

## current reading

- legacy checkpoint としては、Phase 1〜5 closeout / freeze は fixed 済みである。
- legacy Phase 6 front-half compile-ready checkpoint も fixed 済みである。
- current macro phase mainline は、`Macro 6` shared-space docs-first follow-up checkpoint を `specs/examples/375...376` までで一旦閉じたうえで、`specs/examples/377...384` の sequencing judgment / first actual carrier / emitted route / sample-facing summary package に従い、sample-visible theorem/model-check milestone を閉じ、その後に `Macro 7` docs-first I/O / host-facing port boundary を current line として進める line である。
- `Macro 7` は public-surface later ordering を already 固めており、その次段の host-facing integration を `docs-first I/O / host-facing port boundary`（working label）として current line に置くのが current reading である。

## immediate execution order

### 1. docs-first I/O / host-facing port boundary comparison（working label）

- macro phase
  - `Macro 7`
- rough weight
  - 中
- rough 所要
  - 1〜3 task / 数日〜1週

### 2. stable malformed missing-option first reopen actualization comparison

- macro phase
  - `Macro 4`
- rough weight
  - 中
- rough 所要
  - 1〜3 task / 数日〜1週

## next reserve line

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

- current near-term では、public operational surface inventory と later orderingを既存固定点として尊重しつつ、shared-space docs-first follow-up の checkpoint close を保持したまま、その後に theorem/model-check side の **sample-visible milestone** を `specs/examples/381...384` で閉じ、その先の docs-first host-facing boundary を進める。
- sample-visible milestone の current package は
  - emitted route actualization
  - sample-facing theorem/model-check evidence summary and bless/review flow
  の 2 本に分けるのが最小であり、`specs/examples/381...384` で close 済みである。current line は docs-first I/O / host-facing port boundary である。
- `stdin/stdout` の privileged primitive 化や raw FFI actualization を mainline に直接混ぜず、host-facing integration は docs-first I/O / adapter boundary として別 gate に置く。
- backend や higher-level async-control family を current executable core に早く混ぜない。
