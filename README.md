# atcoder (Rust)

Rust 1.89.0 / cargo-compete / cargo-equip を使う競技プログラミング用リポジトリです。

## 自作ライブラリ（ジャンル別クレート）

`cp_library` リポジトリの `rust-1.89.0` ブランチを GitHub から参照します。
単一の `cp_library` クレートは廃止され、`cp_algorithm`・`cp_data_structure`・
`cp_graph`・`cp_grid`・`cp_math`・`cp_utils` に分割されています。
従来 `cp_library` に依存していたコンテストと通常の新規テンプレートには、
6ジャンルをすべて依存として登録しています。コメント解除は不要です。
`cargo-equip` は nightly / cargo-udeps により対象バイナリの未使用依存を判定し、
使ったジャンルだけを提出コードへ含めます。たとえばグラフの依存設定は:

```toml
[dependencies]
cp_graph = { git = "https://github.com/itto4869/cp_library.git", branch = "rust-1.89.0" }
```

```rust
use cp_graph::topological_sort;
```

旧 `cp_library::math::...` は `cp_math::...`、`cp_library::lis` は
`cp_algorithm::lis`、Yes/No の関数・マクロは `cp_utils::yes_no` /
`cp_utils::yes_no_custom` に置き換えます。
`./scripts/cp-library-update-clean.sh` は現在の workspace の依存に登録された
`cp_*` を更新し、そのビルドキャッシュを削除します。
依存がない場合は何も変更せず終了します。過去のコンテストを更新する場合は、
先に `./scripts/compuse` で workspace を切り替えてください。
cargo-equip の提出設定は従来のまま使えます。
