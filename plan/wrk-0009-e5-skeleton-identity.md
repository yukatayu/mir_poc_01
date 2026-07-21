# WRK-0009 e5 proof-skeleton literal identity evidence

**LAB evidence.** This file is not Canon and does not change an OBL, theorem,
carrier, runtime, helper/schema, Gate/Phase, conformance, or sample workflow.
It records the command and literal comparison registered in
`mirrorea_canon/working/WRK-0009-current-l2-e5-skeleton-identity.md` at clean
pinned base `1b2b542f132f4fef2d71ea413ff2d26172dd08bc`.

## Question and fixed comparison rule

The question is whether the Lean foundation's ordered
`(subject_ref, obligation_kind, theorem_name)` tuple for e5 literally equals
the tuple emitted by the existing current-L2 static route. Equality means exact
spelling, punctuation, order, and each displayed field. This record does not
infer a mapping from hyphen/underscore spelling or obligation labels.

## Reproduced command and environment

On 2026-07-22 JST, the registered command ran from clean pushed `main` with
temporary root `/tmp/mirrorea-wrk0009-e5-skeleton.EZcJT9`:

```bash
workdir="$(mktemp -d /tmp/mirrorea-wrk0009-e5-skeleton.XXXXXX)" && \
lean samples/lean/foundations/CurrentL2ProofSkeleton.lean && \
sed -n '17,40p;56,64p' samples/lean/foundations/CurrentL2ProofSkeleton.lean && \
cargo test -p mir-semantics --test current_l2_lean_theorem_stub_support && \
python3 scripts/current_l2_theorem_lean_stub_pipeline.py e5-underdeclared-lineage --artifact-root "$workdir" --run-label wrk0009 && \
jq -S '[.[] | {subject_ref, obligation_kind: .row.obligation_kind}]' "$workdir/proof-notebook-review-units/wrk0009-e5-underdeclared-lineage/e5-underdeclared-lineage.proof-notebook-review-unit.json" && \
jq -S '[.[] | {subject_ref, obligation_kind, theorem_name}]' "$workdir/lean-theorem-stubs/wrk0009-e5-underdeclared-lineage/e5-underdeclared-lineage.lean-theorem-stub.json" && \
python3 scripts/current_l2_source_sample_regression.py regression --artifact-root "$workdir/regression" --run-label wrk0009
```

The temporary root is disposable and is not a retained repository artifact.
Before execution root storage was 188 GiB total, 167 GiB used, and 13 GiB
available; memory available was 8.9 GiB.

## Literal matrix

| Position | Foundation display | Existing emitted static route | Literal result |
| --- | --- | --- | --- |
| 1 | `e5-underdeclared-lineage`, `rollback_cut_non_interference`, predicted `e5-underdeclared-lineage__rollback_cut_non_interference` | `e5_underdeclared_lineage`, `canonical_normalization_law`, `e5_underdeclared_lineage__canonical_normalization_law` | mismatch in subject spelling, obligation label, and theorem name |
| 2 | `e5-underdeclared-lineage`, `no_re_promotion`, predicted `e5-underdeclared-lineage__no_re_promotion` | `e5_underdeclared_lineage`, `no_re_promotion`, `e5_underdeclared_lineage__no_re_promotion` | mismatch in subject spelling and theorem name |

The foundation defines `mkLeanStub` as subject reference, two underscores, and
`obligationName`; the predicted foundation theorem names are the direct literal
expansion of that displayed rule. The emitted route has two review units and
two Lean stubs with two matched pairs. Its formal-hook JSON has the same two
emitted contract rows, and the e5 static-gate JSON reports `underdeclared`.

## Result and stop line

**Result class: literal mismatch.** The registered equality test fails at both
positions. The result is limited to artifact identity. It does not establish
whether the difference is a mapping, intentional synthetic role, defect, or
semantic divergence; no relation is selected and no repair is made.

The theorem-stub support target passed 4 tests. The current-L2 source regression
completed all 23 commands, including e5 static formal hook, theorem-stub, and
model-check carrier conformance. These executions show the existing lane
remains runnable at this base; they do not promote artifacts to Canon or prove
the foundation relation.

The pre-registration's status-quo prose spells the second emitted obligation as
`no_repromotion`; execution and the emitter display `no_re_promotion`. This
evidence file preserves the executed spelling and does not rewrite the
pre-registration. The following working-record manifest must add a dated
clarification; this does not change either literal-mismatch row.

## Next reopen condition

Keep this outcome as a bounded mismatch until a separately registered question
supplies explicit source evidence for a lossless mapping or intentional
synthetic-role statement. Do not alter the proof skeleton, pipeline, helper,
schema, runner, Canon theory, OBL ledger, or lifecycle from this record.
