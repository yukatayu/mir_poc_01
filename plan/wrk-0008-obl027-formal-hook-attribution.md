# WRK-0008 - OBL-027 formal-hook attribution audit

## Position

- `mirrorea_canon/` is normative; this document is LAB evidence and repository
  memory only.
- The pre-registration is
  `mirrorea_canon/working/WRK-0008-obl027-formal-hook-attribution.md` at
  `31365085cb1826e423dddf5f43db340623832301`.
- The question concerns the evidence attribution of the existing current-L2
  formal hook. It does not establish, deny, refine, or discharge Canon
  OBL-027, and it does not select a BND-003 carrier.

## Question

Does the current-L2 `runtime_try_cut_cluster` formal-hook artifact distinguish
the same-Place `atomic_cut` rollback frontier required by Canon theory/04, or
does it attach `rollback_cut_non_interference` from a coarser runtime event
classification?

## Reproduction

The registered command created a unique disposable directory under `/tmp`, ran
the two focused Cargo suites, emitted the four runtime formal-hook artifacts,
printed their normalized JSON, and ran the existing 23-command current-L2
regression set. All commands completed successfully:

| Check | Result |
| --- | --- |
| `current_l2_formal_hook_support` | 5 passed |
| `current_l2_source_sample_runner` | 2 passed |
| current-L2 regression | 23 / 23 commands passed |

The disposable artifacts were 456 KiB across 52 files and are not committed.

## Observed distinction matrix

| Sample | Relevant emitted event kinds | Terminal outcome | Formal-hook row |
| --- | --- | --- | --- |
| `e1-place-atomic-cut` | `atomic-cut`, no `rollback` | `explicit_failure` | `runtime_try_cut_cluster`; `rollback_cut_non_interference`; refs only to its fixture and runtime cluster |
| `e2-try-fallback` | `rollback`, no `atomic-cut` | `success` | the same row shape and only its fixture/runtime-cluster refs |
| `e21-try-atomic-cut-frontier` | both `atomic-cut` and `rollback` | `success` | the same row shape and only its fixture/runtime-cluster refs |
| `e22-try-atomic-cut-place-mismatch` | both, with the source cut nested in `profile_annotation` rather than the surrounding `draft_profile` | `success` | the same row shape and only its fixture/runtime-cluster refs |

The pinned helper accepts a detached bundle when any event kind is `rollback`
or `atomic-cut`. Its formal-hook output retains a subject identity and symbolic
fixture/runtime-cluster references. It does not retain an occurrence identity,
Place/locus, event order, pre/post-cut frontier, removed occurrence set, or
rollback-crossing relation.

The current-L2 interpreter has a separate LAB-local Place-sensitive rollback
snapshot path, and its focused runtime tests distinguish the `e21` and nested
Place `e22` final stores. That is counter-evidence to any claim that the
interpreter lacks locality. It does not repair the formal-hook artifact: the
row emitted by this route carries none of that locality/frontier relation.

## Result

The registered falsifier did not occur. The existing helper's formal-hook row
is an entry/reachability and identity reference for a runtime try/cut cluster;
it is not by itself evidence that a same-Place rollback cannot cross an
`atomic_cut` frontier. This is a scoped evidence-attribution result: the row
may remain a non-production formal-hook preview, but it must not be described
as a witness of Canon OBL-027's relation.

## Boundary

This result does **not** show that the current-L2 runtime violates Canon
theory/04, that Canon OBL-027 is false or proved, or that `e22` has Canon
Place semantics. It does not define history, occurrence identity, causality,
locus equality, rollback semantics, a proof/model-check carrier, diagnostics,
or a future helper schema. It changes no Canon text, OBL status, contract,
SCN, Gate, Phase, conformance classification, runtime behavior, public API, or
product claim.

## Reopen condition

Any future claim of `rollback_cut_non_interference` needs an authorized carrier
that records the relation actually being checked and an evidence path that can
distinguish the `e1`, `e2`, `e21`, and `e22` cases for that relation. Designing
such a carrier, changing the helper/schema, or mapping it to Canon OBL-027 is
outside WRK-0008 and requires the appropriate canon/owner route.
