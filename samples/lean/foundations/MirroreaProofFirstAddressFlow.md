# Fixed reference resolution — LAB candidate

An arbitrary fixed function resolves source-reference names to concrete cells.
It need not be injective. Expression evaluation, inferred type and dependency
rank commute with renaming when the source store, schema and policy are pulled
back from the same concrete cells. Assignment updates all source references
resolving to the target cell. For arbitrary finite aborting programs, lowering
preserves the pulled-back final store and the ordered outcomes mapped to cells.
The aborting-sequence checker commutes with the same translation.

The positive alias control resolves two names to one cell, assigns 11 through
one name, then increments through the other and obtains 12. A model treating
spellings as independent cells instead obtains 11. Inconsistent labels for two
aliases cannot be a pullback from a single cell policy.

These results concern an explicitly supplied, common, stable resolver. They do
not prove that the actual Mir parser, checked artifact, request arguments or
runtime address lookup computes that resolver, nor that it is live or
authorized. Dynamic reconfiguration requires version/incarnation checks and a
new correspondence obligation. Failure metadata and observation provenance are
not supplied by mapping the target. At draft creation the Oracle review was pending; its recovery is recorded below.

2026-09-28 forward review update: the unchanged finite cut was reviewed by the read-only Oracle and locally dispositioned; see `docs/proof-first/W4_CHECK.json` (recorded `mir-w4-c-abort-address` session). Earlier pending wording describes the draft state. This closes that finite review dependency only; supplied metadata/source/runtime, authority/resources and physical observation remain open.
