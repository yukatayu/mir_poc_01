# plan/16 — shared-space membership と example boundary

## current working boundary

- authoritative-room minimal working subset は current docs-first line に含めてよい
- exhaustive shared-space catalog はまだ final public line に上げない
- membership / authority / provider placement / witness requirement / replay / fairness は分けて扱う
- `Place` は participant と同一ではなく、queue / state / capability / visibility / observation frontier を持つ execution locus として読む
- current Sugoroku sample の `world` は host/server-side sugar として読み、Mir core primitive だと既成事実化しない
- current Sugoroku helper closeout では `world_surface.surface_role = host_server_side_sugar`、`world_surface.mir_core_status = not_a_primitive` を explicit に保ってよい

## active example boundary

- active order / handoff examples は `samples/clean-near-end/order-handoff/`
- mutex / weak-memory / broken mutex は `samples/clean-near-end/model-check/`
  に分ける
- Sugoroku world runtime attachment example は `samples/clean-near-end/sugoroku-world/`
  に置き、single OS process logical multi-place emulator として扱う
- Sugoroku 側の participant set は fixed literal principal だけでなく `MembershipRegistry` から読む line を current reading に置く
- current Sugoroku helper closeout では `membership_model.source_of_truth = MembershipRegistry` と `membership_model.late_join_handoff_boundary` を explicit に保ってよい
- `P11` current second cut では `MembershipRegistry` typed source-of-truth substrate、`PlaceCatalog`、participant-place-kind-gated `LogicalPlaceRuntimeShell` を `crates/mirrorea-core` に actualize してよいが、`WorldState` / `PlaceRuntime` / `MessageQueue` / `SugorokuState` / event-timeline-view catalog は helper-local evidence surface または later tranche に残してよい
- avatar fairy follow representative slice は `samples/clean-near-end/avatar-follow/` に置き、active canonical sample として読む
- phase 8 residual planned family は `samples/not_implemented/avatar-fairy-follow/` に置き、historical prototype anchor は `samples/prototype/current-l2-dynamic-attach-detach/` に残す
- phase 9 planned source family は `samples/not_implemented/typed-external-boundary/` に置き、current synthetic preview helper subset は `EXT-03` / `EXT-04` に留める
- phase 9 residual planned family は `samples/not_implemented/typed-external-boundary/` に置き、current evidence anchor は `05_delegated_rng_service` の `provider_boundary` と Sugoroku helper の `local_queue` envelope / visualization carrier に結び直す
- phase 13 planned family は `samples/not_implemented/network-transport/` に置き、current evidence anchor は Sugoroku helper の `local_queue` envelope と clean near-end `provider_boundary` に結び直す

## current judgment

- thread / node を同じ causal language で書いてよい
- ただし lowering / transport / failure / durability / policy は分ける
- authoritative room を shared-space 全体の exhaustive default と同一視しない
- authentication を transport に潰さない
- helper-local current cut では `auth none` baseline、`membership_epoch` / `member_incarnation` freshness、`witness_refs` separate lane を visible にしてよい
- visualization / telemetry を untyped debug leak にしない
- effect-based OS-like substrate という内側の解釈を使っても、standard I/O core built-in 化や subsystem collapse は行わない
- verification / visualization / telemetry は `VerificationLayer` composition の current candidate として typed lane を保つ
- standard I/O を Mir core primitive にせず、typed external boundary の planned familyは adapter / transport / auth / witness / visualization を separate lane として保つ
- `atomic_cut` を room-level durable commit にしない
- system-wide source から place-specific program へ projection できる性質を future line の invariant として保つ
- current docs-first plan は `plan/20-projection-and-placement-roadmap.md` に置き、generator / optimizer / equivalence checker は still later に残す
- `SUG-01` attach と `SUG-09` detach TODO boundary は hot-plug docs-first plan の current anchor であり、storage detach script と混同しない
- current Sugoroku helper closeout では `hotplug_stop_line.detach_boundary = explicit_todo_boundary`、`rollback_protocol = deferred`、`durable_migration_engine = deferred`、`final_public_hotplug_abi = deferred` を explicit に保ってよい
- transport widening current plan は `plan/22-network-transport-roadmap.md` に置き、loopback / reconnect / failure matrix を helper-local baseline と分けて読む

## still later

- exhaustive shared-space catalog
- final witness / provider public contract
- final replay taxonomy
- distributed fairness theorem
- avatar fairy follow residual widening
- real network transport / multi-server consensus / durable distributed commit
- projection generator / placement optimizer
- detach lifecycle implementation
- `AttachPoint` compatibility / migration actualization
