FROM rust:1.88-slim as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

# 第一阶段：预编译依赖（不加 --locked，允许解析兼容版本）
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 复制真实源码
COPY . .

# 第二阶段：用 --locked 编译应用
RUN cargo build --release --locked

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/mes-backend /app/mes-backend
COPY migrations /app/migrations

EXPOSE 8080

CMD ["./mes-backend"]
