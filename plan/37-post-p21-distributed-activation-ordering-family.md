# plan/37 — post-P21 distributed activation ordering family

## role

この文書は、`P21` runtime-crate hot-plug completed-engine narrow cut の後に残る
later family のうち、
**`distributed activation ordering` を second recommendation として読む current repository memory**
を置く。

- exact package label / numbering はここでも固定しない
- actual distributed activation ordering completion を claim しない
- `rollback / durable migration` first recommendation family は
  `plan/36-post-p21-rollback-durable-migration-family.md`
  で close 済みとして前提化する
- final public hot-plug ABI を同じ tranche に collapse しない

## fixed preconditions

- `P21` は close 済みであり、
  `crates/mir-runtime/src/hotplug_runtime.rs` に
  `HotPlugRuntimeEngineState`
  `HotPlugRuntimeEngineReport`
  `assemble_hotplug_runtime_engine_report()`
  `build_hotplug_runtime_engine_report()`
  が actualize 済みである
- `plan/36-post-p21-rollback-durable-migration-family.md`
  により、
  `rollback / durable migration` family は first recommendation として
  docs-first に harden 済みである
- current helper-local boundary matrix は
  `plan/32-hotplug-real-migration-rollback-boundary.md`
  にあり、
  `activation_cut != distributed activation ordering`
  `migration_contract row != protocol`
  rejected `detached_roll_request#1 != completed rollback`
  を already fixed している
- current helper-local attach anchor は
  `attach_request#1`
  `attach_lifecycle`
  `attach_activation#1`
  `hotplug_activation_boundary`
  を持つが、
  それ自体は distributed ordering proof ではない

## why this family is second

- `rollback / durable migration` family を first recommendation として切り出した後に残る
  independent widening seam は
  attach activation の visibility frontier と commit ordering である
- この seam は
  `migration_contract` honesty row よりも
  `activation_cut` と `hotplug_activation_boundary` の widening criteria に依存する
- multi-place / multi-server attach activation ordering、
  durable activation commit、
  visibility frontier の確定は
  rollback / reattach semantics と同一 validation floor では読み切れない
- final public hot-plug ABI は
  post-`P18` mixed gate / `U1` hold line への依存を持つため、
  third recommendation に残す

## family scope

この family で先に整理するのは次である。

- `activation_cut` から distributed activation ordering family へ widen する criteria
- multi-place / multi-server attach activation ordering を
  current helper-local attach cut とどう切り分けるか
- durable activation commit と
  visible active state frontier の relation
- membership epoch / capability / witness / auth lane を
  transport collapse なしで ordering family に結びつける current boundary
- runtime-private ordering evidence と
  final public request/response/event naming を分ける理由

この family でまだ fixed しないのは次である。

- actual rollback protocol completion
- actual durable migration engine completion
- actual distributed activation ordering implementation
- actual consensus / network commit protocol
- final public hot-plug ABI / package catalog naming
- helper-local anchor naming の runtime-canonical or public 化

## keep-as-one-family vs split-again criteria

| question | keep as one family when | split again when |
|---|---|---|
| local activation cut vs distributed ordering | same visibility frontier / witness family / failure lane で議論できる | local activation cut と cross-place commit ordering が別 validation floor を要求する |
| distributed ordering vs durable activation commit | ordering proof と commit frontier が同じ emitted/runtime trace family で読める | commit acknowledgement / replay / consensus witness が standalone carrier を要求する |
| runtime-private names vs public names | runtime-private state / witness / reason family のまま読める | public request/response/event / package catalog naming を決める必要が出る |

## current recommendation

- exact post-`P21` package label は intentionally unfixed のまま保つ
- `rollback / durable migration` family hardening は
  first recommendation close 済みとして扱う
- second recommendation closeout memory は
  **`distributed activation ordering` family hardening**
  と読む
- third recommendation family の current repository memory は
  **`final public hot-plug ABI` family**
  として
  `plan/38-post-p21-final-public-hotplug-abi-family.md`
  を参照する
- `AttachPoint` / `Patch` packaging identity、
  helper-local sample IDs / view IDs / telemetry IDs、
  runtime-private state naming は
  second recommendation family でも public ABI に昇格させない

## validation floor

- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## stop line

- `activation_cut` を distributed activation ordering completion と書かない
- helper-local `local_queue` attach sample を
  multi-server consensus / durable commit proof と書かない
- distributed activation ordering family を
  rollback / durable migration completion と混同しない
- authentication / authorization / membership / capability / witness を
  transport に collapse しない
- public request/response/event naming を
  second recommendation family の名目で freeze しない

## related memory

- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/22-network-transport-roadmap.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- `plan/32-hotplug-real-migration-rollback-boundary.md`
- `plan/35-post-p20-hotplug-next-package-inventory.md`
- `plan/36-post-p21-rollback-durable-migration-family.md`
- `plan/38-post-p21-final-public-hotplug-abi-family.md`
