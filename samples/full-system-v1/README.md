# Full System V1 Samples

This root is the source-first Full System V1 line.

Current state:

- `computational/` is actualized for `P-MIR-01..04` as parser, typed checker, bounded effectful runtime, and runtime report evidence.
- `avatar-pose/` is actualized for `P-POSE-03` as bounded runtime PoseGraph evidence with 8 executable rows and 1 planned save/load row.
- world-core, membership-chat, sugoroku-world, projection, server/client, and provider-adapter families remain planned.

Commands available now:

```bash
python3 scripts/textual_mir_samples.py matrix --format json
python3 scripts/textual_mir_samples.py check-all --format json
python3 scripts/full_system_v1_samples.py matrix --format json
python3 scripts/full_system_v1_samples.py check-all --format json
python3 scripts/posegraph_runtime_samples.py matrix --format json
python3 scripts/posegraph_runtime_samples.py check-all --format json
cargo test -p mir-runtime --test full_system_v1_session -- --nocapture
cargo test -p mir-runtime --test posegraph_runtime -- --nocapture
```

Stop lines:

- no final public grammar
- no final typed IR / interpreter / runtime API completion here
- no pose-aware save/load or final devtools completion here
- no package artifact generation here
