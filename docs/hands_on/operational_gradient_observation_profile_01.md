# Operational Gradient Observation Profile 01

この guide は、operational suite の portal/shard future line における **gradient observation profile** を docs-first / profile-first に読むための入口です。

これは runtime widening ではありません。current active runtime root は `samples/product-alpha1/operational/two-shard-hard-boundary/` の bounded hard-authority cut のままであり、gradient observation は `planned_only` profile に留まります。

## Read The Future Profiles

```bash
python3 -m json.tool samples/product-alpha1/operational/future/spatial-shard-future.profile.json
python3 -m json.tool samples/product-alpha1/operational/future/gradient-observation.profile.json
sed -n '1,160p' samples/product-alpha1/operational/future/two-shard-hard-boundary/README.md
```

Expected bounded evidence:

- `spatial-shard-future.profile.json` reports `gradient_observation_status = planned_profile_present`
- `gradient-observation.profile.json` reports `current_status = planned_only`
- `active_runtime_root = ../two-shard-hard-boundary`
- each `gradient_zones[]` row keeps `write_capability = false`
- `freshness_requirements.vector_clock_default = false`
- `replication_profile_requirement.gradient_requires_replication_profile = false`

## What The Profile Fixes

- gradient observation is **observer-only** and does not weaken the hard-authority first cut
- overlap zones may show bounded ghost/presence views, but they do not grant write capability
- freshness is carried by `membership_epoch`, `member_incarnation`, `config_epoch`, `owner_epoch`, and `sequence`
- replication profile remains optional future work; gradient observation does not force CRDT/vector-clock default adoption

## Current Non-Claims

- no gradient observation runtime
- no continuous spatial synchronization
- no WAN federation
- no distributed durable save/load
- no default CRDT / vector-clock replication
