#!/usr/bin/env bash

# Nix が Rust toolchain と rust-src を管理するため、Wasabi の Makefile が
# 呼び出す rustup の追加インストール要求だけを成功扱いにする。
exit 0
