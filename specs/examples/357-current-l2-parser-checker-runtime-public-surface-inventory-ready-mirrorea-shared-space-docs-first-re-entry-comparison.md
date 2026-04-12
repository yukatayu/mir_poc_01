# 357 — current L2 parser-checker-runtime-public-surface-inventory-ready Mirrorea/shared-space docs-first re-entry comparison

## 目的

`specs/examples/356-current-l2-parser-checker-runtime-public-surface-inventory-ready-minimal-parser-checker-runtime-public-surface-inventory-threshold.md`
で parser / checker / runtime public surface inventory の minimum を fixed した次段として、

- repo-level current line を old `FutureWork` bucket へ戻さずに Macro 6 docs-first boundary track としてどう再入場させるか
- Mirrorea / shared-space / Typed-Effect / Prism / app line をどこで分けるのが current first choice か
- final shared-space catalog や operational runtime realization をどこまで still later に残すべきか

を比較する。

ここで固定するのは
**current L2 parser-checker-runtime-public-surface-inventory-ready Mirrorea/shared-space docs-first re-entry comparison**
であり、

- final shared-space catalog
- control-plane separated carrier actualization
- distributed fairness protocol
- Mirrorea operational runtime / path-proof / consensus realization
- upper-layer application finalization

はまだ固定しない。

## scope

- current package は docs-only boundary / roadmap package に留める。
- Phase 4 shared-space self-driven closeout (`specs/examples/121...125`, `295...296`) を current shared-space package entry criteria として読む。
- Mirrorea / PrismCascade / Typed-Effect Wiring Platform / app line を separable track として扱い、同一 runtime へ潰さない。
- actual runtime control-plane、final auth / admission catalog、concrete app goal は still later に残す。

## current 前提

current repo では次が成立している。

1. `specs/03-layer-model.md` は L2 Mirrorea fabric と L3 shared-space を分離している。
2. `specs/05-mirrorea-fabric.md` は Mirrorea の責務を logical naming / routing / overlay / patch / audit / scaling / reconfiguration に置いている。
3. `specs/08-cross-system-relations.md`、`specs/06-prismcascade-positioning.md`、`specs/07-typed-effects-wiring-platform.md` は Mirrorea / Prism / Typed-Effect を未分化 runtime に潰さない current boundary を already 持っている。
4. shared-space 側の current self-driven package は `specs/examples/121...125` と `295...296` により checkpoint-close 済みであり、final activation / authority / auth / identity / admission / consistency / fairness catalog は user-spec-required、control-plane actualization / distributed fairness / final operational realizationは retained-later と読める。

したがって current 問いは、
**repo-level current line を Mirrorea/shared-space docs-first re-entry と呼ぶなら、どの narrow bundle で再入場させるのが自然か**
である。

## 比較観点

1. shared-space closeout package をそのまま entry criteria として再利用できるか
2. Mirrorea / shared-space / Typed-Effect / Prism / app を 1 bucket に潰さずに整理できるか
3. final shared-space catalog と app target の user-spec gate を current boundary bundle から分離できるか
4. next docs-first reopen line を identity/auth layering などへ narrow に handoff できるか

## 比較対象

### 案 1. repo-level re-entry bundle を作らず、shared-space closeout と old FutureWork bucket の間に置く

#### 利点

- source docs は増えない。
- Phase 4 closeout wording 自体は維持できる。

#### 欠点

- repo-level current line が何を再開しているかが prose 依存になる。
- Mirrorea / shared-space / Typed-Effect / Prism / app line が再び 1 bucket に見えやすい。
- user-spec-required gate と self-driven boundary が snapshot で drift しやすい。

### 案 2. Mirrorea/shared-space docs-first re-entry bundle を切る

#### 読み

repo-level re-entry bundle は、Mirrorea / shared-space を current Macro 6 boundary track として再開しつつ、
adjacent subsystem と user-spec gate を分けて持つ。

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

#### 利点

- shared-space closeout package と repo-level current line の接点を 1 段で見せられる。
- Typed-Effect / Prism / app line を Mirrorea/shared-space と同一 runtime bucket にせずに済む。
- user-spec-required final catalog と self-driven next reopen line を同時に明示できる。
- Macro 6 next reopen を identity/auth layering などへ narrow に handoff しやすい。

#### 欠点

- field は少し増える。
- Mirrorea 自体の operational realization は still later に残る。

### 案 3. final shared-space catalog と Mirrorea operational realization を current re-entry bundle に含める

#### 利点

- repo-level current line は強く見える。

#### 欠点

- self-driven boundary と user-spec-required gate を混ぜる。
- control-plane actualization / consensus / path-proof realization を premature に current package へ引き込む。
- Typed-Effect / Prism / app line の separable track 読みを壊しやすい。

## current judgment

current L2 で最も自然なのは、
**案 2. Mirrorea/shared-space docs-first re-entry bundle を切る**
である。

理由は次の通り。

1. shared-space current package は Phase 4 closeout で既に checkpoint-close 済みなので、repo-level current line には closeout package を再拡張するより re-entry bundle の方が自然である。
2. Mirrorea / shared-space / Typed-Effect / Prism / app line を separable track として並べるのが `specs/05` / `06` / `07` / `08` の current boundary と整合する。
3. final shared-space catalog や upper-layer app target は user-spec-required gate に残し、Macro 6 next reopen は docs-first boundary line に留める方が ratchet に合う。

## current first choice details

- current boundary bundle は `mirrorea_fabric_boundary + shared_space_practical_boundary` を core にする。
- Typed-Effect Wiring Platform と PrismCascade は adjacent subsystem boundary として明示するが、current Macro 6 core bundle には混ぜない。
- final shared-space catalog と upper-layer app target は user-spec-required gate に置く。
- next docs-first reopen line は `shared_space_identity_auth_layering -> shared_space_admission_compile_time_visibility -> shared_space_authority_resource_ownership` の順で読む。
- control-plane separated carrier actualization、distributed fairness protocol、Mirrorea operational runtime realization は still later に残す。

## next promoted line

next promoted line は、
**mirrorea-shared-space-docs-first-re-entry-ready model-check-public-checker-second-reserve-inventory comparison**
に置く。

## open questions

- `mirrorea_fabric_boundary` を repo-level re-entry bundle でどこまで narrow に mirror するか
- Macro 6 の next docs-first reopen line を identity/auth layering から始める current order をどこまで snapshot に昇格するか
- Mirrorea operational runtime / path-proof / route-change realization をどの reopen point まで boundary-only に留めるか
