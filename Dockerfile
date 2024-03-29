FROM rust:1.76.0-buster as builder
WORKDIR /usr/src/blog-api
COPY . .
RUN cargo build --release

# ステージ2: 実行環境
FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev libsqlite3-0 && rm -rf /var/lib/apt/lists/*
RUN mkdir /data
COPY --from=builder /usr/src/blog-api/target/release/blog-api /usr/local/bin/blog-api
VOLUME /data
CMD ["blog-api"]
