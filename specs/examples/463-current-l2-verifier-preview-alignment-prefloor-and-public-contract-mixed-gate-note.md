# 463 — current L2 verifier preview alignment pre-floor and public-contract mixed-gate note

## 目的

`specs/examples/446`、`447`、`453`、`454`、`459`、`462` を前提に、

- helper-local `verification_preview`
- helper-local `artifact_preview`
- fixture-backed emitted-artifact route
- sample-local preview-aligned typed artifact route

の関係を current mixed-gate pre-floor として整理する。

ここで fixed するのは、
**preview summary を final public verifier contract に昇格させずに compare/validation floor を 1 段厚くする current judgment**
であり、

- final public verifier contract
- theorem discharge transport adoption
- settled property language adoption
- concrete theorem / model-check tool seam

は still later に残す。

## source-backed floor

current repo では、少なくとも次が source-backed にある。

1. helper-local `verification_preview`
   - `formal_hook_status`
   - `subject_kind`
   - `subject_ref`
   - proof notebook review unit obligation list
   - model-check concrete carrier obligation list
   - `guard_reason`
2. helper-local `artifact_preview`
   - proof notebook review unit preview
   - model-check concrete carrier preview
3. fixture-backed emitted-artifact route
   - current authored source sample fixture / detached artifact route から formal hook / review unit / model-check carrier へ fan-out する helper-local route
4. corrected runtime/static/guarded samples
   - `e5-underdeclared-lineage`
   - `p05-dice-owner-guarded-chain`
   - `p06-typed-proof-owner-handoff`
   - `p07-dice-late-join-visible-history`
   - `p08-dice-stale-reconnect-refresh`

## observed drift

current evidence では、次の drift がある。

1. fixture-backed emitted-artifact route は current authored source sample fixture chain を principal にしており、prototype bucket をそのままは cover しない。
2. helper-local preview は sample-facing summary であるため、`subject_ref` と symbolic `evidence_refs` で sample-stem aligned kebab-case を使う。
3. fixture-backed formal hook / review-unit / model-check carrier は fixture-side `fixture_id` aligned subject ref を使うため、preview summary と literal exact match にはならない。

したがって、
**preview summary と fixture-backed emitted-artifact route を literal same-shape contract と読むのは current floor では不正確**
である。

## comparison

| candidate | reading | strengths | current risk |
|---|---|---|---|
| preview JSON direct promotion | current CLI summary をそのまま public contract 候補へ繰り上げる | outward shape は見やすい | shell convenience が final contract に見えやすい |
| fixture-backed route only | existing emitted-artifact route だけを compare floor にする | typed artifact line との continuity は強い | prototype bucket と sample-facing preview drift を吸収できない |
| preview-alignment pre-floor | sample-local preview-aligned typed artifact route を helper-local compare floor に置き、fixture-backed route は parallel evidence に残す | current sample corpus 全体で runtime/static/guarded/prototype compare を回しやすい | public contract と誤読すると widen しやすい |

## current judgment

current L2 で最も自然なのは、
**preview-alignment pre-floor**
である。

これは次を意味する。

1. `verification_preview` / `artifact_preview` は current shell-local summary に留める。
2. sample-local preview-aligned typed artifact route を test/support helper-local compare floor に追加してよい。
3. fixture-backed emitted-artifact route は source-authored fixture chain の evidence として retained する。
4. public contract を reopen するときも、current shell JSON を immediate source of truth にしない。

## pre-floor invariants

current pre-floor では、少なくとも次が alignment target である。

| field family | current alignment target |
|---|---|
| route status | `reached` / `guarded_not_reached` |
| subject envelope | `subject_kind` + sample-local `subject_ref` |
| theorem row set | proof notebook review unit obligation list |
| model-check row set | model-check concrete carrier obligation list |
| proof row detail | `obligation_kind` + `goal_text` + symbolic `evidence_refs` |
| model-check row detail | `obligation_kind` + symbolic `evidence_refs` |

## adequacy corpus

current compare floor では、少なくとも次を representative corpus にしてよい。

- `e5-underdeclared-lineage`
  - static reached
- `p05-dice-owner-guarded-chain`
  - guarded not reached
- `p06-typed-proof-owner-handoff`
  - typed/theorem/model-check runtime reached
- `p07-dice-late-join-visible-history`
  - order/handoff runtime reached
- `p08-dice-stale-reconnect-refresh`
  - stale reconnect runtime reached

## keep

- helper-local preview と final public contract を分ける
- sample-local preview-aligned route と fixture-backed emitted-artifact route を分ける
- theorem discharge transport / public-contract seam を mixed gate に残す
- model-check property-language / tool seam を mixed gate に残す

## drop from current package

- preview JSON の public contract promotion
- fixture-backed emitted-artifact route の prototype bucket 既成事実化
- concrete theorem / model-check tool seam adoption
- final emitted schema / public verifier contract adoption

## next promoted line

next promoted line は、
**theorem discharge transport / public-contract と property-language / tool seam の mixed-gate concretization**
に置く。

## what is not decided here

- final public verifier contract
- theorem discharge result transport shape
- first settled property language
- concrete theorem / model-check tool brand
- sample-local preview-aligned route を public layer へ昇格するかどうか
