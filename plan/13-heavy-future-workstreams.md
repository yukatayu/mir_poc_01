# plan/13 — 重い将来 workstream

## 目的

この文書は、current executable / theory-lab line に即昇格させない heavy workstream を整理する。
detail-side で今まさに詰める線は `plan/18` と `specs/examples/458...469` に置き、ここでは **まだ重すぎるもの** をまとめる。

## heavy workstream 一覧

### 1. full strong type system

- 主題:
  ownership / lifetime / contract / effect / capability / modality をどこまで型規則へ持ち上げるか
- なぜ heavy か:
  parser / checker / theorem / runtime を同時に拘束するため
- current recommendation:
  checker attachment principal、source-visible structural marker family、stronger typed-surface threshold framing、multimodal dependent core research direction、layered theory stackまでは self-driven に進めるが、
  full calculus と final typed syntax はここに残す

### 2. production-grade theorem proving

- 主題:
  concrete prover binding、proof artifact contract、review workflow finalization
- なぜ heavy か:
  tool choice と public contract が逆流しやすいため
- current recommendation:
  semantic-core theorem pilot、notebook-first threshold、discharge-entry reserve、Lean-first proof roadmap、Lean-first non-production stub pilot、repo-local artifact-conformance bridge second、representative trace alignment third までは self-driven に進めるが、
  concrete prover / proof object finalization はここに残す

### 3. production-grade model checking / protocol verification

- 主題:
  concrete model-check tool binding、property language、protocol-family public contract
- なぜ heavy か:
  runtime policy / shared-space / public checker / host integration と強く絡むため
- current recommendation:
  row-local carrier、small-cluster projection reserve、later-gate framing までは self-driven に進めるが、
  production contract はここに残す

### 4. scheduler / memory-model / authority-handoff runtime finalization

- 主題:
  low-level memory-order family と higher-level authority-serial / witness-aware family の最終関係、
  scheduler / fairness / replay / authority handoff の operational semantics
- なぜ heavy か:
  local finalization / ordering / observation / commit を混同しやすく、shared-memory と distributed handoff の両方へ波及するため
- current recommendation:
  higher-level family comparison、relation decomposition、adequacy corpus、verifier-boundary matrix、wording reserve までは self-driven に進め、
  low-level source-surface adoption と runtime semantics finalization はここに残す

### 5. backend / codegen / editor surface

- 主題:
  LLVM-family backend、external codegen、editor / LSP / graph tooling
- なぜ heavy か:
  syntax / lowering / runtime / public packaging を早期固定しやすいため

### 6. operational shared-space / fabric runtime

- 主題:
  final activation / authority / auth / consistency / fairness catalog と concrete protocol profile
- なぜ heavy か:
  user specification と operational policy の影響が大きいため

### 7. raw external integration

- 主題:
  raw FFI、game engine direct binding、production visualizer / substrate integration
- なぜ heavy か:
  host-facing boundaryを越えた concrete target choice が必要になるため

### 8. domain / application realization

- 主題:
  synchronized shared-space、collaborative editing、virtual-world 系などの concrete app
- なぜ heavy か:
  acceptance criteria と evaluation target が user 依存だから

## theoretical reading pack inventory

この文書で immediate adoption をしないが、reserve / heavy line の比較基盤として維持する reading pack は次である。

- distributed order / snapshot
  - Lamport 1978
  - Chandy–Lamport 1985
- language-level memory model / scoped synchronization / dependency ordering
  - Boehm–Adve 2008
  - HSA System Architecture 1.2
  - WG21 P0750R1
  - WG21 P3475R2
  - optional comparison: WG21 P1239
- relaxed memory / event structures / correctness conditions
  - Jeffrey–Riely 2019
  - Herlihy–Wing 1990
- staged / modal / multimodal foundations
  - Davies–Pfenning 2001
  - Yuse–Igarashi 2006
  - Guarded Lambda-Calculus
  - Modal Dependent Type Theory and Dependent Right Adjoints
  - Multimodal Dependent Type Theory

この pack は **reference family** であり、repo の normative surface をそのまま決めるものではない。

## current recommendation

- `specs/examples/466...490` が再確認したとおり、corrected runnable version の current floorは reached であり、principal first-line package も close 済みだが、これは theory solved を意味しない。`M1/M2/M3` threshold package、theorem/model-check/order-handoff/shared-space actual adoption も close 済みであり、remaining heavy work は mixed gate / user-spec gate に残る。
- `plan/18` にある typed / theorem / model-check / ordering / syntax / modality の detailed program は、compare-floor から actual adoption package、helper-local actualization、principal theory spine / Lean-first proof roadmap package までは close 済みであり、remaining heavy work は mixed gate / user-spec gate にある。
- low-level memory-order family は retained-later reference family として保持しつつ、
  higher-level family comparison を先に進める。
- この文書の線は、boundary / pilot / adequacy corpus が固まっても mainline に混ぜない。
