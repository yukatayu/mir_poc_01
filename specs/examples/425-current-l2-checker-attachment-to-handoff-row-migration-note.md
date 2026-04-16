# 425 — current L2 checker attachment to handoff row migration note

## 目的

typed-core line で、

- checker-adjacent principal source
- source-visible structural marker family
- optional family hint
- downstream theorem / model-check handoff refs

の migration cut を docs-first に整理する。

ここで fixed するのは
**principal source を checker attachment に残したまま handoff row へ何を渡すか**
であり、

- full typed syntax
- unified obligation block
- public typed API

は still later に残す。

## source-backed floor

- current first typed attachment candidate は source syntax ではなく
  checker-adjacent semantic carrier 側に置く。
- source-visible marker family は
  capability / `lease` / `admit` / lineage / `require` / `ensure`
  を first comparison material に留める。
- theorem/model-check artifacts は
  `formal_hook -> proof_notebook_review_units / model_check_concrete_carriers`
  の derived route に留める。

## current migration shape

```text
typed_attachment_migration = {
  source_anchor_refs,
  checker_attachment_refs,
  source_visible_marker_refs,
  optional_family_hint_refs,
  derived_handoff_refs,
  guard_refs,
  kept_later_refs
}
```

## current judgment

1. principal typed source は
   **checker attachment**
   に残す。
2. source-visible markers は current first candidate として残すが、
   principal source や direct handoff row source of truth にはしない。
3. handoff row は
   checker attachment から導出される downstream reserve row
   として扱い、review/model-check artifact を principal typed source と同一視しない。
4. current migration cut では
   `source anchor -> checker cluster row -> optional family hint -> derived handoff ref`
   の 4 段を分ける方が drift を抑えやすい。

## guard

- full typed syntax を fixed しない。
- unified obligation block を fixed しない。
- dedicated attachment block を fixed しない。
- review/model-check artifact を principal typed source と書かない。

## next promoted line

next promoted line は、
**proof artifact / bridge stop-line refresh**
に置く。

## what is not decided here

- `family_refs[]` の exact namespace
- request / predicate / `try` cluster をどこまで同じ attachment shape に寄せるか
- full type calculus / inference / annotation design
