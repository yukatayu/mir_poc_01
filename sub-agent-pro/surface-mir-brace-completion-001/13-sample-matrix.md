# 13 — Required Sample Matrix

## Surface syntax

| ID | Description | Expected |
|---|---|---|
| SURF-01 | `S { state player[p: Participant]: Player }` parses | accepted |
| SURF-02 | `S[ ... ]` place syntax | check_rejection `bracket_place_scope_not_supported` |
| SURF-03 | record literal `Player { hp: 1 }` | accepted |
| SURF-04 | ambiguous place/type name | check_rejection `ambiguous_brace_construct` |
| SURF-05 | `Participant[self] { when start { ... } }` | accepted |

## Indexed state

| ID | Description | Expected |
|---|---|---|
| IDX-01 | server-owned participant-indexed state | accepted |
| IDX-02 | key write without capability | rejection |
| IDX-03 | stale key access after leave | runtime_rejection |
| IDX-04 | compaction blocked by savepoint | rejection/deferred |

## Auto communication

| ID | Description | Expected |
|---|---|---|
| ELAB-01 | cross-place read generates request | accepted |
| ELAB-02 | cross-place write generates request | accepted |
| ELAB-03 | private field auto-publish | rejection |
| ELAB-04 | undeclared generated failure | check_rejection |
| ELAB-05 | generated Core IR source spans | accepted |

## Role admission

| ID | Description | Expected |
|---|---|---|
| ROLE-01 | BrowserClient join accepted | accepted |
| ROLE-02 | role claim without grant cannot write server | rejection |
| ROLE-03 | stale membership message | runtime_rejection |
| ROLE-04 | package hash binding optional report | accepted |

## Patch hot-plug

| ID | Description | Expected |
|---|---|---|
| PATCH-01 | source patch adds visible state | accepted |
| PATCH-02 | patch undeclared failure | check_rejection |
| PATCH-03 | patch self-grants server authority | rejection |
| PATCH-04 | patch lifecycle devtools export | accepted |

## Computational

| ID | Description | Expected |
|---|---|---|
| COMP-SURF-01 | pure add_one.mir | accepted |
| COMP-SURF-02 | variables/scope | accepted + negative |
| COMP-SURF-03 | arrays bounds | accepted + negative |
| COMP-SURF-04 | records Vec3 | accepted + negative |
| COMP-SURF-05 | control flow | accepted + negative |
| COMP-SURF-06 | imports/functions | accepted + negative |

## PoseGraph

| ID | Description | Expected |
|---|---|---|
| POSE-SURF-01 | head transform | accepted |
| POSE-SURF-02 | anchored object | accepted |
| POSE-SURF-03 | fallback anchor | accepted |
| POSE-SURF-04 | no-split-frame | accepted |
| POSE-SURF-05 | split-frame violation | violation_export |

## End-to-end

| ID | Description | Expected |
|---|---|---|
| E2E-SURF-01 | WorldCore source workflow | accepted |
| E2E-SURF-02 | MembershipChat source workflow | accepted |
| E2E-SURF-03 | Sugoroku source workflow | accepted |
| E2E-SURF-04 | Source patch hot-plug | accepted |
| E2E-SURF-05 | Devtools shows generated communication | accepted |
| E2E-SURF-06 | local/Docker controlled transport | accepted |
