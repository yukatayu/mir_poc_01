# Report 0842 — package87 phase6 parser second tranche shared single attachment frame first package ratchet

- Date: 2026-04-20T03:10:00+09:00
- Author / agent: Codex
- Scope: Package 87 actualization、shared single attachment frame first-package manifest / tests / snapshot sync
- Decision levels touched: L2

## 1. Objective

Package 86 close 後の次段として、
shared single attachment frame を
multiline extraction bridge + existing predicate fragment parser reuse の narrow code cut で
`mir_ast::current_l2` へ actualize する。

併せて `specs/` / `plan/` / `progress.md` / `tasks.md` / `Documentation.md`
を Package 87 close / Package 88 next 読みに同期する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/313-current-l2-phase6-parser-side-follow-up-package-sequencing-ready-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-comparison.md`
- `specs/examples/314-current-l2-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready-minimal-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-threshold.md`
- `specs/examples/315-current-l2-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready-fixed-subset-source-sample-corpus-scope-and-file-layout-comparison.md`
- `specs/examples/316-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-ready-minimal-fixed-subset-source-sample-corpus-scope-and-file-layout-threshold.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs`
- `docs/reports/0841-package86-phase6-parser-side-followup-package-sequencing-ratchet.md`

## 3. Actions taken

1. RED として `crates/mir-ast/tests/current_l2_shared_single_attachment_frame_manifest.rs` を追加し、missing import failure で shared single attachment frame manifest 不在を確認した。
2. `crates/mir-ast/src/current_l2.rs` に
   `CurrentL2SharedSingleAttachmentFrameManifest` と
   `current_l2_shared_single_attachment_frame_manifest()`
   を追加し、
   `carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs`
   current cut を crate 本体で inspectable にした。
3. `crates/mir-ast/src/lib.rs` の crate docs を更新し、
   narrow second tranche と shared single attachment frame first package を分けて読む wording に揃えた。
4. `specs/examples/560` と `D-147` を追加し、
   Package 87 close / Package 88 next を
   `Documentation.md`、`progress.md`、`tasks.md`、`plan/`、`specs/`、`plan/90-source-traceability.md`
   に同期した。

## 4. Files changed

- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/current_l2_shared_single_attachment_frame_manifest.rs`
- `specs/examples/560-current-l2-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-threshold-helper-mirror.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`

## 5. Commands run and exact outputs

1. RED:
   - `cargo test -p mir-ast --test current_l2_shared_single_attachment_frame_manifest current_l2_shared_single_attachment_frame_manifest_keeps_multiline_bridge_cut -- --exact`
   - output summary:
     `error[E0432]: unresolved import mir_ast::current_l2::current_l2_shared_single_attachment_frame_manifest`
2. GREEN:
   - `cargo test -p mir-ast --test current_l2_shared_single_attachment_frame_manifest current_l2_shared_single_attachment_frame_manifest_keeps_multiline_bridge_cut -- --exact`
   - output summary:
     `test result: ok. 1 passed; 0 failed`
3. Regression anchor:
   - `cargo test -p mir-ast --test current_l2_stage3_multiline_attachment_spike`
   - output summary:
     `test result: ok. 8 passed; 0 failed`
4. Formatting:
   - `cargo fmt --all`
5. Docs validation:
   - `python3 scripts/validate_docs.py`
   - exact output:
     `Documentation scaffold looks complete.`
     `Found 841 numbered report(s).`
6. Diff hygiene:
   - `git diff --check`
   - output: なし

## 6. Evidence / findings

- `CurrentL2SharedSingleAttachmentFrameManifest` の current cut は
  `carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs`
  の 4 field に留め、Package 87 の minimum を crate-local に inspectable にできた。
- accepted surface は
  - option-local `admit:` multiline extraction
  - request-local `require:` multiline extraction
  - request-local `ensure:` multiline extraction
  - existing predicate fragment parser reuse
  に限定され、request clause suite publicization や final grammar は retained-later に押し戻せた。
- stage3 multiline attachment spike suite 8 本が green のままであり、
  shared single attachment frame first package が existing multiline extraction bridge を壊していないことを確認した。
- `specs/examples/560` と `D-147` を追加し、Package 87 close と Package 88
  `fixed-subset-source-sample-corpus-scope-and-file-layout ratchet`
  への current queue を `Documentation.md` / `progress.md` / `tasks.md` / `plan/` / `specs/` / `plan/90` に同期した。

## 7. Changes in understanding

- shared single attachment frame first package は、
  request clause suite や perform head public API を待たず、
  multiline extraction bridge minimum だけを crate-local manifest と regression suite へ actualize する切り方で十分に閉じられる。
- parser-side next line は request clause suite publicization より先に、
  source corpus の scope / layout を第 3 層として固定する方が drift を減らす。
- `samples/current-l2/` 既存 corpus を final grammar と混同しないためにも、
  Package 88 では directory / extension / naming / non-goal minimum を先に narrow に actualize するのが自然である。

## 8. Open questions

- Package 88 の actualization carrier を
  `manifest` に置くか、source-corpus policy / sample metadata anchor に置くか。
- representative prose / fixture corpus / source corpus の mapping matrix を
  どの docs/code anchor から始めるか。
- actual sample content widening と parser-to-`Program` lowering を
  scope/layout minimum とどこで切り分けるか。

## 9. Suggested next prompt

- Package 88 として、
  repo-root `samples/current-l2/` flat `.txt` layer の scope / directory / naming / non-goal minimum を actualize し、
  representative / fixture / source の第 3 層を source-backed に固定する。
