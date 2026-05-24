# plan/67 — source patch hot-plug roadmap

## purpose

This document is repository memory for
`specs/42-source-patch-hotplug-semantics.md`.

It keeps source patch hot-plug on the checked source pipeline rather than direct
eval.

## current decision

Decided:

- source patch pipeline is parse -> typecheck -> elaborate -> compatibility /
  admission -> HotPlugRequest -> HotPlugVerdict -> activation_cut.
- rejected patches do not mutate active runtime state.
- deferred patches remain visible in lifecycle evidence.
- patch authority is capability/admission based.

Not decided:

- final hot-plug ABI.
- distributed activation ordering.
- durable migration.
- production patch signing / registry workflow.

## package order

| Package | Role | Completion gate |
|---|---|---|
| `P-SURF-00B` | docs/spec rebaseline | patch semantics and roadmap exist |
| `P-SURF-06` | source patch pipeline | `check-source`, `parse-source`, `elaborate-source`, `patch-source`, and `export-core-ir` have accepted/rejected rows |
| `P-SURF-08` | devtools | source span, Core IR diff, verdict, activation cut, and state migration summary visible |
| `P-SURF-99` | audit | compatibility anchors rerun and non-claims preserved |

## planned rows

- `PATCH-01` source patch adds visible state.
- `PATCH-02` undeclared failure rejected.
- `PATCH-03` self-grant of server authority rejected.
- `PATCH-04` patch lifecycle devtools export accepted.

## validation anchors

Future anchors:

```bash
python3 scripts/surface_mir_samples.py run PATCH-01 --format json
python3 scripts/surface_mir_samples.py run PATCH-02 --format json
python3 scripts/surface_mir_samples.py run PATCH-03 --format json
cargo test -p mir-runtime --test source_patch_hotplug -- --nocapture
cargo test -p mirrorea-cli --test surface_mir_cli -- --nocapture
```

## stop lines

- no direct eval.
- no mutation on rejected patch.
- no patch self-grant authority.
- no distributed durable migration claim.
