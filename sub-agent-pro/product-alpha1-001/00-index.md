# Product/Public Alpha-1 handoff index

## Purpose

This handoff package instructs Codex to move from bounded operational workflows to a **product/public-ready Mirrorea Spaces α-1**.

Current repo state already has:

- bounded operational α-0.5 local observable runtime;
- bounded operational α-0.8 same-session hot-plug runtime;
- bounded operational α-0.9 session-bound devtools export;
- bounded practical α-1 integrated workflow carrier.

It still lacks product/public-ready α-1.

## Read order

1. `prompt.md`
2. this `00-index.md`
3. `01-current-state-and-gap.md`
4. `02-product-alpha1-definition.md`
5. `03-architecture-and-repo-plan.md`
6. `04-theory-freeze-requirements.md`
7. `05-message-recovery-and-savepoint.md`
8. `06-auth-and-layer-algebra.md`
9. `07-devtools-ux-design.md`
10. `08-native-package-and-host-policy.md`
11. `09-sample-and-validation-matrix.md`
12. `10-package-sequence.md`
13. `11-subagent-review-plan.md`
14. `12-closeout-protocol.md`
15. `13-risk-register.md`

## Main distinction

Do not confuse these categories:

| Category | Meaning |
|---|---|
| evidence | docs / reports / expected JSON / helper result |
| first-floor evidence | limited runner/carrier/sample family works |
| bounded operational workflow | a developer can reproduce a layer workflow end-to-end |
| bounded practical workflow | several layer workflows are composed |
| product/public-ready α-1 | outside developer can use the α package/toolchain with documented public-ish boundary |

The target is the last category, but still **alpha**, not final product.

## Package line

Recommended package sequence:

1. `P-A1-25` product/public α-1 boundary recut
2. `P-A1-26` CLI + package schema stabilization
3. `P-A1-27` product demo same-session runtime
4. `P-A1-28` message failure/recovery + quiescent-save
5. `P-A1-29` product devtools viewer UX
6. `P-A1-30` native launch bundle
7. `P-A1-31` release-candidate validation closeout

Proceed autonomously while each package remains narrow, validated, and non-overclaiming.
