---
id: working/WRK-0025
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, spec/01-lexical-and-modules, spec/02-surface-grammar, theory/01-mircore-v0, theory/03-elaboration, meta/proposal-004, meta/proposal-008, meta/proposal-015]
summary: P004 A と P015 return exclusion 後も、表示済み Surface grammar の全 parse form が Core または明示的 Diagnostic へ一意に分類されるかを literal inventory で検査する。exact domain、grammar、Core、OBL は選ばない。
open_items: []
---

# WRK-0025 - Surface totality-domain inventory

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@616d291cb181c4d71352df0ef8bc1ce4b569c1cd:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, spec/01-lexical-and-modules@616d291cb181c4d71352df0ef8bc1ce4b569c1cd:558337a52a04e94441bdda161d890d0faf3fa4afb2492e4dd3b090415d1bf2ed, spec/02-surface-grammar@616d291cb181c4d71352df0ef8bc1ce4b569c1cd:7d97fb4e77f493c3e0be4dbffdee64f206936c1aca1d5c535c19195f81f592b1, theory/01-mircore-v0@616d291cb181c4d71352df0ef8bc1ce4b569c1cd:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/03-elaboration@616d291cb181c4d71352df0ef8bc1ce4b569c1cd:2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641, meta/proposal-004@616d291cb181c4d71352df0ef8bc1ce4b569c1cd:9770de1a2657640a08688207b31f8bffaef63fe11f4019e5a058f5f2ac5cf1f7, meta/proposal-008@616d291cb181c4d71352df0ef8bc1ce4b569c1cd:777a6b2e043ae0313c402c836341bdedf9e12758f480c44fef8391715d34f3dc, meta/proposal-015@616d291cb181c4d71352df0ef8bc1ce4b569c1cd:e8b016be00bf4dd9bc8204451b7d72a871fc4fd29a88d7f4cdbb5090619f7745
LAB inputs: LAB:plan/199-selected-semantic-composition-and-inference-boundary.md@616d291cb181c4d71352df0ef8bc1ce4b569c1cd:0c209c7f7b6b16253b08ff3756469139668378cb8f58e9c119f471ad6d16a63e
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: After the recorded P004 Participant-only direction and P015 `return`
exclusion, does every form still admitted by the displayed Surface grammar and
P004 candidate EBNF have either (a) a literal MirCore/elaboration counterpart
or (b) an explicit existing Diagnostic rejection? If not, can a finite
inventory identify the unclassified forms without choosing the future exact
`WellScoped` predicate, grammar amendment, Core operation, diagnostic family,
or obligation identity?
Status quo: P008 A records that total outcome production must become a separate
obligation, but it does not select its domain. P004 A directs a Participant-only
Surface closure but retains calls, multiplicative/relational expressions and
other forms whose Core counterpart is not stated; P015 excludes `return` but
does not rewrite `spec/02`. `theory/03` promises an outcome only for a
well-scoped Surface item, without defining that exact finite subset.
Alternative: the pinned sources already map every displayed/admitted form to a
Core form or an explicit rejection, so the inventory has no unclassified row.
Expected falsifier: Any pinned digest differs; every inventory row has a
literal Core/elaboration mapping or explicit existing Diagnostic; an apparent
gap is covered by a cited source sentence; or retaining the result requires
selecting/rewording grammar, `WellScoped`, a Core constructor, Diagnostic,
OBL/theory/11 status, helper/schema/CI, or public contract.
Rollback / reopen trigger: On any falsifier set `Reliance status` to `frozen`,
retain only reproducible procedure evidence, and do not repair or rerun this
record. Escalate rather than repair if a follow-up chooses the exact domain,
grammar production, rejection code, Core/elaboration operation, outcome
obligation, SCN/Gate/Phase, implementation, or public interface.

## Method and evidence plan

Result class: literal-transcription
Commands: rg -n -C 2 'Stmt|Return|Keyspace|Expression precedence|CallArgs|multiplicative|comparisons|Surface s|Expr e|Core  c|well-scoped|Diagnostic|OPEN-011' mirrorea_canon/spec/01-lexical-and-modules.md mirrorea_canon/spec/02-surface-grammar.md mirrorea_canon/theory/01-mircore-v0.md mirrorea_canon/theory/03-elaboration.md mirrorea_canon/meta/proposals/PROPOSAL-004-surface-v0-grammar-closure.md mirrorea_canon/meta/proposals/PROPOSAL-008-elaboration-outcome-totality-boundary.md mirrorea_canon/meta/proposals/PROPOSAL-015-surface-fallback-and-return-closure.md; python3 -c "from pathlib import Path; sources = {p: Path(p).read_text() for p in ('mirrorea_canon/spec/02-surface-grammar.md', 'mirrorea_canon/theory/01-mircore-v0.md', 'mirrorea_canon/theory/03-elaboration.md')}; required = {'mirrorea_canon/spec/02-surface-grammar.md': ('Return', 'CallArgs', 'multiplicative'), 'mirrorea_canon/theory/01-mircore-v0.md': ('Surface s', 'Core  c'), 'mirrorea_canon/theory/03-elaboration.md': ('well-scoped Surface item', 'Diagnostic')}; assert all(all(token in sources[p] for token in tokens) for p, tokens in required.items())"; git diff --check
Execution cut: `616d291cb181c4d71352df0ef8bc1ce4b569c1cd` is the authority/input snapshot. Execute every outcome command only after this registration is committed and pushed. The evidence commit may add only `plan/wrk-0025-surface-totality-domain-inventory.md`, its `plan/00-index.md` entry, a direct numbered report, allowed working-record metadata/control files, and no helper, schema, CI/Make, parser, checker, theory, contract, or public artifact. A later metadata-only commit may append the exact evidence commit and artifact digest without rewriting this pre-registration.
Non-claims: This does not define the exact totality domain, alter Surface
grammar or lexical rules, allocate a Diagnostic, add a Core term, interpret
calls or operators, decide parser behavior, change an OBL/theory/11 status,
SCN, Gate/Phase, conformance, runtime, transport, API, or public behavior.

## Results and review

Reliance status: not-promoted
Positive evidence: Pending registration and the registered commands.
Negative evidence: Pending registration and the registered commands.
Evidence artifacts: none
Evidence commits: none
Impact / non-effects: Until an outcome is retained, this record only fixes a
read-only inventory question. It neither turns parseability into well-scopedness
nor treats a proposal direction as a grammar/Core amendment.
Independent review: not-required-for-L3

## Supersession

Supersession: none
