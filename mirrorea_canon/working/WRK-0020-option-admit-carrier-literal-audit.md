---
id: working/WRK-0020
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/01-mircore-v0, theory/06-existence-fallback, spec/02-surface-grammar, spec/04-core-ir]
summary: option-local admit を含む Canon の chain / Surface / companion notation と theory/01 の Option payload の literal carrier 対応を既存 current-L2 lane で監査する。carrier、grammar、OBL は選ばない。
open_items: []
---

# WRK-0020 - Option admit carrier literal audit

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@64d2571c3653a33fcd2f53afb3f1747e4a19ad4e:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/01-mircore-v0@64d2571c3653a33fcd2f53afb3f1747e4a19ad4e:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/06-existence-fallback@64d2571c3653a33fcd2f53afb3f1747e4a19ad4e:3da20d43a0a87ec8417a4519700777adea141f499e2627f433927ce975a086c8, spec/02-surface-grammar@64d2571c3653a33fcd2f53afb3f1747e4a19ad4e:7d97fb4e77f493c3e0be4dbffdee64f206936c1aca1d5c535c19195f81f592b1, spec/04-core-ir@64d2571c3653a33fcd2f53afb3f1747e4a19ad4e:50c23acf01deedbe5bdb78baeba58053e28c940d8202b6d25bfd1f03546fd950
LAB inputs: LAB:plan/07-parser-free-poc-stack.md@64d2571c3653a33fcd2f53afb3f1747e4a19ad4e:13811bee2e8f336c3823f32508a5f0925184b419a59a2d2ea6ab32fb7a345a22, LAB:samples/current-l2/e3-option-admit-chain.txt@64d2571c3653a33fcd2f53afb3f1747e4a19ad4e:1132579d3c70c875940c526ff3a8bdef835a5d6bdc5dbf16d8ff6bcde0a6952f
Permitted LAB locations: plan, samples/current-l2
Reserved surfaces: excluded

## Pre-registered working question

Question: At the pinned source cut, do the named Canon descriptions give one
literal carrier account for option-local `admit`? In particular, does
`theory/01`'s `Option o ::= option(name, target, cap, lease)` either include
`admit` or name its preserved carrier, while `theory/06`, `spec/02`,
`spec/04`, and the pinned current-L2 e3 input explicitly make `admit`
option-local and selection-relevant?
Status quo: The selected theory/06 option shape, Surface grammar, Core
companion notation, and current-L2 e3 input spell an option-local `admit`.
The selected theory/01 Option syntax names only `name`, `target`, `cap`, and
`lease`; its chain-access sketch says to resolve the leftmost admissible option
but does not name an `admit` payload or a carrier for it. `plan/07` records
that the fixture-side handoff remains docs-only deferred, not a Canon bridge.
Alternative: The pinned theory/01 text may already include `admit` or an
explicit preserved carrier in the named Option/chain-access passages; or one
of the selected fallback, Surface, companion, or e3 texts may not actually
make `admit` option-local. Either result rejects the proposed literal mismatch.
Expected falsifier: Any pinned digest differs; the registered source audit
cannot establish the exact selected text facts; or it finds a direct
theory/01-local statement that carries option-local `admit` through the
abstract Option or chain-access form. A need to choose whether `admit` belongs
in Option, constraints, residual obligations, another Core form, or a new
grammar/JSON field is an escalation, not a repair.
Rollback / reopen trigger: On any falsifier, set Reliance status to frozen,
retain only the permitted audit evidence, and do not edit the Canon syntax,
grammar, current-L2 input, parser, helper, schema, or runtime. Reopen only as
a distinct record with a newly fixed carrier question, or escalate for the
ordinary Canon process if a carrier decision is needed.

## Method and evidence plan

Result class: literal-transcription
Commands: python3 -c "from pathlib import Path; import hashlib; files={'mirrorea_canon/theory/01-mircore-v0.md':'35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12','mirrorea_canon/theory/06-existence-fallback.md':'3da20d43a0a87ec8417a4519700777adea141f499e2627f433927ce975a086c8','mirrorea_canon/spec/02-surface-grammar.md':'7d97fb4e77f493c3e0be4dbffdee64f206936c1aca1d5c535c19195f81f592b1','mirrorea_canon/spec/04-core-ir.md':'50c23acf01deedbe5bdb78baeba58053e28c940d8202b6d25bfd1f03546fd950','plan/07-parser-free-poc-stack.md':'13811bee2e8f336c3823f32508a5f0925184b419a59a2d2ea6ab32fb7a345a22','samples/current-l2/e3-option-admit-chain.txt':'1132579d3c70c875940c526ff3a8bdef835a5d6bdc5dbf16d8ff6bcde0a6952f'}; text={path:Path(path).read_text() for path in files}; assert all(hashlib.sha256(text[path].encode()).hexdigest()==digest for path,digest in files.items()); core=text['mirrorea_canon/theory/01-mircore-v0.md']; option=core[core.index('Option      o ::='):core.index('Surface s ::=')]; chain=core[core.index('**[CHAIN-ACCESS]**'):core.index('## 5.')]; assert option.strip()=='Option      o ::= option(name, target, cap, lease)' and 'admit' not in option and 'admit' not in chain; assert 'oᵢ = option(name, target, cap, lease [, admit])' in text['mirrorea_canon/theory/06-existence-fallback.md']; assert '[ \"admit\" Expr ]' in text['mirrorea_canon/spec/02-surface-grammar.md']; assert 'admit pred' in text['mirrorea_canon/spec/04-core-ir.md']; assert 'option owner_writer on profile_doc capability write lease live admit owner_is(session_user)' in text['samples/current-l2/e3-option-admit-chain.txt']; assert 'fixture-side `OptionDecl.admit` handoff' in text['plan/07-parser-free-poc-stack.md']"
Execution cut: `64d2571c3653a33fcd2f53afb3f1747e4a19ad4e` is the authority/input snapshot. Execute the first outcome command only after this registration commit is committed and pushed. The evidence commit may add only `plan/wrk-0020-option-admit-carrier-literal-audit.md`, its `plan/00-index.md` entry, a direct numbered report, allowed working-record metadata/control files, and no source/runtime/helper/schema/CI artifact. A later metadata-only commit may append the exact evidence commit and artifact digest without rewriting this pre-registration.
Non-claims: This does not decide whether `admit` belongs in Option, a constraint, a residual obligation, a new Core constructor, another preserved carrier, or a grammar/JSON/runtime field. It does not state a complete fallback semantics, prove THM-002, supply an OBL-005/006/007/008 premise, change theory/11, select a parser lowering, reinterpret current-L2 as Canon, add a helper/schema/CI/Make surface, or make a conformance, Gate, Phase, runtime, public, or L2-promotion claim.

## Results and review

Reliance status: not-promoted
Positive evidence: not-run
Negative evidence: not-run
Evidence artifacts: none
Evidence commits: none
Impact / non-effects: The registered outcome is limited to whether the pinned
named texts literally agree on an explicit option-local `admit` carrier. It
introduces no source, test, helper, schema, CI/Make target, API, runtime,
contract, proof-status, conformance, Gate, Phase, or public claim. A positive
literal mismatch is evidence for a later owner decision, not an instruction to
choose its resolution.
Independent review: not-required-for-L3

## Supersession

Supersession: none
