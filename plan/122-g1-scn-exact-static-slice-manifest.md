# plan/122 - G1 SCN exact static slice manifest

## Purpose

This file is LAB repository memory.

It turns the `G1-MVS-ASSIGNMENT-STATIC` candidate from
`plan/121-g1-minimal-vertical-slice-candidate-map.md` into a concise manifest
for SCN-01 and SCN-02 static evidence. The goal is to make the next OBL-001 or
statement-boundary package cite the exact current rows without overclaiming
conformance, proof, runtime behavior, or final ABI.

This file does not edit canon, does not promote a package, does not close G0 or
G1, does not move proof-obligation status, does not claim C-static
conformance, and does not change runnable sample status.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB planning and evidence: legacy `specs/`, `plan/`, samples, helpers,
  reports, Rust code, and Lean statement drafts outside `mirrorea_canon/`
- Snapshot status: `progress.md` and `tasks.md`
- Runnable dashboard: `samples_progress.md`

If LAB evidence conflicts with canon, canon wins. "Exact" in this file means
exact for the current LAB static-evidence manifest only; it does not mean canon
conformance, proof, final exchange format, runtime dispatch, or public API.

## Current phase reading

The overall plan remains in `Macro 0 / T0-G0 rebaseline` when judged by
`mirrorea_canon/plan/01-phases.md`. Existing Product Alpha, Full System V1, and
Surface artifacts are LAB evidence floors that reach ahead of canon
implementation state, but they do not move the canon phase out of T0.

Within that holding state, the nearest theory-facing line remains G1 ordinary
assignment. This manifest is a bridge from current LAB Surface evidence to the
SCN-01/SCN-02 static bullets that pressure THM-001 / OBL-001.

## Relation to plan/121

`plan/121` selected the source-first static assignment spine. This file narrows
the executable row usage:

- center exact static evidence on `ELAB-11` for SCN-01;
- center exact static evidence on `ELAB-12` for SCN-02;
- use `ELAB-02`, `ELAB-05`, `ELAB-07`, `ELAB-09`, and `ELAB-10` as structural
  support only;
- keep SCN runtime bullets, runtime failures, store mutation, occurrence
  ordering, admission lifecycle, and distributed transport as explicit gaps /
  out of scope.

## Classification labels

| Label | Meaning in this file | Non-meaning |
|---|---|---|
| `exact current executable evidence` | The current LAB fixture source and expected JSON directly match the named SCN static bullet shape. | C-static pass, proof, final JSON/API, or runtime behavior. |
| `structural support only` | The fixture supports the same mechanism or guard but differs in source shape, diagnostic carrier, or exact negative variant. | Exact SCN row pass. |
| `explicit gap / out of scope` | No current static manifest row should be cited as evidence for that bullet. | A requirement cancellation. |

## SCN-01 exact static manifest

Canon source: `mirrorea_canon/scenarios/SCN-01-sugoroku-roll.md`.

SCN-01 source shape:

```text
BrowserClient[self] {
  when roll(draw: Int64)
    fails MissingCapability, MissingWitness, RouteUnavailable,
          StaleMembership, VisibilityDenied {
    World { player[self].position = player[self].position + draw }
  }
}
```

| Static bullet | Classification | Evidence to cite | Boundary |
|---|---|---|---|
| request edge from `BrowserClient[self]` to `World` for `write player[self].position` | `exact current executable evidence` | `ELAB-11` source and expected JSON: one `remote_request_summaries` row with `request_kind = write`, `requester_locus = role:BrowserClient`, `owner_locus = World`, `state_name = player`, `key_expr = self`, `generated_from = nested_place_block`, and `failure_row_complete = true`. | Static elaboration evidence only; no request serving, C-static pass, or final Core ABI. |
| same-field RHS dependency for `player[self].position` | `exact current executable evidence` | `ELAB-11` `dependency_summaries`: one `rhs_indexed_read` row linked to `req-0001`, owner `World`, key `self`, field `position`. Also cite `plan/75` for the dependency-gap rationale. | `rhs_indexed_read` is a LAB carrier; do not freeze read materialization, transport, cache, reply, or OPEN-014 policy. |
| visible write yields publish / observe consequence for `position` | `exact current executable evidence` | `ELAB-11` `generated_edge_kinds` include `auto_publish` and `auto_observe`; `publication_summaries` and `observation_summaries` name `World`, `player`, `self`, `position`, and `observer_safe`. | LAB `message_envelope`, `auto_publish`, and `auto_observe` names are carriers, not runtime dispatch or final telemetry ABI. |
| spans on generated consequences | `exact current executable evidence` | `ELAB-11` `source_span_entity_kinds` includes transition, message envelope, generated edges, publication, observation, remote request, and dependency. `ELAB-05` remains span-support evidence. | No span proof, no final `source_map`, no final Diagnostic JSON ABI. |
| obligations include write capability for `player` | `structural support only` | `ELAB-11` and adjacent write rows carry generic obligation codes for explicit transitions, remote requests, source spans, generated failure rows, and pending role/runtime integration. `plan/72` already classifies SCN-01 capability evidence as LAB support. | Current rows do not prove the exact canon capability theorem and do not discharge G3 / THM-004 authority work. |
| negative: removing `VisibilityDenied` yields E-ROW-002 at assignment span | `exact current executable evidence` after `plan/123` | `ELAB-17` demonstrates the exact SCN-01 visible-write/publish underdeclaration with `E-ROW-002` / `VisibilityDenied`. `ELAB-10` remains structural read/observe support only. | Do not claim Diagnostic ABI freeze, repair ABI freeze, OBL-024/025 discharge, OBL-001 proof, C-static conformance, or G1 exit. |
| runtime `roll(3)` store/order behavior | `explicit gap / out of scope` | none for this manifest. | Admission, store update, request-before-serve-before-publish ordering, runtime dispatch, and distributed transport are outside `G1-MVS-ASSIGNMENT-STATIC`. |

## SCN-02 exact static manifest

Canon source: `mirrorea_canon/scenarios/SCN-02-attack.md`.

SCN-02 source shape:

```text
BrowserClient[self] {
  when attack(target: Participant)
    fails StaleMembership, MissingCapability, MissingWitness, RouteUnavailable {
    S { player[target].hp = player[target].hp - player[self].atk }
  }
}
```

| Static bullet | Classification | Evidence to cite | Boundary |
|---|---|---|---|
| request edge to `S` for `write player[target].hp` | `exact current executable evidence` | `ELAB-12` source and expected JSON: one `remote_request_summaries` row with `request_kind = write`, `requester_locus = role:BrowserClient`, `owner_locus = S`, `state_name = player`, `key_expr = target`, `generated_from = nested_place_block`, and `failure_row_complete = true`. | Static elaboration evidence only; no runtime serving, store mutation, C-static pass, or final Core ABI. |
| RHS dependency for `player[target].hp` | `exact current executable evidence` | `ELAB-12` `dependency_summaries`: one `rhs_indexed_read` row for owner `S`, key `target`, field `hp`, linked to `req-0001`. | No runtime read materialization policy, freshness policy, or OPEN-014 resolution. |
| RHS dependency for `player[self].atk` | `exact current executable evidence` | `ELAB-12` `dependency_summaries`: one `rhs_indexed_read` row for owner `S`, key `self`, field `atk`, linked to `req-0001`. | No observe/read-request finalization and no cross-locus read transport claim. |
| generated failures are contained in declared `fails` | `exact current executable evidence` for the positive row | `ELAB-12` `remote_request_summaries[0].failure_row_complete = true` and empty `diagnostic_codes`. | This is positive containment evidence only; it does not freeze canon diagnostic IDs or prove row containment generally. |
| nested `S { ... }` is not ambient authority | `exact current executable evidence` for edge shape | `ELAB-12` keeps `requester_locus = role:BrowserClient`, `owner_locus = S`, and `generated_from = nested_place_block`. `IDX-05` and `ELAB-02` remain structural non-ambient-authority support. | Do not promote this to THM-004 / G3 authority proof or production capability behavior. |
| negative (a): dropping `MissingCapability` yields E-ROW-001 | `structural support only` | `ELAB-07` is a write-side E-ROW-001-shaped underdeclared-failure row with LAB `generated_failure_not_declared` and non-final `lab_diagnostic_details`; `plan/72` records the diagnostic-ID gap. | Near-shape only: RHS is simplified, the emitted diagnostic code is a LAB alias, and repair details are non-final. |
| negative (b): treating nested block as direct local write must fail C-static | `structural support only` | `ELAB-02` and `ELAB-12` show the required positive request edge shape. | There is no dedicated executable negative row for an implementation that directly mutates remote state. |
| runtime `hp 100 -> 90`, runtime `MissingCapability`, and stale target behavior | `explicit gap / out of scope` | none for this manifest. | Runtime admission, capability failure occurrence, unchanged store, stale membership, and distributed execution are outside the static manifest. |

## Support rows

| LAB row | Use | Limit |
|---|---|---|
| `SURF-01` / `SURF-02` / `SURF-09` | Surface syntax pressure: `S { ... }` is canonical place syntax, `S[ ... ]` is not place-scope sugar, and role-instance indexing remains distinct. | Parser evidence only; no final public grammar freeze. |
| `IDX-01` | S-owned indexed-state map pressure. | Not exact SCN pass evidence. |
| `IDX-02` | Key-is-not-authority guard. | Not authority theorem proof. |
| `IDX-05` | Nested place block is not ambient authority and must elaborate to generated request before writing S-owned indexed state. | Not runtime or G3 proof. |
| `ELAB-02` | Owner-directed write request and nested-block non-ambient-authority structural support. | Not exact SCN-01 source shape and not a direct-local-write negative. |
| `ELAB-05` | Source-span support for generated rows. | Not span proof or final source-map ABI. |
| `ELAB-07` | E-ROW-001-shaped write-side underdeclared generated failure structural support. | Not exact SCN-02 negative pass or final diagnostic / repair ABI. |
| `ELAB-09` | Visible write publish / observe structural support. | Superseded by `ELAB-11` for SCN-01 exact positive shape; no runtime dispatch claim. |
| `ELAB-10` | E-ROW-002-shaped `VisibilityDenied` underdeclaration support. | Visible read/observe negative only; no exact SCN-01 visible-write negative. |
| `ELAB-11` | Primary exact SCN-01 static evidence. | No conformance, proof, runtime, or final ABI. |
| `ELAB-12` | Primary exact SCN-02 static evidence. | No conformance, proof, runtime, or final ABI. |
| `ELAB-17` | Exact SCN-01 visible-write `VisibilityDenied` negative evidence after `plan/123`. | Rejected-row failure-containment pressure only; diagnostic projection / repair payload details remain OBL-024 / OBL-025 LAB evidence, not OBL-001 content. |

## Post-plan/123 / plan/124 addendum

`plan/123-g1-scn01-visibility-negative-actualization.md` added `ELAB-17`, so
the SCN-01 `VisibilityDenied` negative is no longer a current exact-evidence
gap. `plan/124-g1-obl001-boundary-audit.md` then audited this row against the
existing OBL-001 abstract boundary and found no missing Lean predicate.

This addendum updates the current manifest reading only. It does not change the
historical fact that this file was first written before `ELAB-17`, and it does
not claim canon edit, OBL completion, proof discharge, conformance, G1 exit,
runtime dispatch, final Diagnostic / repair ABI, or sample-status relabel.

## What this manifest permits next

Use this file to keep the next package narrow:

1. If OBL-001 statement wording is refined, cite `ELAB-11` and `ELAB-12` for
   exact current static pressure and cite support rows only as mechanism
   guards.
2. If a future SCN negative gap is actualized, make it a targeted row and keep
   it separate from runtime/admission behavior.
3. If canon wording seems insufficient, draft a proposal only; do not edit
   canon without the human/canon process.

## Required non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No G2..G7 exit.
- No proof-obligation status movement.
- No proof discharge.
- No proof skeleton completion.
- No C-static, C-runtime, or C-distributed conformance claim.
- No runtime MessageEnvelope dispatch.
- No request serving, store mutation, occurrence ordering, admission lifecycle,
  stale-membership runtime failure, or distributed transport claim.
- No final grammar/API/Core IR/diagnostic/repair/runtime/transport/projection
  ABI freeze.
- No promotion of helper/sample/report/Lean compile-check evidence to canon.
- No promotion of `World`, `S`, `Player`, `position`, `hp`, `atk`, role names,
  keys, provider names, transport, or package artifacts into Mir core
  primitives.

## Open questions

- Does OBL-001 need an abstract predicate for visible publish / observe
  consequence, or can the current THM-001 wording and SCN-01 manifest carry it?
  This is answered for the current checkpoint by `plan/124`: no new predicate is
  needed for `ELAB-11` / `ELAB-12` / `ELAB-17`.
- Should direct-local-write rejection be represented as an explicit negative
  fixture later, or should positive owner-directed request shape remain enough
  for the initial G1 static bridge?

## Close condition

This file is closed when `plan/00-index.md`, `plan/90-source-traceability.md`,
the docs validators, current snapshot docs, and the package report are
synchronized.

Close condition is manifest-only: no canon edit, no gate exit, no proof, no
conformance claim, no implementation change, and no runnable sample status
change.
