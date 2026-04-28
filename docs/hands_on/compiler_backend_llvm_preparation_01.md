# compiler backend llvm preparation 01

## この文書の役割

この文書は `P17` storage / LLVM / backend preparation の
**current first-cut closeout** を最短で確認するための hands-on です。

- actual LLVM build ではありません
- root-only setup script の言い換えでもありません
- external workdir / cleanup / LLVM staging ownership mismatch を
  non-destructive probe で確認する入口です

## まず実行するコマンド

```bash
bash scripts/env/mirrorea_storage_env.sh
bash scripts/env/mirrorea_storage_env.sh --ensure-dirs
bash scripts/storage/detach_prepare.sh
bash scripts/storage/cleanup_disposable_artifacts.sh --list
df -h
free -h
lsblk -f
findmnt
ls -ld target /mnt/mirrorea-work/cargo-target /mnt/mirrorea-work/cargo-registry-cache /mnt/mirrorea-work/llvm /mnt/mirrorea-work/llvm/src /mnt/mirrorea-work/llvm/build /mnt/mirrorea-work/llvm/install
CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run
```

## 何を見るか

- `/mnt/mirrorea-work` が `/dev/vdb1` として mounted されていること
- `target/` が `/mnt/mirrorea-work/cargo-target` へ symlink していること
- `CARGO_HOME` が `/mnt/mirrorea-work/cargo-registry-cache` を向けること
- `llvm/src` / `llvm/build` / `llvm/install` が staging path として存在すること
- `/mnt/mirrorea-work/llvm` の owner / writable status が明示されること
- cleanup helper の list-mode が `llvm/src` を disposable candidate に含めないこと
- helper 実装上は parent non-writable 時の `llvm/build` / `llvm/install` cleanup guard を持つこと

## これで確認できること

- small VPS 前提で heavy disposable artifact を external workdir へ逃がす current rule
- cleanup helper が repo source / report を削除しない current safety line
- LLVM staging path readiness と ownership mismatch が helper 出力で visible になっていること
- cleanup refusal branch 自体は non-destructive current closeout では実行せず、guard implementation / stop line として文書化していること
- actual backend choice や LLVM checkout/build に入る前の implementation-ready staging が current repo に固定されていること

## これではまだ確認できないこと

- actual LLVM checkout / configure / build
- final backend choice
- final packaging / installed binary / FFI / engine adapter target
- root-owned llvm parent の ownership repair

## 関連

- `../../plan/23-compiler-backend-llvm-guardrail-roadmap.md`
- `../research_abstract/compiler_backend_llvm_preparation_01.md`
- `current_phase_closeout_01.md`
