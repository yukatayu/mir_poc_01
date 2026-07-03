# progress

最終更新: 2026-07-03 20:21 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins; cite legacy `specs/` /
`plan/` as `LAB:` evidence unless mirrored into canon.

## document role

This document is the repo-wide **current roadmap snapshot**. It is not normative
source.

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: `plan/`, legacy `specs/`
- Runnable dashboard: `samples_progress.md`
- Current task map: `tasks.md`
- Execution evidence: `docs/reports/`

Use workflow status and evidence class as the primary reading. Do not use
percentage as the main metric.

## project axis

```text
正しい理論に基づき、Mir source files を意味の正本として、
各 server / browser-like runtime / backend がそれ由来 artifact を実行し、
正しく hot-plug / 通信 / 検証 / 可視化できる仮想空間 system を作る。
```

This keeps Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform
separable.

## final ideal

The current long direction is source-first:

```text
.mir source files
  -> Surface Mir parser / AST
  -> Surface-to-Core elaboration
  -> Core Mir / typed IR
  -> checker / residual proof-model obligations
  -> interpreter and runtime session
  -> projection IR / deployment plan
  -> server / client / adapter artifacts
  -> provider boundary and devtools evidence
```

`package.mir.json` remains an alpha compatibility / package artifact. It is not
semantic source authority.

## current milestone position

- Current package: none promoted for the Surface line after `P-SURF-99`.
- Current canon position: `mirrorea_canon/plan/01-phases.md` places the project
  at T0/G0 rebaseline. LAB implementation and sample rows below are evidence,
  not canon implementation-state completion.
- Current migration note: root LAB entry points now point to `mirrorea_canon/`
  as canon, and `plan/70-lab-to-canon-reconciliation-ledger.md` now maps
  high-risk legacy LAB claim families to canon IDs, rejected historical claim
  patterns, or OPEN follow-up. This is LAB evidence and does not claim G0 exit.
- Current G1 planning note: `plan/71-g1-ordinary-assignment-target.md` now
  drafts the ordinary simple-assignment target/proof-boundary split in LAB memory. It
  targets THM-001 / BND-001 / SCN-01 / SCN-02 alignment only and does not claim
  G1 exit, theorem discharge, Lean proof completion, runtime dispatch, or public
  API freeze.
- Current SCN consequence note: `plan/72-g1-scn01-scn02-static-consequence-drilldown.md` maps
  SCN-01 and SCN-02 C-static expectations to canon targets, LAB ELAB evidence,
  LAB gaps, and runtime/proof boundaries. It does not claim C-static
  conformance.
- Current OBL-001 inventory note: `plan/73-g1-obl001-lean-statement-inventory.md`
  inventories the minimum Lean statement vocabulary, predicate split, SCN row
  coverage, adjacent-obligation separation, and overfit guards for THM-001. It
  adds no Lean statement file and does not move OBL status.
- Current planning note: consultation-derived strategy has been captured as
  non-normative repository memory in `plan/69`. It does not promote a new
  package, change `specs/`, or decide whether work should move to a separate
  theory/design repository.
- Current status after this snapshot: `P-SURF-99` closed the bounded Surface
  alpha audit over `P-SURF-01..08`. Surface alpha evidence has 46 helper rows
  and 47 `.mir` source files, with parser, indexed-state, elaboration,
  generated communication, role admission, source patch, source operational,
  and static devtools diagnostics floors revalidated.
- Next gap: the safe self-driven theory package is either an actual repo-local
  OBL-001 Lean statement draft, statement only, or a narrow SCN dependency-gap
  package if the statement would otherwise over-abstract SCN-01 same-field RHS
  and SCN-02 two-read RHS coverage. This remains before any later
  user-spec-required reopen for final runtime/transport, final source patch
  ABI, final viewer/telemetry ABI, or broader public grammar.
- Current truthful summary:
  Product Alpha-1 and the operational product suite remain bounded alpha floors.
  Full System V1 remains closed through bounded release-check / final audit.
  The closed Surface Mir alpha line has fixed that `.mir` source files own semantic
  authority, `package.mir.json` is alpha artifact, indexed state is owner-locus
  state keyed by participants or later constrained keyspaces, role claims are
  not authority, source patches go through parse/typecheck/elaborate/admit and
  activation cut, and generated communication / publish / observe must be
  visible in Core IR and devtools. `P-SURF-01` is parser/helper/sample evidence
  only. `P-SURF-02` is indexed-state semantic checker/sample evidence only; it
  does not claim runtime execution or role admission. `P-SURF-03` is
  elaboration evidence and `P-SURF-04` is generated communication elaboration
  evidence, `P-SURF-05` is role-admission evidence, `P-SURF-06` is source
  patch pipeline evidence, `P-SURF-07` is source operational evidence, and
  `P-SURF-08` is static devtools diagnostics evidence; `P-SURF-99` is final
  validation / claim-non-claim audit evidence. None
  claims runtime MessageEnvelope dispatch, production identity, hardware
  attestation, WAN admission, final source patch ABI, distributed durable
  migration, final operational runtime/transport, or final devtools viewer /
  telemetry ABI completion.

## milestone map

| Milestone | Meaning | Status | Evidence | Next gap |
|---|---|---|---|---|
| `P-A1` | Product alpha release candidate | `product-alpha-ready` | `mirrorea-alpha`, `package.mir.json`, product release check | keep as alpha compatibility floor |
| `P-OPS` | Operational product suite | `workflow-ready` | six bounded Product Alpha operational roots and helper checks | final catalog breadth remains user decision |
| `P-FSV1` | Full System V1 bounded source-first line | `workflow-ready release-check lane; audit closed` | `specs/33..38`, `plan/58..63`, `scripts/full_system_v1_release_check.py` | later public/broader reopen only |
| `P-SURF-00B` | Surface Mir brace/source-authority docs rebaseline | `closed` | `specs/39..43`, `plan/64..68`, snapshot docs and guides | implementation line opened |
| `P-SURF-01` | Surface brace parser | `evidence-closed parser lane` | `crates/mir-ast::surface_alpha`, `surface_mir_alpha_parse`, `samples/full-system-v1-surface/syntax/`, `scripts/surface_mir_samples.py` | keep non-final grammar; feed parser AST into later Surface packages |
| `P-SURF-02` | indexed state | `evidence-closed semantic checker lane` | `crates/mir-semantics::surface_indexed_state`, `surface_indexed_state_check`, `samples/full-system-v1-surface/indexed-state/`, `IDX-01..05` | integrate with Surface-to-Core elaboration and runtime carrier later |
| `P-SURF-03` | Surface-to-Core elaboration | `evidence-closed elaboration lane` | `crates/mir-semantics::surface_to_core_elaboration`, `surface_to_core_elaborate`, `samples/full-system-v1-surface/elaboration/`, `ELAB-01/02/04/05/06/07/08` | keep feeding later runtime/admission work |
| `P-SURF-04` | auto communication / publish / observe | `evidence-closed generated communication lane` | `crates/mir-semantics::surface_to_core_elaboration`, `surface_to_core_elaborate`, `samples/full-system-v1-surface/elaboration/`, `ELAB-03/09/10` plus widened `ELAB-01/05/08` | runtime dispatch and TypeMismatch discharge remain later |
| `P-SURF-05` | role admission | `evidence-closed admission/grant lane` | `crates/mir-semantics::surface_role_admission`, `surface_role_admission_check`, `samples/full-system-v1-surface/role-admission/`, `ROLE-01..04` | runtime identity/admission lifecycle remains later |
| `P-SURF-06` | source patch hot-plug | `evidence-closed source patch lane` | `crates/mir-runtime::surface_source_patch_hotplug`, `mirrorea-alpha check-source/parse-source/elaborate-source/patch-source/export-core-ir`, `samples/full-system-v1-surface/source-patch/`, `PATCH-01..04` | final hot-plug ABI and migration planner remain later |
| `P-SURF-07` | Surface source operational suite | `evidence-closed source operational lane` | `samples/full-system-v1-surface/{world-core,membership-chat,sugoroku-world,portal-worldlink,two-shard-hard-boundary,gradient-observation}/`, `E2E-SURF-01..12` | final operational runtime/transport remain later |
| `P-SURF-08` | Surface devtools / diagnostics | `evidence-closed static diagnostics lane` | `samples/full-system-v1-surface/devtools/`, `DEV-01..02` | final viewer/telemetry ABI and runtime devtools remain later |
| `P-SURF-99` | Surface Mir alpha audit | `audit-closed` | Surface release check, Product Alpha anchor, operational helper, minimal pattern anchor, docs validators | later public/broader reopen only |

## line snapshots

### Product Alpha line

Status: `product-alpha-ready`

Current evidence:

- `mirrorea-alpha` command family.
- versioned `package.mir.json`.
- local/Docker controlled runtime.
- observer-safe devtools/viewer.
- R0/R2 save evidence.
- native host launch bundle.

Next gap:

- keep as alpha floor while Surface Mir shifts source authority to `.mir` files.

### Operational Suite line

Status: `workflow-ready`

Current evidence:

- `samples/product-alpha1/operational/`
- `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation`
- `python3 scripts/operational_product_samples.py check-all --format json`

Next gap:

- no widening in `P-SURF-00B`; later Surface roots must not overwrite this
  product-alpha compatibility floor.

### Mir Language line

Status: `first-floor-evidence` for Full System V1, `parser-floor-evidence` plus
`indexed-state-checker-evidence` plus `elaboration-evidence` plus
`generated-communication-evidence` plus `role-admission-evidence` plus
`source-patch-hotplug-evidence` plus `source-operational-evidence` plus
`static-devtools-evidence`; Surface audit is closed through P-SURF-99.

Current evidence:

- Full System V1 parser/checker/runtime/projection/provider/release-check line
  remains closed through final audit.
- Legacy LAB Surface Mir evidence docs are `specs/39..43`; canon targets for
  source language, elaboration, conformance, and proof obligations live under
  `mirrorea_canon/spec/` and `mirrorea_canon/theory/`.
- Surface Mir repository memory is `plan/64..68`.
- `crates/mir-ast::surface_alpha` parses canonical `S { ... }`, role-instance
  blocks, `state`, `when`, `join`, record literals, and expected syntax
  rejections.
- `crates/mir-semantics::surface_indexed_state` checks S-owned
  Participant-indexed state declarations, owner/keyspace/value metadata,
  key-not-authority rejection, stale-key rejection, retained-savepoint
  compaction rejection, and nested-place ambient-authority rejection.
- `crates/mir-semantics::surface_to_core_elaboration` generates Core IR
  transitions, remote request rows, generated edges, source spans, and
  obligations for cross-locus indexed reads/writes.
- `crates/mir-semantics::surface_to_core_elaboration` also generates
  MessageEnvelope, visible publish/observe, and observer-safe redaction /
  retention rows for P-SURF-04.
- `crates/mir-semantics::surface_role_admission` records role claims,
  admission requests/verdicts, capability grants, witnesses, stale membership
  rejections, and optional hash metadata for P-SURF-05.
- `crates/mir-runtime::surface_source_patch_hotplug` and `mirrorea-alpha`
  source commands route patches through parse/typecheck/elaborate/compatibility
  / admission and produce HotPlugRequest, HotPlugVerdict, Core IR diff, and
  activation_cut report rows for P-SURF-06.
- `samples/full-system-v1-surface/{world-core,membership-chat,sugoroku-world,portal-worldlink,two-shard-hard-boundary,gradient-observation}/`
  and `operational-matrix.json` provide P-SURF-07 source-first operational
  evidence with `.mir` authority and 12 representative positive/negative rows.
- `samples/full-system-v1-surface/devtools/` provides P-SURF-08 static
  observer-safe diagnostics evidence for source, Core IR, indexed state map,
  communication, admission, patch lifecycle, and source spans.

Next gap:

- No current promoted Surface package. Later work should reopen only with an
  explicit package for final runtime/transport, final devtools viewer/telemetry
  ABI, final source patch ABI, public grammar/API, or broader distribution.

### PoseGraph line

Status: `first-floor-evidence`

Current evidence:

- Product Alpha PoseGraph helper evidence remains bounded.
- Full System V1 avatar-pose runtime/save/devtools evidence remains bounded.

Next gap:

- Surface Mir can later provide source-facing PoseGraph roots, but renderer /
  Unity / UE / WASM / native remain providers, not semantic owners.

### Projection/Backend line

Status: `first-floor-evidence`

Current evidence:

- Full System V1 projection IR / packet schema / FFI schema / local role split
  evidence remains bounded.

Next gap:

- Surface elaboration must preserve generated Core IR and boundary schemas
  before any backend widening.

### Engine/Provider line

Status: `first-floor-evidence`

Current evidence:

- bounded provider admission and renderer pose backend evidence exists under
  Full System V1.

Next gap:

- Surface Mir must preserve provider non-ownership and disabled/inventory
  defaults for native/WASM unless a later explicit package admits more.

### Surface Mir line

Status: `parser-floor-evidence` + `indexed-state-checker-evidence` +
`elaboration-evidence` + `generated-communication-evidence` +
`role-admission-evidence` + `source-patch-hotplug-evidence`

Current evidence:

- `specs/39-surface-mir-placement-elaboration.md`
- `specs/40-indexed-state-semantics.md`
- `specs/41-role-admission-and-capability-grant.md`
- `specs/42-source-patch-hotplug-semantics.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/64..68`
- `crates/mir-ast/src/surface_alpha.rs`
- `crates/mir-semantics/src/surface_indexed_state.rs`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/src/surface_role_admission.rs`
- `crates/mir-runtime/src/surface_source_patch_hotplug.rs`
- `samples/full-system-v1-surface/syntax/matrix.json`
- `samples/full-system-v1-surface/indexed-state/matrix.json`
- `samples/full-system-v1-surface/elaboration/matrix.json`
- `samples/full-system-v1-surface/role-admission/matrix.json`
- `samples/full-system-v1-surface/source-patch/matrix.json`
- `samples/full-system-v1-surface/devtools/matrix.json`
- `scripts/surface_mir_samples.py`

Next gap:

- no current promoted Surface package; later reopen requires an explicit
  runtime/transport/public-ABI/devtools-viewer package.

## validation floor

Required for the current Surface package close:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
cargo test -p mir-ast --test surface_mir_parser -- --nocapture
cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
cargo test -p mir-semantics --test role_admission_capability_grant -- --nocapture
cargo test -p mir-runtime --test source_patch_hotplug -- --nocapture
cargo test -p mirrorea-cli --test surface_mir_cli -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/surface_mir_authoring_check.py check-all --format json
python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release
```

Compatibility anchors when environment permits:

```bash
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/minimal_alpha1_patterns.py check-all --format json
```

## non-claims

- No final public grammar completion.
- No runtime MessageEnvelope dispatch, local queue delivery, or final transport
  implementation yet.
- No TypeMismatch typechecker discharge for generated communication yet.
- No production identity provider, hardware attestation, or WAN admission.
- No indexed-state runtime carrier or distributed compaction protocol yet.
- No final source patch hot-plug ABI, distributed durable migration planner,
  production patch registry/signing workflow, or arbitrary native/WASM
  execution through patches.
- No final Surface operational runtime / transport or shared-space catalog
  completion yet; P-SURF-07 is source operational evidence only.
- No Rust-level language completion.
- No LLVM/native codegen completion.
- No final server/client split compiler completion.
- No arbitrary native/WASM/Unity/UE provider execution.
- No production WAN/federation.
- No distributed durable save-load R3/R4.
- No final public ABI / SDK.
- No final shared-space catalog breadth decision.

## user decision items vs research-discovery items

User decision items:

- final public grammar and compatibility window.
- final ABI / SDK / engine adapter public surface.
- broader distribution beyond developer-built binary plus generated host launch
  bundle.
- final shared-space catalog breadth.
- production WAN/federation and R3/R4 durable distributed save-load.

Research-discovery items:

- indexed-state owner/keyspace/access/stale runtime carrier.
- Surface-to-Core obligation carrier shape.
- role admission capability grant carrier.
- indexed-state tombstone / compaction runtime carrier.
- role admission witness metadata shape.
- source patch compatibility diff and activation-cut carrier.

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | Surface alpha audit closed; no current promoted Surface package | light | maintenance only |
| `Macro 1` | semantic kernel / invariant / boundary stabilization | Surface authority / placement / indexed state / admission / patch boundaries fixed | medium | 着手可能 |
| `Macro 2` | parser-free validation substrate | existing alpha/product helpers remain compatibility anchors | medium | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | parser, indexed-state checker, elaboration, generated communication, role admission, source patch, source operational, static devtools diagnostics, and final audit floors closed | heavy | maintenance only |
| `Macro 4` | executable sample expansion | Surface operational and devtools roots exist as alpha source evidence; final audit closed | heavy | maintenance only |
| `Macro 5` | theorem / model-check / verifier bridge | Surface elaboration soundness is target obligation, not discharged | medium | 着手可能 |
| `Macro 6` | distributed fabric / runtime evolution | local/Docker alpha remains floor | heavy | 後段依存 |
| `Macro 7` | toolchain / backend / developer surface | Surface parser / indexed-state / elaboration helper commands exist; product alpha CLI remains compatibility floor | heavy | 着手可能 |
| `Macro 8` | domain / application realization | Surface WorldCore/MembershipChat/Sugoroku/Portal/TwoShard/Gradient roots are alpha source evidence; final runtime/catalog remain later | heavy | 後段依存 |

## feature maturity rows

| Feature | Status | Reading | Actionability |
|---|---|---|---|
| Surface Mir brace syntax | `parser-floor-evidence` | canonical `S { ... }` parses; `S[ ... ]` rejects with `bracket_place_scope_not_supported`; no sugar | 着手可能 |
| textual Mir source | `first-floor-evidence` | Full System V1 parser exists; Surface parser floor now exists separately | 着手可能 |
| typed IR / checker | `first-floor-evidence` | existing Full System V1 checker remains floor | 着手可能 |
| Surface-to-Core elaboration | `elaboration-evidence` | cross-locus indexed reads/writes lower to explicit Core IR remote requests, generated edges, source spans, and obligations | 着手可能 |
| indexed state | `semantic-checker-evidence` | S-owned Participant-indexed map accepted; key-as-authority, stale key, retained-savepoint compaction, and nested-place ambient-authority negatives reject | 着手可能 |
| auto communication / publish / observe | `generated-communication-evidence` | generated MessageEnvelope / visible publish / observe rows and `VisibilityDenied` failure containment exist in Core IR; runtime dispatch remains later | 着手可能 |
| role admission / capability grant | `role-admission-evidence` | role claim, join admission request, capability grant-backed accepted write, witness, stale rejection with a post-stale write fence, and hash metadata rows exist; runtime identity/admission lifecycle remains later | 着手可能 |
| source patch hot-plug | `source-patch-hotplug-evidence` | parse/typecheck/elaborate/compatibility/admission report, HotPlugRequest/HotPlugVerdict, Core IR diff, activation_cut, no-direct-eval and rejection-without-mutation evidence exist; final ABI/migration planner later | 着手可能 |
| Surface source operational suite | `source-operational-evidence` | `E2E-SURF-01..12` cover WorldCore, MembershipChat, Sugoroku, PortalWorldlink, TwoShardHardBoundary, and GradientObservation positive/negative source rows through required alpha checks | 着手可能 |
| Surface devtools diagnostics | `static-devtools-evidence` | `DEV-01..02` expose required Surface source/Core IR/semantic-checker-backed indexed-state/communication/admission/redacted patch/source-span panels without final viewer/telemetry ABI claims | 着手可能 |
| Product Alpha | `product-alpha-ready` | bounded alpha workflow, not final product | maintenance only |
| operational suite | `workflow-ready` | bounded local/Docker suite remains compatibility anchor | maintenance only |
| projection/backend | `first-floor-evidence` | bounded projection/provider evidence remains lower floor | 着手可能 |

## recent log

- 2026-07-03 17:38 JST
  `mirrorea_canon/` を canon-first source hierarchy として読み込み、local
  checks、sub-agent review、Oracle consult で整合性を確認した。canon 内部は
  T0/G0 の大局正本として採用可能だが、既存 LAB 文書が旧 `specs/` 正本表現を
  持っていたため、root `CANON.md`、entry banner、validator guardrail、
  clean-near-end / highlighter の旧 `world` 語彙注記を追加した。次は
  LAB-to-canon reconciliation ledger と G1 ordinary assignment target。
- 2026-07-03 18:38 JST
  `plan/70-lab-to-canon-reconciliation-ledger.md` を追加し、legacy LAB claim
  family を canon anchor / rejected historical pattern / OPEN follow-up に
  照合した。G0 exit は主張せず、次の安全な自走 package を G1 ordinary
  assignment target draft として整理した。
- 2026-07-03 19:55 JST
  `plan/71-g1-ordinary-assignment-target.md` を追加し、G1 ordinary simple assignment
  の target/proof-boundary split を THM-001、BND-001、SCN-01、SCN-02、
  OBL-001/020/021 に絞って整理した。G1 exit、theorem discharge、
  runtime dispatch、final grammar/API freeze は主張していない。次は
  SCN-01/02 static consequence drilldown または OBL-001 Lean statement inventory。
- 2026-07-03 20:05 JST
  `plan/72-g1-scn01-scn02-static-consequence-drilldown.md` を追加し、SCN-01/SCN-02 の
  C-static 期待を canon target、LAB ELAB support、LAB gap、runtime/proof
  boundary に分解した。C-static conformance、G1 exit、theorem discharge は
  主張していない。次は OBL-001 Lean statement inventory。
- 2026-07-03 20:21 JST
  `plan/73-g1-obl001-lean-statement-inventory.md` を追加し、THM-001 / OBL-001 の
  repo-local Lean statement に必要な最小 datatypes / predicates / theorem shape /
  SCN row coverage / overfit guard を整理した。Lean statement file、OBL status
  movement、G1 exit、theorem discharge は主張していない。次は actual OBL-001
  statement draft または SCN dependency-gap package。
- 2026-07-02 18:03 JST
  Oracle 運用メモを更新し、理論的に難しい判断、全体像、roadmap、複雑な
  design review では積極的に Oracle consult を投げる方針を明記した。長時間
  実行は非同期 reviewer として扱い、必要なら sub-agent に起動 / 監視を任せつつ、
  採否と repo への mirror は main agent が source hierarchy に照らして判断する。
- 2026-07-02 17:52 JST
  ChatGPT 5.5 Pro Extended Oracle browser consults の repo-local 運用メモを
  `.docs/oracle-chatgpt-pro-operations.md` として追加し、`AGENTS.md`、
  `Documentation.md`、`plan/00-index.md` から導線を張った。Oracle は advisory
  review input であり、規範正本ではない。実行は分単位で待ち、遅い場合も
  重複起動せず `oracle status` / `oracle session` で確認する運用にした。
- 2026-06-25 18:10 JST
  相談会話と repo 基礎文書を照合し、post-`P-SURF-99` の source-first
  management synthesis を `plan/69` として非規範の repository memory に保存した。
  新しい promoted package や `specs/` 判断は作らず、ordinary assignment
  elaboration、Surface/Core/Trace/Projection separation、World/Event 非 core
  primitive、ledger 管理案を今後の検討候補として切り分けた。
- 2026-06-25 17:49 JST
  `current_l2_lean_sample_sync.py` の clean-near-end `source_path` 出力を repo-relative に変更し、`samples/lean/clean-near-end/*/*.bundle.json` と各 README の source reference を `samples/...` 相対パスへ再生成した。Lean sync は idempotent に通り、active Lean files、Rust actual Lean probe、`make check`、`git diff --check`、absolute path scan を確認した。
- 2026-06-25 17:38 JST
  `elan` 経由で repo の `lean-toolchain` に合わせた Lean 4.29.1 / Lake 5.0.0 を導入し、active `samples/lean/foundations` / `samples/lean/clean-near-end` と historical `samples/lean/old` を含む 32 Lean files、Lean sync helper、Lean theorem-stub pipeline、Python Lean unit tests、Rust Lean actual probe を確認した。新規 disk 使用量は filesystem 差分で 2,851,463,168 bytes、約 2.66 GiB、`~/.elan` 実体で 2,819,461,716 bytes、約 2.63 GiB。
- 2026-06-25 17:12 JST
  broad build/execution audit と clippy hardening を実施し、`cargo fmt --check`、`cargo check --workspace --all-targets`、`cargo build --workspace --all-targets`、`cargo test --workspace --all-targets --no-fail-fast`、`cargo clippy --workspace --all-targets -- -D warnings`、`python3 -m unittest discover -s scripts/tests`、主要 sample / release check / `mirrorea-alpha` demo 起動確認を通した。Lean stub artifact pipeline は `/tmp` 出力で通ったが、`lean` / `lake` / `elan` は PATH 不在のため compiler mechanization check は未実行。
- 2026-05-24 20:42 JST
  `P-SURF-99` で final Surface alpha audit を close し、Surface release check、Surface helper / authoring check、Product Alpha release anchor、operational product helper、minimal alpha-1 pattern verifier、docs validators、`cargo fmt --check`、`git diff --check` を再実行した。Surface line は 46 helper rows / 47 `.mir` source files の bounded alpha evidence として閉じ、final runtime/transport、final source patch ABI、final devtools viewer/telemetry ABI、public grammar/API は later gate のまま。
- 2026-05-24 20:13 JST
  `P-SURF-08` で static devtools diagnostics evidence floor を actualize し、`samples/full-system-v1-surface/devtools/`、`DEV-01..02`、`scripts/surface_mir_samples.py check-all` を同期した。panel は Surface source / generated Core IR / semantic-checker-backed indexed-state map / generated communication / role admission / redacted patch lifecycle / source spans を持つが、final viewer / telemetry ABI や runtime devtools dispatch ではない。then-promoted package は `P-SURF-99 final surface alpha audit`。
- 2026-05-24 19:58 JST
  `P-SURF-07` で source operational suite evidence floor を actualize し、`samples/full-system-v1-surface/world-core/`、`membership-chat/`、`sugoroku-world/`、`portal-worldlink/`、`two-shard-hard-boundary/`、`gradient-observation/`、`operational-matrix.json`、`E2E-SURF-01..12`、`scripts/surface_mir_samples.py check-all` を同期した。review 後に MembershipChat row も role admission と elaboration / generated communication を同時に通し、operational projection の `source_authority` / `final_public_api_frozen` は下位 semantic payload 由来にした。各 root は `.mir` source authority の positive / negative rows を持つが、final operational runtime / transport ではない。then-promoted package は `P-SURF-08 devtools and diagnostics`。
- 2026-05-24 19:27 JST
  `P-SURF-06` で source patch hot-plug evidence floor を actualize し、`surface_source_patch_hotplug`、`mirrorea-alpha check-source/parse-source/elaborate-source/patch-source/export-core-ir`、`PATCH-01..04`、`cargo test -p mir-runtime --test source_patch_hotplug -- --nocapture`、`cargo test -p mirrorea-cli --test surface_mir_cli -- --nocapture`、`scripts/surface_mir_samples.py check-all` を同期した。source patch は direct eval ではなく、accepted `patch-source` は HotPlugRequest / HotPlugVerdict / activation_cut を出し、`check-source` / `elaborate-source` は inspection-only、rejected patch は mutation しない。then-promoted package は `P-SURF-07 source operational suite`。
- 2026-05-24 18:21 JST
  `P-SURF-05` で role admission / capability grant evidence floor を actualize し、`surface_role_admission_check`、`ROLE-01..04`、`cargo test -p mir-semantics --test role_admission_capability_grant -- --nocapture`、`scripts/surface_mir_samples.py check-all` を同期した。role claim は authority ではなく、authority は admission grant から来る。then-promoted package は `P-SURF-06 source patch hot-plug`。
- 2026-05-24 17:48 JST
  `P-SURF-04` で generated communication evidence floor を actualize し、Core IR に `MessageEnvelope` / publish / observe / observer-safe redaction-retention rows を追加し、`ELAB-03/09/10` と widened `ELAB-01/05/08` で private/non-visible field rejection、visible write publish/observe、`VisibilityDenied` failure-row containment、`cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`、`scripts/surface_mir_samples.py check-all` を同期した。then-promoted package は `P-SURF-05 role admission capability grant`。
- 2026-05-24 16:48 JST
  `P-SURF-03` で Surface-to-Core elaboration evidence floor を actualize し、`ELAB-01/02/04/05/06/07/08` の cross-locus read/write remote request、generated edge、source span、obligation、read/write underdeclared generated failure-row rejection、nested read placement、unsupported-statement rejection、`cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`、`scripts/surface_mir_samples.py check-all` を同期した。then-promoted package は `P-SURF-04 auto communication publish/observe`。
- 2026-05-24 16:14 JST
  `P-SURF-02` で Surface Mir indexed-state semantic checker floor を actualize し、`IDX-01..05` の owner/keyspace/value metadata、key-not-authority rejection、stale-key rejection、retained-savepoint compaction rejection、nested-place ambient-authority rejection、`cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture`、`scripts/surface_mir_samples.py check-all` を同期した。then-promoted package は `P-SURF-03 Surface-to-Core elaboration`。
- 2026-05-24 15:38 JST
  `P-SURF-01` で Surface Mir alpha parser / sample helper floor を actualize し、`SURF-01..09` の positive/negative rows、`surface_mir_alpha_parse` example、authoring check、release-check check-all を同期した。then-promoted package は `P-SURF-02 indexed-state semantics`。
- 2026-05-24 14:00 JST
  `P-SURF-00B` で Surface Mir place-scope syntax を canonical `S { ... }` に rebaseline し、`S[ ... ]` を sugar としても採用しない方針、`.mir` source authority、indexed state owner/keyspace split、role admission/capability grant split、source patch hot-plug pipeline、Surface package sequenceを docs/spec/plan snapshot に固定した。検証結果と commit/push status は report を正本にする。
