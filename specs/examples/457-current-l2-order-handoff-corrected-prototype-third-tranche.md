# 457 — current L2 order/handoff corrected prototype third tranche

## 目的

order/handoff adequacy corpus のうち、

- late join visibility
- stale reconnect refresh

を current L2 corrected prototype として runnable にし、
publication / handoff / rollback / observation の差を sample-visible にする。

## current judgment

1. order/handoff corrected prototype third tranche の current first cut として、
   - `p07-dice-late-join-visible-history`
   - `p08-dice-stale-reconnect-refresh`
   を追加してよい。
2. `p07` は
   - publication
   - `atomic_cut`
   - handoff
   - late join observation
   を 1 本の runtime-reaching prototype に置く current cut である。
3. `p08` は
   - stale reconnect failure
   - rollback
   - refresh fallback
   を 1 本の runtime-reaching prototype に置く current cut である。
4. current tranche は、
   publication / handoff / replay / stale-message family を final semantics として凍らせるものではなく、
   current L2 subset で何を runnable comparison に落とせるかを示す。
5. current tranche close 後、promoted self-driven package は `0` に戻してよい。

## current non-goals

- fairness / replay operational profile を final catalog に上げること
- late join / reconnect semantics を shared-space final contract として固定すること
- snapshot-only cut / durable cut family の final source surface を決めること
