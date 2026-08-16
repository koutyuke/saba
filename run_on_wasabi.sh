#!/usr/bin/env bash

set -Eeuo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="$ROOT_DIR/build"
WASABI_DIR="$BUILD_DIR/wasabi"
WASABI_REPOSITORY="https://github.com/hikalium/wasabi.git"
WASABI_BRANCH="for_saba"
APP_NAME="saba"
APP_TARGET="x86_64-unknown-none"
APP_BIN="$ROOT_DIR/target/$APP_TARGET/release/$APP_NAME"
RUST_LLD_WRAPPER="$ROOT_DIR/tools/rust-lld-wrapper.sh"
RUSTUP_NOOP="$ROOT_DIR/tools/rustup-noop.sh"
WASABI_TOOLCHAIN_DIR="${WASABI_RUST_TOOLCHAIN:-}"
APP_RUSTFLAGS="-C link-args=-e -C link-args=entry -C link-args=-z -C link-args=execstack"
PORT_MONITOR="${PORT_MONITOR:-2345}"
PORT_OFFSET_VNC="${PORT_OFFSET_VNC:-5}"
TCP_FORWARD_PORT="${TCP_FORWARD_PORT:-18080}"

die() {
  printf 'エラー: %s\n' "$*" >&2
  exit 1
}

require_command() {
  command -v "$1" >/dev/null 2>&1 || die "`$1` が見つかりません。先に `nix develop` を実行してください。"
}

require_command cargo
require_command git
require_command make
require_command qemu-system-x86_64
require_command rustc

if [ ! -x "$RUST_LLD_WRAPPER" ]; then
  die "Rust リンカー wrapper が実行できません: $RUST_LLD_WRAPPER"
fi

if [ ! -x "$RUSTUP_NOOP" ]; then
  die "rustup shim が実行できません: $RUSTUP_NOOP"
fi

if [ ! -x "$WASABI_TOOLCHAIN_DIR/bin/cargo" ]; then
  die "Wasabi 用 Rust toolchain が見つかりません。先に `nix develop` を実行してください。"
fi

if [ "$(uname -s)" = "Darwin" ]; then
  # macOS では QEMU のネイティブウィンドウを表示する。
  if [ -z "${DISPLAY:-}" ]; then
    export DISPLAY=1
  fi
fi

mkdir -p "$BUILD_DIR"

if [ -e "$WASABI_DIR" ] && [ ! -d "$WASABI_DIR/.git" ]; then
  die "$WASABI_DIR は Git リポジトリではありません。内容を確認してから再実行してください。"
fi

if [ -d "$WASABI_DIR/.git" ]; then
  printf 'Wasabi OS を更新します: %s\n' "$WASABI_DIR"
  git -C "$WASABI_DIR" switch "$WASABI_BRANCH"
  git -C "$WASABI_DIR" pull --ff-only origin "$WASABI_BRANCH"
else
  printf 'Wasabi OS を取得します: %s\n' "$WASABI_DIR"
  git clone --branch "$WASABI_BRANCH" --single-branch "$WASABI_REPOSITORY" "$WASABI_DIR"
fi

printf '現在のプロジェクトを Wasabi 用にビルドします\n'
CARGO_TARGET_X86_64_UNKNOWN_NONE_LINKER="$RUST_LLD_WRAPPER" \
RUSTFLAGS="$APP_RUSTFLAGS${RUSTFLAGS:+ $RUSTFLAGS}" \
  cargo build \
    --target "$APP_TARGET" \
    --release \
    -Z build-std=core,compiler_builtins,alloc

[ -x "$APP_BIN" ] || die "Wasabi 用バイナリが生成されませんでした: $APP_BIN"

# Wasabi の Makefile は rustup を呼び出すが、Nix の開発シェルでは
# ツールチェーンと rust-src を Nix が管理しているため、追加インストールは不要。
RUSTUP_SHIM_DIR="$(mktemp -d "${TMPDIR:-/tmp}/saba-wasabi-rustup.XXXXXX")"
trap 'rm -rf "$RUSTUP_SHIM_DIR"' EXIT
ln -s "$RUSTUP_NOOP" "$RUSTUP_SHIM_DIR/rustup"

printf 'Wasabi OS と QEMU を起動します\n'
CARGO_TARGET_X86_64_UNKNOWN_UEFI_LINKER="$RUST_LLD_WRAPPER" \
PATH="$RUSTUP_SHIM_DIR:$WASABI_TOOLCHAIN_DIR/bin:$PATH" \
  make -C "$WASABI_DIR" run \
    WITH_APP_BIN="$APP_BIN" \
    APP_CARGO="CARGO_TARGET_X86_64_UNKNOWN_NONE_LINKER='$RUST_LLD_WRAPPER' RUSTFLAGS='$APP_RUSTFLAGS' cargo -Z build-std=core,compiler_builtins,alloc -Z build-std-features=compiler-builtins-mem" \
    PORT_MONITOR="$PORT_MONITOR" \
    PORT_OFFSET_VNC="$PORT_OFFSET_VNC" \
    TCP_FORWARD_PORT="$TCP_FORWARD_PORT"
