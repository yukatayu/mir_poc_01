# W4-C actual owner read receipts — bounded LAB correction

Direct consumer: SYS4's successful owner branch in `LocalFabric::step_locus`
through `OwnerRmwReport`. R05/R06/R12; reads are dependencies, writes actual
occurrences. This does not close those requirements, W4-C, or Plan250 I3-4.

An ordinary literal assignment `avatar[self].hp = 34` produced a receipt claiming
an hp=0 read. M8's successful read map was empty; SYS4 enumerated the target plus
Core read candidates and defaulted absent lookups to zero. A non-target RHS
`avatar[self].atk + avatar[self].atk` also produced an invented target row.
These were actual checked-source/admission/dispatch/receipt assertion failures.

The bounded correction preserves the existing resolved candidate order and
physical-key deduplication, emitting a row only for `Some(value)` in the recorded
M8 outcome. `Some(0)` remains a real read; `None` is no recorded read. Values are
copied from the completed outcome, not re-read from post-write state. A target
which aliases an RHS reference is correctly included once. Repeated accesses
coalesce in the map; the resulting vector does not claim temporal access order.

The smallest alternative is direct enumeration of the actual map. That removes
candidate coverage as an enumeration premise but requires an internal accessor
and handling the existing target-first presentation expectations. The local
Some-only correction fixes the demonstrated invention without changing those
expectations. This is a reversible internal choice under the owner's delegation;
Oracle advice does not decide authority or public contracts.

## Definition, proof and implementation obligations

The standalone Lean module uses `Std`. `Faithful` independently requires output
membership to be justified by an input pair and every input pair to have a row.
`report` is the actual wrapping function. Ten general lemmas establish membership,
list roundtrip/length, empty input, fabricated/dropped-pair controls, and selection
from an immutable partial lookup. All are kernel checked with Lean4.29.1,
`--trust=0 -j1`; no `sorry`, `admit` or Mir-specific axiom. Dependencies are
standard `propext`/`Quot.sound` or none. `Classical.choice` remains allowed only
in the broader imported-foundation audit, not asserted as used by this module.

`selected_report_exact` says a row exists exactly when its key is a candidate
and its actual lookup has that value. `covered_report_exact` removes candidate
membership ONLY under coverage of the entire successful lookup domain. Neither
it nor the Rust change proves that coverage from source. `Faithful` by itself
permits duplicates and reordered rows; the concrete `report` has the stronger
roundtrip theorem. Rust uniqueness comes from resolving before `seen_reads`;
instantiate Lean's candidate list with that deduplicated sequence. Do not call
the selected list the whole actual map without the separate coverage premise.

The current source correspondence has these inspected implementation obligations:

- Parser state references and tree leaves come from the same collected parts;
  checked Core retains the state-reference inventory. This inspection is not a
  compiler theorem.
- Private expression restore recomputes tree facts and rejects unequal read,
  literal or operator metadata before making checked values.
- SYS4 and M8 resolve namespace/index/field with the same argument-or-original-name
  fallback on successful admitted references. M8 rejects missing index/field;
  no successful actual read of such a reference is silently supplied.
- Core, arguments, outcome, locus and selected plan must belong to the same use.
  Existing SYS4 checked patches preserve owner Core and reject pending carriers;
  ordinary M8 patches reject pending owner requests. Full entry/restore refinement
  remains a separate C obligation.

TCB includes Lean's kernel/toolchain/Std, Rust/compiler/map implementation, honest
extraction and plan/argument/locus binding. This repair neither authenticates an
arbitrary supplied map nor authorizes exposing it. `OwnerRmwReport` is internal;
source labels, current release, secrecy and physical isolation remain separate.
The empty-read `M8OwnerRead` phase marker is not certified here as an actual read
occurrence. No failed scratch-read history is reconstructed by this successful
receipt representation.

## Reproduction and evidence

Use the external-copy loop in `samples/lean/README.md`, or copy this standalone
module outside the repository and run Lean4.29.1 with `--trust=0 -j1`.
Actual Rust regression filters are:

```sh
cargo test --locked --offline -j 1 -p mir-runtime --lib proof_first_ -- --test-threads=1
cargo test --locked --offline -j 1 -p mir-runtime --lib -- --test-threads=1
```

Report2614 and W4_CHECK record the original assertion failures, exact command
results, mutations/restoration, axiom audit, frozen Oracle packet and local
review disposition. Fixed controls and Rust tests are finite evidence, not
universal evaluator/compiler proofs. This module is outside frozen W4-B's
206-module manifest. OwnerPartial/state/strict-coverage candidates remain external
research and are not promoted by this narrowly scoped receipt correction.
