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

## Wasabi OS で実行する

開発シェル内でプロジェクトのルートから次を実行すると、Wasabi OS の取得、現在の `saba` のビルド、QEMU の起動をまとめて行えます。

```sh
./run_on_wasabi.sh
```

このスクリプトでは、現在の `saba` を `nightly-2026-08-12` でビルドし、`nightly-2024-01-01` を必要とする Wasabi OS 本体は専用の toolchain でビルドします。`nix develop` の外から実行しないでください。

初回実行時は `build/wasabi` に Wasabi OS の `for_saba` ブランチを取得します。生成物は `build/` 以下に置かれ、Git 管理対象には含まれません。

QEMU が起動したら、Wasabi OS のプロンプトで `saba` と入力して Enter を押します。macOS では QEMU のウィンドウを表示し、Linux では Wasabi OS の既定の表示方法を使用します。

標準ポートが使用中の場合は、空いているポートを指定して実行します。

```sh
PORT_MONITOR=2346 TCP_FORWARD_PORT=18082 ./run_on_wasabi.sh
```

QEMU を終了するには、起動したターミナルで `Ctrl-C` を押します。

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

Wasabi OS の実装や依存関係を確認するときは、上流リポジトリを参照します。通常の Mac/Linux 上の CUI 実装では、上記の Wasabi OS 手順を実行する必要はありません。
