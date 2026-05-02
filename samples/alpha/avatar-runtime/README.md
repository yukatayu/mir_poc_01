# alpha sample family — Avatar / Runtime Package

- Status: mixed
- Phase: Phase 7
- Stage: Stage D -> F bridge
- `mirrorea_alpha_avatar_runtime` と `scripts/alpha_avatar_runtime_samples.py` が `AV-01/02/06/08/09` を runtime-private package/avatar admission floor として実行する。
- この family は active runnable root ではなく、sample-ID keyed non-public runner evidence として扱う。

## Rows

| ID | File | Kind | Expected |
|---|---|---|---|
| `AV-01` | `av-01-placeholder_avatar_runtime.mir` | positive | accepted |
| `AV-02` | `av-02-custom_mir_avatar_runtime.mir` | positive | accepted |
| `AV-03` | `av-03-vrm_adapter_skeleton.mir` | planned/positive | accepted as limited/planned |
| `AV-04` | `av-04-vrchat_compat_adapter_skeleton.mir` | planned/positive | accepted as limited/planned |
| `AV-05` | `av-05-unsupported_shader_fallback.mir` | planned/positive | fallback material |
| `AV-06` | `av-06-untrusted_native_avatar_rejected.mir` | negative | rejected |
| `AV-07` | `av-07-trusted_native_sandbox_limited.mir` | planned/positive/limited | accepted with limited caps |
| `AV-08` | `av-08-runtime_unavailable_placeholder.mir` | positive | placeholder fallback |
| `AV-09` | `av-09-adapter_attempts_undeclared_effect.mir` | negative | rejected |
| `AV-10` | `av-10-package_detach_active_avatar_deferred.mir` | planned/negative/runtime | deferred/rejected |

## Policy

- `.mir` files here remain source-ish anchors rather than parsed inputs.
- `.expected.json` sidecars for `AV-01/02/06/08/09` are generated from current runtime-private example output and act as bridge evidence for the runner/checker floor.
- `AV-03/04/05/07/10` remain planned-only rows.
- Promotion to active/runnable root status requires integrated validation commands, report evidence, and snapshot updates.

## Validation anchor for this package

```bash
cargo test -p mir-runtime --test alpha_avatar_runtime
cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- closeout
python3 scripts/alpha_avatar_runtime_samples.py check-all --format json
```
