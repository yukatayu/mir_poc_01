# plan/00 — plan インデックス

## 目的

`plan/` は、この repo の **long-lived repository memory** である。

- 規範判断の正本は `specs/`
- snapshot は `Documentation.md` / `progress.md` / `tasks.md`
- 詳細経緯は `docs/reports/`
- `plan/` は、その間をつなぐ長期参照を保つ

## 先に読む順序

1. `plan/00-index.md`
2. `plan/01-status-at-a-glance.md`
3. `plan/02-system-overview-and-positioning.md`
4. `plan/03-decision-strengths-and-boundaries.md`
5. current-L2 実装寄りなら `plan/04 ... plan/09`
6. 全体計画なら `plan/10-roadmap-overall.md`
7. 直近の research split は `plan/11-roadmap-near-term.md`
8. projection / placement と hot-plug / transport / backend guardrail / avatar slice / typed external executable widening / viewer prototype / public-freeze mixed gate / post-`P18` user-spec hold option inventory / `VerificationLayer` widening threshold / `AttachPoint` minimal contract / `FAIRY-05` carrier bundling / hot-plug real migration rollback boundary / runtime-crate hot-plug engine ownership cut / runtime-crate hot-plug carrier admission cut / post-`P20` historical bridge / post-`P21` rollback-durable-migration boundary family / post-`P21` distributed-activation-ordering boundary family / post-`P21` final-public-hotplug-ABI boundary family / alpha-local type-system freeze / layer compatibility freeze / cut-save-load checkpoint / runtime package avatar policy / Mirrorea Spaces alpha E2E / practical alpha-1 / operational α-0.5 / α-0.8 / α-0.9 / host-I/O and session runtime / product alpha-1 public boundary / operational product sample suite / portal-shard future boundary / Mir computational core / Transform-PoseGraph / projection-backend boundary / engine adapter boundary / autonomous computational-core execution / Full System V1 source-first roadmap / Surface Mir alpha source-authority roadmap の repository-memory roadmap は `plan/20`、`plan/21`、`plan/22`、`plan/23`、`plan/24`、`plan/25`、`plan/26`、`plan/27`、`plan/28`、`plan/29`、`plan/30`、`plan/31`、`plan/32`、`plan/33`、`plan/34`、`plan/35`、`plan/36`、`plan/37`、`plan/38`、`plan/39`、`plan/40`、`plan/41`、`plan/42`、`plan/43`、`plan/44`、`plan/45`、`plan/46`、`plan/47`、`plan/48`、`plan/49`、`plan/50`、`plan/51`、`plan/52`、`plan/53`、`plan/54`、`plan/55`、`plan/56`、`plan/57`、`plan/58`、`plan/59`、`plan/60`、`plan/61`、`plan/62`、`plan/63`、`plan/64`、`plan/65`、`plan/66`、`plan/67`、`plan/68`
9. リスクと heavy line は `plan/12`, `plan/13`, `plan/18`

## current repo の短い要約

- current 主眼は Mir current-L2
- active base corpus は `samples/current-l2/`、active canonical executable suite は `samples/clean-near-end/` に置く
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
  `specs/39-surface-mir-placement-elaboration.md` と `plan/64-surface-mir-placement-roadmap.md` は、canonical Surface Mir place-scope syntax を `S { ... }` に固定し、`S[ ... ]` を sugar としても採用しない rebaseline、P-SURF-01 parser floor、P-SURF-03 Surface-to-Core elaboration evidence floor、P-SURF-04 generated communication evidence floor を置く。`specs/40..43` と `plan/65..68` は indexed state、role admission / capability grant、source patch hot-plug、Surface Full System V1 package order と stop line を整理し、P-SURF-02 で indexed-state semantic checker floor、P-SURF-05 で role admission / capability grant report-level evidence floor を actualize 済みである。`package.mir.json` は alpha artifact に留め、`.mir` files を semantic source authority として扱う
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
- twin peaks の detailed memory:
  `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## maintenance rule

`plan/` は scratchpad ではない。
決定、未決、仮説、履歴 / comparison を混ぜずに書く。
