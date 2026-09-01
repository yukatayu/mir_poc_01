# Report 2603 — I3-0 reliable-stream transport selection

- Date: 2026-09-02
- Author / agent: Codex parent/orchestrator with bounded implementer, test,
  evaluation, planner, reviewer and browser-backed Oracle advisory sessions
- Scope: `I3-0` / Mirrorea I3 Distributed Foundation
- Decision levels touched: L1 private transport selection under PROPOSAL-037 /
  ADR-0034

## Objective

Run TLS-over-TCP framed reliable stream and QUIC reliable stream through one
source/Core-bound actual-process comparison, select one private I3 adapter by
the fixed criteria, preserve transport non-authority/public non-freeze, and
activate I3-1. Direct consumer: I3-1 checked private carrier mapping for I3-2.

## Scope and assumptions

I3-0 is a bounded transport-selection probe, not the full adapter or owner
runtime. QUIC datagrams are excluded. The probe uses accepted I2 source and
retained owner-request carrier facts. It freezes no public wire, codec, version,
certificate, API/ABI, topology, platform set or production security profile.
The tested platform is Linux x86_64 localhost. Evidence is finite runtime-
monitored probe evidence, not a general proof, durability or exactly-once.

## Start state / dirty state

- Pinned committed start: `0b7ae97056dc48bd1beb65d319fd4ffa40a9ab9b`.
- `HEAD == main == origin/main`; branch `main`.
- Intentional I3-0 state only, with no pre-existing user change: modified
  `Cargo.toml`, `Cargo.lock`, `model.rs`, `sys5_local_slice.rs`; new facade test
  and `crates/mirrorea-i3-probe/`.
- ALIGN-0/1/2 completed; I3-0 active; candidates unselected; OPEN-032 open;
  official I3 unentered.
- Free space fell to about 5.7 GiB. Only recoverable
  `target/debug/incremental` was removed, restoring about 14 GiB; no source was
  deleted. Final evaluation ended with 13 GiB free, above the 10 GiB guard.
- No git worktree was created.

## Documents consulted

Canon-first review covered README, MAP, NORTH-STAR, DESIGN-CONSTITUTION,
architecture 01--08, plan/01 and plan/05, ADR-0033--0036, PROPOSAL-037--039 and
Plan 250 I3-0/I3-1. Accepted I2 carrier/projector/runtime source and Report 2602
were read as LAB evidence; reports were not scanned in bulk.

Oracle session `i3-0-semantic-harness-20260902` wrote
`/tmp/i3-0-semantic-harness-oracle.md`, SHA-256
`e7cb88f8af4f7b1e8568e0dc5608ef690b5e827ec347321d06028163f84814ac`.
Transcript artifact SHA-256:
`b3dcfc497e8445aabca32dbb19a5fba12c57711e001f5c46fb24c7eab573b139`;
output-log SHA-256:
`de43d41a3f239e470c2604c796003f6c1f8f0eb1b1dd369dabf765e1a4c48c`.
Metadata had `verified=false` and no resolved model label, so this is not
claimed as a verified GPT-5.6 Sol + Pro run.

## Actions taken

- Added a retained-carrier fingerprint facade derived from accepted I2 facts.
- Added private framing whose decoded value stays untrusted until exact
  retained-contract/request revalidation.
- Moved decode, admission, fixed-capacity cache and handler linearization into
  the receiver child; parent validates only child-derived events.
- Added a common nine-case supervisor, observer-safe rows, private-pipe
  credentials and forced-timeout cleanup falsifier.
- Implemented actual localhost TLS/TCP and QUIC reliable-stream candidates.
- Corrected the initial thick harness by test-driven changes after REJECT.
- Compared fixed criteria. Criteria 1--7 tied. Criterion 8 implementation/
  library maturity had no auditable winner; LOC/configuration surface is
  simplicity rather than maturity. Criterion 9 had no tested winner beyond
  Linux x86_64 localhost. Criterion 10 future browser relevance was therefore
  the first material difference and selected QUIC. TLS/TCP 584 LOC versus QUIC
  732 LOC and measurements of 0.22 s / 44,052 KiB versus 1.08 s / 43,944 KiB
  remain lower-ranked performance/C12 simplicity evidence only.
- Added PROPOSAL-040 / ADR-0037: private QUIC selected, TLS/TCP deferred as
  replacement baseline, OPEN-032 resolved only here, I3-0 closed, I3-1 active;
  official I3 still unentered.

## Files changed

Implementation/evidence: `Cargo.toml`, `Cargo.lock`; runtime `model.rs` and
`sys5_local_slice.rs`; facade test; new `crates/mirrorea-i3-probe/` manifest,
private child binary, common modules, TLS/QUIC candidates and eight test targets.

Canon: PROPOSAL-040; ADR-0037 and ADR README; Canon README, MAP, CHANGELOG,
generated INDEX; plan/01, plan/02, plan/05 and plan README; agent
instructions/source hierarchy.

LAB/current views: Plan 250, plan index, root README, Documentation,
project-status, progress, tasks, samples dashboard, root `CANON.md`, the
reader overview and its status regression test, and this sole I3-0 report. No
Lean/model/SCN/accepted sample or public/product surface changed.

- `docs/project-status.md`

## Commands run

Heavy evaluation used `CARGO_INCREMENTAL=0`, at most two jobs and the 10 GiB
guard. Final corrective-cut commands included:

```text
cargo test --locked -p mir-runtime --test sys5_i3_probe_facade -- --test-threads=1
cargo test --locked -p mirrorea-i3-probe --test frame_contract -- --test-threads=1
cargo test --locked -p mirrorea-i3-probe --test source_binding -- --test-threads=1
cargo test --locked -p mirrorea-i3-probe --test supervisor_falsifiers -- --test-threads=1
cargo test --locked -p mirrorea-i3-probe --test tls_tcp_candidate -- --test-threads=1
cargo test --locked -p mirrorea-i3-probe --test quic_candidate -- --test-threads=1
cargo test --locked -p mirrorea-i3-probe --test equal_comparison -- --test-threads=1
cargo test --locked -p mirrorea-i3-probe --test observer_safety -- --test-threads=1
cargo clippy --locked -p mirrorea-i3-probe -p mir-runtime --all-targets -- -D warnings
cargo test --locked -p mir-runtime --lib sys6_i2_conformance_tests -- --test-threads=1
cargo test --locked -p mir-runtime --test sys6_i2_cli -- --test-threads=1
cargo test --locked -p mir-runtime --test m10_conformance -- --test-threads=1
cargo test --locked -p mir-runtime --test m10_cli -- --test-threads=1
cargo test --locked --workspace -- --test-threads=1
cargo fmt --all -- --check
git diff --check
git diff -- . ':(exclude)Cargo.lock' | rg -n -i '(AKIA[0-9A-Z]{16}|-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----|github_pat_[A-Za-z0-9_]{20,}|ghp_[A-Za-z0-9]{36,}|sk-[A-Za-z0-9]{20,}|Authorization:[[:space:]]*Bearer[[:space:]]+[A-Za-z0-9._-]{10,})'
```

An attempted `--test sys6_i2_conformance_tests` failed exit 101 because it is a
library module; the correct `--lib` command passed. Earlier retained evidence
pre-dated correction, but final evaluation reran it after the corrective tranche.
Canon/docs commands were:

```text
(cd mirrorea_canon && python3 meta/build-index.py)
(cd mirrorea_canon && python3 meta/build-index.py --check)
python3 -m unittest scripts.tests.test_build_index -v
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_mirrorea_project_overview_html -v
python3 scripts/validate_agent_configs.py
python3 -m unittest scripts.tests.test_validate_agent_configs -v
make docs
git diff --check
```

## Evidence / outputs / test results

- Focused I3-0 21/21: facade 3, frame 8, source 2, supervisor 3, TLS 1, QUIC 1,
  equality 1, observer 2.
- Fresh rerun timing: TLS 0.21 s, QUIC 0.99 s, equality 1.14 s.
- Clippy for probe + runtime all targets passed with warnings denied.
- Fresh retained floors: SYS-6 25/25 (52.03 s) + 8/8 (10.19 s); M10 67/67
  (9.81 s) + 4/4 (0.15 s).
- Locked workspace tests/doc-tests exited 0; Cargo emitted no aggregate count.
  Format/diff passed; no probe child remained.
- Duplicate evidence: two receives/revalidations, one handler and stored result;
  both transports emitted equal normalized semantic rows.
- Final code/harness review: ACCEPT, P0=0, P1=0, eight grouped P2 residuals.
- Canon index generation/check passed with 202 files; index unit tests passed
  5/5. Source hierarchy passed 800/800. Agent configuration and its unit tests
  passed 9/9.
- Full docs validation and `make docs` passed with 1,757 numbered reports.
- The first reader regression run exposed stale Report-2602 expectations and
  current HTML markers. After the reader/test owners synchronized I3-1 active,
  the private selected-adapter state and bounded OPEN-032 resolution, all 12/12
  reader tests passed. Historical ADR-0033/SYS-7 unselected wording remains
  explicitly historical.
- New PROPOSAL-040 (5,228 bytes), ADR-0037 (4,151) and active plan/05 (13,354)
  are below 15,000 bytes. Cumulative navigation/history MAP and CHANGELOG remain
  above that threshold by existing design. The scoped high-confidence secret
  pattern command above returned `rg` status 1 (expected no match); diff checks
  passed.

## What changed in understanding

Actual sockets are insufficient when a parent supplies expected semantic
outcomes. The accepted probe makes the receiver child own semantic work and
lets the parent validate facts only. The transports had no bounded semantic or
safety difference in criteria 1--7. The closeout review exposed that LOC and
configuration surface had been misclassified as criterion-8 maturity. After
restoring the owner-fixed criteria, 8 and 9 also tie and criterion 10 is the
first actual difference, so QUIC wins. TLS/TCP's simpler canary remains useful
replacement evidence but cannot override the fixed ordering.

## Open questions

Eight grouped I3-1 residuals, not I3-0 blockers:

1. credential/private-key `Vec<u8>` zeroization;
2. second deadline after kill/wait/join and canary-local cleanup wording;
3. macOS/Windows/browser/production portability;
4. no mTLS/client auth/live grant admission, plus explicit no-mint before reuse;
5. stronger TLS post-admission ambiguity ordering evidence;
6. fixed 8-entry in-memory/no-eviction cache is not owner runtime/durability/
   exactly-once;
7. duplicate JSON keys and wrong-marker classification separate from unknown
   version;
8. request-hash v2/v1 label alignment and owner-request-only facade scope.

Public wire/API, I3-2 runtime, full I3-3 ordering and official I3 entry remain
later. No owner-reserved decision is needed at this cut.

## Suggested next prompt

Continue with I3-1 only: implement exhaustive checked private carrier mapping
over selected QUIC reliable stream, close the eight residual groups with round-trip/property/
fuzz and typed negatives, preserve TLS/TCP only as replacement evidence, and do
not begin I3-2 runtime early.

## Plan update status

`plan/` 更新済み: Plan 250 now records the truthful criteria, I3-0 close, full active I3-1 Goal
Statement, P2 residuals and inactive I3-2. Plan index mirrors the transition.

## Documentation.md update status

`Documentation.md` 更新済み: private QUIC selection, TLS/TCP deferred baseline, bounded
OPEN-032 resolution, I3-1 active and lifecycle/public non-claims.

## docs/project-status.md update status

更新済み: I3-0 completed, I3-1 sole active, official I3 unentered and selected
adapter private.

## progress.md update status

`progress.md` 更新済み: current axes/tables and a command-derived
`2026-09-02 02:52 JST` close log without owner-runtime/product overclaim.

## tasks.md update status

`tasks.md` 更新済み: fully rewritten as a current snapshot with only I3-1 active, ordered packages,
I3-2 next, discovery questions and owner-reserved stops.

## samples_progress.md update status

`samples_progress.md` 更新済み: an evidence-closed runnable I3-0 probe row, command and I3-1
blocker; explicitly not workflow/product completion or official I3 entry.

## Reviewer findings and follow-up

First review REJECTed four P1s: `CaseExpectation` fabricated semantic outcomes;
the facade was lossy; setup/deadline/cleanup flags were declarative; and
candidate tamper/QUIC labels diverged. Test-driven correction moved semantic
work into the receiver child, made the owner-request fingerprint exhaustive,
derived lifecycle evidence from child handling and used one common tampered
frame. Common-carrier re-review ACCEPTed with no P0/P1.

The first selection-closeout review REJECTed one P1 because LOC/configuration
simplicity had been misclassified as criterion-8 maturity, plus one P2 for a
missing exact secret-scan command. The parent restored the fixed order, selected
QUIC at criterion 10, added the command, and synchronized current Canon/status.
A corrective re-review then REJECTed four stale current mirrors and two P2
wording/byte-count issues; all were corrected before the final re-review. The
final selection-closeout re-review ACCEPTed with P0=0/P1=0/P2=0 and authorized
commit/push. The implementation-review P2 groups remain assigned to I3-1.
Oracle independently
recommended the receiver-child ownership shape; the parent verified it locally
and retained the model caveat above.

## Skipped validations and reasons

Lean `--trust=0`, bounded ordering models, full I3-3 failure injection,
SCN-01/02/03/06, browser/macOS/Windows and production/WAN tests were not run:
I3-0 changes no such claim and those surfaces are later or unsupported. No
skip is a pass. Full plan/05 failures remain I3-3.

The reader suite initially failed stale Report-2602 frontier expectations; this
was not counted as a pass. Parent-assigned reader/test owners synchronized the
current I3-1 state, and the final 12/12 rerun passed. Cross-platform/browser,
Lean/model and later-I3 validations above remain genuinely skipped.

## Commit / push status

No I3-0 commit or push had been made when this report was prepared; it cannot
embed its own future hash. Parent commits with `--no-gpg-sign`, pushes, verifies
`HEAD == main == origin/main` and a clean worktree, then records the cut.

## Sub-agent session close status

Completed: common facade implementer/reviewer, process common, TLS, QUIC, test
author, Oracle operator, final candidate reviewer, final evaluator, planner/
status writer and reader-status synchronizer. Parent remains active for
integration, commit/push/parity and I3-1 continuation.
