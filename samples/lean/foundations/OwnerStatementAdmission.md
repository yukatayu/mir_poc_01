# W4-C ordinary source statements — LAB proof candidate

This is the same W4-C consumer, PL1/PL2 S4/S6. It connects an ordinary
assignment sequence to the existing checked owner executor. It neither closes
C nor starts D, resumes Plan250/I3-4, or promotes Canon/THM/OBL/119 status.
The current implementation still needs a handler manifest, protected invocation
cursor, local dispatch, and closure over all admitted entry and image routes.

The source-only integration manifest is
`docs/proof-first/W4_C_STATEMENT_SOURCE_MANIFEST.json`. It records 171 dependency
modules: 65 reused and 106 added. Eight import names are translated to existing
repository filenames. Definition bodies and namespaces are preserved exactly;
there is no second copy of an existing definition. W4-B's frozen manifest is
unchanged. The admission-cut Oracle review is final and its 14 findings are
dispositioned. The retained-history successor is mechanically checked; it has not
yet received a separate changed-cut review and does not close the production gate.

## General statements

`OwnerStatementIdentity` compares an independently defined positional
elaboration judgment with its executable lowerer/checker. It preserves order,
multiplicity, scope, ordinal and the whole supplied generic Code. The concrete
adapter must choose a complete Code representation; the theorem cannot discover
fields omitted from that representation or authenticate an input image.

`OwnerStatementSession` and the metadata counterpart preserve the exact
completed/stopped/remaining partition. Failed phase shape is intentionally weak.
`OwnerStatementBinding.Matches` applies to a stopped write **while waiting**;
it does not establish full binding for every failed session retaining a wait.
The actual acknowledgment checks equality of the entire saved request.
`OwnerStatementHistory` derives a retained owner-history witness with the same
site, activation, ordinal and control for each completed write in the current
activation. It does not assert ordered global history equality or archive coverage.

`OwnerStatementProgram.Compiled` retains the independently stated compiler
relation for the same stored program and entries, using an existential lowering environment
and stored compilation control, constructed at actual entry/adoption. The predicate
does not expose an independently retained environment field. Mutated current
values do not redefine the derivation.
Continuation/replacement supplies a new derivation from the actual compiler.
The relation composes with cursor partition and actual history correspondence.

`OwnerStatementAdmission.Initial` requires an actual underlying launched
transition history, successful metadata attachment, and separate initial registry
consistency. `Admitted` additionally carries the metadata transition history.
Their theorems derive the old preservation predicates from these premises.
This is a logical entry contract, not an executable arbitrary-image authenticator.
Registry consistency does not authenticate schema, classification, keys or authority.
Initial may attach after earlier actual commits at a drained prefix: attachment is
not necessarily launch or termination, and does not retroactively validate earlier
commits against the newly attached registry.

`OwnerStatementHistoryOrigin` proves that the underlying owner's history equals
the committed-result projection of its actual attempts. This equality is internal
to owner bookkeeping; an unacknowledged commit is still legal. Deleting attempts
preserves all older Facts, including compilation and history membership, but a
nonempty history with no attempts cannot satisfy the anchored Initial relation.
Thus merely rechecking those state predicates is insufficient for recovery.
`OwnerStatementRetainedHistory` preserves both exact history projection and the
request/result pairing of every committed attempt through all metadata transitions.
It derives them for Admitted and excludes erased attempts there too. Even Facts
plus exact history projection can hold when the saved request of a committed row
is substituted; the actual pairing invariant rules that counterfeit out.

`OwnerStatementPosition` enumerates every original source item before filtering
assignments. The actual compiler's indexed write projection equals that sequence.
Bounds, ordinal uniqueness, original-list decomposition, stopped-write origin and
the admitted indexed prefix follow. Assignment rank and generated setup indices
are different quantities. Actual invocation/program/handler scope is also required; activation and ordinal
alone are not globally unique identities.

`OwnerStatementCompletion.tick_delta` states that one actual tick either leaves
the completed-write projection unchanged or appends exactly the stopped waiting
write. The latter branch carries the actual consumed inbox reply, full saved-request
equality, acknowledgment, retained owner record and resulting state. The metadata
wrapper either publishes that candidate or retains the precise old-state refusal
frame and attempted control. Preparation is a pure model operation; the physical
consumer owes admission before publishing any request, enqueue or event.
`completion_changed_iff` characterizes write growth for the actual metadata tick.
The Accepted helper alone does not validate every field of an arbitrary successor.
Unchanged completion is not a no-effect or no-commit guarantee.

## Discriminators and limits

Controls execute the actual model compiler and source/owner/receive driver.
They cover S→T→S, a middle boundary refusal, genuine consumed arithmetic refusal,
postcommit acknowledgment refusal, old-reply substitution, ready gaps, new
activations, sparse ordinals 1/3, trailing ordinary work, and equal text/sites
with distinct attempts. Setup does not count as a source write. Completing every
assignment does not imply that trailing ordinary work has completed.

The forged-attempt control combines generic kernel-checked transformation and
exclusion theorems with finite executable checks of the witness premises.
Expanding its entire concrete execution into a kernel `decide` proof exceeded
the process memory cap during development. That failed run is retained and
excluded; no memory cap was raised and no proof hole was introduced.

The current metadata model can accept an acknowledgment after service followed
by an authorized metadata retirement. The strengthened controls show that the same packet passes before retirement and
fails after it, while packet/inbox/history remain unchanged. Fixture admission is
derived structurally from launch/attach/transitions; finite inhabited/growth
premises are evaluated separately before applying the general completion theorem.
That concrete result is recorded, not called acknowledgment-time metadata currentness. Current authorization and
service-time metadata checks remain distinct; Q18 and stronger recovery policy
are unchanged. Theorems do not supply authentic source/head custody, original
requester retention from a varying tick principal, resource availability, failed
wait recovery, archive correspondence, confidentiality, liveness or network proof.

Actual Rust countertests still expose the local source entry and sequencing
gaps. A temporary identity prototype makes the same-owner and different-owner
projections unambiguous while preserving the original full signature. It is not
adopted production semantics. Guarding SYS4 alone would leave a lower M8 request
entry: generated child names must not permit out-of-order execution. A protected
current invocation/statement binding is separate from authority and budget evidence.
Existing separate `at` blocks express the sequence. An unconditional same-block
parser splitter failed Canon's expression-span regression and was restored;
same-block syntax is explicitly unimplemented in this candidate.

## Reproduction and trusted base

Lean 4.29.1 with `--trust=0 -j1` checks all owned declarations against only
`propext`, `Classical.choice`, and `Quot.sound`. No authored `sorry`, `admit`, or
Mir-specific axiom is used. Fixed controls are not general proofs. The trusted
model inputs include initial source/metadata/auth contexts and serialized
transition execution; actual compiler ownership, admission, custody and physical
mechanisms remain separate correspondence obligations.

Run from the repository root. Set `PROOF_WORKDIR` to an existing external work
directory after checking capacity; the command copies source and emits objects
and logs only there. It does not use the historical handoff originals as a workdir.

```sh
python3 - <<'PY'
import os, pathlib, resource, shutil, subprocess, tempfile
assert 'version 4.29.1,' in subprocess.check_output(['lean', '--version'], text=True)
source = pathlib.Path('samples/lean/foundations')
base = pathlib.Path(os.environ['PROOF_WORKDIR']).resolve(strict=True)
work = pathlib.Path(tempfile.mkdtemp(prefix='owner-statements-', dir=base))
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
visit('OwnerStatementIntegratedAudit')
print(work, flush=True)
for name in order:
    with (work / (name + '.log')).open('x') as log:
        subprocess.run(['lean', '--trust=0', '-j1', '-o', name + '.olean',
                        name + '.lean'], cwd=work,
                       env=dict(os.environ, LEAN_PATH=str(work)),
                       stdout=log, stderr=subprocess.STDOUT,
                       preexec_fn=limits, check=True)
print((work / 'OwnerStatementIntegratedAudit.log').read_text())
# Each negative must fail with its own guard-false diagnostic; another
# compiler error or a successful guard is not evidence of rejection.
negative_sources = {'FalseConsumedRefusalCommits.lean': 'import OwnerStatementBoundaryControls\n'
                                     'open MirroreaProofFirst '
                                     'MirroreaProofFirst.OwnerStatementBoundaryControls '
                                     'MirroreaProofFirst.MixedOwnerContinuation\n'
                                     '#guard (overflow.map fun s => s.state.owner.history.length) '
                                     '= some 2\n',
 'FalseConsumedRefusalRetainsQueue.lean': 'import OwnerStatementBoundaryControls\n'
                                          'open MirroreaProofFirst '
                                          'MirroreaProofFirst.OwnerStatementBoundaryControls '
                                          'MirroreaProofFirst.MixedOwnerContinuation\n'
                                          '#guard (overflow.map fun s => '
                                          's.state.owner.queued.isSome) = some true\n',
 'FalseConsumedRefusalRunsLast.lean': 'import OwnerStatementBoundaryControls\n'
                                      'open MirroreaProofFirst '
                                      'MirroreaProofFirst.OwnerStatementBoundaryControls '
                                      'MirroreaProofFirst.MixedOwnerContinuation\n'
                                      '#guard (overflow.map fun s => '
                                      '(OwnerStatementPosition.writes (drive 6 s 0 '
                                      '7).cursor.completed).length) = some 3\n',
 'FalseEqualOccurrencesCollapse.lean': 'import OwnerStatementBoundaryControls\n'
                                       'open MirroreaProofFirst '
                                       'MirroreaProofFirst.OwnerStatementBoundaryControls '
                                       'MirroreaProofFirst.MixedOwnerContinuation\n'
                                       '#guard (equalLast.map fun s => '
                                       's.state.owner.attempts.length) = some 1\n',
 'FalseForgedAttemptHistoryExact.lean': 'import OwnerStatementBoundaryControls\n'
                                        'open MirroreaProofFirst '
                                        'MirroreaProofFirst.OwnerStatementBoundaryControls '
                                        'MirroreaProofFirst.MixedOwnerContinuation\n'
                                        '#guard forged.session.state.owner.history.length = '
                                        'forged.session.state.owner.attempts.length\n',
 'FalseLastWriteMeansTerminal.lean': 'import OwnerStatementBoundaryControls\n'
                                     'open MirroreaProofFirst '
                                     'MirroreaProofFirst.OwnerStatementBoundaryControls '
                                     'MirroreaProofFirst.MixedOwnerContinuation\n'
                                     '#guard (sparseLast.map fun s => s.cursor.remaining.length) = '
                                     'some 0\n',
 'FalseMetadataRecheckedAtAck.lean': 'import OwnerStatementBoundaryControls\n'
                                     'open MirroreaProofFirst '
                                     'MirroreaProofFirst.OwnerStatementBoundaryControls '
                                     'MirroreaProofFirst.MixedOwnerContinuation\n'
                                     '#guard (acknowledgedAfterChange.map fun s => '
                                     's.session.status) = some (.failed .rejected)\n',
 'FalseRealInitializerRefused.lean': 'import OwnerStatementBoundaryControls\n'
                                     'open MirroreaProofFirst '
                                     'MirroreaProofFirst.OwnerStatementBoundaryControls '
                                     'MirroreaProofFirst.MixedOwnerContinuation\n'
                                     '#guard drained actual = false\n',
 'FalseSparseRanks.lean': 'import OwnerStatementBoundaryControls\n'
                          'open MirroreaProofFirst '
                          'MirroreaProofFirst.OwnerStatementBoundaryControls '
                          'MirroreaProofFirst.MixedOwnerContinuation\n'
                          '#guard (OwnerStatementPosition.indexed 0 sparse.items).map Prod.fst = '
                          '[0,1]\n',
 'FalseCounterfeitSameRequest.lean': 'import OwnerStatementRetainedControls\n'
                                     'open MirroreaProofFirst '
                                     'MirroreaProofFirst.OwnerStatementRetainedControls '
                                     'MirroreaProofFirst.OwnerMetadataSession\n'
                                     '#guard (counterfeit.map fun (written,substituted,_) => '
                                     'decide (written.pending = substituted)) = some true\n',
 'FalseNoCompletedGrowth.lean': 'import OwnerStatementRetainedControls\n'
                                'open MirroreaProofFirst '
                                'MirroreaProofFirst.OwnerStatementRetainedControls '
                                'MirroreaProofFirst.OwnerMetadataSession\n'
                                '#guard OwnerStatementPosition.writes (tick changed 0 '
                                '7).session.cursor.completed = OwnerStatementPosition.writes '
                                'changed.session.cursor.completed\n',
 'FalsePacketInitiallyStale.lean': 'import OwnerStatementRetainedControls\n'
                                   'open MirroreaProofFirst '
                                   'MirroreaProofFirst.OwnerStatementRetainedControls '
                                   'MirroreaProofFirst.OwnerMetadataSession\n'
                                   '#guard (OwnerMetadataSessionControls.transferred.bind '
                                   'packetCurrent) = some false\n',
 'FalsePacketRemainsCurrent.lean': 'import OwnerStatementRetainedControls\n'
                                   'open MirroreaProofFirst '
                                   'MirroreaProofFirst.OwnerStatementRetainedControls '
                                   'MirroreaProofFirst.OwnerMetadataSession\n'
                                   '#guard '
                                   '(OwnerStatementBoundaryControls.changedAfterService.bind '
                                   'packetCurrent) = some true\n',
 'FalseRetirementBlocksActualAck.lean': 'import OwnerStatementRetainedControls\n'
                                        'open MirroreaProofFirst '
                                        'MirroreaProofFirst.OwnerStatementRetainedControls '
                                        'MirroreaProofFirst.OwnerMetadataSession\n'
                                        '#guard (tick changed 0 7).session.status = .failed '
                                        '.rejected\n',
 'FalseRetirementErasesCommit.lean': 'import OwnerStatementRetainedControls\n'
                                     'open MirroreaProofFirst '
                                     'MirroreaProofFirst.OwnerStatementRetainedControls '
                                     'MirroreaProofFirst.OwnerMetadataSession\n'
                                     '#guard changed.session.state.owner.history.length = 0\n'}
for filename, content in negative_sources.items():
    (work / filename).write_text(content)
    result = subprocess.run(['lean', '--trust=0', '-j1', filename], cwd=work,
                            env=dict(os.environ, LEAN_PATH=str(work)),
                            capture_output=True, text=True, preexec_fn=limits)
    diagnostic = result.stdout + result.stderr
    (work / (filename + '.log')).write_text(diagnostic)
    errors = [line for line in diagnostic.splitlines()
              if ': error:' in line or ': error(' in line]
    assert result.returncode == 1 and len(errors) == 1
    assert 'did not evaluate to `true`' in diagnostic
print('qualified false guards:', len(negative_sources))
PY
```

The integrated reconstruction checked 169 sources plus the audit; all 170 exited
zero. Nine intentionally false additional guard files exited one with their
specific guard-false diagnostic. The first wrapper misclassified a multiline
diagnostic; its failure is retained. A supplemental command requalified the
unchanged saved logs and completed the remaining controls without repeating the
successful sources. Exact receipts and failed attempts are in W4_CHECK and the
single accumulating Report2614. A successor freshly compiled the two new modules, reran the all-owned audit over
all 171 modules (17,861 owned declarations), and qualified six additional false
guards: 3 exit-zero commands and 6 intentional exit-one commands. Original
169-module reconstruction evidence remains frozen. Oracle advice is not execution,
independent signature, authority issuance or formal acceptance.
