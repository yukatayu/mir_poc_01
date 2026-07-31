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
20. G1 OBL-001 / OBL-020 / OBL-021 statement guard hardening は `plan/117`
21. G0/G1 ordinary assignment claim-family drilldown は `plan/118`
22. G0 remaining claim-family drilldown priority map は `plan/119`
23. Product Alpha / Full System V1 / Surface evidence recut matrix は `plan/120`
24. G1 minimal source-first vertical slice candidate map は `plan/121`
25. G1 SCN exact static slice manifest は `plan/122`
26. G1 SCN-01 visibility negative actualization は `plan/123`
27. G1 OBL-001 boundary audit は `plan/124`
28. G1 SCN-02 direct-local-write blocker review は `plan/125`
29. G1 OBL-020/021 boundary audit and OBL-021 guard hardening は `plan/126`
30. G1 ordinary-assignment bridge readiness / non-readiness map は `plan/127`
31. G1 bridge handoff / blocker ledger は `plan/128`
32. G1 acceptance-packet preflight は `plan/129`
33. G1 OBL statement/status completion criteria inventory は `plan/130`
34. G1 status proposal packet outline は `plan/131`
35. G1 status evidence readiness dry-run は `plan/132`
36. G1 requested-status options matrix は `plan/133`
37. G1 OBL-020 scope clarification packet は `plan/134`
38. G1 OBL-020 artifact identity / wrapper preflight は `plan/135`
39. G1 OBL-020 artifact annex template は `plan/136`
40. G1 OBL-001 artifact identity / wrapper preflight は `plan/137`
41. G1 OBL-001 artifact annex template は `plan/138`
42. G1 OBL-021 artifact identity / wrapper preflight は `plan/139`
43. G1 OBL-021 artifact annex template は `plan/140`
44. G1 status packet shell with unresolved slots は `plan/141`
45. G1 status packet shell evidence dry-run は `plan/142`
46. G1 OBL-021 equality / diagnostic abstraction decision packet は `plan/143`
47. G1 OBL-020 scope decision reuse / unresolved-slot audit は `plan/144`
48. G1 OBL-001 artifact decision reuse / unresolved-slot audit は `plan/145`
49. G1 OBL-001 explanation-boundary sync guard hardening は `plan/146`
50. G1 next-line promotion-boundary audit は `plan/147`
51. G1 E-ROW diagnostic alignment は `plan/79`
52. G1 diagnostic carrier inventory は `plan/80`
53. G1 OBL-024 statement-shape inventory は `plan/81`
54. G1 OBL-025 statement-shape inventory は `plan/82`
55. G1 E-ROW repair payload inventory は `plan/83`
56. G1 E-ROW carrier-only diagnostic detail prototype は `plan/84`
57. G1 E-ROW carrier precondition hardening は `plan/85`
58. G1 E-ROW-002 visibility repair carrier prototype は `plan/86`
59. G1 OBL-025 repo-local Lean statement draft は `plan/87`
60. G1 E-ROW repair shape inventory は `plan/88`
61. G1 E-ROW-001 non-visibility singleton fixture は `plan/89`
62. G1 E-ROW-001 base singleton fixture closure は `plan/92`
63. G1 E-ROW-001 singleton repair assumption gate は `plan/93`
64. G1 E-ROW-001 singleton repair prototype は `plan/94`
65. G1 E-ROW mixed / multi repair decomposition inventory は `plan/95`
66. G1 E-ROW set-insertion / bundle payload inventory は `plan/96`
67. G1 ELAB-07 set-insertion gate review は `plan/97`
68. G1 ELAB-04 mixed visibility branch inventory は `plan/98`
69. G1 ELAB-07 set-insertion executable preflight は `plan/99`
70. G1 ELAB-07 set-insertion assumption acceptance は `plan/100`
71. G1 ELAB-07 set-insertion payload-model design は `plan/101`
72. G1 ELAB-07 set-insertion executable payload prototype は `plan/102`
73. G1 ELAB-07 set-insertion negative-guard hardening は `plan/103`
74. G1 ELAB-07 set-insertion row-identity guard hardening は `plan/104`
75. G1 ELAB-07 set-insertion exact-locus guard hardening は `plan/105`
76. G1 ELAB-07 child / bundle / partial exclusion fixtures は `plan/106`
77. G1 ELAB-04 mixed visibility payload-model preflight は `plan/107`
78. G1 OBL-025 branch-local non-coverage refinement は `plan/108`
79. G1 OBL-025 repair completeness guard hardening は `plan/116`
80. G1 OBL-024 repo-local Lean statement draft は `plan/109`
81. G1 OBL-024 executable diagnostic-soundness projection carrier は `plan/110`
82. G1 OBL-024 projection Rust fixture guard hardening は `plan/111`
83. G1 OBL-024 replay vocabulary preflight は `plan/112`
84. G1 OBL-024 Lean replay vocabulary refinement は `plan/113`
85. G1 OBL-024 Lean association vocabulary refinement は `plan/114`
86. G1 OBL-024 association guard hardening は `plan/115`
87. storage workdir mountpoint guard hardening は `plan/148`
88. current phase position reading は `plan/149`
89. phase-position validator guard は `plan/150`
90. Discord webhook secret validator guard は `plan/151`
91. Discord notification file inputs は `plan/152`
92. G0 closeout evidence and non-applied exit decision packet は `plan/153`
93. project control cockpit and reporting protocol は `plan/154`
94. T0/G0 governance-profile adoption and one-off evaluation は `plan/155`
95. pre-delegation T0-T2 research autonomy envelope は `plan/156`
96. historical exact-target delegated L2/L3 theory research governance は `plan/157`
97. standing bounded autonomy と first research ratchet は `plan/158`
98. WRK evidence-commit integrity recut は `plan/159`
99. OBL-021 statement-shape checkpoint は `plan/160`
100. post-checkpoint candidate triage and runnable baseline は `plan/161`
101. post-WRK-0006 candidate selection は
    `plan/162-post-wrk0006-candidate-selection.md`
102. foundation integrity と elaboration outcome audit は
    `plan/163-foundation-integrity-and-elaboration-outcome-audit.md`
103. OBL-001 result/write coverage boundary は
    `plan/164-obl001-result-write-coverage-boundary.md`
104. post-WRK-0007 candidate selection は
    `plan/165-post-wrk0007-candidate-selection.md`
105. Mir computational baseline directness audit は
    `plan/166-mir-computational-baseline-directness-audit.md`
106. P-COMP-03 rejection-phase cross-carrier audit は
    `plan/167-pcomp03-rejection-phase-cross-carrier-audit.md`
107. WRK-0001 finite-index reproduction evidence は `plan/wrk-0001-finite-index-reproduction.md`
108. WRK-0001 pilot checkpoint は `plan/wrk-0001-pilot-checkpoint.md`
109. WRK-0002 OBL-021 projection-vacuity countermodel は `plan/wrk-0002-projection-vacuity-countermodel.md`
110. WRK-0007 OBL-001 result/write coverage countermodel は `plan/wrk-0007-obl001-result-write-coverage.md`
111. WRK-0008 OBL-027 formal-hook attribution audit は `plan/wrk-0008-obl027-formal-hook-attribution.md`
112. WRK-0009 e5 proof-skeleton identity selection は `plan/168-wrk0009-e5-skeleton-identity-selection.md`
113. WRK-0009 e5 proof-skeleton literal identity evidence は `plan/wrk-0009-e5-skeleton-identity.md`
114. WRK-0010 static decision attribution selection は `plan/169-wrk0010-static-decision-attribution-selection.md`
115. WRK-0010 static decision attribution evidence は `plan/wrk-0010-static-formal-hook-decision-attribution.md`
116. WRK-0011 current-L2 final-store assertion directness evidence は `plan/wrk-0011-current-l2-final-store-directness.md`
117. post-WRK-0011 candidate selection は `plan/170-post-wrk0011-candidate-selection.md`
118. ChatGPT Pro Oracle browser consults の repo-local 運用は `.docs/oracle-chatgpt-pro-operations.md`
119. リスクと heavy line は `plan/12`, `plan/13`, `plan/18`
120. WRK-0013 retained-reproduction selection は `plan/wrk-0013-retained-reproduction-selection.md`
121. WRK-0013 P-COMP-03 retained reproduction evidence は
    `plan/wrk-0013-pcomp03-retained-reproduction.md`
122. post-WRK-0013 standing-target no-candidate disposition は
    `plan/post-wrk0013-no-candidate-disposition.md`
123. post-WRK-0013 whole-portfolio review は
    `plan/post-wrk0013-portfolio-review.md`
124. theory core correspondence と disposition checkpoint は
    `plan/171-theory-core-correspondence-and-disposition-checkpoint.md`
125. WRK-0015 stale-fence source-local candidate selection は
    `plan/wrk-0015-stale-grant-fence-selection.md`
126. WRK-0015 stale-fence registration preflight は
    `plan/wrk-0015-stale-grant-fence-registration-preflight.md`
127. post-WRK-0015 permitted-root candidate disposition は
    `plan/post-wrk0015-permitted-root-no-candidate-disposition.md`
128. standing-autonomy lane correspondence checkpoint は
    `plan/172-standing-autonomy-lane-correspondence-checkpoint.md`
129. current local predicate constructive-decidability candidate selection は
    `plan/173-local-predicate-constructive-decidability-selection.md`
130. local predicate proposition-decidability successor selection は
    `plan/174-local-predicate-proposition-decidability-selection.md`
131. post-WRK-0017 axiom-profile disposition は
    `plan/175-post-wrk0017-axiom-profile-disposition.md`
132. current standing-candidate disposition は
    `plan/176-current-standing-candidate-disposition.md`
133. THM-005 telemetry-effect boundary selection は
    `plan/177-thm005-telemetry-effect-boundary-selection.md`
134. post-WRK-0018 candidate re-screen は
    `plan/178-post-wrk0018-candidate-rescreen.md`
135. independent source-locus candidate audit は
    `plan/179-independent-source-locus-audit.md`
136. T1/T2 statement identity and dependency closure audit は
    `plan/180-t1-t2-statement-identity-dependency-closure-audit.md`
137. preservation-proof prerequisite literature audit は
    `plan/181-preservation-proof-prerequisite-literature-audit.md`
138. Canon Core minimality and proof-interface audit は
    `plan/182-canon-core-minimality-and-proof-interface-audit.md`
139. transparent cost-bound substitutability decision memory は
    `plan/183-transparent-cost-bound-substitutability-decision.md`
140. WRK-0019 P-COMP-03 bounds direct-carrier evidence は
    `plan/wrk-0019-pcomp03-bounds-direct-carrier.md`
141. WRK-0020 option-admit literal-audit falsifier は
    `plan/wrk-0020-option-admit-carrier-literal-audit.md`
142. WRK-0021 CostBudget scalar-projection falsifier は
    `plan/wrk-0021-costbudget-scalar-projection-falsifier.md`
143. post-WRK-0021 autonomous frontier triage は
    `plan/184-post-wrk0021-autonomous-frontier-triage.md`
144. cost-bound substitutability primary-literature audit は
    `plan/185-cost-bound-substitutability-primary-literature-audit.md`
145. canonical elaboration/trace interface-closure audit は
    `plan/186-canonical-elaboration-trace-interface-closure-audit.md`
146. MirCore value-flow / occurrence identity decision packet は
    `plan/187-mircore-value-flow-and-occurrence-decision-packet.md`
147. parser-free chain closure integrity correction は
    `plan/188-parser-free-chain-closure-integrity.md`
148. 選択済み意味論の合成と推論境界は
    `plan/199-selected-semantic-composition-and-inference-boundary.md`
149. WRK-0024 SCN-02 read/write snapshot ambiguity evidence は
    `plan/wrk-0024-scn02-read-write-snapshot-ambiguity.md`

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
- **repo triage recut matrix**:
  `plan/120-repo-triage-recut-matrix.md` は Product Alpha / Full System V1 /
  Surface evidence を keep-core-idea / useful-floor / archive-exploration /
  postpone/drop-from-current-recut の読みへ分類する。これは LAB memory であり、
  file move / archive operation / canon edit / gate exit / sample status relabel ではない
- **G1 minimal vertical slice candidate**:
  `plan/121-g1-minimal-vertical-slice-candidate-map.md` は、`plan/120` の分類を踏まえ、
  G1 ordinary assignment に渡す最小 source-first static slice を
  `G1-MVS-ASSIGNMENT-STATIC` として整理する。これは LAB memory であり、
  canon edit / G1 exit / proof discharge / conformance / runtime or product
  scope promotion / sample status relabel ではない
- **G1 SCN exact static slice manifest**:
  `plan/122-g1-scn-exact-static-slice-manifest.md` は、`plan/121` の slice を
  SCN-01 / SCN-02 の static bullets へ割り当て、`ELAB-11` と `ELAB-12` を
  exact current executable evidence、`ELAB-02/05/07/09/10` を structural
  support、runtime bullets を explicit gap / out of scope として整理する。
  これは LAB memory であり、canon edit / G1 exit / proof discharge /
  conformance / runtime or product scope promotion / sample status relabel ではない
- **G1 SCN-01 visibility negative actualization**:
  `plan/123-g1-scn01-visibility-negative-actualization.md` は、`plan/122` で
  structural support only だった SCN-01 visible-write `VisibilityDenied`
  negative gap を `ELAB-17` として exact current executable evidence にする。
  OBL-001 の抽象 predicate は現時点で先に広げず、canon edit / G1 exit /
  proof discharge / conformance / runtime or product scope promotion / final
  diagnostic-repair ABI freeze は主張しない
- **G1 OBL-001 boundary audit**:
  `plan/124-g1-obl001-boundary-audit.md` は、`ELAB-11`、`ELAB-12`、
  `ELAB-17` を既存 OBL-001 abstract predicate boundary が運べるかを監査し、
  現時点では Lean predicate refinement 不要と判断する。`ELAB-17` は
  OBL-001 failure-containment pressure として扱い、diagnostic projection /
  repair payload details は OBL-024 / OBL-025 LAB evidence に残す。canon edit /
  OBL completion / proof discharge / conformance / G1 exit は主張しない
- **G1 SCN-02 direct-local-write blocker review**:
  `plan/125-g1-scn02-direct-local-write-blocker-review.md` は、`plan/122` の
  SCN-02 direct-local-write negative (b) が現 G1 bridge の即時 blocker では
  ないと判断する。`ELAB-12` の exact positive owner-directed request shape、
  `ELAB-02` の structural support、`IDX-05` の ambient-authority rejection、
  `plan/124` の OBL-001 boundary audit を根拠にする一方、negative (b) の
  exact executable negative evidence はまだ主張しない
- **G1 OBL-020/021 boundary audit and OBL-021 guard hardening**:
  `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md` は、
  OBL-020 / OBL-021 の current statement boundary を監査し、現時点では
  Lean predicate refinement 不要と判断する。あわせて OBL-021 required body
  links が comment-only で通らないよう sync guard を test-only で強化する。
  canon edit / OBL completion / proof discharge / conformance / G1 exit は主張しない
- **G1 ordinary-assignment bridge readiness / non-readiness map**:
  `plan/127-g1-ordinary-assignment-bridge-readiness-nonreadiness-map.md` は、
  post-`plan/126` の G1 ordinary-assignment bridge について、current LAB
  support、remaining blocker、forbidden claim を canon G1 criterion ごとに
  1 表へ分ける。これは次の narrow support package を続けるための map であり、
  G1 exit readiness、T1 readiness、OBL completion、conformance、runtime readiness
  は主張しない
- **G1 bridge handoff / blocker ledger**:
  `plan/128-g1-bridge-handoff-blocker-ledger.md` は、`plan/127` の blockers を
  human/canon acceptance、future proof-package work、static LAB support-only、
  later runtime / conformance / product、reserve trigger に分類する。これは
  受け渡し ledger であり、canon edit、gate exit、OBL completion、proof、
  conformance、runtime readiness は主張しない
- **G1 acceptance-packet preflight**:
  `plan/129-g1-acceptance-packet-preflight.md` は、future G1 acceptance packet
  のために、human/canon review が受け入れる / 更新する canon files、対応する
  LAB evidence、OBL-001 / OBL-020 / OBL-021 statement/status blockers、OPEN-014
  deferral point、runtime / conformance / product exclusions を事前整理する。
  これは preflight-only であり、canon edit、gate exit、OBL completion、proof、
  conformance、runtime readiness は主張しない
- **G1 OBL statement/status completion criteria inventory**:
  `plan/130-g1-obl-statement-status-completion-criteria-inventory.md` は、
  OBL-001 / OBL-020 / OBL-021 の status movement を将来 proposal する前に
  必要な criteria を、status vocabulary、artifact identity、compile-check /
  guard evidence、SCN evidence trace、out-of-scope proof/runtime/conformance
  boundary に分けて整理する。これは criteria inventory-only であり、canon
  edit、ledger movement、gate exit、OBL completion、proof、conformance、
  runtime readiness は主張しない
- **G1 status proposal packet outline**:
  `plan/131-g1-status-proposal-packet-outline.md` は、`plan/130` の criteria
  matrix を future proposal packet の cover sheet、canon-state summary、
  requested-status matrix、artifact identity annex、evidence trace annex、
  OPEN / deferral annex、non-claim appendix、ledger delta placeholder、
  submission checklist へ展開する。これは outline-only であり、requested
  status selection、draft proposal submission、canon edit、ledger movement、
  gate exit、OBL completion、proof、conformance、runtime readiness は主張しない
- **G1 status evidence readiness dry-run**:
  `plan/132-g1-status-evidence-readiness-dry-run.md` は、future proposal
  packet が cite する OBL-001 / OBL-020 / OBL-021 Lean draft の direct
  compile-check、sync guard、admitted-stub / placeholder scan を dry-run として
  記録する。これは evidence-readiness-only であり、requested status selection、
  draft proposal submission、canon edit、ledger movement、gate exit、
  OBL completion、proof、conformance、runtime readiness は主張しない
- **G1 requested-status options matrix**:
  `plan/133-g1-requested-status-options-matrix.md` は、OBL-001 / OBL-020 /
  OBL-021 の future requested-status 候補として `stated` と `lean-stated` を
  比較する。推奨は advisory-only であり、requested status acceptance、
  proposal submission、canon edit、ledger movement、gate exit、OBL completion、
  proof、conformance、runtime readiness は主張しない
- **G1 OBL-020 scope clarification packet**:
  `plan/134-g1-obl020-scope-clarification-packet.md` は、OBL-020 full-row
  status movement を defer し、G1-supporting statement-scope candidate を
  later human/canon review 用に明示する。これは scope-clarification-only
  であり、requested status acceptance、proposal submission、canon edit、
  ledger movement、gate exit、OBL completion、proof、conformance、runtime
  readiness は主張しない
- **G1 OBL-020 artifact identity / wrapper preflight**:
  `plan/135-g1-obl020-artifact-identity-wrapper-preflight.md` は、現 LAB
  `OBL020StatementDraft` の direct citation を LAB evidence に限定し、
  requested-status artifact identity の前に artifact annex / wrapper
  decision が必要であることを整理する。これは preflight-only であり、
  wrapper file creation、requested status acceptance、proposal submission、
  canon edit、ledger movement、gate exit、OBL completion、proof、conformance、
  runtime readiness は主張しない
- **G1 OBL-020 artifact annex template**:
  `plan/136-g1-obl020-artifact-annex-template.md` は、later requested-status
  packet 用の non-applied artifact annex template として、canon OBL-020
  target、LAB artifact path / namespace / constant、scope label、fresh
  validation slots、decision slots、unresolved items、non-claims をまとめる。
  proposal submission、requested status acceptance、ledger movement、wrapper
  creation、OBL-020 completion、proof、conformance、runtime readiness は
  主張しない
- **G1 OBL-001 artifact identity / wrapper preflight**:
  `plan/137-g1-obl001-artifact-identity-wrapper-preflight.md` は、現 LAB
  `THM001StatementDraft` の direct citation を LAB evidence に限定しつつ、
  OBL-001 が later `lean-stated` の最有力候補であること、ただし
  requested-status artifact identity には artifact annex / wrapper decision
  が必要であることを整理する。wrapper file creation、requested status
  acceptance、proposal submission、canon edit、ledger movement、gate exit、
  OBL-001 completion、OBL-002 proof、conformance、runtime readiness は主張しない
- **G1 OBL-001 artifact annex template**:
  `plan/138-g1-obl001-artifact-annex-template.md` は、later OBL-001
  `lean-stated` packet 用の non-applied artifact annex template として、
  canon target、LAB artifact path / namespace / constant、OPEN-014 deferral、
  simple assignment scope、fresh validation slots、decision slots、unresolved
  items、non-claims をまとめる。proposal submission、requested status
  acceptance、ledger movement、wrapper creation、OBL-001 completion、
  OBL-002 proof、conformance、runtime readiness は主張しない
- **G1 OBL-021 artifact identity / wrapper preflight**:
  `plan/139-g1-obl021-artifact-identity-wrapper-preflight.md` は、現 LAB
  `OBL021StatementDraft` の direct citation を LAB evidence に限定しつつ、
  OBL-021 が abstraction-boundary acceptance 後の conditional later
  `lean-stated` candidate であること、ただし requested-status artifact
  identity には artifact annex / wrapper decision が必要であることを整理する。
  wrapper file creation、requested status acceptance、proposal submission、
  canon edit、ledger movement、gate exit、OBL-021 completion、proof、
  conformance、runtime scheduling determinism、final equality / Diagnostic ABI
  selection は主張しない
- **G1 OBL-021 artifact annex template**:
  `plan/140-g1-obl021-artifact-annex-template.md` は、later OBL-021
  conditional `lean-stated` packet 用の non-applied artifact annex template
  として、canon target、LAB artifact path / namespace / constant、
  abstraction-boundary decision、fresh validation slots、artifact/wrapper
  decision slots、unresolved final equality / Diagnostic ABI /
  projection-totality items、non-claims をまとめる。proposal submission、
  requested status acceptance、ledger movement、wrapper creation、OBL-021
  completion、proof、conformance、runtime scheduling determinism、G1 exit は
  主張しない
- **G1 status packet shell with unresolved slots**:
  `plan/141-g1-status-packet-shell-unresolved-slots.md` は、`plan/138` /
  `plan/136` / `plan/140` の artifact annex template を参照する
  non-applied G1 OBL status packet shell として、requested status、ledger
  delta、artifact identity acceptance、wrapper need、OPEN-014 handling、
  OBL-020 scope、OBL-021 abstraction boundary、proof、conformance、runtime、
  G1 exit をすべて unresolved slot に残す。proposal submission、requested
  status acceptance、canon edit、ledger movement、OBL completion、proof /
  conformance claim、runtime readiness、G1 exit は主張しない
- **G1 status packet shell evidence dry-run**:
  `plan/142-g1-status-packet-shell-evidence-dry-run.md` は、`plan/141` が
  要求した OBL-001 / OBL-020 / OBL-021 Lean compile-check、LAB statement sync
  guard、admitted-stub / placeholder scan、docs / source hierarchy validation、
  secret scan を fresh に実行した LAB evidence record である。requested
  status selection、proposal submission、canon edit、ledger movement、OBL
  completion、proof / conformance claim、runtime readiness、G1 exit は主張しない
- **G1 OBL-021 equality / diagnostic abstraction decision packet**:
  `plan/143-g1-obl021-equality-diagnostic-abstraction-decision-packet.md` は、
  `plan/140` の OBL-021 artifact annex template を埋める前に human/canon
  review に問うべき result equality / diagnostic equivalence /
  projection-totality / fixed-input identity / LAB artifact-or-wrapper
  boundary を切り出す
  decision-packet-only memory である。requested status selection、proposal
  submission、canon edit、ledger movement、OBL-021 completion、proof /
  conformance claim、final equality / Diagnostic ABI selection、runtime
  scheduling determinism、G1 exit は主張しない
- **G1 OBL-020 scope decision reuse / unresolved-slot audit**:
  `plan/144-g1-obl020-scope-decision-reuse-audit.md` は、`plan/134` が
  OBL-020 full-row vs G1-supporting scope question の controlling LAB packet
  であることを確認し、同じ scope matrix を二重に作らないための
  anti-duplication / unresolved-slot audit である。requested status
  selection、proposal submission、canon edit、ledger movement、OBL-020
  completion、proof / conformance claim、wrapper creation、runtime readiness、
  G1 exit は主張しない
- **G1 OBL-001 artifact decision reuse / unresolved-slot audit**:
  `plan/145-g1-obl001-artifact-decision-reuse-audit.md` は、`plan/137` /
  `plan/138` が OBL-001 artifact identity / wrapper / OPEN-014 /
  simple-assignment scope の controlling LAB decision surface であることを
  確認し、同じ preflight / annex を二重に作らないための
  anti-duplication / unresolved-slot audit である。requested status
  selection、proposal submission、canon edit、ledger movement、OBL-001
  completion、OBL-002 proof / conformance claim、wrapper creation、OPEN-014
  resolution、runtime readiness、G1 exit は主張しない
- **G1 OBL-001 explanation-boundary sync guard hardening**:
  `plan/146-g1-obl001-explanation-boundary-guard-hardening.md` は、
  `THM001StatementDraft.md` が LAB/canon hierarchy、canon
  `MirCore.Elab.Soundness (stmt)` target との非同一性、artifact identity /
  wrapper acceptance 未採択、OPEN-014 open を落とさないことを sync test で
  固定する test-only guard hardening である。requested status selection、
  proposal submission、canon edit、ledger movement、OBL-001 completion、
  OBL-002 proof / conformance claim、wrapper creation、OPEN-014 resolution、
  runtime readiness、G1 exit は主張しない
- **G1 next-line promotion-boundary audit**:
  `plan/147-g1-next-line-promotion-boundary-audit.md` は、広い自走依頼が
  OBL-020 / OBL-001 review-facing extraction candidate の promotion ではない
  ことを記録する queue-boundary audit である。次 line は user が明示的に
  OBL-020 または OBL-001 extraction を選んだときだけ昇格する。requested
  status selection、review request extraction、proposal submission、canon
  edit、ledger movement、wrapper creation、OPEN-014 resolution、proof /
  conformance claim、runtime readiness、G1 exit は主張しない
- **storage workdir mountpoint guard hardening**:
  `plan/148-storage-workdir-mountpoint-guard-hardening.md` は、external
  workdir が exact mountpoint ではない場合に `--ensure-dirs` と confirmed
  cleanup が root filesystem 上の通常 directory を mounted 扱いしないよう
  guard を強化する tooling memory である。cleanup、mount provisioning、
  sample status、workflow status、canon edit、OBL status、proof /
  conformance claim、runtime readiness、G1 exit は主張しない
- **current phase position reading**:
  `plan/149-current-phase-position-reading.md` は、全体計画の現在地を
  canon lifecycle では `T0/G0 rebaseline`、人間向け stage count では
  9 段階中 1 段階目として読み、T0 内では late pre-exit だが G0 exit は
  未達と整理する repository memory である。LAB evidence の先行と canon
  phase movement を混同せず、G0/G1 exit、T1 entry、proof / conformance、
  runtime readiness、sample / workflow status change は主張しない
- **phase-position validator guard**:
  `plan/150-phase-position-validator-guard.md` は、`plan/149` の短い読みが
  `progress.md` / `tasks.md` から消えたときに `scripts/validate_docs.py`
  が落ちるようにした docs-validator hardening memory である。現在は
  `T0/G0 rebaseline`、9 段階中 1 段階目、`late pre-exit`、G0 exit 未達を
  snapshot guard phrase として守る。canon edit、phase / gate movement、
  percentage-as-gate、runtime / sample / workflow status change は主張しない
- **Discord webhook secret validator guard**:
  `plan/151-discord-webhook-secret-validator-guard.md` は、tracked docs /
  source に concrete Discord webhook URL shape が入ったときに
  `scripts/validate_docs.py` が URL 本体を出さず path / line だけで失敗する
  guard hardening memory である。credential 保存、通知挙動変更、security
  completeness claim は主張しない
- **Discord notification file inputs**:
  `plan/152-discord-notification-file-inputs.md` は、Discord 通知文に
  backtick / quote / newline / long commit list が含まれる場合に
  `--summary-file` / `--next-step-file` を使い、shell command substitution
  で通知文が欠落する事故を避けるための repo-local notification operation
  hardening memory である。Webhook storage、delivery guarantee、phase /
  gate movement は主張しない
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
- G0/G1 ordinary assignment claim-family drilldown:
  `plan/118-g0-g1-ordinary-assignment-claim-family-drilldown.md`
- G0 remaining claim-family drilldown priority map:
  `plan/119-g0-remaining-claim-family-drilldown-priority.md`
- G1 minimal vertical slice candidate map:
  `plan/121-g1-minimal-vertical-slice-candidate-map.md`
- G1 SCN exact static slice manifest:
  `plan/122-g1-scn-exact-static-slice-manifest.md`
- G1 SCN-01 visibility negative actualization:
  `plan/123-g1-scn01-visibility-negative-actualization.md`
- G1 OBL-001 boundary audit:
  `plan/124-g1-obl001-boundary-audit.md`
- G1 SCN-02 direct-local-write blocker review:
  `plan/125-g1-scn02-direct-local-write-blocker-review.md`
- G1 OBL-020/021 boundary audit and OBL-021 guard hardening:
  `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`
- G1 ordinary-assignment bridge readiness / non-readiness map:
  `plan/127-g1-ordinary-assignment-bridge-readiness-nonreadiness-map.md`
- G1 bridge handoff / blocker ledger:
  `plan/128-g1-bridge-handoff-blocker-ledger.md`
- G1 acceptance-packet preflight:
  `plan/129-g1-acceptance-packet-preflight.md`
- G1 OBL statement/status completion criteria inventory:
  `plan/130-g1-obl-statement-status-completion-criteria-inventory.md`
- G1 status proposal packet outline:
  `plan/131-g1-status-proposal-packet-outline.md`
- G1 status evidence readiness dry-run:
  `plan/132-g1-status-evidence-readiness-dry-run.md`
- G1 requested-status options matrix:
  `plan/133-g1-requested-status-options-matrix.md`
- G1 OBL-020 scope clarification packet:
  `plan/134-g1-obl020-scope-clarification-packet.md`
- G1 OBL-020 artifact identity / wrapper preflight:
  `plan/135-g1-obl020-artifact-identity-wrapper-preflight.md`
- G1 OBL-020 artifact annex template:
  `plan/136-g1-obl020-artifact-annex-template.md`
- G1 OBL-001 artifact identity / wrapper preflight:
  `plan/137-g1-obl001-artifact-identity-wrapper-preflight.md`
- G1 OBL-001 artifact annex template:
  `plan/138-g1-obl001-artifact-annex-template.md`
- G1 OBL-021 artifact identity / wrapper preflight:
  `plan/139-g1-obl021-artifact-identity-wrapper-preflight.md`
- G1 OBL-021 artifact annex template:
  `plan/140-g1-obl021-artifact-annex-template.md`
- G1 status packet shell with unresolved slots:
  `plan/141-g1-status-packet-shell-unresolved-slots.md`
- G1 status packet shell evidence dry-run:
  `plan/142-g1-status-packet-shell-evidence-dry-run.md`
- G1 OBL-021 equality / diagnostic abstraction decision packet:
  `plan/143-g1-obl021-equality-diagnostic-abstraction-decision-packet.md`
- G1 OBL-020 scope decision reuse / unresolved-slot audit:
  `plan/144-g1-obl020-scope-decision-reuse-audit.md`
- G1 OBL-001 artifact decision reuse / unresolved-slot audit:
  `plan/145-g1-obl001-artifact-decision-reuse-audit.md`
- G1 OBL-001 explanation-boundary sync guard hardening:
  `plan/146-g1-obl001-explanation-boundary-guard-hardening.md`
- G1 next-line promotion-boundary audit:
  `plan/147-g1-next-line-promotion-boundary-audit.md`
- Storage workdir mountpoint guard hardening:
  `plan/148-storage-workdir-mountpoint-guard-hardening.md`
- Current phase position reading:
  `plan/149-current-phase-position-reading.md`
- Phase-position validator guard:
  `plan/150-phase-position-validator-guard.md`
- Discord webhook secret validator guard:
  `plan/151-discord-webhook-secret-validator-guard.md`
- Discord notification file inputs:
  `plan/152-discord-notification-file-inputs.md`
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
- G1 OBL-001 / OBL-020 / OBL-021 statement guard hardening:
  `plan/117-g1-obl001-020-021-statement-guard-hardening.md`
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
- G1 OBL-024 Lean association vocabulary refinement:
  `plan/114-g1-obl024-lean-association-vocabulary-refinement.md`
- G1 OBL-024 association guard hardening:
  `plan/115-g1-obl024-association-guard-hardening.md`
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
- G1 OBL-025 repair completeness guard hardening:
  `plan/116-g1-obl025-repair-completeness-guard-hardening.md`
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
- G1 ELAB-07 child / bundle / partial exclusion fixtures:
  `plan/106-g1-erow07-child-bundle-partial-exclusion-fixtures.md`
- G1 ELAB-04 mixed visibility payload-model preflight:
  `plan/107-g1-erow04-mixed-visibility-payload-model-preflight.md`
- G1 OBL-025 branch-local non-coverage refinement:
  `plan/108-g1-obl025-branch-local-noncoverage-refinement.md`
- WRK-0001 finite-index reproduction evidence:
  `plan/wrk-0001-finite-index-reproduction.md`
- WRK-0001 pilot checkpoint:
  `plan/wrk-0001-pilot-checkpoint.md`
- OBL-021 statement-shape checkpoint:
  `plan/160-obl021-statement-shape-checkpoint.md`
- Post-WRK-0006 candidate selection:
  `plan/162-post-wrk0006-candidate-selection.md`
- OBL-001 result/write coverage boundary:
  `plan/164-obl001-result-write-coverage-boundary.md`
- Post-WRK-0007 candidate selection:
  `plan/165-post-wrk0007-candidate-selection.md`
- WRK-0007 OBL-001 result/write coverage countermodel:
  `plan/wrk-0007-obl001-result-write-coverage.md`
- twin peaks の detailed memory:
  `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- preservation-proof prerequisite literature audit:
  `plan/181-preservation-proof-prerequisite-literature-audit.md`
- Canon Core minimality and proof-interface audit:
  `plan/182-canon-core-minimality-and-proof-interface-audit.md`
- transparent cost-bound substitutability decision memory:
  `plan/183-transparent-cost-bound-substitutability-decision.md`
- WRK-0020 option-admit literal-audit falsifier:
  `plan/wrk-0020-option-admit-carrier-literal-audit.md`
- WRK-0021 CostBudget scalar-projection falsifier:
  `plan/wrk-0021-costbudget-scalar-projection-falsifier.md`
- WRK-0022 WRITE-CROSS failure-generation falsifier:
  `plan/wrk-0022-write-cross-failure-generation-boundary-falsifier.md`
- Post-WRK-0022 autonomous frontier triage:
  `plan/191-post-wrk0022-autonomous-frontier-triage.md`
- post-admission request validation-context audit:
  `plan/192-post-admission-request-validation-context-audit.md`
- post-admission validation-context literature and adversarial memo:
  `plan/193-post-admission-validation-context-literature-and-counterexample-memo.md`
- Product Alpha installed-binary replay evidence boundary:
  `plan/194-product-alpha1-installed-binary-replay-evidence.md`
- post-PROPOSAL-013 autonomous frontier delta audit:
  `plan/195-post-proposal013-autonomous-frontier-delta-audit.md`
- T0 から T2 / I1 入口までの条件付き自走ロードマップ:
  `plan/196-t0-t2-implementation-entry-roadmap.md`
- I1 bootstrap の判断・readiness 監査:
  `plan/197-i1-bootstrap-decision-and-readiness-audit.md`
- T0/G0 governance-profile v2 evaluation:
  `plan/198-t0-g0-governance-profile-v2.md`
- WRK-0023 consistent-cut channel-state literal transcription:
  `plan/wrk-0023-consistent-cut-channel-state-boundary.md`
- post-WRK-0023 literal-boundary review:
  `plan/post-wrk0023-literal-boundary-review.md`
- whole-theory foundation audit and reconciliation order:
  `plan/whole-theory-foundation-audit-20260725.md`
- Post-WRK-0021 autonomous frontier triage:
  `plan/184-post-wrk0021-autonomous-frontier-triage.md`
- cost-bound substitutability primary-literature audit:
  `plan/185-cost-bound-substitutability-primary-literature-audit.md`
- canonical elaboration/trace interface-closure audit:
  `plan/186-canonical-elaboration-trace-interface-closure-audit.md`
- MirCore value-flow / occurrence identity decision packet:
  `plan/187-mircore-value-flow-and-occurrence-decision-packet.md`
- parser-free chain closure integrity correction:
  `plan/188-parser-free-chain-closure-integrity.md`
- autonomous theory frontier revalidation:
  `plan/189-autonomous-theory-frontier-revalidation.md`
- first unlocking owner-disposition reading:
  `plan/190-first-unlocking-owner-disposition.md`
- 選択済み意味論の合成と推論境界:
  `plan/199-selected-semantic-composition-and-inference-boundary.md`
- WRK-0027 SCN-08 scalar terminal literal comparison:
  `plan/wrk-0027-scn08-scalar-terminal-correspondence.md`
- re-anchored semantic composition research plan:
  `plan/200-reanchored-semantic-composition-research-plan.md`
- WRK-0028 R0 common-cut source-local fact manifest:
  `plan/wrk-0028-r0-common-cut-fact-manifest.md`
- WRK-0029 C0-B noncircular domain-staging conditional lemma:
  `plan/wrk-0029-c0b-noncircular-domain-staging.md`
- WRK-0030 C2-A source-tagged anti-collapse vocabulary:
  `plan/wrk-0030-c2a-source-tagged-anti-collapse-vocabulary.md`
- WRK-0031 C0-C source-local Diagnostic reference audit:
  `plan/wrk-0031-c0c-source-local-diagnostic-reference-audit.md`
- C5-PRE conditional-A2 issuance-guard candidate selection:
  `plan/201-c5-a2-issuance-guard-candidate-selection.md`
- WRK-0032 C5-PRE ordinary-admission issuance-guard evidence:
  `plan/wrk-0032-c5pre-ordinary-admission-issuance-guard.md`
- V1/R1 presentation-refinement candidate selection:
  `plan/202-v1-r1-presentation-refinement-candidate-selection.md`
- WRK-0033 V1/R1 presentation-refinement evidence:
  `plan/wrk-0033-v1r1-presentation-refinement.md`
- V1/R1 fixed finite-sequence presentation candidate selection:
  `plan/203-v1-r1-finite-sequence-candidate-selection.md`
- WRK-0034 V1/R1 fixed finite-sequence presentation evidence:
  `plan/wrk-0034-v1-r1-finite-sequence-refinement.md`
- WRK-0034 後の意味論合成 frontier provisional disposition:
  `plan/204-wrk0034-semantic-composition-no-candidate-disposition.md`
- C7 parametric factorization candidate selection:
  `plan/205-c7-parametric-factorization-candidate-selection.md`
- WRK-0035 C7 parametric factorization evidence:
  `plan/wrk-0035-c7-parametric-factorization.md`
- C7 cumulative-erasure countermodel candidate selection:
  `plan/206-c7-cumulative-erasure-countermodel-candidate-selection.md`
- WRK-0036 C7 cumulative-erasure countermodel evidence:
  `plan/wrk-0036-c7-cumulative-erasure-countermodel.md`
- WRK-0036 後の自律研究 frontier disposition:
  `plan/207-post-wrk0036-autonomous-frontier-disposition.md`
- C2-B/C3 value-flow design preparation:
  `plan/208-c2b-c3-value-flow-design-preparation.md`
- C2-B/C3 relation-obligation audit:
  `plan/209-c2b-c3-relation-obligation-audit.md`
- C2-B/C3 Family A/B instantiation audit:
  `plan/210-c2b-c3-family-a-b-instantiation-audit.md`
- C2-B/C3 B-primary opaque-anchor candidate selection:
  `plan/211-c2b-c3-b-primary-opaque-anchor-candidate-selection.md`
- WRK-0037 C2-B/C3 B-primary opaque-anchor finite evidence:
  `plan/wrk-0037-c2b-c3-b-primary-opaque-anchor-experiment.md`
- C2-B/C3 bundled/relational presentation comparison selection:
  `plan/212-c2b-c3-bundled-relational-presentation-comparison-selection.md`
- C2-B/C3 fiberwise relational comparison selection:
  `plan/213-c2b-c3-fiberwise-relational-comparison-selection.md`
- WRK-0039 C2-B/C3 fiberwise relational presentation finite evidence:
  `plan/wrk-0039-c2b-c3-fiberwise-relational-presentation-experiment.md`
- Post-WRK-0039 autonomous frontier disposition:
  `plan/214-post-wrk0039-autonomous-frontier-disposition.md`
- C2-B/C3 ordinary design decision packet:
  `plan/215-c2b-c3-ordinary-design-decision-packet.md`
- C2-B/C3 cross-boundary compatibility audit:
  `plan/216-c2b-c3-cross-boundary-compatibility-audit.md`
- C2-B/C3 carrier-neutral conditional comparison:
  `plan/217-c2b-c3-carrier-neutral-conditional-comparison.md`
- C2-B/C3 first candidate-card source preflight:
  `plan/218-c2b-c3-first-card-source-preflight.md`
- C2-B/C3 minimal semantic-residence options:
  `plan/219-c2b-c3-minimal-semantic-residence-options.md`
- C2-B/C3 relation-state proof-obligation audit:
  `plan/220-c2b-c3-relation-state-proof-obligation-audit.md`
- C2-B/C3 Canon proposal preparation:
  `plan/221-c2b-c3-canon-proposal-preparation.md`
- P017 X1 owner-terminal exclusivity candidate selection:
  `plan/222-p017-x1-owner-terminal-exclusivity-candidate-selection.md`
- P017 X1 owner-negative/mutation candidate selection:
  `plan/223-p017-x1-owner-negative-mutation-candidate-selection.md`
- P017 X1 M1 adverse/mutation candidate selection:
  `plan/224-p017-x1-m1-adverse-mutation-candidate-selection.md`
- Post-WRK-0043 fixture-only frontier disposition:
  `plan/225-post-wrk0043-fixture-frontier-disposition.md`
- Post-WRK-0043 cross-lane P0A preflight:
  `plan/226-post-wrk0043-cross-lane-p0a-preflight.md`
- P017 X1 owner-decision vector and choice-neutral consistency matrix:
  `plan/227-p017-x1-decision-vector-and-choice-neutral-consistency.md`
- P017 X1 minimum coherence candidate selection:
  `plan/228-p017-x1-minimum-coherence-candidate-selection.md`
- P017 X1 minimum relation-envelope coherence evidence:
  `plan/wrk-0044-p017-x1-minimum-relation-envelope-coherence.md`
- Post-WRK-0044 no-successor and ordinary-design boundary:
  `plan/229-post-wrk0044-no-successor-ordinary-design-boundary.md`
- P017 X1 first ordinary-design candidate-card preflight:
  `plan/230-p017-x1-first-ordinary-design-card-preflight.md`
- P017 X1 K0 R/L factorization preflight:
  `plan/231-k0-rl-factorization-preflight.md`
- P017 X1 K0 R/L definitional collapse screen:
  `plan/232-p017-x1-k0-rl-definitional-collapse-screen.md`
- P017 X1 K0 B fact-status screen:
  `plan/233-p017-x1-k0-b-fact-status-screen.md`
- P017 X1 K0 typed terminal-success positive-basis card:
  `plan/234-p017-x1-k0-terminal-success-positive-basis-card.md`
- P017 X1 typed owner-result role conformance audit:
  `plan/235-p017-x1-typed-owner-result-role-conformance-audit.md`
- P017 X1 K0 owner-result provenance basis and definability screen:
  `plan/236-p017-x1-k0-owner-result-provenance-basis-and-definability-screen.md`
- P017 X1 K0 owner-outstanding positive-basis and pending-nonconflation card:
  `plan/237-p017-x1-k0-owner-outstanding-positive-basis-and-pending-nonconflation-card.md`
- P017 X1 K0 typed terminal owner-service-failure positive-basis card:
  `plan/238-p017-x1-k0-terminal-failure-positive-basis-and-failure-nonconflation-card.md`
- P017 X1 K0 consulted validation-provenance basis and nonconflation screen:
  `plan/239-p017-x1-k0-consulted-validation-provenance-basis-and-nonconflation-screen.md`
- P017 X1 K0 minimum-model H_K intake and fail-closed receipt-causality gate:
  `plan/240-p017-x1-k0-minimum-model-hk-intake-and-fail-closed-gate.md`
- P017 X1 K0 H_K-rs occurrence-accounting preflight:
  `plan/241-p017-x1-k0-hk-rs-occurrence-accounting-preflight.md`
- P017 X1 K0 H_K-rs integrated conditional candidate selection:
  `plan/242-p017-x1-k0-hk-rs-integrated-conditional-candidate-selection.md`
- P017 X1 K0 H_K-rs L3 standing-eligibility recheck:
  `plan/243-p017-x1-k0-hk-rs-l3-standing-eligibility-recheck.md`
- P017 X1 K0 H_K-rs single-block premise/falsifier design:
  `plan/244-p017-x1-k0-hk-rs-source-premise-falsifier-design.md`
- P017 X1 K0 H_K-rs A-Sigma conditional trace source evidence:
  `plan/wrk-0045-p017-x1-k0-hk-rs-asigma-conditional-trace.md`
  (`Reliance status: frozen`; `DEFER`; registered branch-sharing falsifier reproduced)
- Post-WRK-0045 no-successor / ordinary X1 handoff:
  `plan/245-post-wrk0045-no-successor-ordinary-x1-handoff.md`
  (the original `NO-SUCCESSOR / DEFER` applies only to the predicate-only
  WRK-0045 A-Sigma line; the independent K0 U/L re-screen is recorded in Plan 245)
- WRK-0046 P017 X1 K0 q-fiber U/L finite conditional evidence:
  `plan/wrk-0046-p017-x1-k0-qf-ul-lift.md`
  (`L3-open`, `not-promoted`; A0 conditionally excludes two consumes on one
  supplied finite line and A1 retains the omission/reset control. It has no
  P017 model, semantic-selection, or critical-path effect.)

## maintenance rule

`plan/` は scratchpad ではない。
決定、未決、仮説、履歴 / comparison を混ぜずに書く。
