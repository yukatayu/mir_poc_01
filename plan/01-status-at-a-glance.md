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
- current immediate line は、legacy checkpoint 名で言えば
  - deferred `e3` actualization reopen timing
  - actual `e3` authored-row reopen
  - proof / model-check first concrete tool pilot
  の並びにある。

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
- current authored source sample quintet `e1` / `e2` / `e21` / `e4` / `e23` は runnable ladder に接続済みである。
- tool-neutral formal hook と `proof_notebook_review_unit` first pilot はある。
- plain theorem-side bridge sketch first actualization と compare-ready bridge sketch second reopen もある。
- `e3` は source target only / deferred authored row のままである。

## current mainline

1. deferred `e3` actualization reopen timing
2. actual `e3` authored-row reopen
3. proof / model-check first concrete tool pilot
4. second source-sample cluster sequencing

## biggest later gates

- shared-space final activation / authority / auth / consistency / fairness catalog
- upper-layer application target
- backend / public operational surface success criteria

## Rust / Python split の current reading

- Rust は parser / semantics / runtime / formal handoff 本体を持つ。
- Python は detached-loop orchestration、source-sample regression、docs validation、report scaffolding を持つ。
- current reading は **Rust-heavy core + mixed-tool helper workflow** である。
