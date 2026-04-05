# 85 — current L2 stage 3 admit-slot first tranche actualization

## 目的

この文書は、`specs/examples/84-current-l2-stage3-admit-slot-carrier-and-compare-surface.md` で固定した

- parser-side carrier 名の第一候補は `decl_admit_slot`
- compare surface は lowered fixture-subset compare と parser-side slot retention smoke の 2 本立て
- fixture-side `OptionDecl.admit` への direct lowering はまだ行わない

という current judgment を前提に、
**stage 3 admit-slot branch の success-side first tranche を current repo でどこまで actualize 済みか**
を記録する。

ここで固定するのは final parser grammar ではない。
固定するのは、current repo に存在する

- private / test-only helper
- `e3` option / chain subset compare
- `decl_admit_slot.surface_text` retention smoke

の actual code anchor だけである。

## 前提

- current L2 の core semantics は変更しない。
- stage 1 parser spike の first tranche / malformed-source tranche は維持する。
- stage 3 first tranche は declaration-side `admit` attached slot だけに留める。
- `PerformVia` / request-local `require` / `ensure` / predicate fragment parse は still later stage に残す。
- public parser API、typed parser diagnostics、fixture schema widening は導入しない。

## current actualized helper

### files

- `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`

### current helper scope

current helper が actualize しているのは次だけである。

1. option declaration header
   - `name`
   - `target`
   - `capability`
   - `decl_guard_slot.surface_text`
2. optional `decl_admit_slot.surface_text`
3. chain declaration core
   - `head`
   - `fallback`
   - edge-local lineage metadata
4. lowered fixture-subset compare
   - option declaration structural subset
   - chain structural subset
5. parser-side `decl_admit_slot.surface_text` retention smoke

### current compare contract

#### lowered fixture-subset compare に含めるもの

- option name
- target
- capability
- `lease` compatibility anchor
- chain head
- edge predecessor / successor
- lineage assertion

#### parser-side retention smoke に残すもの

- `decl_admit_slot` が option declaration に attached していること
- `decl_admit_slot.surface_text` が raw に保持されること

#### current first tranche でまだ compare しないもの

- fixture-side `OptionDecl.admit` predicate node
- `PerformVia`
- request-local `require` / `ensure`
- predicate fragment parse
- public parser diagnostics wording

## current source-backed evidence

### success-side input

current first tranche は、`e3-option-admit-chain` 由来の option / chain subset inline text を使う。

つまり current parser input は次だけである。

- `option owner_writer ... admit owner_is(session_user)`
- `option delegated_writer ... admit delegate_granted(session_user)`
- `chain profile_ref = owner_writer`
- `fallback delegated_writer @ lineage(owner_writer -> delegated_writer)`

`PerformVia` 行は current first tranche input から除外している。

### existing fixture anchor

compare target は
`crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
から取り出した structural subset である。

この compare は current helper では、
fixture-side `admit` node を使わず、
option / chain structural subset だけを参照する。

## current meaning

current first tranche が示しているのは次である。

1. declaration-side `admit` attached slot は、private / test-only parser spike として narrow actualization できる
2. その actualization は fixture-side `admit` predicate node へ direct lowering しなくてもよい
3. structural subset compare と parser-side slot retention smoke を分ければ、opaque attached slot cut を壊さずに `e3` contrast の中心へ近づける

## current non-goal

- `PerformVia` line の parse
- request-local `require` / `ensure`
- `decl_admit_slot` の internal predicate parse
- fixture-side `OptionDecl.admit` への direct lowering
- helper-local predicate canonicalization
- public parser API
- malformed-source first tranche

## next narrow step

current first tranche の次に比較するなら、次が最小である。

1. stage 3 admit-slot branch の malformed-source smoke を helper-local にどこまで持たせるか
2. `PerformVia` / request-local clause spillover を stage 3 admit-slot branch と混ぜずに fail-closed で示す最小 pair は何か
3. fixture-side `admit` node への handoff comparison を later stage にどう残すか
