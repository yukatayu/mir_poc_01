# 366 — current L2 shared-space-identity-auth-layering-reopen-ready minimal-shared-space-identity-auth-layering-reopen threshold

## 目的

`specs/examples/365-current-l2-public-operational-surface-actualization-gate-ready-shared-space-identity-auth-layering-reopen-comparison.md`
で shared-space identity / auth layering reopen の current first choice を fixed した次段として、

- identity/auth layering reopen の minimum をどこまでに留めるか
- membership core / auth-admission side / projection side / guard を minimum にどう残すか
- next promoted line と kept-later shared-space follow-up をどう handoff するか

を比較する。

ここで固定するのは
**current L2 shared-space-identity-auth-layering-reopen-ready minimal-shared-space-identity-auth-layering-reopen threshold**
であり、

- admission policy / compile-time visibility reopen の具体形
- authority / resource ownership reopen の具体形
- concrete auth protocol binding
- Mirrorea operational runtime realization

はまだ固定しない。

## 比較観点

1. shared-space identity/auth boundary cut を lossless に minimum へ残せるか
2. membership core と auth/admission/projection の separation を minimum に保てるか
3. next reopen を admission policy / compile-time visibility と model-check concrete carrier gate に handoff できるか
4. raw auth protocol や final catalog を premature に minimum へ混ぜないか

## 比較対象

### 案 1. membership core 名と identity/auth reopen title だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- auth/admission side と projection side の separation が見えない。
- raw auth protocol を room semantics へ持ち込まない guard が弱い。

### 案 2. `reopen_kind + entry_criteria_refs + membership_identity_core_refs + auth_admission_side_refs + projection_side_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- membership core と auth/admission/projection の cut を lossless に残せる。
- principal continuity を残しつつ auth layering を side carrier に押し分ける current lineを minimum に mirror できる。
- next reopen と kept-later shared-space/runtime line を同時に guard できる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. admission policy / compile-time visibility shape まで threshold に含める

#### 利点

- next line との接続は見えやすい。

#### 欠点

- threshold ではなく次段 reopen を先取りする。
- identity/auth layering package 自体の minimum を太らせる。

## current judgment

current L2 で最も自然なのは、
**案 2. `reopen_kind + entry_criteria_refs + membership_identity_core_refs + auth_admission_side_refs + projection_side_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は shared-space identity/auth の first boundary cut を固定することであり、room-local membership / principal continuity / auth layering / projection layering を minimum に分けて残す必要がある。
2. raw auth protocol と concrete admission realization を still later に残す guard が見えないと snapshot drift を起こしやすい。
3. next reopen は admission policy / compile-time visibility 側だが、repo-level next promoted line 自体は model-check concrete carrier gate に handoff すれば十分である。

## current first choice shape

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
    concrete_auth_protocol_binding,
    model_check_concrete_carrier_first_actualization_gate
  ]
}
```

## practical reading

current minimal shared-space identity/auth layering reopen が示すのは、

- membership registry 側には principal continuity と room-local slot だけを残す
- transport / service auth、room-local permission、admission policy は side carrier に残す
- display / projection identity は別軸に残し、membership slot と同一視しない
- concrete auth protocol、admission/visibility exact shape、authority/resource ownership は still later に残す

という最小 cut である。

## next promoted line

next promoted line は、
**minimal-shared-space-identity-auth-layering-reopen-ready model-check-concrete-carrier-first-actualization-gate comparison**
に置く。

## open questions

- `membership_identity_core_refs` に derived `display_snapshot_ref` を足すべきか、それとも projection side に留めるべきか
- admission policy / compile-time visibility reopen を current minimum の kept-later にどこまで mirror するか
- append-friendly room で principal continuity をさらに薄くする later comparison をどこへ置くか
