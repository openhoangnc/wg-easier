# ─── Stage 1: Static Rust binary ───────────────────────────────────────────
FROM rust:1.85-alpine AS rust-builder
RUN apk add --no-cache musl-dev perl make pkgconfig
WORKDIR /app
COPY backend/ .
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/wg-easy-rs

# ─── Stage 2: React SPA ─────────────────────────────────────────────────────
FROM node:20-alpine AS frontend-builder
WORKDIR /app
COPY frontend/ .
RUN npm ci
RUN npm run build

# ─── Stage 3: FROM scratch ──────────────────────────────────────────────────
FROM scratch

COPY --from=rust-builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=rust-builder \
  /app/target/x86_64-unknown-linux-musl/release/wg-easy-rs \
  /usr/local/bin/wg-easy-rs
COPY --from=frontend-builder /app/dist /app/static

VOLUME ["/etc/wireguard"]
EXPOSE 51821/tcp
EXPOSE 51820/udp

ENTRYPOINT ["/usr/local/bin/wg-easy-rs"]
