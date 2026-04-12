# 325 — current L2 verification-ladder-wiring-ready source-sample-authoring-bless-regression-policy comparison

## 目的

`specs/examples/323-current-l2-syntax-backed-sample-runner-first-cut-ready-verification-ladder-wiring-comparison.md`
と
`specs/examples/324-current-l2-verification-ladder-wiring-ready-minimal-verification-ladder-wiring-threshold.md`
で first authored trio の verification ladder wiring を fixed した次段として、

- source sample をどの flow で authored / deferred に保つか
- regression command をどこまで repo-local helper に束ねるか
- bless をどこまで current package で意味づけるか

を比較する。

ここで固定するのは
**current L2 verification-ladder-wiring-ready source-sample-authoring-bless-regression-policy comparison**
であり、

- public runner / public CLI
- retained detached artifact bless / archive policy
- deferred authored row `e1` / `e3` / `e21` の widen
- theorem/model-check concrete tool choice

はまだ固定しない。

## scope

- entry criteria は `specs/examples/315...324` で fixed 済みの source corpus scope / mapping / lowering / runner / ladder に置く。
- current authored source sample は `e2` / `e4` / `e23` first trio に留める。
- current bless は final retained artifact 更新ではなく、**repo-local source / fixture / docs mirror を reviewed 状態へ更新し、current regression bundle を通すこと**を意味する narrow working term とする。

## current 前提

current repo では次が成立している。

1. `samples/current-l2/README.md` は current authored first trio と deferred source target only row を明示している。
2. `mir_runtime::current_l2::run_current_l2_source_sample` と `current_l2_source_sample_verification_ladder.rs` は first authored trio の runner / reached-stage floor を machine-check している。
3. detached formal-hook smoke は `scripts/current_l2_detached_loop.py` にあり、current authored trio の runtime/static formal-hook route は既存 helper 群で回せる。
4. まだ final public runner surface も retained artifact bless/update policy もない。

したがって current 問いは、
**manual review を主に保ちつつ repo-local helper で regression bundle を narrow actualize し、bless を source/fixture/docs mirror の reviewed sync として読むのが自然か**
である。

## 比較観点

1. current authored trio の更新手順を lossless に読めるか
2. deferred source target row を premature に widen しないか
3. regression bundle を repo-local に実行しやすくできるか
4. public CLI / retained artifact policy / concrete tool choice を premature に混ぜないか

## 比較対象

### 案 1. manual policy + repo-local regression helper に留める

#### shape

```text
source_sample_authoring_policy = {
  policy_kind = current_l2_first_trio_repo_local_authoring_policy,
  authored_rows = [
    e2_try_fallback,
    e4_malformed_lineage,
    e23_malformed_try_fallback_missing_fallback_body
  ],
  deferred_rows = [
    e1_place_atomic_cut,
    e3_option_admit_chain,
    e21_try_atomic_cut_frontier
  ],
  authoring_steps = [
    update_or_add_source_text,
    keep_fixture_mapping_aligned,
    keep_readme_matrix_and_ladder_aligned,
    rerun_repo_local_regression_bundle,
    record_report_and_snapshot_drift
  ],
  regression_helper_ref = current_l2_source_sample_regression_helper,
  bless_reading = reviewed_repo_local_sync_without_retained_artifact_policy,
  guard_refs = [
    keep_first_trio_as_current_authored_set,
    avoid_public_cli_premature_surface,
    avoid_retained_artifact_policy_premature_merge,
    avoid_concrete_tool_binding_backpressure
  ]
}
```

#### 利点

- current authored trio の更新手順を narrow に固定できる。
- regression bundle を毎回手で拾い直さずに済む。
- bless を final retention/path policy と切り離せる。

#### 欠点

- final public CLI や retained artifact update flow まではまだ届かない。
- source text と fixture の双方向自動変換は依然として持たない。

### 案 2. public CLI / auto-bless command を current package に含める

#### 利点

- command surface は見えやすい。

#### 欠点

- public surface、retention/path policy、artifact bless/update semantics を premature に固定しやすい。
- current first trio だけの narrow packageを超える。

### 案 3. policy を docs-only に留め、regression helper も足さない

#### 利点

- 実装差分は最小。

#### 欠点

- repo-local 実務 flow が弱く、authoring task ごとに command bundle を拾い直す必要がある。
- drift suppression を current package で支えにくい。

## current judgment

current L2 で最も自然なのは、
**案 1. manual policy + repo-local regression helper に留める**
である。

理由は次の通り。

1. current package は source-sample path の実務 flow を narrow に固定する task であり、public CLI や retained bless/update policy まで持ち込む task ではない。
2. current authored set は first trio であり、widen timing は still later に残す方が境界が明確である。
3. detached loop / formal hook / runner tests は already available なので、thin helper で regression bundle を束ねるのが最小である。

## current first choice details

- source sample を追加・更新するときは、source text / fixture mapping / README matrix / ladder row / snapshot docs を同じ task で更新する。
- `bless` の current reading は、review 済み repo-local sync と regression success の確認であり、retained detached artifact archive や public bless command ではない。
- regression helper は current authored trio の inventory check と regression bundle 実行だけを担い、public runner surface や widen timing を内包しない。

## open questions

- later widen task で `e1` / `e3` / `e21` をどの順で authored row に移すか
- retained detached artifact bless/update semantics をどの phase で reopen するか
- theorem-first concrete tool pilot と source-sample policy の交点をどこまで docs に反映するか
