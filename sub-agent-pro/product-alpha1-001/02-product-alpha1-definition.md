# Product/Public-ready Mirrorea Spaces α-1 definition

## One sentence

Mirrorea Spaces product α-1 is an alpha release where an outside developer can use a documented toolchain to build, run, hot-plug, observe, save, and package a small shared virtual-space product prototype, while the theory boundaries remain explicit and non-ad-hoc.

## It is not

- final public product;
- final public parser grammar;
- direct Mir-to-machine-code compiler;
- production WAN federation;
- durable distributed save/load completion;
- marketplace / registry;
- full VRM / VRChat / Unity compatibility;
- Reversed Library implementation;
- PrismCascade integration.

## It must include

### Public-ish entrypoint

A binary or library entrypoint, preferably Rust CLI:

```text
mirrorea-alpha check
mirrorea-alpha run-local
mirrorea-alpha session
mirrorea-alpha attach
mirrorea-alpha transport
mirrorea-alpha save
mirrorea-alpha load
mirrorea-alpha quiescent-save
mirrorea-alpha export-devtools
mirrorea-alpha view
mirrorea-alpha build-native-bundle
mirrorea-alpha demo
```

The exact name may change, but there must be one canonical developer-facing entrypoint.

### Versioned package format

`package.mir.json` may remain the initial alpha source surface, but product α-1 requires:

- schema version;
- package id / version;
- package kind;
- dependencies;
- effect row;
- failure row;
- capabilities;
- witness requirements;
- observation/redaction/retention policy;
- host/native policy;
- compatibility constraints.

### Product demo

A single small demo should exercise:

- local world;
- participant/action;
- typed `AddOne` or similar host-I/O;
- debug layer attach;
- auth/rate-limit attach;
- object/avatar placeholder package;
- observer-safe devtools;
- local save/load;
- quiescent-save controlled savepoint;
- Docker/local transport path;
- native launch bundle.

### Theory status

The product α-1 release must state exactly what is guaranteed:

- finite decidable checker fragment;
- model-check second line;
- proof/residual obligation side line;
- local/quiescent save classes;
- message failure/recovery classes;
- auth/layer contract transformer laws;
- observability redaction/retention laws.

### Validation status

Product α-1 must pass:

- unit tests;
- integration tests;
- clean clone hands-on commands;
- product demo commands;
- viewer openability check;
- native launch bundle check;
- docs/source hierarchy validators;
- report closeout.

## Definition of Done

Product α-1 is done only if all of the following are true:

1. A new developer can run the canonical hands-on guide from clean clone.
2. The product demo runs through the alpha CLI.
3. The demo has positive and negative cases.
4. Devtools viewer explains what happened.
5. Save/load and quiescent-save guarantees are explicit and tested.
6. Hot-plug attach/reject/defer behavior is visible in the same session.
7. Package/native policy is documented and enforced for alpha.
8. All non-goals are visible.
9. `progress.md`, `tasks.md`, `samples_progress.md`, `README.md`, `Documentation.md`, specs, plans, reports are synchronized.
10. Commit and push are complete.
