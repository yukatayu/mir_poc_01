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

`reference.mir` adds an owner-held reference with an explicitly declared fallback
chain, immutable alias, ordinary invocation, explicit reacquisition and release.
Its main execution uses the checked session boundary. Run this extension with:

```bash
python3 scripts/proof_first_reference_source_check.py --work-root /tmp
```

The command freshly builds the actual parser and complete proof import cone,
runs the main source and adverse cases, audits all owned declarations, and checks
designated-theorem weakening controls. In the printed external directory,
`CURRENT_RUN` locates actual value/write/event logs; `CURRENT_CASES` locates each
real parsed source variant. Three cases supply a separately parsed repair/addition
block to the same running session and retain its previous source archive. These
are local source execution evidence; no expected JSON is used as runtime output.
Generated-consumer receipt hashes are compared before the same verified bytes
are frozen, and originals/frozen files/audit source/manifest are rechecked at the
end. Integrity negatives use copies of actual generated consumers. Compare the
printed work directory's `MANIFEST.json` SHA256 with the `frozen_manifest` entry in
`docs/proof-first/W3_REFERENCE_CHECK.json` for exact source-cut reproduction.
A generic passing result verifies the captured candidate. Honest receipts and
exclusive work-directory ownership remain execution assumptions.

The selected private source profile has one transition per admitted block, typed lifecycle provider
declarations, unary Int64 arithmetic/strict immutable function lets, explicit
finite input contracts and a represented locus list. Dynamic source bindings,
fallback and reacquire are implemented in the finite reference. Extra imports, arbitrary higher-order
source, general contract plugins and write-through references are not implemented
by this adapter. W2's separate proofs do not imply these implementation features.

The initial logical realm/member/authority view is explicit test setup. Source
does not issue claims. It pregrants only the represented actions and targets;
declarations, types and successful proofs do not create authority. This is the
current-reauthorization profile, separate from Q-18 prepared reservation.

This is nonproduction in-memory reference execution and mechanization evidence.
It does not run physical nodes or transport, persist/recover a realm, implement
confidential observation, or complete alpha. W3 is closed as this finite reference candidate. It is outside the legacy clean-runner
and generated Lean manifests. General results and TCB are documented in
`samples/lean/foundations/MirroreaProofFirstDynamicComposition.md`; Report2613
retains the review findings, failed development runs, final checks and scope limits.
The final source/evidence cut81f82a0b is committed and normally pushed.

The provisional authority profile retains issued records/revocation tombstones
and strictly increases head generation. An unrelated head change also invalidates
old held references and pending invocation evidence. Cancellation needs its own
current claim; it records termination without creating a result or promising
remote nonexecution. Reacquisition does not renew absolute logical lease deadlines.
Cancellation may expire another request's lease even though its record survives.
These limits are exercised by the adverse cases, including a failed source repair.
H/C policy alternatives and Q-18 reservation remain distinct and unadopted in Canon.
