#!/usr/bin/env bash

set -Eeuo pipefail

rust_sysroot="$(rustc --print sysroot)"
rustc_host="$(rustc -vV | sed -n 's/^host: //p')"
rust_lld="$rust_sysroot/lib/rustlib/$rustc_host/bin/rust-lld"

if [ ! -x "$rust_lld" ]; then
  printf 'エラー: rust-lld が見つかりません: %s\n' "$rust_lld" >&2
  exit 1
fi

if [ "$(uname -s)" = "Darwin" ]; then
  # rust-lld だけが、同じ Rust ツールチェーンの LLVM を参照する。
  export DYLD_LIBRARY_PATH="$rust_sysroot/lib"
fi

exec "$rust_lld" "$@"
