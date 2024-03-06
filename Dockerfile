# ステージ1: ビルド環境
FROM rust:1.75.0 as builder
# muslターゲットを追加
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/blog-api
COPY . .

# musl libcを使って静的リンクされたバイナリをビルド
RUN apt-get update && apt-get install -y musl-tools musl-dev && \
    cargo build --release --target x86_64-unknown-linux-musl

# ステージ2: 実行環境
# 実行環境としては最小限のベースイメージを使用
FROM scratch
# 必要なライブラリやファイルのみをコピー
COPY --from=builder /usr/src/blog-api/target/x86_64-unknown-linux-musl/release/blog-api /usr/local/bin/blog-api

CMD ["/usr/local/bin/blog-api"]
