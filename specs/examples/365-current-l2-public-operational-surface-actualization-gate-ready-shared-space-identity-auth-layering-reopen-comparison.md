# 365 — current L2 public-operational-surface-actualization-gate-ready shared-space-identity-auth-layering-reopen comparison

## 目的

`specs/examples/364-current-l2-public-operational-surface-actualization-gate-ready-minimal-public-operational-surface-actualization-gate-threshold.md`
で public operational surface actualization gate の minimum を fixed した次段として、

- shared-space line の next docs-first reopen を identity / auth layering に narrow に戻すとき、どの boundary cut が current first choice か
- principal continuity、transport / service auth、room-local admission、display / projection identity をどこで分けるのが自然か
- compile-time admission visibility と authority / resource ownership をどこまで次段へ残すべきか

を比較する。

ここで固定するのは
**current L2 public-operational-surface-actualization-gate-ready shared-space-identity-auth-layering-reopen comparison**
であり、

- concrete auth protocol binding
- admission policy / compile-time visibility reopen
- authority / resource ownership reopen
- Mirrorea operational runtime realization

はまだ固定しない。

## scope

- current package は shared-space docs-first boundary package に留める。
- entry criteria は `specs/examples/121...125`、`295...296`、`357...358`、`363...364` と `plan/16-shared-space-membership-and-example-boundary.md` に置く。
- room semantics へ raw auth protocol や transport refresh policy を持ち込まず、shared-space line では membership core と room-local bridge だけを扱う。
- authoritative room と append-friendly room の両方で読める cut を優先し、closed-world exact admission や final auth stack は still later に残す。

## current 前提

current repo では次が成立している。

1. `specs/examples/357...358` により、Mirrorea/shared-space docs-first re-entry bundle は fixed 済みであり、identity/auth layering は next docs-first reopen refs に置かれている。
2. `plan/12-open-problems-and-risks.md` では、principal identity、transport auth、service login、room permission、display identity を membership carrier に潰すことを current risk として explicit に残している。
3. `plan/16-shared-space-membership-and-example-boundary.md` では、membership registry を source of truth にしつつ、identity / auth / admission / projection を別軸に保つ current working lineが既に比較済みである。
4. authoritative room の current practical line では、`authority-ack` を activation rule の最小候補に保ちつつ、actual auth stack と admission event の受理は runtime control-plane に残す cut が current working line である。

したがって current 問いは、
**shared-space identity / auth line の first reopen を、membership core と auth / admission / projection layering の narrow split に固定するのが自然か**
である。

## 比較観点

1. principal continuity と room-local membership slot を current boundary で保てるか
2. transport / service auth や room admission を membership carrier に潰さずに済むか
3. display / projection identity を principal continuity と別軸に保てるか
4. next reopen を admission policy / compile-time visibility へ clean に handoff できるか

## 比較対象

### 案 1. membership carrier に identity / auth / permission / projection をまとめて埋め込む

#### 利点

- implementation 初期には 1 carrier で room join を説明しやすい。
- participant entry 単体だけで参照先が揃って見える。

#### 欠点

- membership churn と auth refresh / permission update / projection update が混ざりやすい。
- auth layering が room semantics と fairness / authority line に漏れやすい。
- compile-time admission visibility と runtime control-plane の分離が弱くなる。

### 案 2. membership identity core を残し、auth / admission / projection を side carriers に分ける

#### 読み

```text
shared_space_identity_auth_layering_reopen = {
  reopen_kind = current_l2_shared_space_identity_auth_layering,
  entry_criteria_refs = [
    mirrorea_shared_space_docs_first_reentry,
    shared_space_membership_registry_working_boundary
  ],
  membership_identity_core_refs = [
    member_ref,
    principal_ref,
    member_incarnation,
    activation_state
  ],
  auth_admission_side_refs = [
    transport_auth_ref,
    service_auth_ref,
    room_permission_ref,
    admission_policy_ref
  ],
  projection_side_refs = [
    display_projection_ref
  ],
  guard_refs = [
    keep_raw_auth_protocol_outside_room_semantics,
    avoid_embedding_auth_tokens_in_membership_core,
    avoid_collapsing_projection_identity_into_membership_slot
  ],
  kept_later_refs = [
    shared_space_admission_compile_time_visibility,
    shared_space_authority_resource_ownership,
    concrete_auth_protocol_binding
  ]
}
```

#### 利点

- principal continuity と room-local membership slot を残しつつ、auth renewal と admission update を別軸に保てる。
- display / projection identity を principal continuity と同一視せずに済む。
- authoritative room と append-friendly room の両方で audit / blame / capability bridge を説明しやすい。
- next reopen を admission policy / compile-time visibility へ narrow に handoff しやすい。

#### 欠点

- carrier が 1 段増える。
- room profile 側には admission rule と permission ref の接点だけを書き、raw auth protocol を外へ残す discipline が要る。

### 案 3. room core には opaque actor handle しか持ち込まず、identity / auth / projection を外へ出す

#### 利点

- language core から auth details を最もきれいに外せる。
- auth-heavy deployment や external gateway integration と相性が良い。

#### 欠点

- authoritative room で principal continuity と audit connection が弱くなりやすい。
- room-local capability と principal continuity の bridge が別に要る。
- current authoritative room line の `誰が turn を持っていたか` を説明する floor が薄くなる。

## current judgment

current L2 で最も自然なのは、
**案 2. membership identity core を残し、auth / admission / projection を side carriers に分ける**
である。

理由は次の通り。

1. current shared-space line では principal continuity を完全に外へ出すより、membership registry に `principal_ref` を残した方が audit / fairness / capability bridge を説明しやすい。
2. 一方で transport / service auth や room admission を membership carrier に埋め込むと、activation / authority / fairness と同じ carrier に漏れやすい。
3. display / projection identity は principal continuity と同一視せず side carrier に残す方が、platform / world / avatar layering と current repo の separable-track discipline に合う。

## current first choice details

- membership core は `member_ref + principal_ref + member_incarnation + activation_state` に留める。
- transport/session auth、service login、room-local permission、admission policy は auth/admission side refs に置き、room semantics は raw token や protocol detail を見ない。
- display / avatar / projection identity は `display_projection_ref` 側へ押し分け、membership slot の canonical identity にしない。
- authoritative room の join path でも `authority verifies admission policy -> registry creates member core -> room emits players snapshot` の cut に留める。
- compile-time visibility / capability over-approximation と runtime admission / activation reconciliation は次段 reopen に残す。

## next promoted line

next promoted line は、
**shared-space-identity-auth-layering-reopen-ready model-check-concrete-carrier-first-actualization-gate comparison**
に置く。

## open questions

- `display_projection_ref` を membership registry の derived snapshot にどう mirror するか
- admission policy と compile-time visibility over-approximation の bridge をどの declaration floor から reopen するか
- auth-heavy deployment で案 3 の opaque-handle cut をどの later gate に残すか
