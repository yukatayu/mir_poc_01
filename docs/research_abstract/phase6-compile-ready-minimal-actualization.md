# Phase 6 — compile-ready minimal actualization

## この phase の主眼

Phase 6 は、Phase 1〜5 で fixed した semantics / parser-free PoC / proof boundary を entry criteria としつつ、
`mir-ast` / `mir-semantics` / `mir-runtime` の actual code path を **non-production minimal cut** で compile-ready にする phase である。

この phase は production parser / runtime を確定する段階ではない。
主眼は、

- actual parser carrier
- checker / runtime skeleton bridge
- compile-ready gate と formal hook
- syntax-backed fixed-subset source sample lane

を narrow に揃えることである。

## source-backed current floor

- parser / AST first tranche:
  `specs/examples/299...300`
- checker / runtime first tranche:
  `specs/examples/301...302`
- compile-ready verification / formal hook:
  `specs/examples/303...304`
- parser follow-up / source sample corpus / lowering / runner / ladder:
  `specs/examples/311...324`
- theorem/model-check first pilot、sample-visible summary:
  `specs/examples/327...384`
- host-I/O、public surface、thin facade / CLI reserve:
  `specs/examples/363...404`

current authored source sample は
`e1 / e2 / e3 / e4 / e16 / e18 / e19 / e21 / e22 / e23`
の decet である。

## docs-first adjacent theory-lab floor

- typed/theorem/model-check boundary refresh と order/handoff falsifier hardening:
  `specs/examples/413...416`

## current lane split

### execution lane

- `Macro 4 / stable malformed capability second source-backed widening actualization`
- goal:
  fixed-subset source sample と runner / ladder / regression path を narrow に太らせる

### theory-lab adjacent lane

- post-planning quartet:
  typed-surface comparison / theorem lemma hardening / model-check bridge note
- post-falsifier comparison:
  order/handoff candidate reduction / modal follow-up
- goal:
  sample-visible first milestone の次段を docs-first に固定する

### reserve integration lane

- `Macro 6/7 / public operational CLI concrete shell actualization と bridge-only reserve boundary note`
- goal:
  thin facade、support-only surface、CLI shell concern の boundary を hidden promotion なしで保つ

## current status の短い要約

- parser-free path は runnable である。
- syntax-backed source sample path も runnable である。
- tool-neutral formal hook、`proof_notebook_review_unit`、row-local machine-facing model-check carrier、sample-facing evidence summaryは narrow first cut まで source-backed である。
- adjacent theory-lab line では、source-backed actualization を増やしたのではなく、
  docs-first planning / hardening package として
  - typed-core attachment inventory
  - semantic-core theorem pilot order
  - model-check projection reserve inventory
  - order/handoff falsifier loop
  まで docs-first に固まった。
- docs-first host-I/O boundary と shared-space identity / admission / authority boundary も fixed 済みである。
- final parser grammar、final public API、concrete theorem / model-check tool binding、backend / external codegen は still later に残る。

## まだ fixed していないこと

- stage 3 request / predicate reconnect の本格 reopen
- full typed surface / full strong type system
- concrete theorem prover / model-check tool binding
- final public parser / checker / runtime API
- span-rich diagnostics と final grammar
- shared-space final operational catalog
- backend / external codegen binding の timing

## current recommendation

- Phase 6 は「compile-ready checkpoint で止まった phase」ではなく、
  runnable fixed-subset lane と adjacent theory-lab lane を持つ継続 phase と読む。
- ただし、typed / theorem / model-check / order / memory / syntax / modality line は
  execution lane に無理に混ぜず、`plan/18` の theory-lab program として進める。

## 主要 evidence

- `specs/examples/299...324`
- `specs/examples/327...384`
- `specs/examples/363...404`
- `specs/examples/405...412`
- `docs/reports/0611...0699`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-semantics/examples/current_l2_emit_formal_hook.rs`
- `samples/current-l2/README.md`
