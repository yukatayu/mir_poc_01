---
id: spec/07-diagnostics-format
status: L2-working
maturity: draft
depends_on: [theory/10-diagnostics]
summary: 診断 ID 体系、必須フィールド、span 形式、文言規則、代表例。
open_items: [OPEN-028]
---

# 07 — Diagnostics format

## ID scheme `E-<FAMILY>-<###>`

| Family | Scope | Examples |
|---|---|---|
| E-PARSE | lexical/grammar | 001 generic; 002 bracket_place_scope_not_supported (`use S { ... }; [] is indexing`); 003 ambiguous_brace_construct; 005 core-companion token in Surface; 006 `=` used as equality |
| E-NAME | resolution | 001 undeclared place head; 002 undeclared role head; 003 ambiguous name collision; 004 duplicate module path |
| E-TYPE | typing | 001 mismatch; 002 key type ≠ keyspace; 004 record equality unsupported |
| E-DECL | underdeclared | 001 chain missing lineage/target (static error, never dynamic) |
| E-LIN | lineage | 002 malformed annotation/edge; 003 capability strengthening in chain |
| E-EFF | effects | 001 undeclared effect |
| E-ROW | failure rows | 001 generated ⊄ declared fails; 002 undeclared VisibilityDenied |
| E-VIS | visibility | 002 private field auto-publish blocked |
| E-IDX | indexed state | 002 write without owner/capability; 003 stale key |
| E-AUTH | authority | 001 key-is-authority inference; 002 grant-required before verdict; 003 nested block assumed ambient; 004 grant surface misuse |
| E-PATCH | patching | 001 missing compat field; 002 undeclared failure; 003 self-grant |
| E-CUT | cuts/save-load | 001 inconsistent cut; 002 stale resurrection attempt |

## Required fields (theory/10 carrier, JSON)

`id, severity(error|warn), span{file, byte_range, line_col}, rule_instance,
failed_premise, missing_evidence[], suggested_repair[], refs[]`. Every error
must carry ≥1 span; multi-span allowed (declaration site + use site).

## Wording rules

State the principle, then the repair. Use mental-model/03 vocabulary for the
three intuition gaps (e.g. E-LIN family says "fallback moves forward along a
chain; returning to a stronger option needs an explicit reacquire", never
"cannot restore outer value"). Never blame the user; blame the missing
declaration. Sentence case; ≤2 sentences before the repair suggestion.

OPEN-028: localization (Japanese message catalog) at PHASE-I5.
