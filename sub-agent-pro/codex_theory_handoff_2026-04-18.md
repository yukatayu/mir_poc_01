# Codex handoff — theory closeout / principal direction / FAQ08-complete integrated brief (2026-04-18)

## 0. この文書の位置づけ

この文書は、現在の repo に関する **CodeX 向けの統合 handoff** である。  
目的は次の 3 点。

1. ここまでの議論、`faq_006.md` / `faq_007.md` / `faq_008.md`、`specs/`、`plan/`、current samples/prototypes から読める **現在地** を一つにまとめる。
2. 二大問題（Problem 1: 型 / 定理証明 / モデル検査境界、Problem 2: order / handoff / `memory_order` 再構成）について、**これ以上 compare-floor を増殖させず、actual adoption package を閉じるための principal direction** を明示する。
3. そのうえで、CodeX が `specs/` / `plan/` / `progress.md` / `tasks.md` / reports を更新しながら、**self-driven theory line を near-end まで自走**できるようにする。

この文書は **規範判断の正本ではない**。  
常に次の優先順位を守ること。

1. `specs/` = 規範正本
2. `plan/` = repository memory
3. `docs/reports/` = change rationale / historical trace
4. `progress.md` / `tasks.md` / `faq_*.md` = current explanation / snapshot

この文書と `specs/` が衝突したら、**必ず `specs/` を優先**すること。  
ただし、この文書は current reading を更新するための **explicit direction / defaults / package design** を含む。  
source-backed に反映可能なものは、relevant `specs/` / `plan/` / reports に戻すこと。

---

## 1. 一番短い現在地の読み

### 1.1 結論

- repo はもう docs-only abstract research ではない。
- parser-free current L2、compile-ready minimal actualization、fixed-subset source sample、corrected prototype、compare floor、actual adoption package、helper-local actualization / narrowing floor まである。
- 二大問題の **current first line** は source-backed に選ばれ、actual adoption package と helper-local actualization floor まで machine-check 済み。
- しかし、**二大問題の final adoption** と **final public language / tool / app completion** はまだ終わっていない。
- 現在地は  
  **near-end theory closeout の後半、reserve strengthening / second-line / mixed gate の入口**  
  と読むのが最も誤読が少ない。
- 研究の主たる debt は discovery debt ではなく **selection/adoption debt** である。
- したがって、ここから先の主眼は「新しい理論候補を増やすこと」ではなく、**principal spine を固定し、compare-floor を stop line で止め、actual adoption package を閉じること**である。

### 1.2 FAQ 08 の核心を一文で言うと

**「二大問題の current first line は source-backed に選ばれ、actual adoption package と helper-local actualization floor まで machine-check 済みだが、final public language / tool / app completion はまだ mixed gate と user-spec gate に残っている。」**

---

## 2. current repo の規範境界（絶対に崩さないもの）

### 2.1 全体アーキテクチャ

- **Mir** が主眼の意味論コア。
- **Mirrorea** は distributed control / routing / audit / overlay / patch の fabric。
- **PrismCascade** は独立 kernel。Mir runtime に早期統合しない。
- **Typed-Effects Wiring Platform** は lower/adjacent operational layer。Mir そのものではない。
- shared space / VRSNS / Reversed Library は上位 layer / application track。

### 2.2 Mir-0 / Mir-1 / Mirrorea の切り分け

Mir-0 に残すもの:
- event DAG
- `place`
- minimal effect request operation (`perform` は説明用 token)
- effect / contract
- minimal structured failure space
- primitive fallback
- local `try` / rollback
- place-local `atomic_cut`
- linear resource

Mir-0 に入れないもの:
- `barrier`
- `durable_cut`
- full fallback normalization
- distributed scheduler
- rich coroutine / emit / overlay detail

Mirrorea に残すもの:
- logical naming
- route rebinding
- overlay registration
- patch activation
- audit / path proof
- distributed realization

### 2.3 `atomic_cut` の current meaning

`atomic_cut` は **place-local finalizing cut nucleus**。  
これは次を意味しない。

- global consistent cut
- process-wide synchronization point
- distributed agreement point
- durable commit

`atomic_cut` は current `place` の rollback frontier を確定するだけであり、以後の higher-level ordering / witness / fairness / scheduler / commit family を背負わせない。

### 2.4 fallback / `lease` の current meaning

- fallback = guarded option chain
- same-lineage chain = left-to-right monotone degradation
- no re-promotion
- `lease` = option-local lifetime guard
- `lease-expired` は option-local miss / non-admissible metadata であり request-level `Reject` そのものではない
- request-level `Reject` は well-formed chain が admissible success を返せず尽きたときの final outcome
- `try` / rollback / `atomic_cut` は degradation order を巻き戻さない

### 2.5 proof / checker boundary の current meaning

current principal split:
- `core_static_checker`
- `theorem_prover_boundary`
- `protocol_verifier_boundary`
- `runtime_policy_boundary`

local / structural / decidable fragment は checker へ。  
global law / fairness / protocol / richer property は theorem/model-check/runtime policy へ。

### 2.6 syntax の current meaning

- current L2 notation は **companion notation**
- final parser grammar ではない
- explicit edge-row family を current polished first choice とする
- compactness より **semantic honesty** を優先
- final grammar / final public parser API / final keyword set はまだ OPEN

---

## 3. FAQ 08 への完全統合回答

この節は `faq_008.md` の内容を **ここでの handoff 用に完全統合したもの** である。

### 3.1 何が終わっているか

#### floor 読み

現在の floor を誤読なく言い換えると次。

| floor | 現状 | 読み |
|---|---|---|
| compare floor | close | current first line / stop line / mixed gate を compare 可能な形で固定済み |
| actual adoption floor | close | 二大問題と syntax/modality の current recommendation は docs-first actual package に上がっている |
| helper-local actualization / narrowing floor | close on current tranche | theorem-first pilot、room default vertical slice、minimal companion、stage-block secondary、theorem-preflight を helper-local に actualize 済み |
| reserve strengthening floor | active | `auditable_authority_witness`、`delegated_rng_service`、model-check second line が current queue |
| final public language / tool / app floor | not close | parser / public API / final verifier contract / concrete external tool / packaging / host / broader app は未 close |

#### current runnable / machine-check evidence

少なくとも次は current state で runnable / machine-check の evidence を持つ。

- current authored sixteen  
  `e1 / e2 / e3 / e4 / e5 / e12 / e14 / e15 / e16 / e13 / e19 / e21 / e22 / e18 / e20 / e23`
- corrected prototype octet  
  `p01 / p02 / p03 / p04 / p05 / p06 / p07 / p08`
- compare floor
  - verifier preview alignment pre-floor
  - model-check projection pre-floor
  - theorem discharge pre-floor
- helper-local actualization / narrowing floor
  - theorem-first experimental pilot actualization
  - theorem-prover experimental binding preflight
  - authoritative-room vertical-slice actualization
  - minimal companion / experimental order-handoff surface
  - stage-block secondary order-handoff surface

#### 2026-04-18 fresh validation

少なくとも current explanation では次が fresh pass と読まれている。

- `cargo test -p mir-runtime --test current_l2_source_sample_runner` → `22 passed`
- `cargo test -p mir-runtime --test current_l2_operational_cli` → `12 passed`
- `cargo test -p mir-runtime --test current_l2_verifier_preview_alignment` → `5 passed`
- `cargo test -p mir-runtime --test current_l2_model_check_projection_prefloor` → `5 passed`
- `cargo test -p mir-runtime --test current_l2_theorem_discharge_prefloor` → `5 passed`
- `cargo test -p mir-runtime --test current_l2_theorem_first_pilot_actualization` → `5 passed`
- `cargo test -p mir-runtime --test current_l2_theorem_prover_binding_preflight` → `4 passed`
- `cargo test -p mir-runtime --test current_l2_source_sample_emitted_artifact_wiring` → `9 passed`
- `cargo test -p mir-runtime --test current_l2_authoritative_room_vertical_slice_actualization` → `3 passed`
- `cargo test -p mir-runtime --test current_l2_order_handoff_minimal_companion_surface` → `3 passed`
- `cargo test -p mir-runtime --test current_l2_order_handoff_stage_block_surface` → `3 passed`
- `python3 scripts/current_l2_source_sample_regression.py inventory` → authored sixteen present
- `python3 scripts/current_l2_source_sample_regression.py regression` → all regression commands passed

要するに、「実装・実行による比較検証がまだ薄い」という読みはもう正しくない。

### 3.2 何がまだ終わっていないか

#### Problem 1 で未完了

source-backed に close 済み:
- verifier-boundary matrix current first line
- checker-adjacent semantic carrier principal
- source-visible structural marker family first
- notebook-first theorem line
- row-local model-check carrier first
- `p06` bridge-floor evidence
- verifier preview alignment pre-floor
- model-check projection pre-floor
- theorem discharge pre-floor
- theorem-first pilot actualization
- theorem-preflight

未完了:
- stronger typed surface actual adoption
- theorem discharge transport / public-contract actual format
- concrete theorem prover brand / binding
- settled property language
- concrete model-check tool seam
- final public verifier contract
- proof object public schema

つまり、「Problem 1 の current recommendation はある」が、「final public theorem/model-check language/tool contract はまだない」。

#### Problem 2 で未完了

source-backed に close 済み:
- cut family decomposition
- relation decomposition principal
- `authority_serial_transition_family` first
- `witness_aware_commit_family` second
- thread/node parity current safer wording
- low-level `std::memory_order` / `std::kill_dependency` retained-later reference line
- `p07/p08` bridge-floor evidence
- authoritative room first actual default
- vertical-slice actualization
- minimal companion / stage-block floor

未完了:
- final source-surface handoff wording
- final emitted-artifact schema / handoff contract
- exhaustive shared-space final catalog
- stronger fairness / replay operational theorem or profile
- low-level family public stance の最終 cut

つまり、「Problem 2 の decomposition と default profile はある」が、「final public wording と final public handoff contract はまだない」。

#### 言語側実装として未完了

もし「言語側の実装まで終わっている」と言うなら、少なくとも以下が必要だが、まだ無い。

- final parser grammar
- final public parser API
- final public checker API
- final public runtime API
- final public verifier contract
- final source-surface order/handoff wording
- stronger typed surface final disposition
- concrete theorem/model-check production bindings
- packaging / installed binary / host integration completion

### 3.3 overall ladder で今どこか

current best reading:
1. candidate comparison / drift audit → close
2. current first line の actual adoption package 化 → close
3. helper-local actualization / narrowing → current tranche close
4. reserve strengthening / second-line concretization → active
5. final public seams の mixed-gate actual adoption → 未 close
6. final public language / toolchain integration → 未 close
7. broader host / packaging / application realization → user-spec dependent で未 close

macro phase 言い換え:
- execution lane: `Macro 4 active on fixed authored/prototype floor`
- theory-lab lane: `Macro 5 near-end actual-adoption closeout`
- reserve integration lane: `Macro 6 minimal working subset actual default / Macro 7 mixed`

### 3.4 何を答えればさらに長く自走できるか

`faq_008` が一番効くと整理している問いは次。

true user-spec / target side:
1. first completion に packaging / host integration を含めるか
2. authoritative-room first scenario を first completion の acceptance と見なしてよいか
3. broader application target をすぐ要求するか、後段に送ってよいか
4. theorem-first concrete binding を早めに actualize してよいか
5. model-check second line を theorem-first close と同 tranche に置くか
6. final public parser/checker/runtime API をこの closeout の必須条件にするか

現在の解釈:
- これらに答えればかなり先まで自走しやすくなる
- しかし remaining mixed gate は evidence-dependent なので、「これだけで最後まで完全 no-question」はまだ無理

---

## 4. ここまでの議論を踏まえた principal direction（この文書の主張）

この節は、ここまでの対話を踏まえて **assistant 側が責任を持って勧める主軸方針** である。  
以後、CodeX はこれを **current default direction** として使ってよい。  
ただし、`specs/` の settled judgment と正面衝突する場合は report で conflict を明示すること。

### 4.1 原則

- 候補理論を横並びで「全部そのまま融合」してはいけない。
- 一つの **judgmental spine** を principal にし、その上に effect / refinement / IFC / theorem/model-check boundary を attachment layer として載せる。
- 多段階計算は core 寄りである。plugin 的扱いにしない。
- information-flow / security / richer contract は重要だが、first checker cut に全部押し込まない。
- final grammar は theory より後。
- final public implementation は theory closeout と別 lane。
- mainline actualization と theory-lab closeout を分ける。

### 4.2 principal spine

**principal spine = multimodal dependent core**

理由:
- `λ◯□` は stage/later/always の partial basis として非常に有力
- しかし full language は
  - stage
  - place
  - visibility/publication
  - witness/audit
  - durability/cut
  - authority/handoff
  と多軸
- したがって full basis は `λ◯□` 単独では不足し、multimodal dependent type theory 系がより自然

#### current recommendation

- `λ◯□` = partial basis candidate
- guarded lambda-calculus = strong candidate for later/guarded/coinductive line
- Modal Dependent Type Theory = strong candidate for dependent modal core
- Multimodal Dependent Type Theory = strongest current candidate for full spine
- Fitch-style multimodal basis = keep as formulation family

### 4.3 layered theory stack

私が推奨する理論 stack は次。

1. **Multimodal dependent core**
   - stage
   - place
   - stable/later-like structure
   - mode-indexed typing judgment

2. **Linear / affine capability layer**
   - ownership
   - move / borrow / delegation constraints
   - capability monotonicity
   - no hidden duplication

3. **Row-polymorphic effect layer**
   - `perform`
   - publication / observation / handoff / emit / cut 等の effect family
   - effect inference / polymorphism

4. **Decidable refinement / contract layer**
   - `require` / `ensure`
   - option-local admissibility
   - checker-friendly decidable fragment
   - refinement obligations

5. **Information-flow / security-label layer**
   - user-defined label lattice / principal model
   - `Labeled ℓ A`
   - explicit declassification / endorsement
   - compile-time info-flow checks

6. **Theorem / model-check bridge**
   - residual proof obligations
   - protocol properties
   - fairness/replay/global law
   - machine-facing or notebook-facing artifacts

### 4.4 principal judgment sketch

Use this as the conceptual reference judgment:

```text
Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ O
```

Where:

- `Ψ` = mode theory / world assumptions
- `Γ` = unrestricted + dependent context
- `Δ` = linear / ownership context
- `m` = mode (at least stage/place; later may include visibility/witness-oriented refinements)
- `ε` = effect row
- `O` = residual obligations not discharged by the core checker

Interpretation:
- `Γ/Δ` split handles linearity and ownership.
- `m` handles stage/place/modal positioning.
- `ε` handles effect families.
- `O` is the hook to theorem/model-check/runtime-policy boundaries.

This exactly matches the current repo’s intuition that there is a **checker floor** and then **residual proof obligations**.

---

## 5. metatheory design: layer compatibility / compilation / composition

あなたが指摘した通り、局所的に異なる理論層を入れ、後で連携させるなら、**それらの互換性を示すメタ理論** が必要。  
ここは必ず整備すること。

### 5.1 基本構図

各 module / component / source region は、必要な theory layer を持ってよい。  
だが compilation / linking / external boundary では、以下を明示する。

- what modal worlds / modes the module assumes
- what effects it may emit
- what obligations it discharges itself
- what obligations it exports
- what security / label policy it assumes
- what handoff/publication/witness guarantees it requires/provides

### 5.2 必要な compatibility theorems

最低限、次の meta-theory packages が必要。

1. **elaboration soundness**
   - richer source forms / syntax sugar / typed markers / contracts / IFC annotations が core IR へ落ちても typing と semantics が保存される

2. **module interface well-formedness**
   - imported/exported effect rows, label constraints, ownership/capability assumptions, modal assumptions が interface で一致する

3. **cross-layer preservation**
   - capability layer / effect layer / refinement layer / IFC layer を stacked しても preservation が壊れない

4. **obligation export soundness**
   - module A で未解決として輸出した obligation を module boundary で明示できる
   - module B / theorem / model-check side がそれを正しく引き受ける

5. **boundary checking soundness**
   - external boundary checks が source-level assumptions を破らない
   - host / tool / artifact seams で hidden strengthening が起きない

### 5.3 実装上の基本方針

- first implementation では monolithic full calculus を作らない
- core calculus + elaborator + module manifest/interface + obligation artifacts で進める
- richer layers は syntax に直結するより पहले IR / metadata / manifest / artifacts に寄せる
- module-local principal theory と cross-module surface を混ぜない

---

## 6. Problem 1 — 型 / 定理証明 / モデル検査

### 6.1 current reading

現在 close 済み:
- checker-adjacent principal
- source-visible structural marker family first
- notebook-first theorem line
- row-local model-check carrier first
- `p06` bridge-floor evidence
- verifier preview / theorem discharge pre-floor / model-check projection pre-floor

まだ mixed gate:
- stronger typed surface actual adoption
- theorem discharge transport/public-contract actual format
- settled property language
- concrete theorem prover binding
- concrete model-check seam
- final public verifier contract
- proof object public schema

### 6.2 principal direction for Problem 1

#### principal recommendation

- **first checker cut** は local / structural / decidable fragment に限定する
- stronger typed surface は early source principal には上げない
- theorem-first external integration target を採る
- notebook-first theorem line + symbolic refs + preflight manifest を current principal bridge とする
- row-local model-check carrier を second-line concretization target にする
- theorem/model-check public contract は final adoption まで上げない

#### why

- current repo has bridge-floor evidence (`p06`, previews, theorem preflight)
- but no final public theorem/model-check contract
- premature stronger surface adoption would freeze the calculus too early
- theorem-first line gives the best near-end closeout ratio

### 6.3 static verification architecture

#### checker (must be low-cost / natural)

Keep in checker:
- same-lineage static floor
- missing option structure floor
- capability strengthening prohibition
- `try` / `atomic_cut` structural floor
- local contract shape
- minimal predicate fragment
- module interface consistency
- straightforward info-flow / label-flow fragment
- linear/capability floor

#### theorem side

Send to theorem:
- canonical normalization law (general form)
- no re-promotion global law
- rollback-cut non-interference
- stronger typing theorems
- module composition theorems
- IFC richer noninterference / declassification soundness
- modal coherence laws
- elaboration correctness in stronger formulations

#### model-check side

Send to model-check:
- bounded protocol properties
- replay / stale reconnect / late join scenarios
- fairness policy comparisons
- room-profile dynamics
- bounded handoff obligations
- specific order/handoff scenario checks

#### runtime policy

Leave in runtime policy:
- actual transport / retry / resend / timeout / backoff
- host/tool integration policy
- deployment-specific fairness / availability knobs

### 6.4 strong typing recommendation

You explicitly want strong typing. Current recommendation:

- yes to strong typing
- no to “put everything into one first-core huge dependent calculus”

Use:
- linear / capability typing in the core
- effect rows in the core+inference layer
- decidable refinement for first checker cut
- IFC labels / user-defined lattice in the first checker cut
- richer dependent proofs / theorem side for the rest

### 6.5 IFC / security-label plan

This should be a first-class package, not a vague future wish.

#### principal design

Introduce a user-definable label model:

```text
LabelModel = {
  Label       : Type
  flows_to    : Label -> Label -> Prop
  join        : Label -> Label -> Label
  meet        : Label -> Label -> Label
  flows_dec   : Decidable (flows_to a b)
  laws        : preorder / semilattice laws
}
```

Then allow types like:

```text
Labeled ℓ A
```

or a judgmental annotation style:

```text
Γ ; Δ ⊢ e : A @ m ! ε ▷ O [pc = ℓ]
```

#### first checker obligations

Must statically check:
- explicit data flow respects `flows_to`
- pc-flow respected
- move / share of labeled capabilities respects policy
- declassification / endorsement require explicit authority capability
- secret-key-like capabilities do not escape to broader labels without explicit authority

#### theorem side obligations

Keep for theorem / richer proof:
- covert channel considerations
- protocol-level information release through replay/fairness behavior
- stronger noninterference theorems
- distributed trust reasoning

### 6.6 what to prove for Problem 1

#### core metatheory
- preservation
- progress (with explicit external-effect/open-world refinement)
- linearity / no duplication
- label preservation
- effect soundness
- elaboration soundness

#### algorithmic checker results
- soundness of checker for fragment
- completeness for the decidable fragment only
- decidability / inference results for current fragment

#### bridge results
- sound obligation extraction
- sound projection to theorem/model-check carriers
- conservative/bounded abstraction soundness for model-check line

### 6.7 current stop line for Problem 1

Near-end closeout for Problem 1 is reached when:
- first checker fragment is explicitly frozen
- theorem-first concrete binding actualization package exists
- stronger typed surface is either:
  - explicitly retained as non-principal experimental surface, or
  - explicitly adopted with evidence
- model-check second line reaches first settled seam
- remaining work is final public contract / public API / production tool binding only

---

## 7. Problem 2 — order / handoff / `memory_order` 再構成

### 7.1 current reading

close 済み:
- cut family decomposition
- relation decomposition principal
- `authority_serial_transition_family` first
- `witness_aware_commit_family` second
- thread/node parity safer wording
- low-level `std::memory_order` / `std::kill_dependency` retained-later reference line
- `p07/p08` evidence
- authoritative room first actual default
- vertical-slice actualization
- minimal companion / stage-block floor

still mixed:
- final source wording
- final emitted-artifact schema / handoff contract
- fairness / replay final operational profile
- exhaustive shared-space catalog
- low-level family final public stance

### 7.2 principal direction for Problem 2

#### principal recommendation

- principal semantics = **relation decomposition**
- principal operational family = **authority-serial transition**
- second strengthening family = **witness-aware commit/handoff**
- low-level `memory_order` family = **backend-aligned retained reference family**
- thread/node parity wording = **same causal language; different lowering/evidence/policy**

### 7.3 cut family

Keep separate:
- `atomic_cut`
- `barrier`
- `durable_cut`
- optional comparison candidate: `snapshot_cut` / `consistent_cut`

Interpretation:
- `atomic_cut` = local rollback-stop / local finalization nucleus
- `barrier` = ordering-only candidate
- `durable_cut` = commit/evidence-bearing family
- `snapshot_cut` = observation/snapshot-only candidate (comparison only unless source-backed later)

Do **not** collapse these.

### 7.4 relation family

The conceptual order family should explicitly include:

- `program_order`
- `dependency_order`
- `publication_order`
- `observation_order`
- `witness_order`
- `finalization_order`
- `scoped_happens_before`

This lets source language stay high-level while backend/reference lines preserve low-level orderings.

### 7.5 thread/node parity

Use the safer wording:

> thread と node は同じ causal language で書く。  
> 差は lowering / evidence / transport / failure / durability / policy に残す。

Do **not** write `thread == node`.

### 7.6 authority / handoff structure

The structure that should become principal is:

- owner slot
- transition stage family
- stage sequence
- stage-local obligation
- handoff epoch ref
- witness-aware handoff
- replay attachment
- payload ref

This is already aligned with the retained theorem-line bridge docs around `authority_transition_stage_family`, `stage_local_obligation_family`, witness-aware handoff, replay attachment, payload refs, etc.

### 7.7 fairness / replay first default

Use the first default profile:

- activation = `authority-ack`
- authority placement = `single room authority`
- consistency mode = `authoritative serial transition`
- RNG = `authority_rng`
- late join = visible history as past
- stale reconnect = fail then refresh
- replay = stale/incompatible replay invalidated, not silently merged
- no distributed fairness theorem required in first completion line

Reserve strengthening:
- `auditable_authority_witness`
- `delegated_rng_service`
- later maybe broader fairness families

### 7.8 what to prove for Problem 2

#### core semantic results
- cut separation soundness
- no hidden rollback across `atomic_cut`
- relation family coherence
- authority-serial transition soundness
- witness requirement preservation

#### checker fragment
- structural stage sequence well-formedness
- required witness presence
- owner-slot / authority-scope structural floor
- local order/handoff static obligations

#### theorem side
- handoff requires prior publication law
- witness-aware handoff correctness
- no invalid replay across epochs/incarnations
- stronger fairness/replay theorems (optional later)

#### model-check side
- late join / visible history
- stale reconnect fail-then-refresh
- race/replay bounded scenarios
- authority-handoff protocol bounded safety

### 7.9 stop line for Problem 2

Near-end closeout for Problem 2 is reached when:
- relation decomposition is fixed as principal
- source wording narrows to one principal family
- `p07/p08` plus authoritative-room vertical slice are fully integrated in docs/reports/tests
- first fairness/replay operational default is written into docs
- remaining work is final public wording / broader catalog / low-level public stance only

---

## 8. syntax / semantics / modality

### 8.1 core principle

**theory first, syntax second**

Do not let existing languages dictate the surface.  
Do not let parser polish freeze the semantics early.

### 8.2 syntax evaluation axes

Every syntax candidate must be evaluated on:

1. semantic honesty
2. checker legibility
3. modal adequacy
4. misreading resistance
5. lowering friendliness

### 8.3 principal syntax family

#### principal
**explicit edge-row / vertical continuation**

Example:

```text
roll draw via authority_rng
publish draw
handoff dice_owner A -> B
  after publish(draw)
  requires witness(draw)
```

Why principal:
- edge-local info stays edge-local
- static checking is syntax-directed
- aligns with explicit edge-row tradition already chosen for fallback
- avoids packed rows

#### strong secondary
**stage / after / witness stage-block**

Example:

```text
transition handoff_turn(owner = A) {
  stage roll:
    draw <- perform via authority_rng

  stage publish:
    publish draw

  stage handoff:
    change_owner dice_owner to B
      requires witness(draw)
}
```

Why secondary:
- readable
- good for explanation and stage structure
- closer to `λ◯□`-like intuition
- but slightly more block-heavy and may tempt over-structuring

#### reserve sugar
**authoritative-room serial sugar**

Example:

```text
serial on dice_owner {
  draw <- perform via authority_rng
  publish draw
  handoff dice_owner A -> B
    requires witness(draw)
}
```

Why reserve:
- probably good for authoritative room ergonomics
- but too high-level to be principal core surface right now

### 8.4 modal foundation recommendation

- `λ◯□` = partial basis candidate for stage/later/always
- guarded λ-calculus = keep strong candidate for guarded recursion / coinductive traces
- MDTT = strong candidate for dependent modal core
- MTT = strongest current candidate for full multimodal spine
- final calculus adoption = **not in this tranche**

### 8.5 what to prove here

- elaboration from chosen companion/source form to core IR preserves typing and semantics
- syntax sugar desugaring is coherent
- modal annotations / source markers elaborate soundly
- companion forms do not introduce stronger semantics than the core

---

## 9. proof plan and formalization strategy

### 9.1 Lean-first recommendation

Use **Lean-first** for:
- core metatheory
- typing judgments
- elaboration correctness
- checker fragment correctness
- modal/dependent core experiments
- obligation extraction proofs

Use **Rocq/Coq + Iris fallback** only if/when:
- you need very strong concurrent separation logic for runtime internals
- you need an Iris-style existing ecosystem

Do not make Isabelle the principal route for this repo line.

### 9.2 staged proof roadmap

#### Stage A — paper spec / mechanization-ready core
- define core syntax / semantics / typing in prose + pseudocode
- define judgmental spine
- define module/interface and obligation export shapes

#### Stage B — Lean core
- preservation
- progress (open-world/external-effect aware)
- linearity / ownership soundness
- effect soundness
- elaboration soundness

#### Stage C — checker fragment
- algorithmic checker soundness
- decidability proofs where needed
- limited completeness for chosen fragment

#### Stage D — obligation/bridge layer
- sound extraction to theorem/model-check obligations
- row-local carrier/projection soundness
- bounded model-check abstraction soundness

#### Stage E — Rust/Lean alignment
- differential testing
- property-based testing
- artifact conformance
- trace alignment on representative corpus

### 9.3 what does NOT need proof before implementation

Do **not** block implementation on:
- full final calculus adoption
- full final public parser/runtime API proof
- complete global fairness theorems
- exhaustive shared-space catalog proofs

Instead:
- implement against the current principal spine
- prove the core
- use runner/prototypes/tests as evidence
- leave mixed/public/application gates explicit

---

## 10. completion target defaults (assistant recommendation)

These are the defaults I recommend using unless a strong source-backed conflict appears.

### 10.1 first completion definition

**first completion = authoritative-room first scenario**

Include:
- repo-local CLI + tests + artifacts + compare floor
- authoritative-room vertical slice
- theorem-first concrete binding actualization package
- first settled order/handoff source family
- current principal theorem/model-check bridge

Do **not** require:
- installed binary / packaging
- FFI / engine adapter
- broad host integration
- exhaustive shared-space catalog
- final public parser/checker/runtime API

### 10.2 first application target

- authoritative-room first scenario = yes
- append-friendly contrast room = yes, but as contrast not as equally heavy first target
- broader app target = later
- theorem-first integration target = yes
- model-check second line = after theorem-first close or as second strengthening package

### 10.3 problem-specific defaults

#### Problem 1 defaults
- stronger typed surface stays non-principal for now
- checker-adjacent + structural markers first
- theorem-first actualization yes
- notebook-first line yes
- final public verifier contract not required for first completion

#### Problem 2 defaults
- relation decomposition principal
- authority-serial first
- witness-aware second
- edge-row principal source family
- stage-block strong secondary
- serial sugar reserve
- low-level memory-order family retained-later reference only

---

## 11. what Codex should do next

This is the **actual execution program**.

### Package 0 — drift audit / queue reconstruction
Must do:
- reconcile `faq_006` / `faq_007` / `faq_008` with `specs/`, `plan/`, `tasks.md`, `progress.md`
- eliminate queue drift
- reflect current self-driven queue explicitly
- ensure `current self-driven queue != 0`

Expected outputs:
- updated `tasks.md`
- updated `progress.md`
- audit report
- `plan/01`, `plan/11`, `plan/17` sync if needed

### Package 1 — Problem 1 actual adoption closeout
Must do:
- freeze current first line wording for Problem 1
- update `specs/10`, `specs/11`, `specs/12` and relevant `specs/examples/*`
- integrate `p06`
- decide whether stronger typed surface remains experimental or gets a narrow adoption package
- move theorem-first line from preflight to actual experimental binding package
- keep final public verifier contract open, but make stop line explicit

Expected outputs:
- a current adoption note
- a theorem-first actualization package
- updated matrix of proof/checker/model-check responsibilities
- reports and docs alignment

### Package 2 — Problem 2 actual adoption closeout
Must do:
- freeze current first line wording for Problem 2
- integrate `p07`, `p08`, authoritative-room vertical slice
- write the principal source family as recommendation
- keep low-level memory-order family as retained reference mapping note
- define first fairness/replay operational default explicitly
- update shared-space docs and relevant examples

Expected outputs:
- cut family note
- relation family note
- authority/handoff note
- source wording recommendation
- updated shared-space defaults
- reports/tests/docs alignment

### Package 3 — syntax/modality convergence
Must do:
- write syntax-semantics principle explicitly
- record principal / secondary / reserve syntax families
- record modality recommendation:
  - `λ◯□` partial basis
  - guarded / MDTT / MTT / Fitch-style stronger families
- do not freeze final calculus
- do not start endless syntax comparisons

Expected outputs:
- one convergence doc
- updates to `plan/06`, `plan/13`, `specs/10`, maybe new `specs/examples/*`

### Package 4 — Lean/Rust proof plan package
Must do:
- write formal proof roadmap
- write what must be proven before/after implementation
- define Lean-first line and Rocq fallback line
- define artifact conformance and differential-testing strategy
- connect this to current runner/prototype evidence

Expected outputs:
- theory/proof plan doc
- updated roadmap and heavy workstream note
- explicit proof obligations list

### Package 5 — near-end closeout
Must do:
- summarize what is now close
- summarize remaining mixed gates
- summarize true user-spec gates
- define next reserve packages
- ensure no misleading “theory solved” wording appears anywhere

Expected outputs:
- final closeout report
- updated `plan/90-source-traceability.md`
- relevant snapshot updates

---

## 12. explicit “do not do” list

Codex must not:

- claim theory solved
- claim final public language implementation complete
- collapse `atomic_cut` into global cut / seq_cst / durable commit
- import low-level `memory_order` exact surface as source-principal wording
- freeze final parser grammar in this closeout tranche
- freeze final public parser/checker/runtime API in this closeout tranche
- freeze final public verifier contract in this closeout tranche
- finalize the exhaustive shared-space catalog
- treat `λ◯□` as sufficient full basis
- keep compare-floor alive when actual adoption package is already possible
- ignore queue drift
- skip plan/spec/report synchronization

---

## 13. current open but acceptable residuals

It is acceptable for the following to remain open after this tranche:

- final parser grammar
- final public parser/checker/runtime API
- final public verifier contract
- final public theorem/model-check production contract
- exhaustive shared-space final catalog
- broader fairness/distributed-fairness theorem
- packaging / installed binary / host integration target
- upper-layer application target beyond authoritative-room first scenario
- final calculus adoption among stronger modal foundations

The key is:
- these must be **explicitly named**
- they must not block near-end closeout of the theory line
- they must be separated into mixed gate / true user-spec gate / reserve integration

---

## 14. exact answers to the user positions already expressed

These are already-given preferences and should be treated as defaults unless conflict appears.

1. A single strong theoretical spine is preferred over a bag of unrelated theories.
2. Layering theories per module/region is acceptable, but compatibility metatheory must be built.
3. Multistage computation is core-ish and should be treated carefully in the principal theory.
4. Strong typing is desired.
5. Contract-programming-like aspects are desired.
6. Some dependent-typing-like power is desired.
7. User-defined security label models are desired.
8. Compile-time guarantees such as “a secret key is not shared outside policy scope” are desired.
9. Theory should be fixed before syntax.
10. Syntax should correspond cleanly to theory and semantics, not be dragged by existing languages.
11. Lean is preferred if possible.
12. Existing layers like security certification, visualization, host/adapters, etc. are known to exist as later layers.
13. Local theories per source area/module plus checked external boundaries is an acceptable architectural style.

Codex should **not re-open these as if they were unanswered**.  
If a conflict emerges, narrow it to one explicit issue and report it.

---

## 15. one-page summary for fast recall

If you only remember one page, remember this:

- principal spine = multimodal dependent core
- `λ◯□` = partial basis, not full basis
- linear capability + effect rows + decidable refinement + IFC labels = layered typing architecture
- checker / theorem / protocol / runtime policy split = principal verification architecture
- relation decomposition + authority-serial + witness-aware = principal order/handoff direction
- explicit edge-row / vertical continuation = principal syntax family
- stage-block = strong secondary syntax family
- serial sugar = reserve room-level sugar
- Lean-first = principal proof path
- Problem 1 and Problem 2 are **not solved**, but their current first lines are source-backed and machine-checked
- the main debt is adoption/selection, not discovery
- next move = close actual adoption packages, not invent more candidate families

---

## 16. Codex final response requirements

When finishing the next tranche, return at least:

1. updated files
2. new files
3. drift audit summary
4. current first line (Problem 1 / Problem 2 / syntax-modality)
5. retained alternatives
6. stop lines
7. remaining mixed gates
8. remaining true user-spec gates
9. queue reconstruction summary
10. tests / validation actually run
11. prototypes / samples used as evidence
12. next self-driven packages
13. reserve integration packages
14. report paths

And explicitly say:
- what is now actually close
- what is still mixed
- what is still target-dependent
- what you intentionally did **not** decide

---

## 17. appendices

### 17.1 candidate syntax snippets (non-final, but recommended comparison material)

#### A. explicit edge-row / vertical continuation

```text
roll draw via authority_rng
publish draw
handoff dice_owner A -> B
  after publish(draw)
  requires witness(draw)
```

#### B. stage-block secondary

```text
transition handoff_turn(owner = A) {
  stage roll:
    draw <- perform via authority_rng

  stage publish:
    publish draw

  stage handoff:
    change_owner dice_owner to B
      requires witness(draw)
}
```

#### C. serial sugar reserve

```text
serial on dice_owner {
  draw <- perform via authority_rng
  publish draw
  handoff dice_owner A -> B
    requires witness(draw)
}
```

### 17.2 conceptual type/interface sketches

#### judgment sketch

```text
Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ O
```

#### label model sketch

```text
LabelModel = {
  Label       : Type
  flows_to    : Label -> Label -> Prop
  join        : Label -> Label -> Label
  meet        : Label -> Label -> Label
  flows_dec   : Decidable (flows_to a b)
  laws        : preorder / semilattice laws
}
```

#### security-typed value sketch

```text
Labeled ℓ A
```

#### module boundary sketch

```text
module M :
  assumes  modes ΨM
  exports  types, effects, labels, obligations
  proves   discharged obligations OM_local
  requires imported obligations OM_ext
```

### 17.3 literature pack to keep in view

Modal / multimodal / staging:
- Davies–Pfenning
- Yuse–Igarashi (`λ◯□`)
- guarded lambda-calculus
- Modal Dependent Type Theory
- Multimodal Dependent Type Theory

Effects / refinement / IFC:
- Koka / row-polymorphic effects
- Liquid Types
- Jif / DLM
- Flow Caml
- RHTT-like dependent IFC references

Order / cuts / concurrency:
- Lamport 1978
- Chandy–Lamport 1985
- Boehm–Adve 2008
- HSA scoped memory model references
- WG21 `memory_order` / `consume` / `kill_dependency` line
- event-structure / relaxed-memory references
- linearizability references

---

## 18. final instruction

Use this document not as prose to archive, but as an **execution brief**.  
The right next move is not to reopen the whole design space.  
The right next move is to:

1. synchronize docs and queue,
2. lock the principal spine,
3. close actual adoption packages for the two big problems,
4. encode the proof plan,
5. leave only genuine mixed/user-spec gates visible.

That is the shortest route from the current state to a defensible near-end theory closeout.
