FROM rust:1.70.0 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release
FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev libsqlite3-0 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
CMD ["myapp"]
