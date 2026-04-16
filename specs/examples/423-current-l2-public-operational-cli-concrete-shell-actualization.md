# 423 — current L2 public operational CLI concrete shell actualization

## 目的

`specs/examples/403...404` で fixed した
current-L2 scoped public operational shell concern を、

- thin facade delegated entry
- explicit host-plan input
- `pretty|json` output
- current-L2 scoped Rust shell wrapper

まで narrow に actualize した current cut として整理する。

ここで fixed するのは
**current-L2 scoped Rust shell over thin facade**
であり、

- installed public binary packaging
- final `mir` top-level hierarchy
- final public parser / checker / runtime API
- final host/input contract

は still later に残す。

## source-backed floor

- delegated entry / report は
  `run_current_l2_source_sample` / `CurrentL2SourceSampleRunReport`
  のまま維持する。
- current shell concern は
  `mir-current-l2 run-source-sample <sample> --host-plan <path> --format pretty|json`
  に留める。
- `run_current_l2_runtime_skeleton` / `CurrentL2RuntimeSkeletonReport`
  は support cut に残し、
  `lower_current_l2_fixed_source_text` と repo-local helper 群は excluded bucket に残す。

## actualized surface

| surface | current actualization |
|---|---|
| shell parser | `current_l2_cli` helper が `run-source-sample <sample> --host-plan <path> --format pretty|json` を narrow に受ける |
| delegated entry | loaded host plan を `run_current_l2_source_sample` に渡す |
| output mode | `pretty` と `json` の 2 mode を current shell concern に含める |
| callable wrapper | `cargo run -p mir-runtime --example mir_current_l2 -- ...` で current shell concern を呼べる |
| focused tests | pretty success、json static-stop、missing `--host-plan`、invalid `--format` を固定する |

## current judgment

1. current actual shell concern は、installed binary ではなく
   **current-L2 scoped Rust shell helper + example wrapper**
   に留めるのが最小である。
2. delegated thin-facade entry は
   `run_current_l2_source_sample`
   に据え置き、runtime skeleton later support を巻き戻さない。
3. `--host-plan <path>` は explicit input として current shell concern に含めるが、
   default host policy や implicit host plan discovery は current shell に入れない。
4. shell name `mir-current-l2` は current operational concern の user-facing label として保つが、
   installed binary packaging を fixed したことは意味しない。

## guard

- final installed binary packagingを fixed しない。
- final public parser / checker / runtime API を fixed しない。
- `run_current_l2_runtime_skeleton` を shell primary entry に昇格させない。
- `resolve_current_l2_source_sample_path` や repo-local inventory / regression helper を shell concern に入れない。
- host plan implicit default / ambient discovery を current shell concern に入れない。

## next promoted line

next promoted line は、
**shared-space room-profile・host binding bridge-only note**
に置く。

## what is not decided here

- installed public binary packaging の timing
- final `mir` top-level hierarchy
- final host/input contract
- maintenance helper shell と public shell の final naming split
