# Report 2268 - OBL-026 transparent-overlay composition boundary audit

## Objective

Determine whether the transparent-overlay conditions in theory/02 alone
derive a complete proof-facing stack-composition statement for OBL-026.

## Scope and assumptions

Canon remains normative. The disposable Lean models are LAB evidence about the
formalization boundary, not a counterexample to the canonical
transparent-overlay policy, a canonical `Contract` representation, a layer
ABI, a proof of OBL-026, or an implementation decision.

## Start state / dirty state

The worktree was clean at `d11f3ba1`. T-RESEARCH-015 recorded its Discord task
baseline before work and placed its Lean experiment only under `/tmp`. The only
tracked change before this record was the prepared T-RESEARCH-015 section in
`plan/156-t0-t2-research-autonomy-envelope.md`.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and theory/11
- Canon theory/01, theory/02, theory/04, theory/05, theory/07, theory/08,
  theory/09, and theory/10
- LAB `plan/156`, `tasks.md`, `progress.md`, and `docs/project-status.md`
- `.docs/oracle-chatgpt-pro-operations.md` and the Oracle operating guide

## Actions taken

- Re-read theory/02's transparent-overlay paragraph and preserved all ten
  stated directions: input, output, precondition, postcondition, combined
  effect/failure, ordinary-path capability, provided surface, observation,
  redaction, and retention.
- Kept `cost_bound` outside that list because theory/02 records it separately
  as OPEN-013; this audit neither chooses nor models its algebra.
- Built a disposable ten-field natural-number preorder kernel. Under its
  expressly experiment-local componentwise directions, pairwise
  transparency is transitive.
- Built a separate three-label opaque relation with the first-to-second and
  second-to-third labels but no first-to-third label. It shows that pairwise
  labels alone do not supply the semantic order or composition law required by
  the positive kernel.
- Attempted a temporary Oracle source-selection consult twice. Both attempts
  failed before prompt submission in the browser model picker, so no external
  answer was used and the candidate was selected through local source reading.

## Files changed

- `docs/reports/2268-obl026-transparent-overlay-composition-boundary-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

The disposable source remains outside the repository at
`/tmp/mirrorea-t-research-015/OverlayCompositionKernel.lean`.

## Commands run

- focused canon/LAB source searches with `rg` and `sed`
- `lean --trust=0 /tmp/mirrorea-t-research-015/OverlayCompositionKernel.lean`
- forbidden-element scan and `sha256sum` over the disposable Lean source
- `df -h .` and `free -h` before broad validation
- `make check`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- final documentation/source-hierarchy and Git diff checks listed below

## Evidence / outputs / test results

- Frozen source-adequacy result: `0 direct / 0 delegated / 1 missing` coupled
  transparent-overlay composition formalization boundary. This is one complete
  proof-facing row; it does not deny the direct canonical policy directions.
- Theory/02 directly states the ten transparent-overlay directions, the
  `Layer : Contract -> Contract` shape, explicit `ContractUpdate` for a
  non-transparent layer, and the declared-branch constraints on `all_of` /
  `any_of`. It does not define a proof-facing `Contract` carrier, directional
  component orders, a layer-stack composition operation, or an equality /
  extensionality relation from which the OBL-026 statement follows.
- The positive Lean kernel proves transitivity only after selecting ten
  componentwise natural-number preorders. Its directions are not a proposal
  for canonical variance. The opaque-label theorem separately demonstrates
  that a bare relation name cannot replace those preorders and a composition
  law; it is not a countermodel to the canon.
- Trusted Lean execution passed. The source scan found no `sorry`, `admit`,
  declared axiom, `opaque`, `unsafe`, `partial`, or `implemented_by`. `#print
  axioms` reports no axioms for `transparent_transitive`; the opaque-label
  theorem reports Lean `propext` only. Scratch hash:
  `7885b7f7673f66602033cadce123300c7bdd78e3b2207a76b26ac6dca08998e1`.
- Before broad validation, the root filesystem had 21 GB free (89% used) and
  the system reported about 9.2 GB available memory. The package adds no
  tracked build artifact and keeps the Lean model under `/tmp`.
- `make check` passed: source hierarchy found all 704 required paths, document
  validation accepted 1,422 numbered reports, and `cargo check` passed.
  `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed
  all 21 tests. `git diff --check` passed before staging.

## What changed in understanding

The canonical prose gives a complete policy checklist for one transparent
layer, but it does not itself fix the mathematical structure needed to derive
stack composition. The missing part is coupled: each policy direction needs a
chosen order, and the proof needs a layer-stack composition and equality or
extensionality interface. Recording that boundary preserves the policy without
prematurely selecting its variance or ABI.

## Open questions

- What proof-facing carrier represents `Contract` and its fields?
- What are the canonical directions and relations for the ten policy fields?
- What operation represents composition of a layer stack, and what equality or
  extensionality rule applies to its result?
- How do `all_of`, `any_of`, `ContractUpdate`, and the deferred cost algebra
  connect to a later OBL-026 statement without weakening the fixed policy?

## Suggested next prompt

Select another independent existing-lane source cut, or prepare a canon
formalization decision only when an OBL-026 proof-facing package must choose
the contract order and layer-composition interface.

## Plan update status

Updated: plan/156 records the ten-direction source cut, the conditional
preorder kernel, opaque-label boundary, stop threshold, and non-claims.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points did not change.

## docs/project-status.md update status

更新済み: the human view now separates the fixed transparent-overlay policy
from the missing order/composition/equality formalization interface.

## progress.md update status

Updated: current research and the dated recent log include T-RESEARCH-015.

## tasks.md update status

Updated: T-RESEARCH-015 is closed as LAB source-adequacy evidence and the
next source selection excludes silently choosing its contract-order or
composition interface.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample changed.

## Reviewer findings and follow-up

No advisory Oracle conclusion was received. Temporary consultations
`next-theory-source-cut-015` and `next-theory-source-cut-015-2` both failed
before the prompt was submitted because the browser model picker could not
select the requested model despite listing a similarly named option. Per the
retry rule, the second concrete failure ended retries. Local review then
re-read the source cut, verified that the model contains ten rather than nine
policy directions, and kept its two finite models explicitly non-canonical.
No local sub-agent service was available.

## Skipped validations and reasons

Runtime, distributed execution, conformance, and product checks do not apply
to this documentation and disposable-Lean source audit. The runnable sample
corpus was not rerun because no sample, runner, or implementation source
changed.

## Commit / push status

The final package commit uses `--no-gpg-sign` and is pushed immediately after
this status record is included.

## Sub-agent session close status

No local sub-agent service was available. The two temporary Oracle sessions
did not submit a prompt and produced no advisory result; their concrete
pre-submit failure is retained above rather than being represented as review.
