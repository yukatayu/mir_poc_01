# W4-C owner schema preservation — bounded LAB evidence

Direct consumer: `sys3_projection::lowering::project_owner`. An ordinary checked
assignment may write `player[target].hp` using `shield[target].hp` at the same
owner. Keeping only the target namespace drops a real dependency. The existing
projection and private image already carry complete checked schema objects;
the correction selects the target and every RHS read namespace in source order.
It keeps unrelated namespaces out and preserves complete declarations, including
index type, fields, owner and declaration source references.

This is the same W4-C goal, PL1/PL2 S4/S6, R03/R05/R09. It does not close C,
activate W4-D, resume Plan250/I3-4, or promote Canon THM/OBL/phase status.

## General statements and interpretation

`MirroreaProofFirstOwnerRequiredSchema.lean` defines `retain` over complete state
declarations and `needed` over the target plus every strict expression read.
`candidates_retain` preserves the **entire candidate list** for each needed key,
including multiplicity, order, missing declarations and ambiguous refusals.
`select_retain`, `bind_retain` and `tree_retain` lift that equality to selection,
current metadata binding and the expression. `assignment_retain` preserves the
complete result of binding any assignment, for arbitrary state declarations,
current metadata and arguments. `assignment_relative_exact` connects the result
to the independently defined original `AssignmentBinds` judgment.

The retained original schema is an input, not an authority credential. The
projection theorem in `MirroreaProofFirstOwnerCheckedSchema.lean` establishes
state/field membership origin; it does not authenticate arbitrary input. Its
global selector requires a unique **namespace/field candidate**. Two declarations
with the same namespace and disjoint fields can still resolve individually.
Actual M7 rejects duplicate namespaces before field lookup; this stronger
invariant is part of the compiler/input boundary, not a corollary of flattening.

Index type and index binder are retained by the actual schema carrier but erased
from the model's field declaration. Full checked artifact identity, intended
operation, typed entity/capture interpretation and occurrence source references
must remain attached before that projection is consumed. Per-operation field
coordinates and per-artifact owner numbers are not global runtime identities.

`parameters_evaluation` is a homomorphic equation under a pullback argument
environment. It does not establish that independently supplied captures equal
that environment: mapping two distinct captures to one coordinate can change
`a - b` from 5 to 0. The actual M8 evaluator still looks up arguments by name.
Capture typing, labels, control dependencies and source installation remain
separate obligations for the later source/Session connection.

## Implementation and evidence boundary

The runtime tests use real ordinary source, M7 checking, SYS3 projection and
private JSON round trips. They cover a different read namespace, repeated reads,
source order, constant RHS, owner-only restriction, the provider static path,
and refusal by the source verifier after a required schema is erased from a
decoded candidate. Private decoding alone is not executable-image admission.
Existing physical expected-image custody remains a distinct prerequisite.

The source-order statement is about each operation's schema. The locus-wide
`ProjectedCheckedFragments` list is a sorted occurrence inventory: two operations
can contribute the same complete declaration twice. Current execution and seed
validation consume the operation-local schemas. The inventory is not a second
globally unique schema environment, and the overlap control preserves that
distinction through a private image round trip.

An older image that omitted a required declaration is not made complete by an
unchanged source identity. A consumer must compare against a freshly derived
projection or the appropriate expected image. This change does not migrate or
silently repair old images and establishes no public image compatibility policy.

The semantics tests independently walk the executable tree, compare ordered
typed reads with the retained sidecar, and resolve every read against a unique
namespace and field. Changed unused declarations alter full artifact identity
while both present owner Cores remain structurally equal. That equality is
`PartialEq`, not a serialized-byte comparison. Duplicate-namespace and unsupported
parentheses controls check their specific diagnostics.

Optional test exports contain actual checked facts, not expected runtime events.
The external finite JSON-to-Lean controls use explicitly supplied generations
and labels. They are not import validators, label authorities or fake E2E traces.
Visibility requests do not lower confidentiality or grant observation release.
No new network, persistence/recovery, secret noninterference or resource guarantee
follows from this schema correction. Exact runs, initial failures, mutations and
Oracle dispositions are retained in Report2614 and `docs/proof-first/W4_CHECK.json`.

## Reproduction and trusted base

These sources are outside the frozen W4-B 206-module manifest. Ten added modules
preserve the external research definitions and declaration namespaces; only
imports are renamed to the repository's filenames. The existing identical
`MirroreaProofFirstOwnerReadReport` is reused rather than defining its symbols
twice. The full dependency cone has 39 modules. The separate audit checks every
owned declaration's transitive axioms against `propext`, `Classical.choice` and
`Quot.sound`; these are not 39 independent correctness claims.

Run from the repository root with Lean 4.29.1. Set `PROOF_WORKDIR` to an existing
external working directory with enough space. The following copies source only,
builds serially with a 6 GiB process limit and retains logs outside the repository:

```sh
python3 - <<'PY'
import os, pathlib, resource, shutil, subprocess, tempfile
assert 'version 4.29.1,' in subprocess.check_output(['lean', '--version'], text=True)
source = pathlib.Path('samples/lean/foundations')
base = pathlib.Path(os.environ['PROOF_WORKDIR']).resolve(strict=True)
work = pathlib.Path(tempfile.mkdtemp(prefix='owner-schema-', dir=base))
seen, order = set(), []
def visit(name):
    if name in {'Lean', 'Std', 'Init'} or name in seen:
        return
    seen.add(name)
    file = source / (name + '.lean')
    for line in file.read_text().splitlines():
        if line.startswith('import '):
            visit(line[7:].strip())
    shutil.copyfile(file, work / file.name)
    order.append(name)
def limits():
    resource.setrlimit(resource.RLIMIT_AS, (6 * 1024**3, 6 * 1024**3))
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
visit('MirroreaProofFirstOwnerSchemaAudit')
print(work, flush=True)
for name in order:
    with (work / (name + '.log')).open('w') as log:
        subprocess.run(['lean', '--trust=0', '-j1', '-o', name + '.olean',
                        name + '.lean'], cwd=work,
                       env=dict(os.environ, LEAN_PATH=str(work)),
                       stdout=log, stderr=subprocess.STDOUT,
                       preexec_fn=limits, check=True)
print((work / 'MirroreaProofFirstOwnerSchemaAudit.log').read_text())
PY

cargo test --locked --offline -j1 -p mir-semantics --test proof_first_owner_schema
cargo test --locked --offline -j1 -p mir-runtime --lib proof_first_owner_schema -- --test-threads=1
```

The trusted base includes Lean/kernel/standard library, Rust/compiler/M7 checking,
the correspondence between checked tree and sidecar, and the actual carrier's
source/operation binding. There are no authored proof holes or new Mir-specific
axioms. Fixed guards and Rust tests establish finite controls, not general Rust
compiler verification. Oracle is source-reading advice, not proof replay,
cryptographic independence, an issuer, or owner acceptance.
