# ドキュメント要約

## リポジトリの目的

このリポジトリは、次の subsystem を中心とした **specification-first research repo** である。

- **Mir** — 意味論コア言語
- **Mirrorea** — 分散 fabric と control plane
- **PrismCascade** — 独立した media-graph kernel
- **Typed-Effect Wiring Platform** — inspectable / routable / contract-aware な effect layer

これらは related だが、repo では **separable subsystem** として扱う。

## 現在の読み

- プロジェクト全体は **architecture / semantics first** の段階にある。
- ただし current repo は docs-only skeleton ではなく、
  - parser-free PoC
  - compile-ready minimal actualization
  - fixed-subset source sample の runnable ladder
  を already 持つ。
- current status は 3 lane で読むのが自然である。
  - execution lane:
    `Macro 4 / malformed duplicate-cluster and try-rollback malformed-static later reopen inventory`
  - theory-lab lane:
    `Macro 5` follow-up
    (`checker attachment -> handoff migration`、`proof artifact / bridge stop-line refresh`、`sample-visible property summary wording`) と
    `Macro 5/6` follow-up
    (`order/handoff property-language bridge`、`modal promotion-threshold note`)
  - reserve integration lane:
    `Macro 6/7 / public operational CLI concrete shell actualization と shared-space room-profile・host binding bridge-only note`

## source-backed で既にあるもの

- current L2 semantics / parser-free validation substrate / compile-ready minimal actualization
- fixed-subset source sample authored dozen:
  `e1 / e2 / e3 / e4 / e13 / e16 / e18 / e19 / e20 / e21 / e22 / e23`
- sample-visible theorem / model-check first milestone:
  formal hook、`proof_notebook_review_unit`、row-local model-check carrier、emitted artifact wiring、sample-facing summary

## docs-first theory-lab で既にあるもの

- shared-space identity / admission / authority の docs-first boundary
- capability-scoped host-I/O / adapter boundary の docs-first cut
- order / memory-model / authority-handoff / syntax / modality / verifier-boundary comparison package:
  `specs/examples/405...412`
- typed / theorem / model-check / order-handoff follow-up package:
  `specs/examples/413...422`

## まだ OPEN のもの

- final parser grammar
- final public parser / checker / runtime API
- concrete theorem prover / model-check tool binding
- shared-space final operational catalog
- backend / external codegen success criteria
- raw FFI / engine adapter target
- upper-layer application target

## 読み進める入口

1. repo の規範判断:
   `specs/00-document-map.md`、`specs/01`、`specs/02`、`specs/03`、`specs/09`
2. current status の snapshot:
   `progress.md`、`tasks.md`
3. long-lived repository memory:
   `plan/00-index.md`、`plan/01-status-at-a-glance.md`、`plan/10-roadmap-overall.md`、`plan/11-roadmap-near-term.md`
4. theory-lab の detail:
   `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
5. current long-form FAQ:
   `faq_004.md`

## 直近で特に重要な文書

- theory-lab operating model と comparison bundle:
  `specs/examples/405...412`
- typed / theorem / model-check / ordering の current adjacent package:
  `specs/examples/413...422`
- current near-term order:
  `plan/11-roadmap-near-term.md`
- theory-lab の detail:
  `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- open problems / heavy line:
  `plan/12-open-problems-and-risks.md`、`plan/13-heavy-future-workstreams.md`
- compile-ready minimal actualization の current summary:
  `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`

## 補足

- 規範判断の正本は常に `specs/` に残る。
- `plan/` は長期参照用 repository memory であり、snapshot ではない。
- `progress.md` と `tasks.md` は薄い snapshot として保つ。
