# alpha sample family — Avatar / Runtime Package

- Status: planned skeleton only
- Phase: Phase 7
- Stage: Stage D -> F bridge
- Current runners do not execute this family yet.
- Validation for this package is filesystem/docs integrity only.

## Rows

| ID | File | Kind | Expected |
|---|---|---|---|
| `AV-01` | `av-01-placeholder_avatar_runtime.mir` | positive | accepted |
| `AV-02` | `av-02-custom_mir_avatar_runtime.mir` | positive | accepted |
| `AV-03` | `av-03-vrm_adapter_skeleton.mir` | planned/positive | accepted as limited/planned |
| `AV-04` | `av-04-vrchat_compat_adapter_skeleton.mir` | planned/positive | accepted as limited/planned |
| `AV-05` | `av-05-unsupported_shader_fallback.mir` | positive | fallback material |
| `AV-06` | `av-06-untrusted_native_avatar_rejected.mir` | negative | rejected |
| `AV-07` | `av-07-trusted_native_sandbox_limited.mir` | positive/limited | accepted with limited caps |
| `AV-08` | `av-08-runtime_unavailable_placeholder.mir` | positive | placeholder fallback |
| `AV-09` | `av-09-adapter_attempts_undeclared_effect.mir` | negative | rejected |
| `AV-10` | `av-10-package_detach_active_avatar_deferred.mir` | negative/runtime | deferred/rejected |

## Policy

- `.mir` files here are source-ish planned skeletons, not active runnable samples.
- `.expected.json` sidecars record the intended verdict or runtime outcome for future runners/checkers.
- Promotion to active/runnable status requires dedicated validation commands, report evidence, and snapshot updates.

## Validation anchor for this package

```bash
find samples/alpha/avatar-runtime -maxdepth 1 -type f | sort
```
