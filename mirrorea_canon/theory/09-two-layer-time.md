---
id: theory/09-two-layer-time
status: L2-working
maturity: draft
depends_on: [theory/04-ordering-and-cuts, theory/06-existence-fallback]
summary: 離散検証遷移(occurrence DAG)と高頻度ストリーム(pose 等)の二層時間論。frontier 接続、admissibility、保存則。
open_items: [OPEN-022, OPEN-023]
---

# 09 — Two-layer time (working)

Virtual space mixes two times: **discrete verified transitions** (the
occurrence DAG: joins, writes, grants, cuts — tens per second at most, fully
checked) and **high-rate streams** (pose/audio at 60–120 Hz — unverifiable per
sample at reasonable cost). Collapsing them destroys either performance or
verification. This chapter fixes their connection.

## Model

A stream is a family of samples `σ = (anchor_ref, epoch, seq, payload, t)`.
Samples are **not occurrences**: they never enter H and are not saved
per-sample. What is discrete: anchor creation/switch, stream open/close,
epoch changes, snapshot frontiers.

**Frontier connection.** Each stream is bound to an anchor whose existence and
authority live in the discrete layer. A sample σ is *admissible for
interpretation* at a consumer iff its anchor is live in the consumer's current
consistent frontier F and σ.epoch matches the anchor's epoch in F. Inadmissible
samples are dropped (optionally counted in audit), never buffered into
authority.

**Snapshot rule.** SaveObject may include `pose_snapshot_frontier`: the latest
per-anchor sample *at* a consistent cut. Load admissibility extends theory/04:
reject stale anchor witness, stale fallback position, incoherent anchor
component snapshots; recovery is explicit reacquire (new witness/epoch), not
hidden repair.

**No split frame (working law).** A consumer must not render/consume, within
one interpretation step, samples interpreted against two different frontiers
for anchors that the discrete layer declares atomic together (e.g. two bones
of one avatar). Violations are a devtools-visible row class.

**M4 maintained-relation profile.** For one projected relation,
`PresentationContext` fixes one consumer frontier and contains every required
anchor sample at that frontier and the `BindingState`-recorded anchor epoch.
Missing, old-epoch, or mixed-frontier samples reject semantic relation
evaluation; the consumer may report a local presentation gap instead. The gap
does not mutate the owner-held relation binding, fallback option, authority,
lineage, or occurrence history. This is the finite profile of theory/14, not
the resolution of the general clock/latency questions below.

**Two fallback domains.** Semantic fallback is triggered only by discrete
semantic invalidation of an anchor (existence, membership/incarnation, lease,
authority, or lineage loss). It advances the chain with the ordinary
occurrence/frontier record, and recovery is a fresh reacquire occurrence.
Consumer-local sample loss, latency-budget exhaustion, temporary packet loss,
interpolation, prediction, or LOD is presentation fallback. It may choose a
safe local rendering response but does not advance semantic lineage, create an
occurrence, change authority, or permit a stale anchor sample for semantic use.

## Proof surface

OBL-022: samples cannot influence discrete state except via declared adapter
effects (streams are read-side). OBL-023: frontier-admissibility + no-split-
frame imply per-consumer temporal coherence (formal statement pending).

OPEN-022: clock/latency model (per-consumer logical time vs shared budget).
OPEN-023: whether audio/haptics need a distinct admissibility notion or reuse
anchors as-is.
