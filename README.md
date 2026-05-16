# Algorithm and Data Structures in Rust

『プログラミングコンテスト攻略のためのアルゴリズムとデータ構造』を Rust で学習するためのリポジトリです。

標準入力のパースではなく、型付きの `solve` 関数を直接実装・テストします。

## 構成

```text
.
├── Cargo.toml
├── c2/
│   └── alds1_1_d/
│       └── src/main.rs
└── c3/
    └── alds1_1_a/
        └── src/main.rs
```

基本的に編集するのは、各問題の `src/main.rs` にある `solve` 関数です。

```rust
fn solve(prices: &[i32]) -> i32 {
    todo!("ここに解答を実装する")
}
```

## テスト

全体:

```bash
cargo test --workspace
```

特定の問題だけ:

```bash
cargo test -p alds1_1_d
```

未実装の問題がある状態でコンパイルだけ確認:

```bash
cargo test --workspace --no-run
```

