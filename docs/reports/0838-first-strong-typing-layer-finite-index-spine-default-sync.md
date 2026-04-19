# 0838 First Strong Typing Layer Finite-Index Spine Default Sync

## Objective

user が与えた型システム方針を、
既存の Problem 1 current first line と捻れなく整合させたまま
`specs/` / `plan/` / `Documentation.md` / `progress.md` / `tasks.md`
へ current explanation delta ではなく source-backed current default として
mirror する。

## Scope and assumptions

- 規範判断の正本は `specs/` を使う。
- full dependent core や final public type syntax はこの task で固定しない。
- 既存の checker-adjacent principal / theorem-first / row-local model-check first line は preserve する。
- 今回は current first strong typing layer の target narrowing と snapshot drift suppression を扱う。

## Documents consulted

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
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/examples/522-current-l2-layered-strong-typing-and-ifc-first-fragment-actual-adoption.md`
- `specs/examples/523-current-l2-source-side-ifc-explicit-authority-pair-narrow-actualization.md`
- `specs/examples/524-current-l2-source-side-ifc-label-flow-negative-narrow-actualization.md`
- `specs/examples/557-current-l2-first-strong-typing-layer-finite-index-spine-default.md`

## Actions taken

1. user の追加方針を既存 boundary と照合し、full dependent core を first public core に入れない current line と衝突しないことを確認した。
2. `specs/examples/557` を追加し、finite decidable index fragment、`Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ C` conceptual spine、IFC / taint / capture / lifetime / simple cost first-class target を current default として明文化した。
3. `specs/10` / `specs/11` / `specs/12` / `plan/18` を更新し、Problem 1 current first line と mixed gate の読みを新しい default に同期した。
4. `Documentation.md` / `plan/00` / `plan/01` / `plan/10` / `plan/11` / `plan/17` / `progress.md` / `tasks.md` を更新し、snapshot 側に vague な typed-line 読みが残らないようにした。
5. `plan/90-source-traceability.md` に新しい anchor と report を追加し、report chain から辿れるようにした。

## Files changed

- `specs/examples/557-current-l2-first-strong-typing-layer-finite-index-spine-default.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `Documentation.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0838-first-strong-typing-layer-finite-index-spine-default-sync.md`

## Commands run

- `rg -n 'phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package ratchet|phase6-reserve-formal-tool-binding-inventory ratchet|specs/examples/556|specs/examples/557|Package 84|Package 85' Documentation.md progress.md tasks.md plan specs -g '!target'`
- `sed -n '1,320p' progress.md`
- `sed -n '1,360p' tasks.md`

## Evidence / outputs / test results

- `specs/examples/557` により、次を current default として source-backed に整理した。
  - full dependent type は first public core に要求しない
  - finite decidable index fragment を first strong typing layer の principal target に置く
  - `Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ C` を conceptual spine に置く
  - IFC / taint / capture / lifetime / simple cost を first-class target に置く
  - arbitrary type-level computation / general theorem proving in compiler / final public type syntax / final public verifier contract は later に残す
- snapshot docs は Problem 1 current first line を上の読みへ同期し、Package 85 active line と矛盾しない状態になった。

## What changed in understanding

- checker-adjacent principal という表現だけでは typed line が広すぎた。
  finite decidable index fragment と conceptual spine を明示することで、
  「first strong typing layer の current target」と
  「full dependent core / final public surface を later に残す stop line」
  を同時に誤読なく書けるようになった。
- IFC / taint / capture / lifetime / simple cost は separate wish-list ではなく、
  同じ first strong typing layer package の中で読む方が current repo の evidence と整合する。

## Open questions

- finite decidable fragment を越える richer index reasoning をどの時点で theorem side へ送るか。
- stronger typed source principal promotion を experimental adoption package に上げる条件をどこで切るか。
- finite index constraints と theorem/model-check reserve line の emitted artifact seam をどこまで helper-local manifest に actualize するか。

## Suggested next prompt

Package 85 として theorem-first reserve / model-check second reserve /
parser-side mainline keep の inventory minimum を actualize し、
finite-index strong typing line と formal hook reserve line の接続を
helper-local manifest と tests に narrow に落としてください。
