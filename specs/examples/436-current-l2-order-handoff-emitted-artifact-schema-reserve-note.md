# 436 — current L2 order/handoff emitted-artifact schema reserve note

## 目的

`specs/examples/428-current-l2-order-handoff-property-language-bridge-note.md`
までで fixed した

- cut family decomposition
- order relation decomposition
- adequacy corpus
- falsifier coverage
- property-language bridge

を前提に、
order / handoff line の **emitted-artifact schema reserve** を docs-first に整理する。

ここで fixed するのは reserve schema の cut であり、
final property language、final source-surface handoff syntax、concrete tool schema は fixed しない。

## source-backed floor

- `atomic_cut` は local finalization に留まる。
- `barrier` / snapshot-only observation / `durable_cut` は別 family である。
- working relation decomposition は
  `program_order / dependency_order / publication_order / observation_order / witness_order / finalization_order / scoped_happens_before`
  である。
- adequacy corpus と 4-way verifier boundary matrix は already ある。
- kept candidate は `authority_serial_transition_family`、second candidate は `witness_aware_commit_family` である。

## schema comparison

| candidate | reading | strengths | current risk |
|---|---|---|---|
| refs-only reserve schema | current bridge refs を束ねた docs-only emitted-artifact reserve | final property language を固定せずに前進できる | concrete consumer には dry に見える |
| consumer-shaped schema | theorem / protocol / runtime consumer ごとの near-final rows を先に切る | consumer mapping を見やすい | tool/public contract を premature に固定する |
| source-surface-first schema | source syntax token から emitted schema を逆算する | source wording と近い | source-surface handoff syntax を早く固定しすぎる |

## current judgment

current L2 で最も自然なのは、
**refs-only reserve schema**
である。

## reserve schema floor

current package が持つ minimum は次である。

```text
order_handoff_emitted_artifact_schema_reserve = {
  emitted_subject_kind,
  emitted_subject_ref,
  cut_family_refs[],
  order_relation_refs[],
  authority_handoff_refs[],
  boundary_bucket_refs[],
  guard_refs[],
  kept_later_refs[]
}
```

## keep / drop line

### keep

- refs-bundle bridge を principal docs-only floor に留める
- theorem / protocol / runtime の bucket split を guard 付きで保持する
- low-level `memory_order` family を retained-later reference に留める

### drop from current package

- final emitted-artifact schema
- final property language
- final source-surface handoff syntax
- concrete theorem/protocol tool schema

## next promoted line

next promoted line は、
**order / handoff source-surface wording reserve note**
に置く。

## what is not decided here

- final property language
- final emitted-artifact schema
- final source-surface handoff syntax
- concrete theorem / protocol tool binding
- `snapshot_cut` / `consistent_cut` naming
- low-level `memory_order` final stance
- scheduler / runtime finalization
