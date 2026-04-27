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
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --transport loopback_socket --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --transport loopback_socket --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug turn-trace
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers
python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership
python3 scripts/sugoroku_world_samples.py run 08_reset_interleaving_model_check --debug verification
```

## Sample matrix

| ID | Sample | Main point | Phase reading | Preferred debug | Expected outcome |
|---|---|---|---|---|---|
| `SUG-00` | `00_world_bootstrap.mir` | empty world, initial membership, no game yet | PH4 baseline | `summary` | success |
| `SUG-01` | `01_runtime_attach_game.mir` | runtime attach of `SugorokuGame#1` | PH7 attach floor | `summary`, `hotplug` | success |
| `SUG-02` | `02_admin_start_reset.mir` | admin-only start/reset control | PH4 + PH7 control boundary | `summary` | success |
| `SUG-03` | `03_roll_publish_handoff.mir` | roll -> publish -> witness -> handoff | PH7 core E2E | `summary`, `turn-trace` | success |
| `SUG-04` | `04_non_owner_roll_rejected.mir` | stale or wrong owner action rejection | PH4 authority guard | `summary` | rejection |
| `SUG-05` | `05_late_join_history_visible.mir` | late join sees published history | PH4 membership timeline | `membership` | success |
| `SUG-06` | `06_leave_non_owner.mir` | leave increments membership epoch | PH4 leave invalidation | `membership` | success |
| `SUG-07` | `07_owner_leave_reassign.mir` | owner leave triggers reassignment | PH4 + PH7 continuity | `summary`, `membership` | success |
| `SUG-08` | `08_reset_interleaving_model_check.mir` | reset invalidates old-epoch handoff | PH5 + PH7 verification bridge | `verification` | success |
| `SUG-09` | `09_detach_todo.mir` | detach remains explicit TODO | PH14 preview only | `summary`, `hotplug` | explicit TODO / non-final |

## Reading guide

- `PH4 shared-space membership / room boundary`
  - `SUG-00`, `SUG-02`, `SUG-04`, `SUG-05`, `SUG-06`, `SUG-07` are the main evidence.
- `PH7 Sugoroku runtime attach`
  - `SUG-01`, `SUG-03`, `SUG-07`, `SUG-08` show the current repo-local vertical slice.
- `PH14 hot-plug / detach`
  - `SUG-01` と `SUG-09` で helper-local `hotplug_lifecycle` / `--debug hotplug` / attach-detach telemetry-view を読みます。
    `SUG-09` は intentionally not a completion claim であり、detach を visible stop line として残します。
- `PH13 network transport`
  - `SUG-01`, `SUG-03`, `SUG-04` は `--transport loopback_socket` を付けると helper-local `NET-01` parity canary になる。

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
- `envelopes`
  - `MessageEnvelope` / `AuthEvidence` / `PrincipalClaim` first cut の helper-local inventory view です。
    current cut では `auth none` baseline を explicit に見せ、transport / membership / capability / witness を separate lane のまま読めます。
    `--transport loopback_socket` は same-process preview only であり、real socket transport を意味しません。
- `layers`
  - `LayerSignature` first cut の helper-local inventory view です。current helper では `verification`、
    `runtime_trace`、`membership` layer だけを active に見せ、`auth` / `transport` / `telemetry` などは reserve に留めます。
- `visualization`
  - `VisualizationProtocol` first cut の helper-local inventory view です。`visualization_views` と
    `telemetry_rows` を label / authority / redaction 付きで並べ、helper-local evidence view だと明示します。
- `hotplug`
  - `HotPlug Patch / AttachPoint` executable widening の helper-local lifecycle view です。
    `AttachPoint[SugorokuGame#1]`、compatibility、activation cut、post-detach rejection、migration deferred を
    `message_envelopes` 由来の evidence として読みます。

These helper-local outputs are evidence-oriented debug views. They are not the final public visualization protocol.

## Related docs

- `docs/research_abstract/hands_on_sugoroku_00_overview.md`
- `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`
- `docs/research_abstract/hands_on_sugoroku_detail.md`
- `docs/research_abstract/hands_on_sugoroku_07_message_envelope_auth.md`
- `docs/research_abstract/hands_on_sugoroku_08_visualization_protocol.md`
- `docs/reports/0909-sugoroku-world-runtime-attachment-vertical-slice.md`
- `docs/reports/0916-sugoroku-sample-progress-alignment.md`
