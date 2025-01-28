# AMD64 构建阶段
FROM --platform=linux/amd64 rust:1.84.0-slim-bookworm as builder-amd64
WORKDIR /app
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential protobuf-compiler pkg-config libssl-dev nodejs npm openssl \
    && rm -rf /var/lib/apt/lists/*
COPY . .
ENV RUSTFLAGS="-C link-arg=-s -C target-cpu=x86-64-v3"
RUN cargo build --release && \
    cp target/release/cursor-api /app/cursor-api

# ARM64 构建阶段
FROM --platform=linux/arm64 rust:1.84.0-slim-bookworm as builder-arm64
WORKDIR /app
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential protobuf-compiler pkg-config libssl-dev nodejs npm openssl \
    && rm -rf /var/lib/apt/lists/*
COPY . .
ENV RUSTFLAGS="-C link-arg=-s -C target-cpu=apple-m1"
RUN cargo build --release && \
    cp target/release/cursor-api /app/cursor-api

# AMD64 运行阶段
FROM --platform=linux/amd64 debian:bookworm-slim as run-amd64
WORKDIR /app
ENV TZ=Asia/Shanghai
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates tzdata openssl \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder-amd64 /app/cursor-api .

# ARM64 运行阶段
FROM --platform=linux/arm64 debian:bookworm-slim as run-arm64
WORKDIR /app
ENV TZ=Asia/Shanghai
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates tzdata openssl \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder-arm64 /app/cursor-api .

# 通用配置
FROM run-${TARGETARCH}
ENV PORT=3000
EXPOSE ${PORT}
CMD ["./cursor-api"]