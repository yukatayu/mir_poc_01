# Full System V1 Surface Mir Samples

This root is the Surface Mir source-authority line under `specs/39..43` and
`plan/64..68`.

Current state:

- `syntax/` is actualized for `P-SURF-01` as a narrow parser evidence lane for
  canonical `S { ... }` place blocks, role-instance blocks, indexed state
  declarations, record literals, and expected parser rejections.
- `indexed-state/` is actualized for `P-SURF-02` as a narrow semantic checker
  evidence lane for S-owned Participant-indexed state, key-not-authority
  rejection, stale-key rejection, retained-savepoint compaction rejection, and
  nested-place ambient-authority rejection.

Commands:

```bash
python3 scripts/surface_mir_samples.py matrix --format json
python3 scripts/surface_mir_samples.py check-all --format json
cargo test -p mir-ast --test surface_mir_parser -- --nocapture
cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture
```

Stop lines:

- no final public grammar / ABI / SDK.
- no Surface-to-Core elaboration completion yet.
- no runtime execution or source patch hot-plug completion yet.
- no generated package artifact authority.
