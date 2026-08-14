# SaBA

『作って学ぶブラウザの仕組み』に沿って、Rust でブラウザを実装するための作業ディレクトリです。

開発環境は Nix flake で管理します。書籍版・上流リポジトリの再現性を尊重しつつ、Zed の `rust-analyzer` と `rust-src` の互換性を確保するため、この作業ディレクトリでは上流と異なる `nightly-2026-08-12` を固定しています。

## 開発環境を起動する

Nix と flakes が使える状態で、このディレクトリから実行します。

```sh
nix develop
```

`direnv` を使う場合は、初回だけ次を実行します。

```sh
direnv allow
```

シェルへ入った後、`rustc -Vv` の `release` に `nightly` が含まれ、`cargo` が起動できれば準備完了です。使用する日付は [rust-toolchain.toml](./rust-toolchain.toml) に固定しています。

```sh
rustc -Vv
cargo --version
```

## 参照する実装

上流の実装は `ghq` で次の場所へ取得します。

- `$(ghq root)/github.com/d0iasm/saba` — 書籍以上の実装
- `$(ghq root)/github.com/d0iasm/sababook` — 書籍内の実装

このディレクトリでは、書籍版のコードを参照しながら実装を進めます。上流の CUI 実装をビルドする場合は、開発シェルへ入った後に次を実行します。

```sh
cd "$(ghq root)/github.com/d0iasm/saba"
cargo build --features=cui --bin=saba_cui --no-default-features
(cd core && cargo test)
```

上流リポジトリは `Cargo.lock` を管理していません。そのため、将来の crates.io の依存更新によって、固定された Rust ツールチェーンでは MSRV 不足のエラーが出る場合があります。その場合は、ツールチェーンを変更する前に依存バージョンを確認してください。

Wasabi OS 向けの実行には、上流リポジトリの `run_on_wasabi.sh` が必要とする追加の取得・ビルド手順があります。まずは Mac/Linux 上の CUI 実装を対象にします。
