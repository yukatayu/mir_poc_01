# plan/00 — plan インデックス

## 目的

`plan/` は、この repo の **long-lived repository memory** である。

- 規範判断の正本は `mirrorea_canon/`
- この `plan/` tree と legacy `specs/` は LAB evidence / historical repository memory
- snapshot は `Documentation.md` / `progress.md` / `tasks.md`
- 詳細経緯は `docs/reports/`
- `plan/` は、その間をつなぐ長期参照を保つ
- agent / operational policy は `.docs/` に置く。たとえば
  `.docs/oracle-chatgpt-pro-operations.md` は、ChatGPT 5.5 Pro Extended
  Oracle browser consults の advisory review 運用を記録する。

## 先に読む順序

1. canon 正本として `mirrorea_canon/README.md`、`mirrorea_canon/MAP.md`、
   task-specific canon files を先に読む
2. LAB memory として `plan/00-index.md`
3. `plan/01-status-at-a-glance.md`
4. `plan/02-system-overview-and-positioning.md`
5. `plan/03-decision-strengths-and-boundaries.md`
6. current-L2 実装寄りなら `plan/04 ... plan/09`
7. 全体計画なら `plan/10-roadmap-overall.md`
8. 直近の research split は `plan/11-roadmap-near-term.md`
9. projection / placement と hot-plug / transport / backend guardrail / avatar slice / typed external executable widening / viewer prototype / public-freeze mixed gate / post-`P18` user-spec hold option inventory / `VerificationLayer` widening threshold / `AttachPoint` minimal contract / `FAIRY-05` carrier bundling / hot-plug real migration rollback boundary / runtime-crate hot-plug engine ownership cut / runtime-crate hot-plug carrier admission cut / post-`P20` historical bridge / post-`P21` rollback-durable-migration boundary family / post-`P21` distributed-activation-ordering boundary family / post-`P21` final-public-hotplug-ABI boundary family / alpha-local type-system freeze / layer compatibility freeze / cut-save-load checkpoint / runtime package avatar policy / Mirrorea Spaces alpha E2E / practical alpha-1 / operational α-0.5 / α-0.8 / α-0.9 / host-I/O and session runtime / product alpha-1 public boundary / operational product sample suite / portal-shard future boundary / Mir computational core / Transform-PoseGraph / projection-backend boundary / engine adapter boundary / autonomous computational-core execution / Full System V1 source-first roadmap / Surface Mir alpha source-authority roadmap の repository-memory roadmap は `plan/20`、`plan/21`、`plan/22`、`plan/23`、`plan/24`、`plan/25`、`plan/26`、`plan/27`、`plan/28`、`plan/29`、`plan/30`、`plan/31`、`plan/32`、`plan/33`、`plan/34`、`plan/35`、`plan/36`、`plan/37`、`plan/38`、`plan/39`、`plan/40`、`plan/41`、`plan/42`、`plan/43`、`plan/44`、`plan/45`、`plan/46`、`plan/47`、`plan/48`、`plan/49`、`plan/50`、`plan/51`、`plan/52`、`plan/53`、`plan/54`、`plan/55`、`plan/56`、`plan/57`、`plan/58`、`plan/59`、`plan/60`、`plan/61`、`plan/62`、`plan/63`、`plan/64`、`plan/65`、`plan/66`、`plan/67`、`plan/68`
10. post-`P-SURF-99` の相談内容 synthesis / source-first management recut memory は `plan/69`
11. LAB-to-canon claim-family reconciliation は `plan/70`
12. G1 ordinary assignment target draft は `plan/71`
13. G1 SCN-01/SCN-02 static consequence drilldown は `plan/72`
14. G1 OBL-001 Lean statement inventory は `plan/73`
15. G1 OBL-001 repo-local Lean statement draft は `plan/74`
16. G1 SCN RHS dependency-gap LAB evidence は `plan/75`
17. G1 OBL-020/021 dependency inventory は `plan/76`
18. G1 OBL-021 repo-local Lean statement draft は `plan/77`
19. G1 OBL-020 repo-local Lean statement draft は `plan/78`
20. G1 E-ROW diagnostic alignment は `plan/79`
21. G1 diagnostic carrier inventory は `plan/80`
22. G1 OBL-024 statement-shape inventory は `plan/81`
23. G1 OBL-025 statement-shape inventory は `plan/82`
24. G1 E-ROW repair payload inventory は `plan/83`
25. G1 E-ROW carrier-only diagnostic detail prototype は `plan/84`
26. G1 E-ROW carrier precondition hardening は `plan/85`
27. G1 E-ROW-002 visibility repair carrier prototype は `plan/86`
28. G1 OBL-025 repo-local Lean statement draft は `plan/87`
29. G1 E-ROW repair shape inventory は `plan/88`
30. G1 E-ROW-001 non-visibility singleton fixture は `plan/89`
31. G1 E-ROW-001 base singleton fixture closure は `plan/92`
32. G1 E-ROW-001 singleton repair assumption gate は `plan/93`
33. G1 E-ROW-001 singleton repair prototype は `plan/94`
34. G1 E-ROW mixed / multi repair decomposition inventory は `plan/95`
35. G1 E-ROW set-insertion / bundle payload inventory は `plan/96`
36. G1 ELAB-07 set-insertion gate review は `plan/97`
37. G1 ELAB-04 mixed visibility branch inventory は `plan/98`
38. G1 ELAB-07 set-insertion executable preflight は `plan/99`
39. G1 ELAB-07 set-insertion assumption acceptance は `plan/100`
40. G1 ELAB-07 set-insertion payload-model design は `plan/101`
41. G1 ELAB-07 set-insertion executable payload prototype は `plan/102`
42. G1 ELAB-07 set-insertion negative-guard hardening は `plan/103`
43. G1 ELAB-07 set-insertion row-identity guard hardening は `plan/104`
44. G1 ELAB-07 set-insertion exact-locus guard hardening は `plan/105`
45. G1 ELAB-07 child / bundle / partial exclusion fixtures は `plan/106`
46. G1 ELAB-04 mixed visibility payload-model preflight は `plan/107`
47. G1 OBL-025 branch-local non-coverage refinement は `plan/108`
48. G1 OBL-024 repo-local Lean statement draft は `plan/109`
49. G1 OBL-024 executable diagnostic-soundness projection carrier は `plan/110`
50. G1 OBL-024 projection Rust fixture guard hardening は `plan/111`
51. G1 OBL-024 replay vocabulary preflight は `plan/112`
52. G1 OBL-024 Lean replay vocabulary refinement は `plan/113`
53. ChatGPT Pro Oracle browser consults の repo-local 運用は `.docs/oracle-chatgpt-pro-operations.md`
54. リスクと heavy line は `plan/12`, `plan/13`, `plan/18`

## current repo の短い要約

- current 主眼は Mir current-L2
- active base corpus は `samples/current-l2/`、active LAB clean executable suite は `samples/clean-near-end/` に置く
- pre-clean-near-end の authored / corrected prototype set `p01 ... p16` は historical comparison memory であり、current active runner floor ではない
- Problem 1 は
  typed / IFC、theorem-first emitted artifact loop、model-check second-line reserve summary、
  Lean foundation / generated stub acceptance まで repo-local に actualize 済み
- Problem 2 は
  order / handoff / authoritative-room representative pair、reserve route、negative static-stop pair、
  witness / delegated RNG reserve summary まで repo-local に actualize 済み
- ただし final public theorem/model-check contract、final public verifier contract、
  low-level `memory_order` exact surface、final witness/provider public contract、
  packaging / FFI / engine adapter は still later

## repository snapshot reading

- **repo-local near-end**:
  `samples/current-l2/` の base corpus と、clean near-end active suite へ forward する `current_l2_guided_samples.py` の `list / smoke-all / closeout` compatibility front door / clean near-end closeout を分けて辿れば、二大問題の current cut を確認できる。pre-clean-near-end の representative bundle / reserve summary index は historical memory として読む
- **alpha-local Mirrorea Spaces scaffold**:
  `samples/alpha/` は `specs/13..17` / `plan/39..43` と結びつく phase-indexed scaffold であり、theory-freeze から checker/runtime skeleton へ移る current planning lane を読む。current cut では expected-verdict sidecar 付き skeleton に留め、active runnable root としては扱わない
- **practical alpha-1 line**:
  `specs/18-practical-alpha1-scope.md` と `plan/44-practical-alpha1-roadmap.md` は、current-scope evidence closeout と分離された practical first-floor toolchain line を置く
- **operational alpha line**:
  `specs/19..24` と `plan/45..49` は、α-0.5 / α-0.8 / α-0.9 の operational readiness 条件、proof obligations、session runtime / host-I/O の reopen order を置く
- **product alpha-1 line**:
  `specs/25-product-alpha1-public-boundary.md` と `plan/50-product-alpha1-public-boundary-roadmap.md` は、bounded workflow の次に来る product/public-ready alpha-1 の public-ish CLI、package schema、same-session product demo、quiescent save、viewer、native launch bundle、release validation の reopen order を置く
- **operational product sample line**:
  `specs/26-operational-product-sample-suite.md` と `plan/51-operational-product-sample-roadmap.md` は、`WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation` suite、shared attach packages、deployment/projection inventory、release-check helper、template-only authoring starter catalog、retained portal/shard blueprints、portal/shard starter boundary、next widening order を置く
- **backend guardrail and operational backend inventory**:
  `plan/23-compiler-backend-llvm-guardrail-roadmap.md` は small VPS / detachable workdir 前提の backend guardrail を置き、`P-OPS-08` current line では `native host launch bundle` / WASM / LLVM comparison inventory を docs-first boundary としてだけ参照する
- **portal / spatial world future line**:
  `specs/27-spatial-portal-and-shard-extension-boundary.md` と `plan/52-portal-spatial-world-roadmap.md` は、portal/world-link、two-shard hard boundary、observer-only gradient observation profile、replication profile optionality の future boundary を置く
- **Mir computational core rebaseline**:
  `specs/28-mir-computational-core.md` と `plan/53-mir-computational-core-roadmap.md` は、current typed external `AddOne` を host-boundary evidence に限定し、Mir-owned computation の first floor、pure/effect split、explicit failure row、`P-COMP-01` scaffold、`P-COMP-02` first direct executable row、`P-COMP-03` helper-executable first-floor widening、`P-COMP-04` direct host read/write boundary closeout を置く
- **Transform / PoseGraph line**:
  `specs/29-transform-posegraph-semantics.md` と `plan/54-transform-posegraph-roadmap.md` は、avatar/object transform、pose snapshot、anchor graph、fallback admissibility、no-split-frame の docs/spec boundaryと `P-POSE-01` scaffold、`P-POSE-02` bounded helper evidence を置く
- **projection/backend boundary line**:
  `specs/30-projection-and-backend-boundary.md` と `plan/55-projection-backend-roadmap.md` は、target manifest / packet schema / FFI schema inventory と `P-PROJ-01` planned-only scaffold を置き、server/client binary split や backend codegen completion を claim しない
- **engine/WASM/FFI adapter line**:
  `specs/31-engine-wasm-ffi-adapter-boundary.md` と `plan/56-engine-adapter-roadmap.md` は、Unity / Unreal / renderer / WASM / native library を typed provider として扱い、world semantics を Mir / Mirrorea に残す boundary と `P-ENG-01` planned-only scaffold を置く
- **autonomous computational-core execution line**:
  `specs/32-autonomous-execution-and-completion-contract.md` と `plan/57-autonomous-computational-core-master-plan.md` は、front-half `P-COMP-01 -> P-POSE-01 -> P-PROJ-01 -> P-ENG-01` closeout と、implementation half `P-COMP-02 -> P-COMP-03 -> P-COMP-04 -> P-POSE-02`、user-spec-required gate isolation、validation / report / commit cadence を置く
- **Full System V1 source-first roadmap**:
  `specs/33-full-system-v1-scope.md` と `plan/58-full-system-v1-roadmap.md` は、Product Alpha-1 を final product に昇格せず、Mir source files を semantic source of truth に戻す roadmap を置く。`plan/59..63` は textual Mir、computational runtime、PoseGraph runtime、projection/backend、engine/provider の package order と stop line を整理する
- **Surface Mir alpha source-authority roadmap**:
  `specs/39-surface-mir-placement-elaboration.md` と `plan/64-surface-mir-placement-roadmap.md` は、canonical Surface Mir place-scope syntax を `S { ... }` に固定し、`S[ ... ]` を sugar としても採用しない rebaseline、P-SURF-01 parser floor、P-SURF-03 Surface-to-Core elaboration evidence floor、P-SURF-04 generated communication evidence floor を置く。`specs/40..43` と `plan/65..68` は indexed state、role admission / capability grant、source patch hot-plug、Surface Full System V1 package order と stop line を整理し、P-SURF-02 で indexed-state semantic checker floor、P-SURF-05 で role admission / capability grant report-level evidence floor、P-SURF-06 で source patch hot-plug evidence floor、P-SURF-07 で source operational evidence floor、P-SURF-08 で static devtools diagnostics evidence floor、P-SURF-99 で final validation / claim-non-claim audit を actualize 済みである。`package.mir.json` は alpha artifact に留め、`.mir` files を semantic source authority として扱う
- **not final public**:
  concrete tool brand、final shared contract、public API、exact low-level source surfaceはまだ採っていない

## この index からどこへ行くか

- concise status:
  `plan/01-status-at-a-glance.md`
- current-L2 実装面:
  `plan/07-parser-free-poc-stack.md`
  `plan/08-representative-programs-and-fixtures.md`
  `plan/09-helper-stack-and-responsibility-map.md`
- roadmap:
  `plan/10-roadmap-overall.md`
  `plan/11-roadmap-near-term.md`
- projection / placement / hot-plug / transport:
  `plan/20-projection-and-placement-roadmap.md`
  `plan/21-hotplug-attachpoint-roadmap.md`
  `plan/22-network-transport-roadmap.md`
  `plan/23-compiler-backend-llvm-guardrail-roadmap.md`
  `plan/24-avatar-follow-representative-slice-roadmap.md`
  `plan/25-typed-external-boundary-executable-roadmap.md`
  `plan/26-visual-debugger-viewer-roadmap.md`
  `plan/27-public-api-parser-gate-roadmap.md`
  `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
  `plan/29-verification-layer-widening-threshold.md`
  `plan/30-attachpoint-detach-minimal-contract.md`
  `plan/31-fairy05-visibility-return-carrier-bundling.md`
  `plan/32-hotplug-real-migration-rollback-boundary.md`
  `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
  `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
  `plan/35-post-p20-hotplug-next-package-inventory.md`
  `plan/36-post-p21-rollback-durable-migration-family.md`
  `plan/37-post-p21-distributed-activation-ordering-family.md`
  `plan/38-post-p21-final-public-hotplug-abi-family.md`
  `plan/39-type-system-freeze-roadmap.md`
  `plan/40-layer-compatibility-freeze-roadmap.md`
  `plan/41-save-load-checkpoint-roadmap.md`
  `plan/42-runtime-package-avatar-roadmap.md`
  `plan/43-alpha-e2e-roadmap.md`
  `plan/44-practical-alpha1-roadmap.md`
  `plan/45-operational-alpha05-roadmap.md`
  `plan/46-operational-alpha08-roadmap.md`
  `plan/47-operational-alpha09-devtools-roadmap.md`
  `plan/48-theory-freeze-proof-obligations.md`
  `plan/49-host-io-and-session-runtime-roadmap.md`
  `plan/50-product-alpha1-public-boundary-roadmap.md`
  `plan/51-operational-product-sample-roadmap.md`
  `plan/52-portal-spatial-world-roadmap.md`
  `plan/53-mir-computational-core-roadmap.md`
  `plan/54-transform-posegraph-roadmap.md`
  `plan/55-projection-backend-roadmap.md`
  `plan/56-engine-adapter-roadmap.md`
  `plan/57-autonomous-computational-core-master-plan.md`
  `plan/58-full-system-v1-roadmap.md`
  `plan/59-textual-mir-roadmap.md`
  `plan/60-computational-runtime-roadmap.md`
  `plan/61-posegraph-runtime-roadmap.md`
  `plan/62-projection-backend-roadmap.md`
  `plan/63-engine-provider-roadmap.md`
  `plan/64-surface-mir-placement-roadmap.md`
  `plan/65-indexed-state-roadmap.md`
  `plan/66-role-admission-roadmap.md`
  `plan/67-source-patch-hotplug-roadmap.md`
  `plan/68-surface-full-system-v1-roadmap.md`
- consultation-derived management synthesis:
  `plan/69-consultation-synthesis-and-management-roadmap.md`
- LAB-to-canon reconciliation ledger:
  `plan/70-lab-to-canon-reconciliation-ledger.md`
- G1 ordinary assignment target draft:
  `plan/71-g1-ordinary-assignment-target.md`
- G1 SCN-01/SCN-02 static consequence drilldown:
  `plan/72-g1-scn01-scn02-static-consequence-drilldown.md`
- G1 OBL-001 Lean statement inventory:
  `plan/73-g1-obl001-lean-statement-inventory.md`
- G1 OBL-001 repo-local Lean statement draft:
  `plan/74-g1-obl001-lean-statement-draft.md`
- G1 SCN RHS dependency-gap LAB evidence:
  `plan/75-g1-scn-rhs-dependency-gap-evidence.md`
- G1 OBL-020/021 dependency inventory:
  `plan/76-g1-obl020-021-dependency-inventory.md`
- G1 OBL-021 repo-local Lean statement draft:
  `plan/77-g1-obl021-lean-statement-draft.md`
- G1 OBL-020 repo-local Lean statement draft:
  `plan/78-g1-obl020-lean-statement-draft.md`
- G1 E-ROW diagnostic alignment:
  `plan/79-g1-erow-diagnostic-alignment.md`
- G1 diagnostic carrier inventory:
  `plan/80-g1-diagnostic-carrier-inventory.md`
- G1 OBL-024 statement-shape inventory:
  `plan/81-g1-obl024-statement-shape-inventory.md`
- G1 OBL-024 repo-local Lean statement draft:
  `plan/109-g1-obl024-lean-statement-draft.md`
- G1 OBL-024 executable diagnostic-soundness projection carrier:
  `plan/110-g1-obl024-executable-projection-carrier.md`
- G1 OBL-024 projection Rust fixture guard hardening:
  `plan/111-g1-obl024-projection-rust-fixture-guards.md`
- G1 OBL-024 replay vocabulary preflight:
  `plan/112-g1-obl024-replay-vocabulary-preflight.md`
- G1 OBL-024 Lean replay vocabulary refinement:
  `plan/113-g1-obl024-lean-replay-vocabulary-refinement.md`
- G1 OBL-025 statement-shape inventory:
  `plan/82-g1-obl025-statement-shape-inventory.md`
- G1 E-ROW repair payload inventory:
  `plan/83-g1-erow-repair-payload-inventory.md`
- G1 E-ROW carrier-only diagnostic detail prototype:
  `plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md`
- G1 E-ROW carrier precondition hardening:
  `plan/85-g1-erow-carrier-precondition-hardening.md`
- G1 E-ROW-002 visibility repair carrier prototype:
  `plan/86-g1-erow002-visibility-repair-carrier-prototype.md`
- G1 OBL-025 repo-local Lean statement draft:
  `plan/87-g1-obl025-lean-statement-draft.md`
- G1 E-ROW repair shape inventory:
  `plan/88-g1-erow-repair-shape-inventory.md`
- G1 E-ROW-001 non-visibility singleton fixture:
  `plan/89-g1-erow001-non-visibility-singleton-fixture.md`
- G1 E-ROW-001 base singleton fixture closure:
  `plan/92-g1-erow001-base-singleton-fixture-closure.md`
- G1 E-ROW-001 singleton repair assumption gate:
  `plan/93-g1-erow001-singleton-repair-assumption.md`
- G1 E-ROW-001 singleton repair prototype:
  `plan/94-g1-erow001-singleton-repair-prototype.md`
- G1 E-ROW mixed / multi repair decomposition inventory:
  `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- G1 E-ROW set-insertion / bundle payload inventory:
  `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- G1 ELAB-07 set-insertion gate review:
  `plan/97-g1-erow07-set-insertion-gate-review.md`
- G1 ELAB-04 mixed visibility branch inventory:
  `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`
- G1 ELAB-07 set-insertion executable preflight:
  `plan/99-g1-erow07-set-insertion-executable-preflight.md`
- G1 ELAB-07 set-insertion assumption acceptance:
  `plan/100-g1-erow07-set-insertion-assumption-acceptance.md`
- G1 ELAB-07 set-insertion payload-model design:
  `plan/101-g1-erow07-set-insertion-payload-model-design.md`
- G1 ELAB-07 set-insertion executable payload prototype:
  `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md`
- G1 ELAB-07 set-insertion negative-guard hardening:
  `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md`
- G1 ELAB-07 set-insertion row-identity guard hardening:
  `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md`
- G1 ELAB-07 set-insertion exact-locus guard hardening:
  `plan/105-g1-erow07-set-insertion-exact-locus-guard-hardening.md`
- twin peaks の detailed memory:
  `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## maintenance rule

`plan/` は scratchpad ではない。
決定、未決、仮説、履歴 / comparison を混ぜずに書く。
