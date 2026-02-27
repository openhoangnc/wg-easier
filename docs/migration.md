# Migration from wg-easy v15

## Overview

`wg-easy-rs` is a drop-in replacement for `wg-easy`. It uses the same SQLite
database schema and environment variables.

## Steps

1. **Stop the original container:**
   ```bash
   docker stop wg-easy
   ```

2. **Keep the data volume** (`/etc/wireguard`). The database file `wg-easy.db`
   is compatible.

3. **Start the new container:**
   ```bash
   docker run -d \
     --cap-add NET_ADMIN \
     --cap-add SYS_MODULE \
     -v ~/.wg-easy:/etc/wireguard \
     -e WG_HOST=your.domain.com \
     -e PASSWORD_HASH='$2b$12$...' \
     -p 51820:51820/udp \
     -p 51821:51821/tcp \
     ghcr.io/your-org/wg-easy-rs:latest
   ```

## Differences

| Feature | wg-easy | wg-easy-rs |
|---------|---------|------------|
| Backend | Node.js | Rust |
| Frontend | Vue 3 | React 18 |
| WireGuard | wg-quick | rtnetlink + wireguard-control |
| NAT | iptables | nftables (rustables) |
| Image base | node:alpine | scratch |
| Image size | ~200 MB | ~10–15 MB |
| `WG_PRE_UP` etc. | Shell hooks | Not supported (logged as warning) |

## Environment Variables

All original `wg-easy` environment variables are supported.

> **Note:** `WG_PRE_UP`, `WG_POST_UP`, `WG_PRE_DOWN`, `WG_POST_DOWN` are
> parsed but **ignored** with a warning, because the `FROM scratch` image
> has no shell to execute them.
