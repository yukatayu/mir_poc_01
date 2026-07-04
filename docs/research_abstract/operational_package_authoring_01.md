# Operational Package Authoring 01 Summary

`P-OPS-09` は、external developer が operational product package を author するための **validated starter + bounded guide** です。

## What Is Added

- `samples/product-alpha1/operational/templates/`
- `samples/product-alpha1/operational/templates/world-core-starter/`
- `samples/product-alpha1/operational/templates/membership-chat-starter/`
- `samples/product-alpha1/operational/templates/sugoroku-world-starter/`
- `docs/hands_on/operational_package_authoring_01.md`

## What It Gives You

- a validated starter catalog for `world_core`, `membership_chat`, and `sugoroku_world`, with the current `membership_chat` starter already aligned to the bounded `ChatText` room-chat lane
- explicit dependency-retarget guidance for the starters that import upstream operational roots
- a concrete list of fields that must be renamed before treating the template as your own package
- a current stop line that keeps portal/shard authoring on active executable roots instead of adding starter duplicates now
- the practical order `check -> run-local -> session -> export-devtools -> view --check`
- common diagnostics for unsupported package kinds, unsupported recovery policies, missing dependency roots, and direct `.mir` front-door misuse

## What It Does Not Claim

- final textual grammar
- final public ABI / SDK
- generic scaffold CLI
- automatic release helper generation for arbitrary new packages
- final server/client split
- direct LLVM backend

## Entry Points

- hands-on: `../hands_on/operational_package_authoring_01.md`
- operational suite guide: `../hands_on/operational_product_sample_01.md`
- LAB boundary evidence: `../../specs/26-operational-product-sample-suite.md`
- roadmap memory: `../../plan/51-operational-product-sample-roadmap.md`
