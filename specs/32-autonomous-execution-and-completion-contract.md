# 32 — Autonomous Execution and Completion Contract

## role

この文書は、`P-COMP-00` 以後の computational-core / PoseGraph / projection / engine-adapter line を、user が一度実行を依頼した後に agent が途中質問で止まらず進めるための completion contract を置く。

It does not replace subsystem specs. It defines how to sequence them, how to isolate unresolved user-level decisions, and when a package can be called complete.

## decision level

- `L1`
  - autonomous execution must preserve the repo's source hierarchy.
  - unresolved final-product choices must not block lower-layer implementation.
  - no package may claim completion without fresh validation evidence.
  - sub-agent review is evidence, not a substitute for local validation.
- `L2`
  - package order and closeout protocol for `P-COMP`, `P-POSE`, `P-PROJ`, and `P-ENG`.
  - default decisions for ambiguity.
  - reviewer / report / commit / push policy for long autonomous runs.

## autonomous scope

The autonomous line may proceed through a front-half boundary closeout and then implementation packages:

```text
P-COMP-01
P-POSE-01
P-PROJ-01
P-ENG-01
front-half closeout
P-COMP-02
P-COMP-03
P-COMP-04
P-POSE-02
all-up closeout
```

The line may also perform focused maintenance packages required to keep docs, validation, or source hierarchy consistent.

`plan/57` is repository memory for this chain. It is not a second live queue authority. The current promoted reopen point and current blocker map remain in `progress.md` and `tasks.md`.

The line must not silently widen into:

- final textual `.mir` grammar freeze
- final public ABI / SDK
- final public distribution / installer / hosted service
- direct LLVM or Mir-to-machine-code backend
- final server/client binary split
- arbitrary native or WASM execution
- Unity / Unreal / VRM / VRChat compatibility
- WAN / federation completion
- R3/R4 durable distributed save/load

## no-question execution rule

If a choice is needed during autonomous execution, use this order:

1. choose the smallest implementation that satisfies the current package close condition.
2. preserve existing specs and invariants.
3. mark wider choices as `user-spec-required` or `kept-later`.
4. add a machine-readable non-claim or scope block if a helper could be over-read.
5. continue with the lower-layer work that does not depend on the blocked final choice.

Ask the user only if all available choices would create an irreversible final-public commitment or destructive repository action.

## user-spec-required isolation

The following gates remain isolated and must not block the computational-core line:

- broader public distribution shape
- final shared-space catalog breadth
- final textual grammar
- final ABI / SDK
- hosted service / production WAN
- backend realization beyond inventory
- bounded native / WASM provider admission
- final engine adapter ABI

For these, the safe default is:

```text
current_status = user_spec_required
current_default = keep_alpha_local_boundary
implementation_effect = do_not_block_lower_layer
```

## package close protocol

Each non-trivial package must:

- read the relevant specs and plans before editing.
- define exact package objective and stop lines.
- add or update runnable samples only when the package explicitly requires implementation.
- include at least one positive and one negative row when the package claims runtime or checker behavior.
- update `specs/` only when normative boundary changes.
- update `plan/` when roadmap, sample matrix, or package order changes.
- update `progress.md`, `tasks.md`, and `samples_progress.md` when current status changes.
- add a new report under `docs/reports/`.
- run package-specific validation and the common validation floor.
- request focused sub-agent review near package close.
- wait for sub-agent completion unless there is concrete evidence of tool failure or a hung session.
- commit with `git commit --no-gpg-sign`.
- push after each package close unless push fails.

## front-half closeout line

The first autonomous cut closes the docs/spec and scaffold front half:

- `P-COMP-01`
- `P-POSE-01`
- `P-PROJ-01`
- `P-ENG-01`

This cut may create planned sample/helper surfaces and inventory rows, but it does not claim runtime completion, final grammar, projection codegen, native/WASM execution, final ABI, or engine integration.

After this cut, implementation may continue into:

- `P-COMP-02`
- `P-COMP-03`
- `P-COMP-04`
- `P-POSE-02`

The implementation half must add focused positive and negative evidence for every behavior claim.

## sub-agent policy

Sub-agents are used as reviewers or parallel implementers only for bounded tasks with clear ownership. They do not decide normative truth by themselves.

Minimum reviewer set for full autonomous line:

- theory / invariant reviewer
- runtime / toolchain reviewer
- sample architecture reviewer
- PoseGraph / spatial semantics reviewer
- projection / backend boundary reviewer
- docs / source hierarchy reviewer
- security / auth / capability reviewer when auth or host boundary changes

If a sub-agent does not return, retry once unless there is evidence of a tool outage. If still unavailable, record the skipped review and perform local focused review before closeout.

## master completion condition

The autonomous line is complete only when:

- `P-COMP-01` front-half scaffold condition and `P-COMP-02..04` implementation close conditions in `specs/28` are satisfied.
- `P-POSE-01` front-half scaffold condition and `P-POSE-02` implementation close condition in `specs/29` are satisfied.
- `P-PROJ-01` close conditions in `specs/30` are satisfied.
- `P-ENG-01` close conditions in `specs/31` are satisfied.
- the front-half closeout report has recorded all docs/spec inventory non-claims before implementation claims begin.
- active samples and helpers have documented positive / negative evidence.
- docs and dashboards distinguish workflow-ready, evidence-closed, boundary-fixed, and planned-only rows.
- common validation floor passes.
- final report states remaining non-claims.

This is not final public product completion.

## common validation floor

At minimum, package close must run:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Implementation packages must add focused tests and helper checks for the behavior they claim.

## stop lines

- Do not reduce scope silently.
- Do not mark planned-only roots workflow-ready.
- Do not use helper output as completion without actual package evidence.
- Do not treat host adapter behavior as Mir-owned computation.
- Do not collapse auth / capability / witness into transport.
- Do not treat renderer / engine / native provider state as world semantics owner.
- Do not skip validation and claim success.
