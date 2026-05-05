# Native package and host policy

## Native output decision

Product α-1 must provide **native launch bundle**, not direct Mir-to-native code generation.

### Native launch bundle means

A bundle containing:

- compiled Rust `mirrorea-alpha` binary;
- package files;
- devtools viewer assets;
- run script;
- manifest;
- validation reports.

### It does not mean

- Mir source directly compiled to machine code;
- arbitrary native package execution;
- native plugin ABI finalization;
- signed binary = semantic safety.

## Native bundle manifest

```json
{
  "bundle_schema": "mirrorea.alpha1.bundle.v1",
  "bundle_id": "...",
  "built_at": "...",
  "runtime_binary": "bin/mirrorea-alpha",
  "packages": [...],
  "entry_command": "./run.sh",
  "devtools": "devtools/viewer.html",
  "non_claims": [
    "not direct Mir native codegen",
    "not arbitrary native package execution"
  ]
}
```

## Host integration policy

Default host target for product α-1:

```text
native process, local/Docker controlled environment
```

Non-goals:

- browser runtime;
- Unity/Unreal engine adapter;
- VRChat/VRM full compatibility;
- WASM/browser sandbox unless actually implemented.

## Runtime package policy

Package manifest must declare:

- package kind;
- effects;
- failures;
- capabilities;
- witness requirements;
- observation policy;
- native policy;
- host requirements.

## Native execution policy

Use:

```text
NativeExecutionPolicy = Disabled | SandboxedExternalProcess | TrustedDevOnly
```

Product α-1 default:

```text
Disabled
```

If `SandboxedExternalProcess` is implemented, it must include:

- process isolation;
- allowed effect row;
- resource limit;
- timeout;
- audit event;
- revocation story.

If not implemented, keep it non-goal.

## Provenance

Package signature/provenance:

- can prove source/origin/trust root;
- cannot prove semantic safety.

Therefore checker must not accept native execution solely because signature exists.

## Samples

- `NATIVE-A1-01`: native launch bundle created.
- `NATIVE-A1-02`: bundle run script executes demo.
- `NATIVE-A1-03`: unsigned native package rejected.
- `NATIVE-A1-04`: signed over-capability package rejected.
- `NATIVE-A1-05`: unsupported runtime falls back visibly.
- `NATIVE-A1-06`: direct Mir-to-machine-code request returns explicit non-goal / unsupported diagnostic.

## Validation

```bash
mirrorea-alpha build-native-bundle samples/product-alpha1/demo --out /tmp/mirrorea-alpha1-bundle
/tmp/mirrorea-alpha1-bundle/run.sh --check
/tmp/mirrorea-alpha1-bundle/run.sh --demo
test -f /tmp/mirrorea-alpha1-bundle/devtools/viewer.html
```
