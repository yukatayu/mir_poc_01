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
- eight-package follow-up:
  - reserve integration actualized / bridge package:
    `specs/examples/423...424`
  - theory-lab second-order follow-up:
    `specs/examples/425...429`
  - execution-side duplicate-cluster later reopen comparison:
    `specs/examples/430`
- reserve integration closeout follow-up:
  `specs/examples/431...432`
- third-order theory-lab reserve hardening and duplicate next-cut comparison:
  `specs/examples/433...438`
- third-order follow-up:
  `specs/examples/439...442`
- execution closeout actualization:
  `specs/examples/443`
- Macro 5 closeout / mixed-gate boundary follow-up:
  `specs/examples/444...450`

current authored source sample は
`e1 / e2 / e3 / e4 / e14 / e15 / e16 / e13 / e19 / e21 / e22 / e18 / e20 / e23`
の authored fourteen である。

## docs-first adjacent theory-lab floor

- typed/theorem/model-check boundary refresh と order/handoff falsifier hardening:
  `specs/examples/413...416`
- source-visible typed comparison、theorem wording hardening、model-check bridge note、order/handoff candidate reduction、modal follow-up:
  `specs/examples/417...422`
- second-order follow-up:
  `specs/examples/425...429`

## current lane split

### execution lane

- `Macro 0〜4 closeout fixed`
- duplicate pair `e14/e15` actualized、broader try-rollback malformed-static family は kept-later inventory
- goal:
  current fixed-subset widening closeout を保ったまま broader malformed-static family を hidden promotion なしで reserve に残す

### theory-lab adjacent lane

- boundary / pilot / framing closeout fixed:
  request/predicate/try cluster typed-surface reserve note / admissible evidence contraction note / tool-binding stop-line refresh /
  order/handoff emitted-artifact schema reserve note / guarded-vs-MDTT/MTT reduction timing note /
  typed-surface family split / notebook-consumer threshold /
  model-check small-cluster projection keep/drop / order-handoff source-surface wording reserve /
  modality internalization trigger / stronger typed-surface threshold framing /
  theorem discharge transport-public-contract later-gate framing /
  model-check property-language-tool-binding later-gate framing
- goal:
  sample-visible first milestone の次段 reserve を docs-first に固定したうえで、boundary / pilot / framing closeout を current scoped fixed 読みに揃え、remaining topics を mixed gate に送る

### reserve integration lane

- `Macro 6/7 mixed-gate boundary fixed`
- goal:
  thin facade、support-only surface、CLI shell concern、shared-space fairness / replay line を hidden promotion なしで保ったまま、remaining topics を mixed gate に留める

## current status の短い要約

- parser-free path は runnable である。
- syntax-backed source sample path も runnable である。
- tool-neutral formal hook、`proof_notebook_review_unit`、row-local machine-facing model-check carrier、sample-facing evidence summaryは narrow first cut まで source-backed である。
- adjacent theory-lab line では、source-backed actualization を増やしたのではなく、
  docs-first planning / hardening package として
  - typed-core attachment inventory
  - source-visible typed-surface comparison
  - semantic-core theorem pilot order と lemma wording floor
  - model-check projection reserve inventory と bridge grain
  - order/handoff falsifier loop と candidate reduction
  - modal comparison follow-up
  まで docs-first に固まった。
- docs-first host-I/O boundary と shared-space identity / admission / authority boundary も fixed 済みである。
- current-L2 scoped Rust shell over thin facade、shared-space room-profile / host-binding bridge-only note、public operational CLI installed-binary / packaging success-criteria mixed-gate boundary、shared-space fairness / replay mixed-gate boundary も fixed 済みであり、reserve integration line は mixed-gate boundary fixed に入っている。
- duplicate-cluster line は current では pair-first actualization を close 済みとし、broader malformed-static family は kept-later inventory に残す。
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
- `specs/examples/405...443`
- `docs/reports/0611...0703`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-semantics/examples/current_l2_emit_formal_hook.rs`
- `samples/current-l2/README.md`
