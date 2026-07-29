---
id: root/map
status: L1-fixed
maturity: draft
depends_on: [root/readme]
summary: 知識マップ 1 枚。全域の依存図、3 種の読み筋、ID 体系。
open_items: []
---

# MAP — 知識マップ

## 全域の依存図

```text
NORTH-STAR (軸)
   │
   ▼
adr/ (決定 ADR-0001..0014) ◄──── GLOSSARY (概念 CON-###)
   │
   ├──── working/ (WRK-####: 可逆な L2/L3 research annex、既存正本は read-only)
   │
   ▼
theory/ (MirCore v0)                     mental-model/ (表と裏の対応)
  00 見取り図                                 ▲
  01 計算体系 ──► 02 型/効果/失敗              │ 03 が theory/10 と対応
  01 ──► 03 elaboration ──► THM-001           │
  01 ──► 04 順序と cut ──► 05 権限            │
  01 ──► 06 存在と fallback                   │
  01 ──► 07 観測  08 patch  09 二層時間  10 診断
  11 定理・義務台帳(THM/OBL)  12 文献
   │
   ▼
spec/ (Mir Report: 文法・静的意味・Core IR・runtime・適合・診断)
   │                                    architecture/ (層 S0-S7、
   ▼                                      契約 BND-###、toolchain、carrier)
scenarios/ (SCN-01..10 = 適合性の凍結基準)
   │
   ▼
plan/ (Gate 0-7 → Phase T0-T2, I1-I6)   meta/ (規約・正本関係・agent 規約)
```

## 3 種の読み筋

- **理論筋**(体系を理解・拡張する): NORTH-STAR → adr → theory/00 → 01 → (関心の章) → 11 → scenarios。
- **実装筋**(toolchain を作る): spec/02..07 → architecture/03..04 → scenarios → plan/01 の該当 Phase。
- **運用筋**(進め方を知る): plan/00..03 → meta/agent-instructions → adr/ADR-0012 → adr/ADR-0014。

## ID 体系

| 接頭辞 | 意味 | 台帳 |
|---|---|---|
| ADR-#### | 決定 | adr/ |
| CON-### | 概念 | GLOSSARY.md |
| THM-### | 定理(statement) | theory/11 |
| OBL-### | 証明・検証義務 | theory/11 |
| SCN-## | 適合性シナリオ | scenarios/ |
| BND-### | 層間契約 | architecture/02 |
| GATE-# / PHASE-x | 計画単位 | plan/00, 01 |
| OPEN-### | 未決 | 各ファイル open_items と INDEX.json |
| WRK-#### | 可逆な research working proposition | working/ |
| E-XXXX-### | 診断 ID | spec/07 |

相互参照は必ず ID で行う。`INDEX.json` に全 id → path → status → depends_on の索引がある(`meta/build-index.py` で再生成)。

## Current working records

| ID | Status | Bounded question |
|---|---|---|
| WRK-0001 | L3-open | theory/02 の有限 index 許容範囲と helper-local Lean 正例・拒否例の再現。`working/WRK-0001-finite-index-boundaries.md` |
| WRK-0002 | L3-open | OBL-021 LAB statement draft の projection vacuity を既存 Lean lane の countermodel で検査。`working/WRK-0002-obl021-projection-vacuity.md` |
| WRK-0003 | L3-open | OBL-021 LAB statement draft の total/unique projection でも残る Result extensionality gap を既存 Lean lane の countermodel で検査。`working/WRK-0003-obl021-projection-extensionality.md` |
| WRK-0004 | L3-open | OBL-021 LAB statement draft が well-scoped input の outcome 存在を要求するかを既存 Lean lane の no-outcome countermodel で検査。`working/WRK-0004-obl021-outcome-totality.md` |
| WRK-0005 | L3-open | OBL-021 LAB statement draft と明示 outcome-totality 前提が実験用 SameOutcome relation を導くかを既存 Lean lane の conditional lemma で検査。`working/WRK-0005-obl021-conditional-outcome-relation.md` |
| WRK-0006 | L3-open | OBL-020 LAB statement draft の global preservation と familywise preservation の論理的接続を既存 Lean lane で検査。coverage は experiment-local 条件に留める。`working/WRK-0006-obl020-familywise-global-boundary.md` |
| WRK-0007 | L3-open | OBL-001 LAB statement draft が実験用 Result 内の write を `GeneratedWrite` で尽くすことを要求するかを検査。Core 表現・OBL status は選ばない。`working/WRK-0007-obl001-result-write-coverage.md` |
| WRK-0008 | L3-open | current-L2 runtime try/cut formal hook が same-Place の atomic_cut frontier を根拠として区別するかを既存レーンで監査。OBL-027、carrier、helper/schema は変更しない。`working/WRK-0008-obl027-formal-hook-attribution.md` |
| WRK-0009 | L3-open | current-L2 static e5 route と Lean proof-skeleton の review-unit / emitted-stub identity tuple が literal に整合するかを既存レーンで監査。theorem 意味、OBL、carrier、helper/schema は変更しない。`working/WRK-0009-current-l2-e5-skeleton-identity.md` |
| WRK-0010 | L3-open | current-L2 static-gate decision payload が static formal-hook artifact に literal または明示的 lossless reference として残るかを既存レーンで監査。診断意味、defect、carrier、helper/schema は変更しない。`working/WRK-0010-static-formal-hook-decision-attribution.md` |
| WRK-0011 | L3-open | current-L2 e21/e22 source route が exact final store を直接アサートするか、fixture/direct-evaluator lane に限定されるかを literal に監査。状態意味、同値性、defect、coverage 要求、carrier は選ばない。`working/WRK-0011-current-l2-final-store-directness.md` |
| WRK-0012 | L3-open, frozen | P-COMP-03 の固定一正例・一負例を既存 Product Alpha `world` package の direct carrier で検証した L3 record。二つの sidecar は観測されたが、番号付き結果 artifact の登録には許可外の validator/source-hierarchy 変更が必要となり凍結。helper、schema、runtime、CLI、public carrier、全 row coverage は変更・主張しない。`working/WRK-0012-pcomp03-direct-carrier.md` |
| WRK-0013 | L3-open, not-promoted | frozen WRK-0012 の二つの direct-world sidecar を入力としてのみ pin し、登録後の fresh execution と既存 unnumbered plan artifact 経路を独立に再現・保持した L3 record。sidecar、validator、helper、schema、runtime、CLI、public carrier、全 row coverage は変更・主張しない。`working/WRK-0013-pcomp03-retention-reproduction.md` |
| WRK-0014 | L3-open, not-promoted | Canon carrier を具体化せず、同一 carrier 上の relation inclusion が safety/coherence と outcome existence に必要とする向きを既存 Lean lane で検査する登録済み L3 record。OBL、Core/Config/Step/WellFormed、outcome representation、proof interface は変更しない。`working/WRK-0014-same-carrier-variance.md` |
| WRK-0016 | L3-open, frozen | exact current-L2 Lean foundation の二 constructor `Capability` に限り、任意 `CaptureSet` 間の `captureSubset` をソース可視トップレベルの非 instance `Decidable` 値として構成できるかを検査した L3 record。Lean の `theorem` は値をその形で保持できず、禁止済みの data-valued declaration が必要になるためこの route を凍結。構成的 undecidability、OBL-003、Line-1、generic carrier、global instance、checker、Canon carrier/API は変更・主張しない。`working/WRK-0016-local-predicate-constructivity.md` |
| WRK-0017 | L3-open, frozen | exact current-L2 Lean foundation の二 constructor `Capability` に限る proposition-valued `captureSubset` excluded-middle route。局所の一時 theorem は公理なしで通ったが、登録済み opaque generic-domain control は `Classical.choice` を伴って通ったため、明示 finite interface なしに generic decision を拒むという停止条件を満たし frozen。source は復元済みで、構成性、OBL-003、Line-1、checker、generic carrier、Canon carrier/APIは変更・主張しない。`working/WRK-0017-local-predicate-proposition-decidability.md` |
| WRK-0018 | L3-open, frozen | exact IFC Lean foundation 内の marked tail が最初にコンパイル失敗した registered falsifier で凍結した L3 record。後続の green toy tail は修正であり保持せず、source は登録時 digest に復元済み。telemetry semantics、label lattice、export ABI、THM-005/OBL、BND-008、grammar、SCN、runtime は変更・主張しない。`working/WRK-0018-thm005-telemetry-effect-boundary.md` |
| WRK-0019 | L3-open, not-promoted | P-COMP-03 arrays-bounds negative の一つの non-production direct-world sidecar は、既存 Product Alpha package route で固定 `MirCompute` / `OutOfBounds` を観測した L3 record。helper、schema、validator、runtime、CLI、public failure carrier、failure phase、Gate/Phase は変更・主張しない。`working/WRK-0019-pcomp03-bounds-direct-carrier.md` |
| WRK-0020 | L3-open, frozen | theory/01 Option、theory/06 chain、Surface grammar、Core companion、current-L2 e3 の option-local `admit` literal audit は、登録済み command の shell quoting falsifier で凍結。source consistency、Option/constraint/residual/other carrier、grammar、OBL、parser/runtime は結論しない。`working/WRK-0020-option-admit-carrier-literal-audit.md` |
| WRK-0021 | L3-open, frozen | active clean-near-end の三 counter CostBudget に限る scalar-total reflection audit は、登録後の Lean proof が comparison aliases の `Decidable` instance を合成できない first falsifier で凍結。countermodel、Contract cost rule、final algebra、runtime は結論しない。`working/WRK-0021-costbudget-scalar-projection.md` |
| WRK-0022 | L3-open, frozen | `[WRITE-CROSS]` の表示済み failure-row 包含節だけを対象にする finite Lean audit は、登録後の bare `lean` command が `samples` module prefix を解決できない first falsifier で凍結。finite premise result、生成関数、Canon derivation、failure-row equality、OBL は結論しない。`working/WRK-0022-write-cross-failure-generation-boundary.md` |
| WRK-0023 | L3-open, not-promoted | theory/04 の表示済み event-only Consistent(Kc) と `send -> receive` の literal Lean transcription は、既記載 consequence と既存 generic closure kernel の再現である。表示済み定義が channel-state branch を形式化・event membership と交換可能と確立していない境界だけを記録する。checkpoint carrier、checker、OBL は選ばない。`working/WRK-0023-consistent-cut-channel-state-boundary.md` |
| WRK-0024 | L3-open, not-promoted | SCN-02 の cross-locus read-dependent write において、owner-serial mutation だけでは二つの read reply 後の stale write を排除できないことを最小 non-production countermodel で検査する。snapshot、pending/request identity、Core rule、SCN、OBL は選ばない。`working/WRK-0024-scn02-read-write-snapshot-ambiguity.md` |
| WRK-0025 | L3-open, frozen | P004 A と P015 return exclusion 後も、表示済み Surface grammar の各 parse form が Core 又は明示的 Diagnostic へ分類されるかを literal inventory で検査する登録済み記録。登録 command の source-marker falsifier により凍結し、exact domain、grammar、Core、OBL は選ばない。`working/WRK-0025-surface-totality-domain-inventory.md` |
| WRK-0026 | L3-open, frozen | M1 request-local claims と既存 authority/history 文言が、同一 claims を持つ二つの request を replay と別個の正当要求に分類する semantic relation を既に供給するかを literal inventory で検査する登録済み記録。登録 command の source-marker falsifier により凍結し、request identity、replay policy、Core、runtime は選ばない。`working/WRK-0026-m1-replay-discrimination-inventory.md` |
| WRK-0027 | L3-open, not-promoted | SCN-08 の scalar `room_anchor` と terminal `default_pose` について、表示済み Surface/Core/static source が明示的な宣言・解決対応を既に供給するかを literal comparison で検査する。scalar representation、grammar、Core、fallback policy は選ばない。`working/WRK-0027-scn08-scalar-terminal-correspondence.md` |
| WRK-0028 | L3-open, not-promoted | Plan 200 の C0/C2 pre-enumerated source span を current Canon cut で literal に転記し、current wording と bounded proposal direction を source 自身の authority language だけから区別して保持した。意味論の合成・選択はしない。`working/WRK-0028-r0-common-cut-fact-manifest.md` |
| WRK-0029 | L3-open, not-promoted | Plan 200 C0-B の lexical/parse/surface-static/`WellScoped` input role を opaque node とする conditional dependency graph を retained した。条件付き有限 DAG 以外は結論せず、domain、`WellScoped`、outcome、Diagnostic、Core は選ばない。`working/WRK-0029-c0b-noncircular-domain-staging.md` |
| WRK-0030 | L3-open, not-promoted | Plan 200 C2-A の request/authority/occurrence/replay wording を WRK-local question label で source-tag し、label 間の documentary non-substitution だけを retained した。semantic vocabulary、field partition、identity、binding、attempt、replay classifier は選ばない。`working/WRK-0030-c2a-source-tagged-anti-collapse-vocabulary.md` |
| WRK-0031 | L3-open, not-promoted | Plan 200 C0-C の pre-enumerated source span に literal terminal/reject/`Diagnostic` wording 又は明示 cross-reference があるかを source-local に検査し、source-local query record だけを retained した。stage、coverage、reject domain、Diagnostic relation、totality は選ばない。`working/WRK-0031-c0c-source-local-diagnostic-reference-audit.md` |
| WRK-0032 | L3-open, not-promoted | Plan 201 C5-PRE の pre-enumerated ordinary-admission source span に、verdict から独立した issuance phase を示す literal wording があるかを source-local に記録した。P012 guard direction と four named theory/spec span の non-match だけを retained し、A2 atomicity/facet/compatibility、occurrence identity、Core/history/runtime は選ばない。`working/WRK-0032-c5pre-ordinary-admission-issuance-guard.md` |
| WRK-0033 | L3-open, not-promoted | Plan 202 の V1/R1 administrative binding と one-slot machine presentation を opaque LAB correlation と明示 matching/single-use/failure assumptions の下で比較した有限 conditional lemma と三つの adverse distinction を retained。Mir pending/request/occurrence identity、Core/history/runtime は定義・選択しない。`working/WRK-0033-v1r1-presentation-refinement.md` |
| WRK-0034 | L3-open, not-promoted | Plan 203 の WRK-0033 fixed presentation を変えず、opaque LAB reply の arbitrary finite list に対する translation-preservation と local-observation equality を conditional lemma として retained。Mir trace、pending/request/occurrence identity、Core/history/runtime、source inference は定義・選択しない。`working/WRK-0034-v1r1-finite-sequence-refinement.md` |
| WRK-0035 | L3-open, not-promoted | C7 の concrete source rule を選ばず、local parametric `erase`/`observe` に対する fiber constancy と range 上の pointwise unique observation を constructive conditional lemma として retained。explicit collision は両 predicate を refute し、fixed full-codomain countermodel は global reconstruction を否定する。choice、quotient、Mir carrier、source inference は定義・選択しない。`working/WRK-0035-c7-parametric-factorization.md` |
| WRK-0036 | L3-open, not-promoted | C7 の concrete source rule を選ばず、individually fiber-constant な二つの local erasure とその common coarsening が paired observation を失う固定有限 countermodel を retained した。個別 check を同時 omission の許可へ合成しない negative guard に限り、choice、quotient、Mir carrier、source inference は定義・選択しない。`working/WRK-0036-c7-cumulative-erasure-countermodel.md` |
| WRK-0037 | L3-open, not-promoted | C2-B/C3 の Canon carrier を選ばず、equal-incidental な二つの opaque request atom と direct staged projection/injective restore を持つ fixed finite B-primary experiment を pre-register した。Core/Config/history/SaveObject、identity/equality、authority、source rule、runtime は定義・選択しない。`working/WRK-0037-c2b-c3-b-primary-opaque-anchor.md` |
| WRK-0038 | L3-open, not-promoted, unexecuted | bare DirectView が supplied key を保持しない scope review の後、実験せず WRK-0039 へ前方 supersede した。元の pre-registration は保持し、carrier、identity、authority、persistence、source rule、runtime は定義・選択しない。`working/WRK-0038-c2b-c3-bundled-relational-presentation.md` |
| WRK-0039 | L3-open, not-promoted | WRK-0037 の全十 supplied key ごとの independently enumerated relation graph と bundled lookup の fiberwise comparison を finite L3 evidence として retained。key recovery、identity、authority、persistence、source rule、runtime は定義・選択しない。`working/WRK-0039-c2b-c3-fiberwise-relational-presentation.md` |
| WRK-0040 | L3-open, not-promoted | P017 X1 の V1/R1 cross-locus read に限り、二つの supplied occurrence / restore witness を用いる predicate-only finite countermodel が `SEP`、`PHASE`、`ONE`、`AUTH`、`OBS` の五 collapse を検出できるかを検査した。有限 fixture の detector table は通ったが、Core、relation schema、identity、transition、restore、runtime は定義・選択しない。`working/WRK-0040-p017-x1-coupled-anti-collapse-countermodel.md` |
| WRK-0041 | L3-open, not-promoted | P017 X1 の V1/R1 cross-locus read に限り、supplied fixture の owner-terminal positive / negative fact が同時に成立する collapse を predicate-only finite countermodel で検出した。四 fixture の detector table は通ったが、failure row、transition、carrier、runtime は定義・選択しない。`working/WRK-0041-p017-x1-owner-terminal-exclusivity-countermodel.md` |

## 「ここから推論してはいけないこと」(canon 全体)

- 図・例・シナリオの語彙(`World`, `Player` 等)は S5 のドメイン語彙であり、core primitive ではない(ADR-0001)。
- ファイルが存在すること自体は、その内容が実装済みであることを意味しない。実装状態は plan/01 のみが語る。
- LAB(旧 repo)の記述は evidence であり規範ではない(ADR-0012)。
- `working/` の記録は settled theory ではない。既存の theory/spec/scenario/plan/ledger を変更せず、ADR-0014 の範囲だけを示す。
