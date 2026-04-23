# plan/06 — surface notation の現在地

## 位置づけ

current L2 の surface notation は **companion notation** である。
final parser grammar ではない。

clean near-end suite の `.mir` files は
repo-local alpha current layer を説明する active sample text だが、
それでもなお final public grammar を意味しない。

## current principal

- typing:
  `index theory` / `policy` / `principal` / `resource` / `transition` / `stage`
- order / handoff:
  `publish` / `witness` / `handoff` / `after`
- modal:
  `stable` / `later` / `published(room)` / `witnessed(draw_pub)`
- model-check:
  `model` / `process` / `property`

## explicit keep

- domain predicate は builtin にしない
- authority hierarchy は user-defined finite preorder
- label hierarchy は user-defined finite lattice
- capture は finite capability/capture set
- lifetime は finite preorder
- cost は simple decidable bound

## order / handoff narrowing

- source principal は low-level `memory_order_*` exact token ではない
- current working vocabulary:
  `program_order`
  `dependency_order`
  `publication_order`
  `observation_order`
  `witness_order`
  `finalization_order`
  `scoped_happens_before`
- `atomic_cut` は local finalization / rollback frontier に留める

## modal foundation

- raw `◯` / `□` は final syntax にしない
- current correspondence:
  `A @ later`
  `A @ stable`
  `A @ published(room)`
  `A @ witnessed(draw_pub)`

## still not principal

- low-level `memory_order` / `kill_dependency` exact surface
- full dependent typed source principal
- final modal token import
- final parser punctuation / reserved keyword set

## historical note

pre-clean-near-end prototype wording と `p..` sample-specific notation は
historical comparison material であり、active notation principal ではない。
