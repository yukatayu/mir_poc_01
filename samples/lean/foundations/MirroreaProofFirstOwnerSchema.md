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

The explicit index-parameter refinement in `MirroreaProofFirstOwnerTypedIndex`
checks a unique parameter against the complete state's declared index type.
Its independent `IndexTyped` judgment has executable soundness and relative
completeness, including duplicate occurrences. `explicit_exact` and
`footprint_exact` state the extension over declared parameters only; passing a
nonparameter index does not certify its typing. Required-schema filtering
preserves this check as well as the concrete parameter binding.

The separate `Binds` judgment requires a present capture with the matching
supplied type tag and retains its exact identity. Its accepted materialization
takes substitution rather than literal-name fallback, and reusing the binding
requires the same capture value. These are conditional data-binding statements;
the supplied entity tag is not authenticated by the checker. Principal/literal
typing, current capture labels, custody and runtime admission remain separate.
The reviewed static M7 refinement is applied to explicit index parameters at
both assignment targets and recursive RHS reads. An accessed duplicate parameter
is rejected at its second declaration; an index-type mismatch is rejected at the
whole reference. Scalar-only and unused duplicate parameters retain the earlier
finite behavior. Existing M6/declaration/failure checks precede this check; within
the reference helper, index mismatch precedes unknown-field lookup. Eleven source
tests cover these boundaries. These proofs do not change the low-level M8 fallback
or certify runtime capture admission.

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

## Exact invocation context (bounded LAB implementation)

The stronger static index rule does not supply a missing runtime argument. The
existing executor could use the literal entity named `target` when a declared
`target` was absent, or let an undeclared `self` argument override a nonparameter
RHS. These are actual source/executor counterexamples, not authorization bypasses.

`OwnerArgumentPresence` independently defines supplied declarations. It is
insufficient to exclude extra bindings. `OwnerArgumentDomain` uses a finite map
and independently defines exact distinct name-domain matching; `check_exact`
proves both directions. Parameter substitution and nonparameter preservation
use the actual supplied map. `invoke_exact` relates an explicit state/events/result
transition to its checker; refusal leaves protected body state and body events
unchanged. Queue, authorization, lifecycle and failure observations are outside
that frame. Unused invalid scalar strings can satisfy this domain predicate.

`OwnerInvocationContext` retains artifact, operation, actor, owner, statement and
parameter source references, full parameter order/multiplicity and generic Code.
The independent declaration/plan relation includes each field; Code stands for
all target/expression/budget material consumed by the next interpreter.
`invocation_code_result` uses separate source-Code and plan-Code interpretations.
`retained_domain_exact` checks duplicated outer/signature coordinates instead of
assuming them equal. `ordered_implies_inventory` connects the ordered producer
relation to distinct-key bidirectional inventory matching. Presentation-order
independence is not source execution-order independence or lookup correctness.

The bounded internal implementation retains this signature in M8 owner plans and
SYS3 owner fragments. Current authorization precedes the use-time exact-domain
guard; the guard precedes body materialization, reads and writes. Sealed creation,
private restoration and final fabric bootstrap compare complete inventories.
Private M8/SYS3 versions3/2 require the signature; old/missing forms fail closed.
Candidate image consistency remains separate from independently held expected
start binding and actual authority. Provider nested snapshots remain scoped.

This stricter provisional profile requires unused parameters, rejects extra names
and duplicate declaration names. Explicit `other=other` retains the old111 positive;
omission now refuses. Old fallback evidence remains historical. An explicitly
declared `self` still follows parameter lookup, not nonparameter preservation.
No new optional/default syntax, nominal value checker, issuer or public wire is
selected. The bounded Rust increment was integrated after the changed-cut review and local
dispositions recorded in Report2614; whole-C obligations remain open.

Actual finite tests distinguish M8 body/access/state from a result's reported
reads; cover auth precedence/revocation, copy/pending cut/patch, full private fields,
actual codec/start and provider scoped restoration. Removed/late/early guards and
independent image/bootstrap/patch/producer mutations fail their intended checks.
I3 handoff retains full carrier bytes before a synchronous owned move into M8;
the lower handoff predicate alone does not compare arguments. A test-only body
probe checks the complete consumed map, including unused values; a deliberate
post-verification unused-value change is detected. Expression tree and redundant
operator-chain mutations are checked separately at the structural decoder. Compiler ownership,
no mutating callback, live authority and expected-image custody are TCB premises.
M8 name-only selection across same-event multiple owners, ordinary source
continuation and the owner-local entry without a requester remain whole-C duties.
These lemmas/tests do not claim arbitrary Rust correctness, network, privacy,
persistence/recovery, alpha, or official THM/OBL acceptance.

## Reproduction and trusted base

These sources are outside the frozen W4-B 206-module manifest. Fourteen added modules
preserve the external research definitions and declaration namespaces; only
imports are renamed to the repository's filenames. The existing identical
`MirroreaProofFirstOwnerReadReport` is reused rather than defining its symbols
twice. The full dependency cone has 43 modules. The separate audit checks every
owned declaration's transitive axioms against `propext`, `Classical.choice` and
`Quot.sound`; these are not 43 independent correctness claims. The earlier
39-module required-schema and 40-module explicit-index checkpoints are retained
separately; the current 43-module reconstruction adds the invocation definitions.

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
visit('MirroreaProofFirstOwnerInvocationAudit')
print(work, flush=True)
for name in order:
    with (work / (name + '.log')).open('w') as log:
        subprocess.run(['lean', '--trust=0', '-j1', '-o', name + '.olean',
                        name + '.lean'], cwd=work,
                       env=dict(os.environ, LEAN_PATH=str(work)),
                       stdout=log, stderr=subprocess.STDOUT,
                       preexec_fn=limits, check=True)
print((work / 'MirroreaProofFirstOwnerInvocationAudit.log').read_text())
PY

cargo test --locked --offline -j1 -p mir-semantics --test proof_first_owner_schema
cargo test --locked --offline -j1 -p mir-semantics --test proof_first_owner_index
cargo test --locked --offline -j1 -p mir-runtime --lib proof_first_owner_schema -- --test-threads=1
cargo test --locked --offline -j1 -p mir-runtime --lib proof_first_invocation_ -- --test-threads=4
cargo test --locked --offline -j1 -p mir-runtime --test m8_runtime_owner_queue -- --test-threads=1
```

The trusted base includes Lean/kernel/standard library, Rust/compiler/M7 checking,
the correspondence between checked tree and sidecar, and the actual carrier's
source/operation binding. There are no authored proof holes or new Mir-specific
axioms. Fixed guards and Rust tests establish finite controls, not general Rust
compiler verification. Oracle is source-reading advice, not proof replay,
cryptographic independence, an issuer, or owner acceptance.
