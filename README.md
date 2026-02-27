# wg-easy-rs

> Drop-in Rust/React replacement for [wg-easy](https://github.com/wg-easy/wg-easy).

Same Docker interface. No Node.js. No wg-quick. ~10 MB image.

## Usage

```bash
docker run -d \
  --cap-add NET_ADMIN \
  --cap-add SYS_MODULE \
  -v ~/.wg-easy:/etc/wireguard \
  -e WG_HOST=your.domain.com \
  -e PASSWORD_HASH='...' \
  -p 51820:51820/udp \
  -p 51821:51821/tcp \
  docker pull ghcr.io/openhoangnc/wg-easier:latest
```

See `docs/` for full documentation.

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `WG_HOST` | — | **Required.** Public hostname or IP for WireGuard endpoint |
| `WG_PORT` | `51820` | WireGuard UDP listen port |
| `WG_MTU` | — | Optional MTU override |
| `WG_DEFAULT_ADDRESS` | `10.8.0.x` | Client IP range |
| `WG_DEFAULT_DNS` | `1.1.1.1` | DNS for clients |
| `WG_ALLOWED_IPS` | `0.0.0.0/0` | Allowed IPs pushed to clients |
| `PORT` | `51821` | Web UI / API HTTP port |
| `PASSWORD_HASH` | — | bcrypt hash of the admin password |
| `INSECURE` | `false` | Disable authentication (dev only) |
| `WG_DB_PATH` | `/etc/wireguard/wg-easy.db` | SQLite database path |
| `WG_OUTBOUND_IFACE` | `eth0` | Physical network interface for NAT outbound traffic |

## Architecture

- **Backend**: Rust + Axum, statically linked musl binary
- **Frontend**: React + Vite + Tailwind CSS SPA
- **WireGuard**: rtnetlink + wireguard-control (no wg-quick)
- **NAT**: rustables / nftables (no iptables)
- **Image**: `FROM scratch`, ~10–15 MB
