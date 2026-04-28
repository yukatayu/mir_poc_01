# 0994 — post-P21 rollback / durable migration family first recommendation

## Objective

`P21` close 後の hot-plug later family のうち、
exact package label を fixed せずに
`rollback / durable migration` を current self-driven first recommendation として harden する。

## Scope and assumptions

- scope は docs-first family hardening に限定する。
- runtime code は増やさない。
- distributed activation ordering は second recommendation に残す。
- final public hot-plug ABI / package catalog は post-`P18` mixed gate / `U1` hold line の third recommendation に残す。
- helper-local anchor naming を runtime-canonical or public names に昇格させない。

## Documents consulted

- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- `plan/32-hotplug-real-migration-rollback-boundary.md`
- `plan/35-post-p20-hotplug-next-package-inventory.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md`
- `docs/hands_on/hotplug_real_migration_rollback_boundary_01.md`
- `docs/hands_on/current_phase_closeout_01.md`

## Actions taken

1. `plan/36-post-p21-rollback-durable-migration-family.md` を新設し、`P21` close 後の first recommendation family、keep-one-family vs split-again criteria、validation floor、stop line を repository memory として固定した。
2. reader-facing summary / landing page として `docs/research_abstract/post_p21_rollback_durable_migration_family_01.md` と `docs/hands_on/post_p21_rollback_durable_migration_family_01.md` を追加した。
3. `plan/32`、`plan/35`、`progress.md`、`tasks.md`、`samples_progress.md`、`plan/01`、`plan/11`、`plan/17`、`specs/10`、`specs/11`、`docs/hands_on/current_phase_closeout_01.md` に current self-driven first recommendation を mirror した。
4. `R4` closeout memory の reader-facing docs を historical-next wording と current repo state に切り分けた。

## Evidence / outputs / test results

- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  - pass
  - `activation_cut` is still a request-vs-visible-state split and not ordering proof
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  - pass
  - rejected `detached_roll_request#1` and `migration_contract.status = deferred` remain explicit
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass
  - `hotplug_validation_floor` remains helper-local attach/detach lifecycle evidence only
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton`
  - pass
  - 8/8 green
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
  - `Found 992 numbered report(s).`
- `git diff --check`
  - pass

## What changed in understanding

- `P21` close の後は vague な grouped later family だけでは weak であり、first recommendation / second recommendation / third recommendation の順序だけでも current repo memory に置く価値がある。
- rollback と durable migration はまだ one-family recommendation に保てるが、その条件は state frontier / witness need / failure lane が共有できるかで読むのが自然である。
- `AttachPoint` / `Patch` naming を early に public/package-catalog へ持ち上げるより、まず runtime-private family と mixed-gate family を分け続ける方が drift を防げる。

## Open questions

- rollback と durable migration / reattach semantics をいつ split-again すべきかは未決。
- distributed activation ordering を standalone family に保つか、durable activation commit family へ吸収するかは未決。
- public request/response/event naming、package catalog breadth、host target、first shipped public surface scope は引き続き user-spec hold に残る。

## Suggested next prompt

post-`P21` later family の second recommendation として `distributed activation ordering` を docs-first に harden し、`activation_cut` から何がまだ証明されておらず、multi-place / multi-server ordering family へ widen するために何が必要かを `plan/37` 相当の repository memory と reader-facing docs に整理してください。final public ABI は同じ tranche に混ぜないでください。
