# 0972 — P17 storage / LLVM / backend preparation first-cut closeout

## Objective

`P17` storage / LLVM / backend preparation の current first-cut closeout を
repo current line に同期し、external workdir / cleanup / LLVM staging ownership mismatch を
non-destructive probe floor として固定する。

## Scope and assumptions

- scope は current first-cut closeout のみであり、actual LLVM checkout / configure / build、backend choice、installed binary / FFI / engine adapter target は含めない
- `/mnt/mirrorea-work` は live mount として扱うが、mount rewrite / format / ownership repair は routine helper に入れない
- user-owned ではない `/mnt/mirrorea-work/llvm` parent は current risk として可視化し、routine cleanup helper は guarded behavior のみを担う
- unrelated user worktree dirtiness は次の 2 files にあるため触らない
  - `crates/mir-ast/examples/current_l2_inspect_request_head_clause_bundle.rs`
  - `crates/mir-ast/src/current_l2.rs`

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/23-compiler-backend-llvm-guardrail-roadmap.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/compiler_backend_llvm_preparation_01.md`
- `scripts/README.md`
- `scripts/env/mirrorea_storage_env.sh`
- `scripts/storage/detach_prepare.sh`
- `scripts/storage/cleanup_disposable_artifacts.sh`

## Actions taken

1. Expanded storage/env helper output so the current line exposes `llvm` root/src/build/install path presence, owner, and writable status.
2. Tightened storage audit output to show live `llvm` staging directory ownership and to state explicitly that `llvm/src` is outside disposable cleanup.
3. Tightened cleanup behavior so `llvm/build` / `llvm/install` cleanup is refused when the `llvm` parent remains non-writable for the current user.
4. Added a dedicated hands-on landing page for `P17` and updated front-door docs so storage/backend closeout commands include `--ensure-dirs`, `detach_prepare.sh`, cleanup `--list`, `free -h`, explicit `ls -ld`, and the `mir-ast --no-run` cargo probe.
5. Updated snapshot / roadmap documents so `P17` is closed for current scope and `P18` becomes the promoted next mixed gate.

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/23-compiler-backend-llvm-guardrail-roadmap.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/compiler_backend_llvm_preparation_01.md`
- `docs/research_abstract/compiler_backend_llvm_preparation_01.md`
- `scripts/README.md`
- `scripts/env/mirrorea_storage_env.sh`
- `scripts/storage/detach_prepare.sh`
- `scripts/storage/cleanup_disposable_artifacts.sh`

## Evidence / outputs / test results

### Storage / mount / capacity

- `df -h`
  - `/dev/vda2` root: `99G` total, `32G` avail
  - `/dev/vdb1` `/mnt/mirrorea-work`: `196G` total, `180G` avail
- `free -h`
  - memory available: `271Mi`
  - swap available: `18Gi`
- `lsblk -f`
  - `/dev/vdb1` ext4 label `mirrorea-work`
- `findmnt`
  - `/mnt/mirrorea-work` is mounted from `/dev/vdb1`
- `ls -ld target /mnt/mirrorea-work/cargo-target /mnt/mirrorea-work/cargo-registry-cache /mnt/mirrorea-work/llvm /mnt/mirrorea-work/llvm/src /mnt/mirrorea-work/llvm/build /mnt/mirrorea-work/llvm/install`
  - `target -> /mnt/mirrorea-work/cargo-target`
  - `/mnt/mirrorea-work/llvm` owner is `root:root`
  - `src/build/install` are user-owned staging dirs

### Helper / cleanup probe

- `bash scripts/env/mirrorea_storage_env.sh`
  - pass
  - emits `MIRROREA_WORKDIR`, `CARGO_TARGET_DIR`, `CARGO_HOME`, `MIRROREA_LLVM_{SRC,BUILD,INSTALL}_DIR`
  - warns that `/mnt/mirrorea-work/llvm` is not writable by the current user
  - emits `MIRROREA_LLVM_ROOT_OWNER=root:root` and writable status per path
- `bash scripts/env/mirrorea_storage_env.sh --ensure-dirs`
  - pass
- `bash scripts/storage/detach_prepare.sh`
  - pass
  - reports repo usage `102M`, `.git` `80M`, external workdir `6.0G`, cargo target `5.9G`, cargo registry cache `9.7M`
  - reports `llvm` staging dir ownership and guarded cleanup note
- `bash scripts/storage/cleanup_disposable_artifacts.sh --list`
  - pass
  - lists only disposable directories
  - excludes `llvm/src`
  - shows `llvm` root owner/writable state
  - note: refusal branch for parent-non-writable `llvm/build` / `llvm/install` cleanup is implemented but not exercised in current non-destructive closeout

### Cargo / doc validation

- `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run`
  - pass
  - warnings only from existing unused support functions
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `git diff --check`
  - pass

## What changed in understanding

- `P17` の honest closeout line は actual LLVM build ではなく、ownership mismatch を見えない前提にしない storage/backend guardrail の固定である
- `llvm/src` は build/install と違って source checkout lifecycle が未決なので、disposable cleanup から明示的に外す必要がある
- `P18` は storage/backend package の続きとして actual build を始める gate ではなく、repo-side public-boundary inventory と final freeze conditions を整理する mixed gate と読む方が current queue に整合する

## Open questions

- `/mnt/mirrorea-work/llvm` parent の ownership repair をどの explicit setup path / operator flow に残すか
- `llvm/src` checkout retention / cleanup lifecycle を later package でどう定義するか
- actual backend choice、installed binary / FFI / engine adapter target をいつ user-spec hold line から promote するか

## Suggested next prompt

`P18` public API / parser grammar gate の repo-side first cut を進め、final parser grammar / public API / public verifier / public viewer / public adapter contract について、current helper/runtime/report-local carrier から何を freeze candidate、kept-later gate、true user-spec hold line に分けるかを `specs/` / `plan/` / `progress.md` / `tasks.md` / `samples_progress.md` / 新しい report に同期してください。
