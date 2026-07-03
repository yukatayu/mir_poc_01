---
id: spec/03-static-semantics
status: L1-fixed
maturity: draft
depends_on: [spec/02-surface-grammar, theory/02-types-effects-failures, theory/03-elaboration]
summary: 宣言義務、名前解決、型付け(theory への束縛)、failure row 包含、可視性、keyspace 制限。
open_items: []
---

# 03 — Static semantics

Binding: the typing/elaboration authority is theory/01–03's unified judgment.
This chapter fixes the surface-visible obligations an implementation must
check, each with its diagnostic id (spec/07).

1. **Declaration obligations.** Every place/role/record/keyspace used must be
   declared or imported (E-NAME-001/002). Handlers exist only inside place or
   role-instance blocks. `grant`/`require`/`publish` only in loci whose role
   permits admission surfaces (E-AUTH-004 otherwise).
2. **Indexed state.** `state x[k:K]: A at ℓ` denotes an ℓ-owned partial map
   `Active(K, epoch) ⇀ A`. Keyspaces are declared, finite kinds; arbitrary
   maps do not gain membership/authority semantics unless declared through
   this form. The key never confers authority (E-AUTH-001 on the inference).
3. **Failure-row containment.** For every handler, generated failures ⊆
   declared `fails` (E-ROW-001). Generated set per theory/03; undeclared
   `VisibilityDenied` from visible-field communication is E-ROW-002.
4. **Visibility.** `visible` without `fields` marks the whole value at the
   given level; `fields` narrows the generated surface. Private(-looking)
   fields never auto-publish (E-VIS-002); observer_safe output obeys
   theory/07 policies.
5. **Cross-locus access.** Reads need observe/visibility authority; writes
   need write capability or owner mediation; nested `O { ... }` from L ≠ O is
   never ambient authority (E-AUTH-003 if assumed).
6. **Chains.** Static evidence floor per theory/06: targets + edge-local
   lineage annotations required (E-DECL-001 underdeclared, E-LIN-002
   malformed); capability strengthening along a chain is E-LIN-003.
7. **Types.** Record field access/typing standard; comparisons homogeneous;
   `==` on Int64/Float64/Bool/Text/Key only (record equality undecided —
   E-TYPE-004 for now). Index expressions must have the declared keyspace
   type (E-TYPE-002).
8. **Effect rows.** Statements requiring undeclared effects (external
   adapters, patch ops) are E-EFF-001.
9. **Patch modules.** A patch module must satisfy the compatibility carrier
   fields (theory/08); missing fields are E-PATCH-001.
