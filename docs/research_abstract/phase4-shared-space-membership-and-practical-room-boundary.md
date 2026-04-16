# Phase 4 要約 — shared-space / membership と practical room boundary

## 何をした phase か

Phase 4 は、shared-space / membership line を
**language core の finalization に押し込まず、practical room boundary と autonomy gate を docs-first に整理する phase**
である。

主に整えたのは次である。

- authoritative room baseline
- small cross-room working subset catalog
- `auditable_authority_witness` minimal witness core
- authoritative room 側 delegated provider placement の practical cut
- control-plane separated causal carrier の reopen threshold

## current self-driven package

self-driven current package として source-backed に読めるのは次である。

- `specs/examples/121-shared-space-authoritative-room-baseline.md`
- `specs/examples/122-shared-space-catalog-working-subset-comparison.md`
- `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md`
- `specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md`
- `specs/examples/125-shared-space-control-plane-carrier-threshold.md`

ここでの current package は、

- authoritative room baseline
- append-friendly room baseline
- append-friendly optional RNG capability row
- witness core
- provider placement
- control-plane threshold

までを含む。

## closeout で固定した current reading

Phase 4 self-driven closeout では、current package を次の cut で fixed した。

- self-driven current package は `121...125` までで checkpoint close と読んでよい
- final activation / authority / auth / identity / admission / consistency / fairness catalog は user-spec-required final catalog に残す
- `append_friendly_room_with_rng_capability` row は optional capability source を current row に残すが、`delegated_provider_attestation` は room-core claim に固定せず later candidate に残す
- control-plane separated carrier actualization、distributed fairness protocol、final operational realizationは retained-later に残す

## まだ未決のもの

- final activation overlay catalog
- final authority / auth / identity / admission catalog
- final consistency / fairness catalog
- control-plane separated carrier actualization
- distributed fairness protocol
- final operational realization

## 次 phase へ渡したもの

Phase 4 closeout fixed 時点での handoff 先は、
Phase 5 proof / protocol / runtime-policy handoff closeout である。
一方で shared-space line 自体は、final catalog が user specification を要する phase として残る。

その後の repo-level follow-up では、
`specs/examples/357...358` により shared-space line を old `FutureWork` bucket へ戻さず、
Mirrorea/shared-space docs-first re-entry bundle として再配置した。
ここで current core bundle は `mirrorea_fabric_boundary + shared_space_practical_boundary`、
Typed-Effect / Prism は adjacent track、
shared-space final catalog と upper-layer app target は user-spec-required gate に残す。
その後 `specs/examples/365...366` により identity / auth layering reopen、`specs/examples/373...374` により admission / compile-time visibility reopen、`specs/examples/375...376` により authority / resource ownership reopen、`specs/examples/424` により room-profile / host-binding bridge-only note も docs-first boundary package として fixed 済みであり、shared-space docs-first follow-up checkpoint はここで一旦 close と読むのが current reading である。shared-space line はここから先、fairness / replay strengthening reserve と final operational catalog gate を残す adjacent boundary line として扱うのが自然である。
