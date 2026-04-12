# 358 — current L2 Mirrorea/shared-space docs-first re-entry-ready minimal-Mirrorea/shared-space docs-first re-entry threshold

## 目的

`specs/examples/357-current-l2-parser-checker-runtime-public-surface-inventory-ready-mirrorea-shared-space-docs-first-re-entry-comparison.md`
で Mirrorea/shared-space docs-first re-entry の current first choice を fixed した次段として、

- repo-level re-entry bundle の minimum をどこまでに留めるか
- boundary / adjacent track / user-spec gate / next docs-first reopen line を minimum にどう残すか
- control-plane actualization や operational runtime realization をどこまで still later に残すか

を比較する。

ここで固定するのは
**current L2 Mirrorea/shared-space docs-first re-entry-ready minimal-Mirrorea/shared-space docs-first re-entry threshold**
であり、

- final shared-space catalog
- control-plane separated carrier actualization
- distributed fairness protocol
- Mirrorea operational runtime realization
- upper-layer application finalization

はまだ固定しない。

## 比較観点

1. repo-level re-entry の minimum を lossless に残せるか
2. adjacent subsystem boundary と user-spec-required gate を distinguish できるか
3. next docs-first reopen line を minimum に handoff できるか
4. operational realization を premature に minimum へ混ぜないか

## 比較対象

### 案 1. re-entry title と current boundary 名だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- adjacent subsystem boundary、user-spec gate、next docs-first reopen line が抜ける。
- snapshot drift に弱い。

### 案 2. `reentry_kind + entry_criteria_refs + current_boundary_refs + adjacent_track_boundary_refs + user_spec_required_refs + next_docs_first_reopen_refs + kept_later_refs` を持つ

#### 利点

- repo-level re-entry bundle の cut を lossless に残せる。
- Mirrorea/shared-space と Typed-Effect / Prism / app gate の separation を minimum に反映できる。
- Macro 6 next reopen line と later realization line を同時に guard できる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. final catalog 候補や operational runtime candidate まで minimum に含める

#### 利点

- later line との接続は見えやすい。

#### 欠点

- threshold ではなく future finalization を先取りする。
- self-driven boundary と user decision gate を混ぜやすい。

## current judgment

current L2 で最も自然なのは、
**案 2. `reentry_kind + entry_criteria_refs + current_boundary_refs + adjacent_track_boundary_refs + user_spec_required_refs + next_docs_first_reopen_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は repo-level docs-first re-entry bundle を narrow に fixed することであり、shared-space closeout と adjacent subsystem boundary の接点を minimum に残す必要がある。
2. Typed-Effect / Prism / app gate が minimum に見えないと、old FutureWork bucket 的な誤読が戻りやすい。
3. next docs-first reopen line と kept-later operational realization を分けて残す方が Macro 6 ratchet に合う。

## current first choice shape

```text
mirrorea_shared_space_docs_first_reentry = {
  reentry_kind = macro6_docs_first_boundary_reentry,
  entry_criteria_refs = [
    parser_checker_runtime_public_surface_inventory,
    phase4_shared_space_practical_boundary_checkpoint
  ],
  current_boundary_refs = [
    mirrorea_fabric_boundary,
    shared_space_practical_boundary
  ],
  adjacent_track_boundary_refs = [
    typed_effect_wiring_platform_boundary,
    prismcascade_independent_kernel_boundary
  ],
  user_spec_required_refs = [
    shared_space_final_catalog,
    upper_layer_application_target
  ],
  next_docs_first_reopen_refs = [
    shared_space_identity_auth_layering,
    shared_space_admission_compile_time_visibility,
    shared_space_authority_resource_ownership
  ],
  kept_later_refs = [
    control_plane_separated_carrier_actualization,
    distributed_fairness_protocol,
    mirrorea_operational_runtime_realization
  ]
}
```

## practical reading

current minimal Mirrorea/shared-space docs-first re-entry が示すのは、

- repo-level current boundary track は `mirrorea_fabric_boundary + shared_space_practical_boundary` である
- Typed-Effect Wiring Platform と PrismCascade は adjacent subsystem boundary として保つ
- final shared-space catalog と upper-layer app target は user-spec-required gate に残す
- next docs-first reopen line は identity/auth layering、admission / compile-time visibility、authority / resource ownership に置く
- control-plane actualization、distributed fairness、Mirrorea operational runtime realizationは still later に残す

という最小 cut である。

## next promoted line

next promoted line は、
**minimal-mirrorea-shared-space-docs-first-re-entry-ready model-check-public-checker-second-reserve-inventory comparison**
に置く。

## open questions

- `adjacent_track_boundary_refs` に app line 自体の user-spec gate ref を別 field として切る必要があるか
- `mirrorea_fabric_boundary` を route proof / patch interaction / scaling line まで current re-entry minimum に含めるべきか
- next docs-first reopen refs の順序を current threshold にどこまで固定するか
