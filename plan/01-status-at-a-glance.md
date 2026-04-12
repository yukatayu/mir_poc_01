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
  まで fixed 済みと読む。

## macro phase reading

| Macro phase | current state | autonomy gate |
|---|---|---|
| `Macro 0` repository memory | maintenance | self-driven |
| `Macro 1` semantic kernel | late | self-driven |
| `Macro 2` parser-free substrate | late | self-driven |
| `Macro 3` compile-ready minimal actualization | late | self-driven |
| `Macro 4` executable fixed-subset expansion | active | self-driven |
| `Macro 5` theorem / model-check / static bridge | early | self-driven up to boundary |
| `Macro 6` fabric / shared-space / runtime evolution | docs-first boundary only | mixed |
| `Macro 7` toolchain / backend / public dev surface | inventory only | later |
| `Macro 8` domain / application realization | not started | user spec required |

## current capability snapshot

- parser-free current L2 PoC は runnable。
- current authored source sample septet `e1` / `e2` / `e3` / `e21` / `e22` / `e4` / `e23` は runnable ladder に接続済みであり、`e22` は runtime/formal-hook reached、`e3` は formal hook stage だけ guarded に残す。
- first post-sextet cluster は `e21` / `e22` try-rollback locality contrast として actualize 済みであり、stable static malformed post-contrast sequencing も fixed 済みである。parser / checker / runtime public surface inventory も fixed 済みであり、already-public parser-free stack、crate-public but non-production tranche、repo-local helper surface の 3 bucket split を current reading に置く。さらに `specs/examples/357...358` により Mirrorea/shared-space docs-first re-entry bundle、`specs/examples/359...360` により model-check/public-checker second reserve inventory も fixed 済みであり、Mirrorea/shared-space は repo-level current boundary track、Typed-Effect / Prism は adjacent track、`proof_notebook_review_unit` は current first concrete pilot、public-checker docs-only chain は second reserve bucket に留める。repo-level next line は stable-static edge-pair first reopen に置き、public operational surface、shared-space identity/auth layering、model-check concrete carrier first actualization gate を後続に残す。
- tool-neutral formal hook と `proof_notebook_review_unit` first pilot はある。
- plain theorem-side bridge sketch first actualization と compare-ready bridge sketch second reopen もある。

## current mainline

1. stable-static edge-pair first reopen
2. public operational surface actualization gate
3. shared-space identity / auth layering reopen
4. model-check concrete carrier first actualization gate

## biggest later gates

- shared-space final activation / authority / auth / consistency / fairness catalog
- upper-layer application target
- backend / public operational surface success criteria

## Rust / Python split の current reading

- Rust は parser / semantics / runtime / formal handoff 本体を持つ。
- Python は detached-loop orchestration、source-sample regression、docs validation、report scaffolding を持つ。
- current reading は **Rust-heavy core + mixed-tool helper workflow** である。
