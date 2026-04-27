# Sugoroku world runtime attachment samples

This directory is the active clean near-end sample family for a repo-local Mir / Mirrorea vertical slice.

It demonstrates:

- empty world bootstrap
- runtime Sugoroku game attachment
- logical multi-Place execution in one OS process
- server-appointed admin
- admin-only start/reset
- dice-owner-only roll
- roll -> publish -> witness -> handoff
- late join published-history visibility
- leave epoch / incarnation invalidation
- owner leave reassignment
- reset interleaving model-check
- detach as an explicit TODO lifecycle boundary

This is not final public parser grammar and does not implement real networking, consensus, durable distributed commit, or final public runtime API.

## How to run

```bash
python3 scripts/sugoroku_world_samples.py list
python3 scripts/sugoroku_world_samples.py check-all
python3 scripts/sugoroku_world_samples.py closeout --format json
```

Useful focused runs:

```bash
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug turn-trace
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures
python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership
python3 scripts/sugoroku_world_samples.py run 08_reset_interleaving_model_check --debug verification
```

## Sample matrix

| ID | Sample | Main point | Phase reading | Preferred debug | Expected outcome |
|---|---|---|---|---|---|
| `SUG-00` | `00_world_bootstrap.mir` | empty world, initial membership, no game yet | PH4 baseline | `summary` | success |
| `SUG-01` | `01_runtime_attach_game.mir` | runtime attach of `SugorokuGame#1` | PH7 attach floor | `summary` | success |
| `SUG-02` | `02_admin_start_reset.mir` | admin-only start/reset control | PH4 + PH7 control boundary | `summary` | success |
| `SUG-03` | `03_roll_publish_handoff.mir` | roll -> publish -> witness -> handoff | PH7 core E2E | `summary`, `turn-trace` | success |
| `SUG-04` | `04_non_owner_roll_rejected.mir` | stale or wrong owner action rejection | PH4 authority guard | `summary` | rejection |
| `SUG-05` | `05_late_join_history_visible.mir` | late join sees published history | PH4 membership timeline | `membership` | success |
| `SUG-06` | `06_leave_non_owner.mir` | leave increments membership epoch | PH4 leave invalidation | `membership` | success |
| `SUG-07` | `07_owner_leave_reassign.mir` | owner leave triggers reassignment | PH4 + PH7 continuity | `summary`, `membership` | success |
| `SUG-08` | `08_reset_interleaving_model_check.mir` | reset invalidates old-epoch handoff | PH5 + PH7 verification bridge | `verification` | success |
| `SUG-09` | `09_detach_todo.mir` | detach remains explicit TODO | PH14 preview only | `summary` | explicit TODO / non-final |

## Reading guide

- `PH4 shared-space membership / room boundary`
  - `SUG-00`, `SUG-02`, `SUG-04`, `SUG-05`, `SUG-06`, `SUG-07` are the main evidence.
- `PH7 Sugoroku runtime attach`
  - `SUG-01`, `SUG-03`, `SUG-07`, `SUG-08` show the current repo-local vertical slice.
- `PH14 hot-plug / detach`
  - `SUG-09` is intentionally not a completion claim. It keeps detach as a visible stop line.

## Important debug surfaces

- `summary`
  - short human-readable action and state summary.
- `turn-trace`
  - best view for `roll -> publish -> handoff`.
- `membership`
  - best view for join/leave, epoch, incarnation, and pending/active distinction.
- `verification`
  - best view for model-check, stale witness invalidation, and reset safety statements.
- `signatures`
  - `TermSignature` first cut の helper-local inventory view です。effect / transition / witness / relation / property を
    evidence-oriented に並べ、final public visualization protocol と混同しないための current debug surface です。

These helper-local outputs are evidence-oriented debug views. They are not the final public visualization protocol.

## Related docs

- `docs/research_abstract/hands_on_sugoroku_00_overview.md`
- `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`
- `docs/research_abstract/hands_on_sugoroku_detail.md`
- `docs/reports/0909-sugoroku-world-runtime-attachment-vertical-slice.md`
- `docs/reports/0916-sugoroku-sample-progress-alignment.md`
