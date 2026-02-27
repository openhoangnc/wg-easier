# ─── Stage 1: Static Rust binary ────────────────────────────────────────────
# Build on the native host platform to avoid QEMU emulation overhead.
# cargo-zigbuild uses Zig as a cross-linker to produce binaries for TARGETPLATFORM.
#
# Use Debian (glibc) instead of Alpine (musl) for the builder so that build
# scripts compiled by cargo can dlopen libclang.so via bindgen/libloading.
# Statically-linked musl executables return "Dynamic loading not supported"
# from dlopen, which breaks bindgen.  The final image is FROM scratch so
# builder image size is irrelevant.
FROM --platform=$BUILDPLATFORM rust:1.93-slim AS rust-builder

ARG TARGETPLATFORM

# Install build tools:
#   clang / libclang-dev  – clang toolchain + libclang.so required by bindgen
#   linux-libc-dev        – kernel headers (netfilter, etc.) for bindgen
#   musl-dev / musl-tools – musl libc stubs (zigbuild produces the actual musl target)
#   python3-pip           – used to install the ziglang PyPI package
#   perl / make / pkg-config – common build-script helpers
RUN apt-get update && apt-get install -y --no-install-recommends \
    clang libclang-dev \
    linux-libc-dev \
    musl-dev musl-tools \
    python3-pip \
    perl make pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install zig via the official PyPI release (avoids hard-coding a download URL).
# The ziglang package bundles the zig binary for the current host architecture.
# Use Python to create the symlink to avoid shell quoting issues in RUN.
RUN pip3 install ziglang --break-system-packages
RUN python3 -c "import ziglang, os; os.symlink(os.path.join(os.path.dirname(ziglang.__file__), 'zig'), '/usr/local/bin/zig')"

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
