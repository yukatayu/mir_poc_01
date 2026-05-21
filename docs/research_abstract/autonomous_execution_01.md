# Autonomous Execution Line

## summary

`P-COMP-00` made the main drift explicit: Product Alpha-1 is a useful operational floor, but current `AddOne` is typed external host-boundary evidence, not Mir-owned computation.

The autonomous line now has enough structure to run without asking the user for intermediate decisions. It does so by separating lower-layer implementation from final-product choices.

## execution chain

The chain is:

```text
Mir-owned computation scaffold
  -> PoseGraph scaffold
  -> no-split-frame evidence
  -> projection target / packet / FFI inventory
  -> engine / WASM / FFI provider contract
  -> front-half closeout
  -> pure AddOne in Mir
  -> variables / arrays / records / control-flow
  -> effect boundary around computation
  -> no-split-frame runtime evidence
  -> all-up closeout
```

Each package must close with positive/negative evidence when behavior is claimed, docs synchronization, report, validation, commit, and push.

`P-COMP-01` creates planned-only sample/helper surfaces. Runtime evidence starts at `P-COMP-02`, using a narrow `mir-semantics` computational module rather than the existing adapter-owned `AddOne` lane.

## user decisions

The following remain user-spec-required but do not block the autonomous line:

- final public distribution
- final shared-space catalog breadth
- final grammar / ABI / SDK
- hosted service / production WAN
- backend realization beyond inventory
- bounded native / WASM provider admission
- final engine adapter ABI

## quality rule

No validation means no success claim. Reviewer output is useful, but it does not replace local validation.
