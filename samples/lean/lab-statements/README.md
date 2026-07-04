# samples/lean/lab-statements

This directory stores repo-local LAB statement-shape drafts that compile under
Lean but do not move canon proof-obligation status.

These files are not `mirrorea_canon/` content, not proof discharge, and not
final public theorem contracts.

Current draft families:

- `obl001/`: THM-001 / OBL-001 assignment elaboration soundness shape.
- `obl020/`: OBL-020 step well-formedness preservation shape.
- `obl021/`: OBL-021 elaboration determinism shape.
- `obl024/`: OBL-024 explanation soundness / diagnostic replay shape.
- `obl025/`: OBL-025 explanation completeness / repair coverage shape.

Current sync guard note:

- `obl001/`, `obl020/`, and `obl021/` have body-level sync guards for the
  current statement-shape links. These guards are drift checks only; they are
  not proof skeleton completion or canon ledger movement.
