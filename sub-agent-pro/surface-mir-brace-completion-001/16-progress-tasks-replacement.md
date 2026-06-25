# 16 — progress.md / tasks.md Replacement Model

## progress.md required shape

```markdown
# progress

最終更新: <date>

## Project Axis

正しい理論に基づき、Mir source files を意味の正本として、各 server / browser-like runtime / backend がそれ由来 artifact を実行し、正しく hot-plug / 通信 / 検証 / 可視化できる仮想空間 system を作る。

## Current Position

- Product Alpha-1 release candidate: ready in bounded local/Docker alpha scope.
- Operational product suite: workflow-ready narrow showcase.
- Mir computational core: first-floor evidence.
- Surface Mir source authority: <status>.
- Indexed state / auto communication / role admission / patch hot-plug: <status>.
- Final public product: not ready.

## Milestone Map

| Milestone | Meaning | Status | Evidence | Next gap |
|---|---|---|---|---|
| P-A1 | Product alpha release candidate | ... | ... | ... |
| P-OPS | Operational suite | ... | ... | ... |
| P-COMP | Mir-owned computation | ... | ... | ... |
| P-SURF-00 | docs/spec rebaseline | ... | ... | ... |
| P-SURF-01 | parser | ... | ... | ... |
| P-SURF-02 | indexed state | ... | ... | ... |
| P-SURF-03 | elaboration | ... | ... | ... |
| P-SURF-04 | auto communication | ... | ... | ... |
| P-SURF-05 | role admission | ... | ... | ... |
| P-SURF-06 | source patch | ... | ... | ... |
| P-SURF-07 | source operational suite | ... | ... | ... |
| P-SURF-08 | devtools | ... | ... | ... |
| P-SURF-99 | final audit | ... | ... | ... |

## Non-claims

- final public grammar
- Rust-level language completion
- LLVM/native codegen
- production WAN/federation
- distributed durable save-load R3/R4
- arbitrary native/WASM execution
```

## tasks.md required shape

```markdown
# tasks

## Current promoted package

P-SURF-00 or next package.

## Ordered self-driven packages

| Order | Package | Objective | Close condition |
|---|---|---|---|
| 1 | P-SURF-00 | docs/spec rebaseline | docs + validators pass |
| 2 | P-SURF-01 | parser | positive/negative parser rows |
...

## User decision gates

- final distribution.
- final public grammar freeze.
- production WAN/federation.
- LLVM/native backend.

## Stop lines

...
```
