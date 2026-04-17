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
    `Macro 0〜4 closeout fixed`
    （duplicate pair `e14/e15` actualized、broader try-rollback malformed-static family は kept-later inventory）
  - theory-lab lane:
    `Macro 5 boundary / pilot / framing closeout fixed`
    （modality internalization trigger、stronger typed-surface threshold framing、theorem discharge transport / public-contract later-gate framing、model-check property-language / tool-binding later-gate framing は fixed 済み、remaining topics は mixed gate only）
  - reserve integration lane:
    `Macro 6/7 mixed-gate boundary fixed`
    （shared-space fairness / replay mixed-gate boundary と public operational CLI installed-binary / packaging success-criteria mixed-gate boundaryは fixed 済み、remaining topics は mixed gate / user-spec-required）

## source-backed で既にあるもの

- current L2 semantics / parser-free validation substrate / compile-ready minimal actualization
- fixed-subset source sample authored sixteen:
  `e1 / e2 / e3 / e4 / e5 / e12 / e14 / e15 / e16 / e13 / e19 / e21 / e22 / e18 / e20 / e23`
- runnable prototype sample octet:
  `p01 / p02 / p03 / p04 / p05 / p06 / p07 / p08`
  （`samples/prototype/` に置き、current lowerer / runner へ explicit path で流す）
  - `p01...p05 / p07 / p08` は order/handoff family
  - `p06` は typed/theorem/model-check sample-visible tranche
- helper-local debug output preview:
  prototype / sample 実行時に `debug_*` または `_debug_` を含み `_output` / `_pipe` で終わる target の record を `debug_outputs` として見せる current cut がある
- helper-local verification preview:
  prototype / sample 実行時に `formal_hook_status`、`subject_kind`、obligation list を `verification_preview` として見せる current cut がある
- helper-local artifact preview:
  prototype / sample 実行時に proof notebook review unit / model-check concrete carrier の derived row preview を `artifact_preview` として見せる current cut がある
- exact rough stimulus preservation bucket:
  `samples/not_implemented/`
- underdeclared source omission actualization:
  `e5-underdeclared-lineage` と `e12-underdeclared-target-missing` は `samples/current-l2/` authored corpus に actualize 済みであり、helper-local `verification_preview` / `artifact_preview` でも `fixture_static_cluster` route を reached として見せる
- sample-visible theorem / model-check first milestone:
  formal hook、`proof_notebook_review_unit`、row-local model-check carrier、emitted artifact wiring、sample-facing summary

## docs-first theory-lab で既にあるもの

- shared-space identity / admission / authority の docs-first boundary
- capability-scoped host-I/O / adapter boundary の docs-first cut
- order / memory-model / authority-handoff / syntax / modality / verifier-boundary comparison package:
  `specs/examples/405...412`
- typed / theorem / model-check / order-handoff follow-up package:
  `specs/examples/413...422`
- eight-package closeout の mixed follow-up:
  - reserve integration actualized / bridge package:
    `specs/examples/423...424`
  - theory-lab second-order follow-up:
    `specs/examples/425...429`
  - execution-side duplicate-cluster later reopen comparison:
    `specs/examples/430`
- reserve integration closeout follow-up:
  `specs/examples/431...432`
- theory-lab reserve hardening and duplicate next-cut comparison:
  `specs/examples/433...438`
- theory-lab third-order follow-up first pair:
  `specs/examples/439...440`
- theory-lab third-order follow-up second pair:
  `specs/examples/441...442`
- execution closeout actualization:
  `specs/examples/443`
- Macro 5 / mixed-gate boundary closeout:
  `specs/examples/444...450`
- runnable prototype / not-implemented bucket policy:
  `specs/examples/451`
- current sample debug output preview helper cut:
  `specs/examples/452`
- current sample verification preview and prototype second tranche:
  `specs/examples/453`
- current artifact preview helper cut:
  `specs/examples/454`
- current underdeclared source actualization and static artifact widening:
  `specs/examples/455`
- current typed/theorem/model-check sample-visible corrected prototype tranche:
  `specs/examples/456`
- current order/handoff corrected prototype third tranche:
  `specs/examples/457`

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
   `faq_005.md`

## 直近で特に重要な文書

- theory-lab operating model と comparison bundle:
  `specs/examples/405...412`
- typed / theorem / model-check / ordering の current adjacent package:
  `specs/examples/413...442`
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
