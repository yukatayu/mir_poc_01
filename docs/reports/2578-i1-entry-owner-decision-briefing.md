# Report 2578 — I1 入口までの owner 判断 briefing

- Date: 2026-08-01T04:50:18.517743Z
- Author / agent: Codex
- Scope: existing Canon and S2-A LAB evidence を読むための decision briefing
- Decision levels touched: none. この briefing は判断を記録・変更せず、必要な
  owner/Canon decision を説明するだけである。

## Objective

I1 の単一プロセス参照実装を開始できる地点までに、owner が何を、いつ、なぜ選ぶ必要が
あるかを、前提知識なしで読める形にする。現在の直接 blocker と、後続の証拠が揃ってから
初めて行う受理判断を区別し、実装詳細を早期に固定しない。

## Scope and assumptions

- `mirrorea_canon/` が規範正本である。Plan 246、Report 2577、本 briefing は LAB
  evidence / repository memory であり、Canon を変更しない。
- `World`、`Game`、transport、View、provider は Mir Core の組込み概念にしないという
  既決方向を変えない。
- 説明用の疑似コードは Mir 構文提案ではない。意味論上の差を示すだけである。
- 「決める」は、将来の Canon proposal を無審査で確定する意味ではない。owner は
  N1/N2/N3 の candidate direction を選び、agent は最小 ordinary amendment draft を
  作る。その draft は ordinary Canon process で選択・凍結されてから S2-B の
  model/prototype が始まる。S2-B は未選択 candidate を試す場ではない。

## Start state / dirty state

`main` は `origin/main` と一致し、worktree は clean だった。直前の S2-A package
（Report 2577）は C1/C2 の比較を完了し、Canon 変更なしで owner 判断待ちになっている。
official lifecycle は依然 `T0`、OBL-001..028 は全件 `open` である。

## Documents consulted

- Canon hierarchy: `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`,
  `CANON.md`
- Core and scenario: `theory/01-mircore-v0.md`, `theory/03-elaboration.md`,
  `scenarios/SCN-02-attack.md`
- Direction and process: `meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md`,
  `PROPOSAL-013-post-admission-request-validation-context.md`,
  `PROPOSAL-017-c2b-c3-relation-state-envelope.md`, `adr/ADR-0013.md`,
  `adr/ADR-0014.md`, `plan/01-phases.md`
- LAB current state: Plan 246, Report 2577, `Documentation.md`,
  `docs/project-status.md`, `progress.md`, and `tasks.md`

## Actions taken

1. Rechecked the current Core `[READ-CROSS]`, `[WRITE-CROSS]`, handler failure
   containment, and SCN-02 source rather than relying on a prior summary.
2. Extracted every owner/Canon checkpoint on the path from the completed S2-A
   comparison to I1-entry stop, then classified it as `decide now`, `prepare
   then decide`, or `do not decide yet`.
3. Prepared the context-free decision explanation below. No candidate, source
   grammar, Core rule, Config field, failure member, SCN expectation, proof
   status, or lifecycle status was selected by this task.

## Files changed

- `docs/reports/2578-i1-entry-owner-decision-briefing.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short --branch`
- targeted `sed` / `rg` reads of the Canon/LAB documents named above
- `python3 scripts/new_report.py --slug i1-entry-owner-decision-briefing`
- `git diff --check`
- `make docs`

Reviewer and commit/push commands are recorded in closeout.

## Evidence / outputs / test results

### 1. Current location in one picture

```text
S2-A comparison complete
  -> CP-3: choose/reconcile C1 and C2                 [DECIDE NOW]
  -> selected ordinary amendment freeze                [before S2-B]
  -> S2-B: shared Core/Config/Step/Elab model          [agent research]
  -> S3: statement / SCN explanation                   [agent prepares; review]
  -> S4: narrow T2 and G5 skeleton                     [proof-facing work]
  -> S5: I1 readiness / all-SCN implementation scope   [prepare then decide]
  -> CP-7: explicit I1 authorization                   [DECIDE LATER]
  -> stop before the first I1 implementation package   [owner instruction]
```

This is **not** I1's immediate predecessor. It is the point immediately before
the shared semantic model on which all later proof and implementation work
would depend. Separately, the official lifecycle remains at T0; its G0/T1/T2
acceptance route must also be completed before an official I1 authorization.

### 2. Decisions needed now: the smallest useful ballot

#### Fixed constraints: not a fourth choice

N1/N2/N3 のどれを選んでも、P013/M1 の境界は変わらない。request-local claims と
recorded provenance は authority ではない。owner は principal、epoch、incarnation、
membership、capability/witness lineage、admission、visibility、history を live state に
照らして検査し、copied/replayed/stale/wrong-target/severed-lineage request を明示 failure
として store mutation なしで拒否する。transport/session/queue metadata も authority の
代わりにはならない。これは実装時に省略してよい詳細ではなく、選択済みの fail-closed
constraint である。

#### N1 — SCN-02's two reads and their authority

SCN-02 contains the ordinary source assignment:

```mir
S { player[target].hp = player[target].hp - player[self].atk }
```

The canonical expected static result says that both `player[target].hp` and
`player[self].atk` are cross-locus read dependencies. The current worked
elaboration displays only the `atk` dependency. In addition, `[READ-CROSS]`
requires visibility/observe authority and its generated failure set must be
contained by the handler's `fails` row. SCN-02 has a `fails` row, but neither a
visibility declaration nor `VisibilityDenied` containment.

This is not a question about whether the program is useful. It is a question
about preserving the project's authority boundary while making the canonical
example type-checkable and explainable.

| Choice | Meaning | Recommendation and reason |
| --- | --- | --- |
| **N1-A normal read authority** | Keep both dependencies as cross-locus reads. Require ordinary visibility/observe authority and add the generated failure treatment required by the selected rule. | **Recommend.** It preserves the existing rule that write authority never silently becomes permission to inspect private state. It is the smallest reconciliation consistent with SCN-02's own stated two-dependency expectation. |
| N1-B operation/declassification authority | Introduce a new, separately named authority by which an authorized operation may inspect an otherwise private RHS operand. | Defer. This can be useful in a future domain-specific policy, but it adds a new security and failure contract before a concrete need is established. |
| N1-C make a read owner-local / erase it | Treat one dependency as not cross-locus, or remove it from the dependency result. | Do not choose. It contradicts the frozen SCN-02 expectation unless the scenario itself is reopened. |

Concrete consequence: a user with permission to reduce a target's hit points
does not automatically learn a private `atk` value. Under N1-A, the action
must also satisfy the ordinary read/visibility policy, or fail explicitly.

#### N2 — C1: when is a read-dependent write evaluated?

With `hp=100` and `atk=10`, imagine two accepted attacks reach the same owner.

| Choice | Example result | What it means |
| --- | --- | --- |
| **C1-A-r owner-sampled atomic update** | first service reads 100 and writes 90; second reads 90 and writes 80 | The target owner validates, reads all same-owner RHS state, evaluates the bounded pure RHS once, and mutates once as one SW1 service. |
| C1-B requester-sampled determined value | both clients can read 100, calculate 90, and later write 90; final result can be 90 | Keeps the current `WRITE-CROSS(..., v′)` shape, but reads and write are separate operations. |
| C1-D defer | no semantic result is claimed | Does not enable a proof/runtime claim for SCN-02's `v′`. |

**Recommendation: choose C1-A-r.** It matches ordinary-program intuition for
an accepted attack against owner-local mutable state, hides communication from
ordinary source, and avoids silently accepting a lost update. Its scope is
strict: every dynamic RHS state dependency must have the same write owner. It
does not create a distributed transaction, common snapshot, retry, cache, or
multi-owner atomicity. If an expression needs state from another owner, it
stops/defer rather than pretending the operation is atomic.

N2 and N1 are intentionally separate. C1-A-r changes **when** same-owner
operands are sampled; it must not make write authority equal read authority.

#### N3 — C2: may a V1/R1 cross-locus read result resume later computation?

Consider an illustrative, non-Mir-syntax flow:

```text
request a remote read -> owner returns result -> requester accepts it once
-> use the result to form a later operation
```

The current `Config` has queues and stores but no semantic pending/result/
receipt/use state. P017 records the direction that a request occurrence `q`
needs a named relation-state residence (`X`), but deliberately does not choose
the final occurrence or storage presentation.

| Choice | Meaning | Recommendation and reason |
| --- | --- | --- |
| **C2-A-r candidate extension of X1** | Each in-scope V1/R1 request occurrence has a distinct semantic exchange state: owner pending/success/failure, requester receipt-pending/accepted, and one-shot use. The candidate states a causal path `request -> service/result -> receipt -> use`, plus cut/save/load closure. | **Recommend.** This is the minimum honest basis for V1/R1 result-dependent computation. It prevents two equal-looking requests from merging and prevents duplicate delivery or restore from enabling two uses. |
| C2-D defer | Do not add semantic result/receipt/use state on this line. | Valid only if V1/R1 result-dependent computation is deferred too. It cannot supply C1-B through this line; C1-B would require C2-A-r **or another separately selected R1 residence**. Queue order, spans, payloads, transport data, and evaluator-local tables cannot substitute for semantic state. |

Choosing C2-A-r now means accepting its invariant bundle: unique `q`-anchored
branches, owner failure has no mutation and enables no success receipt,
requester receipt is distinct from raw delivery, one accepted use at most, and
save/load cannot reset or merge that budget. In this candidate, the semantic
receipt is a separate requester occurrence, while consumption is a
zero-occurrence transition. P017 itself requires a receipt transition and
one-shot consumption but leaves the consumption presentation unselected;
C2-A-r proposes this presentation for the ordinary amendment. It does **not**
choose field names, wire messages, public IDs, timeout/retry/fairness, global
exactly-once, or a storage encoding.

### 3. The recommended immediate answer

The minimum coherent selection is:

```text
N1 = N1-A normal read/visibility authority
N2 = C1-A-r owner-sampled same-owner atomic update
N3 = C2-A-r X1-extension for result/receipt/one-shot use
```

This combination keeps authority explicit, gives ordinary same-owner
read-dependent actions the expected behavior, and supplies a V1/R1 result path
without turning the language into an event-driven or transport-defined
language. It does not require C1-A-r and C2-A-r to be one feature: SCN-02 uses
the former without a requester receipt; C2-A-r serves in-scope V1/R1
result-dependent programs. C1-B would also need C2-A-r or another separately
selected R1 residence.

### 4. What must be decided later, after evidence exists

| Later checkpoint | What the owner decides | Why not now | Current recommendation |
| --- | --- | --- | --- |
| Ordinary amendment freeze before S2-B | Freeze the exact selected Core/Config/Step/SaveObject delta, including whether an existing static read-request row licenses a response or a distinct static row is added | S2-B must model selected semantics, not choose them. P017 does not force either static representation, so the N3-selected amendment draft must make this minimum delta explicit and receive ordinary review first. | After N1/N2/N3, have the agent prepare the smallest draft; freeze it through the Canon process before S2-B. Reopen only if the draft exposes an unselected semantic choice. |
| CP-4 / CP-5 | Accept the T1 statement/profile package and narrow T2 + G5 proof-facing package | It requires model-derived statements, imports, and checks that do not exist yet. | Follow P016's recorded narrow-T2 then separate-readiness direction unless evidence exposes a defect. |
| CP-1 / CP-2 official governance | Keep the current pins/defer, or start a normal Canon rebase proposal; later accept only a valid `pass` artifact for G0-D3 | The current artifact is a valid `fail`; silent rebase/retry is prohibited. This does not block S2-B semantic work. | Retain/defer unless a normal rebase proposal is intentionally opened. |
| CP-6 / CP-7 I1 readiness and authorization | Accept the all-SCN I1 scope, profile, evidence classification, C-static wording, and explicit authorization | These are meaningful only after selected semantics, model, statements, and narrow-T2 evidence exist. | Reach this point, produce the requested I1-entry closeout, and stop before implementation. |

### 5. Things the owner does not need to decide now

- Final Mir grammar, error ABI, field names, static row spelling, wire format,
  transport protocol, public IDs, identity provider, timeout/retry/fairness,
  performance kernel, renderer, View UI, or federation.
- Whether `World` or `Game` become builtins: they remain user-defined concepts.
- A global transaction, global exactly-once delivery, or remote cache policy:
  neither C1-A-r nor C2-A-r claims them.
- Field layout and storage encoding for the selected amendment. C2-A-r's
  semantic receipt/zero-occurrence-consumption presentation is its proposed
  candidate and must be frozen before S2-B; it is not a runtime implementation
  representation to defer until after S2-B.

### 6. Documentary validation

`make docs` passed after the briefing and its reviewer corrections: the Canon
index reported 134 files, source hierarchy reported 796 required and present
with zero missing, and documentation validation reported 1,732 numbered
reports. `git diff --check` passed. No runtime/Cargo/Lean/sample command is
evidence for this documentation-only package.

## What changed in understanding

No theory changed. The key management result is that I1 preparation does not
need a large number of speculative owner choices. It needs three linked but
separable selections now (N1, N2, N3), followed immediately by an ordinary
amendment draft/freeze before S2-B, then evidence-backed acceptance at later
checkpoints. The existing P016 direction already supplies the high-level T2 to
I1 route; its operational profile must wait for the selected model.

## Open questions

The immediate open questions are N1, N2, N3, and the minimum amendment wording
needed to freeze the chosen candidate before S2-B. Field layout/storage encoding
and lifecycle acceptance remain deliberately deferred to the evidence-producing
checkpoints listed in section 4.

## Suggested next prompt

Reply with one line such as:

```text
N1-A, C1-A-r, C2-A-r を採用。S2-B の前に、C2-A-r の static response path を
含む最小 ordinary amendment draft を提示し、Canon process で凍結すること。
```

Or select a different option with the intended behavioral reason. After that,
prepare and freeze the ordinary amendment; only then resume the goal with S2-B
shared-model work.

## Plan update status

`plan/` 更新不要: Plan 246 already contains the authoritative LAB comparison,
options, falsifiers, and stop line. This report only repackages that evidence
for an owner decision and changes no roadmap or hypothesis.

## Documentation.md update status

`Documentation.md` 更新不要: current reader-facing status already separates
N1/C1/C2 from later I1 checkpoints. No status changed.

## docs/project-status.md update status

更新不要: the S2-A-complete / S2-B-owner-selection status is unchanged; this
report provides the requested explanation only.

## progress.md update status

`progress.md` 更新不要: no phase, evidence classification, blocker, or readiness
state changed.

## tasks.md update status

`tasks.md` 更新不要: CP-3 and the later CP-1..7 task map are unchanged.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample or validation surface changed.

## Reviewer findings and follow-up

The first independent reviewer found one high and three medium briefing errors:
the amendment freeze had been placed after S2-B, C2-A-r had been broadened past
P017's V1/R1 scope, P013/M1 fail-closed live validation was omitted, and
semantic receipt was conflated with zero-occurrence consumption. Each finding
was rechecked against Plan 246, P013, and P017, then corrected.

A fresh narrow reviewer cleared the corrections: the ordinary amendment freeze
is before S2-B; C2-A-r remains an in-scope V1/R1 candidate and C1-B permits it
or another selected R1 residence; M1 claims/provenance remain non-authoritative
with live fail-closed validation; and C2-A-r makes receipt a separate requester
occurrence while consumption is a zero-occurrence transition. No reviewer
edited files.

## Skipped validations and reasons

- No runtime, Cargo, Lean, or sample command is relevant: this package is a
  documentary reorganization of already reviewed evidence.
- Full documentation validation and two focused reviews completed before commit.

## Commit / push status

Pending closeout commit/push.

## Sub-agent session close status

Two read-only reviewer sub-agents completed and are closed. No sub-agent edited
repository files.
