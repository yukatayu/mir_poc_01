# W3 ordinary composition source (LAB reference candidate)

`main.mir` is handwritten source, not generated evidence or an expected trace.
Its unary functions compute values in Mir. One transition registers a checked
definition, instantiates a base on A/B/C and an addition on A/C, assigns an
ordinary local value, invokes instances, reparents, exchanges a compatible
definition and retires the addition. The base keeps its original behavior.

Run from repository root with an existing external directory:

```bash
python3 scripts/proof_first_composition_source_check.py --work-root /tmp
```

The runner builds the actual Rust `mir-ast` parser offline, copies the Lean import
cone, checks each module with Lean4.29.1 `--trust=0`, and executes the parsed source
through the same checked composition machine. It prints an external work directory
containing source hashes, command logs, actual values, requests/results and private
write/read-producer histories. Lean/Cargo children run sequentially with a 4 GiB
address-space limit; the source and generated evidence remain separate.

`controls/` contains adverse source inputs, not expected outputs. Additional
assignment, leave/rejoin, retirement, overflow, duplicate-binding and renamed-code
cases are generated in the external work directory by the case runner. The
assertions inspect the actual machine reached from each parsed source.

The selected private source profile has one transition, typed lifecycle provider
declarations, unary Int64 arithmetic/strict immutable function lets, explicit
finite input contracts and a represented locus list. Dynamic source bindings and
fallback/reacquire are still being connected. Extra imports, arbitrary higher-order
source, general contract plugins and write-through references are not implemented
by this adapter. W2's separate proofs do not imply these implementation features.

The initial logical realm/member/authority view is explicit test setup. Source
does not issue claims. It pregrants only the represented actions and targets;
declarations, types and successful proofs do not create authority. This is the
current-reauthorization profile, separate from Q-18 prepared reservation.

This is nonproduction in-memory reference execution and mechanization evidence.
It does not run physical nodes or transport, persist/recover a realm, implement
confidential observation, or close W3/alpha. It is outside the legacy clean-runner
and generated Lean manifests. General results and TCB are documented in
`samples/lean/foundations/MirroreaProofFirstDynamicComposition.md`; Report2613
retains the review findings, failed development runs and remaining gates.
