# Phase 6 — compile-ready minimal actualization

## この phase の主眼

Phase 6 は、Phase 1〜5 で fixed した semantics / parser-free PoC / proof boundary を entry criteria としつつ、
`mir-ast` / `mir-semantics` / `mir-runtime` の actual code path を **non-production minimal cut** で compile-ready にする phase である。

この phase は production parser / runtime を確定する段階ではない。
主眼は、

- actual parser carrier
- checker / runtime skeleton bridge
- compile-ready gate と formal hook

を narrow に揃えることである。

## ここまでに fixed したこと

### 1. parser first tranche の cut

`specs/examples/299...300` により、Phase 6 front-half actual parser / AST carrier first tranche の current first choiceは、

- `mir-ast` crate 本体へ `mir_ast::current_l2` を追加する
- accepted floor を stage 1 option/chain と stage 2 try/fallback structural surface に留める
- stage 3 admit / request clause / predicate fragment、perform head final public API、span-rich diagnostics、final grammar は retained-later に残す

というものに固定した。

### 2. code anchor

parser first tranche の actual code anchor は次である。

- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs`

ここでは stage 1 / stage 2 spike tests を crate API import へ寄せ、TDD の red/green を通して narrow carrier を昇格した。

### 3. checker / runtime first tranche の cut

`specs/examples/301...302` により、Phase 6 front-half actual checker / runtime first tranche の current first choice は、

- `mir-semantics` に `static_gate_program_detailed`、`DirectStyleEvaluator::from_program`、`FixtureHostStub::run_program` を足す
- `mir-runtime` に `mir_runtime::current_l2` thin orchestrator を足す
- parser side evidence は stage 1 reconnect summary と stage 2 try/rollback structural summary の optional bridge input に留める
- bridge input と semantic `Program` が食い違うときは fail-closed に止める

というものに固定した。

### 4. compile-ready verification / formal hook の cut

`specs/examples/303...304` により、Phase 6 front-half compile-ready verification / formal hook の current first choice は、

- selected gate を `cargo test -p mir-ast`、`cargo test -p mir-runtime`、`cargo test -p mir-semantics --test current_l2_*` selected suite、Python detached-loop suiteに留める
- `crates/mir-semantics/examples/current_l2_emit_formal_hook.rs` と support helper で tool-neutral emitted formal hook を actualize する
- emitted hook の envelope / row core は Phase 5 theorem-line existing cut と同じ `subject_kind + subject_ref + contract_rows(obligation_kind + typed symbolic evidence_refs)` に揃える
- source artifact の `schema_version` / `artifact_kind` mismatch は fail-closed に止める

というものに固定した。

## まだ fixed していないこと

- stage 3 request / predicate reconnect
- source target only row `e3` の authored-set widen timing
- bridge sketch / compare-bless metadata
- concrete theorem / model-check tool binding
- perform head final public parser API
- span-rich diagnostics と final grammar
- LLVM-family backend / external codegen binding の timing

## この phase の evidence

- `specs/examples/287...290`
- `specs/examples/299...300`
- `specs/examples/301...302`
- `specs/examples/303...304`
- `specs/examples/311...312`
- `specs/examples/313...314`
- `specs/examples/315...316`
- `specs/examples/317...318`
- `specs/examples/319...320`
- `specs/examples/321...322`
- `specs/examples/323...324`
- `specs/examples/325...326`
- `specs/examples/327...328`
- `specs/examples/329...330`
- `specs/examples/331...332`
- `specs/examples/333...334`
- `specs/examples/335...336`
- `specs/examples/337...338`
- `specs/examples/339...340`
- `docs/reports/0611-phase6-actual-parser-ast-carrier-first-tranche-package.md`
- `docs/reports/0612-phase6-actual-checker-runtime-skeleton-first-tranche-package.md`
- `docs/reports/0613-phase6-compile-ready-verification-and-formal-hook-package.md`
- `docs/reports/0614-phase6-compile-ready-checkpoint-drift-suppression-and-mirror-sweep.md`
- `docs/reports/0620-phase6-parser-followup-sequencing-package.md`
- `docs/reports/0621-phase6-shared-single-attachment-frame-first-package.md`
- `docs/reports/0622-phase6-source-sample-corpus-scope-and-layout.md`
- `docs/reports/0624-phase6-representative-fixture-source-mapping-matrix.md`
- `docs/reports/0625-phase6-source-lowering-first-cut.md`
- `docs/reports/0626-phase6-source-sample-runner-first-cut.md`
- `docs/reports/0628-phase6-source-sample-verification-ladder-wiring.md`
- `docs/reports/0629-phase6-source-sample-authoring-bless-regression-policy.md`
- `docs/reports/0630-phase6-theorem-first-concrete-tool-pilot.md`
- `docs/reports/0636-phase6-deferred-authored-row-widen-sequencing-package.md`
- `docs/reports/0637-phase6-proof-notebook-bridge-sketch-reopen-ordering-package.md`
- `docs/reports/0638-phase6-mirror-sweep-follow-up-maintenance-and-document-consistency-audit.md`
- `docs/reports/0639-phase6-first-widened-authored-row-e1-actualization-package.md`
- `docs/reports/0640-phase6-second-widened-authored-row-e21-actualization-package.md`
- `docs/reports/0641-phase6-third-widened-row-e3-guard-comparison-package.md`
- `docs/reports/0642-phase6-plain-proof-notebook-bridge-sketch-actualization-package.md`
- `docs/reports/0652-phase6-second-source-sample-cluster-sequencing-package.md`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-semantics/examples/current_l2_emit_formal_hook.rs`
- `cargo test -p mir-ast`
- `cargo test -p mir-runtime`
- `python3 -m unittest scripts.tests.test_current_l2_static_gate_loop scripts.tests.test_current_l2_detached_loop`

## 次の mainline

`specs/examples/305...306` により、
compile-ready checkpoint close 後の next reopen sequencing は parser second tranche first に fixed 済みである。

`specs/examples/307...308` により、
parser second tranche first package も attached-slot / minimal predicate fragment まで actualize 済みである。

`specs/examples/309...310` により、
reserve formal tool binding inventory も fixed 済みであり、theorem-first concrete binding は first reserve、model-check side は second reserve に整理した。

`specs/examples/311...312` により、
parser-side follow-up package sequencing も fixed 済みであり、shared single attachment frame を next package、request clause suite / perform head / richer diagnostics / final grammar を deferred reopen に置いた。

`specs/examples/313...314` により、
shared single attachment frame first package も actualize 済みであり、`mir_ast::current_l2` は multiline extraction bridge を crate surface に持つ non-production carrier まで進んだ。

`specs/examples/315...316` により、
fixed-subset source-sample corpus scope / file layout も fixed 済みであり、repo-root `samples/current-l2/` flat `.txt` layer と fixture-stem-aligned naming を current first choice に置いた。

`specs/examples/317...318` により、
representative / fixture / source mapping matrix も fixed 済みであり、current initial cluster `e1` / `e2` / `e3` / `e4` / `e21` / `e23` を sample stem ごとに 3 層 row へ束ね、`e3` は `E3-variant`、`e23` は unresolved representative anchor に留める current cut を採った。

`specs/examples/319...320` により、
actual parser-to-`Program` lowering first cut も fixed 済みであり、`mir_runtime::current_l2::lower_current_l2_fixed_source_text` を helper-local lowerer に置き、initial authored sextet を semantic `Program` + optional stage 1 / stage 2 bridge evidence へ fail-closed に落とす current cut を採った。

`specs/examples/321...322` により、
syntax-backed sample runner first cut も fixed 済みであり、`mir_runtime::current_l2::run_current_l2_source_sample` と `resolve_current_l2_source_sample_path` を helper-local thin wrapper に置き、accepted sample set 内の explicit path / sample stem shorthand と explicit `FixtureHostPlan` input だけを current cut に残した。

`specs/examples/323...324` により、
verification ladder wiring initial cut も fixed 済みであり、first authored trio `e2` / `e4` / `e23` に reached-stage row を付け、`e2` は `static gate -> interpreter -> runtime_try_cut_cluster formal hook`、`e4` / `e23` は `static gate -> fixture_static_cluster formal hook` まで current reached と読んだ。

`specs/examples/333...334` により、
first widened authored row `e1` actualization も fixed 済みであり、`e1-place-atomic-cut` を helper-compatible single-line `ensure` source row、runner accepted set、regression helper current authored inventory、README ladder へ narrow actualize し、`static gate -> interpreter -> runtime_try_cut_cluster formal hook` reached row に昇格した。`e21` / `e3` は後続 widen slot として残した。

`specs/examples/335...336` により、
second widened authored row `e21` actualization も fixed 済みであり、`e21-try-atomic-cut-frontier` を helper-compatible source row、runner accepted set、regression helper current authored inventory、README ladder へ narrow actualize し、`static gate -> interpreter -> runtime_try_cut_cluster formal hook` reached row に昇格した。`E21` / `E22` contrast と `e3` guard line は still later に残す。

`specs/examples/325...326` により、
source-sample authoring / bless / regression policy も fixed 済みであり、`.docs/current-l2-source-sample-authoring-policy.md` と `python3 scripts/current_l2_source_sample_regression.py` を current repo-local policy anchor に置き、`bless` の current meaning を source / fixture / matrix / ladder / snapshot docs の reviewed sync と regression success に留めた。

`specs/examples/327...328` により、
theorem-first concrete tool pilot も fixed 済みであり、tool-neutral formal hook artifact を入力にする non-production `proof_notebook_review_unit` consumer を `crates/mir-semantics/examples/support/current_l2_proof_notebook_review_unit_support.rs` と example CLI に置き、bridge sketch / compare-bless metadata / proof-assistant adapter / model-check side は still later に残した。

`specs/examples/337...338` により、
third widened row `e3` theorem-side / formal-hook guard comparison も fixed 済みであり、`e3` は source row / runner / inventory / ladder actualization の前段で current theorem-side consumer `proof_notebook_review_unit` と current formal-hook top `runtime_try_cut_cluster` を保つ guard line に置いた。

`specs/examples/339...340` により、
plain proof-notebook bridge sketch actualization も fixed 済みであり、old theorem-line `specs/examples/140` の docs-only bridge sketch shape (`bridge_subject_ref + review_units + bridge_goal_text`) を current theorem-side first actualization として再利用し、compare-ready metadata / helper-emitter / concrete tool binding は still later に残す。

`specs/examples/341...342` により、
compare-ready bridge sketch second reopen も fixed 済みであり、old theorem-line `specs/examples/141` の compare basis-only bridge sketch shape (`bridge_subject_ref + review_units + bridge_goal_text + comparison_basis_refs`) を current theorem-side second actualization として再利用し、bless / review-session metadata / helper-emitter / concrete tool binding は still later に残す。

`specs/examples/343...344` により、
deferred `e3` actualization reopen timing も fixed 済みであり、compare-ready bridge fixed 後の次段で `e3` authored-row line を reopen してよい一方、current theorem-side consumer と current formal-hook top は保ち、new formal-hook family と concrete theorem / model-check tool pilot は still later に残す current cut を採った。

`specs/examples/345...346` により、
actual `e3` authored-row actualization も fixed 済みであり、`e3-option-admit-chain` を helper-compatible inline-`admit` source row、runner accepted set、regression helper current authored inventory、README ladder へ narrow actualize し、`static gate = reached(valid)`、`interpreter = reached(success)`、`formal hook = not reached (guarded)` の current cut に置いた。`admit-miss` は non-admissible skip に留め、new formal-hook family と theorem/model-check concrete binding は still later に残す。

`specs/examples/349...350` により、
second source-sample cluster sequencing も fixed 済みであり、post-sextet first cluster は `e21` / `e22` try-rollback locality contrast に置き、next actual source row は `e22-try-atomic-cut-place-mismatch` に置く current cut を採った。expiry / monotone degradation family、request-contract family、stable static malformed broader cluster、public surface inventory は still later に残す。

`specs/examples/351...352` により、
actual `e22` contrast-row source actualization も fixed 済みであり、`e22-try-atomic-cut-place-mismatch` を helper-compatible source row、runner accepted set、regression helper current authored inventory、README ladder、runtime detached formal-hook smoke に actualize しつつ、stage 2 parser bridge では nested `place` block を top-level `Other` statement head として畳む current helper-local floorに留めた。current authored source sample は `e1` / `e2` / `e3` / `e4` / `e19` / `e21` / `e22` / `e23` の octetに進んだ。

`specs/examples/353...354` により、
stable static malformed post-contrast sequencing も fixed 済みであり、second broader cluster は stable reason-code / fixture-static cluster、Macro 4 side の next reopen point は `e4` / `e19` edge-pair に置く current cut を採った。duplicate cluster、`TryFallback` / `AtomicCut` malformed-static family、broader runtime family は still later に残し、repo-level next line は public surface inventory に handoff した。

`specs/examples/355...356` により、
parser / checker / runtime public surface inventory も fixed 済みであり、already-public parser-free helper stack、crate-public but non-production compile-ready tranche、repo-local helper / example emitter surface の 3 bucket split を current reading に置いた。`pub` visibility だけで final public operational contract に昇格させず、repo-level next line は Mirrorea / shared-space docs-first re-entry に handoff した。

`specs/examples/357...358` により、
Mirrorea/shared-space docs-first re-entry bundle も fixed 済みであり、`mirrorea_fabric_boundary + shared_space_practical_boundary` を current repo-level boundary core に置き、Typed-Effect / Prism を adjacent track、shared-space final catalog と upper-layer app target を user-spec-required gate、identity/auth layering・admission/compile-time visibility・authority/resource ownership を next docs-first reopen refs に置く current cut を採った。control-plane separated carrier actualization、distributed fairness protocol、Mirrorea operational runtime realizationは still later に残す。

`specs/examples/359...360` により、
model-check/public-checker second reserve inventory も fixed 済みであり、`proof_notebook_review_unit` を current first concrete pilot に維持したまま、model-check side は tool-neutral formal hook / compare-ready bridge sketch 起点の second reserve、public-checker side は payload schema から verifier handoff surface までの docs-only chain を second reserve bucket に留める current cut を採った。concrete tool binding、actual public checker migration、actual emitted verifier handoff artifact は still later に残す。

`specs/examples/361...362` により、
stable-static edge-pair first reopen も fixed 済みであり、existing `e4` row と deferred `e19` row を source-backed static-stop pair として actualize し、`e19-malformed-target-mismatch` を helper-compatible source row、runner accepted set、regression helper current authored inventory、README ladder、fixture-static detached formal-hook smoke に narrow actualize した。duplicate cluster、`TryFallback` / `AtomicCut` malformed-static family、public operational surface actualization gate は still later に残す。

`specs/examples/363...364` により、
public operational surface actualization gate も fixed 済みであり、already-public parser-free helper stack を stable bucket として据え置いたまま、later public pressure の first docs-only candidate を `mir_runtime::current_l2::run_current_l2_source_sample` に narrow に置き、`run_current_l2_runtime_skeleton` / `lower_current_l2_fixed_source_text` を tranche-internal support、`resolve_current_l2_source_sample_path` と repo-local script/example surface を excluded bucket に留めた。final public parser / checker / runtime API、public operational CLI、shared-space identity / auth layering reopen、model-check concrete carrier first actualization gate は still later に残す。

`specs/examples/365...366` により、
shared-space identity / auth layering reopen も fixed 済みであり、membership identity core を `member_ref + principal_ref + member_incarnation + activation_state` に narrow に残しつつ、transport/service auth、room admission、display/projection identity を side carriers に押し分ける current cut を採った。admission/compile-time visibility、authority/resource ownership、concrete auth protocol binding は still later に残す。

`specs/examples/367...368` により、
model-check concrete carrier first actualization gate も fixed 済みであり、`proof_notebook_review_unit` current first pilot を巻き戻さず、`tool_neutral_formal_hook_only_input + compare_ready_docs_only_bridge_sketch` を entry にする narrow gate を current first choice に昇格した。public-checker docs-only chain は parallel reserve に留め、model-check concrete carrier actualization、actual public-checker migration、actual emitted verifier handoff artifact、concrete tool binding、bless/review-session metadata は still later に残す。

`specs/examples/369...370` により、
stable malformed broader follow-up inventory も fixed 済みであり、broader stable malformed next reopen order は missing-option family first (`e16/e17/e18`)、capability family second (`e13/e20`) に固定し、duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family は kept-later に残す current cut を採った。

`specs/examples/371...372` により、
public operational CLI / final public contract later gate も fixed 済みであり、`run_current_l2_source_sample` current gate を巻き戻さず、first later gate を final public parser / checker / runtime API、second later gate を public operational CLI に置き、repo layout / accepted-set / repo-local helper surface を current final contract の外へ残す current cut を採った。

current mainline は、
**repo-level current line は model-check concrete carrier first actualization**
である。

ここでは fixed entry criteria を崩さずに、

- deferred authored-row widen sequencing `e1 -> e21 -> e3`
- proof-notebook bridge-sketch reopen ordering (`plain docs-only bridge sketch` first, `compare-ready bridge sketch` second)
- theorem-side compare-ready bridge sketch second reopen（fixed 済み）
- deferred `e3` actualization reopen timing（fixed 済み）
- model-check concrete carrier first actualization
- source-sample emitted verification artifact wiring
- sample-facing theorem / model-check evidence summary and bless/review flow
- docs-first I/O / host-facing port boundary comparison（working label, later）
- stable malformed missing-option first reopen actualization comparison（reserve）
- stable malformed missing-option first reopen actualization comparison（reserve）
- final public parser / checker / runtime API first later gate actualization comparison（reserve）

を揃えるのが主眼になる。

current recommendation としては、

- fixed subset の executable sample を厚くすること自体はよい
- ただし higher-level async-control family や low-level memory-order-like surface を current executable core に混ぜるのは早い
- theorem/model-check side は `proof_notebook_review_unit` current first pilot を保ったまま、sample-visible milestone を 3 package に分けて進める方が自然である
- host-facing integration は `stdin/stdout` privileged 化より先に、docs-first の capability-scoped I/O / adapter boundary を切る方が自然である
- theorem/model-check concrete tool binding や LLVM-family backend / external codegen も、source corpus / lowering / runner / ladder のあとに narrow pilot として扱う
- widened authored-row order は `e1 -> e21 -> e3` に置き、`e1` / `e21` は actual authored row として閉じ、`e3` は admit-family / theorem-side guard を伴う third slot に残す
- theorem-side plain bridge sketch は authored-row line の後で actualize 済みであり、compare-ready bridge sketch は current second reopen line に置く
