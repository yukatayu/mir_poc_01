# prototype samples

この directory は、**機能 family 自体は比較検討中だが、current L2 parser / runner で修正 prototype を回して挙動比較したい** sample を置く。

- `samples/current-l2/` には入れない。
- ただし current L2 fixed subset の text lowerer で parse できる形には寄せる。
- 実行するときは `mir-current-l2 run-source-sample <path> --format pretty|json` を使う。
- prototype sample は explicit path で実行し、sample file の隣に `.host-plan.json` sidecar があれば `--host-plan` を省略できる。
- sample debugging では、sidecar が `debug_*` または `_debug_` を含み `_output` / `_pipe` で終わる target に append した record を CLI が `debug_outputs` として preview してよい。
  これは helper-local convenience であり、final host-I/O / shell contract ではない。
- theorem / model-check bridge の current floor では、CLI が `verification_preview` と helper-local `artifact_preview` を出してよい。
  これは final public verifier contract や final emitted artifact schema を意味しない。

## current subdirectories

- `current-l2-order-handoff/`
  - `atomic_cut` / publication / handoff / room-local serial transition / static-stop / guarded chain / late join / stale reconnect refresh の corrected prototype
  - `p07-dice-late-join-visible-history` と `p08-dice-stale-reconnect-refresh` は、order/handoff current first line を sample-visible にする corrected prototype であり、final source wording / final replay profile ではない
  - `p09-dice-delegated-rng-provider-placement` は、`delegated_rng_service` provider placement を narrow prototype として actualize する corrected prototype であり、final provider receipt schema / delegated provider attestation ではない
- `current-l2-typed-proof-model-check/`
  - `admit` / `require` / `ensure` を持つ typed marker family が helper-local verifier preview にどう映るかを比較する corrected prototype
  - `p06-typed-proof-owner-handoff` は sample-visible corrected prototype であり、final typed calculus / final public verifier contract / settled property language ではない
  - `p10-typed-authorized-fingerprint-declassification` と `p11-typed-unauthorized-fingerprint-release` は、explicit authority declassification success/failure を source-side で比較する corrected prototype であり、final typed source principal / final IFC syntax / final public verifier contract ではない
  - `p12-typed-classified-fingerprint-publication-block` は、authority 欠如ではなく label-flow mismatch によって public publication が止まることを source-side で比較する corrected prototype であり、final typed source principal / final IFC syntax / final public verifier contract ではない
- `current-l2-dynamic-attach-detach/`
  - attach / detach の corrected prototype

## current limit

- `underdeclared` omission family (`e5` / `e12`) は `samples/current-l2/` authored corpus に actualize 済みである。
- prototype bucket は、それでもなお mixed-gate の corrected prototype を runnable に比較したい family を置く場所として保つ。
- helper-local `verification_preview` / `artifact_preview` は prototype bucket でも使ってよいが、public contract や final emitted schema と混同しない。
- narrow mixed-gate pre-floor では、sample-local preview-aligned typed artifact route を compare floor に使ってよいが、これも helper-local support であり public contract ではない。
- narrow mixed-gate pre-floor では、sample-local model-check projection pre-floor route を compare floor に使ってよいが、これも helper-local support であり settled property language / concrete tool seam adoption ではない。
- narrow mixed-gate pre-floor では、sample-local theorem discharge pre-floor route を compare floor に使ってよいが、これも helper-local support であり actual discharge transport / public theorem contract adoption ではない。
- current actualization floor では、sample-local theorem-first experimental pilot route を helper-local emitted-artifact / compare floor に使ってよいが、これも public verifier contract や proof object public schema ではない。
- current actualization floor では、sample-local theorem-prover experimental binding preflight route を helper-local brand-neutral preflight manifest に使ってよいが、これも concrete prover brand や public theorem contract ではない。
- current actualization floor では、sample-local theorem Lean-first non-production stub pilot route を helper-local emitted stub refs first に使ってよいが、これも actual Lean tool execution や public theorem contract や final public verifier contract ではない。
- current actualization floor では、representative source sample `e2 / e5` を使う theorem review-unit to Lean-stub repo-local artifact-conformance bridge を regression-integrated compare floor に使ってよいが、これも actual Lean tool execution や prototype-wide trace alignment や public theorem contract ではない。
- current actualization floor では、representative runtime/static/prototype corpus `e2 / e5 / p06 / p07 / p08` と guard-only `p05` を使う theorem Lean-stub representative trace-alignment bridge を helper-local runtime test に使ってよいが、これも actual Lean tool execution や public theorem contract や proof object public schema ではない。
- current actualization floor では、authoritative-room vertical-slice route を helper-local emitted-artifact / runtime-evidence floor に使ってよいが、これも final emitted handoff schema や exhaustive shared-space catalog ではない。
- current actualization floor では、minimal companion / experimental order-handoff surface route を helper-local compare floor に使ってよいが、これも final parser grammar や final source wording ではない。
- current actualization floor では、stage-block secondary surface route を helper-local compare floor に使ってよいが、これも final parser grammar や authoritative-room serial sugar adoption ではない。
