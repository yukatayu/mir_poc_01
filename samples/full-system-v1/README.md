# Full System V1 Samples

This root is the source-first Full System V1 line.

Current state:

- `computational/` is actualized for `P-MIR-01..04` as parser, typed checker, bounded effectful runtime, and runtime report evidence.
- `avatar-pose/` is actualized for `P-POSE-03` / `P-POSE-04` as bounded runtime PoseGraph evidence with 9 executable rows, save/load admissibility evidence, and observer-safe devtools export summaries.
- `projection/` is actualized for `P-PROJ-02` as bounded projection IR evidence with 2 executable rows, source-derived target manifests, preservation reports, and one explicit client-write authority rejection row.
- world-core, membership-chat, sugoroku-world, server/client, and provider-adapter families remain planned.

Commands available now:

```bash
python3 scripts/textual_mir_samples.py matrix --format json
python3 scripts/textual_mir_samples.py check-all --format json
python3 scripts/full_system_v1_samples.py matrix --format json
python3 scripts/full_system_v1_samples.py check-all --format json
python3 scripts/posegraph_runtime_samples.py matrix --format json
python3 scripts/posegraph_runtime_samples.py check-all --format json
python3 scripts/projection_v1_samples.py matrix --format json
python3 scripts/projection_v1_samples.py check-all --format json
cargo test -p mir-runtime --test full_system_v1_session -- --nocapture
cargo test -p mir-runtime --test posegraph_runtime -- --nocapture
cargo test -p mir-runtime --test projection_ir -- --nocapture
cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture
```

Stop lines:

- no final public grammar
- no final typed IR / interpreter / runtime API completion here
- no packet or FFI payload schema semantics completion here
- no executable server/client role split here
- no distributed durable pose save/load or final devtools family completion here
- no package artifact generation here
