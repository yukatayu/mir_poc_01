# Shard Missing Witness Negative

Negative `P-FSV1-02` row for TwoShardHardBoundary.

- emits observer-visible old-owner and stale-config reject events before commit
- attempts commit handoff without a live witness
- must fail as `missing_live_witness`; old-owner/stale-config are narrated reject events, not separate enforced failure reasons in this row
