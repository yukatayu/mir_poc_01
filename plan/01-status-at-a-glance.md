# plan/01 — 現況サマリ

## repo 全体の主眼

- 主眼は **Mir** の意味論基盤である。
- current repo は architecture-first だが、parser-free PoC、compile-ready minimal actualization、fixed-subset source sample decet を already 持つ。
- Mirrorea / shared-space / Typed-Effect / Prism / 上位アプリは separable track として扱う。

## current lane snapshot

- legacy checkpoint としては、Phase 1〜5 closeout / freeze と legacy Phase 6 compile-ready checkpoint は fixed 済みである。
- current active lane は 3 本で読むのが自然である。
  - execution lane:
    `Macro 4 / stable malformed capability second source-backed widening actualization`
  - theory-lab lane:
    `Macro 5` typed / theorem / model-check planning と
    `Macro 5/6` order / memory / authority-handoff / syntax / modality comparison
  - reserve integration lane:
    `Macro 6/7 / public operational CLI concrete shell actualization と bridge-only reserve boundary note`
- immediate blocker は `0`、current lane を止める user decision も `0` と読む。
- current authored source sample は `e1 / e2 / e3 / e4 / e16 / e18 / e19 / e21 / e22 / e23` の decet である。

## macro phase reading

| Macro phase | current state | autonomy gate |
|---|---|---|
| `Macro 0` repository memory | maintenance | self-driven |
| `Macro 1` semantic kernel | late | self-driven |
| `Macro 2` parser-free substrate | late | self-driven |
| `Macro 3` compile-ready minimal actualization | late | self-driven |
| `Macro 4` executable fixed-subset expansion | active | self-driven |
| `Macro 5` typed / theorem / model-check bridge | active at boundary | self-driven up to boundary |
| `Macro 6` fabric / shared-space / runtime evolution | docs-first boundary only | mixed |
| `Macro 7` toolchain / backend / host-facing integration | thin facade plus reserve shell | mixed |
| `Macro 8` domain / application realization | not started | user spec required |

## current capability snapshot

- parser-free current L2 PoC は runnable である。
- syntax-backed fixed-subset source sample path も runnable である。
- runtime / static / formal-hook ladder と regression helper は decet を current corpus として回せる。
- theorem / model-check line は
  - tool-neutral formal hook
  - `proof_notebook_review_unit`
  - row-local machine-facing model-check carrier
  - emitted artifact wiring
  - sample-facing evidence summary
  まで narrow first cut が source-backed である。
- current typed work は full type system ではなく、semantic carrier / checker boundary / source-visible first attachment candidate を比較する段階にある。
- shared-space は identity / admission / authority まで docs-first boundary があり、final operational catalog は later gate に残る。
- host-facing integration は privileged `stdin/stdout` ではなく、capability-scoped I/O / adapter boundary の docs-first cut に留める。
- ordering / `memory_order` reinterpretation は `atomic_cut` の place-local nucleus を越えたところでは still theory-first である。

## current recommendation

- execution lane は fixed-subset widening を narrow に継続する。
- theory-lab lane は typed / theorem / model-check planning と order / memory / syntax / modality comparison を mainline と混ぜずに進める。
- reserve integration lane は CLI / host / shared-space boundary note を hidden promotion なしで保つ。

## biggest later gates

- shared-space final activation / authority / auth / consistency / fairness catalog
- first external integration target
- backend / public operational surface success criteria
- upper-layer application target

## Rust / Python split の current reading

- Rust は parser / semantics / runtime / formal handoff / thin facade の本体を持つ。
- Python は detached orchestration、regression、docs validation、report scaffolding を持つ。
- current reading は **Rust-heavy core + mixed-tool helper workflow** である。
