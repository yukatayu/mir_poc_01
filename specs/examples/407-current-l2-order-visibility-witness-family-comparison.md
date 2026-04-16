# 407 — current L2 order / visibility / witness family comparison

## 目的

low-level memory-order family を direct import せずに、
order / visibility / witness を relation decomposition として比較する。

## source-backed floor

- low-level memory-order family は retained-later reference family である。
- higher-level async-control first cut は `authority_serial_transition_family` である。
- `atomic_cut` は local finalization nucleus であり、low-level order family と同一視しない。

## working relation decomposition

| relation | working reading | current use |
|---|---|---|
| `po` | same-place program order | local structural / proof reading |
| `dep` | dependency-preserving order | consume / kill-dependency reference family comparison |
| `pub` | publication / release-like relation | handoff precondition / visibility edge |
| `obs` | observation / acquire-or-consume-like relation | receiver / reader side visibility |
| `wit` | witness / receipt / proof / handoff evidence relation | theorem / protocol boundary handoff |
| `final` | local or scoped finalization relation | `atomic_cut` nucleus and later scoped finalization families |
| `hb(scope)` | scope-aware admissible order relation | shared-memory lowering / distributed handoff lowering comparison |

## current judgment

1. source language では exact `acquire / release / consume / seq_cst` API を凍らせない。
2. backend / expert refinement / lowering side には reference family として残してよい。
3. `kill_dependency` line は、dependency relation をどこで切るかの comparison material として読む。
4. relation decomposition は shared-memory lowering と distributed handoff lowering の双方に接続できるように保つ。

## comparison note

- C++ `consume` line は、dependency ordering が現実に必要だという reference family である。
- 同時に、current standard surface が unstable であることも reference material になる。
- したがって current repo では
  **reference family yes / direct surface import no**
  を first wording に置く。

## what is not decided here

- relation 名を final user-facing token にすること
- backend lowering contract
- theorem / protocol / runtime の final handoff schema
