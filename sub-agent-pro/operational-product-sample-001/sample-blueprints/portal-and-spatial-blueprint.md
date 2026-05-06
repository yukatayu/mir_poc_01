# portal and spatial blueprint

## portal-worldlink package sketch

```json
{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "ops.portal-worldlink",
  "package_version": "0.1.0",
  "package_kind": "portal_worldlink",
  "current_status": "planned_manifest_only",
  "provided_surfaces": ["portal.resolve", "portal.handoff", "portal.admit"],
  "effects": ["PortalResolve", "PortalHandoff", "PortalAdmit"],
  "failures": ["DestinationUnavailable", "PortalAdmissionRejected", "MissingCapability", "MissingWitness", "RouteUnavailable"],
  "capabilities": ["UsePortal", "AdmitPortalTraveler"],
  "contracts": [
    {
      "surface": "portal.handoff",
      "preconditions": ["UsePortal capability", "destination reachable", "handoff witness if required"],
      "postconditions": ["traveler admitted or explicit failure"]
    }
  ],
  "non_claims": [
    "no WAN federation",
    "no continuous spatial sync",
    "no final portal ABI"
  ]
}
```

## two-shard future profile

```json
{
  "spatial_profile_version": "ops-spatial-shard-v0",
  "current_status": "planned_only",
  "space": {
    "kind": "integer_chunk_grid",
    "dimensions": 2
  },
  "shards": [
    {
      "shard_id": "ShardA",
      "authority_region": "x < 0",
      "observation_region": "x < 16",
      "config_epoch": 0
    },
    {
      "shard_id": "ShardB",
      "authority_region": "x >= 0",
      "observation_region": "x >= -16",
      "config_epoch": 0
    }
  ],
  "ownership": {
    "default_profile": "SingleOwnerSequence",
    "object_stamp": ["owner_shard", "owner_epoch", "sequence"]
  },
  "replication_profiles": {
    "current": ["SingleOwnerSequence", "OwnerEpochSequence"],
    "future_optional": ["CRDTJoinSemilattice", "DottedVersionVector", "IntervalTreeClock"]
  },
  "non_claims": [
    "no global participant vector clock default",
    "no continuous infinite federation implementation",
    "no WAN federation",
    "no distributed durable save/load"
  ]
}
```

## model-check properties to plan

```json
{
  "properties": [
    "no_double_owner_after_handoff",
    "old_owner_write_rejected_after_commit",
    "missing_handoff_witness_rejected",
    "stale_config_epoch_rejected",
    "observer_ghost_has_no_write_capability"
  ]
}
```
