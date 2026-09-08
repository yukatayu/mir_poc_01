# Project status

最終更新: 2026-09-08 10:53 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

Owner control: finish I3-3 through validation, independent review, commit/push
and parity, then pause. I3-4 requires explicit owner resume. This does not
weaken I3-3's gates, close the program or mark it blocked; I3-3 remains active
until acceptance, after which Plan 250 is retained with no active milestone.

## この文書の役割

これは人間向けの短い **派生ビュー** である。規範判断は
`mirrorea_canon/`、closed execution historyはPlan 247 / 249、詳細証跡はmilestone
reportsにある。この文書はGate/Phase、OBL、proof、compatibility、goalを決めない。

## 全体の進行チェックリスト

```text
closed M0--M10 finite reference baseline
-> [x] SYS-0 baseline / goal alignment
-> [x] SYS-1 runtime kernel / internal carrier
-> [x] SYS-2 ST / OW1 concurrency refinement
-> [x] SYS-3 checked Core -> per-locus artifacts and generated plans
-> [x] SYS-4 in-process generated dispatch
-> [x] SYS-5 four-locus toy + typed devtools
-> [x] SYS-6 finite I2 assurance / lifecycle closeout
-> [x] SYS-7 inactive I3 entry contract only / program closed
-> [x] ALIGN-0 bounded-program activation / meta-drift alignment (completed)
-> [x] ALIGN-1 project/product layer constitution (completed)
-> [x] ALIGN-2 Browser/Host/package/View/provider boundary contracts (completed)
-> [x] I3-0 transport candidate evidence and private selection (completed; `mirrorea_canon/adr/ADR-0037.md`)
-> [x] I3-1 checked private adapter/encoding boundary (completed; `mirrorea_canon/adr/ADR-0038.md`)
-> [x] I3-2 two-or-more-process generated-artifact runtime (completed/accepted bounded evidence; `mirrorea_canon/adr/ADR-0039.md`)
-> [ ] I3-3 network failure/order refinement (active after owner resume)
```

Plan 247とPlan 249はclosed recordsである。PROPOSAL-037 / ADR-0034により
Mirrorea I3 Distributed Foundation bounded programは継続中で、owner resumeによりI3-3
のみactiveである。Plan 250がsole current roadmap、ALIGN-0 / ALIGN-1 / ALIGN-2 /
I3-0 / I3-1 / I3-2はcompleted、I3-4/I3-5/I3-6/NEXT-0はdependency-gated inactiveである。

## 現在地

| 観点 | 状態 | 根拠 |
|---|---|---|
| theory | **T1** | `mirrorea_canon/plan/01-phases.md` |
| broad PHASE-I1 | **unaccepted**; OPEN-026/027とfull carrier freezeが残る | `mirrorea_canon/architecture/04-runtime-carriers.md` |
| bounded I2 lifecycle | **official entry accepted, then official exit accepted** | `mirrorea_canon/adr/ADR-0032.md` |
| ADR-0026 program | **SYS-0--SYS-7 closed** | `mirrorea_canon/adr/ADR-0033.md` |
| active roadmap / goal | **Plan 250 / I3-3 active after owner resume at accepted I3-2 cut** | `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md` |
| I3 / OPEN-032 | **bounded program current; I3-3 active and later milestones dependency-gated; lifecycle entry not official / resolved only for this program by ADR-0037** | `mirrorea_canon/adr/ADR-0037.md`, `mirrorea_canon/adr/ADR-0039.md` |
| public/product | final grammar/CLI/API/ABI/wireもproductionも未受理 | `mirrorea_canon/adr/ADR-0033.md` |

Accepted SYS-6 implementation/evidence cutは
`5429712de89a7e41c46cfd7fb4a39c4a492864c4`、Canon/status integration cutは
`bcb0f767edbb3e9e581c3b4c7f2a49e077f44067`である。provisional
`mir conform-i2`はexact 22 finite rowsを検査するがlifecycleをself-authorizeしない。

SYS-7は候補A TLS-over-TCP framed reliable-stream adapterと候補B QUIC reliable-
stream adapterをともにUNSELECTEDで保持した。I3-0は両候補の同一private
source/Core-bound nine-case actual-process canaryを通し、criteria 1--7のtie後、criteria
8/9に勝者がなく、criterion 10 future browser relevanceを最初のmaterial differenceとして
Bをprivate selected adapterとした。Aはrejected/deferred replacement baseline、QUIC datagramは未admit・
未評価である。public version、codec、wire、certificate、API/ABI、deployment、platform
又はproductionは未選定で、transport/session/certificate/route metadataはauthorityではない。

## 現在の停止線

ALIGN-0 / ALIGN-1 / ALIGN-2 / I3-0 / I3-1 / I3-2 completed、owner resume後はI3-3のみactiveで、I3-4/I3-5/I3-6/NEXT-0はdependency-gated inactive。ALIGN-2 は Browser/Host/package/View/provider の責任境界を Canon 化し、BND-010..BND-016、trust tier T0–T4（Theory T0–T2 とは別）、package admission と semantic grant の分離、raw FFI 禁止、redaction と resource/termination 責任を明示した。固定順序はALIGN-0..2 → I3-0..6 → NEXT-0である。
ALIGN-1ではsemantic strata S0--S6、project/product PL-0--PL-6、lifecycle T0--T2 / I1--I6を独立したmany-to-many座標としてCanon化した。PL-4は責任境界のみ、PL-6は別application、satellitesは別系統である。
ALIGN-0 acceptanceはI3 lifecycle entry、transport選定、production/public freezeを
含まない。これらは各後段gate又はowner-reserved boundaryへ残る。
Current authority and milestone gates are
`mirrorea_canon/adr/ADR-0034.md` and
`plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`.
I3-3 is the only active promoted semantic package after explicit owner resume;
I3-4/I3-5/I3-6/NEXT-0 remain inactive pending their dependency gates. The
accepted I3-2 source cut and remote parity are the preserved entry evidence.

Prior checkpoint `87ee2418` retains this slice evidence: full-runtime
integration 51/51 (44.43s), evaluator owner-capacity 1/1, I3-3 filter 4/4, probe
library lifecycle evidence 2/2 (one filtered), selected runtime library 1/1 +
1/1 (old-owner withdrawal and exact-ledger different-snapshot), actual probe
integration 17/17, and `cargo fmt --all -- --check` pass. The
historical ledger-source binary remains 49/49 in 47.70s only. Bounded model
evidence remains 3/3 tests (432 states, 2136 transitions) with three mutant
counterexamples, not a general runtime proof. Full 20-family matrix remains
unverified; post-admission retry remains duplicate-rejected with requester
pending/ambiguous rather than successful recovery.
Runtime-only retry 2/2 and full same-feature library 285/285 (82.33s) now pass.
Focused two-crate all-target Clippy with `-Dwarnings` passes; model-target
dead-code warnings without its private feature are not a deny-lint profile.
The actual reconnect slice covers two live OS processes: the pre-write path
consumes one checked receipt; in the separate post-admission path, retry after
a lost reply is duplicate-rejected while the requester remains pending/ambiguous.
The successor slice installs genuine M9 revocation before reconnect and rejects
retained old ingress without owner mutation, keeping A pending. G1 completes
once; registered-B ACK publication, actual tainted A input, one-shot ingress
and terminal validation pass independent repair review (no remaining P0/P1).
Delivery-checkpoint runtime integration 61/61 (55.23s), I2 local 5/5 (0.05s), I2 CLI 8/8
(11.60s), probe 32/32, private QUIC unit 2/2, focused Clippy and format/diff
pass. Feature library 289/289, M10 67/67 and model 3/3 remain prior evidence;
earlier probe lifecycle 2/2 is prior evidence. I2 regressions pass (5/5, 8/8).
The delivery checkpoint executes actual endpoint-unavailability, complete
two-application-write round-trip and strict truncation/FIN rejection before
owner admission. Only validated owner records reach the joined rejection
view; genuine presence and malformed absence are both tested. Full probe
library 3/3 also passes. These do not close whole fault families or I3-3.
ADR-0041/spec/16 C3 checkpoint: all six named budget tests (seven cases), full probe 38/38, runtime library 332/332, integration 63/63, I2 SYS5 5/5, SYS6 CLI 8/8, M10 67/67, Clippy and format are green. Narrow local-wait review is closed: B serves/withholds and closes, A observes no reply and completes its actual wait without independently classifying a peer-close error. Actual row-11 reply replay remains next; genuine G1→G2 requester stale-binder evidence is already local-green and must retain network-path correspondence. Full-time review, provider and the remaining matrix/order stay open. No row-17/I3-3 acceptance.
Remaining authority/order and full matrix validation stay open; I3-4 is inactive.

Detailed edge contracts: [`mirrorea_canon/architecture/07-browser-host-trust-boundaries.md`](../mirrorea_canon/architecture/07-browser-host-trust-boundaries.md).
Cross-edge binding/freshness/revocation/redaction/resource rules: [`mirrorea_canon/architecture/08-browser-host-security-invariants.md`](../mirrorea_canon/architecture/08-browser-host-security-invariants.md).
View は authoritative domain semantics を所有せず、presentation-local computation のみを許可する。View からの入力は typed command/effect request とし direct store を禁止する。I3-0 はprivate transport選定をclosedし、OPEN-032はこのbounded programだけresolvedした。I3-1とI3-2はbounded evidenceとしてclosedした。official I3 lifecycle は未entry、I3-3のみactive、I3-4/I3-5/I3-6/NEXT-0はdependency-gated inactive、I5 implementation は inactiveである。

I3-2の最終bounded evidenceはlocalnet 12/12（repeat）、full probe 62/62、runtime default 29/29、seam 47/47、library 281/281、docs compile-fail 1/1（default/private）である。6/11 milestones acceptedであり、これは重み付き完成率ではない。FM-5 bounded executable ratchetで、public workflowや100% completionは主張しない。I3-3全体は未受理であり、過去のcleanupは許可されたtarget/debug/incrementalのみ。root free spaceは現在約12.03 GiBで、current runtime integration 63/63とprobe 38/38を確認した。

The active bounded I3 programはinternal carrierとpublic wireを分離し、route/handshake/framing/
disconnect/reconnect/ambiguous delivery/duplicate/reorder/stale authority/backpressure/
timeout/provider/redaction/patch/cut failureをtypedに扱い、network occurrencesをMir
orderingへrefineしなければならない。hidden retry、exactly-once、hidden transactionは不可。

I3-4/I3-6のC-distributed evidenceはordinary-source SCN-01/02/03/06のpositive/falsifier、
source/Core/artifact/carrier/network/runtime correspondence、observer-safe diagnostics、
evidence classification、independent reviewを必要とする。I2 evidenceだけでは満たさない。

Reopen accepted I2 evidenceはmissing/manual edge、owner movement、direct remote store、
source-free mint、selected ST/OW divergence、stale cut/patch mutation、relation/designated
drift、observer leak、lower-layer conformance dependency、M10 regressionの場合だけ。

## オーナーの確認・判断待ち

OPEN-032はPROPOSAL-040 / ADR-0037によりこのbounded programだけresolvedした。
owner/userの明示的resumeによりI3-3をactivateした。これはI3-2 cutとremote parityを
保持した再開であり、I3-4以降は前段acceptanceまでactivateしない。
このpause gateに加え、次のbounded sequence外の変更もowner decisionを必要とする。

- public API/ABI/wire/grammar/CLI compatibility freeze;
- production deployment、external publication、paid resource;
- North Star、authority/privacy/redaction/no-stale guaranteeの変更;
- World/Avatar等のCore primitive化、hidden multi-owner transaction; および
- Constitutionでも解けないirreversible semantic tie。

Authority boundaryは`mirrorea_canon/meta/agent-instructions.md`と
`mirrorea_canon/adr/ADR-0034.md`を参照する。これらは未完了SYS-7 taskではない。

## 根拠と詳細

| 読みたい内容 | 一次の確認先 |
|---|---|
| Canon entry | `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md` |
| lifecycle | `mirrorea_canon/plan/01-phases.md` |
| SYS-6 acceptance | `mirrorea_canon/adr/ADR-0032.md`, `mirrorea_canon/spec/15-sys6-i2-conformance.md` |
| inactive I3 contract | `mirrorea_canon/adr/ADR-0033.md`, `mirrorea_canon/plan/05-i3-entry-contract.md` |
| active bounded I3 program | `mirrorea_canon/adr/ADR-0034.md`, `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md` |
| I3-0 private transport selection | `mirrorea_canon/adr/ADR-0037.md`, `docs/reports/2603-mirrorea-i3-distributed-foundation-i3-0-transport-selection.md` |
| Browser/Host trust edges | `mirrorea_canon/architecture/07-browser-host-trust-boundaries.md` |
| cross-edge security invariants | `mirrorea_canon/architecture/08-browser-host-security-invariants.md` |
| proof/evidence class | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| closed I2 roadmap | `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md` |
| SYS-6 close evidence | `docs/reports/2598-mirrorea-i2-systems-foundation-sys6-i2-conformance-closeout.md` |
| SYS-7 close evidence | `docs/reports/2599-mirrorea-i2-systems-foundation-sys7-i3-entry-contract-closeout.md` |
| runnable commands | `samples_progress.md` |

Inherited validation floor: SYS-6 25+8、SYS-2/3/4/5 28/28/104/62、M10 67+4、
workspace。I3-0 adds private facade/frame/source/supervisor/TLS/QUIC/equality/
observer test evidence, format, warnings-denied focused Clippy, diff and final
independent ACCEPT; exact current results and skipped reruns are in Report 2603.

## 更新規約

active program/roadmap、official lifecycle、major blocker、accepted cut、evidence class、
またはuser-visible commandが変わるtaskで同期する。authorityは常にCanonへ戻し、
詳細履歴はone milestone reportへ置く。未実行validationをpassと書かず、helper/reportを
general proofやpublic product completionとして数えない。
