# ─── Stage 1: Static Rust binary ────────────────────────────────────────────
# Build on the native host platform to avoid QEMU emulation overhead.
# cargo-zigbuild uses Zig as a cross-linker to produce binaries for TARGETPLATFORM.
FROM --platform=$BUILDPLATFORM rust:1.93-alpine AS rust-builder

ARG TARGETPLATFORM

# Install build tools:
#   musl-dev       – musl libc headers (needed for static musl builds)
#   linux-headers  – kernel netfilter headers required by rustables/bindgen
#   clang-dev      – provides libclang.so, required by bindgen at build time
#   zig            – cross-linker used by cargo-zigbuild
RUN apk add --no-cache musl-dev linux-headers clang-dev perl make pkgconf zig

# Install cargo-zigbuild (cross-compilation wrapper using Zig)
RUN cargo install cargo-zigbuild --locked

# Map Docker's TARGETPLATFORM to the corresponding Rust musl target triple
RUN case "${TARGETPLATFORM}" in \
      "linux/amd64")   echo "x86_64-unknown-linux-musl"      ;; \
      "linux/arm64")   echo "aarch64-unknown-linux-musl"     ;; \
      "linux/arm/v7")  echo "armv7-unknown-linux-musleabihf" ;; \
      *) echo "Unsupported platform: ${TARGETPLATFORM}" >&2 && exit 1 ;; \
    esac > /rust-target.txt

RUN rustup target add "$(cat /rust-target.txt)"

WORKDIR /app
COPY backend/ .

# Build a fully-static binary; strip symbols at compile time (no cross-arch strip needed)
RUN RUSTFLAGS="-C strip=symbols" \
    cargo zigbuild --release --target "$(cat /rust-target.txt)"

RUN cp "target/$(cat /rust-target.txt)/release/wg-easy-rs" /wg-easy-rs

# ─── Stage 2: React SPA ─────────────────────────────────────────────────────
# Always build the frontend on the native host; output is platform-independent JS/CSS.
FROM --platform=$BUILDPLATFORM node:20-alpine AS frontend-builder
WORKDIR /app
COPY frontend/ .
RUN npm ci
RUN npm run build

# ─── Stage 3: FROM scratch ──────────────────────────────────────────────────
FROM scratch

COPY --from=rust-builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=rust-builder /wg-easy-rs /usr/local/bin/wg-easy-rs
COPY --from=frontend-builder /app/dist /app/static

VOLUME ["/etc/wireguard"]
EXPOSE 51821/tcp
EXPOSE 51820/udp

ENTRYPOINT ["/usr/local/bin/wg-easy-rs"]
