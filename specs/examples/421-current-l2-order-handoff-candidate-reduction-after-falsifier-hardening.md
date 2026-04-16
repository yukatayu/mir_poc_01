# 421 — current L2 order / handoff candidate reduction after falsifier hardening

## 目的

`specs/examples/405...416` で揃えた cut family comparison、relation decomposition、adequacy corpus、falsifier family を前提に、

- current kept candidate family
- second candidate family
- derived/debug candidate
- retained-later reference family

を reduction note として整理する。

## source-backed floor

- cut family は local finalization / observation snapshot / ordering-only barrier / durable commit を分ける。
- relation decomposition は `program_order / dependency_order / publication_order / observation_order / witness_order / finalization_order / scoped_happens_before` を working vocabulary に持つ。
- falsifier family は relation collapse、handoff precondition omission、replay/epoch omission、provider/authority collapse、fairness overclaim、seriality overfit を already 含む。

## current reduction

| family | current role | reason |
|---|---|---|
| `authority_serial_transition_family` | current first higher-level candidate | authoritative room の publication / handoff / witness / payload / epoch を、`publication_order != observation_order != witness_order` を保ったまま最も素直に書ける |
| `witness_aware_commit_family` | second candidate | durability / evidence-bearing transition が必要な line に向くが、local-finalization line へ premature に混ぜない方がよい |
| `event_tree_execution_view` | derived/debug candidate | execution explanation / audit には有用だが、primary user-facing control family にすると order と view が collapse しやすい |
| low-level `memory_order` / `kill_dependency` family | retained-later reference family | backend-aligned comparisonには重要だが、current source surface に直輸入しない |

## current judgment

1. next kept family は **`authority_serial_transition_family`** に置く。
2. `witness_aware_commit_family` は durable/evidence burden が必要な second candidate に留める。
3. `event_tree_execution_view` は derived execution / debug candidate であり、primary control family にしない。
4. low-level `memory_order` family は reference family として保ちつつ、current language core には入れない。

## reduction rule

kept candidate と呼べるためには、少なくとも次を満たす必要がある。

- `publication_order == observation_order` と short-hand しない
- `observation_order == witness_order` と short-hand しない
- `finalization_order == durable` と short-hand しない
- `scoped_happens_before == authority_seriality` と short-hand しない
- provider placement と authority slot を別軸に保つ
- fairness claim を safety claim に潰さない

## next promoted line

next promoted line は、
**order / handoff property-language bridge note**
に置く。

## what is not decided here

- final property language
- source-surface handoff syntax
- concrete theorem / protocol tool binding
- `snapshot_cut` / `consistent_cut` の final naming
