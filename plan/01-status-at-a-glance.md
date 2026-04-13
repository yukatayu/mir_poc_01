# plan/01 — 現況サマリ

## project 全体の主眼

- 主眼は **Mir** の意味論基盤である。
- ただし current repo は、単なる architecture memo ではなく、
  - parser-free current L2 PoC
  - `mir-ast` / `mir-semantics` / `mir-runtime` の narrow actual path
  - fixed-subset source sample の runnable ladder
  を持つ **architecture-first + runnable fixed subset** の段階にある。
- Mirrorea / shared-space / Typed-Effect / Prism / 上位アプリは separable な track として扱う。

## legacy checkpoint reading

- Phase 1〜5 closeout / freeze は fixed 済みである。
- legacy Phase 6 front-half compile-ready checkpoint も fixed 済みである。
- legacy checkpoint 名での recent close sequence は
  - actual `e3` authored-row reopen
  - proof / model-check first concrete tool pilot
  - second source-sample cluster sequencing
  - actual `e22` contrast-row source actualization
  - stable static malformed post-contrast sequencing
  - parser / checker / runtime public surface inventory
  - Mirrorea/shared-space docs-first re-entry
  - shared-space identity / auth layering reopen
  - model-check concrete carrier first actualization gate
  - stable malformed broader follow-up inventory
  - public operational CLI / final public contract later gate
  - shared-space admission / compile-time visibility reopen
  - shared-space authority / resource ownership reopen
  - model-check concrete carrier actualization comparison
  - model-check concrete carrier first actualization
  - source-sample emitted verification artifact wiring
  - sample-facing theorem/model-check evidence summary and bless/review flow
  まで fixed 済みと読む。

## macro phase reading

| Macro phase | current state | autonomy gate |
|---|---|---|
| `Macro 0` repository memory | maintenance | self-driven |
| `Macro 1` semantic kernel | late | self-driven |
| `Macro 2` parser-free substrate | late | self-driven |
| `Macro 3` compile-ready minimal actualization | late | self-driven |
| `Macro 4` executable fixed-subset expansion | active | self-driven |
| `Macro 5` theorem / model-check / static bridge | early-active | self-driven up to boundary |
| `Macro 6` fabric / shared-space / runtime evolution | docs-first boundary only | mixed |
| `Macro 7` toolchain / backend / public dev surface | inventory plus later-gate order | later |
| `Macro 8` domain / application realization | not started | user spec required |

## current capability snapshot

- parser-free current L2 PoC は runnable。
- current authored source sample decet `e1` / `e2` / `e3` / `e4` / `e16` / `e19` / `e21` / `e22` / `e18` / `e23` は runnable ladder に接続済みであり、`e16` / `e18` / `e4` / `e19` / `e23` は fixture-static formal-hook reached、`e1` / `e2` / `e21` / `e22` は runtime/formal-hook reached、`e3` は formal hook stage だけ guarded に残す。
- first post-sextet cluster は `e21` / `e22` try-rollback locality contrast として actualize 済みであり、stable-static edge-pair first reopen も fixed 済みである。
- parser / checker / runtime public surface inventory も fixed 済みであり、already-public parser-free stack、crate-public but non-production tranche、repo-local helper surface の 3 bucket split を current reading に置く。
- Mirrorea/shared-space docs-first re-entry、shared-space identity / auth layering reopen、model-check/public-checker second reserve inventory、model-check concrete carrier first actualization gate、stable malformed broader follow-up inventory、public operational CLI / final public contract later gate も fixed 済みである。
- source-sample emitted verification artifact wiring も fixed 済みであり、`run_current_l2_source_sample` の public/report shape を変えずに runtime test/support helper-local route と `proof_notebook_review_unit` / model-check carrier fan-out を actualize 済みである。
- sample-facing theorem/model-check evidence summary and bless/review flow も fixed 済みであり、README / `.docs` / snapshot docs を sample-facing surface、reviewed repo-local sync + inventory/regression success を current bless に置く docs-first cut を採った。
- docs-first I/O / host-facing port boundary も fixed 済みであり、language core に privileged `stdin/stdout` を入れず、capability-scoped input/output port / adapter boundary を first docs-only cut に置き、visualizer / host substrate / host runtime を consumer/provider 側、FFI / game engine adapter と final naming を later gate に残す current cut を採った。
- stable malformed missing-option first reopen actualization comparison も fixed 済みであり、helper-local compare を entry evidence に再利用しつつ、first reopen family は `e16/e17/e18` triplet に維持し、current next actualization mode は source-backed widening first に置く current cut を採った。
- stable malformed missing-option first source-backed widening actualization も fixed 済みであり、`e16` / `e18` を source-authored static-stop pair に widen しつつ、`e17` は same-family staged guard に留める current cut を採った。
- final public parser/checker/runtime first later gate actualization comparison も fixed 済みであり、current first later cut は `run_current_l2_source_sample` と `CurrentL2SourceSampleRunReport` を public entry / report に置く runtime-led thin facade に留め、support-only tranche と excluded bucket を明示した。
- `proof_notebook_review_unit` は current first concrete theorem/model-check pilot、`run_current_l2_source_sample` は later public-pressure の first docs-only candidate に留まる。
- public operational CLI second later gate actualization comparison も fixed 済みであり、current first cut は runtime-led thin facade を巻き戻さない Rust-side operational wrapper over `run_current_l2_source_sample` に留め、operational request/output shell concern だけを narrow に残す。
- repo-level current line は final public parser/checker/runtime thin-facade later support actualization に置き、その後に stable malformed capability second reopen actualization comparison を reserve に並べるのが current reading である。

## current mainline

1. public operational CLI second later gate actualization comparison
2. reserve final public parser/checker/runtime thin-facade later support actualization
3. reserve stable malformed capability second reopen actualization comparison

## biggest later gates

- shared-space final activation / authority / auth / consistency / fairness catalog
- upper-layer application target
- backend / public operational surface success criteria
- visualizer / host substrate / FFI / game engine adapter の first target profile

## Rust / Python split の current reading

- Rust は parser / semantics / runtime / formal handoff 本体を持つ。
- Python は detached-loop orchestration、source-sample regression、docs validation、report scaffolding を持つ。
- current reading は **Rust-heavy core + mixed-tool helper workflow** である。
