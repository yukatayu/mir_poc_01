# samples/lean

This directory records what the repo currently validates with Lean in a repo-local, inspectable form.

## Layout

- `foundations/`
  - small self-contained Lean files with actual proofs
  - current focus: IFC / label-model first fragment and proof-skeleton / obligation-shape first fragment
- `current-l2/`
  - generated Lean theorem stubs for the representative theorem quartet: e5-underdeclared-lineage, p06-typed-proof-owner-handoff, p07-dice-late-join-visible-history, p08-dice-stale-reconnect-refresh
  - these files are accepted by Lean but still contain `sorry`

## Reading Rule

- `foundations/` shows the **mechanization-ready core** that is already precise enough to prove small facts.
- `current-l2/` shows the **actual emitted theorem bridge surface** that the repo generates for representative samples.
- The generated current-L2 stubs demonstrate artifact alignment and Lean acceptance, not completed theorem discharge.

## Rebuild

Run:

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

This regenerates the committed Lean sample corpus and verifies it with `lean`.
