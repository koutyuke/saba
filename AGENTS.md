# AGENTS.md

## プロジェクトの目的

このディレクトリは、書籍『作って学ぶブラウザの仕組み』に沿って、Rust でブラウザ `saba` を実装するための作業場所です。

上流リポジトリは `ghq` で次の場所にあります。

- `$(ghq root)/github.com/d0iasm/sababook` — 書籍内の実装。章ごとのコードを参照する正本
- `$(ghq root)/github.com/d0iasm/saba` — 書籍以降を含む実装。完成形や追加実装を参照する場所

この作業ディレクトリの実装と上流リポジトリの実装を混同しないこと。上流 clone のファイルは、ユーザーが明示的に依頼した場合を除き変更しないこと。

## 開発環境

- 開発シェルは [flake.nix](./flake.nix) の `nix develop` で起動する。`direnv` を使う場合は、初回だけ `direnv allow` を実行する。
- Rust のバージョンは [rust-toolchain.toml](./rust-toolchain.toml) の `nightly-2026-08-12` を正本とする。書籍版・上流実装は `nightly-2024-01-01` を使うため、この作業ディレクトリでは `rust-analyzer` と `rust-src` の互換性を保つ目的で意図的に別の nightly を固定している。ツールチェーンを更新する場合は、書籍版・上流実装との互換性を確認し、理由を変更内容へ記録する。
- Nix の入力を更新した場合は `flake.lock` も更新し、`nix flake check --all-systems` を実行する。
- Rust や補助ツールをシステムへ直接インストールせず、原則として Nix 開発シェル内のものを使用する。

## 実装方針

- 書籍の章を実装するときは、対象章の `$(ghq root)/github.com/d0iasm/sababook/chN/saba` を参照し、後続章や本家の追加実装を先取りして混ぜない。
- 書籍にない挙動を追加する場合は、書籍どおりの実装と追加変更の境界が分かるようにする。
- 公開 API、CLI、設定、環境変数を変更した場合は、関連する README や手順の更新要否を確認する。
- ドキュメントとコメントは日本語で記述する。コード識別子、コマンド、パス、環境変数、crate 名などは原表記を保つ。

## 検証

開発シェルへ入った後、対象に応じて次を実行する。

```sh
# 本家 saba の Mac/Linux CUI 実装
cd "$(ghq root)/github.com/d0iasm/saba"
cargo build --features=cui --bin=saba_cui --no-default-features
(cd core && cargo test)
```

書籍版は、`$(ghq root)/github.com/d0iasm/sababook/chN/saba` でその章の `Cargo.toml` に従って検証する。変更した Rust コードに対しては、可能な範囲で `cargo fmt --check` と対象 crate のテストも実行する。

上流リポジトリは `Cargo.lock` を管理していない。依存解決が将来の crate の MSRV に引っ張られ、固定された Rust 1.77 系 nightly でビルドできない場合がある。その場合は、まず失敗した crate と要求された Rust バージョンを特定し、依存バージョンを固定するかツールチェーンを更新するかを、書籍再現性とのトレードオフ付きで報告する。

## Wasabi OS

Wasabi OS 用の `run_on_wasabi.sh` は、外部リポジトリの取得・更新と生成物の作成を行う。通常の CUI 検証では実行せず、Wasabi OS 向けビルドが依頼された場合だけ、取得先・生成物・外部変更の範囲を確認して実行する。
