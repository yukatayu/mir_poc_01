# 416 — current L2 order / handoff falsifier loop and adequacy-corpus hardening

## 目的

order / handoff line を prose-only comparison に留めず、

- falsifier family
- negative corpus coverage
- boundary-matrix hardening

を docs-first に整理する。

## source-backed floor

- cut family comparison は `atomic_cut / barrier / snapshot-only candidate / durable_cut` の 4 分解である。
- relation decomposition は `po / dep / pub / obs / wit / final / hb(scope)` の working vocabulary を持つ。
- order / handoff adequacy corpus と property-to-boundary matrix は already 存在する。
- 4-way verifier split は fixed 済みである。

## falsifier family

| falsifier family | what it tries to kill |
|---|---|
| cut-family conflation | `atomic_cut` を snapshot / durable commit と short-hand する reading |
| relation collapse | `pub == obs`、`obs == wit`、`final == durable`、`hb(scope) == authority-seriality` のような collapse |
| handoff precondition omission | publication / witness なし handoff が admissible に見える reading |
| replay / epoch omission | stale receipt / duplicate receipt / epoch mismatch を hidden repair する reading |
| provider / authority collapse | provider placement と authority slot を同一視する reading |
| fairness overclaim | safety と fairness を同じ bundle に潰す reading |
| seriality overfit | authority-serial transition を append-friendly room にも既定適用する reading |

## current judgment

1. adequacy corpus は **positive motivating scenarios だけでは足りない**。
2. package 4 の hardening では、negative corpus coverage を first-class に扱う。
3. falsifier loop は
   - cut / order collapse
   - handoff / replay / provider / fairness collapse
   - boundary-matrix hardening
   の 3 段で読むのが自然である。

## current first hardening targets

- handoff-before-publication
- handoff-without-witness
- duplicate or stale witness receipt
- handoff epoch mismatch
- provider / authority mismatch
- fairness fails while safety still holds
- snapshot visible but not commit-evidenced

## what is not decided here

- final property language
- final theorem / protocol tool binding
- final emitted artifact schema
- `snapshot_cut` / `consistent_cut` の final naming
