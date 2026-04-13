# plan/17 — research phases and autonomy gates

## 目的

この文書は、repo 全体の長期研究を

- legacy checkpoint
- macro phase
- autonomy gate

の 3 つで見やすく整理する。

ここでいう phase は strict waterfall ではない。
複数の line が並走してよい。

## how to read this document

### legacy checkpoint

- `specs/examples/...` や report に出てくる `Phase 1..7` は、当時の closeout / freeze / reopen line を示す historical label である。
- これは source traceability のために残す。

### macro phase

- `progress.md` と `tasks.md` では、repo 全体の大局を **macro phase** で整理する。
- old `Phase 7 = FutureWork` を再導入しないためである。

## legacy checkpoint reading

| legacy checkpoint | current reading |
|---|---|
| Phase 1 | closeout fixed |
| Phase 2 | closeout fixed |
| Phase 3 | freeze fixed |
| Phase 4 | closeout fixed |
| Phase 5 | closeout fixed |
| legacy Phase 6 front-half | compile-ready checkpoint fixed |
| old Phase 7 | 今後は単独 bucket としては使わず、macro phase `6/7/8` へ分解する |

## macro phase 一覧

| Macro phase | 主眼 | 現在位置 | 重さ | autonomy gate |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | maintenance | 低い | self-driven |
| `Macro 1` | semantic kernel / invariant stabilization | late | 中 | self-driven |
| `Macro 2` | parser-free validation substrate | late | 中 | self-driven |
| `Macro 3` | compile-ready minimal actualization | late | 中〜重い | self-driven |
| `Macro 4` | executable fixed-subset sample expansion | active | 重い | self-driven |
| `Macro 5` | theorem / model-check / static reasoning bridge | early-active | 重い | self-driven up to boundary |
| `Macro 6` | fabric / shared-space / runtime evolution | docs-first boundary only | 重い | mixed |
| `Macro 7` | toolchain / backend / developer surface | inventory plus later-gate order | 重い | mixed |
| `Macro 8` | domain / application realization | not started | とても重い | user spec required |

## autonomy gate detail

### self-driven でよい line

- semantic kernel の narrow refinement
- parser-free regression / helper maintenance
- compile-ready minimal actualization の narrow widening
- fixed-subset source sample expansion
- theorem-side bridge の compare-ready / first pilot
- sample-visible theorem/model-check milestone の narrow actualization

### boundary までは self-driven でよい line

- Mirrorea / shared-space の docs-first boundary
- public operational surface の inventory
- docs-first I/O / host-facing port boundary（working label）

### user specification が必要な line

- shared-space final activation / authority / auth / identity / admission / consistency / fairness catalog
- upper-layer application target
- final backend / tooling success criteria
- visualizer / host substrate / FFI / game engine adapter の first target profile

## current mainline

1. `Macro 7` final public parser / checker / runtime thin-facade later support actualization

## current reserve line

- `Macro 4` stable malformed capability second reopen actualization comparison

## current stop lines

次はまだ finalization しない。

- final parser grammar
- final public parser / checker / runtime API
- concrete theorem / model-check production contract
- shared-space final catalog
- backend / codegen public surface
- upper-layer app contract
- raw FFI / game engine direct binding actualization

## current judgments

- current repo は architecture-first だが、fixed-subset runnable path を already 持つ。
- Mirrorea/shared-space docs-first re-entry bundle、model-check/public-checker second reserve inventory、stable-static edge-pair first reopen、public operational surface actualization gate、shared-space identity/auth layering reopen、model-check concrete carrier first actualization gate、model-check concrete carrier actualization comparison、model-check concrete carrier first actualization、source-sample emitted verification artifact wiring、sample-facing theorem/model-check evidence summary and bless/review flow、stable malformed broader follow-up inventory、public operational CLI / final public contract later gate、shared-space admission / compile-time visibility reopen、shared-space authority / resource ownership reopen、docs-first I/O / host-facing port boundary、stable malformed missing-option first reopen actualization comparison、final public parser / checker / runtime first later gate actualization comparison、stable malformed missing-option first source-backed widening actualization、public operational CLI second later gate actualization comparison も fixed 済みであり、current mainline は `Macro 7` final public parser / checker / runtime thin-facade later support actualization へ進んでいる。
- host-facing integration は `stdin/stdout` privileged 化ではなく、`specs/examples/385...386` の docs-first I/O / port / adapter boundary として first cut を閉じ、visualizer / host substrate / FFI / engine adapter を separate gate に分ける。
- Mirrorea / shared-space / backend / apps は old `FutureWork` ではなく separable track として扱う。
- current immediate mainline を止める user decision は 0 件である。
